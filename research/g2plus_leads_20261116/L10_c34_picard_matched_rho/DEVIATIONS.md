# L10 deviations from PROTOCOL.md / SPEC.md (executor record; thresholds, arms, seed rules and decision rules unchanged)

All numbers are [pcc:unverified]. PROTOCOL.md and SPEC.md are untouched (sha256 equal to frozen.sha256 after the run).

1. **`cargo test` and the lab file.** SPEC 1 says the lab file "compiles standalone". It does for
   `cargo build`, but under `cargo test` its own `#[cfg(test)] pub(crate) mod tests` is compiled too and
   fails with `use of unresolved module serde_json` (lines 2243 and 3293 of plane_quartic_pic.rs; no
   dependencies are allowed, and the lab file may not be modified). Minimal fix: the unit tests U1-U11
   of SPEC 11 are ordinary functions in module `selftest` of `src/main.rs`, run by the subcommand
   `l10 selftest --name uN_*`, and `tests/unit.rs` (integration test, one `#[test]` per U-test) calls the
   binary. `Cargo.toml` got a `[[bin]] name="l10" path="src/main.rs" test=false` stanza so that cargo
   does not compile the bin with `--test`. The frozen command `cargo test --release --offline -j1 --
   --test-threads=1` ran unchanged: 11 tests passed (raw/test.log). The lab file's own tests were not run.
   U8 re-implements the empty-row reduction inline (it checks the reduction rule and `orbit_key`, not the
   private reduction code path inside `run_search`); the same reduction is exercised by U7 and by T6/T7.
2. **build.log.** The first logged `cargo build` found the binary already built (while the code was being
   written), so it prints only "Finished". A second build after the diagnostic edit of item 3 was appended.
3. **tiny re-run with diagnostic fields.** After the first `tiny` run, the tiny subcommand was extended with
   diagnostic-only JSON fields (`diag_T7c_kerdim`, `diag_orbit_kerdim_ell_lambda`, `diag_orbit_kerdim_ell_lambda2`),
   and tiny was re-run for all five primes. Gate fields of the first run (raw/tiny_superseded_v1.jsonl) and
   of the second (raw/tiny.jsonl) are identical (checked by script). No gate or threshold was changed.
4. **rhocost timing probe.** Three rhocost commands (Q, C, P; p=2053, seed 1) were run as a timing probe and
   logged; `raw/rhocost.jsonl` was deleted and the frozen rhocost loop re-run in full (27 lines).
5. **T7b status for a curve** (SPEC 10.1 gives one T7b field per curve but two arms): the field is the worse
   of the P_full and P_orbit statuses (determined > underdetermined > fail). Both statuses, both kernel
   dimensions and the cap-hit flags are written as extra fields at the end of the tiny line
   (`T7b_full`, `T7b_orbit` as 2/1/0 = determined/underdetermined/fail, `kerdim_*`, `cap_hit_*`, `T8_w_r`,
   `T8_w_ell`). `ok` for a tiny Picard curve requires T7b != fail, not "determined".
   The count of curves with T7b determined is 7 of 10 under this rule and 9 of 10 for the orbit arm alone.
6. **T1** also requires the number of brute-force points to equal N1 from the point count (extra check).
7. **dup / trivial order** (SPEC 6 step 7 is read literally): if the key was already seen -> `dup`, else the
   key is stored; then empty row -> `trivial`, else `accepted`. This affects only the split between `dup` and
   `trivial`, never `trials` or `accepted`.
8. **Tiny tier, rows fewer than columns.** For C and Q at the tiny tier hitting the cap (60 x rows_target) is
   treated as in the P arms (not a failure; T6 is checked on the rows collected). For T8, if fewer than
   n_cols_orbit accepted rows exist, the square system is padded with zero rows (the Krylov count depends only
   on n and w).
9. **`n_fixed`** counts P0 and the affine points with y = 0 (N - 3 n_orbits); it is also printed for P_full.
10. **Resource limit event at p = 16411** (SPEC 13): `measure --p 16411` was stopped by the 600 s CPU limit
    after 23 of 24 rows. The instance set was not changed. Missing row: Q_full, curve 3, solver seed 2.
    See raw/spec_issues.txt. Measure commands for p <= 8209 were launched from a loop and, because of the
    tool's 2-minute foreground limit, moved to background by the harness; the commands themselves are unchanged.
11. **analyze.py interpretation choices** (SPEC 12 leaves these open): bootstrap interval = sorted[50] and
    sorted[1949] of 2000 resampled geometric means; PC ratio = geomean over the 6 big-p runs divided by the
    geomean over the 6 small-p runs, for `trials` and for `n_cols+10`; H3' uses the paired geomean ratios;
    `G(p)` uses the rhocost prime nearest in log p (8209 for p >= 8209); proxy distance uses the geometric
    mean of the P_orbit `fpm_total`. H3 is reported "inconclusive" when any gate T1-T8 (tiny tier), M1-M5,
    A/A or PC (used primes) fails; the numeric-only verdict is printed next to it in analysis.json
    (`H3.numeric_only_verdict`) and in tables.md. PROTOCOL 7 does not say whether tiny-tier gate failures
    count as "at a prime used"; the conservative reading was implemented.
12. `code/target/` (2 MB build output) is left in place; `raw/binary.sha256` records the binary's hash.
