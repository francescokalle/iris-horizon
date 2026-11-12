#!/usr/bin/env python3
"""
Script di packaging per la mod nativa Iris per Horizon (.hnmod).
Crea il file .hnmod esattamente conforme alla specifica e allo schema richiesto da Horizon.
"""

import os
import sys
import json
import zipfile
from pathlib import Path

def create_hnmod_bundle(project_dir: Path, output_file: Path):
    print(f"[*] Packaging Horizon Native Mod (.hnmod) in: {output_file}")

    manifest_path = project_dir / "horizon-native.json"
    if not manifest_path.exists():
        raise FileNotFoundError(f"Manifest 'horizon-native.json' non trovato in {project_dir}")

    # Verifica conformità dello schema manifest (campi ammessi da Horizon: schema, id, version, game, horizon, mixins, hooks)
    with open(manifest_path, "r", encoding="utf-8") as f:
        manifest = json.load(f)

    allowed_keys = {"schema", "id", "version", "game", "horizon", "mixins", "hooks"}
    extra_keys = set(manifest.keys()) - allowed_keys
    if extra_keys:
        raise ValueError(f"Campi non ammessi nel manifest horizon-native.json: {extra_keys}")

    output_file.parent.mkdir(parents=True, exist_ok=True)

    with zipfile.ZipFile(output_file, "w", zipfile.ZIP_DEFLATED) as zf:
        # 1. Manifest alla radice del file .hnmod
        zf.write(manifest_path, arcname="horizon-native.json")
        print("  + Aggiunto: horizon-native.json")

        # 2. File all'interno di mixins/iris_horizon/
        mixins_dir = project_dir / "mixins" / "iris_horizon"
        if not mixins_dir.exists():
            raise FileNotFoundError(f"Cartella {mixins_dir} non trovata!")

        for f in sorted(mixins_dir.glob("*")):
            if f.is_file():
                arcname = f"mixins/iris_horizon/{f.name}"
                zf.write(f, arcname=arcname)
                print(f"  + Aggiunto: {arcname}")

    print(f"[OK] Bundle nativo generato: {output_file} ({output_file.stat().st_size} bytes)")

if __name__ == "__main__":
    base_dir = Path(__file__).resolve().parent
    out_hnmod = base_dir / "iris_horizon-0.1.0.hnmod"
    create_hnmod_bundle(base_dir, out_hnmod)
