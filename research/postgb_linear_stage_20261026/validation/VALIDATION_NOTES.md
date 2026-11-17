# Validation notes, round 2: postgb_linear_stage_20261026

This is an independent validation of snapshot `6f356f71ef8b5392fe27da22170b914e37109390`
(branch `feat/sleepy-tesla-1ucug8`), made on 2026-10-27. It covers amendment 3, the R3
cards, the native summary, and the negation-map rho reference. The validator did not
produce the study.

**Inputs.** Every input was read with `git show <commit>:<path>`. The working tree was
not used. `examples/postgb_la_summary.rs` was neither read nor run.

**Target commit.** The work started on 12a5bba630. The Coordinator then moved the
target to 6f356f71ef. Between the two commits no file under `runs/` or `cards/` and no
protocol file changed. Only `summary.json`, its generator, the write-ups and the
leaderboard changed.

## Tooling (AGENTS.md, "Implementation language: no Python")

The validator's tools are now one Rust crate, `vtool/`. It is a standalone Cargo
package with its own `[workspace]`. Its only dependencies are `serde_json`, `sha2` and
`rand` 0.8, and it uses no repository code.

- `vtool sample` draws the nonce sample.
- `vtool verify-scalars` is the scalar check. It uses the validator's own F_p,
  F_{p²}, F_{p⁶} and curve arithmetic.
- `vtool recompute` recomputes the summary and checks receipts, coverage, gates and
  the rho band.
- `vtool check-replay` compares the replays and writes the certificate.

`round2/run.sh` is the only shell piece. It is thin launch orchestration: it hashes
and refuses a non-pinned binary, runs it, and appends a journal line.

**Build:** `CARGO_TARGET_DIR=<scratch> cargo build --release --offline` in `vtool/`.

**Python removed.** The round-1 Python tools are deleted from the tree:
`recompute.py`, `verify_scalars.py` and `replay/{sample,run_replay,run_dump_replay,check_replay}.py`.
They stay in git history (8165804332).

**Round-1 outputs kept.** The round-1 outputs are kept as historical evidence of
Python provenance and are superseded by round 2: `replay/`, `scalar-verification.json`
and `scalar-dump/`.

## Status

| file | status |
|:--|:--|
| `validator-report.json` | `pass`, `independent: true`. 276 checks, 0 problems, 693 values compared, 151 headline numbers at rel_tol 1e-9, 0 mismatches. |
| `replay-certificate.json` | `verified`, `independent: true`, `scalars_verified: true` (gate checker: no errors). It is backed by the full R3-C replay (253/253 systems, counts exact), 2 DLP replays, and the scalar verification. |
| `round2/scalar-verification.json` | The 8 R3-D instances, the 8 round-1 dumps and the validator's 2 replay dumps all verify. |
| `validation_report.yaml` | Role-contract report, verdict `passed`. |

**Intermediate finding on 12a5bba630.** That commit failed on 17 `counted_ratio`
fields:
- In `dense_over_fast` and `host_a_dense_over_fast`, the figure labelled dense/fast
  was R2 sparse/fast.
- `ablation_sparse_over_portable_fast` and `host_a_sparse_over_fast` used the pre-fix
  formula counts.
- `P4.counted_outside_25pct` = 121 was in the sparse/fast direction.

The producer fixed these in 3b4ec13ae0 and 6f356f71ef. The dense, ablation and host-A
blocks now carry no counted ratio, because no fixed-counter counts exist for them. P4
now counts fast/sparse outside [0.75, 1.25]: 97 systems, made up of 85 dense,
8 gaudry-s4, 2 gaudry-d4, 2 gaudry-s5 and 0 jv-nagao. The validator recomputed all of
these.

## Replay (round 2)

- **Nonce.** `390ee4d7e27df3970a4bddaa8a10b09ea36962f5d9e9f12e81fdf86330c1faac`, from
  `/dev/urandom`, drawn at 2026-10-27T09:00:14Z. That is after commit 12a5bba630
  (08:54:36Z). The target 6f356f71ef came later, but changed no run record.
- **Pinned binaries.** `postgb_la_bench` 06351d89… and `jv_cover` a49184b0… were copied
  from `target/` the moment their hashes matched the R3 cards' binds. The copies were
  used throughout.
- **Sample.** 14 R3-C systems, drawn in `round2/sample.json`:
  - jv-nagao: p53-3, p101-9c, p53-7
  - gaudry-s4: p1039-2, p1039-6, p8191-1
  - gaudry-d4: p4093-3, p1033-1, p4093-5
  - gaudry-s5: p13-0
  - dense: n6-d3-p32-2, n5-d2-p31-0, n3-d4-p32-0, n6-d2-p16-0
- **R3-C was replayed in full.** The counted multiplications depend on each system's
  index `si` in its invocation, and R3-C ran every family in one process. So the only
  way to reach the sampled systems at their recorded `si` was to replay the whole
  R3-C card command: 253 systems, sparse and fast, `--reps 1`.
- **R3-C result.** All 253 systems equal the recorded rows in every non-timing field,
  `muls` included (253/253). That covers sols, give-up, fallback, and the fast arm's
  krylov, bm, nf, roots and readout counters with its delta, dense_cols,
  functionals_tried, nnz, nf_rows and roots. The 14 sampled systems are among them.
  The counts are exact.
- **DLP.** The nonce chose one R3-D instance per arm: p=53 seed=1 for both. Each ran
  with the pinned `jv_cover`, `JV_RHO_NEG=1` and `JV_DUMP_INSTANCE`. Both replays equal their R3-D1 and R3-D2 rows on all 57 non-timing fields: `lin_muls` 4032563426 (sparse) and 3922354568 (fast), relations 31, S 731.5903153798725 and 727.1174163750129, solved and correct, and the 4 rho and 4 rho_neg runs. Their instance-dump lines are byte-identical to R3. Timing fields (`f4_ms`, `lin_ms`, `wall_ms`) were not compared.
- **Journal quirk.** The `env` of the R3-C journal line reads `[""]`. That is the
  shell's rendering of an empty environment list; no variable was set.

## Scalars

`vtool verify-scalars` checks every dumped instance with its own arithmetic.

**Instances covered.**
- R3-D1 and R3-D2: 8;
- the round-1 `scalar-dump/` lines: 8;
- the validator's own replay dumps: 2.

**Checks on each instance.**
- w is a non-residue mod p, by Euler's criterion and by finding no root of i² − w.
- s is a non-cube in F_{p²}, by Euler's criterion and by finding no root of θ³ − s.
- The curve is non-singular, and G and Q lie on it.
- l is prime (Miller–Rabin and trial division), G ≠ O and l·G = O.
- dd·G = Q, dd = d, and 0 < d < l.

**Self-tests and negative controls** behaved as required on every instance.

**Ties to the measured runs.**
- Each dumped (p, seed, l) is that of its R3-D row, and that row is solved and
  correct.
- The R3 dump lines are byte-identical to the round-1 dumps (R2-equivalent runs).
- The sparse and fast instances are identical.
- The validator's own dumps are byte-identical to R3's.

**Extras (not used to decide the result).** On each curve 4l lies in the Hasse
interval and kills random points, and the 2-torsion is rational, consistent with
#E = 4l.

## Recompute (`vtool recompute`, 6f356f71ef)

**What matches.** Every number in `analysis/summary.json` matches its recomputation
from the raw R2 and R3 runs:
- **Corpus blocks:** the per-family geometric means, minima and maxima of sparse/fast
  and dense/fast (R2 wall time), and the upper medians.
- **Bootstrap CIs:** the validator's own `StdRng::seed_from_u64(20261027)` bootstrap,
  4000 resamples and nearest-rank, reproduces every interval within 8e-16.
- **Counted ratio:** `sparse_over_fast` counted ratio from R3-C.
- **Predictions:** P1–P5, Spearman ρ = 0.93080369034499, give-up fraction 1/253,
  P4's 97 and its range, the ablation geometric means, and `portable_over_simd_fast`.
- **DLP block:** S, S/rho and S/rho_neg from R3 with S rebuilt from its counters;
  `lin_ms`, `f4_ms` and `wall_ms` from R2; the sparse/fast S and `lin_ms` ratios;
  relations; correctness.
- **Bookkeeping:** gates, the 253 `systems` entries (R2 medians, R3 `muls`, R2
  `muls_r2`, fast phases) and the meta and skipped blocks.

**Receipts.** All 13 R2 and R3 receipts are `completed`, every command exited 0, and
each card hash equals its card.
- The R3 cards bind the R3 pins, PROTOCOL.md and amendments 1–3.
- The output hashes (including the `instances.jsonl` files), the stdout, stderr and
  `lscpu` hashes, and the launcher logs all match the committed files.
- The cards ran one at a time: R3-C, then R3-D1, then R3-D2.
- The timeouts are 7000 s, and the env and argv are those of amendment 3.
- All R2 and R3 cards ran on host B, with byte-identical `lscpu`.

**R3 gates.**
- R3-C's solution sets equal R2's on all 253 systems, along with agree, delta,
  planted, give-up and fallback.
- R3-D's relations, solved logarithms and plain-rho runs equal R2-D's.
- All 32 rho runs and all 32 rho_neg runs are correct.

**Scope of the counter fix.** The fix does what amendment 3 says and nothing more:
- At the same `si` (the first 96 systems), R3-C's sparse counts equal R2's (96/96).
- The fast arm's non-root counters equal R2's (96/96); only `roots_muls` moves
  (95/96).
- R3-D1's rows differ from R2-D1's only in the new rho_neg fields.
- R3-D2's rows differ from R2-D2's only in `lin_muls` and the counts derived from it:
  `oracle_muls`, `total_muls`, s, s/rho, c_cov, relation_over_rho,
  predicted_s_over_rho.

**R2 records unchanged since round 1.** The R2 run files are byte-identical to the
round-1 snapshot.

**Conventions.**
1. `sparse_over_fast.counted_ratio` and `P4.counted_ratio_range` are in the sparse/fast
   direction, while `P4.counted_outside_25pct` counts fast/sparse outside
   [0.75, 1.25]. In fast/sparse terms the range would be [0.467, 1.300].
   Recommendation: label the direction of the range.
2. The median over an even number of systems is the upper median.
3. `portable_over_simd_fast` is the geometric mean of per-system portable/SIMD fast
   ratios.

## Rho reference (for VALIDATION.json)

**Strongest rho on this curve: yes, the negation map, `kind: negation`.**
- On all 8 dumped curves (4 distinct instances), the j-invariant is neither 0 nor
  1728. The checks are c₄ ≠ 0 and c₆ ≠ 0, with the identity 1728Δ = c₄³ − c₆²
  verified. So Aut(E) = {±1}.
- j is in neither F_{p²} nor F_{p³} (j^{p²} ≠ j and j^{p³} ≠ j). So E is not
  isomorphic, even over the closure, to a curve defined over a proper subfield. That
  rules out any Frobenius-type class speed-up.
- The claim "only ±1, no Frobenius acts" is confirmed by computation, not taken from
  α. α is not dumped; j decides the question.
- `rho_e_neg` uses the negation map with look-ahead against fruitless 2-cycles. Longer
  cycles end in a db = 0 collision and a restart. Every group operation is charged:
  setup, look-ahead candidates and doublings.

**Same unit: yes, with one convention.**
- IC S = counted F_p multiplications / (c_add_E · √l), where c_add_E is the measured
  F_p-multiplication cost of one generic E-addition (`unit_costs`).
- rho S = counted group operations / √l. Additions and doublings each count one.
- So both are group-addition equivalents per √l.

**Same process: yes.** `rho_e_neg` runs in the same `jv_cover` process, on the same
`Spec` (curve, G, Q, l), after the IC clock, with seeds seed·1000 + k. Its 4 runs are
identical in the sparse and fast processes.

**Matches amendment 3: yes.** Each R3-D row has 4 `rho_neg` runs, and
`s_over_rho_neg` = S / mean(rho_neg S). The validator recomputed this from
`group_ops`/√l.

**§5a band: S is not the same at 0, 0.1 and 1 GAE.**
- `rho_e_neg` leaves native work unpriced, and the binary emits no counter for it.
  Per step there is 1 table lookup, 1 insert and 1 partition hash. Per candidate
  addition there is 1 canonicalisation (an F_{p⁶} negation and a compare), 1 hash and
  2 coefficient updates.
- From the emitted `steps` and `group_ops`:
  - lower bound u_lo = 5·steps, the §5a-listed operations only;
  - upper bound u_hi = 3·steps + 4·group_ops, all non-group native work.
- The IC also emits no unpriced counter (F_p additions, hash lookups, F4 symbolic
  work). Its S at w > 0 is therefore a lower bound.
- The IC "stays the same across the band" only because nothing is emitted. It is not
  complete.

**Figures** (fast arm S against the mean S of the rho_neg runs; rho given as
lo / hi):

| | w = 0 | w = 0.1 | w = 1 |
|:--|--:|--:|--:|
| pooled, 4 instances / 16 rho_neg runs: IC fast mean S (lower bound) | 408.25 | 408.25 | 408.25 |
| pooled rho_neg mean S | 0.8210 | 1.1956 / 1.3741 | 4.5671 / 6.3525 |
| pooled IC / rho_neg | 497.3 | 341.5 / 297.1 | 89.4 / 64.3 |
| p=53 seed 1: fast S 727.12, rho_neg 0.5549 | 1310.5 | 912.5 / 788.6 | 244.4 / 172.0 |
| p=53 seed 2: fast S 326.45, rho_neg 0.6450 | 506.2 | 351.7 / 304.3 | 93.9 / 66.3 |
| p=101 seed 1: fast S 220.42, rho_neg 0.6881 | 320.3 | 218.6 / 190.8 | 56.7 / 41.1 |
| p=101 seed 2: fast S 359.02, rho_neg 1.3959 | 257.2 | 175.2 / 153.0 | 45.3 / 32.9 |

Pooled by p, the rows are in `validator-report.json`, `rho_reference.pooled_by_p`.

**Proposed `gae_band`.** This uses the pooled figures and the conservative upper bound
for rho, so the distance remaining is not overstated:

```json
{"0": {"ic": 408.25, "rho": 0.821}, "0.1": {"ic": 408.25, "rho": 1.374}, "1": {"ic": 408.25, "rho": 6.352}}
```

Mark ic as a lower bound at 0.1 and 1.

**The order holds across the band.** rho_neg is cheaper than both IC arms on every
instance at every weight: the IC lower bound is above the rho_neg upper bound. The
IC's missing unpriced work can only widen the gap. So `order_holds_across_band: true`,
with the IC behind rho and the distance remaining between 33× and 1310× per instance
(pooled 64×–497×).

**Caveat: rho_neg sample size.** The rho_neg means rest on 4 runs per instance and are
noisy:
- the per-instance means range from 0.55 to 1.40, and at p=101 seed 2 the rho_neg
  mean exceeds the plain-rho mean;
- the pooled steps/√l of 0.749 is below the theoretical √(π/4) = 0.886.

The pooled figure is the better reference, and more rho_neg runs would tighten it.

## Issues and limitations carried forward

1. **Planted points outside the dense family.** The bench records no planted point
   for the constructed jv-nagao systems. All 16 have at least one solution.
2. **Dense-family F4 budget.** PROTOCOL.md's text says 900 s for the dense family;
   the code uses 600 s. No system timed out.
3. **DLP `lin_ms`.** It is summed inside a 4-thread pool, not single-threaded wall
   time.
4. **Aborted preflights before R3-C and R3-D1.** These were launched at 08:03:40Z and
   08:04:00Z, close together. R3-C's was terminated (exit 143). Neither left a receipt
   or an output. The real cards' `output-absent` checks passed later. No run record
   comes from them.
5. **Binaries not rebuilt.** Neither binary was rebuilt to confirm its source. Both
   are bound by sha256. R3's mtimes (07:56:39Z) precede the commit 30cef72a5b that
   records their source.
6. **Concurrent work in the worktree.** The producer kept working in the shared
   worktree during validation; for example, `summary.json` was modified after
   12a5bba630. Only committed snapshots were read.
7. **Round-1 artifacts are historical.** Their journals were produced by the deleted
   Python tools.

## Not assessed

- Whether the speed-up matters.
- The leaderboard and benchmarks rows that 3b4ec13ae0 and f393dac1f0 changed. The
  `VALIDATION.json` that would name these files is for the Coordinator.
