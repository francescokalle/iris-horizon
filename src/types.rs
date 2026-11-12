use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Geometry,
    Compute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassType {
    // Shadow raster & compute passes
    Shadow,
    ShadowComp,
    ShadowComp1,
    ShadowComp2,
    
    // Prepare passes (compute or raster)
    Prepare,
    Prepare1,
    Prepare2,
    Prepare3,
    Prepare4,
    
    // G-Buffers passes
    GbuffersTerrain,
    GbuffersEntities,
    GbuffersBlock,
    GbuffersTextured,
    GbuffersTexturedLit,
    GbuffersBasic,
    GbuffersHand,
    GbuffersHandWater,
    GbuffersWeather,
    GbuffersSkyBasic,
    GbuffersSkyTextured,
    GbuffersClouds,
    GbuffersWater,

    // Deferred passes
    Deferred,
    Deferred1,
    Deferred2,
    Deferred3,
    Deferred4,
    Deferred5,
    Deferred6,
    Deferred7,

    // Composite passes
    Composite,
    Composite1,
    Composite2,
    Composite3,
    Composite4,
    Composite5,
    Composite6,
    Composite7,

    // Final screen pass
    Final,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComputeDispatchSize {
    Fixed { x: u32, y: u32, z: u32 },
    ScreenRelative { scale_x: f32, scale_y: f32, local_x: u32, local_y: u32 },
}

#[derive(Debug, Clone)]
pub struct Image3DDescriptor {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: wgpu::TextureFormat,
}

impl PassType {
    pub fn file_prefix(&self) -> &'static str {
        match self {
            PassType::Shadow => "shadow",
            PassType::ShadowComp => "shadowcomp",
            PassType::ShadowComp1 => "shadowcomp1",
            PassType::ShadowComp2 => "shadowcomp2",
            PassType::Prepare => "prepare",
            PassType::Prepare1 => "prepare1",
            PassType::Prepare2 => "prepare2",
            PassType::Prepare3 => "prepare3",
            PassType::Prepare4 => "prepare4",
            PassType::GbuffersTerrain => "gbuffers_terrain",
            PassType::GbuffersEntities => "gbuffers_entities",
            PassType::GbuffersBlock => "gbuffers_block",
            PassType::GbuffersTextured => "gbuffers_textured",
            PassType::GbuffersTexturedLit => "gbuffers_textured_lit",
            PassType::GbuffersBasic => "gbuffers_basic",
            PassType::GbuffersHand => "gbuffers_hand",
            PassType::GbuffersHandWater => "gbuffers_hand_water",
            PassType::GbuffersWeather => "gbuffers_weather",
            PassType::GbuffersSkyBasic => "gbuffers_skybasic",
            PassType::GbuffersSkyTextured => "gbuffers_skytextured",
            PassType::GbuffersClouds => "gbuffers_clouds",
            PassType::GbuffersWater => "gbuffers_water",
            PassType::Deferred => "deferred",
            PassType::Deferred1 => "deferred1",
            PassType::Deferred2 => "deferred2",
            PassType::Deferred3 => "deferred3",
            PassType::Deferred4 => "deferred4",
            PassType::Deferred5 => "deferred5",
            PassType::Deferred6 => "deferred6",
            PassType::Deferred7 => "deferred7",
            PassType::Composite => "composite",
            PassType::Composite1 => "composite1",
            PassType::Composite2 => "composite2",
            PassType::Composite3 => "composite3",
            PassType::Composite4 => "composite4",
            PassType::Composite5 => "composite5",
            PassType::Composite6 => "composite6",
            PassType::Composite7 => "composite7",
            PassType::Final => "final",
        }
    }

    pub fn is_compute_candidate(&self) -> bool {
        matches!(
            self,
            PassType::ShadowComp
                | PassType::ShadowComp1
                | PassType::ShadowComp2
                | PassType::Prepare
                | PassType::Prepare1
                | PassType::Prepare2
                | PassType::Prepare3
                | PassType::Prepare4
                | PassType::Deferred
                | PassType::Deferred1
                | PassType::Deferred2
                | PassType::Deferred3
                | PassType::Composite
                | PassType::Composite1
                | PassType::Composite2
                | PassType::Composite3
        )
    }

    pub fn is_composite_or_deferred(&self) -> bool {
        matches!(
            self,
            PassType::Deferred
                | PassType::Deferred1
                | PassType::Deferred2
                | PassType::Deferred3
                | PassType::Deferred4
                | PassType::Deferred5
                | PassType::Deferred6
                | PassType::Deferred7
                | PassType::Composite
                | PassType::Composite1
                | PassType::Composite2
                | PassType::Composite3
                | PassType::Composite4
                | PassType::Composite5
                | PassType::Composite6
                | PassType::Composite7
                | PassType::Final
        )
    }
}

#[derive(Debug, Clone)]
pub struct ShaderProgramSource {
    pub pass: PassType,
    pub vertex_source: Option<String>,
    pub fragment_source: Option<String>,
    pub compute_source: Option<String>,
    pub compute_dispatch: Option<ComputeDispatchSize>,
    pub path: PathBuf,
}
