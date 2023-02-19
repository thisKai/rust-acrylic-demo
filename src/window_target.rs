use raw_window_handle::HasRawWindowHandle;
use windows::{
    core::Interface,
    Win32::{Foundation::HWND, System::WinRT::Composition::ICompositorDesktopInterop},
    UI::Composition::{Compositor, Desktop::DesktopWindowTarget},
};

pub trait CompositionDesktopWindowTargetSource {
    fn create_window_target(
        &self,
        compositor: &Compositor,
        is_topmost: bool,
    ) -> windows::core::Result<DesktopWindowTarget>;
}

impl<T> CompositionDesktopWindowTargetSource for T
where
    T: HasRawWindowHandle,
{
    fn create_window_target(
        &self,
        compositor: &Compositor,
        is_topmost: bool,
    ) -> windows::core::Result<DesktopWindowTarget> {
        // Get the window handle
        let window_handle = self.raw_window_handle();
        let window_handle = match window_handle {
            raw_window_handle::RawWindowHandle::Win32(window_handle) => window_handle.hwnd,
            _ => panic!("Unsupported platform!"),
        };

        let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;

        unsafe {
            compositor_desktop.CreateDesktopWindowTarget(HWND(window_handle as isize), is_topmost)
        }
    }
}
