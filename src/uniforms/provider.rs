use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::*;
use crate::uniforms::iris_ubo::IrisGlobalUBOData;

pub struct IrisUniformProvider {
    pub buffer: Buffer,
    pub current_data: IrisGlobalUBOData,
    pub is_dirty: bool,
}

impl IrisUniformProvider {
    pub fn new(device: &Device) -> Self {
        let initial_data = IrisGlobalUBOData::default();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Iris_Global_Uniform_Buffer"),
            contents: bytemuck::bytes_of(&initial_data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            current_data: initial_data,
            is_dirty: true,
        }
    }

    /// Aggiorna i dati uniform e li scrive nel buffer del GPU se modificati
    pub fn update(&mut self, queue: &Queue, data: &IrisGlobalUBOData) {
        self.current_data = *data;
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.current_data));
        self.is_dirty = false;
    }

    /// Ritorna il binding del buffer per l'inclusione nei BindGroup
    pub fn as_entire_binding(&self) -> BindingResource {
        self.buffer.as_entire_binding()
    }
}
