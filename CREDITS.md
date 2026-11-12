# Credits & Acknowledgements

The `iris-horizon` project is a native Rust port designed for the **Horizon** engine, made possible through the groundbreaking contributions of the following open-source projects and authors:

---

### 1. Iris Shaders Team
* **Project:** [Iris Shaders](https://github.com/IrisShaders/iris)
* **License:** LGPL-3.0
* **Authors & Contributors:** IMS212, coderbot, embeddedt, FoundationGames, and all Iris contributors.
* **Contribution:** Pioneered modern Minecraft shaderpack execution, GLSL preprocessor specifications, memory layouts, pass management (`shadow`, `prepare`, `gbuffers_*`, `deferred*`, `composite*`, `final`), `colortex` and `depthtex` target management, and OptiFine compatibility.

---

### 2. Horizon Rust Engine
* **Project:** [horizon-rust/horizon](https://github.com/horizon-rust/horizon)
* **Authors:** Horizon Engine Contributors
* **Contribution:** Clean-room native Rust rewrite of *Minecraft: Java Edition*, modern `wgpu` rendering layer, and the `horizon-mixins` compile-time AST transformation system.

---

### 3. Rethinking Voxels & Complementary Shaders
* **Authors:** **gri573** (*Rethinking Voxels*) and **EminGT** (*Complementary Shaders*).
* **Contribution:** Real-time volumetric voxel path tracing, colored block lighting physics, and the modern compute shader (`.csh`) architecture utilizing 3D storage textures (`voxelCols`, `voxel_img`).

---

### 4. Sodium & Nvidium Teams
* **Project:** [Sodium](https://github.com/CaffeineMC/sodium-fabric)
* **Authors:** JellySquid / CaffeineMC, IMS, Cafe.
* **Contribution:** Pioneered modern chunk rendering techniques: MultiDraw Indirect (`draw_indexed_indirect`), vertex memory layout compaction, and GPU draw call optimization.

---

### 5. OptiFine
* **Author:** **sp614x**
* **Contribution:** The original standard for Minecraft shaderpacks (`shaders.properties`, `#include` directives, and buffer formatting).
