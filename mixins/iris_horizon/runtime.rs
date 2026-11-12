// Iris Shaders for Horizon Client: core shader pack loader compatible
// with Iris and OptiFine on the wgpu graphics backend.

pub const ID: &str = "iris_horizon";
pub const NAME: &str = "Iris Shaders";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Complete Iris & OptiFine shader pack compatibility layer with real-time Voxel Path Tracing (Rethinking Voxels), compute shaders, and high-performance chunk GPU culling.";

#[derive(Debug, Clone, PartialEq)]
pub struct IrisSettings {
    pub enabled: bool,
    pub shader_pack: String,
    pub profile: String,
    pub chunk_gpu_culling: bool,
    pub render_scale: f32,
    pub shadow_distance: f32,
}

impl Default for IrisSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            shader_pack: "(OFF)".into(),
            profile: "MEDIUM".into(),
            chunk_gpu_culling: true,
            render_scale: 1.0,
            shadow_distance: 96.0,
        }
    }
}

impl IrisSettings {
    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Shaders: {}", if self.enabled { "ENABLED" } else { "DISABLED" }),
            format!("Shaderpack: {}", self.shader_pack),
            format!("Voxel Profile: {}", self.profile),
            format!("Chunk Compute: {}", if self.chunk_gpu_culling { "GPU CULLING (Ultra)" } else { "CPU Standard" }),
            format!("Render Scale: {:.2}x", self.render_scale),
            format!("Shadow Distance: {}m", self.shadow_distance as u32),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.enabled = !self.enabled,
            1 => {
                // Shaderpack cycle (handled by bridge)
            }
            2 => {
                // Cycle voxel / quality profiles
                self.profile = match self.profile.as_str() {
                    "LOW" => "MEDIUM".into(),
                    "MEDIUM" => "HIGH".into(),
                    "HIGH" => "ULTRA".into(),
                    _ => "LOW".into(),
                };
            }
            3 => {
                // Toggle GPU compute culling for chunks
                self.chunk_gpu_culling = !self.chunk_gpu_culling;
            }
            4 => {
                // Cycle render resolution scale: 0.50x -> 0.75x -> 1.0x
                self.render_scale = match (self.render_scale * 100.0) as u32 {
                    50 => 0.75,
                    75 => 1.0,
                    100 => 0.50,
                    _ => 1.0,
                };
            }
            5 => {
                // Cycle shadow map distance
                self.shadow_distance = match self.shadow_distance as u32 {
                    64 => 96.0,
                    96 => 128.0,
                    128 => 192.0,
                    _ => 64.0,
                };
            }
            _ => {}
        }
    }
}
