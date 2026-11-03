Standalone source and native bundle packaging of the community Punchy Rust port for Horizon.

- Portable engine tests use embedded data and numerical fixtures; no original JAR needed.
- Punchy hand poses are the default.
- Original author: Dev Punchy Man. Port maintainer: MoonFvv.
- Includes original Punchy and Horizon license notices.

Validation: 55 Rust tests, Clippy with warnings denied, and the pinned Horizon native-bundle parser passed. The underlying 0.1.2 runtime previously passed 12 isolated graphical checks, including consumption/mining phases, creative instant breaks and successful shovel use. This packaging release does not claim full original Punchy parity.

Minecraft 26.3; pinned Horizon 81ae4a7d8f27d38e5cabbdcda0a06b2c4b000045 and horizon-mixins ddd9700b21997c95d95ac6ee5ecafe3830cf622a. The bundle requires Horizon's normal native-mod build, with no separate Punchy JAR/Fabric install.
