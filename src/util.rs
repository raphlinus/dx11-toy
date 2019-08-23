use winapi::shared::winerror::{HRESULT, SUCCEEDED};
use winapi::Interface;

use wio::com::ComPtr;

#[derive(Debug)]
pub struct Error(HRESULT);

pub unsafe fn wrap<T, U, F>(hr: HRESULT, ptr: *mut T, f: F) -> Result<U, Error>
where
    F: Fn(ComPtr<T>) -> U,
    T: Interface,
{
    if SUCCEEDED(hr) {
        Ok(f(ComPtr::from_raw(ptr)))
    } else {
        Err(Error(hr))
    }
}

pub fn wrap_unit(hr: HRESULT) -> Result<(), Error> {
    if SUCCEEDED(hr) {
        Ok(())
    } else {
        Err(Error(hr))
    }
}
