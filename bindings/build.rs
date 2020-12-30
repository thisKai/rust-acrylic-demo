fn main() {
    winrt::build!(
        microsoft::graphics::canvas::*
        microsoft::graphics::canvas::effects::*
        microsoft::graphics::canvas::ui::composition::*
        windows::foundation::numerics::{Vector2, Vector3}
        windows::foundation::TimeSpan
        windows::graphics::SizeInt32
        windows::system::DispatcherQueueController
        windows::ui::composition::{
            AnimationIterationBehavior,
            CompositionBatchTypes,
            CompositionBorderMode,
            CompositionColorBrush,
            CompositionGeometry,
            CompositionShape,
            CompositionSpriteShape,
            Compositor,
            ContainerVisual,
            SpriteVisual,
            CompositionEffectSourceParameter,
        }
        windows::ui::composition::desktop::DesktopWindowTarget
        windows::ui::Colors
        windows::storage::streams::{InMemoryRandomAccessStream, IBuffer, DataWriter}
    );
}
