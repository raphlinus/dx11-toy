use std::ptr::null_mut;

use winapi::um::{d3d11, d3dcommon};

use wio::com::ComPtr;

use crate::util::{wrap, Error};

pub struct D3D11Device(ComPtr<d3d11::ID3D11Device>);
pub struct D3D11DeviceContext(ComPtr<d3d11::ID3D11DeviceContext>);

impl D3D11Device {
    // This function only supports a fraction of available options.
    pub fn create() -> Result<(D3D11Device, D3D11DeviceContext), Error> {
        unsafe {
            let mut ptr = null_mut();
            let mut ctx_ptr = null_mut();
            let hr = d3d11::D3D11CreateDevice(
                null_mut(), /* adapter */
                d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
                null_mut(), /* module */
                d3d11::D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                null_mut(), /* feature levels */
                0,
                d3d11::D3D11_SDK_VERSION,
                &mut ptr,
                null_mut(), /* feature level */
                &mut ctx_ptr,
            );
            let device = wrap(hr, ptr, D3D11Device)?;
            let device_ctx = wrap(hr, ctx_ptr, D3D11DeviceContext)?;
            Ok((device, device_ctx))
        }
    }
}
