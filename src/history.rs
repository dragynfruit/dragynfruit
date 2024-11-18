use egui::{self, CentralPanel};

use crate::Tab;

pub struct HistoryTab {}
impl Tab for HistoryTab {
    fn title(&self) -> &str {
        "History"
    }
    fn url(&self) -> &str {
        "browser:history"
    }
    fn update(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("History");
        });
    }
}
