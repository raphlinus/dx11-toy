use std::ptr::{null, null_mut};

use winapi::shared::{dxgi, dxgi1_2, windef};
use winapi::um::d3d11;
use winapi::Interface;

use wio::com::ComPtr;

use crate::d3d11::{D3D11Device, D3D11Texture2D};
use crate::util::{wrap, wrap_unit, Error};

pub struct DXGIFactory2(ComPtr<dxgi1_2::IDXGIFactory2>);
pub struct DXGISwapChain1(ComPtr<dxgi1_2::IDXGISwapChain1>);

impl DXGIFactory2 {
    pub fn create() -> Result<DXGIFactory2, Error> {
        unsafe {
            let mut ptr = null_mut();
            let hr = dxgi::CreateDXGIFactory1(
                &dxgi1_2::IDXGIFactory2::uuidof(),
                &mut ptr as *mut _ as *mut _,
            );
            wrap(hr, ptr, DXGIFactory2)
        }
    }

    pub unsafe fn create_swapchain_for_hwnd(
        &self,
        device: &D3D11Device,
        hwnd: windef::HWND,
        desc: &dxgi1_2::DXGI_SWAP_CHAIN_DESC1,
    ) -> Result<DXGISwapChain1, Error> {
        let mut ptr = null_mut();
        let hr = self.0.CreateSwapChainForHwnd(
            device.as_raw() as *mut _,
            hwnd,
            desc,
            null(),
            null_mut(),
            &mut ptr as *mut _ as *mut _,
        );
        wrap(hr, ptr, DXGISwapChain1)
    }
}

impl DXGISwapChain1 {
    pub fn present(&self, sync_interval: u32, flags: u32) -> Result<(), Error> {
        unsafe {
            let hr = self.0.Present(sync_interval, flags);
            wrap_unit(hr)
        }
    }

    pub fn get_buffer(&mut self, buffer: u32) -> Result<D3D11Texture2D, Error> {
        unsafe {
            let mut ptr = null_mut();
            let hr = self.0.GetBuffer(
                buffer,
                &d3d11::ID3D11Texture2D::uuidof(),
                &mut ptr as *mut _ as *mut _,
            );
            wrap(hr, ptr, D3D11Texture2D)
        }
    }
}
