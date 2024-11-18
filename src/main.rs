#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate toml_edit;

//mod bar;
//mod config;
//mod history;
//mod settings;
//mod userscripts;
//
////use mimalloc::MiMalloc;
////
////#[global_allocator]
////static GLOBAL: MiMalloc = MiMalloc;
//
//use eframe::{
//    egui::{
//        self, CentralPanel, FontData, FontDefinitions, FontFamily, FontId, Key, Modifiers,
//        RichText, TextEdit, Vec2,
//    },
//    NativeOptions,
//};

fn main() {
    env_logger::init();

    dragynfruit::Browser::new().run();

    //std::thread::sleep_ms(5000);

    //let mut app = App {
    //    show_options_menu: false,
    //    tabs: Vec::new(),
    //    current_tab: 0,
    //    current_url: String::new(),
    //    started_up: false,
    //};
    //eframe::run_native(
    //    "Dragynfruit",
    //    NativeOptions::default(),
    //    Box::new(|ctx| Ok(Box::new(app))),
    //)
    //.unwrap();
}

//pub struct App {
//    current_tab: usize,
//    current_url: String,
//    tabs: Vec<Box<dyn Tab>>,
//    show_options_menu: bool,
//    started_up: bool,
//}
//impl eframe::App for App {
//    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
//        if self.show_options_menu {
//            if !self.started_up {
//                dragynfruit_engine::Engine::new().unwrap();
//
//                self.started_up = true;
//            }
//        }
//        //if self.show_options_menu {
//        //    let mut fonts = FontDefinitions::default();
//        //    fonts.font_data.insert(
//        //        "Falling Sky".to_owned(),
//        //        FontData::from_static(include_bytes!("../FallingSky.otf")),
//        //    );
//        //    fonts.families.insert(
//        //        FontFamily::Name("Falling Sky".into()),
//        //        vec!["Falling Sky".into()],
//        //    );
//        //    fonts
//        //        .families
//        //        .get_mut(&FontFamily::Proportional)
//        //        .unwrap()
//        //        .insert(0, "Falling Sky".into());
//        //    ctx.set_fonts(fonts);
//        //}
//
//        // Show the options menu if it's open
//        if self.show_options_menu {
//            self.options_menu(ctx);
//        }
//
//        // Render the top bar
//        self.top_bar(ctx);
//
//        // If all tabs are closed, open the home page
//        if self.tabs.is_empty() {
//            self.tabs.push(Box::new(Home::default()));
//        }
//
//        // If the current tab is the last one and is closed, go to the new last one
//        if self.current_tab >= self.tabs.len() {
//            self.current_tab = self.tabs.len() - 1;
//        }
//
//        // Render the current tab
//        self.tabs.get_mut(self.current_tab).unwrap().update(ctx);
//
//        ctx.input_mut(|input| {
//            if input.consume_key(Modifiers::CTRL, Key::T) {
//                self.tabs.push(Box::new(Home::default()));
//            }
//            if input.consume_key(Modifiers::CTRL, Key::W) {
//                self.tabs.remove(self.current_tab);
//            }
//            if input.consume_key(Modifiers::CTRL, Key::ArrowLeft) {
//                if self.current_tab == 0 {
//                    self.current_tab = self.tabs.len() - 1;
//                } else {
//                    self.current_tab -= 1;
//                }
//            }
//            if input.consume_key(Modifiers::CTRL, Key::ArrowRight) {
//                if self.current_tab == self.tabs.len() - 1 {
//                    self.current_tab = 0;
//                } else {
//                    self.current_tab += 1;
//                }
//            }
//        });
//    }
//}
//
//pub trait Tab {
//    fn title(&self) -> &str;
//    fn url(&self) -> &str;
//    fn update(&mut self, ctx: &egui::Context);
//}
//
//#[derive(Default)]
//pub struct Home {
//    search: String,
//}
//impl Tab for Home {
//    #[inline(always)]
//    fn title(&self) -> &str {
//        "Home"
//    }
//    #[inline(always)]
//    fn url(&self) -> &str {
//        "browser:home"
//    }
//    fn update(&mut self, ctx: &egui::Context) {
//        CentralPanel::default().show(ctx, |ui| {
//            ui.vertical_centered(|ui| {
//                ui.label(RichText::new("Dragynfruit").strong().size(36.0));
//                ui.add(
//                    TextEdit::singleline(&mut self.search)
//                        .font(FontId::proportional(18.0))
//                        .hint_text("Search...")
//                        .min_size(Vec2::new(200.0, 26.0)),
//                );
//            });
//        });
//    }
//}
