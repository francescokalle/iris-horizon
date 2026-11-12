use std::collections::HashMap;
use crate::types::Image3DDescriptor;

#[derive(Debug, Clone)]
pub struct ShaderProperties {
    pub profile: String,
    pub profile_definitions: HashMap<String, String>,
    pub options: HashMap<String, String>,
    pub buffer_formats: HashMap<String, wgpu::TextureFormat>,
    pub buffer_sizes: HashMap<String, (f32, f32)>, // Scale relative to screen (e.g. 0.5 for half res)
    pub images_3d: HashMap<String, Image3DDescriptor>,
    pub shadow_map_resolution: u32,
    pub shadow_distance: f32,
    pub separate_ao: bool,
    pub old_hand_light: bool,
    pub voxelize_light_blocks: bool,
    pub cloud_height: Option<f32>,
}

impl Default for ShaderProperties {
    fn default() -> Self {
        Self {
            profile: "MEDIUM".into(),
            profile_definitions: HashMap::new(),
            options: HashMap::new(),
            buffer_formats: HashMap::new(),
            buffer_sizes: HashMap::new(),
            images_3d: HashMap::new(),
            shadow_map_resolution: 2048,
            shadow_distance: 128.0,
            separate_ao: true,
            old_hand_light: false,
            voxelize_light_blocks: true,
            cloud_height: None,
        }
    }
}

pub struct PropertiesParser;

impl PropertiesParser {
    pub fn parse(content: &str) -> ShaderProperties {
        let mut props = ShaderProperties::default();

        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parsing dei profili es: profile.LOW = ...
            if let Some(rest) = line.strip_prefix("profile.") {
                if let Some((prof_name, defs)) = rest.split_once('=') {
                    props.profile_definitions.insert(prof_name.trim().to_string(), defs.trim().to_string());
                }
                continue;
            }

            // Parsing di immagini 3D per voxel tracing (OptiFine / Iris format)
            // Es: image.voxel_img = voxel_sampler RED_INTEGER R8UI UNSIGNED_INT true false 128 96 128
            // Es: image.voxelCols = none RED_INTEGER R32I INT true false 128 384 128
            if let Some(rest) = line.strip_prefix("image.") {
                if let Some((img_name, spec)) = rest.split_once('=') {
                    let parts: Vec<&str> = spec.split_whitespace().collect();
                    if parts.len() >= 9 {
                        let internal_format = parts[2];
                        let w = parts[6].parse::<u32>().unwrap_or(128);
                        let h = parts[7].parse::<u32>().unwrap_or(128);
                        let d = parts[8].parse::<u32>().unwrap_or(128);

                        let format = match internal_format {
                            "R32I" => wgpu::TextureFormat::R32Sint,
                            "R32UI" => wgpu::TextureFormat::R32Uint,
                            "R8UI" => wgpu::TextureFormat::R8Uint,
                            "RGBA16F" => wgpu::TextureFormat::Rgba16Float,
                            "RGBA32F" => wgpu::TextureFormat::Rgba32Float,
                            _ => wgpu::TextureFormat::Rgba8Unorm,
                        };

                        props.images_3d.insert(
                            img_name.trim().to_string(),
                            Image3DDescriptor {
                                name: img_name.trim().to_string(),
                                width: w,
                                height: h,
                                depth: d,
                                format,
                            },
                        );
                    }
                }
                continue;
            }

            if let Some((raw_key, raw_val)) = line.split_once('=') {
                let key = raw_key.trim();
                let val = raw_val.trim();

                if key.starts_with("colortex") || key.starts_with("shadowcolor") {
                    if let Some(target) = key.strip_suffix(".format") {
                        let format = match val {
                            "R8" => wgpu::TextureFormat::R8Unorm,
                            "RG8" => wgpu::TextureFormat::Rg8Unorm,
                            "RGB8" | "RGBA8" => wgpu::TextureFormat::Rgba8Unorm,
                            "RGB16" | "RGBA16" => wgpu::TextureFormat::Rgba16Unorm,
                            "RGB16F" | "RGBA16F" => wgpu::TextureFormat::Rgba16Float,
                            "RGB32F" | "RGBA32F" => wgpu::TextureFormat::Rgba32Float,
                            "R11F_G11F_B10F" => wgpu::TextureFormat::Rg11b10Float,
                            "RGB10_A2" => wgpu::TextureFormat::Rgb10a2Unorm,
                            _ => wgpu::TextureFormat::Rgba8Unorm,
                        };
                        props.buffer_formats.insert(target.to_string(), format);
                    } else if let Some(target) = key.strip_suffix(".scale") {
                        if let Ok(scale) = val.parse::<f32>() {
                            props.buffer_sizes.insert(target.to_string(), (scale, scale));
                        }
                    }
                } else if key == "shadowMapResolution" {
                    if let Ok(res) = val.parse::<u32>() {
                        props.shadow_map_resolution = res;
                    }
                } else if key == "shadowDistance" {
                    if let Ok(dist) = val.parse::<f32>() {
                        props.shadow_distance = dist;
                    }
                } else if key == "separateAo" {
                    props.separate_ao = val.eq_ignore_ascii_case("true");
                } else if key == "oldHandLight" {
                    props.old_hand_light = val.eq_ignore_ascii_case("true");
                } else if key == "voxelizeLightBlocks" {
                    props.voxelize_light_blocks = val.eq_ignore_ascii_case("true");
                } else if key == "cloudHeight" {
                    props.cloud_height = val.parse::<f32>().ok();
                } else if key == "profile" {
                    props.profile = val.to_string();
                } else {
                    props.options.insert(key.to_string(), val.to_string());
                }
            }
        }

        // Se è impostato un profilo (es. MEDIUM o LOW), carica le relative opzioni
        if let Some(defs) = props.profile_definitions.get(&props.profile).cloned() {
            for token in defs.split_whitespace() {
                if let Some((k, v)) = token.split_once('=') {
                    props.options.insert(k.to_string(), v.to_string());
                } else if let Some(k) = token.strip_prefix('!') {
                    props.options.remove(k);
                } else {
                    props.options.insert(token.to_string(), "true".to_string());
                }
            }
        }

        props
    }
}
