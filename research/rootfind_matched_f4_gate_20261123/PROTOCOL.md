# PROTOCOL (frozen before any data): matched genuine-F4 arm for program B's 2x/8x gate

Frozen 2026-11-23, before the calibration run, before any panel run, and before
re-running program B on its frozen NIST charts. Edits after this point go to
`DEVIATIONS` at the end of this file. They do not change the text above it.

## The gate as written (sources)

`research/rootfinding_checkpoint_20261109/v3/REPORT.md` line 101, repeated in
`v4/REPORT.md` and `rootfinding_metric_20261110_v1/REPORT.md`:

> 20 preregistered hard completed matching original-all-roots instances,
> fastest genuine baseline >= 1s, independent exact root-set equality,
> >= 3 cold runs, >= 2x geometric gain, <= 1.10 worst slowdown, <= 10% A/A
> spread, qualified isolation and required field/curve coverage. Requested
> gain 8x; immutable floor 2x.

**The 20 instances were never preregistered.** The closest frozen lists are
(i) the 5 planted NIST charts
`research/rootfinding_checkpoint_20261109/code/research/dual_ladder_20261109_v3/planted_inputs.json`
(sha256 403db508…), which are labelled "PLANTED_NOT_BENCHMARK", and (ii) the 20
"training" cases of `rootfinding_metric_20261110_v1`. List (ii) is
reconstructed from `raw-profile.json.gz` and
`research/curved_blocks_20261109_v6`, and neither is in this checkout. So list
(ii) cannot be rebuilt.

## Program-B solver under test

`primal_parabola_20261109_v5/native.cpp` (sha256 7e4464c8…) is program B's
newest exact all-roots chart solver with complete code in the repo. It is
rebuilt here from byte-identical sources by
`scripts/build_program_b.sh` (g++ -O2, system GMP). The missing include
`dual_lattice_20261109_v2/frozen_prefix.hpp` is supplied from
`dual_ladder_20261109_v3/frozen_prefix.hpp`, whose EXTRACTION.json names it as
that parent's prefix. Its contract takes the public chart `(p, M, t0..t2, u0..u2)`
and returns every ordered integer triple (x,y,z) in [0,M]^3 with
e2 ≡ t0 S²+t1 S+t2 and e3 ≡ u0 S²+u1 S+u2 (mod p), S = x+y+z. It exits with an
error unless exactly 5 cases are given, and it runs its own tiny brute-force
controls in every process. Both are charged to it.

Scope caveat, stated before data: this is the *conditional chart* contract.
Original S3/S5 auxiliary, ghost and exceptional branches stay outside it, as in
every program-B report. The panel charts below are synthetic charts in this
exact public contract, with uniform random coefficients. They are not derived
from curve data. The solver receives only `(p,M,t,u)` in either case.

## Panel A: the frozen NIST charts (feasibility of a matched arm)

Program B runs on the 5 frozen planted charts (P-192…P-521, M = 2^40−1 …
2^106−1), with 3 cold runs and the root sets checked against `expected`. A
matched genuine F4 arm must encode x ∈ [0,M] as the domain polynomial
∏_{j≤M}(x−j) of degree M+1 ≥ 2^40. Every lab F4 over a prime field
(`f4_fp`) also requires p < 2^32. These are recorded as structural facts:
**no matched F4 arm can be posed on Panel A**. Prediction: program B returns
the expected 3 ordered tuples per chart.

## Panel B: the measured matched panel

- Field: p = 4294967291 (largest prime < 2^32, inside `f4_fp`'s scope).
- Arms (same chart, same output contract):
  - **B**: program B, as above.
  - **F4-ideal**: `f4_fp::solve` (degree-bounded F4 plus substitution) on
    {f1, f2, D(x), D(y), D(z)}, with D(v) = ∏_{j=0}^{M}(v−j). Degree cap 3(M+1)+3.
  - **F4-visit**: `f4_fp::solve_visit` on {f1, f2} with domains [0..M] per
    variable (the lab's F4 with domain branching). Degree cap 12.
  - F5/F6: no F5 or F6 implementation over odd prime fields exists in the repo
    (`matrix_f5_f2`, F6-IC and the inherited F4 are characteristic 2). These arms
    are recorded as **not available**, not run.
  - Code: `examples/rootfind_matched_f4_gate_20261123.rs`. It replays every
    returned root against the integer contract, and every copy must agree.
- Correctness: on every instance all three arms must return the identical
  ordered root set, and that set must equal an independent Python brute force
  over [0,M]^3 (`scripts/panel.py`). Any mismatch voids that instance's ratio and
  is reported.
- Batching: program B requires 5 cases per process, so every arm solves 5
  identical copies of one instance per process. Per-instance time = process
  wall / 5 (monotonic clock around `subprocess.run`). F4 also reports
  in-process solve seconds. These are reported separately and not used for the
  gate.
- Calibration (not scored): 3 calibration charts (seeds 9001–9003, planted)
  on the ladder M ∈ {4, 6, 8, 10, 12, 16, 24, 32, 48, 64, 96, 128}, one F4-ideal
  and one F4-visit process each, with a 120 s budget per solve. Stop climbing an
  arm once it exceeds the budget. **M_hard** = the smallest ladder M at which the
  fastest *completed* F4 arm has median per-instance time ≥ 1 s. If no ladder M
  qualifies, use the largest M at which some F4 arm completes, and record the
  ≥ 1 s condition as unmet. F4-ideal is then run on the panel only if it
  completed at M_hard in calibration. Otherwise it is recorded as timed out (a
  lower bound).
- Instances: 20 at M_hard. Seeds 1–10 are planted (one uniformly random
  triple; coefficients t0,t1,u0,u1 uniform, t2,u2 set so the triple solves). Seeds
  11–20 are unplanted (all six coefficients uniform). The generator is
  `scripts/panel.py gen`.
- Order: 3 repetitions. Repetition r shuffles the instance order with
  seed 100+r. For each instance the process sequence is
  B, F4-visit, [F4-ideal], F4-visit', B'. The primed runs are the A/A duplicates.
  Cold means a fresh process per run (OS page cache not dropped, which is
  recorded).
- Metric per instance and arm: the median over 3 repetitions of the unprimed
  run. Ratio_i = baseline_i / B_i, where baseline = the fastest completed genuine
  F4 arm on that instance.
- A/A spread: per instance and arm, |log(t/t')| from each repetition, reported
  as the median and the 90th percentile across instances, for B and F4-visit.
  Gate condition: the median spread for both is ≤ 10 %.
- Counted units, reported and never cross-divided: F4 `field_ops`
  (F_p multiplications in row reduction plus root extraction), and program B
  enumeration `nodes` and `LLL_steps`.

## Gate scoring rule (fixed now)

Release (2x) passes on the measured conditions only if all of these hold:
geometric mean of Ratio_i ≥ 2; min_i Ratio_i ≥ 1/1.10; fastest baseline per
instance ≥ 1 s; all root sets are equal; the A/A median is ≤ 10 % for both arms;
there are 3 repetitions. Target (8x) is the same with 8. "Qualified isolation"
cannot be met here (L0, a shared 4-vCPU host, three agents), and neither can
"required field/curve coverage" (prime only; no binary, Koblitz or extension
fields). Even a measured pass is therefore reported as
**"measured conditions met, gate not released (isolation/coverage unmet)"**.

## Predictions and falsifiers

- P1: B beats F4-ideal by ≥ 8x geometric mean at M_hard. Falsifier: geometric
  mean < 8.
- P2: F4-visit is the stronger baseline (faster than F4-ideal on ≥ 18/20).
  Falsifier: it is not.
- P3: B beats the fastest F4 arm by ≥ 2x geometric mean with worst slowdown
  ≤ 1.10. Falsifier: geometric mean < 2, or any instance with ratio < 1/1.10.
- P4: all arms return identical root sets on 20/20, equal to brute force.
- Interpretation fixed in advance: a win is a property of program B's
  *Archimedean* chart restriction (x a small integer), which F4 can express only
  through degree-(M+1) domain polynomials or domain branching. It is a component
  result on synthetic charts. It changes nothing about relation yield or the
  counted S of any ECDLP pipeline, and is not a leaderboard or benchmarks row.

## Stopping rule

Stop after calibration plus 3 repetitions of the 20-instance panel. Also stop
early if free disk < 800 MB or total wall > 2.5 h. In that case score what has
completed and label it partial.

## Isolation

L0: a local container shared with two other agents. `RAYON_NUM_THREADS=1` for the
F4 arms, and program B is single-threaded. Every measured number is
[pcc:unverified].

## DEVIATIONS

1. (Before any calibration or panel data; build environment.) `cargo build`
   of the crypto lib fails at base 71a7881ede:
   `src/cryptanalysis/prime_root_trace.rs:1108` has an unclosed `{` in committed
   HEAD, and this task may not edit existing files. The F4 arms are therefore
   built from `f4shim/gen_shim.py`. That crate includes `f4_fp.rs` and
   `f4_fp/*.rs` byte-identical through symlinks, copies its few helpers verbatim
   by line range, and replaces `jv_bitops` (op-count instrumentation) with
   no-ops. Source hashes are in `f4shim/SOURCE_HASHES.json`. `field_ops` is
   counted by f4_fp's own atomic counters and is unaffected.
2. (Found in the M=4 smoke test, before calibration.) `f4_fp::solve` finds
   univariate roots by scanning every element of F_p (gdb: `roots_univariate`
   filters `0..p`), which never finishes at p ≈ 2^32. The F4-ideal arm
   therefore calls `f4_fp::solve_visit` with **no** domains on
   {f1, f2, D(x), D(y), D(z)}. This is the same F4 core with Frobenius root
   extraction, and the domain is imposed only by the D polynomials. The order
   stays Grevlex; `F4_IDEAL_ORDER=lex` is a diagnostic switch, not used in
   the panel. Smoke test M=4, seeds 1 and 11: all three arms agree
   (6 ordered roots, then none).
