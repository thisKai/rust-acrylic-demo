mod interop;
mod window_target;

use {
    bindings::{
        microsoft::graphics::canvas::{
            effects::{
                BlendEffect, BlendEffectMode, BorderEffect, ColorSourceEffect, CompositeEffect,
                EffectBorderMode, GaussianBlurEffect, OpacityEffect, SaturationEffect,
            },
            ui::composition::CanvasComposition,
            CanvasBitmap, CanvasComposite, CanvasDevice, CanvasEdgeBehavior,
        },
        windows::{
            foundation::{numerics::Vector2, Size},
            graphics::directx::{DirectXAlphaMode, DirectXPixelFormat},
            storage::streams::{DataWriter, InMemoryRandomAccessStream},
            ui::{
                composition::{CompositionEffectSourceParameter, CompositionStretch, Compositor},
                Color, Colors,
            },
        },
    },
    futures::executor::block_on,
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

    let acrylic_effect = {
        let effect = BlendEffect::new()?;

        effect.set_mode(BlendEffectMode::Overlay)?;
        effect.set_background({
            let effect = CompositeEffect::new()?;
            effect.set_mode(CanvasComposite::SourceOver)?;
            let sources = effect.sources()?;

            sources.append({
                let effect = BlendEffect::new()?;
                effect.set_mode(BlendEffectMode::Exclusion)?;
                effect.set_background({
                    let effect = SaturationEffect::new()?;
                    effect.set_saturation(2.)?;
                    effect.set_source({
                        let effect = GaussianBlurEffect::new()?;
                        effect.set_source(CompositionEffectSourceParameter::create("Backdrop")?)?;
                        effect.set_border_mode(EffectBorderMode::Hard)?;
                        effect.set_blur_amount(30.)?;

                        effect
                    })?;

                    effect
                })?;
                effect.set_foreground({
                    let effect = ColorSourceEffect::new()?;
                    effect.set_color(Color {
                        a: 26,
                        r: 24,
                        g: 24,
                        b: 24,
                    })?;
                    effect
                })?;
                effect
            })?;
            sources.append({
                let effect = ColorSourceEffect::new()?;
                effect.set_color(Color {
                    a: 128,
                    r: 24,
                    g: 24,
                    b: 24,
                })?;
                effect
            })?;
            effect
        })?;
        effect.set_foreground({
            let effect = OpacityEffect::new()?;
            effect.set_opacity(0.02)?;
            effect.set_source({
                let effect = BorderEffect::new()?;
                effect.set_extendx(CanvasEdgeBehavior::Wrap)?;
                effect.set_extendy(CanvasEdgeBehavior::Wrap)?;
                effect.set_source(CompositionEffectSourceParameter::create("Noise")?)?;
                effect
            })?;
            effect
        })?;

        effect
    };

    let canvas_device = CanvasDevice::get_shared_device()?;
    let composition_graphics_device =
        CanvasComposition::create_composition_graphics_device(&compositor, &canvas_device)?;

    let noise_drawing_surface = composition_graphics_device.create_drawing_surface(
        Size {
            width: 256.,
            height: 256.,
        },
        DirectXPixelFormat::B8G8R8A8UIntNormalized,
        DirectXAlphaMode::Premultiplied,
    )?;

    let noise_bytes = include_bytes!("noise.png");
    let noise_stream = InMemoryRandomAccessStream::new()?;
    let data_writer = DataWriter::create_data_writer(&noise_stream)?;
    data_writer.write_bytes(noise_bytes)?;
    block_on(data_writer.store_async()?)?;
    let bitmap = block_on(CanvasBitmap::load_async_from_stream(
        &canvas_device,
        &noise_stream,
    )?)?;
    {
        let ds = CanvasComposition::create_drawing_session(&noise_drawing_surface)?;
        ds.clear(Colors::transparent()?)?;
        ds.draw_image_at_origin(&bitmap)?;
    }
    let noise_brush = compositor.create_surface_brush_with_surface(&noise_drawing_surface)?;
    noise_brush.set_stretch(CompositionStretch::None)?;

    let effect_factory = compositor.create_effect_factory(&acrylic_effect)?;
    let acrylic_effect_brush = effect_factory.create_brush()?;
    let destination_brush = compositor.create_backdrop_brush()?;
    acrylic_effect_brush.set_source_parameter("Backdrop", &destination_brush)?;
    acrylic_effect_brush.set_source_parameter("Noise", &noise_brush)?;

    root.set_brush(acrylic_effect_brush)?;
    root.set_relative_size_adjustment(Vector2 { x: 1., y: 1. })?;
    target.set_root(&root)?;

    event_loop.run(move |event, _target, control_flow| match event {
        Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        },
        _ => {}
    });
}
