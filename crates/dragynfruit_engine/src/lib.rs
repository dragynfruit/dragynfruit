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
extern crate servo_url;
// extern crate webrender;
// extern crate webrender_api;
// extern crate webrender_traits;

mod compositor;
mod egui_glow;

use std::{error::Error, rc::Rc};

use ::egui_glow::{egui_winit::winit::event_loop::EventLoop, winit};
use compositing_traits;
use compositor::Compositor;
use constellation::Constellation;
use egui::ViewportBuilder;
use egui_glow::EguiGlow;
use embedder_traits::EmbedderProxy;
use net::protocols::ProtocolRegistry;
use net_traits::ResourceThreads;
use script::{document_loader::DocumentLoader, DomObject};
use script_traits::{InitialScriptState, ServiceWorkerManagerFactory};
use servo::{
    compositing::{
        windowing::{EmbedderCoordinates, EmbedderMethods, WindowMethods},
        CompositeTarget,
    },
    euclid::Size2D,
    ipc_channel,
    profile_traits::ipc,
    webrender_api::RenderNotifier,
    Servo,
};
use servo_url::ServoUrl;
use webrender::{create_webrender_instance, RenderApi, Renderer, WebRenderOptions};
use webrender_traits::RenderingContext;

pub struct Embedder {}
impl EmbedderMethods for Embedder {
    fn get_version_string(&self) -> Option<String> {
        Some("Dragynfruit".to_owned())
    }
    fn get_protocol_handlers(&self) -> ProtocolRegistry {
        let mut reg = ProtocolRegistry::default();

        reg.register("about", |url| {
            //
        });

        reg
    }
    fn create_event_loop_waker(&mut self) -> Box<dyn servo::embedder_traits::EventLoopWaker> {
        //
    }
    fn get_user_agent_string(&self) -> Option<String> {}
}

pub struct Window {
    ctx: eframe::egui::Context,
    fr: Box<eframe::Frame>,
}
impl RenderNotifier for Window {
    fn new_frame_ready(
        &self,
        _: servo::webrender_api::DocumentId,
        scrolled: bool,
        composite_needed: bool,
        frame_publish_id: servo::webrender_api::FramePublishId,
    ) {
        self.ctx.request_repaint();
    }
}
impl WindowMethods for Window {
    fn rendering_context(&self) -> RenderingContext {
        create_webrender_instance(
            Rc::new(self.fr.gl().unwrap()),
            Box::new(self),
            WebRenderOptions::default(),
            None,
        );
    }
    fn get_coordinates(&self) -> servo::compositing::windowing::EmbedderCoordinates {
        EmbedderCoordinates {
            hidpi_factor: (),
            screen_size: (),
            available_screen_size: (),
            window_rect: (),
            framebuffer: (),
            viewport: (),
        }
    }
    fn set_animation_state(&self, _state: servo::compositing::windowing::AnimationState) {
        //
    }
}

pub struct Engine {
    scr_rt: script::script_runtime::Runtime,
    renderer: Renderer,
}
impl Engine {
    pub fn new() -> Result<(), Box<dyn Error>> {
        let mut s = Servo::new(embedder, window, None, CompositeTarget::Window);
        let scr_rt = script::init();

        let mut ctx = egui::Context::default();
        //
        let mut ev_loop = EventLoop::new()?;
        let window = egui_winit::create_window(&ctx, ev_loop, ViewportBuilder::default())?;

        EguiGlow::new(&ev_loop, gl, shader_version);

        let (core_tx, core_rx) = ipc_channel::ipc::channel();
        let (resource_tx, resource_rx) = ipc_channel::ipc::channel();

        let resource_threads = ResourceThreads::new(core_tx, resource_tx);

        let doc_loader = DocumentLoader::new_with_threads(
            resource_threads,
            Some(ServoUrl::parse("https://books.toscrape.com/index.html")),
        );

        Ok(())
    }
}
