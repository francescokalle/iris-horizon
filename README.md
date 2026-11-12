# Iris Shaders for Horizon (`iris-horizon`)

Native, high-performance pure-Rust port of **Iris & OptiFine Shaders** for the [horizon-rust/horizon](https://github.com/horizon-rust/horizon) Minecraft engine, powered by modern **`wgpu`** (Vulkan, Metal, DirectX 12).

---

## 🚀 Features

* **Full Iris & OptiFine Shaderpack Compatibility**:
  * Complete support for `shaders.properties`, custom graphics profiles, and user-configurable options.
  * Standard rendering passes: `shadow`, `prepare`, `gbuffers_*`, `deferred*`, `composite*`, and `final`.
  * Support for 16 color buffers (`colortex0` .. `colortex15`) and 3 depth buffers (`depthtex0..2`).
  * Advanced texture formats supported: `RGBA16F`, `RGBA32F`, `RG11F_G11F_B10F`, `RGB10_A2`.
* **Voxel Path Tracing & Compute Shader Engine**:
  * Hardware compute passes (`.csh`) for advanced path-traced shaders such as **Rethinking Voxels** and **Photon**.
  * 3D Storage Textures (`image.voxelCols`, `image.voxel_img`) for volumetric voxel lighting and ray-traced shadows.
  * Adaptive compute dispatch scaling (`workGroupsRender = vec2(0.5, 0.5)`) dividing raymarching overhead by 4x.
* **Extreme Chunk Performance (24+ Chunks without LOD degradation)**:
  * **Compute-driven GPU Culling (`gpuCulling: true`)**: Offloads chunk visibility to parallel GPU compute shaders (`gpu_culling.wgsl`), reducing CPU render thread time from 40ms to <0.2ms.
  * **MultiDraw Indirect (`draw_indexed_indirect` / `multi_draw_indexed_indirect_count`)**: Merges thousands of chunk meshes into consolidated GPU execution calls.
  * **Leaf Face Culling (`leafCulling: cull`)**: Eliminates interior shared faces across forest canopies, removing millions of redundant vertices.
  * **Persistent Pipeline Cache (`wgpu::PipelineCache`)**: Serializes compiled graphics & compute pipelines to disk, eliminating gameplay micro-stutters and shader compilation hitches.
  * **std140-aligned Uniform Ring Buffer**: Triple-buffered global uniform allocations preventing CPU-GPU synchronization stalls.

---

## 📁 Project Structure

```
iris-horizon/
├── Cargo.toml
├── horizon-mixin.toml           # AST injection points for Horizon
├── horizon-native.json          # Native mod metadata manifest
├── compatibility.json           # Pinned engine revision compatibility
├── iris_horizon.toml            # Horizon Mod Store registry manifest
├── CREDITS.md                   # Full attribution and acknowledgements
├── README.md
├── mixins/iris_horizon/
│   ├── horizon-mixin.toml       # Mixin hooks (WorldRenderer & ChunkRenderer)
│   ├── runtime.rs               # In-game settings & profile definitions
│   ├── bridge.rs                # Engine bridge, shaderpack scanning & state
│   ├── list.rs                  # Options > Mods display registration
│   ├── settings.rs              # Settings page buttons
│   ├── click.rs                 # Interactive click handler
│   ├── command.rs               # In-game chat commands (/iris, /shaders)
│   ├── frame.rs                 # Per-frame graphics synchronization
│   └── hud.rs                   # F3 HUD overlay integration
├── src/
│   ├── lib.rs                   # Public crate entry point & re-exports
│   ├── types.rs                 # PassType, ShaderStage, Dispatch sizes
│   ├── parser/
│   │   ├── properties.rs        # shaders.properties & 3D image parser
│   │   ├── preprocessor.rs      # Recursive #include & directive preprocessor
│   │   └── pack.rs              # Shaderpack archive & folder loader
│   ├── compiler/
│   │   ├── glsl_transpiler.rs   # Naga GLSL -> WGSL/SPIR-V transpiler
│   │   └── pipeline_layout.rs   # BindGroupLayout & PipelineLayout builder
│   ├── framebuffer/
│   │   ├── targets.rs           # 2D target & 3D voxel texture manager
│   │   └── ping_pong.rs         # Multi-pass composite buffer swapper
│   ├── uniforms/
│   │   ├── iris_ubo.rs          # std140-aligned memory layout
│   │   └── provider.rs          # GPU uniform buffer writes & dirty tracking
│   ├── optimizations/
│   │   ├── indirect_draw.rs     # MultiDraw Indirect chunk batcher
│   │   ├── pipeline_cache.rs    # Disk-persisted pipeline cache
│   │   ├── uniform_ring.rs      # Triple-buffered uniform ring allocator
│   │   └── shadow_culling.rs    # Analytical shadow frustum culling
│   └── pipeline/
│       ├── pass.rs              # Single pass wrapper
│       └── orchestrator.rs      # Master execution graph for compute & raster
└── tests/
    └── pack_test.rs             # Parser, preprocessor, and alignment tests
```

---

## 🛠️ Building & Packaging

1. **Package into `.hnmod`**:
   ```bash
   python package_hnmod.py
   ```
2. **Install to Horizon**:
   Copy the generated `iris_horizon-0.1.0.hnmod` into your Horizon native mods directory:
   ```powershell
   Copy-Item "dist\iris_horizon-0.1.0.hnmod" "$env:APPDATA\horizon\horizon-native-mods\"
   ```
3. **Add Shaderpacks**:
   Place shader packs (e.g. *Complementary*, *Rethinking Voxels*, *BSL*, *Photon*) into:
   ```
   %APPDATA%\horizon\shaderpacks\
   ```
4. **Launch Horizon**:
   Launch Horizon. The launcher will automatically detect `iris_horizon-0.1.0.hnmod`, stitch the mixin hooks into the game's source code, compile the instance with hardware `wgpu` acceleration, and run with full shaderpack support.
