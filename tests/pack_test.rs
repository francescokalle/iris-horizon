use std::collections::HashMap;
use std::path::PathBuf;
use iris_horizon::parser::{PropertiesParser, ShaderPreprocessor};
use iris_horizon::uniforms::IrisGlobalUBOData;

#[test]
fn test_properties_parser() {
    let sample_props = r#"
        profile=ULTRA
        shadowMapResolution=4096
        shadowDistance=256.0
        colortex0.format=RGBA16F
        colortex4.format=R11F_G11F_B10F
        colortex4.scale=0.5
        separateAo=true
    "#;

    let props = PropertiesParser::parse(sample_props);
    assert_eq!(props.profile, "ULTRA");
    assert_eq!(props.shadow_map_resolution, 4096);
    assert_eq!(props.shadow_distance, 256.0);
    assert_eq!(props.buffer_formats.get("colortex0"), Some(&wgpu::TextureFormat::Rgba16Float));
    assert_eq!(props.buffer_formats.get("colortex4"), Some(&wgpu::TextureFormat::Rg11b10Float));
    assert_eq!(props.buffer_sizes.get("colortex4"), Some(&(0.5, 0.5)));
    assert!(props.separate_ao);
}

#[test]
fn test_preprocessor_conditionals_and_transforms() {
    let mut options = HashMap::new();
    options.insert("ATMOSPHERIC_FOG".to_string(), "true".to_string());

    let preprocessor = ShaderPreprocessor::new(options);

    let sample_glsl = r#"
        #ifdef ATMOSPHERIC_FOG
        vec4 fogColor = gl_FragData[0];
        #else
        vec4 fogColor = vec4(0.0);
        #endif
        mat4 mv = gl_ModelViewMatrix;
    "#;

    let processed = preprocessor.preprocess(sample_glsl, &PathBuf::from("test.fsh"), &PathBuf::from(".")).unwrap();
    assert!(processed.contains("iris_frag_out_0"));
    assert!(processed.contains("gbufferModelView"));
    assert!(!processed.contains("vec4(0.0)"));
}

#[test]
fn test_ubo_std140_alignment() {
    let ubo = IrisGlobalUBOData::default();
    let bytes = bytemuck::bytes_of(&ubo);
    // Verifica che la dimensione sia multiplo di 16 (allineamento std140)
    assert_eq!(bytes.len() % 16, 0);
}
