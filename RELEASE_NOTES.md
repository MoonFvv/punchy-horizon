Native first-person hand and item animations for Horizon (Minecraft 26.3).

- Attacks, shields, inspection and item interactions.
- Eating, drinking and mining start-loop-end sequences, including creative instant breaks.
- Live animation transitions and bundled resources; no separate Punchy JAR or Fabric installation.
- Punchy hand poses enabled by default.

Download the `.hnmod` asset, add it to your instance's native mods, enable it and build the game from the launcher.

Pinned to Horizon `81ae4a7d8f27d38e5cabbdcda0a06b2c4b000045` and horizon-mixins `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`.

Validation: 55 Rust tests and Clippy passed; the bundle is checked with the pinned Horizon parser. The 0.1.2 runtime baseline passed 12 isolated graphical checks. This release updates standalone packaging/documentation; full original-mod parity is not claimed.

Based on Punchy by Dev Punchy Man. Original license notices included.
