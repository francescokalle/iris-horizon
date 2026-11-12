use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use log::{info, warn};
use wgpu::*;

pub struct PersistentPipelineCache {
    cache: Option<PipelineCache>,
    cache_file_path: PathBuf,
}

impl PersistentPipelineCache {
    pub fn new(device: &Device, path: PathBuf) -> Self {
        let existing_data = if path.exists() {
            match File::open(&path) {
                Ok(mut f) => {
                    let mut buf = Vec::new();
                    if f.read_to_end(&mut buf).is_ok() {
                        info!("Cache pipeline caricata da disco ({} bytes)", buf.len());
                        Some(buf)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        } else {
            None
        };

        let cache = unsafe {
            device.create_pipeline_cache(&PipelineCacheDescriptor {
                label: Some("Iris_Horizon_Pipeline_Cache"),
                data: existing_data.as_deref(),
                fallback: true,
            })
        };

        Self {
            cache: Some(cache),
            cache_file_path: path,
        }
    }

    pub fn get_cache(&self) -> Option<&PipelineCache> {
        self.cache.as_ref()
    }

    /// Salva su disco lo stato compilato delle pipeline grafiche
    pub fn persist(&self) {
        if let Some(ref cache) = self.cache {
            if let Some(data) = cache.get_data() {
                if let Some(parent) = self.cache_file_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if let Ok(mut f) = File::create(&self.cache_file_path) {
                    let _ = f.write_all(&data);
                    info!("Cache pipeline salvata con successo su {:?}", self.cache_file_path);
                } else {
                    warn!("Impossibile salvare la cache su {:?}", self.cache_file_path);
                }
            }
        }
    }
}
