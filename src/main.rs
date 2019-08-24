use std::ptr::{null, null_mut};

use winapi::shared::dxgi::DXGI_SWAP_EFFECT_FLIP_DISCARD;
use winapi::shared::{dxgi1_2, dxgiformat, dxgitype, minwindef};
use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT;
use winapi::um::d3dcommon::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use winapi::um::d3d11::D3D11_INPUT_PER_VERTEX_DATA;
use winapi::um::{wingdi, winuser};

use wio::wide::ToWide;

mod d3d11;
mod dxgi;
mod util;

use d3d11::D3DBlob;

const VERTEX_SHADER_HLSL: &str = r#"
float4 main( float4 pos : POSITION ) : SV_POSITION
{
	return pos;
}
"#;

const PIXEL_SHADER_HLSL: &str = r#"
float4 main() : SV_TARGET
{
	return float4(1.0f, 1.0f, 1.0f, 1.0f);
}
"#;

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
            winuser::WS_OVERLAPPEDWINDOW,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );

        let (d3d_device, mut device_context) = d3d11::D3D11Device::create().unwrap();
        let dxgi_factory = dxgi::DXGIFactory2::create().unwrap();
        let desc = dxgi1_2::DXGI_SWAP_CHAIN_DESC1 {
            Width: 0,
            Height: 0,
            AlphaMode: dxgi1_2::DXGI_ALPHA_MODE_IGNORE,
            BufferCount: 2,
            Format: dxgiformat::DXGI_FORMAT_B8G8R8A8_UNORM,
            Flags: 0,
            BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Scaling: dxgi1_2::DXGI_SCALING_STRETCH,
            Stereo: minwindef::FALSE,
            // Note: FLIP_DISCARD is Windows 8 only; negotiate
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
        };
        let mut swap_chain = dxgi_factory
            .create_swapchain_for_hwnd(&d3d_device, hwnd, &desc)
            .unwrap();

        let vertex_shader_bc =
            D3DBlob::compile_shader(VERTEX_SHADER_HLSL, "vs_5_0", "main", 0).unwrap();
        let vertex_shader = d3d_device.create_vertex_shader(&vertex_shader_bc).unwrap();
        let pixel_shader_bc =
            D3DBlob::compile_shader(PIXEL_SHADER_HLSL, "ps_5_0", "main", 0).unwrap();
        let pixel_shader = d3d_device.create_pixel_shader(&pixel_shader_bc).unwrap();

        let ieds = [
            winapi::um::d3d11::D3D11_INPUT_ELEMENT_DESC {
                SemanticName: "POSITION\0".as_ptr() as *const _,
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];
        let input_layout = d3d_device.create_input_layout(&ieds, &vertex_shader_bc).unwrap();

        device_context.vs_set_shader(&vertex_shader);
        device_context.ps_set_shader(&pixel_shader);
        device_context.ia_set_input_layout(&input_layout);

        let viewport = winapi::um::d3d11::D3D11_VIEWPORT {
            TopLeftX: 0.,
            TopLeftY: 0.,
            Width: 800., // TODO set dynamically
            Height: 600., // TODO set dynamically
            MinDepth: 0.,
            MaxDepth: 0.,
        };
        device_context.set_viewport(&viewport);

        let buf = swap_chain.get_buffer(0).unwrap();
        let mut rtv = d3d_device.create_render_target_view(&buf).unwrap();
        device_context.set_render_target(&rtv);
        device_context.clear_render_target_view(&mut rtv, &[0.0, 0.2, 0.4, 1.0]);

        let vertices = [
            [0.0f32, 0.5f32, 0.0f32],
            [0.45f32, -0.5f32, 0.0f32],
            [-0.45f32, -0.5f32, 0.0f32],
        ];
        let vertex_buf = d3d_device
            .create_buffer_from_data(
                &vertices,
                winapi::um::d3d11::D3D11_USAGE_DEFAULT,
                winapi::um::d3d11::D3D11_BIND_VERTEX_BUFFER,
                0,
                0,
                false,
            )
            .unwrap();
        device_context.ia_set_vertex_buffer(&vertex_buf);
        device_context.ia_set_primitive_topology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        device_context.draw(3, 0);

        swap_chain.present(1, 0).unwrap();
        // Show window after first present to avoid flash of background color.
        //
        // I did some research and found that this is very conservative, lots of
        // other combinations eliminate the flash, including just creating the
        // d3d11 factory before creating the window (and using WS_VISIBLE). But
        // this seems robust and doesn't seem to have a significant downside.
        winuser::ShowWindow(hwnd, winuser::SW_SHOWNORMAL);

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
