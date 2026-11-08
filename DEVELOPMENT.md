# Development

## Compatibility

- Minecraft: **26.3**.
- Bundle: **0.1.8** (In-Game Live HUD Editor F9, overhauled settings, Punchy! Moves shoulder camera, free/orbit camera mode, turn easing).
- Horizon Nightly 2026-11-05: `66f3981f522b24c250b21523a70080965b940424`.
- horizon-mixins: `5a64af938ec86b00d53d16a408aabc0038fb3e57`.

Other revisions require validation. This native mod runs with the same account
permissions as Horizon; review its source before building it.

## Develop and package

Rust and Python 3.9+ are needed for development. Engine tests use bundled data
and checked-in numerical fixtures; no original JAR or decompiler is needed.

```sh
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --check
rustfmt --edition 2024 --check mixins/punchy/geometry.rs mixins/punchy/render.rs
```

To build a bundle, check out the pinned Horizon source and run:

```sh
git clone https://github.com/horizon-rust/horizon.git horizon-source
git -C horizon-source checkout 66f3981f522b24c250b21523a70080965b940424
python3 tools/package.py --horizon-source horizon-source
```

The tool validates the exact `.hnmod` with Horizon's actual Bundle::read parser.
Output is in `dist/`. Add `--offline` to use cached Cargo dependencies.
The original license notices are included with the bundle. Source exports with
Horizon's `.horizon-revision` marker are accepted too.

The mixin application and game compilation must also pass: parser validation alone
does not check private gameplay APIs or rendering signatures. Unpack the bundle,
then use the pinned horizon-mixins CLI to apply its `mixins/` directory to a fresh
export of the pinned Horizon source. Build `horizon-client` and the
`horizon-client-lib` binary `horizon-ctl` from that generated stage.

`tools/runtime_smoke.py` can exercise a compiled native game with `--binary`,
`--ctl`, `--resources` and `--report`. It creates fresh temporary homes and a
throwaway world; it never uses the player's saves. See VALIDATION.md for scope.


Add `--viewmodel` to the runtime smoke command to exercise the real mod-settings
buttons, verify independent arm/item matrices, restart the game to check
persistence, and test both reset actions. It uses the same scratch game directory
for that restart. Numeric defaults, input validation, bounds and transform order
are also covered by engine tests in `viewmodel.rs`.
