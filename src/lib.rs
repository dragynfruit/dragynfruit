//!
//! # Dragynfruit
//! The next web browser.
//!

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate toml_edit;

mod bar;
mod config;
mod history;
mod settings;
mod userscripts;

use std::{error::Error, sync::Arc, time::Instant};

use dashmap::DashMap;
use egui::{mutex::Mutex, CentralPanel, FontId, Key, Modifiers, RichText, TextEdit, Vec2};
use egui_glow::{
    egui_winit::winit::{
        dpi::LogicalSize,
        event::{DeviceEvent, Event as WinitEvent, WindowEvent},
        event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
        raw_window_handle::{HasDisplayHandle, HasWindowHandle},
        window::{Window as WinitWindow, WindowId},
    },
    glow, EventResponse,
};
use egui_glue::EguiGlow;
use surfman::{Connection, SurfaceType};
use webrender::euclid::Size2D;
use webrender_traits::RenderingContext;

mod egui_glue;

pub struct Window {
    eg: EguiGlow,
    render_ctx: RenderingContext,
    win: WinitWindow,

    st: WindowState,
}
pub struct WindowState {
    current_tab: usize,
    current_url: String,
    tabs: Vec<Box<dyn Tab>>,
    show_options_menu: bool,
    started_up: bool,
}
impl Window {
    pub fn new(ev_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>> {
        let attrs = WinitWindow::default_attributes()
            .with_title("Dragynfruit".to_string())
            .with_inner_size(LogicalSize::new(800, 600))
            .with_visible(true);

        let win = ev_loop
            .create_window(attrs)
            .expect("Failed to create window.");

        // #[cfg(any(target_os = "linux", target_os = "windows"))]
        // {
        //     let icon_bytes = include_bytes!("../../../resources/servo_64.png");
        //     winit_window.set_window_icon(Some(load_icon(icon_bytes)));
        // }

        let dh = win.display_handle().unwrap();
        let conn = Connection::from_display_handle(dh).unwrap();
        let adap = conn.create_adapter().unwrap();
        let wh = win.window_handle().unwrap();

        let inner_sz = win.inner_size();
        let native_widget = conn
            .create_native_widget_from_window_handle(
                wh,
                Size2D::<u32, u32>::new(inner_sz.width, inner_sz.height)
                    .to_i32()
                    .to_untyped(),
            )
            .expect("Failed to create native widget");

        let surface_type = SurfaceType::Widget { native_widget };
        let rendering_context = RenderingContext::create(&conn, &adap, surface_type)
            .expect("Failed to create WR surfman");
        rendering_context.make_gl_context_current().unwrap();

        let gl = Arc::new(unsafe {
            glow::Context::from_loader_function(|s| rendering_context.get_proc_address(s))
        });

        let eg = EguiGlow::new(&ev_loop, gl.clone(), None);
        win.set_visible(true);
        win.focus_window();

        Ok(Self {
            eg: eg,
            render_ctx: rendering_context,
            win,
            st: WindowState {
                show_options_menu: false,
                tabs: Vec::new(),
                current_tab: 0,
                current_url: String::new(),
                started_up: false,
            },
        })
    }
    fn on_win_ev(&mut self, event: &WindowEvent) -> EventResponse {
        self.eg.on_window_event(&self.win, event)
    }
    fn update(&mut self) {
        let Self { eg, win, st, .. } = self;

        eg.run(&win, |ctx| {
            Self::render(st, ctx);
        });
        eg.paint(&self.win);
        self.render_ctx.present().unwrap();
    }
    fn render(st: &mut WindowState, ctx: &egui::Context) {
        //if self.show_options_menu {
        //    let mut fonts = FontDefinitions::default();
        //    fonts.font_data.insert(
        //        "Falling Sky".to_owned(),
        //        FontData::from_static(include_bytes!("../FallingSky.otf")),
        //    );
        //    fonts.families.insert(
        //        FontFamily::Name("Falling Sky".into()),
        //        vec!["Falling Sky".into()],
        //    );
        //    fonts
        //        .families
        //        .get_mut(&FontFamily::Proportional)
        //        .unwrap()
        //        .insert(0, "Falling Sky".into());
        //    ctx.set_fonts(fonts);
        //}

        // Show the options menu if it's open
        if st.show_options_menu {
            Self::options_menu(st, ctx);
        }

        // Render the top bar
        Self::top_bar(st, ctx);

        // If all tabs are closed, open the home page
        if st.tabs.is_empty() {
            st.tabs.push(Box::new(Home::default()));
        }

        // If the current tab is the last one and is closed, go to the new last one
        if st.current_tab >= st.tabs.len() {
            st.current_tab = st.tabs.len() - 1;
        }

        // Render the current tab
        st.tabs.get_mut(st.current_tab).unwrap().update(ctx);

        ctx.input_mut(|input| {
            if input.consume_key(Modifiers::CTRL, Key::T) {
                st.tabs.push(Box::new(Home::default()));
            }
            if input.consume_key(Modifiers::CTRL, Key::W) {
                st.tabs.remove(st.current_tab);
            }
            if input.consume_key(Modifiers::CTRL, Key::ArrowLeft) {
                if st.current_tab == 0 {
                    st.current_tab = st.tabs.len() - 1;
                } else {
                    st.current_tab -= 1;
                }
            }
            if input.consume_key(Modifiers::CTRL, Key::ArrowRight) {
                if st.current_tab == st.tabs.len() - 1 {
                    st.current_tab = 0;
                } else {
                    st.current_tab += 1;
                }
            }
        });
    }
}

pub struct Browser {
    ev_loop: EventLoop<()>,
    windows: DashMap<WindowId, Window>,
}
impl Browser {
    pub fn new() -> Self {
        let ev_loop = EventLoop::with_user_event()
            .build()
            .expect("failed to create event loop");

        ev_loop.create_proxy().send_event(()).unwrap();

        Self {
            ev_loop,
            windows: DashMap::new(),
        }
    }
    pub fn run(self) {
        self.ev_loop.set_control_flow(ControlFlow::Poll);
        self.ev_loop
            .run(|ev, ev_loop| {
                match ev {
                    WinitEvent::UserEvent(()) => {
                        let win = Window::new(ev_loop).unwrap();
                        let id = win.win.id().clone();
                        self.windows.insert(id, win);
                    }
                    WinitEvent::WindowEvent { event, window_id } => {
                        let mut win = self.windows.get_mut(&window_id).unwrap();
                        match event {
                            WindowEvent::RedrawRequested => {
                                //let run_st = Instant::now();
                                //let _wait = eg.run(&win, |ctx| );
                                //println!("run: {:?}", run_st.elapsed());

                                //let p_st = Instant::now();
                                //eg.paint(&win);
                                //println!("pai: {:?}", p_st.elapsed());

                                //let pr_st = Instant::now();
                                //render_ctx.present().expect("failed to present");
                                //println!("pre: {:?}", pr_st.elapsed());
                                win.update();
                            }
                            WindowEvent::CloseRequested => {
                                win.eg.destroy();
                                ev_loop.exit();
                            }
                            _ => {
                                let needs_repaint = win.on_win_ev(&event).repaint;
                                if needs_repaint {
                                    win.win.request_redraw();
                                }
                            }
                        }
                    }
                    _ => {}
                }
            })
            .unwrap();
    }
}

pub trait Tab {
    fn title(&self) -> &str;
    fn url(&self) -> &str;
    fn update(&mut self, ctx: &egui::Context);
}

#[derive(Default)]
pub struct Home {
    search: String,
}
impl Tab for Home {
    #[inline(always)]
    fn title(&self) -> &str {
        "Home"
    }
    #[inline(always)]
    fn url(&self) -> &str {
        "browser:home"
    }
    fn update(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Dragynfruit").strong().size(36.0));
                ui.add(
                    TextEdit::singleline(&mut self.search)
                        .font(FontId::proportional(18.0))
                        .hint_text("Search...")
                        .min_size(Vec2::new(200.0, 26.0)),
                );
            });
        });
    }
}
