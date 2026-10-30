#!/usr/bin/env python3
"""Apply the bundle on the pinned Horizon and cargo check the game. Horizon's clone is only read."""
import argparse
import io
import json
import shutil
import subprocess
import tarfile
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--horizon", required=True, help="a Horizon git clone holding the pinned commit")
    parser.add_argument("--mixins", required=True, help="the horizon-mixins binary")
    parser.add_argument("--target-dir", help="a Cargo target directory to reuse")
    args = parser.parse_args()

    revision = json.loads((ROOT / "compatibility.json").read_text())["horizon_revision"]
    work = Path(tempfile.mkdtemp(prefix="legacy-pvp-"))
    try:
        source, stage = work / "source", work / "stage"
        source.mkdir()
        archive = subprocess.run(["git", "-C", args.horizon, "archive", revision], capture_output=True, check=True)
        with tarfile.open(fileobj=io.BytesIO(archive.stdout)) as tar:
            tar.extractall(source, filter="data")
        (source / ".horizon-revision").write_text(revision)
        subprocess.run([args.mixins, "apply", "--workspace", source, "--mods", ROOT / "mixins", "--out", stage], check=True)
        command = ["cargo", "check", "--locked", "-p", "horizon-client"]
        if args.target_dir:
            command += ["--target-dir", args.target_dir]
        subprocess.run(command, cwd=stage, check=True)
    finally:
        shutil.rmtree(work, ignore_errors=True)


if __name__ == "__main__":
    main()
