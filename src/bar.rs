use eframe::egui::{
    self, scroll_area::ScrollBarVisibility, Button, Color32, Frame, Pos2, RichText, Rounding,
    ScrollArea, Sense, Stroke, Style, TextEdit, TopBottomPanel, Vec2, Window,
};

use crate::{history::HistoryTab, settings::SettingsTab, userscripts::UserscriptsTab, Home};

impl crate::App {
    /// Show the top bar
    pub(crate) fn top_bar(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(4.0);
            ScrollArea::horizontal()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let mut del = None;
                        let iter = self
                            .tabs
                            .iter()
                            .map(|x| x.title().to_string())
                            .enumerate()
                            .collect::<Vec<_>>();
                        for (i, title) in iter {
                            let res = TabWidget {
                                // TODO: this is bad for performance
                                title: title.clone(),
                                active: self.current_tab == i,
                            }
                            .ui(ui);
                            let len = self.tabs.len();

                            // Move tabs
                            if res.drag_delta().x == 0.0 {
                            } else if res.drag_delta().x > 10.0 {
                                self.tabs.swap(i, (i + 1).min(len));
                            } else if res.drag_delta().x < -10.0 {
                                self.tabs.swap(i, (i - 1).min(len));
                            }

                            // Switch to this tab if clicked
                            if res.clicked() {
                                self.current_tab = i;
                                self.current_url = title;
                            }

                            // Close this tab if middle clicked
                            if res.middle_clicked() {
                                del = Some(i);
                            }
                        }
                        if let Some(del) = del {
                            self.tabs.remove(del);
                        }

                        // Button to open a new tab
                        if ui
                            .add(Button::new("+").rounding(Rounding::same(5.0)))
                            .on_hover_text("New tab")
                            .clicked()
                        {
                            self.tabs.push(Box::new(Home::default()));
                        }
                    })
                });
            ui.horizontal(|ui| {
                // Buttons to go back and forward in history
                ui.add(Button::new("⏴").rounding(Rounding::same(5.0)))
                    .on_hover_text("Back");
                ui.add(Button::new("⏵").rounding(Rounding::same(5.0)))
                    .on_hover_text("Forward");

                // Button to refresh current page
                ui.add(Button::new("⟳").rounding(Rounding::same(5.0)))
                    .on_hover_text("Refresh");

                // Search / URL bar
                ui.add_sized(
                    Vec2::new((ui.available_width() / 4.0) * 3.0, 18.0),
                    // self.tabs
                    // .get(self.current_tab)
                    // .map(|x| x.url())
                    // .unwrap_or("a"),
                    TextEdit::singleline(&mut self.current_url),
                );

                // Button to open the options menu
                if ui
                    .add(Button::new("…").rounding(Rounding::same(5.0)))
                    .on_hover_text("Options")
                    .clicked()
                {
                    self.show_options_menu = !self.show_options_menu;
                }
            });
            ui.add_space(4.0);
        });
    }

    /// Show the options menu
    pub(crate) fn options_menu(&mut self, ctx: &egui::Context) {
        let mut style = Style::default();
        style.visuals.window_rounding = Rounding::same(5.0);
        Window::new("Options")
            .title_bar(false)
            .resizable([false, false])
            .max_width(100.0)
            .movable(false)
            .fixed_pos(Pos2::new(ctx.available_rect().width() - 115.0, 60.0))
            .frame(Frame::window(&style))
            .show(ctx, |ui| {
                const MIN_SIZE: Vec2 = Vec2::new(100.0, 0.0);
                if ui
                    .add(
                        Button::new("Bookmarks")
                            .min_size(MIN_SIZE)
                            .rounding(Rounding::same(5.0)),
                    )
                    .clicked()
                {
                    //self.tabs.push(Box::new(Bookmarks {}));
                }
                if ui
                    .add(
                        Button::new("History")
                            .min_size(MIN_SIZE)
                            .rounding(Rounding::same(5.0)),
                    )
                    .clicked()
                {
                    self.tabs.push(Box::new(HistoryTab {}));
                    self.show_options_menu = false;
                }
                if ui
                    .add(
                        Button::new("Userscripts")
                            .min_size(MIN_SIZE)
                            .rounding(Rounding::same(5.0)),
                    )
                    .clicked()
                {
                    self.tabs.push(Box::new(UserscriptsTab {}));
                    self.show_options_menu = false;
                }
                ui.separator();
                if ui
                    .add(
                        Button::new("Settings")
                            .min_size(MIN_SIZE)
                            .rounding(Rounding::same(5.0)),
                    )
                    .clicked()
                {
                    self.tabs.push(Box::new(SettingsTab {}));
                    self.show_options_menu = false;
                }
            });
    }
}

struct TabWidget {
    title: String,
    active: bool,
}
impl TabWidget {
    fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut fr = Frame::group(ui.style())
            .inner_margin(Vec2::new(4.0, 2.0))
            .rounding(Rounding::same(5.0));

        if self.active {
            fr = fr.stroke(Stroke::new(1.0, Color32::GREEN));
        }

        let fr = fr
            .show(ui, |ui| {
                let mut text = RichText::new(&self.title);
                if self.active {
                    text = text.strong();
                }
                ui.label(text);
            })
            .response;

        let res = ui.interact(fr.rect, fr.id, Sense::click_and_drag());

        res
    }
}
