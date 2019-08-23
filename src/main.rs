use std::ptr::{null, null_mut};

use winapi::um::{errhandlingapi, wingdi, winuser};

use wio::wide::ToWide;

mod d3d11;
mod dxgi;
mod util;

fn main() {
    unsafe {
        let class_name = "dx11".to_wide_null();
        let title = "dx11 toy".to_wide_null();
        let icon = winuser::LoadIconW(null_mut(), winuser::IDI_APPLICATION);
        let brush = wingdi::CreateSolidBrush(0xff_ff_ff);
        let class = winuser::WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(winuser::DefWindowProcW),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: null_mut(),
            hIcon: icon,
            hCursor: null_mut(),
            hbrBackground: brush,
            lpszMenuName: null(),
            lpszClassName: class_name.as_ptr(),
        };
        let _class_atom = winuser::RegisterClassW(&class);
        let hwnd = winuser::CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            winuser::WS_OVERLAPPEDWINDOW | winuser::WS_VISIBLE,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );

        let (d3d_device, d3d_device_context) = d3d11::D3D11Device::create().unwrap();
        let dxgi_factory = dxgi::DXGIFactory2::create().unwrap();

        loop {
            let mut msg = std::mem::zeroed();
            // Note: we filter on hwnd so we get an error when the window is closed.
            if winuser::GetMessageW(&mut msg, hwnd, 0, 0) <= 0 {
                break;
            }
            winuser::TranslateMessage(&mut msg);
            winuser::DispatchMessageW(&mut msg);
        }
    }
}
