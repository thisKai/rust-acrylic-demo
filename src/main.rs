mod interop;
mod window_target;

use {
    bindings::windows::{
        foundation::numerics::Vector2,
        ui::{composition::Compositor, Colors},
    },
    interop::{create_dispatcher_queue_controller_for_current_thread, ro_initialize, RoInitType},
    window_target::CompositionDesktopWindowTargetSource,
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::windows::WindowBuilderExtWindows,
        window::WindowBuilder,
    },
};

fn main() -> winrt::Result<()> {
    ro_initialize(RoInitType::MultiThreaded)?;
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap();

    let compositor = Compositor::new()?;
    let target = window.create_window_target(&compositor, false)?;

    let root = compositor.create_sprite_visual()?;
    root.set_brush(compositor.create_color_brush_with_color(Colors::teal()?)?)?;
    root.set_relative_size_adjustment(Vector2 { x: 1., y: 1. })?;
    target.set_root(&root)?;

    event_loop.run(move |event, _target, control_flow| match event {
        Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                let size = Vector2 {
                    x: size.width as f32,
                    y: size.height as f32,
                };
            }
            _ => {}
        },
        _ => {}
    });
}
