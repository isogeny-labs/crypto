# band_wcal follow-up (agent S) progress
Build: flock /tmp/claude-0/-home-user/b3fd4014-06f7-5e0c-8338-36d52644c351/scratchpad/cargo.lock env CARGO_BUILD_JOBS=2 cargo test --release --lib ecbench
Design: RunOptions.w_cal (Option<WCal>, skip-if-none) in session.json; sidecar <session>/w_cal.json written by `ecbench wcal --session DIR`;
 runner::read_wcal(dir) reads either; band CLI: flags override, else all dirs must agree.
 band.rs: priced_s_split(r,wf,wo,pricing) + CalibratedGrid helper; compare OpsBand.calibrated, challenge BandVerdict.calibrated, bounds Size.calibrated (all skip-if-none).
 test: counter-name class membership (committed records + cost.unpriced literals in src/).
- [x] branch checked out clean at origin/main 28da34a03b
- [ ] band.rs helpers  - [ ] runner/RunOptions - [ ] CLI - [ ] compare - [ ] challenge - [ ] bounds - [ ] class test - [ ] backend-leaderboard build, build_ic_leaderboard.py - [ ] clippy - [ ] NOTE.md
- band.rs helpers, runner, compare, challenge edited (uncompiled). next: bounds, CLI, tests
- bounds/CLI/class test written+rustfmt'd (chained_transfer/ic_control.rs diff is agent T's); next: compile+test
- compile+test running (s1.log in scratchpad). after: tests for w_cal round-trip/compare, NOTE.md, backend-leaderboard build, clippy, python check
- first full test run ok except class test (fixed: phase-qualified names, linalg_mulmod). added compat tests; rerunning s2.log
- NOTE.md appended, lint 0. waiting s2.log; then backend-leaderboard build, python check, clippy, full ecbench test
