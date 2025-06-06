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

use egui::{Id, RichText};

use crate::gui::Colors;
use crate::gui::icons::{BEZIER_CURVE, BOUNDING_BOX, CHART_SCATTER, CIRCLES_THREE, CLOCK_COUNTDOWN, HAND_COINS};
use crate::gui::platform::PlatformCallbacks;
use crate::gui::views::{Modal, TextEdit, View};
use crate::gui::views::network::settings::NetworkSettings;
use crate::gui::views::types::{ContentContainer, ModalPosition};
use crate::node::NodeConfig;

/// Memory pool setup section content.
pub struct PoolSetup {
    /// Base fee value that's accepted into the pool.
    fee_base_edit: String,
    /// Reorg cache retention period value in minutes.
    reorg_period_edit: String,
    /// Maximum number of transactions allowed in the pool.
    pool_size_edit: String,
    /// Maximum number of transactions allowed in the stempool.
    stempool_size_edit: String,
    /// Maximum total weight of transactions to build a block.
    max_weight_edit: String,
}

/// Identifier for base fee value [`Modal`].
const FEE_BASE_MODAL: &'static str = "fee_base";
/// Identifier for reorg cache retention period value [`Modal`].
const REORG_PERIOD_MODAL: &'static str = "reorg_period";
/// Identifier for maximum number of transactions in the pool [`Modal`].
const POOL_SIZE_MODAL: &'static str = "pool_size";
/// Identifier for maximum number of transactions in the stempool [`Modal`].
const STEMPOOL_SIZE_MODAL: &'static str = "stempool_size";
/// Identifier for maximum total weight of transactions [`Modal`].
const MAX_WEIGHT_MODAL: &'static str = "max_weight";

impl Default for PoolSetup {
    fn default() -> Self {
        Self {
            fee_base_edit: NodeConfig::get_base_fee(),
            reorg_period_edit: NodeConfig::get_reorg_cache_period(),
            pool_size_edit: NodeConfig::get_max_pool_size(),
            stempool_size_edit: NodeConfig::get_max_stempool_size(),
            max_weight_edit: NodeConfig::get_mineable_max_weight(),
        }
    }
}

impl ContentContainer for PoolSetup {
    fn modal_ids(&self) -> Vec<&'static str> {
        vec![
            FEE_BASE_MODAL,
            REORG_PERIOD_MODAL,
            POOL_SIZE_MODAL,
            STEMPOOL_SIZE_MODAL,
            MAX_WEIGHT_MODAL
        ]
    }

    fn modal_ui(&mut self,
                ui: &mut egui::Ui,
                modal: &Modal,
                cb: &dyn PlatformCallbacks) {
        match modal.id {
            FEE_BASE_MODAL => self.fee_base_modal(ui, modal, cb),
            REORG_PERIOD_MODAL => self.reorg_period_modal(ui, modal, cb),
            POOL_SIZE_MODAL => self.pool_size_modal(ui, modal, cb),
            STEMPOOL_SIZE_MODAL => self.stem_size_modal(ui, modal, cb),
            MAX_WEIGHT_MODAL => self.max_weight_modal(ui, modal, cb),
            _ => {}
        }
    }

    fn container_ui(&mut self, ui: &mut egui::Ui, _: &dyn PlatformCallbacks) {
        View::sub_title(ui, format!("{} {}", CHART_SCATTER, t!("network_settings.tx_pool")));
        View::horizontal_line(ui, Colors::stroke());
        ui.add_space(6.0);

        ui.vertical_centered(|ui| {
            // Show base fee setup.
            self.fee_base_ui(ui);

            ui.add_space(6.0);
            View::horizontal_line(ui, Colors::item_stroke());
            ui.add_space(6.0);

            // Show reorg cache retention period setup.
            self.reorg_period_ui(ui);

            ui.add_space(6.0);
            View::horizontal_line(ui, Colors::item_stroke());
            ui.add_space(6.0);

            // Show pool size setup.
            self.pool_size_ui(ui);

            ui.add_space(6.0);
            View::horizontal_line(ui, Colors::item_stroke());
            ui.add_space(6.0);

            // Show stem pool size setup.
            self.stem_size_ui(ui);

            ui.add_space(6.0);
            View::horizontal_line(ui, Colors::item_stroke());
            ui.add_space(6.0);

            // Show max weight of transactions setup.
            self.max_weight_ui(ui);
        });
    }
}

impl PoolSetup {
    /// Draw fee base setup content.
    fn fee_base_ui(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(t!("network_settings.pool_fee"))
            .size(16.0)
            .color(Colors::gray())
        );
        ui.add_space(6.0);

        let fee = NodeConfig::get_base_fee();
        View::button(ui, format!("{} {}", HAND_COINS, &fee), Colors::white_or_black(false), || {
            // Setup values for modal.
            self.fee_base_edit = fee;
            // Show fee setup modal.
            Modal::new(FEE_BASE_MODAL)
                .position(ModalPosition::CenterTop)
                .title(t!("network_settings.change_value"))
                .show();
        });
        ui.add_space(6.0);
    }

    /// Draw fee base [`Modal`] content.
    fn fee_base_modal(&mut self, ui: &mut egui::Ui, modal: &Modal, cb: &dyn PlatformCallbacks) {
        let on_save = |c: &mut PoolSetup| {
            if let Ok(fee) = c.fee_base_edit.parse::<u64>() {
                NodeConfig::save_base_fee(fee);
                Modal::close();
            }
        };

        ui.add_space(6.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(t!("network_settings.pool_fee"))
                .size(17.0)
                .color(Colors::gray()));
            ui.add_space(8.0);

            // Draw fee base text edit.
            let mut edit = TextEdit::new(Id::from(modal.id)).h_center().numeric();
            edit.ui(ui, &mut self.fee_base_edit, cb);
            if edit.enter_pressed {
                on_save(self);
            }

            // Show error when specified value is not valid or reminder to restart enabled node.
            if self.fee_base_edit.parse::<u64>().is_err() {
                ui.add_space(12.0);
                ui.label(RichText::new(t!("network_settings.not_valid_value"))
                    .size(17.0)
                    .color(Colors::red()));
            } else {
                NetworkSettings::node_restart_required_ui(ui);
            }
            ui.add_space(12.0);

            // Show modal buttons.
            ui.scope(|ui| {
                // Setup spacing between buttons.
                ui.spacing_mut().item_spacing = egui::Vec2::new(8.0, 0.0);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.cancel"), Colors::white_or_black(false), || {
                            // Close modal.
                            Modal::close();
                        });
                    });
                    columns[1].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.save"), Colors::white_or_black(false), || {
                            on_save(self);
                        });
                    });
                });
                ui.add_space(6.0);
            });
        });
    }

    /// Draw reorg cache retention period setup content.
    fn reorg_period_ui(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(t!("network_settings.reorg_period"))
            .size(16.0)
            .color(Colors::gray())
        );
        ui.add_space(6.0);
        let period = NodeConfig::get_reorg_cache_period();
        View::button(ui,
                     format!("{} {}", CLOCK_COUNTDOWN, &period),
                     Colors::white_or_black(false), || {
            // Setup values for modal.
            self.reorg_period_edit = period;
            // Show reorg period setup modal.
            Modal::new(REORG_PERIOD_MODAL)
                .position(ModalPosition::CenterTop)
                .title(t!("network_settings.change_value"))
                .show();
        });
        ui.add_space(6.0);
    }

    /// Draw reorg cache retention period [`Modal`] content.
    fn reorg_period_modal(&mut self, ui: &mut egui::Ui, modal: &Modal, cb: &dyn PlatformCallbacks) {
        let on_save = |c: &mut PoolSetup| {
            if let Ok(period) = c.reorg_period_edit.parse::<u32>() {
                NodeConfig::save_reorg_cache_period(period);
                Modal::close();
            }
        };

        ui.add_space(6.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(t!("network_settings.reorg_period"))
                .size(17.0)
                .color(Colors::gray()));
            ui.add_space(8.0);

            // Draw reorg period text edit.
            let mut edit = TextEdit::new(Id::from(modal.id)).h_center().numeric();
            edit.ui(ui, &mut self.reorg_period_edit, cb);
            if edit.enter_pressed {
                on_save(self);
            }

            // Show error when specified value is not valid or reminder to restart enabled node.
            if self.reorg_period_edit.parse::<u32>().is_err() {
                ui.add_space(12.0);
                ui.label(RichText::new(t!("network_settings.not_valid_value"))
                    .size(17.0)
                    .color(Colors::red()));
            } else {
                NetworkSettings::node_restart_required_ui(ui);
            }
            ui.add_space(12.0);

            // Show modal buttons.
            ui.scope(|ui| {
                // Setup spacing between buttons.
                ui.spacing_mut().item_spacing = egui::Vec2::new(8.0, 0.0);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.cancel"), Colors::white_or_black(false), || {
                            // Close modal.
                            Modal::close();
                        });
                    });
                    columns[1].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.save"), Colors::white_or_black(false), || {
                            on_save(self);
                        });
                    });
                });
                ui.add_space(6.0);
            });
        });
    }

    /// Draw maximum number of transactions in the pool setup content.
    fn pool_size_ui(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(t!("network_settings.max_tx_pool"))
            .size(16.0)
            .color(Colors::gray())
        );
        ui.add_space(6.0);

        let size = NodeConfig::get_max_pool_size();
        View::button(ui, format!("{} {}", CIRCLES_THREE, size), Colors::white_or_black(false), || {
            // Setup values for modal.
            self.pool_size_edit = size;
            // Show pool size setup modal.
            Modal::new(POOL_SIZE_MODAL)
                .position(ModalPosition::CenterTop)
                .title(t!("network_settings.change_value"))
                .show();
        });
        ui.add_space(6.0);
    }

    /// Draw maximum number of transactions in the pool [`Modal`] content.
    fn pool_size_modal(&mut self, ui: &mut egui::Ui, modal: &Modal, cb: &dyn PlatformCallbacks) {
        let on_save = |c: &mut PoolSetup| {
            if let Ok(size) = c.pool_size_edit.parse::<usize>() {
                NodeConfig::save_max_pool_size(size);
                Modal::close();
            }
        };

        ui.add_space(6.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(t!("network_settings.max_tx_pool"))
                .size(17.0)
                .color(Colors::gray()));
            ui.add_space(8.0);

            // Draw pool size text edit.
            let mut edit = TextEdit::new(Id::from(modal.id)).h_center().numeric();
            edit.ui(ui, &mut self.pool_size_edit, cb);
            if edit.enter_pressed {
                on_save(self);
            }

            // Show error when specified value is not valid or reminder to restart enabled node.
            if self.pool_size_edit.parse::<usize>().is_err() {
                ui.add_space(12.0);
                ui.label(RichText::new(t!("network_settings.not_valid_value"))
                    .size(17.0)
                    .color(Colors::red()));
            } else {
                NetworkSettings::node_restart_required_ui(ui);
            }
            ui.add_space(12.0);

            // Show modal buttons.
            ui.scope(|ui| {
                // Setup spacing between buttons.
                ui.spacing_mut().item_spacing = egui::Vec2::new(8.0, 0.0);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.cancel"), Colors::white_or_black(false), || {
                            // Close modal.
                            Modal::close();
                        });
                    });
                    columns[1].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.save"), Colors::white_or_black(false), || {
                            on_save(self);
                        });
                    });
                });
                ui.add_space(6.0);
            });
        });
    }

    /// Draw maximum number of transactions in the stempool setup content.
    fn stem_size_ui(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(t!("network_settings.max_tx_stempool"))
            .size(16.0)
            .color(Colors::gray())
        );
        ui.add_space(6.0);

        let size = NodeConfig::get_max_stempool_size();
        View::button(ui,
                     format!("{} {}", BEZIER_CURVE, &size),
                     Colors::white_or_black(false), || {
            // Setup values for modal.
            self.stempool_size_edit = size;
            // Show stempool size setup modal.
            Modal::new(STEMPOOL_SIZE_MODAL)
                .position(ModalPosition::CenterTop)
                .title(t!("network_settings.change_value"))
                .show();
        });
        ui.add_space(6.0);
    }

    /// Draw maximum number of transactions in the stempool [`Modal`] content.
    fn stem_size_modal(&mut self, ui: &mut egui::Ui, modal: &Modal, cb: &dyn PlatformCallbacks) {
        let on_save = |c: &mut PoolSetup| {
            if let Ok(size) = c.stempool_size_edit.parse::<usize>() {
                NodeConfig::save_max_stempool_size(size);
                Modal::close();
            }
        };

        ui.add_space(6.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(t!("network_settings.max_tx_stempool"))
                .size(17.0)
                .color(Colors::gray()));
            ui.add_space(8.0);

            // Draw stempool size text edit.
            let mut edit = TextEdit::new(Id::from(modal.id)).h_center().numeric();
            edit.ui(ui, &mut self.stempool_size_edit, cb);
            if edit.enter_pressed {
                on_save(self);
            }

            // Show error when specified value is not valid or reminder to restart enabled node.
            if self.stempool_size_edit.parse::<usize>().is_err() {
                ui.add_space(12.0);
                ui.label(RichText::new(t!("network_settings.not_valid_value"))
                    .size(17.0)
                    .color(Colors::red()));
            } else {
                NetworkSettings::node_restart_required_ui(ui);
            }
            ui.add_space(12.0);

            // Show modal buttons.
            ui.scope(|ui| {
                // Setup spacing between buttons.
                ui.spacing_mut().item_spacing = egui::Vec2::new(8.0, 0.0);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.cancel"), Colors::white_or_black(false), || {
                            // Close modal.
                            Modal::close();
                        });
                    });
                    columns[1].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.save"), Colors::white_or_black(false), || {
                            on_save(self);
                        });
                    });
                });
                ui.add_space(6.0);
            });
        });
    }

    /// Draw maximum total weight of transactions setup content.
    fn max_weight_ui(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(t!("network_settings.max_tx_weight"))
            .size(16.0)
            .color(Colors::gray())
        );
        ui.add_space(6.0);

        let weight = NodeConfig::get_mineable_max_weight();
        View::button(ui,
                     format!("{} {}", BOUNDING_BOX, &weight),
                     Colors::white_or_black(false), || {
            // Setup values for modal.
            self.max_weight_edit = weight;
            // Show total tx weight setup modal.
            Modal::new(MAX_WEIGHT_MODAL)
                .position(ModalPosition::CenterTop)
                .title(t!("network_settings.change_value"))
                .show();
        });
        ui.add_space(6.0);
    }

    /// Draw maximum total weight of transactions [`Modal`] content.
    fn max_weight_modal(&mut self, ui: &mut egui::Ui, modal: &Modal, cb: &dyn PlatformCallbacks) {
        let on_save = |c: &mut PoolSetup| {
            if let Ok(weight) = c.max_weight_edit.parse::<u64>() {
                NodeConfig::save_mineable_max_weight(weight);
                Modal::close();
            }
        };

        ui.add_space(6.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(t!("network_settings.max_tx_weight"))
                .size(17.0)
                .color(Colors::gray()));
            ui.add_space(8.0);

            // Draw tx weight text edit.
            let mut edit = TextEdit::new(Id::from(modal.id)).h_center().numeric();
            edit.ui(ui, &mut self.max_weight_edit, cb);
            if edit.enter_pressed {
                on_save(self);
            }

            // Show error when specified value is not valid or reminder to restart enabled node.
            if self.max_weight_edit.parse::<u64>().is_err() {
                ui.add_space(12.0);
                ui.label(RichText::new(t!("network_settings.not_valid_value"))
                    .size(17.0)
                    .color(Colors::red()));
            } else {
                NetworkSettings::node_restart_required_ui(ui);
            }
            ui.add_space(12.0);

            // Show modal buttons.
            ui.scope(|ui| {
                // Setup spacing between buttons.
                ui.spacing_mut().item_spacing = egui::Vec2::new(8.0, 0.0);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.cancel"), Colors::white_or_black(false), || {
                            // Close modal.
                            Modal::close();
                        });
                    });
                    columns[1].vertical_centered_justified(|ui| {
                        View::button(ui, t!("modal.save"), Colors::white_or_black(false), || {
                            on_save(self);
                        });
                    });
                });
                ui.add_space(6.0);
            });
        });
    }
}