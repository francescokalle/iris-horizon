use wgpu::*;
use crate::types::PassType;

pub struct IrisRenderPass {
    pub pass_type: PassType,
    pub pipeline: RenderPipeline,
    pub bind_group: BindGroup,
}

impl IrisRenderPass {
    pub fn new(pass_type: PassType, pipeline: RenderPipeline, bind_group: BindGroup) -> Self {
        Self {
            pass_type,
            pipeline,
            bind_group,
        }
    }
}
