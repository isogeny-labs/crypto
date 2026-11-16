## 2026-11-16T05:50Z — ±P pairing in the linear 2+1 scan, against the repository arm (Claude, pilot subagent)

For the main session to fold into `docs/session-logs/2026-11-16.md`; this pilot wrote only inside
`research/scan_pm_pairing_20261116/`.

### What was done (commands, files, features)

- `python3 tools/lessons.py brief --scope research-pilot` read first.
- Read `examples/ic_algebraic_floor/linear21.rs`, `src/cryptanalysis/koblitz_fast.rs`
  (`add_many`, `add_many_lazy`, `add_many_lazy_paired_x`, `add_many_lazy_soa_paired`),
  `Gf2::batch_inv`; `research/scan_floor_leaf_20261116/README.md` and `SKEPTIC.md`.
- `cargo build --release --example ic_algebraic_floor` at HEAD `8d039abd94`: built, 11 min 09 s.
- Harness `main.rs` + `linear21_pm.rs` (Rust), compiled with `rustc` against the cargo-built `rlib`
  (`build.sh`). Arms A1/A2 (repository `linear21.rs`, byte-identical copy), BC, PM, LZ, LP.
- `PROTOCOL.md` committed alone as `b2836595af` before the measured run; one measured run (`run.sh`,
  seed 20261116, 8 cells n = 23..59, 632 targets); `tables.md`, `results.json`, `raw/`.
- Verified by this pilot's own gates (all `[pcc:unverified]`, no independent review): verdict, first
  witness and decomposition sets equal across arms; exhaustive enumeration at n = 23, 29, 31.

### Decisions made (and why)

- Did not stop at "already present": the library pairs the addition's denominator
  (`add_many_lazy_paired_x`, `add_many_lazy_soa_paired`) but `lin21-ht` uses neither, and nothing
  shares its second inversion.
- Added arms LZ and LP (the library's lazy and paired additions) as the comparators the earlier pilot
  lacked; rule evaluated on field operations (verdict of record) and with the elimination priced.
- Result under the frozen rule: PM fails (counted 1.056, 1.053, 1.051 at n = 47, 53, 59; wall 1.013,
  1.000, 0.997, inside the A/A band); LP inconclusive (counted 1.082, 1.077, 1.072 modelled; wall
  1.044, 1.034, 1.019). The t stage is 2.4 to 3.5% of the repository arm's wall time.
- No benchmark record, leaderboard or `benchmarks/` change: shared container, no validator.

### Open threads (unfinished, pick up next time)

- The leaf is 97% of `lin21-ht`'s wall time (cross-crate product calls, tracked elimination); a leaf
  inside the library is the next thing to measure, on an admissible host.
- Skeptic review of this pilot has not been done.
- Candidate lesson (not recorded; `tools/lessons.py` writes outside the pilot directory): a stage
  share from a standalone port does not transfer to the repository arm; take the wall stage breakdown
  of the real arm before piloting a stage.

### Errors (what went wrong, how it was handled)

- First rustc link line named a non-existent `libcrypto_lib.rlib`; the file is under `deps/` with a hash.
- Development seed: PM trial counter off by one on early stops; paired t stage kept the table-path
  branch because the field was reached through a second pointer. Both fixed before the freeze.
- `research-kb`, lead atlas, `ecdb` not available in the container; not queried.
