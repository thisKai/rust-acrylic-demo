use crate::util::get_hwnd_from_raw_window_handle;
use raw_window_handle::HasRawWindowHandle;
use windows::{
    core::Interface,
    Win32::System::WinRT::Composition::ICompositorDesktopInterop,
    UI::Composition::{Compositor, Desktop::DesktopWindowTarget},
};

pub(crate) unsafe fn create_compositor_desktop_window_target<W: HasRawWindowHandle>(
    window: &W,
    compositor: &Compositor,
    is_topmost: bool,
) -> windows::core::Result<DesktopWindowTarget> {
    let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;

    compositor_desktop
        .CreateDesktopWindowTarget(get_hwnd_from_raw_window_handle(window), is_topmost)
}
