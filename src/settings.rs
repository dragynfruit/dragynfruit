use egui::{
    self, scroll_area::ScrollBarVisibility, Button, CentralPanel, Color32, CursorIcon, Frame, Id,
    Margin, ScrollArea, Sense, Shadow, Spacing, Stroke, Style, TextEdit, TopBottomPanel, Vec2,
};

use crate::Tab;

pub struct SettingsTab {}
impl Tab for SettingsTab {
    fn title(&self) -> &str {
        "Settings"
    }
    fn url(&self) -> &str {
        "browser:settings"
    }
    fn update(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Settings");
        });
    }
}
