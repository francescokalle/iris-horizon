use std::sync::Arc;
use wgpu::*;

pub struct IrisPipelineLayoutBuilder;

impl IrisPipelineLayoutBuilder {
    /// Crea il layout del BindGroup primario per gli Uniform e Sampler Iris
    pub fn create_main_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Iris_Main_BindGroupLayout"),
            entries: &[
                // Binding 0: IrisGlobalUBO
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Binding 1: Sampler lineare
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // Binding 2: Sampler nearest (per depth e pixel-art textures)
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
                // Binding 3: Shadow comparison sampler
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        })
    }

    /// Crea il layout del BindGroup per i render target 2D (colortex0..15, depthtex0..2)
    /// e le texture 3D storage per il Voxel Tracing (binding 19, 20, 21)
    pub fn create_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
        let mut entries = Vec::new();
        // Binding 0..15: colortex0 fino a colortex15
        for i in 0..16 {
            entries.push(BindGroupLayoutEntry {
                binding: i,
                visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });
        }

        // Binding 16..18: depthtex0, depthtex1, depthtex2
        for i in 16..19 {
            entries.push(BindGroupLayoutEntry {
                binding: i,
                visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Depth,
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });
        }

        // Binding 19: voxel_img 3D texture (R8Uint o R8Unorm)
        entries.push(BindGroupLayoutEntry {
            binding: 19,
            visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadWrite,
                format: TextureFormat::R8Uint,
                view_dimension: TextureViewDimension::D3,
            },
            count: None,
        });

        // Binding 20: voxelCols 3D texture (R32Sint)
        entries.push(BindGroupLayoutEntry {
            binding: 20,
            visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadWrite,
                format: TextureFormat::R32Sint,
                view_dimension: TextureViewDimension::D3,
            },
            count: None,
        });

        // Binding 21: voxel_data 3D texture generale (Rgba16Float)
        entries.push(BindGroupLayoutEntry {
            binding: 21,
            visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadWrite,
                format: TextureFormat::Rgba16Float,
                view_dimension: TextureViewDimension::D3,
            },
            count: None,
        });

        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Iris_Textures_BindGroupLayout"),
            entries: &entries,
        })
    }

    /// Crea il PipelineLayout combinando Uniform, Sampler e Texture targets
    pub fn create_pipeline_layout(
        device: &Device,
        main_layout: &BindGroupLayout,
        texture_layout: &BindGroupLayout,
    ) -> PipelineLayout {
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Iris_PipelineLayout"),
            bind_group_layouts: &[main_layout, texture_layout],
            push_constant_ranges: &[],
        })
    }
}
