use eframe::egui::{
    self, scroll_area::ScrollBarVisibility, Button, CentralPanel, Color32, CursorIcon, Frame, Id,
    Margin, ScrollArea, Sense, Shadow, Spacing, Stroke, Style, TextEdit, TopBottomPanel, Vec2,
};

use crate::Tab;

pub struct UserscriptsTab {}
impl Tab for UserscriptsTab {
    fn title(&self) -> &str {
        "Userscripts"
    }
    fn url(&self) -> &str {
        "browser:userscripts"
    }
    fn update(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Userscripts manager");
        });
    }
}
