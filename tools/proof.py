#!/usr/bin/env python3
"""The mod in a running modded game, offscreen on fresh temporary data: its row and page in Options > Mods, the page's
files, and the sword's first-person pose. Single player, with the mode forced to Always, because Auto leaves a world of
the window's own alone. Nothing touches the player's data; screenshots go to --out."""
import argparse
import json
import os
import shutil
import subprocess
import tempfile
import time
from pathlib import Path


class Game:
    def __init__(self, ctl, resources, files):
        self.ctl, self.resources, self.files = ctl, resources, files

    def __enter__(self):
        self.dirs = [tempfile.mkdtemp(prefix=f"legacy-pvp-{n}-") for n in ("home", "launcher", "data", "config", "ctl")]
        self.home = Path(self.dirs[0])
        for name, text in self.files.items():
            (self.home / name).parent.mkdir(parents=True, exist_ok=True)
            (self.home / name).write_text(text)
        self.socket = str(Path(self.dirs[4]) / "ctl.sock")
        self.env = dict(os.environ, HORIZON_HOME=self.dirs[0], HORIZON_LAUNCHER_HOME=self.dirs[1],
                        XDG_DATA_HOME=self.dirs[2], XDG_CONFIG_HOME=self.dirs[3],
                        HORIZON_LAUNCHER_NO_SELF_UPDATE="1", HORIZON_NO_AUDIO="1")
        self.run("launch", "--world", "Proof", "--creative", "--offscreen", "--wait", "--resources", self.resources)
        time.sleep(3)
        return self

    def __exit__(self, *_):
        subprocess.run([self.ctl, "--socket", self.socket, "quit"], env=self.env, cwd=self.home, capture_output=True)
        time.sleep(2)
        for folder in self.dirs:
            shutil.rmtree(folder, ignore_errors=True)

    def run(self, *args):
        # To files, not pipes: a launched game inherits them and outlives horizon-ctl.
        with tempfile.TemporaryFile("w+") as out, tempfile.TemporaryFile("w+") as err:
            code = subprocess.run([self.ctl, "--socket", self.socket, *args], env=self.env, cwd=self.home,
                                  stdout=out, stderr=err, timeout=240).returncode
            out.seek(0)
            err.seek(0)
            stdout, stderr = out.read(), err.read()
        if code:
            raise AssertionError(f"{args}: {stdout} {stderr}")
        try:
            return json.loads(stdout)
        except json.JSONDecodeError:
            return stdout

    def widgets(self):
        return self.run("ui").get("widgets", [])

    def labels(self):
        return [w.get("label") for w in self.widgets()]

    def ids(self):
        return {w.get("label"): w.get("id") for w in self.widgets()}

    def play(self):
        for _ in range(6):
            if self.run("status").get("screen") == "playing":
                return
            self.run("key", "esc", "tap")
            time.sleep(0.5)
        raise AssertionError(f"not back in the world: {self.run('status').get('screen')}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ctl", required=True, help="horizon-ctl beside the modded horizon-client")
    parser.add_argument("--resources", required=True, help="prepared 26.3 resources")
    parser.add_argument("--out", required=True, help="a folder for the screenshots")
    args = parser.parse_args()
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    data = "horizon-mods/.horizon-mod-data/legacy_pvp"
    with Game(args.ctl, args.resources, {f"{data}/mode": "always"}) as g:
        g.run("options", "mods")
        rows = g.labels()
        print("mods page:", rows)
        assert any(str(r).startswith("Legacy PvP 0.1.0") for r in rows), rows
        g.run("click", g.ids()["Settings"])
        page = g.labels()
        print("settings page:", page)
        assert "Mode: Always on" in page, page
        assert "Sword blocking: ON" in page, page
        # The list scrolls; `ui` lists the rows on screen.
        g.run("scroll-at", "640", "360", "-20")
        time.sleep(0.5)
        page = g.labels()
        print("scrolled:", page)
        assert "Swing while using (1.7 look): OFF" in page, page
        g.run("click", g.ids()["1.8 hurt tilt direction: ON"])
        assert "1.8 hurt tilt direction: OFF" in g.labels(), g.labels()
        assert (g.home / data / "hurt_tilt").read_text() == "false"
        g.run("click", g.ids()["1.8 hurt tilt direction: OFF"])
        g.play()

        g.run("command", "gamemode survival")
        g.run("command", "give @s diamond_sword")
        g.run("command", "tp @s ~ ~ ~ 0 10")
        time.sleep(2)
        print("state:", json.dumps(g.run("state"))[:600])
        g.run("screenshot", str(out / "1-sword-held.png"))
        g.run("mouse", "right", "down")
        time.sleep(1)
        g.run("screenshot", str(out / "2-sword-blocking.png"))
        print("blocking state:", json.dumps(g.run("state"))[:600])
        g.run("key", "f5", "tap")
        time.sleep(1)
        g.run("screenshot", str(out / "3-third-person-blocking.png"))
        g.run("key", "f5", "tap")
        g.run("key", "f5", "tap")
        g.run("mouse", "right", "up")
        time.sleep(1)
        g.run("screenshot", str(out / "4-released.png"))
    print("ok")


if __name__ == "__main__":
    main()
