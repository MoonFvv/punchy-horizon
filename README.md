# Punchy Horizon

A native animation mod for [Horizon](https://github.com/horizon-rust/horizon)
(Minecraft Java 26.3 in Rust), bringing Punchy-style first-person hand and item
animations to the client.

## Features

- **Combat animations** for attacks and shield use.
- **Mining animations** with start, loop and end phases, including creative instant breaks.
- **Eating and drinking** with complete start-loop-end sequences.
- **Item inspection** and animated interactions such as shovel use.
- **Smooth transitions** that keep the outgoing animation moving when an action changes.
- **Self-contained resources**: 223 bundled clips, geometry and tuning; no separate Punchy JAR or Fabric install.

Settings: **Options > Mods > Punchy Rust**. Hand poses default to **Punchy**.
**F6** inspects the held item; **F7** reloads bundled animation data. Keys can be changed in the mod settings.

## Installation

1. Download the `.hnmod` file from [Releases](https://github.com/MoonFvv/punchy-horizon/releases/latest).
2. Add it to your Horizon instance's native mods and enable Punchy.
3. Build the game with native mods in the launcher, then launch it.

Horizon still prepares its own Minecraft resources from your Minecraft installation.
Native mods run with the same access as the game.

## Compatibility and development

Pinned to Horizon `81ae4a7d8f27d38e5cabbdcda0a06b2c4b000045` and
horizon-mixins `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`.

```sh
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

See [DEVELOPMENT.md](DEVELOPMENT.md) for packaging and [VALIDATION.md](VALIDATION.md)
for test coverage. Full locomotion, physics, special renderers and the original
editor are not implemented; this is not complete parity with original Punchy.

## Credits and license

Community port based on [Punchy](https://modrinth.com/mod/punchy-fpa) by Dev Punchy Man.
Includes adapted implementation and original animation data. This is an unofficial project.
Original notices are retained in [LICENSE-Punchy](LICENSE-Punchy) and
[LICENSE-Horizon](LICENSE-Horizon); see [LICENSE](LICENSE) and
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
