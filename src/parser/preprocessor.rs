use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct ShaderPreprocessor {
    options: HashMap<String, String>,
}

impl ShaderPreprocessor {
    pub fn new(options: HashMap<String, String>) -> Self {
        Self { options }
    }

    /// Preprocessa ricorsivamente include, direttive di condizionalità e macro
    pub fn preprocess(&self, source: &str, file_path: &Path, pack_root: &Path) -> Result<String, String> {
        let mut visited_files = HashSet::new();
        visited_files.insert(file_path.to_path_buf());

        let inlined = self.resolve_includes(source, file_path, pack_root, &mut visited_files)?;
        let conditional = self.evaluate_conditionals(&inlined);
        let transformed = self.transform_legacy_glsl(&conditional);

        Ok(transformed)
    }

    /// Risolve ricorsivamente le direttive `#include "..."`
    fn resolve_includes(
        &self,
        source: &str,
        current_file: &Path,
        pack_root: &Path,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<String, String> {
        let include_regex = Regex::new(r#"#include\s+["<]([^">]+)[">]"#).unwrap();
        let mut result = String::with_capacity(source.len());

        for line in source.lines() {
            if let Some(caps) = include_regex.captures(line) {
                let include_rel = caps.get(1).unwrap().as_str();

                // Prova prima relativo alla cartella del file corrente, poi relativo alla cartella shaders/
                let target_path = if let Some(parent) = current_file.parent() {
                    let p = parent.join(include_rel);
                    if p.exists() {
                        p
                    } else {
                        pack_root.join(include_rel)
                    }
                } else {
                    pack_root.join(include_rel)
                };

                if visited.contains(&target_path) {
                    // Prevenzione loop di inclusione circolare
                    result.push_str("// [Iris] Include circolare evitato\n");
                    continue;
                }

                if target_path.exists() {
                    visited.insert(target_path.clone());
                    let included_content = std::fs::read_to_string(&target_path)
                        .map_err(|e| format!("Impossibile leggere include {:?}: {}", target_path, e))?;

                    let expanded = self.resolve_includes(&included_content, &target_path, pack_root, visited)?;
                    result.push_str(&expanded);
                    result.push('\n');
                } else {
                    result.push_str(&format!("// [Iris] File include non trovato: {}\n", include_rel));
                }
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        Ok(result)
    }

    /// Valuta `#ifdef`, `#ifndef`, `#if defined(...)` basandosi sulle opzioni attive dello shaderpack
    fn evaluate_conditionals(&self, source: &str) -> String {
        let mut output = String::with_capacity(source.len());
        let mut condition_stack = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("#ifdef") {
                let key = trimmed.strip_prefix("#ifdef").unwrap().trim();
                let is_active = self.options.contains_key(key);
                condition_stack.push(is_active);
                continue;
            } else if trimmed.starts_with("#ifndef") {
                let key = trimmed.strip_prefix("#ifndef").unwrap().trim();
                let is_active = !self.options.contains_key(key);
                condition_stack.push(is_active);
                continue;
            } else if trimmed.starts_with("#else") {
                if let Some(last) = condition_stack.pop() {
                    condition_stack.push(!last);
                }
                continue;
            } else if trimmed.starts_with("#endif") {
                condition_stack.pop();
                continue;
            }

            // Se tutte le condizioni nello stack sono vere (o stack vuoto), include la riga
            let emit = condition_stack.iter().all(|&cond| cond);
            if emit {
                output.push_str(line);
                output.push('\n');
            }
        }

        output
    }

    /// Riscrive costrutti legacy OptiFine/Iris in GLSL standard modernizzabile da Naga/wgpu
    fn transform_legacy_glsl(&self, source: &str) -> String {
        let mut res = source.to_string();

        // 1. Riscrive gl_FragData[n] in iris_frag_out_n
        let frag_data_re = Regex::new(r"gl_FragData\[(\d+)\]").unwrap();
        res = frag_data_re.replace_all(&res, "iris_frag_out_$1").to_string();

        // 2. Mappa attributi e matrici legacy al layout UBO moderno
        res = res.replace("gl_ModelViewMatrixInverse", "gbufferModelViewInverse")
                 .replace("gl_ProjectionMatrixInverse", "gbufferProjectionInverse")
                 .replace("gl_ModelViewMatrix", "gbufferModelView")
                 .replace("gl_ProjectionMatrix", "gbufferProjection")
                 .replace("gl_MultiTexCoord0", "iris_uv0")
                 .replace("gl_MultiTexCoord1", "iris_light_uv")
                 .replace("gl_Vertex", "iris_vertex_pos")
                 .replace("gl_Normal", "iris_normal");

        res
    }
}
