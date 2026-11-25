# Source-commit-pinned rebuild and behavioural replay — 2026-11-25

Every earlier report in this lane carried the same limitation: the `ic`
binary was pinned by SHA-256 but its JSON reported `git_commit: null`, so the
source tree behind a store could only be inferred from a separate build
receipt. This closes that.

## Build

Detached worktree at commit `cc7d6141f26bc02ae02876bc094b6a7783990947`, source
unmodified, build directory copied from the `a550` tree:

```bash
export CRYPTO_BUILD_GIT_COMMIT=$(git rev-parse HEAD)
export CRYPTO_BUILD_GIT_DIRTY=0
export RUSTC=…/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc
…/toolchains/1.99.1-aarch64-apple-darwin/bin/cargo build --release --locked --bin ic
```

Finished in 3 m 44 s with 21 library warnings. The `rust-objcopy` strip step
aborted on a missing `libLLVM.dylib`; the optimized binary was produced and
links, as in `SOURCE_BUILD_RECEIPT_20261124.md`.

- Binary SHA-256 `8e6545292260169ed86c2a696f55bab1ea3e6c576a1c960ae80c79f8c6d43261`
- `software.binary_blake3` `b449282386818d154e1039f7e9c1bdd5d90bcbb40bca8d845a44317bedefeee4`
- `software.git_commit` `cc7d6141f26bc02ae02876bc094b6a7783990947`
- `software.git_commit_source` `CRYPTO_BUILD_GIT_COMMIT`, `git_dirty` false

This is the first binary in the lane whose stores self-report a commit.

## Behavioural replay against the evidence binary

The published component panel used `c98e290c…`, built from `a5503f4b` with no
commit embedded. Replaying its first timed call of each arm on instance
`20261185` with the new binary reproduces the recorded workload exactly:

| Arm | Source | Calls | Multiply-adds | Relations | Trials |
|:--|:--|---:|---:|---:|---:|
| interval | stored `c98e290c…` | 1645 | 209,541,485,486 | 219 | 1666 |
| interval | replay `8e654529…` | 1645 | 209,541,485,486 | 219 | 1666 |
| candidate | stored `c98e290c…` | 1610 | 6,404,307,749 | 208 | 1626 |
| candidate | replay `8e654529…` | 1610 | 6,404,307,749 | 208 | 1626 |

Preflights also reproduce the instance `icv1-fp13-tm13-9e2d11f6` with 18
abscissae in both arms. Replay records:
`replay_cc7d6141_interval_20261185.json`
(SHA-256 `23f2c36a3378109558bcc355f81067ca6854349f7aba9fd3faacd0b490624650`) and
`replay_cc7d6141_candidate_20261185.json`
(SHA-256 `792059e9b3069f302b86cc9538b99106fdb773c0f41daffd0f542d2b15ea38eb`).

## What this does and does not establish

Establishes: the recorded operation counts of the 13-bit panel are reproducible
from a named commit by a binary that self-reports that commit, and the source
difference between `a5503f4b` and `cc7d6141f2` (a factor-base registration in
`src/bin/ic/bench.rs` plus docs and research files) does not touch these paths.

Does not establish: reproducibility of the *timing* fields. `ns/op`
calibration, wall and child CPU vary with host load and were not compared
here; the replay was run while no other benchmark was active, so its
`ns/op` is not comparable to the stored one either. The check is on
deterministic solver work — calls, multiply-adds, relations, trials.

The path forward is now clean: any further panel can be run on a
commit-embedded build, and its stores will carry their own provenance.
