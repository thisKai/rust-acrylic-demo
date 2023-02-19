mod interop;
mod window_subclass;
mod window_target;

use futures_executor::block_on;
use interop::create_dispatcher_queue_controller_for_current_thread;
use win2d_uwp::Microsoft::Graphics::Canvas::{
    CanvasBitmap, CanvasComposite, CanvasDevice, CanvasEdgeBehavior,
    Effects::{
        BlendEffect, BlendEffectMode, BorderEffect, ColorSourceEffect, CompositeEffect,
        EffectBorderMode, GaussianBlurEffect, OpacityEffect, SaturationEffect,
    },
    UI::Composition::CanvasComposition,
};
use window_subclass::WindowSubclass;
use window_target::CompositionDesktopWindowTargetSource;
use windows::{
    h,
    Foundation::{Numerics::Vector2, Size},
    Graphics::{
        DirectX::{DirectXAlphaMode, DirectXPixelFormat},
        Effects::IGraphicsEffectSource,
    },
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream},
    UI::{
        Color, Colors,
        Composition::{CompositionEffectSourceParameter, CompositionStretch, Compositor},
    },
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowBuilderExtWindows,
    window::WindowBuilder,
};

fn main() -> windows::core::Result<()> {
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_visible(false)
        .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap();

    unsafe {
        window.apply_subclass();
    }
    window.set_visible(true);

    let compositor = Compositor::new()?;
    let target = window.create_window_target(&compositor, false)?;

    let root = compositor.CreateSpriteVisual()?;
    let clip = compositor.CreateInsetClip()?;
    clip.SetTopInset(1.)?;
    root.SetClip(&clip)?;

    let acrylic_effect = {
        let effect = BlendEffect::new()?;
        effect.SetMode(BlendEffectMode::Overlay)?;
        effect.SetBackground(
            &({
                let effect = CompositeEffect::new()?;
                effect.SetMode(CanvasComposite::SourceOver)?;
                let sources = effect.Sources()?;

                sources.Append(&{
                    let effect = BlendEffect::new()?;
                    effect.SetMode(BlendEffectMode::Exclusion)?;
                    effect.SetBackground(&{
                        let effect = SaturationEffect::new()?;
                        effect.SetSaturation(2.)?;
                        effect.SetSource(&{
                            let effect = GaussianBlurEffect::new()?;
                            effect.SetSource(&CompositionEffectSourceParameter::Create(h!(
                                "Backdrop"
                            ))?)?;
                            effect.SetBorderMode(EffectBorderMode::Hard)?;
                            effect.SetBlurAmount(30.)?;

                            effect
                        })?;

                        effect
                    })?;
                    effect.SetForeground(&{
                        let effect = ColorSourceEffect::new()?;
                        effect.SetColor(Color {
                            A: 26,
                            R: 24,
                            G: 24,
                            B: 24,
                        })?;
                        effect
                    })?;
                    IGraphicsEffectSource::try_from(effect)?
                })?;
                sources.Append(&{
                    let effect = ColorSourceEffect::new()?;
                    effect.SetColor(Color {
                        A: 128,
                        R: 24,
                        G: 24,
                        B: 24,
                    })?;
                    IGraphicsEffectSource::try_from(effect)?
                })?;
                effect
            }),
        )?;
        effect.SetForeground(
            &({
                let effect = OpacityEffect::new()?;
                effect.SetOpacity(0.02)?;
                effect.SetSource(&{
                    let effect = BorderEffect::new()?;
                    effect.SetExtendX(CanvasEdgeBehavior::Wrap)?;
                    effect.SetExtendY(CanvasEdgeBehavior::Wrap)?;
                    effect.SetSource(&CompositionEffectSourceParameter::Create(h!("Noise"))?)?;
                    effect
                })?;
                effect
            }),
        )?;

        effect
    };

    let canvas_device = CanvasDevice::GetSharedDevice()?;
    let composition_graphics_device =
        CanvasComposition::CreateCompositionGraphicsDevice(&compositor, &canvas_device)?;

    let noise_drawing_surface = composition_graphics_device.CreateDrawingSurface(
        Size {
            Width: 256.,
            Height: 256.,
        },
        DirectXPixelFormat::B8G8R8A8UIntNormalized,
        DirectXAlphaMode::Premultiplied,
    )?;

    let noise_bytes = include_bytes!("noise.png");
    let noise_stream = InMemoryRandomAccessStream::new()?;
    let data_writer = DataWriter::CreateDataWriter(&noise_stream)?;
    data_writer.WriteBytes(noise_bytes)?;
    block_on(data_writer.StoreAsync()?)?;
    let bitmap = block_on(CanvasBitmap::LoadAsyncFromStream(
        &canvas_device,
        &noise_stream,
    )?)?;
    {
        let ds = CanvasComposition::CreateDrawingSession(&noise_drawing_surface)?;
        ds.Clear(Colors::Transparent()?)?;
        ds.DrawImageAtOrigin(&bitmap)?;
    }
    let noise_brush = compositor.CreateSurfaceBrushWithSurface(&noise_drawing_surface)?;
    noise_brush.SetStretch(CompositionStretch::None)?;
    noise_brush.SetHorizontalAlignmentRatio(0.)?;
    noise_brush.SetVerticalAlignmentRatio(0.)?;

    let effect_factory = compositor.CreateEffectFactory(&acrylic_effect)?;
    let acrylic_effect_brush = effect_factory.CreateBrush()?;
    let destination_brush = compositor.CreateBackdropBrush()?;
    acrylic_effect_brush.SetSourceParameter(h!("Backdrop"), &destination_brush)?;
    acrylic_effect_brush.SetSourceParameter(h!("Noise"), &noise_brush)?;

    root.SetBrush(&acrylic_effect_brush)?;
    root.SetRelativeSizeAdjustment(Vector2 { X: 1., Y: 1. })?;
    target.SetRoot(&root)?;

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
