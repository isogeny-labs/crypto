# band_wcal progress (agent R)
- [x] read band.rs; counters chosen (FIELD_ARITHMETIC_COUNTERS)
- [ ] band.rs: class + WCal + band_with_wcal + calibrated report field
- [ ] src/cryptanalysis/ecbench/wcal.rs timing helper (+ mod.rs line) + CLI flags in src/bin/ecbench.rs (Band)
- [ ] tests; clippy; compat replay of 3 records
- [ ] NOTE.md, pcc lint-prose, problems/ status
Build: flock .../cargo.lock env CARGO_BUILD_JOBS=2 cargo test --release --lib ecbench::band
- band.rs edited (class, WCal, band_with_wcal, tests) - not yet compiled
- wcal.rs written (not in mod.rs yet; wait for test run bfzu12xxl)
- mod.rs wcal line, CLI flags (--w-cal*, Wcal subcmd) added; next: test+clippy
- NOTE.md written, lint 0. waiting on t2.log test run; then clippy, CLI replay, python check
- full ecbench test: only failures = 5 tests needing k20-s0 records.jsonl (absent S3 artifact, pre-existing) + my compat test (swapped to r26, rerunning t3.log). next: clippy, CLI replay vs committed band.json, python check
- band tests pass (4 pre-existing k20-s0 failures only). running clippy t4.log
- clippy clean (t4). next: build ecbench bin, CLI replay vs committed band.json, wcal run, report
- DONE: all steps; report sent
