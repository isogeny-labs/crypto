# band_wcal follow-up 3 (agent X) progress — resumable checklist
Items: (1) pooled compare calibrated block; (2) `--unit bitop/v1` calibrated (spec check); (3) `--w-cal` override on compare/challenge/bounds.
Build: flock <scratchpad>/cargo.lock env CARGO_BUILD_JOBS=3 cargo test --release --lib ecbench
- [ ] read code (compare.rs, pooled.rs, challenge.rs, bounds.rs, band.rs, runner read_wcal, bin/ecbench.rs)
- [ ] item 1  - [ ] item 2 (spec?)  - [ ] item 3  - [ ] tests  - [ ] rustfmt/clippy/backend-leaderboard  - [ ] NOTE.md + lint-prose
- [x] read code; items 1,3 written (band.rs scaled grid+source, compare.rs override, pooled.rs PooledCalibrated, challenge VerdictInputs.w_cal, bounds FitOptions.w_cal, bin WCalOverride). item 2: STOP (BITOPS.md: bitop/v1 prices field_mul at frozen weights; no spec) — override refused with bitop. next: compile, tests
- tests written (pooled x4, band dir test, bounds, challenge); rustfmt done; NOTE.md section appended, lint-prose 0. next: compile+test (x1.log/x2.log), clippy, backend-leaderboard build
- x2: all new tests pass; 5 known S3 failures; records_without_w_cal test updated for override-in-id (rerun). next: rerun that test, clippy, backend-leaderboard
- clippy -D warnings clean (added allow(large_enum_variant) on ChallengeCmd like BoundCmd); backend-leaderboard builds. next: CLI smoke
- CLI smoke ok (override grid source=override, refusals for 0/-1/nan/inf and bitop/v1, flags on bound fit + challenge verdict). ALL DONE; handed back.
