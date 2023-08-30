use windows::Win32::Graphics::{
    Direct3D::D3D11_SRV_DIMENSION_TEXTURE2D,
    Direct3D11::{
        ID3D11Device, ID3D11ShaderResourceView, ID3D11Texture2D, D3D11_BIND_SHADER_RESOURCE,
        D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_SHADER_RESOURCE_VIEW_DESC_0, D3D11_SUBRESOURCE_DATA,
        D3D11_TEX2D_SRV, D3D11_TEXTURE2D_DESC, D3D11_USAGE,
    },
    Dxgi::Common::{DXGI_FORMAT, DXGI_SAMPLE_DESC},
};

/// Creates a 2-dimensional texture with the given description and subresource data.
pub fn create_texture2d(
    device: &ID3D11Device,
    desc: &D3D11_TEXTURE2D_DESC,
    data: &D3D11_SUBRESOURCE_DATA,
) -> windows::core::Result<ID3D11Texture2D> {
    let mut id = None;
    unsafe { device.CreateTexture2D(desc, Some(data), Some(&mut id)) }?;
    Ok(id.expect("CreateTexture2D failed without error"))
}

/// Creates a 2-dimensional texture from in-memory data.
pub fn create_texture2d_from_mem(
    device: &ID3D11Device,
    data: &[u8],
    width: u32,
    height: u32,
    pitch: u32,
    format: DXGI_FORMAT,
    usage: D3D11_USAGE,
) -> windows::core::Result<ID3D11Texture2D> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: format,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: usage,
        BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as _,
        ..Default::default()
    };
    let sub_resource = D3D11_SUBRESOURCE_DATA {
        pSysMem: data.as_ptr() as _,
        SysMemPitch: pitch,
        SysMemSlicePitch: 0,
    };
    create_texture2d(device, &desc, &sub_resource)
}

/// Creates a shader resource view for the given 2-dimensional texture.
pub fn create_texture2d_view(
    device: &ID3D11Device,
    texture: &ID3D11Texture2D,
    format: DXGI_FORMAT,
) -> windows::core::Result<ID3D11ShaderResourceView> {
    let mut id = None;
    let desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
        Format: format,
        ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
        Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
            Texture2D: D3D11_TEX2D_SRV {
                MostDetailedMip: 0,
                MipLevels: 1,
            },
        },
    };
    unsafe { device.CreateShaderResourceView(texture, Some(&desc), Some(&mut id)) }?;
    Ok(id.expect("CreateShaderResourceView failed without error"))
}
