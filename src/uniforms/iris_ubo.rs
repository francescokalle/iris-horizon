use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3, Vec4};

/// Struttura memoria rigorosamente allineata std140 per Uniform Buffer wgpu/SPIR-V
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct IrisGlobalUBOData {
    // Matrici primarie (16 float ciascuna = 64 byte, allineate a 16 byte)
    pub gbuffer_model_view: Mat4,
    pub gbuffer_projection: Mat4,
    pub gbuffer_model_view_inverse: Mat4,
    pub gbuffer_projection_inverse: Mat4,
    pub shadow_model_view: Mat4,
    pub shadow_projection: Mat4,
    pub shadow_model_view_inverse: Mat4,
    pub shadow_projection_inverse: Mat4,

    // Vettori celesti e sole (allineati a 16 byte: vec3 + 1 float pad)
    pub sun_position: Vec3,
    pub sun_angle: f32,

    pub moon_position: Vec3,
    pub shadow_light_position: f32,

    pub up_position: Vec3,
    pub shadow_distance: f32,

    // Posizione giocatore e camera
    pub camera_position: Vec3,
    pub eye_altitude: f32,

    pub previous_camera_position: Vec3,
    pub is_eye_in_water: i32,

    // Coordinate colori ambientali
    pub fog_color: Vec4,
    pub sky_color: Vec4,

    // Dimensioni vista e temporizzazione
    pub view_size: Vec2,
    pub aspect_ratio: f32,
    pub frame_time_counter: f32,

    pub frame_counter: i32,
    pub world_time: i32,
    pub world_day: i32,
    pub rain_strength: f32,

    // Padding per allineamento a blocco 16 byte
    pub _padding: [f32; 4],
}

impl Default for IrisGlobalUBOData {
    fn default() -> Self {
        Self {
            gbuffer_model_view: Mat4::IDENTITY,
            gbuffer_projection: Mat4::IDENTITY,
            gbuffer_model_view_inverse: Mat4::IDENTITY,
            gbuffer_projection_inverse: Mat4::IDENTITY,
            shadow_model_view: Mat4::IDENTITY,
            shadow_projection: Mat4::IDENTITY,
            shadow_model_view_inverse: Mat4::IDENTITY,
            shadow_projection_inverse: Mat4::IDENTITY,
            sun_position: Vec3::new(0.0, 1.0, 0.0),
            sun_angle: 0.0,
            moon_position: Vec3::new(0.0, -1.0, 0.0),
            shadow_light_position: 1.0,
            up_position: Vec3::new(0.0, 1.0, 0.0),
            shadow_distance: 128.0,
            camera_position: Vec3::ZERO,
            eye_altitude: 64.0,
            previous_camera_position: Vec3::ZERO,
            is_eye_in_water: 0,
            fog_color: Vec4::new(0.7, 0.8, 1.0, 1.0),
            sky_color: Vec4::new(0.5, 0.7, 1.0, 1.0),
            view_size: Vec2::new(1920.0, 1080.0),
            aspect_ratio: 1920.0 / 1080.0,
            frame_time_counter: 0.0,
            frame_counter: 0,
            world_time: 6000,
            world_day: 1,
            rain_strength: 0.0,
            _padding: [0.0; 4],
        }
    }
}
