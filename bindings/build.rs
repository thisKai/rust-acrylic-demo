fn main() {
    windows::build!(
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
        windows::storage::streams::{
            InMemoryRandomAccessStream,
            IBuffer,
            DataWriter,
        }
        windows::win32::system_services::{
            CreateDispatcherQueueController,
            TRUE,
            FALSE,
            WM_CREATE,
            WM_ACTIVATE,
            WM_NCCALCSIZE,
            WM_NCHITTEST,
            WS_CAPTION,
            WS_OVERLAPPEDWINDOW,
            SWP_FRAMECHANGED,
            HTTOPLEFT, HTTOP, HTCAPTION, HTTOPRIGHT,
            HTLEFT, HTNOWHERE, HTRIGHT,
            HTBOTTOMLEFT, HTBOTTOM, HTBOTTOMRIGHT,
        }
        windows::win32::winrt::{ICompositorDesktopInterop, RoInitialize}
        windows::win32::windows_and_messaging::{
            GetWindowRect,
            AdjustWindowRectEx,
            SetWindowPos,
            NCCALCSIZE_PARAMS,
        }
        windows::win32::shell::{
            SetWindowSubclass,
            DefSubclassProc,
        }
        windows::win32::dwm::{
            DwmExtendFrameIntoClientArea,
            DwmIsCompositionEnabled,
            DwmDefWindowProc,
        }
    );
}
