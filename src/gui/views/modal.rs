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

use lazy_static::lazy_static;
use std::sync::Arc;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use egui::{Align2, Rect, RichText, Rounding, Stroke, Vec2};
use egui::epaint::{RectShape, Shadow};
use egui::os::OperatingSystem;

use crate::gui::Colors;
use crate::gui::views::{Content, View};
use crate::gui::views::types::{ModalPosition, ModalState};

lazy_static! {
    /// Showing [`Modal`] state to be accessible from different ui parts.
    static ref MODAL_STATE: Arc<RwLock<ModalState>> = Arc::new(RwLock::new(ModalState::default()));
}

/// Stores data to draw modal [`egui::Window`] at ui.
#[derive(Clone)]
pub struct Modal {
    /// Identifier for modal.
    pub(crate) id: &'static str,
    /// Position on the screen.
    pub position: ModalPosition,
    /// To check if it can be closed.
    closeable: Arc<AtomicBool>,
    /// Title text
    title: Option<String>
}

impl Modal {
    /// Margin from [`Modal`] window at top/left/right.
    const DEFAULT_MARGIN: f32 = 8.0;
    /// Maximum width of the content.
    const DEFAULT_WIDTH: f32 = Content::SIDE_PANEL_WIDTH - (2.0 * Self::DEFAULT_MARGIN);

    /// Create closeable [`Modal`] with center position.
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            position: ModalPosition::Center,
            closeable: Arc::new(AtomicBool::new(true)),
            title: None
        }
    }

    /// Setup position of [`Modal`] on the screen.
    pub fn position(mut self, position: ModalPosition) -> Self {
        self.position = position;
        self
    }

    /// Change [`Modal`] position on the screen.
    pub fn change_position(position: ModalPosition) {
        let mut w_state = MODAL_STATE.write();
        w_state.modal.as_mut().unwrap().position = position;
    }

    /// Mark [`Modal`] closed.
    pub fn close(&self) {
        let mut w_nav = MODAL_STATE.write();
        w_nav.modal = None;
    }

    /// Setup possibility to close [`Modal`].
    pub fn closeable(self, closeable: bool) -> Self {
        self.closeable.store(closeable, Ordering::Relaxed);
        self
    }

    /// Disable possibility to close [`Modal`].
    pub fn disable_closing(&self) {
        self.closeable.store(false, Ordering::Relaxed);
    }

    /// Enable possibility to close [`Modal`].
    pub fn enable_closing(&self) {
        self.closeable.store(true, Ordering::Relaxed);
    }

    /// Check if [`Modal`] is closeable.
    pub fn is_closeable(&self) -> bool {
        self.closeable.load(Ordering::Relaxed)
    }

    /// Set title text on [`Modal`] creation.
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title.to_uppercase());
        self
    }

    /// Set [`Modal`] instance into state to show at ui.
    pub fn show(self) {
        let mut w_nav = MODAL_STATE.write();
        w_nav.modal = Some(self);
    }

    /// Remove [`Modal`] from [`ModalState`] if it's showing and can be closed.
    /// Return `false` if Modal existed in [`ModalState`] before call.
    pub fn on_back() -> bool {
        let mut w_state = MODAL_STATE.write();

        // If Modal is showing and closeable, remove it from state.
        if w_state.modal.is_some() {
            let modal = w_state.modal.as_ref().unwrap();
            if modal.is_closeable() {
                w_state.modal = None;
            }
            return false;
        }
        true
    }

    /// Return id of opened [`Modal`].
    pub fn opened() -> Option<&'static str> {
        // Check if modal is showing.
        {
            if MODAL_STATE.read().modal.is_none() {
                return None;
            }
        }

        // Get identifier of opened modal.
        let r_state = MODAL_STATE.read();
        let modal = r_state.modal.as_ref().unwrap();
        Some(modal.id)
    }

    /// Set title text for current opened [`Modal`].
    pub fn set_title(title: String) {
        // Save state.
        let mut w_state = MODAL_STATE.write();
        if w_state.modal.is_some() {
            let mut modal = w_state.modal.clone().unwrap();
            modal.title = Some(title.to_uppercase());
            w_state.modal = Some(modal);
        }
    }

    /// Draw opened [`Modal`] content.
    pub fn ui(ctx: &egui::Context, add_content: impl FnOnce(&mut egui::Ui, &Modal)) {
        let has_modal = {
            MODAL_STATE.read().modal.is_some()
        };
        if has_modal {
            let modal = {
                let r_state = MODAL_STATE.read();
                r_state.modal.clone().unwrap()
            };
            modal.window_ui(ctx, add_content);
        }
    }

    /// Draw [`egui::Window`] with provided content.
    fn window_ui(&self, ctx: &egui::Context, add_content: impl FnOnce(&mut egui::Ui, &Modal)) {
        let is_fullscreen = ctx.input(|i| {
            i.viewport().fullscreen.unwrap_or(false)
        });
        let is_mac_os = OperatingSystem::from_target_os() == OperatingSystem::Mac;

        let mut rect = ctx.screen_rect();
        if View::is_desktop() && !is_mac_os {
            let margin = if !is_fullscreen {
                Content::WINDOW_FRAME_MARGIN
            } else {
                0.0
            };
            rect = rect.shrink(margin - 0.5);
            rect.min += egui::vec2(0.0, Content::WINDOW_TITLE_HEIGHT + 0.5);
            rect.max.x += 0.5;
        }
        egui::Window::new("modal_bg_window")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .fixed_rect(rect)
            .frame(egui::Frame {
                fill: Colors::semi_transparent(),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.set_min_size(rect.size());
            });

        // Setup width of modal content.
        let side_insets = View::get_left_inset() + View::get_right_inset();
        let available_width = rect.width() - (side_insets + Self::DEFAULT_MARGIN);
        let width = f32::min(available_width, Self::DEFAULT_WIDTH);

        // Show main content Window at given position.
        let (content_align, content_offset) = self.modal_position(is_fullscreen);
        let layer_id = egui::Window::new(format!("modal_window_{}", self.id))
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .min_width(width)
            .default_width(width)
            .anchor(content_align, content_offset)
            .frame(egui::Frame {
                shadow: Shadow {
                    offset: Default::default(),
                    blur: 30.0,
                    spread: 3.0,
                    color: egui::Color32::from_black_alpha(32),
                },
                rounding: Rounding::same(8.0),
                fill: Colors::fill(),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if self.title.is_some() {
                    self.title_ui(ui);
                }
                self.content_ui(ui, add_content);
            }).unwrap().response.layer_id;

        // Always show main content Window above background Window.
        ctx.move_to_top(layer_id);

    }

    /// Get [`egui::Window`] position based on [`ModalPosition`].
    fn modal_position(&self, is_fullscreen: bool) -> (Align2, Vec2) {
        let align = match self.position {
            ModalPosition::CenterTop => Align2::CENTER_TOP,
            ModalPosition::Center => Align2::CENTER_CENTER
        };

        let x_align = View::get_left_inset() - View::get_right_inset();

        let is_mac_os = OperatingSystem::from_target_os() == OperatingSystem::Mac;
        let extra_y = if View::is_desktop() && !is_mac_os {
            Content::WINDOW_TITLE_HEIGHT + if !is_fullscreen {
                Content::WINDOW_FRAME_MARGIN
            } else {
                0.0
            }
        } else {
            0.0
        };
        let y_align = View::get_top_inset() + Self::DEFAULT_MARGIN + extra_y;

        let offset = match self.position {
            ModalPosition::CenterTop => Vec2::new(x_align, y_align),
            ModalPosition::Center => Vec2::new(x_align, 0.0)
        };
        (align, offset)
    }

    /// Draw provided content.
    fn content_ui(&self, ui: &mut egui::Ui, add_content: impl FnOnce(&mut egui::Ui, &Modal)) {
        let mut rect = ui.available_rect_before_wrap();
        rect.min += egui::emath::vec2(6.0, 0.0);
        rect.max -= egui::emath::vec2(6.0, 0.0);

        // Create background shape.
        let rounding = if self.title.is_some() {
            Rounding {
                nw: 0.0,
                ne: 0.0,
                sw: 8.0,
                se: 8.0,
            }
        } else {
            Rounding::same(8.0)
        };
        let mut bg_shape = RectShape {
            rect,
            rounding,
            fill: Colors::fill(),
            stroke: Stroke::NONE,
            blur_width: 0.0,
            fill_texture_id: Default::default(),
            uv: Rect::ZERO
        };
        let bg_idx = ui.painter().add(bg_shape);

        // Draw main content.
        let mut content_rect = ui.allocate_ui_at_rect(rect, |ui| {
            (add_content)(ui, self);
        }).response.rect;

        // Setup background shape to be painted behind main content.
        content_rect.min -= egui::emath::vec2(6.0, 0.0);
        content_rect.max += egui::emath::vec2(6.0, 0.0);
        bg_shape.rect = content_rect;
        ui.painter().set(bg_idx, bg_shape);
    }

    /// Draw title content.
    fn title_ui(&self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();

        // Create background shape.
        let mut bg_shape = RectShape {
            rect,
            rounding: Rounding {
                nw: 8.0,
                ne: 8.0,
                sw: 0.0,
                se: 0.0,
            },
            fill: Colors::yellow(),
            stroke: Stroke::NONE,
            blur_width: 0.0,
            fill_texture_id: Default::default(),
            uv: Rect::ZERO
        };
        let bg_idx = ui.painter().add(bg_shape);

        // Draw title content.
        let title_resp = ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(Self::DEFAULT_MARGIN + 1.0);
                ui.label(RichText::new(self.title.as_ref().unwrap())
                    .size(19.0)
                    .color(Colors::title(true))
                );
                ui.add_space(Self::DEFAULT_MARGIN);
                // Draw line below title.
                View::horizontal_line(ui, Colors::item_stroke());
            });
        }).response;

        // Setup background shape to be painted behind title content.
        bg_shape.rect = title_resp.rect;
        ui.painter().set(bg_idx, bg_shape);
    }
}