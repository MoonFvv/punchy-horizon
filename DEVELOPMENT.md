# Development

## Compatibility

- Minecraft: **26.3**.
- Bundle: **0.1.4** (portable source/build packaging of the locally tested 0.1.2 runtime).
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

