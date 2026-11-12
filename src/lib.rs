//! # Iris for Horizon (`iris-horizon`)
//!
//! Porting ad alte prestazioni delle Iris / OptiFine Shaders per il motore
//! Minecraft in Rust **Horizon** basato su `wgpu`.

pub mod types;
pub mod parser;
pub mod compiler;
pub mod framebuffer;
pub mod uniforms;
pub mod optimizations;
pub mod pipeline;
pub mod mixins;

// Re-export dei componenti primari
pub use types::{PassType, ShaderProgramSource, ShaderStage};
pub use parser::{ShaderPack, ShaderProperties, PropertiesParser, ShaderPreprocessor};
pub use compiler::{IrisGlslTranspiler, IrisPipelineLayoutBuilder};
pub use framebuffer::{RenderTargetManager, PingPongController, TargetTexture};
pub use uniforms::{IrisGlobalUBOData, IrisUniformProvider};
pub use optimizations::{
    DrawIndexedIndirectArgs, IndirectBatchRenderer, PersistentPipelineCache,
    ShadowFrustumCuller, UniformRingBuffer,
};
pub use pipeline::{IrisPipelineOrchestrator, IrisRenderPass};
pub use mixins::{WorldRendererMixin, ChunkRendererMixin, IRIS_ORCHESTRATOR};

use std::path::Path;
use log::info;

/// Inizializza il runtime Iris in Horizon con uno specifico shader pack
pub fn load_shader_pack(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pack_dir: &Path,
    screen_width: u32,
    screen_height: u32,
    pipeline_cache_file: &Path,
) -> Result<(), String> {
    info!("Caricamento Shader Pack Iris da: {:?}", pack_dir);

    let pack = ShaderPack::load_from_directory(pack_dir)?;
    info!("Shader pack caricato: '{}'", pack.name);

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

    info!("Iris Shaders abilitate con successo!");
    Ok(())
}

/// Disabilita gli shader e ripristina la pipeline vanilla di Horizon
pub fn disable_shader_pack() {
    let mut guard = IRIS_ORCHESTRATOR.write();
    *guard = None;
    info!("Iris Shaders disabilitate, pipeline vanilla ripristinata.");
}
