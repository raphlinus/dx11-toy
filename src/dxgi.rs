use std::ptr::null_mut;

use winapi::shared::{dxgi, dxgi1_2};
use winapi::Interface;

use wio::com::ComPtr;

use crate::util::{wrap, Error};

pub struct DXGIFactory2(ComPtr<dxgi1_2::IDXGIFactory2>);

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
}
