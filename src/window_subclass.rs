use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, LRESULT, RECT, TRUE, WPARAM},
    Graphics::Dwm::{DwmDefWindowProc, DwmExtendFrameIntoClientArea, DwmIsCompositionEnabled},
    UI::{
        Controls::MARGINS,
        Shell::{DefSubclassProc, SetWindowSubclass},
        WindowsAndMessaging::{
            AdjustWindowRectEx, GetWindowRect, SetWindowPos, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT,
            HTCAPTION, HTLEFT, HTNOWHERE, HTRIGHT, HTTOP, HTTOPLEFT, HTTOPRIGHT, NCCALCSIZE_PARAMS,
            SWP_FRAMECHANGED, WINDOW_EX_STYLE, WM_ACTIVATE, WM_CREATE, WM_NCCALCSIZE, WM_NCHITTEST,
            WS_CAPTION, WS_OVERLAPPEDWINDOW,
        },
    },
};

pub trait WindowSubclass {
    unsafe fn apply_subclass(&self);
}
impl<W: HasRawWindowHandle> WindowSubclass for W {
    unsafe fn apply_subclass(&self) {
        // Get the window handle
        let window_handle = self.raw_window_handle();
        let window_handle = match window_handle {
            RawWindowHandle::Win32(window_handle) => window_handle.hwnd,
            _ => panic!("Unsupported platform!"),
        };
        SetWindowSubclass(HWND(window_handle as isize), Some(subclass_procedure), 1, 0);
    }
}

extern "system" fn subclass_procedure(
    h_wnd: HWND,
    u_msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
    _u_id_subclass: usize,
    _dw_ref_data: usize,
) -> LRESULT {
    unsafe {
        if is_dwm_enabled() {
            let (dwm_result, dwm_handled) = {
                let mut result = LRESULT(0);
                let handled =
                    DwmDefWindowProc(h_wnd, u_msg, w_param, l_param, &mut result).as_bool();
                (result, handled)
            };

            if u_msg == WM_CREATE {
                let mut rect = RECT::default();
                GetWindowRect(h_wnd, &mut rect);

                // Inform application of the frame change.
                let width = rect.right - rect.left;
                let height = rect.bottom - rect.top;
                SetWindowPos(
                    h_wnd,
                    HWND(0),
                    rect.left,
                    rect.top,
                    width,
                    height,
                    SWP_FRAMECHANGED as _,
                );
            }
            if u_msg == WM_ACTIVATE {
                // Extend the frame into the client area.
                let p_mar_inset = MARGINS {
                    cyTopHeight: 2,
                    ..Default::default()
                };
                let _ = DwmExtendFrameIntoClientArea(h_wnd, &p_mar_inset);
            }
            if u_msg == WM_NCCALCSIZE && w_param == WPARAM(TRUE.0 as _) {
                let frame_rect = window_frame_borders(true);
                let caption_height = -frame_rect.top;

                // Calculate new NCCALCSIZE_PARAMS based on custom NCA inset.
                let pncsp = &mut *(l_param.0 as *mut NCCALCSIZE_PARAMS);

                pncsp.rgrc[0].left -= 0;
                pncsp.rgrc[0].top -= caption_height;
                pncsp.rgrc[0].right += 0;
                pncsp.rgrc[0].bottom += 1;
            }
            if u_msg == WM_NCHITTEST && dwm_result == LRESULT(0) {
                let hit_test_result = hit_test_nca(h_wnd, l_param);

                if hit_test_result == LRESULT(HTNOWHERE as _) {
                    return LRESULT(HTCAPTION as _);
                }
                return hit_test_result;
            }

            if dwm_handled {
                return dwm_result;
            }
        }

        DefSubclassProc(h_wnd, u_msg, w_param, l_param)
    }
}

unsafe fn is_dwm_enabled() -> bool {
    DwmIsCompositionEnabled()
        .map(BOOL::as_bool)
        .unwrap_or_default()
}

unsafe fn hit_test_nca(h_wnd: HWND, l_param: LPARAM) -> LRESULT {
    // Get the point coordinates for the hit test.
    let (x, y) = get_l_param_point(l_param);

    // Get the window rectangle.
    let mut window_rect = RECT::default();
    GetWindowRect(h_wnd, &mut window_rect);

    // Get the frame rectangle, adjusted for the style without a caption.
    let frame_rect = window_frame_borders(false);

    // Get the frame rectangle, adjusted for the style with a caption.
    let caption_frame_rect = window_frame_borders(true);

    // Determine if the hit test is for resizing. Default middle (1,1).
    let mut row = 1;
    let mut col = 1;
    let mut on_resize_border = false;

    // Determine if the point is at the top or bottom of the window.
    if y >= window_rect.top && y < window_rect.top - caption_frame_rect.top {
        on_resize_border = y < (window_rect.top - frame_rect.top);
        row = 0;
    } else if y < window_rect.bottom && y >= window_rect.bottom - caption_frame_rect.bottom {
        row = 2;
    }

    // Determine if the point is at the left or right of the window.
    if x >= window_rect.left && x < window_rect.left - caption_frame_rect.left {
        col = 0; // left side
    } else if x < window_rect.right && x >= window_rect.right - caption_frame_rect.right {
        col = 2; // right side
    }

    // Hit test (HTTOPLEFT, ... HTBOTTOMRIGHT)
    let hit_tests = [
        [
            HTTOPLEFT,
            if on_resize_border { HTTOP } else { HTCAPTION },
            HTTOPRIGHT,
        ],
        [HTLEFT, HTNOWHERE, HTRIGHT],
        [HTBOTTOMLEFT, HTBOTTOM, HTBOTTOMRIGHT],
    ];
    LRESULT(hit_tests[row][col] as _)
}

unsafe fn window_frame_borders(with_caption: bool) -> RECT {
    let style_flags = if with_caption {
        WS_OVERLAPPEDWINDOW
    } else {
        WS_OVERLAPPEDWINDOW & !WS_CAPTION
    };

    let mut rect = RECT::default();
    AdjustWindowRectEx(&mut rect, style_flags, false, WINDOW_EX_STYLE(0));
    rect
}

pub fn get_l_param_point(lp: LPARAM) -> (i32, i32) {
    (
        lo_word(lp.0 as u32) as i16 as i32,
        hi_word(lp.0 as u32) as i16 as i32,
    )
}

pub const fn lo_word(l: u32) -> u16 {
    (l & 0xffff) as u16
}
pub const fn hi_word(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}
