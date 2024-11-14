use std::rc::Rc;

use compositing::windowing::WindowMethods;
use compositing_traits::{CompositorProxy, CompositorReceiver};
use constellation::Constellation;
use eframe::{egui, egui_glow, glow};
use webrender::{create_webrender_instance, RenderApi, Renderer};
use webrender_api::{DocumentId, RenderNotifier};
use webrender_traits::RenderingContext;

pub struct Compositor {
    sender: CompositorProxy,
    receiver: CompositorReceiver,
    renderer: Renderer,
    document: DocumentId,
    render_api: RenderApi,
    render_ctx: RenderingContext,
}
impl Compositor {
    pub fn new(fr: eframe::Frame) {
        let sender = CompositorProxy {};
        let receiver = CompositorReceiver {};
        compositing::IOCompositor::new(
            a,
            state,
            composite_target,
            exit_after_load,
            convert_mouse_to_touch,
            top_level_browsing_context_id,
            version_string,
        );
        create_webrender_instance(
            fr.gl().unwrap(),
            notifier,
            webrender::WebRenderOptions::default(),
            None,
        );
    }
}
impl WindowMethods for Compositor {
    fn get_coordinates(&self) -> compositing::windowing::EmbedderCoordinates {}
}
