# Combined-source startup and prime-chain software validation

Runtime source: `f4249f62df082b401717a03ae2cae1b7b1aeddd3`.
Integrated main: `5a250bd47f7674b1f8fbc8a5ca577805aa235ecc`.
Frozen CLI fixture: `127ab13b48e1ec70b54fd189a2fe8c3c87430a26`.
Release binary SHA-256: `1f4bae481e114bb081acd595722431112610f2253d4b4037cfc1317299d5a5fc`.

Fresh validation passed 5,170 release library tests (zero failed, 123 ignored),
both changed hyperelliptic binary catalog tests, strict maintained Clippy,
release build, eight CLI checks and the frozen ten-command two-twist fixture.
The source was checked before and after each validation command. Earlier source
results remain unchanged in `../validation-startup-integration-20261029/`.

`.artifacts.json` locates 31 software objects, totaling 43,223,799 bytes, in
`s3://setec-astronomy-artifacts/store`. `remote-readback.json` records a fresh GET,
size and SHA-256 check of every object without using a local cache. The initial
12-object readback remains as incremental checkpoint evidence.

`startup-main-cli-fixture.tar.xz` contains all 159 fixture files under `runs/`,
including command logs, original inputs, bounded traces, verification receipts
and the corrupted copy. Archive SHA-256:
`937e0c59108d438807c25ecb938903889cd36a7553ad968ff9a1811a1fb2221f`.
The independent audit in `../startup-main-cli-fixture-audit.json` verifies all
158 inventory entries, archive members, frozen fixture files, binary/source
bindings and ten commands. The two n=7/m=3 cells cover both twists: one verified
PDP lift and one encoding-relative completion without a verified lift. Rehashed
algebraic corruption passes integrity status and fails semantic replay.

The initial sparse-checkout target-resolution failure and wrong binary target
spelling are retained. Exact frozen Git blobs restored missing build inputs;
the corrected Cargo target is `hyperelliptic-cover`. These were validation setup
failures, with no runtime source change or lint suppression.

This is software evidence: zero research-corpus rows, zero independently
recovered ECDLP cells and zero verified cloud dataset cells. No positive
production startup, complete instruction count, native-call recovery, cloud
restore, fall-degree theorem, trained-model result or full-corpus claim follows.
The inherited checkpoint flag describes exposed-prefix persistence only.
