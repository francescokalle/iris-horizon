# Iris Shaders per Horizon (`iris-horizon`)

Porting nativo in **Rust puro** delle **Iris Shaders** compatibile con il motore [horizon-rust/horizon](https://github.com/horizon-rust/horizon), basato su **`wgpu`** (Vulkan, Metal, DirectX 12).

---

## 🚀 Caratteristiche

* **Piena Compatibilità Iris / OptiFine**:
  * Supporto per `shaders.properties`, profili grafici e opzioni personalizzate.
  * Gestione dei passaggi standard: `shadow`, `prepare`, `gbuffers_*`, `deferred*`, `composite*`, e `final`.
  * Supporto per 16 buffer colore (`colortex0` .. `colortex15`) e 3 buffer profondità (`depthtex0..2`).
  * Supporto per formati di texture avanzati: `RGBA16F`, `RGBA32F`, `RG11F_G11F_B10F`, `RGB10_A2`.
* **Transcompilazione da GLSL a `wgpu`**:
  * Parser di macro e direttive `#include`, `#define`, `#ifdef`.
  * Riscrivi automatica dell'AST per i costrutti OpenGL legacy (`gl_ModelViewMatrix`, `gl_FragData[n]`, ecc.).
  * Pipeline con layout memoria conforme a **`std140`** (`IrisGlobalUBOData`).
* **Ottimizzazioni Avanzate**:
  * **MultiDraw Indirect (`draw_indexed_indirect`)**: Riduce drasticamente le draw calls unendo i chunk mesh.
  * **Persistent Pipeline Cache (`wgpu::PipelineCache`)**: Salva i blob compilati su disco eliminando i freeze/micro-stutter.
  * **Uniform Ring Buffer**: Evita blocchi di sincronizzazione CPU-GPU tramite triplo buffering.
  * **Shadow Frustum Culling**: Calcolo analitico dei piani di vista del frustum della luce per escludere i chunk non proiettanti ombre.

---

## 📁 Struttura del Progetto

```
iris-horizon/
├── Cargo.toml
├── horizon-mixin.toml
├── src/
│   ├── lib.rs                   # Entry point e re-export
│   ├── types.rs                 # PassType, ShaderStage, Definizioni base
│   ├── parser/
│   │   ├── properties.rs        # Parser di shaders.properties
│   │   ├── preprocessor.rs      # Gestore #include, direttive e AST patch
│   │   └── pack.rs              # Loader per cartelle e archivi shaderpack
│   ├── compiler/
│   │   ├── glsl_transpiler.rs   # Transcompilatore Naga GLSL -> WGSL/SPIR-V
│   │   └── pipeline_layout.rs   # Generatore BindGroupLayout e PipelineLayout
│   ├── framebuffer/
│   │   ├── targets.rs           # Allocatore colortex0-15, depthtex0-2
│   │   └── ping_pong.rs         # Controller di scambio buffer (Ping-Pong)
│   ├── uniforms/
│   │   ├── iris_ubo.rs          # Struttura allineata std140 per matrici e dati di gioco
│   │   └── provider.rs          # Gestore UBO e scritture GPU
│   ├── optimizations/
│   │   ├── indirect_draw.rs     # Batching indiretto dei chunk di rendering
│   │   ├── pipeline_cache.rs    # Cache binaria persistente su disco
│   │   ├── uniform_ring.rs      # Ring buffer a triplo buffering per uniform
│   │   └── shadow_culling.rs    # Frustum culling specifico per le ombre
│   ├── pipeline/
│   │   ├── pass.rs              # Rappresentazione di un singolo pass
│   │   └── orchestrator.rs      # Grafo di esecuzione completo dei pass
│   └── mixins/
│       └── render_mixin.rs      # Hook di compilazione per Horizon
└── tests/
    └── pack_test.rs             # Test di parsing, preprocessor e UBO alignment
```

---

## 🛠️ Come Compilare e Integrare con Horizon

1. **Prerequisiti**:
   * Rust (nightly o stable recente) con supporto per l'edizione 2024.
   * `cargo` e gli strumenti di sviluppo C++ per il backend Vulkan/DirectX.

2. **Integrazione Mixin**:
   * La mod include `horizon-mixin.toml`. Quando il launcher di Horizon compila l'istanza con le native mod abilitate, applica automaticamente gli hook di `WorldRendererMixin` e `ChunkRendererMixin` all'interno di `horizon_render`.

3. **Installazione degli Shaderpack**:
   * Posiziona i tuoi pacchetti di shader preferiti (es. *Complementary*, *BSL*, *SEUS*) nella cartella:
     ```
     .horizon/shaderpacks/
     ```
   * Avvia Horizon e attiva lo shaderpack tramite la schermata delle impostazioni video.
