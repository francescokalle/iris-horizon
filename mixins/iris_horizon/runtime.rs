// Iris Shaders per Horizon Client: core del loader di shader compatibili
// con Iris e OptiFine per Minecraft Java in Rust su wgpu.

pub const ID: &str = "iris_horizon";
pub const NAME: &str = "Iris Shaders";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Supporto completo per shaderpack compatibili con Iris e OptiFine, con ottimizzazioni per Voxel Path Tracing e calcolo ultra-rapido dei chunk a 24+ chunk (GPU Culling compute pass e culling facce).";

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
            format!("Shaders: {}", if self.enabled { "ABILITATI" } else { "DISABILITATI" }),
            format!("Pack: {}", self.shader_pack),
            format!("Profilo Voxels: {}", self.profile),
            format!("Calcolo Chunk: {}", if self.chunk_gpu_culling { "GPU CULLING (Ultra)" } else { "CPU Standard" }),
            format!("Risoluzione Render: {:.2}x", self.render_scale),
            format!("Ombre: {}m", self.shadow_distance as u32),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.enabled = !self.enabled,
            1 => {
                // Ciclo dei pacchetti (gestito dal bridge)
            }
            2 => {
                // Ciclo profili voxel
                self.profile = match self.profile.as_str() {
                    "LOW" => "MEDIUM".into(),
                    "MEDIUM" => "HIGH".into(),
                    "HIGH" => "ULTRA".into(),
                    _ => "LOW".into(),
                };
            }
            3 => {
                // Toggle calcolo chunk su GPU (GPU Culling)
                self.chunk_gpu_culling = !self.chunk_gpu_culling;
            }
            4 => {
                // Scala risoluzione render
                self.render_scale = match (self.render_scale * 100.0) as u32 {
                    50 => 0.75,
                    75 => 1.0,
                    100 => 0.50,
                    _ => 1.0,
                };
            }
            5 => {
                // Distanza ombre
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
