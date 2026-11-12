# Crediti e Riconoscimenti (Credits & Acknowledgements)

Questo progetto (`iris-horizon`) è un porting nativo in Rust per il motore **Horizon** ed è reso possibile grazie allo straordinario lavoro delle seguenti comunità e progetti open source:

---

### 1. Iris Shaders Team
* **Progetto:** [Iris Shaders](https://github.com/IrisShaders/iris)
* **Licenza:** LGPL-3.0
* **Autori & Contributori:** IMS212, coderbot, embeddedt, FoundationGames e tutti i collaboratori di Iris.
* **Contributo:** Definizione dell'architettura della pipeline, transcompilazione GLSL moderna, specifiche dei pass `shadow`, `prepare`, `gbuffers_*`, `deferred*`, `composite*` e `final`, formati dei render target `colortex` e `depthtex`, gestione uniform std140 e compatibilità OptiFine.

---

### 2. Horizon Rust Engine
* **Progetto:** [horizon-rust/horizon](https://github.com/horizon-rust/horizon)
* **Autori:** Horizon Contributors
* **Contributo:** Riscrittura completa in Rust di Minecraft: Java Edition, architettura grafica moderna basata su `wgpu`, e framework di iniezione compile-time `horizon-mixins`.

---

### 3. Rethinking Voxels & Complementary Shaders
* **Autori:** **gri573** (*Rethinking Voxels*) e **EminGT** (*Complementary Shaders*).
* **Contributo:** Architettura di voxel path tracing in tempo reale, illuminazione volumetrica colorata dei blocchi (colored block lighting), e pipeline basata su compute shaders (`.csh`) con texture 3D storage (`voxelCols`, `voxel_img`).

---

### 4. Sodium & Nvidium Teams
* **Progetto:** [Sodium](https://github.com/CaffeineMC/sodium-fabric)
* **Autori:** JellySquid / CaffeineMC, IMS, Cafe.
* **Contributo:** Pioniere delle tecniche di rendering moderno dei chunk Minecraft: MultiDraw Indirect, compattazione dei vertici e ottimizzazione del throughput GPU.

---

### 5. OptiFine
* **Autore:** **sp614x**
* **Contributo:** Creazione dello standard originale per gli shader pack di Minecraft (`shaders.properties`, direttive `#include`, formati dei buffer).
