Punchy 0.1.5 updates native hand and item animations for **Horizon Nightly 2026-11-05 (`66f3981`)**, Minecraft 26.3.

- Adapts first-person rendering to Horizon's hand/item shader ID contract.
- Preserves current hand-pose hooks, shield sizing and held-item resolution.
- Includes all 223 clips; no separate Punchy JAR or Fabric installation.

Download `punchy-rust-preview-0.1.5.hnmod`, add it to your instance's native mods, enable Punchy and rebuild the game in the launcher. It pins the tested Nightly; later Horizon revisions require validation.

Horizon: `66f3981f522b24c250b21523a70080965b940424`.
horizon-mixins: `5a64af938ec86b00d53d16a408aabc0038fb3e57`.

Validation: 55 Rust tests, Clippy, formatting, Horizon's bundle parser, mixin application and the native game build passed. All 12 graphical smoke checks passed again on macOS arm64 / Metal, including attacks, inspection, shields, consumption, mining and shovel interaction.

Windows/Linux, shader packs, custom item texture packs and combined native mods were not tested. Full original-Punchy parity is not claimed. Previous releases remain available for rollback.

Based on Punchy by Dev Punchy Man. Original license notices included.
