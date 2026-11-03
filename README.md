# Punchy for Horizon

Community Rust port of [Punchy](https://modrinth.com/mod/punchy-fpa) first-person
animations for [Horizon](https://github.com/horizon-rust/horizon).

Original Punchy by **Dev Punchy Man**. Port maintained by **MoonFvv**.
This is an unofficial community project.

## Install

Use a release's `.hnmod` file. In Horizon's launcher, add it to your instance's
native mods, enable Punchy and build the game with native mods. Keep your own
Minecraft 26.3 installation available for Horizon's normal resource preparation.

No separate Punchy JAR, Java runtime or Fabric installation is needed to run
this port. Its 223 bundled animation clips, geometry and tuning are compiled
into the native game. **Hand poses default to Punchy.** F6 inspects; F7 reloads
bundled animations. Key bindings and animation speed can be changed in Mods.

## Current scope

Implemented/tested cases include sword attacks, inspection, shield use,
eating/drinking/mining start-loop-end sequences, mining release, creative
instant breaks and successful shovel use. Interrupted animations retain a
live outgoing pose during transitions.

This is an incomplete port. Full locomotion, physics, special item renderers,
the original editor/config layering and all compound hand/equip transitions
are not implemented. Full visual equivalence with original Punchy is not established.

## Compatibility

- Minecraft: **26.3**.
- Bundle: **0.1.3** (portable source/build packaging of the locally tested 0.1.2 runtime).
- Horizon: `81ae4a7d8f27d38e5cabbdcda0a06b2c4b000045`.
- horizon-mixins: `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`.

Other revisions require validation. This native mod runs with the same account
permissions as Horizon; review its source before building it.

## Develop and package

Rust and Python 3.9+ are needed for development. Engine tests use bundled data
and checked-in numerical fixtures; no original JAR or decompiler is needed.

```sh
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --check
```

To build a bundle, check out the pinned Horizon source and run:

```sh
git clone https://github.com/horizon-rust/horizon.git horizon-source
git -C horizon-source checkout 81ae4a7d8f27d38e5cabbdcda0a06b2c4b000045
python3 tools/package.py --horizon-source horizon-source
```

The tool validates the exact `.hnmod` with Horizon's actual Bundle::read parser.
Output is in `dist/`. Add `--offline` to use cached Cargo dependencies.
The original license notices are included with the bundle.

`tools/runtime_smoke.py` can exercise a compiled native game with `--binary`,
`--ctl`, `--resources` and `--report`. It creates fresh temporary homes and a
throwaway world; it never uses the player's saves. See VALIDATION.md for scope.

## Credits and licensing

See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), [LICENSE](LICENSE),
[LICENSE-Punchy](LICENSE-Punchy) and [LICENSE-Horizon](LICENSE-Horizon).
The Horizon MIT license does not relicense Punchy's implementation or assets.
A registry listing or review acceptance is not claimed.
