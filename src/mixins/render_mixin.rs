use parking_lot::RwLock;
use std::sync::Arc;
use crate::pipeline::IrisPipelineOrchestrator;
use crate::uniforms::IrisGlobalUBOData;

pub static IRIS_ORCHESTRATOR: RwLock<Option<IrisPipelineOrchestrator>> = RwLock::new(None);

pub struct WorldRendererMixin;

impl WorldRendererMixin {
    /// Iniezione a monte del rendering del mondo
    pub fn on_render_world_start(
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        uniforms: &IrisGlobalUBOData,
    ) {
        let mut guard = IRIS_ORCHESTRATOR.write();
        if let Some(ref mut orchestrator) = *guard {
            orchestrator.uniform_provider.update(queue, uniforms);
        }
    }

    /// Iniezione a valle del rendering del mondo per composite e post-processing
    pub fn on_render_world_end(
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        swapchain_view: &wgpu::TextureView,
        uniforms: &IrisGlobalUBOData,
    ) {
        let mut guard = IRIS_ORCHESTRATOR.write();
        if let Some(ref mut orchestrator) = *guard {
            orchestrator.render_frame(encoder, swapchain_view, uniforms, queue);
        }
    }
}

pub struct ChunkRendererMixin;

impl ChunkRendererMixin {
    /// Hook per sostituire i singoli draw call dei chunk con batch indiretto
    pub fn on_draw_chunk_mesh(
        index_count: u32,
        first_index: u32,
        base_vertex: i32,
    ) {
        // I chunk vengono registrati nell'IndirectBatchRenderer
    }
}
