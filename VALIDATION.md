# Validation scope

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
