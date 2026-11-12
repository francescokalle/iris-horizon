use std::collections::HashMap;
use wgpu::*;
use crate::types::Image3DDescriptor;

pub struct TargetTexture {
    pub texture: Texture,
    pub view: TextureView,
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
}

pub struct TargetTexture3D {
    pub texture: Texture,
    pub view: TextureView,
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

pub struct RenderTargetManager {
    pub color_targets: HashMap<String, TargetTexture>,
    pub ping_pong_targets: HashMap<String, TargetTexture>,
    pub depth_targets: HashMap<String, TargetTexture>,
    pub images_3d: HashMap<String, TargetTexture3D>,
    pub shadow_color: Option<TargetTexture>,
    pub shadow_depth: Option<TargetTexture>,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl RenderTargetManager {
    pub fn new(
        device: &Device,
        screen_width: u32,
        screen_height: u32,
        custom_formats: &HashMap<String, TextureFormat>,
        custom_scales: &HashMap<String, (f32, f32)>,
        images_3d_desc: &HashMap<String, Image3DDescriptor>,
        shadow_resolution: u32,
    ) -> Self {
        let mut color_targets = HashMap::new();
        let mut ping_pong_targets = HashMap::new();

        // 1. Creazione colortex0 .. colortex15
        for i in 0..16 {
            let name = format!("colortex{}", i);
            let format = custom_formats.get(&name).copied().unwrap_or(TextureFormat::Rgba8Unorm);
            let (scale_x, scale_y) = custom_scales.get(&name).copied().unwrap_or((1.0, 1.0));
            let width = ((screen_width as f32 * scale_x).round() as u32).max(1);
            let height = ((screen_height as f32 * scale_y).round() as u32).max(1);

            let primary = Self::create_color_texture(device, width, height, format, &format!("{}_primary", name));
            let ping_pong = Self::create_color_texture(device, width, height, format, &format!("{}_pingpong", name));

            color_targets.insert(name.clone(), primary);
            ping_pong_targets.insert(name, ping_pong);
        }

        // 2. Creazione depthtex0, depthtex1, depthtex2
        let mut depth_targets = HashMap::new();
        for name in &["depthtex0", "depthtex1", "depthtex2"] {
            let dt = Self::create_depth_texture(device, screen_width, screen_height, name);
            depth_targets.insert(name.to_string(), dt);
        }

        // 3. Creazione delle texture 3D per Voxel Tracing (voxel_img, voxelCols, ecc.)
        let mut images_3d = HashMap::new();
        for (name, desc) in images_3d_desc {
            let tex3d = Self::create_3d_storage_texture(
                device,
                desc.width,
                desc.height,
                desc.depth,
                desc.format,
                &format!("Iris_Voxel3D_{}", name),
            );
            images_3d.insert(name.clone(), tex3d);
        }

        // Se lo shader non dichiara esplicitamente le texture voxel ma ha compute pass (es. voxel_data di default),
        // allochiamo un buffer voxel 3D standard 128x128x128
        if images_3d.is_empty() {
            let default_voxel = Self::create_3d_storage_texture(
                device,
                128,
                128,
                128,
                TextureFormat::Rgba16Float,
                "Iris_Voxel3D_Default",
            );
            images_3d.insert("voxel_data".to_string(), default_voxel);
        }

        // 4. Creazione shadow map e shadow color
        let shadow_depth = Some(Self::create_shadow_depth_texture(device, shadow_resolution, "shadowtex0"));
        let shadow_color = Some(Self::create_color_texture(
            device,
            shadow_resolution,
            shadow_resolution,
            TextureFormat::Rgba8Unorm,
            "shadowcolor0",
        ));

        Self {
            color_targets,
            ping_pong_targets,
            depth_targets,
            images_3d,
            shadow_color,
            shadow_depth,
            screen_width,
            screen_height,
        }
    }

    fn create_color_texture(
        device: &Device,
        width: u32,
        height: u32,
        format: TextureFormat,
        label: &str,
    ) -> TargetTexture {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::STORAGE_BINDING
                | TextureUsages::COPY_SRC
                | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());

        TargetTexture {
            texture,
            view,
            format,
            width,
            height,
        }
    }

    fn create_depth_texture(device: &Device, width: u32, height: u32, label: &str) -> TargetTexture {
        let format = TextureFormat::Depth32Float;
        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());

        TargetTexture {
            texture,
            view,
            format,
            width,
            height,
        }
    }

    fn create_shadow_depth_texture(device: &Device, resolution: u32, label: &str) -> TargetTexture {
        let format = TextureFormat::Depth32Float;
        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: Extent3d {
                width: resolution,
                height: resolution,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());

        TargetTexture {
            texture,
            view,
            format,
            width: resolution,
            height: resolution,
        }
    }

    fn create_3d_storage_texture(
        device: &Device,
        width: u32,
        height: u32,
        depth: u32,
        format: TextureFormat,
        label: &str,
    ) -> TargetTexture3D {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: depth,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D3,
            format,
            usage: TextureUsages::STORAGE_BINDING
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());

        TargetTexture3D {
            texture,
            view,
            format,
            width,
            height,
            depth,
        }
    }
}
