use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use wgpu::*;

use crate::compiler::{IrisGlslTranspiler, IrisPipelineLayoutBuilder};
use crate::framebuffer::{PingPongController, RenderTargetManager};
use crate::optimizations::{IndirectBatchRenderer, PersistentPipelineCache};
use crate::parser::ShaderPack;
use crate::pipeline::pass::IrisRenderPass;
use crate::types::{ComputeDispatchSize, PassType, ShaderStage};
use crate::uniforms::{IrisGlobalUBOData, IrisUniformProvider};

pub struct IrisPipelineOrchestrator {
    pub pack: ShaderPack,
    pub targets: RenderTargetManager,
    pub uniform_provider: IrisUniformProvider,
    pub transpiler: IrisGlslTranspiler,
    pub pipeline_cache: PersistentPipelineCache,
    pub indirect_batcher: IndirectBatchRenderer,
    pub raster_passes: HashMap<PassType, IrisRenderPass>,
    pub compute_passes: HashMap<PassType, (ComputePipeline, ComputeDispatchSize)>,
    pub main_bind_group_layout: BindGroupLayout,
    pub texture_bind_group_layout: BindGroupLayout,
    pub main_bind_group: BindGroup,
    pub texture_bind_group: BindGroup,
    pub linear_sampler: Sampler,
    pub nearest_sampler: Sampler,
    pub shadow_sampler: Sampler,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl IrisPipelineOrchestrator {
    pub fn new(
        device: &Device,
        queue: &Queue,
        pack: ShaderPack,
        width: u32,
        height: u32,
        cache_path: &Path,
    ) -> Result<Self, String> {
        let targets = RenderTargetManager::new(
            device,
            width,
            height,
            &pack.properties.buffer_formats,
            &pack.properties.buffer_sizes,
            &pack.properties.images_3d,
            pack.properties.shadow_map_resolution,
        );

        let uniform_provider = IrisUniformProvider::new(device);
        let transpiler = IrisGlslTranspiler::new();
        let pipeline_cache = PersistentPipelineCache::new(device, cache_path.to_path_buf());
        let indirect_batcher = IndirectBatchRenderer::new(device, 4096);

        let linear_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Iris_Linear_Sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        });

        let nearest_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Iris_Nearest_Sampler"),
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let shadow_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Iris_Shadow_Comparison_Sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            compare: Some(CompareFunction::LessEqual),
            ..Default::default()
        });

        let main_bind_group_layout = IrisPipelineLayoutBuilder::create_main_bind_group_layout(device);
        let texture_bind_group_layout = IrisPipelineLayoutBuilder::create_texture_bind_group_layout(device);

        let main_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Iris_Main_BindGroup"),
            layout: &main_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_provider.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&linear_sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&nearest_sampler),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Sampler(&shadow_sampler),
                },
            ],
        });

        // Costruzione Texture BindGroup (2D Target + 3D Voxel Storage Textures)
        let mut tex_entries = Vec::new();
        for i in 0..16 {
            let name = format!("colortex{}", i);
            if let Some(target) = targets.color_targets.get(&name) {
                tex_entries.push(BindGroupEntry {
                    binding: i as u32,
                    resource: BindingResource::TextureView(&target.view),
                });
            }
        }

        for (idx, name) in ["depthtex0", "depthtex1", "depthtex2"].iter().enumerate() {
            if let Some(dt) = targets.depth_targets.get(*name) {
                tex_entries.push(BindGroupEntry {
                    binding: (16 + idx) as u32,
                    resource: BindingResource::TextureView(&dt.view),
                });
            }
        }

        // Texture 3D Voxel bindings (19, 20, 21)
        let default_view_3d = targets.images_3d.values().next().map(|t| &t.view).unwrap();
        let voxel_img_view = targets.images_3d.get("voxel_img").map(|t| &t.view).unwrap_or(default_view_3d);
        let voxel_cols_view = targets.images_3d.get("voxelCols").map(|t| &t.view).unwrap_or(default_view_3d);
        let voxel_data_view = targets.images_3d.get("voxel_data").map(|t| &t.view).unwrap_or(default_view_3d);

        tex_entries.push(BindGroupEntry {
            binding: 19,
            resource: BindingResource::TextureView(voxel_img_view),
        });
        tex_entries.push(BindGroupEntry {
            binding: 20,
            resource: BindingResource::TextureView(voxel_cols_view),
        });
        tex_entries.push(BindGroupEntry {
            binding: 21,
            resource: BindingResource::TextureView(voxel_data_view),
        });

        let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Iris_Texture_BindGroup"),
            layout: &texture_bind_group_layout,
            entries: &tex_entries,
        });

        let mut orchestrator = Self {
            pack,
            targets,
            uniform_provider,
            transpiler,
            pipeline_cache,
            indirect_batcher,
            raster_passes: HashMap::new(),
            compute_passes: HashMap::new(),
            main_bind_group_layout,
            texture_bind_group_layout,
            main_bind_group,
            texture_bind_group,
            linear_sampler,
            nearest_sampler,
            shadow_sampler,
            screen_width: width,
            screen_height: height,
        };

        orchestrator.compile_all_passes(device)?;

        Ok(orchestrator)
    }

    /// Compila sia i pass raster che i compute pass (.csh per voxel path tracing)
    pub fn compile_all_passes(&mut self, device: &Device) -> Result<(), String> {
        let pipeline_layout = IrisPipelineLayoutBuilder::create_pipeline_layout(
            device,
            &self.main_bind_group_layout,
            &self.texture_bind_group_layout,
        );

        for (pass_type, source) in &self.pack.programs {
            // 1. Compilazione Compute Shader (.csh)
            if let Some(ref csh) = source.compute_source {
                if let Ok(comp_module) = self.transpiler.transpile_glsl(csh, ShaderStage::Compute) {
                    if let Ok(comp_wgsl) = self.transpiler.emit_wgsl(&comp_module) {
                        let comp_shader = device.create_shader_module(ShaderModuleDescriptor {
                            label: Some(&format!("{}_compute", pass_type.file_prefix())),
                            source: ShaderSource::Wgsl(comp_wgsl.into()),
                        });

                        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
                            label: Some(&format!("{}_compute_pipeline", pass_type.file_prefix())),
                            layout: Some(&pipeline_layout),
                            module: &comp_shader,
                            entry_point: Some("main"),
                            compilation_options: PipelineCompilationOptions::default(),
                            cache: self.pipeline_cache.get_cache(),
                        });

                        let dispatch_size = source.compute_dispatch.unwrap_or(ComputeDispatchSize::ScreenRelative {
                            scale_x: 0.5,
                            scale_y: 0.5,
                            local_x: 16,
                            local_y: 16,
                        });

                        self.compute_passes.insert(*pass_type, (compute_pipeline, dispatch_size));
                    }
                }
            }

            // 2. Compilazione Raster Shader (.vsh / .fsh)
            if let (Some(ref vsh), Some(ref fsh)) = (&source.vertex_source, &source.fragment_source) {
                if let (Ok(vert_module), Ok(frag_module)) = (
                    self.transpiler.transpile_glsl(vsh, ShaderStage::Vertex),
                    self.transpiler.transpile_glsl(fsh, ShaderStage::Fragment),
                ) {
                    if let (Ok(vert_wgsl), Ok(frag_wgsl)) = (
                        self.transpiler.emit_wgsl(&vert_module),
                        self.transpiler.emit_wgsl(&frag_module),
                    ) {
                        let vert_shader = device.create_shader_module(ShaderModuleDescriptor {
                            label: Some(&format!("{}_vert", pass_type.file_prefix())),
                            source: ShaderSource::Wgsl(vert_wgsl.into()),
                        });

                        let frag_shader = device.create_shader_module(ShaderModuleDescriptor {
                            label: Some(&format!("{}_frag", pass_type.file_prefix())),
                            source: ShaderSource::Wgsl(frag_wgsl.into()),
                        });

                        let targets_format = match pass_type {
                            PassType::Final => TextureFormat::Bgra8UnormSrgb,
                            _ => self.targets.color_targets.get("colortex0").map(|t| t.format).unwrap_or(TextureFormat::Rgba8Unorm),
                        };

                        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                            label: Some(&format!("{}_pipeline", pass_type.file_prefix())),
                            layout: Some(&pipeline_layout),
                            vertex: VertexState {
                                module: &vert_shader,
                                entry_point: Some("main"),
                                buffers: &[],
                                compilation_options: PipelineCompilationOptions::default(),
                            },
                            fragment: Some(FragmentState {
                                module: &frag_shader,
                                entry_point: Some("main"),
                                targets: &[Some(ColorTargetState {
                                    format: targets_format,
                                    blend: Some(BlendState::REPLACE),
                                    write_mask: ColorWrites::ALL,
                                })],
                                compilation_options: PipelineCompilationOptions::default(),
                            }),
                            primitive: PrimitiveState {
                                topology: PrimitiveTopology::TriangleList,
                                cull_mode: None,
                                ..Default::default()
                            },
                            depth_stencil: None,
                            multisample: MultisampleState::default(),
                            multiview: None,
                            cache: self.pipeline_cache.get_cache(),
                        });

                        let pass = IrisRenderPass::new(*pass_type, pipeline, self.main_bind_group.clone());
                        self.raster_passes.insert(*pass_type, pass);
                    }
                }
            }
        }

        Ok(())
    }

    /// Esegue il dispatch di un compute pass per voxel path tracing
    fn dispatch_compute_pass(&self, encoder: &mut CommandEncoder, pass_type: &PassType) {
        if let Some((pipeline, dispatch_size)) = self.compute_passes.get(pass_type) {
            let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some(&format!("Iris_Compute_{:?}", pass_type)),
                timestamp_writes: None,
            });

            cpass.set_pipeline(pipeline);
            cpass.set_bind_group(0, &self.main_bind_group, &[]);
            cpass.set_bind_group(1, &self.texture_bind_group, &[]);

            match *dispatch_size {
                ComputeDispatchSize::Fixed { x, y, z } => {
                    cpass.dispatch_workgroups(x, y, z);
                }
                ComputeDispatchSize::ScreenRelative { scale_x, scale_y, local_x, local_y } => {
                    let w = ((self.screen_width as f32 * scale_x).ceil() as u32).max(1);
                    let h = ((self.screen_height as f32 * scale_y).ceil() as u32).max(1);
                    let groups_x = (w + local_x - 1) / local_x;
                    let groups_y = (h + local_y - 1) / local_y;
                    cpass.dispatch_workgroups(groups_x, groups_y, 1);
                }
            }
        }
    }

    /// Esegue l'intero grafo di rendering di Iris con voxel compute passes ottimizzati
    pub fn render_frame(
        &mut self,
        encoder: &mut CommandEncoder,
        swapchain_view: &TextureView,
        uniform_data: &IrisGlobalUBOData,
        queue: &Queue,
    ) {
        // 1. Aggiorna l'UBO std140
        self.uniform_provider.update(queue, uniform_data);

        // 2. Pass Ombre Raster
        if let Some(_shadow_pass) = self.raster_passes.get(&PassType::Shadow) {
            if let Some(ref shadow_depth) = self.targets.shadow_depth {
                let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Iris_Shadow_Pass"),
                    color_attachments: &[],
                    depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                        view: &shadow_depth.view,
                        depth_ops: Some(Operations {
                            load: LoadOp::Clear(1.0),
                            store: StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                self.indirect_batcher.execute(&mut rpass);
            }
        }

        // 3. Pass Compute Ombre / Voxelization (shadowcomp, shadowcomp1, shadowcomp2)
        // Calcola la propagazione della luce nei voxel 3D generati dall'illuminazione dei blocchi
        self.dispatch_compute_pass(encoder, &PassType::ShadowComp);
        self.dispatch_compute_pass(encoder, &PassType::ShadowComp1);
        self.dispatch_compute_pass(encoder, &PassType::ShadowComp2);

        // 4. Pass Compute Prepare (prepare4 per filtri e denoiser voxel)
        self.dispatch_compute_pass(encoder, &PassType::Prepare4);

        // 5. Pass Compute Deferred (deferred1: Raymarching dei voxel a risoluzione ottimizzata)
        self.dispatch_compute_pass(encoder, &PassType::Deferred1);

        // 6. Pass Raster Deferred & Composite con ping-ponging
        let composite_passes = [
            PassType::Composite,
            PassType::Composite1,
            PassType::Composite2,
            PassType::Composite3,
        ];

        for pass_type in composite_passes {
            if let Some(pass) = self.raster_passes.get(&pass_type) {
                if let Some(target) = self.targets.color_targets.get("colortex0") {
                    let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                        label: Some(&format!("Iris_{:?}", pass_type)),
                        color_attachments: &[Some(RenderPassColorAttachment {
                            view: &target.view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Load,
                                store: StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    rpass.set_pipeline(&pass.pipeline);
                    rpass.set_bind_group(0, &self.main_bind_group, &[]);
                    rpass.set_bind_group(1, &self.texture_bind_group, &[]);
                    rpass.draw(0..3, 0..1); // Full-screen triangle
                }

                PingPongController::swap(&mut self.targets, "colortex0");
            }
        }

        // 7. Pass Finale (output direttamente sulla vista dello swapchain)
        if let Some(final_pass) = self.raster_passes.get(&PassType::Final) {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Iris_Final_Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: swapchain_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&final_pass.pipeline);
            rpass.set_bind_group(0, &self.main_bind_group, &[]);
            rpass.set_bind_group(1, &self.texture_bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }
    }
}
