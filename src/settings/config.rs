// Copyright 2023 The Grim Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use grin_core::global;
use grin_core::global::ChainTypes;
use serde_derive::{Deserialize, Serialize};
use crate::gui::views::Content;

use crate::node::NodeConfig;
use crate::Settings;
use crate::wallet::ConnectionsConfig;

/// Application configuration, stored at toml file.
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    /// Run node server on startup.
    pub(crate) auto_start_node: bool,
    /// Chain type for node and wallets.
    pub(crate) chain_type: ChainTypes,

    /// Flag to check if Android integrated node warning was shown.
    android_integrated_node_warning: Option<bool>,

    /// Flag to show wallet list at dual panel wallets mode.
    show_wallets_at_dual_panel: bool,
    /// Flag to show all connections at network panel or integrated node info.
    show_connections_network_panel: bool,

    /// Width of the desktop window.
    width: f32,
    /// Height of the desktop window.
    height: f32,

    /// Position of the desktop window.
    x: Option<f32>, y: Option<f32>,

    /// Locale code for i18n.
    lang: Option<String>,
    /// Flag to use English locale layout on keyboard.
    english_keyboard: Option<bool>,

    /// Flag to check if dark theme should be used, use system settings if not set.
    use_dark_theme: Option<bool>,

    /// Flag to use proxy for network requests.
    use_proxy: Option<bool>,
    /// Flag to use SOCKS5 or HTTP proxy for network requests.
    use_socks_proxy: Option<bool>,
    /// HTTP proxy URL.
    http_proxy_url: Option<String>,
    /// SOCKS5 proxy URL.
    socks_proxy_url: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auto_start_node: false,
            chain_type: ChainTypes::default(),
            android_integrated_node_warning: None,
            show_wallets_at_dual_panel: false,
            show_connections_network_panel: false,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            x: None,
            y: None,
            lang: None,
            english_keyboard: None,
            use_dark_theme: None,
            use_proxy: None,
            use_socks_proxy: None,
            http_proxy_url: None,
            socks_proxy_url: None,
        }
    }
}

impl AppConfig {
    /// Desktop window frame margin sum, horizontal or vertical.
    const FRAME_MARGIN: f32 = Content::WINDOW_FRAME_MARGIN * 2.0;
    /// Default desktop window width.
    pub const DEFAULT_WIDTH: f32 = Content::SIDE_PANEL_WIDTH * 3.0 + Self::FRAME_MARGIN;
    /// Default desktop window height.
    pub const DEFAULT_HEIGHT: f32 = 706.0;
    /// Minimal desktop window width.
    pub const MIN_WIDTH: f32 = Content::SIDE_PANEL_WIDTH + Self::FRAME_MARGIN;
    /// Minimal desktop window height.
    pub const MIN_HEIGHT: f32 = 630.0 + Content::WINDOW_TITLE_HEIGHT + Self::FRAME_MARGIN;

    /// Application configuration file name.
    pub const FILE_NAME: &'static str = "app.toml";

    /// Default i18n locale.
    pub const DEFAULT_LOCALE: &'static str = "en";

    /// Save application configuration to the file.
    pub fn save(&self) {
        Settings::write_to_file(self, Settings::config_path(Self::FILE_NAME, None));
    }

    /// Change global [`ChainTypes`] and load new [`NodeConfig`].
    pub fn change_chain_type(chain_type: &ChainTypes) {
        let current_chain_type = Self::chain_type();
        if current_chain_type != *chain_type {
            // Save chain type at app config.
            {
                let mut w_app_config = Settings::app_config_to_update();
                w_app_config.chain_type = *chain_type;
                w_app_config.save();
            }
            // Load node configuration for selected chain type.
            {
                let mut w_node_config = Settings::node_config_to_update();
                let node_config = NodeConfig::for_chain_type(chain_type);
                w_node_config.node = node_config.node;
                w_node_config.peers = node_config.peers;
            }
            // Load connections configuration
            {
                let mut w_conn_config = Settings::conn_config_to_update();
                *w_conn_config = ConnectionsConfig::for_chain_type(chain_type);
            }
        }
        if !global::GLOBAL_CHAIN_TYPE.is_init() {
            global::init_global_chain_type(*chain_type);
        } else {
            global::set_global_chain_type(*chain_type);
            global::set_local_chain_type(*chain_type);
        }
    }

    /// Get current [`ChainTypes`] for node and wallets.
    pub fn chain_type() -> ChainTypes {
        let r_config = Settings::app_config_to_read();
        r_config.chain_type
    }

    /// Check if integrated node is starting with application.
    pub fn autostart_node() -> bool {
        let r_config = Settings::app_config_to_read();
        r_config.auto_start_node
    }

    /// Toggle integrated node autostart.
    pub fn toggle_node_autostart() {
        let autostart = Self::autostart_node();
        let mut w_app_config = Settings::app_config_to_update();
        w_app_config.auto_start_node = !autostart;
        w_app_config.save();
    }

    /// Check if it's needed to show wallet list at dual panel wallets mode.
    pub fn show_wallets_at_dual_panel() -> bool {
        let r_config = Settings::app_config_to_read();
        r_config.show_wallets_at_dual_panel
    }

    /// Toggle flag to show wallet list at dual panel wallets mode.
    pub fn toggle_show_wallets_at_dual_panel() {
        let show = Self::show_wallets_at_dual_panel();
        let mut w_app_config = Settings::app_config_to_update();
        w_app_config.show_wallets_at_dual_panel = !show;
        w_app_config.save();
    }

    /// Check if it's needed to show all connections or integrated node info at network panel.
    pub fn show_connections_network_panel() -> bool {
        let r_config = Settings::app_config_to_read();
        r_config.show_connections_network_panel
    }

    /// Toggle flag to show all connections or integrated node info at network panel.
    pub fn toggle_show_connections_network_panel() {
        let show = Self::show_connections_network_panel();
        let mut w_app_config = Settings::app_config_to_update();
        w_app_config.show_connections_network_panel = !show;
        w_app_config.save();
    }

    /// Save desktop window width and height.
    pub fn save_window_size(w: f32, h: f32) {
        let mut w_app_config = Settings::app_config_to_update();
        w_app_config.width = w;
        w_app_config.height = h;
        w_app_config.save();
    }

    /// Get desktop window width and height.
    pub fn window_size() -> (f32, f32) {
        let r_config = Settings::app_config_to_read();
        (r_config.width, r_config.height)
    }

    /// Save desktop window position.
    pub fn save_window_pos(x: f32, y: f32) {
        let mut w_app_config = Settings::app_config_to_update();
        w_app_config.x = Some(x);
        w_app_config.y = Some(y);
        w_app_config.save();
    }

    /// Get desktop window position.
    pub fn window_pos() -> Option<(f32, f32)> {
        let r_config = Settings::app_config_to_read();
        if r_config.x.is_some() && r_config.y.is_some() {
            return Some((r_config.x.unwrap(), r_config.y.unwrap()))
        }
        None
    }

    /// Save locale code.
    pub fn save_locale(lang: &str) {
        let mut w_app_config = Settings::app_config_to_update();
        w_app_config.lang = Some(lang.to_string());
        w_app_config.save();
    }

    /// Get current saved locale code.
    pub fn locale() -> Option<String> {
        let r_config = Settings::app_config_to_read();
        if r_config.lang.is_some() {
            return Some(r_config.lang.clone().unwrap())
        }
        None
    }

    /// Toggle English locale layout. for software keyboard.
    pub fn toggle_english_keyboard() {
        let english = Self::english_keyboard();
        let mut w_app_config = Settings::app_config_to_update();
        w_app_config.english_keyboard = Some(!english);
        w_app_config.save();
    }

    /// Check if English locale layout should be used for software keyboard.
    pub fn english_keyboard() -> bool {
        let r_config = Settings::app_config_to_read();
        r_config.english_keyboard.unwrap_or(false)
    }

    /// Check if integrated node warning is needed for Android.
    pub fn android_integrated_node_warning_needed() -> bool {
        let r_config = Settings::app_config_to_read();
        r_config.android_integrated_node_warning.unwrap_or(true)
    }

    /// Mark integrated node warning for Android as shown.
    pub fn show_android_integrated_node_warning() {
        let mut w_config = Settings::app_config_to_update();
        w_config.android_integrated_node_warning = Some(false);
        w_config.save();
    }

    /// Check if dark theme should be used.
    pub fn dark_theme() -> Option<bool> {
        let r_config = Settings::app_config_to_read();
        r_config.use_dark_theme.clone()
    }

    /// Setup flag to use dark theme.
    pub fn set_dark_theme(use_dark: bool) {
        let mut w_config = Settings::app_config_to_update();
        w_config.use_dark_theme = Some(use_dark);
        w_config.save();
    }

    /// Check if proxy for network requests is needed.
    pub fn use_proxy() -> bool {
        let r_config = Settings::app_config_to_read();
        r_config.use_proxy.clone().unwrap_or(false)
    }

    /// Enable or disable proxy for network requests.
    pub fn toggle_use_proxy() {
        let use_proxy = Self::use_proxy();
        let mut w_config = Settings::app_config_to_update();
        w_config.use_proxy = Some(!use_proxy);
        w_config.save();
    }

    /// Check if SOCKS5 or HTTP proxy should be used.
    pub fn use_socks_proxy() -> bool {
        let r_config = Settings::app_config_to_read();
        r_config.use_socks_proxy.clone().unwrap_or(true)
    }

    /// Enable SOCKS5 or HTTP proxy.
    pub fn toggle_use_socks_proxy() {
        let use_proxy = Self::use_socks_proxy();
        let mut w_config = Settings::app_config_to_update();
        w_config.use_socks_proxy = Some(!use_proxy);
        w_config.save();
    }

    /// Get SOCKS proxy URL.
    pub fn socks_proxy_url() -> Option<String> {
        let r_config = Settings::app_config_to_read();
        r_config.socks_proxy_url.clone()
    }

    /// Save SOCKS proxy URL.
    pub fn save_socks_proxy_url(url: Option<String>) {
        let mut w_config = Settings::app_config_to_update();
        w_config.socks_proxy_url = url;
        w_config.save();
    }

    /// Get HTTP proxy URL.
    pub fn http_proxy_url() -> Option<String> {
        let r_config = Settings::app_config_to_read();
        r_config.http_proxy_url.clone()
    }

    /// Save HTTP proxy URL.
    pub fn save_http_proxy_url(url: Option<String>) {
        let mut w_config = Settings::app_config_to_update();
        w_config.http_proxy_url = url;
        w_config.save();
    }

}