# negation_remeasure progress (agent Q, 2026-11-13)

Resume: read this file first. Wrapper for every cargo call:
`flock /tmp/claude-0/-home-user/b3fd4014-06f7-5e0c-8338-36d52644c351/scratchpad/cargo.lock env CARGO_BUILD_JOBS=2 cargo ... --release`

## Done
- [x] branch fast-forwarded to origin/main 47dc95a934 (clean tree).
- [x] read REPORT.md (validation), instrument_fixes PROTOCOL/RESULTS, instrument.rs, instrument_rerun.rs.

## Pending (in order)
1. DONE PROTOCOL.md frozen 8771b2dc...cba86 (frozen.sha256) at 03:00Z
2. WRITTEN (not yet compiled): src/cryptanalysis/chained_transfer/negation_remeasure.rs (+ mod.rs line), src/bin/negation_remeasure.rs -> next: cargo check, fix, then tests
3. tests, clippy
4. bench record (benchrec init/run/add-raw/metric; leave open)
5. runs: v3, cover, tower (<=16001), tower big (32003, 64007 if time)
6. RESULTS.md, remeasure.json, remeasure.svg, run.log, lint-prose

## Notes
- bench record created: benchmarks/records/20261113T0320Z-negation-remeasure.json (export BENCHREC=...; leave OPEN)
- cargo is shared: another agent's `cargo test --release --lib` holds the flock; wait.
- Output parts: remeasure_{v3,cover,tower}[tag].json; `negation_remeasure merge` -> remeasure.json + remeasure.svg
- Run commands (after build), from /home/user/crypto:
  python3 tools/benchrec.py run $BENCHREC -- ./target/release/negation_remeasure v3 cover --out-dir research/negation_remeasure_20261113
  ... tower (default primes <=16001), then `--tag _big --tower-primes 32003,64007 --tower-instances 1 --tower-old-seeds 2 --tower-seeds 2`
  then `merge`.

## State at 04:15Z
- binary compile errors fixed (closure moves); cargo build of bin running (lib recompiles each time other agent edits the shared tree; ~10 min).
- render_tables.py written (reads remeasure.json -> tables.md, summary.json).
- NEXT: when build done -> run `cargo test --release --lib chained_transfer::negation_remeasure`, then runs.
- 04:22Z v3 + cover DONE under benchrec (parts remeasure_v3.json, remeasure_cover.json, gates reproduce). NEXT: tower <=16001, then tower big (tag _big), merge, tests, clippy, record metrics
- 04:25Z tower <=16001 DONE (remeasure_tower.json). Launched big.sh in background (32003 then 64007; tags _p32003,_p64007; output appended to run.log). Check run.log for 'stage ... exit'. 64007 uses --tower-old-seeds 4 --tower-seeds 2 (deviation).
- 04:23Z waiting on big.sh (run.log) and cargo test (scratchpad/nr_test.log). After: merge, render_tables, metrics, RESULTS.md, lint-prose, clippy.
- 04:37Z tower 32003 & 64007 DONE (3 inst each, gates reproduce). Parts: remeasure_tower_p32003.json, remeasure_tower_p64007.json. TODO: rebuild bin (adds curves_checked), run 'tower --tag _aut --tower-old-seeds 0 --tower-seeds 0 --tower-rho-seeds 0 --no-calibration --tower-primes 251,...,64007' to list j of all instances; merge; scorecard.py; render_tables.py; add_metrics.py; tests; clippy; RESULTS.md
- 04:47Z cargo test --lib chained_transfer: 61 passed (incl. 8 new). Launched post.sh (bin rebuild -> scratchpad/nr_build2.log, then clippy -> scratchpad/nr_clippy.log). Parts merged; scorecard N1,N4,N7,N8,N9 hold; N2,N3,N5 refuted; N6 pending aut run.
- 04:59Z bin rebuilt; launched aut-check run (tag _aut) -> run.log 'stage aut exit'. clippy queued (scratchpad/nr_clippy.log)
- 05:08Z ALL DONE: 214 metrics + 9 raw files added to record (OPEN); benchrec report regenerated; RESULTS.md lint 0; PROTOCOL.md lint 5 (frozen). Not committed/pushed per instructions.
