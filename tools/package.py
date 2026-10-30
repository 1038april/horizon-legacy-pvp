#!/usr/bin/env python3
"""Pack the bundle into dist/<id>-<version>.hnmod: horizon-native.json and the bundle's .rs and .toml files. The same
source gives the same bytes."""
import hashlib
import json
import sys
import tomllib
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "mixins" / "legacy-pvp"


def main():
    compat = json.loads((ROOT / "compatibility.json").read_text())
    manifest = tomllib.loads((BUNDLE / "horizon-mixin.toml").read_text())
    mod_id, version = manifest["mod"]["id"], manifest["mod"]["version"]
    pinned = manifest["compatibility"]["horizon_revision"]
    if pinned != compat["horizon_revision"]:
        sys.exit(f"horizon-mixin.toml pins {pinned}, compatibility.json {compat['horizon_revision']}")
    # No `hooks`: raw targets beside the hook points tie it to this exact commit.
    native = {"schema": 1, "id": mod_id, "version": version, "game": compat["game"],
              "horizon": compat["horizon_revision"], "mixins": compat["horizon_mixins_revision"]}
    files = {"horizon-native.json": (json.dumps(native, indent=2, sort_keys=True) + "\n").encode()}
    for path in sorted(BUNDLE.iterdir()):
        if path.suffix in (".rs", ".toml"):
            files[f"mixins/{mod_id}/{path.name}"] = path.read_bytes()
    out = ROOT / "dist" / f"{mod_id}-{version}.hnmod"
    out.parent.mkdir(exist_ok=True)
    with zipfile.ZipFile(out, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, content in sorted(files.items()):
            info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, content)
    print(json.dumps({"file": str(out), "size": out.stat().st_size,
                      "sha256": hashlib.sha256(out.read_bytes()).hexdigest()}, indent=2))


if __name__ == "__main__":
    main()
