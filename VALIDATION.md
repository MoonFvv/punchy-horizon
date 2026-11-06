# Validation scope

## 0.1.5 — Nightly 2026-11-05

Validated on macOS arm64 / Apple M4 Pro (Metal) on 2026-11-06:

- Horizon `66f3981f522b24c250b21523a70080965b940424` (the shipped Nightly).
- horizon-mixins `5a64af938ec86b00d53d16a408aabc0038fb3e57`.
- 55 engine tests passed, including checked-in math and matrix oracle fixtures.
- Engine Clippy with warnings denied and formatting checks passed.
- The exact 0.1.5 bundle passed the pinned Horizon `Bundle::read` parser.
- All 10 injections and 2 support modules applied to pristine exported source.
- The generated `horizon-client` and `horizon-ctl` binaries compiled with
  `cargo build --locked -p horizon-client -p horizon-client-lib --bin horizon-client --bin horizon-ctl`.
  macOS ld emitted a large unwind-section warning; compilation succeeded.
- All 12 graphical checks listed below passed again in a fresh temporary world
  on the default server-thread path, with no external Punchy JAR.
- The final bundle was applied separately after formatting and documentation
  changes; its generated Rust was byte-identical to the compiled/tested stage.

The renderer now supplies hand/item runs for the shader-pack item-ID contract,
uses the current held-item resolver, and preserves Horizon's hand-pose hook and
shield-size setting. Shader packs, custom item texture packs, additional mods,
and Windows/Linux were not exercised; the Metal smoke test does not establish
compatibility on those configurations. Full original-Punchy parity remains outside
this validation.

Workspace evidence: `workspace/reports/punchy/nightly-0.1.5/runtime-smoke.json`,
the three PNGs beside it, and `workspace/reports/punchy/nightly-0.1.5-*.log`.
These workspace reports are kept outside the publication repository.

Rollback: keep using the previous 0.1.4 bundle with its original Horizon pin.
0.1.5 does not migrate player data or alter saves. Older release artifacts and the
separate local preview source tree are retained.

## Historical baseline

The 0.1.2 runtime baseline passed 55 Rust tests and the following 12 isolated
graphical checks on installed performance build native-81ae4a7-3afc63bbb08f.
Those runtime checks are historical baseline evidence, not a claim that every
original Punchy feature or every Horizon revision works.

- observed render matrices finite: passed.
- bundled 223 clips without external jar: passed.
- source skeleton reaches renderer: passed.
- sword attack reaches runtime: passed.
- inspect key accepted: passed.
- shield use reaches runtime: passed.
- eat start loop end: passed.
- drink start loop end: passed.
- mining start loop end: passed.
- mining releases after key up: passed.
- successful shovel use triggers configured clip: passed.
- creative instant break triggers mining: passed.

0.1.3 changes source portability, tests, documentation and packaging metadata.
Record its standalone build/check results in release notes. Re-test the native
game when runtime code, supported pins or rendering integration changes.

Original-JAR numerical evidence: 338 math/expression/sound rows and 128 matrix
cases. Their generated results are checked in under mixins/punchy/data/oracles/.
Full original-mod visual/runtime parity has not been established.
