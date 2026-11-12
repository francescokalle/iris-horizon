//! # Iris for Horizon (`iris-horizon`)
//!
//! High-performance pure-Rust port of Iris & OptiFine Shaders for the
//! **Horizon** Minecraft engine based on `wgpu`.

pub mod types;
pub mod parser;
pub mod compiler;
pub mod framebuffer;
pub mod uniforms;
pub mod optimizations;
pub mod pipeline;
pub mod mixins;

// Re-export primary components
pub use types::{ComputeDispatchSize, Image3DDescriptor, PassType, ShaderProgramSource, ShaderStage};
pub use parser::{PropertiesParser, ShaderPack, ShaderPreprocessor, ShaderProperties};
pub use compiler::{IrisGlslTranspiler, IrisPipelineLayoutBuilder};
pub use framebuffer::{PingPongController, RenderTargetManager, TargetTexture, TargetTexture3D};
pub use uniforms::{IrisGlobalUBOData, IrisUniformProvider};
pub use optimizations::{
    DrawIndexedIndirectArgs, IndirectBatchRenderer, PersistentPipelineCache,
    ShadowFrustumCuller, UniformRingBuffer,
};
pub use pipeline::{IrisPipelineOrchestrator, IrisRenderPass};
pub use mixins::{ChunkRendererMixin, IRIS_ORCHESTRATOR, WorldRendererMixin};

use std::path::Path;
use log::info;

/// Initializes the Iris runtime in Horizon with a specific shaderpack
pub fn load_shader_pack(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pack_dir: &Path,
    screen_width: u32,
    screen_height: u32,
    pipeline_cache_file: &Path,
) -> Result<(), String> {
    info!("Loading Iris Shader Pack from: {:?}", pack_dir);

    let pack = ShaderPack::load_from_directory(pack_dir)?;
    info!("Loaded shader pack: '{}'", pack.name);

    let orchestrator = IrisPipelineOrchestrator::new(
        device,
        queue,
        pack,
        screen_width,
        screen_height,
        pipeline_cache_file,
    )?;

    let mut guard = IRIS_ORCHESTRATOR.write();
    *guard = Some(orchestrator);

    info!("Iris Shaders enabled successfully!");
    Ok(())
}

/// Disables shaders and restores vanilla Horizon rendering pipeline
pub fn disable_shader_pack() {
    let mut guard = IRIS_ORCHESTRATOR.write();
    *guard = None;
    info!("Iris Shaders disabled, vanilla pipeline restored.");
}
