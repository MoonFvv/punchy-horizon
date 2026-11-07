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
- **Adjustable viewmodel**: separate arm and item position (X/Y/Z) and scale.
- **Punchy! Moves third-person shoulder camera**: over-the-shoulder dynamic camera with obstacle ray clipping, configurable distance, height, shoulder horizontal offset, and FOV.
- **In-Game Live HUD Editor (F9)**: real-time on-screen editor overlay rendered directly in your world. Adjust camera and arm viewmodels live while seeing your character move!
- **Free/orbit camera mode (F8)**: smoothly orbit the camera around the player character without turning the player model.
- **Stationary turn-in-place easing**: body yaw smoothing and 50-degree head rotation constraint matching Punchy! Moves locomotion.
- **Self-contained resources**: 223 bundled clips, geometry and tuning; no separate Punchy JAR or Fabric install.

Settings: **Options > Mods > Punchy Rust**. Hand poses default to **Punchy**.
- **F6**: inspect held item
- **F7**: reload bundled animations and configuration
- **F8**: toggle Punchy! Moves free/orbit camera mode
- **F9**: toggle In-Game Live HUD Editor

## In-Game Live HUD Editor (F9)

Press **F9** while playing in any world or server to bring up the live interactive editor:
- **`Tab`**: switch between **Moves 3rd-Person Camera**, **1st-Person Arms Viewmodel**, and **1st-Person Held Items**.
- **`↑` / `↓`** (or `Page Up` / `Page Down`): select which parameter to adjust.
- **`←` / `→`** (or `-` / `=`, `[` / `]`): adjust the parameter in real time. The camera and arms move right before your eyes!
- **`Backspace`**: reset the selected parameter to its default.
- **`R`**: reset all parameters in the current category to defaults.
- **`F9`**: close the editor.

## Overhauled Settings Menu

Open **Options > Mods > Punchy Rust > Settings** to view and configure all settings with full visibility:
- **Dedicated `[-]` / `[+]` stepping buttons**: every parameter shows its name, exact current value, and step delta (e.g. `[-] Horizontal: +1.17 blk (-0.10)` / `[+] Horizontal: +1.17 blk (+0.10)`).
- **Active Section switcher**: click to cycle between **Moves 3rd-Person Camera**, **Arms Viewmodel**, and **Held Items Viewmodel**. All 4 parameters for the active section are displayed simultaneously.
- **Reset buttons**: reset individual sections or all mod settings to defaults with one click.
- **Master toggles**: easily toggle preview, in-game live HUD editor, shoulder camera, free orbit mode, and hand pose styles.

## Installation

1. Download the `.hnmod` file from [Releases](https://github.com/MoonFvv/punchy-horizon/releases/latest).
2. Add it to your Horizon instance's native mods and enable Punchy.
3. Build the game with native mods in the launcher, then launch it.

Horizon still prepares its own Minecraft resources from your Minecraft installation.
Native mods run with the same access as the game.

## Compatibility and development

Updated for Horizon **Nightly 2026-11-05** (`66f3981`). Pinned to Horizon `66f3981f522b24c250b21523a70080965b940424` and
horizon-mixins `5a64af938ec86b00d53d16a408aabc0038fb3e57`.

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
