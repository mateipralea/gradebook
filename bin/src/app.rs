// Copyright © 2025 Matei Pralea <matei@pralea.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;

use crate::extra_impl::extra_ctx_impl::ExtraCtxImpl;
use crate::extra_impl::extra_ui_impl::ExtraUiImpl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Application {
    compact: bool,
}

impl Default for Application {
    fn default() -> Self {
        Self { compact: false }
    }
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.compact = ctx.is_compact();

        egui::Window::new("Gradebook").show(ctx, |ui| {
            ui.custom_heading("Gradebook");
            ui.theme_combo_box();
            ui.heading(format!("Compact: {}", self.compact));
        });
    }
}
