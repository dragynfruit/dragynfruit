extern crate base;
// extern crate compositing;
// extern crate compositing_traits;
// extern crate constellation;
// extern crate embedder_traits;
// extern crate fonts;
extern crate net;
extern crate net_traits;
extern crate script;
extern crate script_traits;
// extern crate servo_config;
// extern crate servo_geometry;
extern crate ipc_channel;
extern crate servo_url;
extern crate surfman;
// extern crate webrender;
// extern crate webrender_api;
// extern crate webrender_traits;

//mod compositor;
mod egui_glue;
//mod proto;
//mod egui_glow;

use std::{error::Error, rc::Rc, sync::Arc, time::Instant};

use compositing::{
    windowing::{EmbedderCoordinates, EmbedderMethods, WindowMethods},
    CompositeTarget,
};
use egui::CentralPanel;
use egui_glow::egui_winit::winit::{
    self,
    dpi::{LogicalSize, PhysicalSize},
    event::Event as WinitEvent,
    event_loop::ActiveEventLoop,
    raw_window_handle::{HasRawWindowHandle, HasWindowHandle},
};
use egui_glow::egui_winit::winit::{
    event_loop::EventLoop, raw_window_handle::HasDisplayHandle, window::Window,
};
use egui_glow::{egui_winit::winit::event::WindowEvent, glow};
use egui_glue::EguiGlow;
use net::protocols::{ProtocolHandler, ProtocolRegistry};
use net_traits::ResourceThreads;
use script::{
    document_loader::{DocumentLoader, LoadType},
    script_thread::ScriptThread,
};
use servo_url::ServoUrl;
use surfman::{Connection, NativeWidget, SurfaceType};
use webrender::{
    create_webrender_instance,
    euclid::{Scale, Size2D},
    RenderApi, Renderer, WebRenderOptions,
};
use webrender_traits::RenderingContext;

//pub struct Embedder {
//    user_agent: String,
//}
//impl EmbedderMethods for Embedder {
//    fn get_version_string(&self) -> Option<String> {
//        Some("Dragynfruit".to_owned())
//    }
//    fn get_protocol_handlers(&self) -> ProtocolRegistry {
//        let mut reg = ProtocolRegistry::with_internal_protocols();
//
//        reg
//    }
//    fn create_event_loop_waker(&mut self) -> Box<dyn embedder_traits::EventLoopWaker> {
//        Box::new(self.)
//    }
//    fn get_user_agent_string(&self) -> Option<String> {
//        Some(self.user_agent.clone())
//    }
//}

#[derive(Clone)]
struct RenderNotifier {
    ctx: egui::Context,
}
impl webrender_api::RenderNotifier for RenderNotifier {
    fn new_frame_ready(
        &self,
        _: webrender_api::DocumentId,
        scrolled: bool,
        composite_needed: bool,
        frame_publish_id: webrender_api::FramePublishId,
    ) {
        // TODO: implement this
    }
    fn wake_up(&self, composite_needed: bool) {
        // TODO: implement this
    }
    fn shut_down(&self) {
        // TODO: implement this
    }
    fn clone(&self) -> Box<dyn webrender_api::RenderNotifier> {
        Box::new(<Self as Clone>::clone(self))
    }
}

pub struct Engine {
    scr_rt: script::script_runtime::Runtime,
    renderer: Renderer,
}
impl Engine {
    pub fn new() -> Result<(), Box<dyn Error>> {
        let mut ev_loop = EventLoop::with_user_event()
            .build()
            .expect("failed to create event loop");

        let attrs = Window::default_attributes()
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

        //let (core_tx, core_rx) = ipc_channel::ipc::channel()?;
        //let (resource_tx, resource_rx) = ipc_channel::ipc::channel()?;

        //let resource_threads = ResourceThreads::new(core_tx, resource_tx);

        //let doc_loader = DocumentLoader::new_with_threads(
        //    resource_threads,
        //    Some(ServoUrl::parse("https://books.toscrape.com/index.html")?),
        //);

        let gl = Arc::new(unsafe {
            glow::Context::from_loader_function(|s| rendering_context.get_proc_address(s))
        });

        let mut eg = EguiGlow::new(&ev_loop, gl.clone(), None);
        win.set_visible(true);
        win.focus_window();
        let _wait = eg.run(&win, |ctx| {
            CentralPanel::default().show(ctx, |ui| {
                if ui.button("a").clicked() {
                    println!("a");
                }
            });
        });
        eg.paint(&win);
        rendering_context.present().unwrap();

        ev_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        ev_loop
            .run(|ev, ev_loop| {
                if let WinitEvent::WindowEvent { event, .. } = ev {
                    match event {
                        WindowEvent::RedrawRequested => {
                            let run_st = Instant::now();
                            let _wait = eg.run(&win, |ctx| {
                                CentralPanel::default().show(ctx, |ui| {
                                    if ui.button("a").clicked() {
                                        println!("a");
                                    }
                                });
                            });
                            println!("run: {:?}", run_st.elapsed());

                            let p_st = Instant::now();
                            eg.paint(&win);
                            println!("pai: {:?}", p_st.elapsed());

                            let pr_st = Instant::now();
                            rendering_context.present().expect("failed to present");
                            println!("pre: {:?}", pr_st.elapsed());
                        }
                        WindowEvent::CloseRequested => {
                            eg.destroy();
                            ev_loop.exit();
                        }
                        _ => {
                            if eg.on_window_event(&win, &event).repaint {
                                win.request_redraw();
                            }
                        }
                    }
                }
            })
            .unwrap();

        Ok(())
    }
}
