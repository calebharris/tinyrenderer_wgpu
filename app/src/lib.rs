use std::{process::Termination, sync::Arc};

use cfg_if::cfg_if;
use engine::Engine;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod engine;
mod resources;
mod texture;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug);
        } else {
            env_logger::init()
        }
    }
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let window = Arc::new(window);

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas().unwrap());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    if let Some(new_size) = window.request_inner_size(LogicalSize::new(400, 400)) {
        log::debug!("Got new size after request: {:?}", new_size);
    } else {
        log::debug!("None returned from request");
    }

    let engine = Engine::new(window, 800, 800).await.unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, target| match event {
            //TODO: check window_id
            #[cfg(not(target_arch = "wasm32"))]
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => target.exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                log::debug!("Window was just resized to {:?}", new_size);
            }
            Event::AboutToWait => {
                if let Err(e) = engine.render() {
                    log::error!("got some kind of error while rendering: {}", e);
                }
            }
            _ => {}
        })
        .report();
}
