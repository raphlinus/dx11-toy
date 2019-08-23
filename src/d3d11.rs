use std::ptr::{null, null_mut};

use winapi::um::{d3d11, d3dcommon};

use wio::com::ComPtr;

use crate::util::{wrap, Error};

pub struct D3D11Device(ComPtr<d3d11::ID3D11Device>);
pub struct D3D11DeviceContext(ComPtr<d3d11::ID3D11DeviceContext>);
pub struct D3D11Texture2D(pub ComPtr<d3d11::ID3D11Texture2D>);
pub struct D3D11RenderTargetView(pub ComPtr<d3d11::ID3D11RenderTargetView>);

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

    pub fn as_raw(&self) -> *mut d3d11::ID3D11Device {
        self.0.as_raw()
    }

    pub fn create_render_target_view(
        &self,
        resource: &D3D11Texture2D,
    ) -> Result<D3D11RenderTargetView, Error> {
        unsafe {
            let mut ptr = null_mut();
            let hr = self
                .0
                .CreateRenderTargetView(resource.0.as_raw() as *mut _, null(), &mut ptr);
            wrap(hr, ptr, D3D11RenderTargetView)
        }
    }
}

impl D3D11DeviceContext {
    pub fn clear_render_target_view(&mut self, rtv: &mut D3D11RenderTargetView, color: &[f32; 4]) {
        unsafe {
            self.0.ClearRenderTargetView(rtv.0.as_raw(), color);
        }
    }
}
