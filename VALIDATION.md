# Validation scope

## 0.1.7 — Punchy! Moves integration

Validated on 2026-11-07 with the same Horizon and mixins pins as 0.1.5,
on macOS arm64 / Apple M4 Pro (Metal):

- 66 engine tests passed (+8 unit tests covering camera parameters, stepping,
  clamping, shoulder positioning, multi-sample obstacle ray clipping, orbit/free
  camera turning, stationary turn-in-place easing, humanoid pose interpolation,
  and settings persistence).
- Engine Clippy with warnings denied, formatting, and package validation passed.
- Packaging with `python3 tools/package.py` passed Horizon's `Bundle::read`
  validator with 40 files in `punchy-rust-preview-0.1.7.hnmod`.
- Mixin injections added for `native_camera_pose`, `native_compute_fov`,
  `turn_player`, and `native_gui_render_post` (14 injections total).
- 17 graphical and UI runtime checks passed in a fresh temporary world, verifying
  combat, shield, eating, drinking, mining, independent arm/item viewmodels,
  settings persistence across restarts, and reset actions. Evidence:
  `workspace/reports/punchy/moves-0.1.7/runtime-smoke.json` and PNGs beside it.

## 0.1.6 — arm/item position and scale

Validated on 2026-11-06 with the same Horizon and mixins pins as 0.1.5,
on macOS arm64 / Apple M4 Pro (Metal):

- 58 engine tests passed, including three new tests for neutral defaults,
  malformed/non-finite settings, bounds, independent transforms and scale origin.
- Engine Clippy with warnings denied, formatting, Python compilation and
  whitespace checks passed.
- The exact 0.1.6 bundle passed Horizon's parser; all 10 injections and both
  support modules applied. The generated game and control tool compiled.
- The final bundle's generated Rust is byte-identical to the compiled stage.
- All 12 existing graphical animation checks passed again.
- Five new graphical checks passed through real settings buttons: arm changes
  leave items unchanged, item changes leave arms unchanged, values survive a
  full game restart, resetting one group preserves the other, and resetting
  everything writes neutral defaults to persistent storage.

The UI tests compare the rendered authored pose matrices against independently
expected offsets/scales, and keep screenshots before and after adjustments.
Evidence: `workspace/reports/punchy/viewmodel-0.1.6-final/runtime-smoke.json`,
the PNGs beside it, and `workspace/reports/punchy/viewmodel-0.1.6-*.log`.
Windows/Linux, other mods, shader/custom texture packs, map rendering and the
Horizon hand-pose fallback were not exercised by this graphical run.
Their transform paths compile; they are not runtime parity claims.

Settings are separate from existing animation tuning. Missing or invalid values
use neutral defaults; position is bounded to +/-16 model pixels and scale to
25–200%. Both hands share each group's adjustment. No worlds or player saves
are migrated. The 0.1.5 artifact remains available for rollback.

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
