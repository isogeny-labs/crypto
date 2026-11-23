VALIDATION REPORT: GB-SOLVER-SHOOTOUT-20261116 run-local (local diagnostic, non-admitted). Every result here is [pcc:unverified].

**Disposition: `insufficient_evidence` (not `validated`).** My recomputation from the data to the numbers passes. I held the disposition below `validated` for three reasons:
- The artifact set is not committed.
- The pre-data pins required by AMENDMENT1 (amendment-2, PA-3/4/5) are absent.
- The M8 abandon call rests on a rho CPU reading quantised to 4 ms clock ticks (F3).

If the main session commits these exact bytes (hashes below), a re-dispatch can upgrade the recomputation scope without rerunning anything.

## Binding
- The tree was at HEAD f8480787ec31523ec67885bfc1fc6d1a96027117, branch claude/fastest-grobner-base-algorithm-u74ipw. It was dirty; run-local/, analysis-local/, run_local.py and the native adapters are untracked. No commit sha binds this evidence.
- I validated a snapshot taken at 08:31Z: `/tmp/claude-0/-home-user/a3ce588d-65c6-5d5a-a7d0-cb936deafa77/scratchpad/snap_hashes.txt`, 38 files. I re-hashed at the end and nothing had changed. No producer process was running; the driver finished at 08:19:51Z with exit 0 and 56:17.76 wall.
- rows.jsonl `7c5c98f91cd6404f6308f06341731d60a23006d644f390eab06560877b2aedf1`
- cells.json `87d4b7a84b71c3f22167292f77f560d26c79419716e29c667b17af88f71b7f0f`

## Requirement results

| Requirement | Result | Evidence |
|---|---|---|
| manifest.json hashes | PASS, 4/4 | blinding_key.json, cells.json, driver.log, rows.jsonl all match |
| Harness and binary pins | PASS | cells.json `identities_start` / `identities_end`: 11/11 harness files and 4/4 binaries match the current files at start and end; ldd closure matches |
| Protocol hashes | PASS | Recomputed canonical sha (freeze block removed): v1 = 116b33f7…bfee, amend1 = c521e04c…fe858; file shas 9ab64347… and 1d9b44fa… |
| Seed rule | PASS | I regenerated the natural stream independently: 160/160 targets identical (x_R, sign, index, seed, rejections, ids); 2896 draws, 2736 discarded; planted seeds 5/5; rho d and walk_seed 1000/1000 |
| Curve and oracle facts | PASS | #E = 131174 by exhaustive count; factor base = the 14 abscissae in cells.json; 65572 liftable abscissae outside V; 1757 SAT, p_sat = 1757/65572; mean decompositions 1831/1757 = 1.04212 |
| Seeded relation sample, independent re-addition | PASS, 100/100 | Rule: `random.Random(int(sha256(rows.jsonl),16)).sample(pool,100)`, pool = 528 (row, relation) pairs. I also re-added all 528: 0 failures |
| Per-row oracle consistency | PASS | All 996 rows: `oracle_relations` and `verified_relations` (non-excluded rows) equal my independent oracle; strata, `planted_found` and `n_verified` are consistent |
| Boolean root sets | PASS | I enumerated every root of the chained-S3 system for all 166 targets myself. Every arm's root set equals mine (996/996 rows); `roots_per_multiset` and zero-abscissa counts match; 0 spurious roots |
| Primary endpoint and decision numbers | PASS, every point estimate matches | See below |
| Producer analysis reproducibility | PASS | extract.sh output and all 5 `out/` files are byte-identical when rebuilt from `rustc -O analyze_amend1.rs` (blind then unmask) |
| Rho determinism replay (validator scratch, same binary) | PASS | All 1000 per-DLP counters identical |
| Tamper control | PASS literally, weak | See F-tamper |
| Committed, frozen artifact set | FAIL | F1 |
| Pre-data pins / protocol compliance | FAIL | F2 (deviations disclosed in report.typ) |

Point estimates that match exactly:
- Rates: inherited-f4 0.786412, msolve-f4 0.0058527, cms-xor 0.0012501 rel/CPU-s.
- Mean CPU per target, SAT/UNSAT: inherited-f4 0.03531/0.03535, msolve-f4 4.9419/4.7446, cms-xor 24.937/22.164.
- Rate ratios: 4.68186, 134.366 and 629.085.
- Median log2 CPU ratios: −2.25509, −7.13861, −9.41836.
- Wilcoxon p, exact: 1.368456e-48 for the registered pairs; A/A 0.3689039 / 0.3345719 / 0.0094754.
- Native ops identical across passes on 160/160 targets for every arm.
- Max charged CPU, RSS, degree, matrix and planted 5/5 all match.
- Rho: A = 55.197, U = 111.296, 3.99507e-5 s per DLP, ns cross-check 3.718e-5.
- log2 distances: inherited-f4 22.333 / 24.327 / clock 19.543; msolve-f4 31.276 / 33.270 / 26.613; cms-xor 24.927 / 26.921 / 28.840.
- Bootstrap CIs agree to within Monte Carlo noise.

## Mismatches against the report
There is no REPORT.md anywhere in the study. I compared against analysis-local/report.typ, report.pdf, summary*.json and registered.tsv. `benchmarks/20261117-gf2-semaev-solver-shootout/results-report.pdf` is byte-identical to report.pdf, so it carries the same errors.

1. **cells.json hash in the report.** The report gives `87d4b7a8…b71c3f0f`; the actual hash ends in `…f71b7f0f`.
2. **analyze_amend1.rs hash in the report.** The report gives `d62848b4…514223a`; the actual hash is `d62848b44c636b1664368d1481ad78e8b750a764025262a47a77af38f75db3f3`. The file at the actual hash reproduces every output, so the text is wrong, not the file.
3. **"log2 distance: inherited-f4 w=0: 0".** summary.unblinded.json has ratio 0.0 and `log2_distance` null: C_arm(0) = 0, so log2 is −∞. Printing "0" reads as parity with rho.
4. **"|median log2| ≥ 2.26".** The smallest is 2.25509 (C-msolve-cms), so the bound should be ≥ 2.25.
5. **msolve A/A floor 0.0230.** My bootstraps over 5 seeds give 0.0211–0.0216. The bootstrap-median distribution has a gap at the 97.5% rank: it jumps from 0.0223 to 0.0230, and P(≥ 0.0230) = 0.0228 in my resamples. This is Monte Carlo variation, not an arithmetic error, and no verdict changes.

## Findings
- **F1. No commit binds the evidence.** The run was launched from a dirty tree, and the evidence is uncommitted.
- **F2. Pre-registration was not met.**
  - Amendment-2 was never filed, so PA-3, PA-4 and PA-5 were not pinned before data.
  - protocol.amend1.json `stopping_rules[3]` says the run does not start without PA-1..PA-5.
  - The analysis script was written after the data, and the blinding key was printed before phase 1. Both are disclosed.
  - `analysis_plan.script` still pins analyze.py v1 (0b0eee39…, which matches the file), although the amendment declares v1 unused.
  - PA-4 names the driver `run_amend1.py`; the run used `run_local.py`.
  - Mitigation: my recomputation from the frozen metric definition reproduces every number.
- **F3. The M8 abandon call is not resolved at the stated precision.**
  - The rho CPU figure of 3.995e-5 s per DLP averages 990 readings of 0.0 and 10 readings of about 4 ms.
  - A Poisson 95% interval on 10 ticks scales that figure by ×0.48 to ×1.84, which puts inherited-f4's clock distance between 2^18.66 and 2^20.60. That range straddles the 2^20 abandon threshold.
  - The report's interval [19.47, 19.61] carries only the arm-side bootstrap.
  - The ns cross-check gives 2^19.65, which supports "not triggered".
  - Cause reproduced: the RLIMIT_CPU cap on the child made the process CPU clock tick-granular. With `ulimit -t 60`, 991/1000 readings are 0; without the cap, 0/1000 are.
  - The solvers' wait4 CPU readings are not quantised: none of the 498 values sits on the 4 ms grid.
  - t_GA was measured with the same quantised clock. It is not used in the endpoint.
- **F4. The rho price in the band is narrower than band.rs's rule.**
  - C_rho counts walk additions only. band.rs's G for a rho record also charges setup and verification; the adapter recorded 17 setup additions plus 34 setup and 1 verification scalar multiplications per DLP.
  - Adding only the 17 setup additions lowers the distances by 0.39 bits (w=0), 0.33 bits (w=0.1) and 0.14 bits (w=1). The scalar multiplications stay unpriced.
  - The formula was pre-registered, and this choice overstates the gap to rho.
  - U_rho matches ecbench StrongRho's canonicalisations plus partition hashes (frobenius_maps is exactly 17 × canonicalisations).
  - There is no ecbench session, so `ecbench band` cannot be run.
  - The reference is `koblitz_signed_frobenius_rho_with_progress`, not `rho.signed_frobenius_strong` (AGENTS §12). That choice was pre-registered as PA-2.
- **F5. The rho solves carry no independent certificate.** The recovered logs are not recorded, only booleans. My replay shows determinism, not correctness.
- **F6. Build provenance is incomplete.**
  - Both binaries exist only in the session scratchpad. They were built from eb01341d3a plus added files, not from HEAD, and the report does not say so.
  - build_identity.json (07:05:55) predates the gc_rho_match build (07:08:50) and has no entry for it.
  - build_native.sh was edited at 07:17:22, after both builds.
- **F7. The evidence is not durable yet.**
  - rows.jsonl is 3,183,337 B. That is a .jsonl over 64 KiB, so the artifact-store policy (§13) requires S3, which is unavailable.
  - The benchrec record is still open.
  - The run did not capture the msolve version string (only the binary sha and libneogb-0.6.5).
- **F8. The headline needs a scope qualifier.** "inherited-f4 is fastest" holds only among the 3 available arms; Magma, msolve 0.11 SIMD and Julia/trace-f4 are missing. CMS native ops have K/M/G print resolution, so their "identical across passes" check is coarse.
- **F9. The clock comparison is slightly asymmetric.** Arms are charged child CPU including process start-up (about 2 ms of inherited-f4's ~35 ms); rho's in-process CPU excludes start-up. This favours rho.
- **F-tamper. Half the tamper checks cannot fail.** The six checks reproduce, but 3 of them flip U bits (27, 17, 22). Those witnesses are not roots of the system, yet they still project to oracle relations. Only bits 6, 11 and 7 actually test rejection.

## Lessons ledger
- Tests recorded: T-913f881e3ea0 (confirms L-546d37ca7750) and T-0e8e3fe4b337 (confirms L-99ee952350b2). All 28 jq paths in extract.sh exist in all 996 rows.
- New candidate lessons: L-f853b9e6de95 (RLIMIT_CPU quantises the process CPU clock) and L-dc920790fc4e (tamper flips outside the decoded variables cannot be rejected).
- Outcome recorded: O-6afa5daa7ade.
- These wrote new files under `/home/user/crypto/docs/lessons/entries/`; they are not committed.

I edited no evidence. I did not commit, push or open a PR.

Files are in `/tmp/claude-0/-home-user/a3ce588d-65c6-5d5a-a7d0-cb936deafa77/scratchpad/val/`:
- indep_verify.py
- endpoint.py
- roots_check.py
- stream_check.py
- tamper_check.py
- indep_verify.out
- endpoint.out
- rootsall.out
- repro/
- rho_replay/