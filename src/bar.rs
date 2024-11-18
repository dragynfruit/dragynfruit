use egui::{
    self, scroll_area::ScrollBarVisibility, Button, Color32, Frame, Pos2, RichText, Rounding,
    ScrollArea, Sense, Stroke, Style, TextEdit, TopBottomPanel, Vec2, Window,
};

use crate::{
    history::HistoryTab, settings::SettingsTab, userscripts::UserscriptsTab, Home, WindowState,
};

impl crate::Window {
    /// Show the top bar
    pub(crate) fn top_bar(st: &mut WindowState, ctx: &egui::Context) {
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(4.0);
            ScrollArea::horizontal()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let mut del = None;
                        let iter = st
                            .tabs
                            .iter()
                            .map(|x| x.title().to_string())
                            .enumerate()
                            .collect::<Vec<_>>();
                        for (i, title) in iter {
                            let res = TabWidget {
                                // TODO: this is bad for performance
                                title: title.clone(),
                                active: st.current_tab == i,
                            }
                            .ui(ui);
                            let len = st.tabs.len();

                            // Move tabs
                            if res.drag_delta().x == 0.0 {
                            } else if res.drag_delta().x > 10.0 {
                                st.tabs.swap(i, (i + 1).min(len));
                            } else if res.drag_delta().x < -10.0 {
                                st.tabs.swap(i, (i - 1).min(len));
                            }

                            // Switch to this tab if clicked
                            if res.clicked() {
                                st.current_tab = i;
                                st.current_url = title;
                            }

                            // Close this tab if middle clicked
                            if res.middle_clicked() {
                                del = Some(i);
                            }
                        }
                        if let Some(del) = del {
                            st.tabs.remove(del);
                        }

                        // Button to open a new tab
                        if ui
                            .add(Button::new("+").rounding(Rounding::same(5.0)))
                            .on_hover_text("New tab")
                            .clicked()
                        {
                            st.tabs.push(Box::new(Home::default()));
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
                    TextEdit::singleline(&mut st.current_url),
                );

                // Button to open the options menu
                if ui
                    .add(Button::new("…").rounding(Rounding::same(5.0)))
                    .on_hover_text("Options")
                    .clicked()
                {
                    st.show_options_menu = !st.show_options_menu;
                }
            });
            ui.add_space(4.0);
        });
    }

    /// Show the options menu
    pub(crate) fn options_menu(st: &mut WindowState, ctx: &egui::Context) {
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
                    st.tabs.push(Box::new(HistoryTab {}));
                    st.show_options_menu = false;
                }
                if ui
                    .add(
                        Button::new("Userscripts")
                            .min_size(MIN_SIZE)
                            .rounding(Rounding::same(5.0)),
                    )
                    .clicked()
                {
                    st.tabs.push(Box::new(UserscriptsTab {}));
                    st.show_options_menu = false;
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
                    st.tabs.push(Box::new(SettingsTab {}));
                    st.show_options_menu = false;
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
