use std::sync::atomic::{AtomicUsize, Ordering};
use wgpu::*;

/// Ring buffer a triplo buffering per evitare sincronizzazioni CPU-GPU bloccanti
pub struct UniformRingBuffer {
    buffers: [Buffer; 3],
    current_index: AtomicUsize,
    buffer_size: BufferAddress,
}

impl UniformRingBuffer {
    pub fn new(device: &Device, size: BufferAddress, label_prefix: &str) -> Self {
        let create_buf = |idx: usize| {
            device.create_buffer(&BufferDescriptor {
                label: Some(&format!("{}_{}", label_prefix, idx)),
                size,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };

        Self {
            buffers: [create_buf(0), create_buf(1), create_buf(2)],
            current_index: AtomicUsize::new(0),
            buffer_size: size,
        }
    }

    /// Ottiene il buffer da aggiornare per il frame corrente
    pub fn advance_frame(&self) -> (usize, &Buffer) {
        let next_idx = (self.current_index.load(Ordering::Relaxed) + 1) % 3;
        self.current_index.store(next_idx, Ordering::Relaxed);
        (next_idx, &self.buffers[next_idx])
    }

    pub fn current_buffer(&self) -> &Buffer {
        let idx = self.current_index.load(Ordering::Relaxed);
        &self.buffers[idx]
    }
}
