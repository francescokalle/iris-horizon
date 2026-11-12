use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use regex::Regex;
use crate::types::{ComputeDispatchSize, PassType, ShaderProgramSource};
use crate::parser::properties::{PropertiesParser, ShaderProperties};
use crate::parser::preprocessor::ShaderPreprocessor;

#[derive(Debug)]
pub struct ShaderPack {
    pub name: String,
    pub root_dir: PathBuf,
    pub properties: ShaderProperties,
    pub programs: HashMap<PassType, ShaderProgramSource>,
}

impl ShaderPack {
    pub fn load_from_directory(dir: &Path) -> Result<Self, String> {
        let name = dir.file_name().unwrap_or_default().to_string_lossy().to_string();
        let shaders_dir = if dir.join("shaders").exists() {
            dir.join("shaders")
        } else {
            dir.to_path_buf()
        };

        // Cartella world0 per gli shader specifici dell'Overworld (usata da Rethinking Voxels)
        let world0_dir = shaders_dir.join("world0");

        let prop_path = shaders_dir.join("shaders.properties");
        let properties = if prop_path.exists() {
            let content = std::fs::read_to_string(&prop_path).unwrap_or_default();
            PropertiesParser::parse(&content)
        } else {
            ShaderProperties::default()
        };

        let preprocessor = ShaderPreprocessor::new(properties.options.clone());
        let mut programs = HashMap::new();

        let all_passes = [
            PassType::Shadow,
            PassType::ShadowComp,
            PassType::ShadowComp1,
            PassType::ShadowComp2,
            PassType::Prepare,
            PassType::Prepare1,
            PassType::Prepare2,
            PassType::Prepare3,
            PassType::Prepare4,
            PassType::GbuffersTerrain,
            PassType::GbuffersEntities,
            PassType::GbuffersBlock,
            PassType::GbuffersTextured,
            PassType::GbuffersTexturedLit,
            PassType::GbuffersBasic,
            PassType::GbuffersHand,
            PassType::GbuffersHandWater,
            PassType::GbuffersWeather,
            PassType::GbuffersSkyBasic,
            PassType::GbuffersSkyTextured,
            PassType::GbuffersClouds,
            PassType::GbuffersWater,
            PassType::Deferred,
            PassType::Deferred1,
            PassType::Deferred2,
            PassType::Deferred3,
            PassType::Composite,
            PassType::Composite1,
            PassType::Composite2,
            PassType::Composite3,
            PassType::Final,
        ];

        let re_wg_render = Regex::new(r"workGroupsRender\s*=\s*vec2\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*\)").unwrap();
        let re_wg_fixed = Regex::new(r"workGroups\s*=\s*ivec3\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
        let re_local_x = Regex::new(r"local_size_x\s*=\s*(\d+)").unwrap();
        let re_local_y = Regex::new(r"local_size_y\s*=\s*(\d+)").unwrap();

        for pass in all_passes {
            let prefix = pass.file_prefix();

            // Cerca prima in world0/, poi nella cartella shaders/
            let find_file = |ext: &str| -> Option<PathBuf> {
                let fname = format!("{}.{}", prefix, ext);
                let w0 = world0_dir.join(&fname);
                if w0.exists() {
                    return Some(w0);
                }
                let root = shaders_dir.join(&fname);
                if root.exists() {
                    return Some(root);
                }
                None
            };

            let vsh_path = find_file("vsh");
            let fsh_path = find_file("fsh");
            let csh_path = find_file("csh");

            let vertex_source = vsh_path.as_ref().and_then(|p| {
                std::fs::read_to_string(p).ok().and_then(|s| preprocessor.preprocess(&s, p, &shaders_dir).ok())
            });

            let fragment_source = fsh_path.as_ref().and_then(|p| {
                std::fs::read_to_string(p).ok().and_then(|s| preprocessor.preprocess(&s, p, &shaders_dir).ok())
            });

            let mut compute_dispatch = None;
            let compute_source = csh_path.as_ref().and_then(|p| {
                std::fs::read_to_string(p).ok().and_then(|raw| {
                    let pre = preprocessor.preprocess(&raw, p, &shaders_dir).ok()?;

                    // Determina la dimensione del dispatch compute per il voxel tracing
                    if let Some(caps) = re_wg_fixed.captures(&pre) {
                        let x = caps.get(1).unwrap().as_str().parse::<u32>().unwrap_or(16);
                        let y = caps.get(2).unwrap().as_str().parse::<u32>().unwrap_or(12);
                        let z = caps.get(3).unwrap().as_str().parse::<u32>().unwrap_or(16);
                        compute_dispatch = Some(ComputeDispatchSize::Fixed { x, y, z });
                    } else if let Some(caps) = re_wg_render.captures(&pre) {
                        let sx = caps.get(1).unwrap().as_str().parse::<f32>().unwrap_or(0.5);
                        let sy = caps.get(2).unwrap().as_str().parse::<f32>().unwrap_or(0.5);
                        let lx = re_local_x.captures(&pre).and_then(|c| c.get(1).unwrap().as_str().parse::<u32>().ok()).unwrap_or(16);
                        let ly = re_local_y.captures(&pre).and_then(|c| c.get(1).unwrap().as_str().parse::<u32>().ok()).unwrap_or(16);
                        compute_dispatch = Some(ComputeDispatchSize::ScreenRelative {
                            scale_x: sx,
                            scale_y: sy,
                            local_x: lx,
                            local_y: ly,
                        });
                    } else {
                        // Default screen-relative a metà risoluzione (0.5x) per i compute pass raytracing
                        compute_dispatch = Some(ComputeDispatchSize::ScreenRelative {
                            scale_x: 0.5,
                            scale_y: 0.5,
                            local_x: 16,
                            local_y: 16,
                        });
                    }

                    Some(pre)
                })
            });

            if vertex_source.is_some() || fragment_source.is_some() || compute_source.is_some() {
                let target_path = vsh_path.or(csh_path).unwrap_or_else(|| shaders_dir.join(format!("{}.glsl", prefix)));
                programs.insert(
                    pass,
                    ShaderProgramSource {
                        pass,
                        vertex_source,
                        fragment_source,
                        compute_source,
                        compute_dispatch,
                        path: target_path,
                    },
                );
            }
        }

        Ok(ShaderPack {
            name,
            root_dir: shaders_dir,
            properties,
            programs,
        })
    }
}
