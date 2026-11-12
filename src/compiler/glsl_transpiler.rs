use naga::front::glsl;
use naga::valid::{Capabilities, ValidationFlags, Validator};
use crate::types::ShaderStage;

pub struct IrisGlslTranspiler {
    ubo_injection_header: String,
}

impl IrisGlslTranspiler {
    pub fn new() -> Self {
        let ubo_injection_header = r#"
        // [Iris Horizon UBO Layout]
        layout(std140, set = 0, binding = 0) uniform IrisGlobalUBO {
            mat4 gbufferModelView;
            mat4 gbufferProjection;
            mat4 gbufferModelViewInverse;
            mat4 gbufferProjectionInverse;
            mat4 shadowModelView;
            mat4 shadowProjection;
            vec3 sunPosition;
            float sunAngle;
            vec3 cameraPosition;
            float frameTimeCounter;
            vec2 viewSize;
            float rainStrength;
            int isEyeInWater;
        };
        "#.to_string();

        Self { ubo_injection_header }
    }

    /// Inietta l'UBO globale standard Iris nel sorgente GLSL prima della compilazione
    pub fn inject_global_ubo(&self, source: &str) -> String {
        // Cerca la direttiva #version per inserire subito dopo
        if let Some(pos) = source.find("#version") {
            if let Some(newline_pos) = source[pos..].find('\n') {
                let insert_at = pos + newline_pos + 1;
                let (prefix, suffix) = source.split_at(insert_at);
                return format!("{}\n{}\n{}", prefix, self.ubo_injection_header, suffix);
            }
        }

        // Se manca #version, aggiunge #version 450 compatibility con UBO
        format!("#version 450 compatibility\n{}\n{}", self.ubo_injection_header, source)
    }

    /// Compila il sorgente GLSL in un Naga Module con validazione estesa
    pub fn transpile_glsl(
        &self,
        source: &str,
        stage: ShaderStage,
    ) -> Result<naga::Module, String> {
        let injected = self.inject_global_ubo(source);

        let naga_stage = match stage {
            ShaderStage::Vertex => naga::ShaderStage::Vertex,
            ShaderStage::Fragment => naga::ShaderStage::Fragment,
            ShaderStage::Compute => naga::ShaderStage::Compute,
            ShaderStage::Geometry => {
                return Err("I Geometry shader non sono supportati nativamente da WebGPU/wgpu".into())
            }
        };

        let mut parser = glsl::Frontend::default();
        let options = glsl::Options::from(naga_stage);

        let module = parser.parse(&options, &injected)
            .map_err(|e| format!("Errore di parsing Naga GLSL: {:?}", e))?;

        let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
        validator.validate(&module)
            .map_err(|e| format!("Errore di validazione SPIR-V/WGSL: {:?}", e))?;

        Ok(module)
    }

    /// Converte il modulo Naga in WGSL sorgente per wgpu
    pub fn emit_wgsl(&self, module: &naga::Module) -> Result<String, String> {
        use naga::back::wgsl;
        let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
        let module_info = validator.validate(module)
            .map_err(|e| format!("Validazione modulo Naga fallita: {:?}", e))?;

        let wgsl = wgsl::write_string(module, &module_info, wgsl::WriterFlags::empty())
            .map_err(|e| format!("Errore emissione WGSL: {:?}", e))?;

        Ok(wgsl)
    }
}
