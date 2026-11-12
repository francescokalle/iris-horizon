use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use wgpu::*;

/// Struttura compatibile con WebGPU / Vulkan per draw_indexed_indirect
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct DrawIndexedIndirectArgs {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub base_vertex: i32,
    pub first_instance: u32,
}

pub struct IndirectBatchRenderer {
    pub command_buffer: Buffer,
    pub capacity: usize,
    pub active_draw_count: u32,
}

impl IndirectBatchRenderer {
    pub fn new(device: &Device, initial_capacity: usize) -> Self {
        let initial_size = (initial_capacity * std::mem::size_of::<DrawIndexedIndirectArgs>()) as BufferAddress;
        let command_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Iris_Indirect_Draw_Buffer"),
            size: initial_size.max(256),
            usage: BufferUsages::INDIRECT | BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        Self {
            command_buffer,
            capacity: initial_capacity,
            active_draw_count: 0,
        }
    }

    /// Carica una serie di chunk draw calls in un unico buffer indiretto
    pub fn upload_batches(&mut self, device: &Device, queue: &Queue, batches: &[DrawIndexedIndirectArgs]) {
        self.active_draw_count = batches.len() as u32;
        let required_size = (batches.len() * std::mem::size_of::<DrawIndexedIndirectArgs>()) as BufferAddress;

        if batches.len() > self.capacity {
            self.capacity = batches.len().next_power_of_two();
            self.command_buffer = device.create_buffer(&BufferDescriptor {
                label: Some("Iris_Indirect_Draw_Buffer_Resized"),
                size: (self.capacity * std::mem::size_of::<DrawIndexedIndirectArgs>()) as BufferAddress,
                usage: BufferUsages::INDIRECT | BufferUsages::COPY_DST | BufferUsages::STORAGE,
                mapped_at_creation: false,
            });
        }

        if !batches.is_empty() {
            queue.write_buffer(&self.command_buffer, 0, bytemuck::cast_slice(batches));
        }
    }

    /// Esegue il batch indiretto sul RenderPass
    pub fn execute<'a>(&'a self, rpass: &mut RenderPass<'a>) {
        let stride = std::mem::size_of::<DrawIndexedIndirectArgs>() as BufferAddress;
        for i in 0..self.active_draw_count {
            let offset = i as BufferAddress * stride;
            rpass.draw_indexed_indirect(&self.command_buffer, offset);
        }
    }
}
