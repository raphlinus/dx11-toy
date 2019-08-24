use std::ptr::{null, null_mut};

use winapi::um::{d3d11, d3dcommon, d3dcompiler};

use wio::com::ComPtr;

use crate::util::{wrap, Error};

pub struct D3D11Device(ComPtr<d3d11::ID3D11Device>);
pub struct D3D11DeviceContext(ComPtr<d3d11::ID3D11DeviceContext>);
pub struct D3D11Texture2D(pub ComPtr<d3d11::ID3D11Texture2D>);
pub struct D3D11RenderTargetView(pub ComPtr<d3d11::ID3D11RenderTargetView>);
pub struct D3D11Buffer(ComPtr<d3d11::ID3D11Buffer>);
pub struct D3DBlob(ComPtr<d3dcommon::ID3DBlob>);
pub struct D3D11VertexShader(ComPtr<d3d11::ID3D11VertexShader>);
pub struct D3D11PixelShader(ComPtr<d3d11::ID3D11PixelShader>);
pub struct D3D11InputLayout(ComPtr<d3d11::ID3D11InputLayout>);

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

    /// Create a buffer from data.
    ///
    /// This method doesn't expose all possible options and is not
    /// suitable for creating an uninitialized buffer.
    pub fn create_buffer_from_data<T>(
        &self,
        data: &[T],
        usage: d3d11::D3D11_USAGE,
        bind_flags: u32,
        cpu_access_flags: u32,
        misc_flags: u32,
        is_srv: bool,
    ) -> Result<D3D11Buffer, Error> {
        unsafe {
            let mut ptr = null_mut();
            let size = std::mem::size_of_val(data);
            // TODO: this should probably be try_from
            assert!(size <= 0xffff_ffff);
            let byte_stride = if is_srv { std::mem::size_of::<T>() } else { 0 };
            assert!(byte_stride <= 0xffff_ffff);
            let desc = d3d11::D3D11_BUFFER_DESC {
                ByteWidth: size as u32,
                Usage: usage,
                BindFlags: bind_flags,
                CPUAccessFlags: cpu_access_flags,
                MiscFlags: misc_flags,
                StructureByteStride: byte_stride as u32,
            };
            let srd = d3d11::D3D11_SUBRESOURCE_DATA {
                pSysMem: data.as_ptr() as *const _,
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            let hr = self.0.CreateBuffer(&desc, &srd, &mut ptr);
            wrap(hr, ptr, D3D11Buffer)
        }
    }

    pub fn create_vertex_shader(&self, shader: &D3DBlob) -> Result<D3D11VertexShader, Error> {
        unsafe {
            let mut ptr = null_mut();
            let hr = self.0.CreateVertexShader(
                shader.0.GetBufferPointer(),
                shader.0.GetBufferSize(),
                null_mut(),
                &mut ptr,
            );
            wrap(hr, ptr, D3D11VertexShader)
        }
    }

    pub fn create_pixel_shader(&self, shader: &D3DBlob) -> Result<D3D11PixelShader, Error> {
        unsafe {
            let mut ptr = null_mut();
            let hr = self.0.CreatePixelShader(
                shader.0.GetBufferPointer(),
                shader.0.GetBufferSize(),
                null_mut(),
                &mut ptr,
            );
            wrap(hr, ptr, D3D11PixelShader)
        }
    }

    pub fn create_input_layout(
        &self,
        descs: &[d3d11::D3D11_INPUT_ELEMENT_DESC],
        bytecode: &D3DBlob,
    ) -> Result<D3D11InputLayout, Error> {
        unsafe {
            assert!(descs.len() <= 0xffff_ffff);
            let mut ptr = null_mut();
            let hr = self.0.CreateInputLayout(
                descs.as_ptr(),
                descs.len() as u32,
                bytecode.0.GetBufferPointer(),
                bytecode.0.GetBufferSize(),
                &mut ptr,
            );
            wrap(hr, ptr, D3D11InputLayout)
        }
    }
}

impl D3D11DeviceContext {
    /// Sets a single render target, no stencil.
    pub fn set_render_target(&mut self, rtv: &D3D11RenderTargetView) {
        unsafe {
            let rtvs = [rtv.0.as_raw()];
            self.0.OMSetRenderTargets(1, rtvs.as_ptr(), null_mut());
        }
    }

    /// Sets a single viewport.
    pub fn set_viewport(&mut self, viewport: &d3d11::D3D11_VIEWPORT) {
        unsafe {
            self.0.RSSetViewports(1, viewport);
        }
    }

    pub fn clear_render_target_view(&mut self, rtv: &mut D3D11RenderTargetView, color: &[f32; 4]) {
        unsafe {
            self.0.ClearRenderTargetView(rtv.0.as_raw(), color);
        }
    }

    pub fn vs_set_shader(&mut self, shader: &D3D11VertexShader) {
        unsafe {
            self.0.VSSetShader(shader.0.as_raw(), null(), 0);
        }
    }

    pub fn ps_set_shader(&mut self, shader: &D3D11PixelShader) {
        unsafe {
            self.0.PSSetShader(shader.0.as_raw(), null(), 0);
        }
    }

    pub fn ia_set_vertex_buffer(&mut self, buf: &D3D11Buffer) {
        let stride = std::mem::size_of::<[f32; 3]>();
        let bufs = [buf.0.as_raw()];
        let strides = [stride as u32];
        let offsets = [0];
        unsafe {
            self.0
                .IASetVertexBuffers(0, 1, bufs.as_ptr(), strides.as_ptr(), offsets.as_ptr());
        }
    }

    pub fn ia_set_primitive_topology(&mut self, topology: d3d11::D3D11_PRIMITIVE_TOPOLOGY) {
        unsafe {
            self.0.IASetPrimitiveTopology(topology);
        }
    }

    pub fn ia_set_input_layout(&mut self, layout: &D3D11InputLayout) {
        unsafe {
            self.0.IASetInputLayout(layout.0.as_raw());
        }
    }

    pub fn draw(&mut self, count: u32, start: u32) {
        unsafe {
            self.0.Draw(count, start);
        }
    }
}

impl D3DBlob {
    pub fn compile_shader(
        hlsl: &str,
        target: &str,
        entry: &str,
        flags: u32,
    ) -> Result<D3DBlob, Error> {
        let target = [target, "\0"].concat();
        let entry = [entry, "\0"].concat();
        unsafe {
            let mut ptr = null_mut();
            let hr = d3dcompiler::D3DCompile(
                hlsl.as_ptr() as *const _,
                hlsl.len(),
                null(),
                null(),
                d3dcompiler::D3D_COMPILE_STANDARD_FILE_INCLUDE,
                entry.as_ptr() as *const _,
                target.as_ptr() as *const _,
                flags,
                0,
                &mut ptr,
                null_mut(),
            );
            //println!("blob: {} bytes", (*ptr).GetBufferSize());
            wrap(hr, ptr, D3DBlob)
        }
    }
}
