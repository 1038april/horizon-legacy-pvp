# Legacy PvP (Horizon native mod)

Makes Horizon play the way the vanilla 1.8.9 client does on 1.8 servers, reached through a server's ViaVersion layer.
It changes only the client, and never adds reach, clicks or hit box beyond 1.8.9.

**Experimental.** It is pinned to one Horizon Nightly (`compatibility.json`) and has not been tested against a real
1.8 server yet. Test on your own server before you take it anywhere with an anticheat.

## What it does

Options > Mods > Legacy PvP > Settings has the mode and one switch per feature.

| Setting | Default | What it does | 1.8.9 source (obfuscated) |
|---|---|---|---|
| Mode | Auto | Auto: on for servers, off in your own world. The client can't see a server's version, so a modern server counts as a server: turn the mod off there | |
| Hide attack indicator | On | No indicator at the crosshair or hotbar | no cooldown exists |
| No weapon lowering | On | The hand dips only when the item changes | `bfn.a` |
| Swing before attack packet | On | Sends the swing, then the attack | `ave.aw` |
| Slow down after sprint hits | On | 0.6 speed cut and sprint stop after a knockback hit on a player | `wn.f` |
| Sword blocking | On | Right click uses the sword: 1.8 pose, 0.2 speed, use and release packets | `bfn.d`, `bbj.a` |
| 1.8 picking margin | On | Mob and player boxes grow by 0.1 when aiming | `pk.ao`, `bfk.a` |
| 1.8 sneak eye and box | On | Eye at 1.54, box stays 0.6 x 1.8, also for other players' boxes when aiming | `wn.aS`, `wn.I` |
| Instant eye height | On | The camera doesn't ease when sneaking | `bfk.f` |
| 1.8 hurt tilt direction | On | The view always tilts the same way | `pr.a(byte)` |
| Swing while using | **Off** | Shows a swing while blocking, eating or drawing. This is 1.7's look: 1.8.9 passes swing 0 to every use pose (`bfn.a(float)`). Changes no packets | |

## Not done yet

- Golden Age Combat's server rebalancing (the planned "singleplayer / own server" mode).
- A per-server override of the mode.
- The picking margin on boats, minecarts, armor stands and other non-mob entities.
- Swimming, crawling and elytra poses, which 1.8 doesn't have.
- The blocking sword's size is 26.3's normal held size. Its position and rotation are computed from 1.8.9's values.

## Install

Native mods run with your user's full rights, so read the code first. The first build downloads Rust and takes a while.

1. In the launcher, turn on Settings > Game > **Native mods (experimental)**.
2. Put `legacy_pvp-0.1.0.hnmod` in `horizon-native-mods/` in the game folder (the one holding `options.txt`).
3. Add the line `mod.legacy_pvp:true` to `horizon-mods.txt` in the same folder. A file put there by hand doesn't turn itself on.
4. Press **Build** in the launcher, and play the instance it builds.

Or unpack the `.hnmod` and build that folder from the command line:

```sh
horizon-launcher native-build <unpacked folder> --trust-native-code
```

## Package it

```sh
python tools/package.py        # dist/legacy_pvp-0.1.0.hnmod; the same source gives the same file
```

## Check it

```sh
cargo test                     # settings, mode selection, 1.8 values, poses, packet order
python tools/check.py --horizon ../horizon --mixins path/to/horizon-mixins   # apply on the pin and cargo check the game
```

`--horizon` is a Horizon git clone holding the pinned commit; it is only read. `--mixins` is `horizon-mixins` built at
the commit in `compatibility.json`.

## Licence

[MPL-2.0](LICENSE). See [NOTICE.md](NOTICE.md).
