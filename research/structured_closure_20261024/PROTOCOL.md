# Pre-registration: a structured closure solver for the blocked degree cells

Registered 2026-10-24 by the commit that adds this file, before the engine
exists and before any cell below has run on it. Owner request 2026-10-24:
"preregister the structured Semaev solver".

Every statement carries one label: *recorded* (a committed row), *derived*
(a calculation shown here) or *predicted* (stated before the runs).

## 0. Why this solver and not the symmetrised one

The first candidate for "structured Semaev solver" was the symmetrised `S₄`
(elementary symmetric functions, quadratic after descent, `e₃` enumeration
plus linear solves). It is already built (`src/cryptanalysis/koblitz_sym_s4.rs`)
and already measured, negatively:

- end to end, 987×–769,221× strong rho at n = 17, 23 and 31, where the pair
  table on the same curves is 9×–131× (*recorded*,
  `research/koblitz_symmetrised_e2e_20260927/RESULTS.md`);
- `decompose_by_e3` against the pair table, 553×–34,901× worse, a sealed
  negative (*recorded*, `research/pdp_shape_replay_20261015/EXPERIMENTS.md`
  L3c).

The lead atlas (crypto-autoresearcher `knowledge/leads/ecdlp/`, reviewed
2026-10-24) closes faster Boolean F4/F5 engines as dominated by the pair
table (`LEAD-ICE-boolean-f4-f5-f6-engines`, `closed_dominated`). The one
solver-side question it holds open that could move the exponent is
`LEAD-ICE-descent-closure-degree-at-scale` (`open_blocked`): does the MutantXL
closure degree of the descended chain stay at 4 as ℓ grows? It is blocked on
memory, and its `next_action` names "a host with ≥ 250 GB RAM (or a sparse
closure)". This protocol registers the second option. The symmetrised solver
is re-tested separately, as a replication of a negative, in
[`../sym_s4_closure_base_20261024/PROTOCOL.md`](../sym_s4_closure_base_20261024/PROTOCOL.md).

Related, not duplicated: `LEAD-ICE-syzygy-cascade-signatures` counts F5
signatures to ask whether the degree drops. This study uses syzygies only to
skip rows that provably reduce to zero, so that the closure fits in memory.
It counts nothing about signatures.

## 1. What blocks the lead (*recorded*)

- Degree 4 refutes through (m, ℓ) = (4, 6) at N = 54: A1 draws 42 and 214
  refute at degree 4, the satisfiable control draw 0 does not (closure rank
  214,391 of 342,537 columns), at 25.8 GB resident
  (`research/dreg_closure_m4l6_20261018/RESULTS.md`).
- At (m, n, ℓ) = (3, 19, 7) none of the 4 committed unsatisfiable draws
  (0, 3, 5, 7) refutes at degree 4, and the same holds at (3, 21, 7), draws
  0, 2, 3, 6 (`research/dreg_closure_scale_20261016/draws/`). Degree 5 there has
  C(40, ≤5) = 760,099 columns; the dense closure exhausted 169 GB of a
  184 GB cap after 17.5 min (C1, capped, inconclusive).
- The existing sparse engine, `sparse_closure_refutes`
  (`src/cryptanalysis/koblitz_bench.rs`), agrees with the dense closure on
  36 comparisons but did not finish a degree-4 cell in 30 min where the
  dense one took 37 s. After reduction the falls are dense across the
  ~10⁵ degree-≤4 columns (`research/dreg_closure_driver_20261016/RESULTS.md`).
- Fixing one variable brings 6 of those 8 draws back to degree 4, and both
  branches of one guess refute on 5 of 8 (2 of 4 at n = 19, 3 of 4 at n = 21)
  (`research/dreg_closure_specialise_20261017/RESULTS.md`).
- The descended chain is multilinear in its blocks (one block of ℓ per
  summand, one of n per intermediate point), and the test
  `the_decomposition_systems_are_multilinear_in_their_blocks` pins that. A uniform
  per-block degree bound on the shifts is a measured negative (0.21× at
  n = 15). The note names the Faugère–Safey El Din–Spaenlehauer use of the
  forced Jacobian syzygies as "a different algorithm … unbuilt"
  (`research/notes/ecc2k130/RESEARCH_KOBLITZ_INDEX_CALCULUS.md`, multilinear
  section).

## 2. The engine to build (`structured_closure`)

It is native Rust, in `src/cryptanalysis/structured_closure.rs` with a driver
`examples/structured_closure.rs` that reads the same draw files and emits the
same per-degree JSON fields as `examples/dreg_closure.rs`. It computes the
**same object** as `ReducedBasis::from_system_closed_full`: the degree-D
Macaulay row space saturated under MutantXL's fixed-degree step. There are
three parts:

- **S1, row pruning, sound by construction.** These Macaulay rows are skipped
  before elimination:
  - the Boolean trivial syzygies (`f·f = f`, and `f_i·f_j` taken once);
  - rows that the block-multilinear Jacobian syzygies predict reduce to
    zero. A row is skipped **only** when a certificate shows it lies in the
    span of rows already inserted. The certificate is the syzygy vector,
    whose product with the generators is checked to vanish in the Boolean
    ring. An uncertified prediction never skips a row.

  So S1 changes time and row count, never the row space.
- **S2, sparse-dense hybrid basis under a hard cap.** Rows are kept sparse
  while their fill is below a threshold. Past it they move to a bit-packed
  dense core reduced M4RI-style. Peak resident memory is capped by
  `--max-rss`. On reaching the cap the run returns *capped*, which is
  unmeasured and never a refutation or a non-refutation.
- **S3, k-variable hybrid.** `--guess k` fixes the first k variables of a
  stated order, in all 2^k assignments. It runs S1 and S2 at degree D on each
  branch. The system is refuted at (k, D) when every branch refutes. That is
  sound, because a common solution survives in exactly one branch. The order
  is fixed now: intermediate-point coordinates first, then summand
  coordinates, lowest index first. The specialise study found the
  intermediate fix as effective as any other.

Correctness tests that land with the engine, before any cell runs:

- agreement with the dense closure and with `sparse_closure_refutes` on the
  36 comparisons of `sparse_closure_matches_the_dense_closure`;
- every S1 skip re-verified in debug builds;
- S3 with k = 1 reproduces the 32 committed specialise records' verdicts
  (A0/A1/C arms are one-variable fixes; B is a three-variable fix), on the
  same draws and fixes.

## 3. Gates (all must pass before §4 is read)

| gate | what | pass |
|---|---|---|
| G1 agreement | A1 (m = 4, n = 15, ℓ = 6) draws 42, 214 and control draw 0, degree 4, from `research/dreg_closure_m4l6_20261018/draws/m4-15-6.jsonl` | both draws refute, the control does not, and the control's degree-4 `closure_rank` equals the committed 214,391 of 342,537 columns (`runs/control-m4-15-6.d0.jsonl`), and its degree-3 rank equals the committed 2,748 |
| G2 resource | the same three runs | peak RSS ≤ 12 GB each (the dense closure used 25.8 GB) |
| G3 soundness | every satisfiable control run in §4 | never refuted |

If G1 fails, the engine is wrong and nothing else is reported. If G2 fails,
§4's S2 arm is not run on this host (a 15 GB container, 4 vCPU, recorded at
run time) and only the S3 arm is run.

## 4. Cells and arms

The draws are the committed files of the predecessor studies, untouched.

| id | m | (n, ℓ) | N | arm | degree | draws |
|---|--:|---|--:|---|--:|---|
| C1-5 | 3 | (19, 7) | 40 | S1+S2 | 5 | `research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl`: unsatisfiable draws 0, 3, 5, 7; satisfiable draw 1 as control |
| C1-H | 3 | (19, 7) | 40 | S3, k = 1, 2, 3 (stop at the first k where all branches refute) | 4 | as C1-5 |
| C2-H | 3 | (21, 7) | 42 | S3, k = 1, 2, 3 | 4 | `research/dreg_closure_scale_20261016/draws/m3-21-7.jsonl`: unsatisfiable draws 0, 2, 3, 6; satisfiable draw 1 as control |
| A1-H | 4 | (15, 6) | 54 | S3, k = 1 | 3 and 4 | the A1 draws (does a guess lower degree 4 to 3?) |
| B1-H | 4 | (17, 7) | 62 | S3, k = 1, 2, 3 | 4 | fresh: `examples/dreg_ladder --cells 17:7:3 --m 4 --unsat 4 --ffd-max 3 --seed 20261018 --max-draws 4096`, committed before any closure run on them, with one satisfiable control |

Each process runs under `prlimit --as` at 92 % of the cgroup limit and
`timeout 14400`. A kill for memory or time is recorded as capped with the
degrees and branches it finished.

## 5. Questions and decision rule, fixed now

- **Q1 (instrument).** Does the structured engine compute the dense closure
  exactly, in at most half its memory? Read by G1 and G2.
- **Q2 (predecessor's Q3).** On C1-5, does the closure refute at degree 5 on
  every unsatisfiable draw?
  - All refute: the degree at ℓ = 7, m = 3 is exactly 5, and the "degree 5
    held" row of `RESEARCH_ECC2K130_DESCENT_DEGREE.md` keeps measured support.
  - None refutes, closure saturated: the degree is ≥ 6, and that row loses
    support.
  - Mixed, or capped: inconclusive, with the draws named.
- **Q3 (hybrid degree).** For each H cell, what is the smallest k such that
  every branch refutes at degree 4 on every unsatisfiable draw? The reading is
  the cell's **hybrid degree (k, 4)** and its matrix size
  2^k · C(N − k, ≤ 4) against C(N, ≤ 5) (*derived*, recorded per cell). If no
  k ≤ 3 works, the hybrid degree at that cell is above (3, 4), which is
  recorded as such.
- **Q4 (does the hybrid hold at m = 4).** If B1-H's hybrid degree is (k, 4)
  with k ≤ 2, the ℓ = 7 rise is a guess's worth at m = 4 as at m = 3. If
  k ≥ 3 or none, it is not.

No cell is a cost row, an improvement or a claim about ECC2K-130 or about rho.
AGENTS.md §5a pricing does not apply, because no cost is compared. Any
consequence for the ECC2K-130 cost rows is written as a *derived* update to
`research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` in the result
PR, and the atlas lead gets the evidence row.

## 6. Predictions (before the runs)

1. G1 passes (confidence 0.8). The closure is a well-defined row space, and
   two implementations already agree on it.
2. G2 passes (0.5). The control's dense basis alone is about
   214,391 × 342,537 / 8 bytes ≈ 9.2 GB (*derived*), so 12 GB leaves little
   room for the sparse part.
3. C1-5 is **capped** on this host (0.8). A degree-5 basis of rank r on
   760,099 columns needs r × 95 KB dense (*derived*; r ≈ 600k gives
   ≈ 57 GB). Pruning removes rows, but not basis rank. The predecessor's
   prediction for the same question (refutes at 5, 0.7) still stands for a
   large host.
4. C1-H: hybrid degree (1, 4) on at least 2 of 4 draws, and (2, 4) on all 4
   (0.5). In the specialise study both values of one variable refuted on
   draws 3 and 7, and draw 5 resisted every single fix.
5. C2-H: (1, 4) on at least 3 of 4 draws (0.5). The specialise study had
   3 of 4 under both values of variable 6.
6. A1-H: k = 1 does not lower degree 4 to 3 (0.7).
7. B1-H: hybrid degree (k, 4) with k ≤ 2 (0.5). Generation finds at least 4
   unsatisfiable draws in 4096 (0.4); the A1 rate was 2 in 251 at S = −9, and
   B1 is at S = −11.

## 7. What was run before this protocol

Nothing at any cell on this engine, because it does not exist yet. Read:

- the four `dreg_closure_*` studies;
- `research/dreg_closure_specialise_20261017`;
- `examples/dreg_closure.rs`, and `sparse_closure_refutes` with its tests;
- the multilinear section of `RESEARCH_KOBLITZ_INDEX_CALCULUS.md`;
- the atlas leads named in §0.

## 8. Evidence and reporting

The run directory holds the run artifacts:

- `runs/*.jsonl` (one line per draw, degree and branch), `runs/*.log`
  (`/usr/bin/time -v`);
- `runs/host.json` (commit, binary sha256, rustc, cgroup limit, lscpu);
- `RESULTS.md` read against §5.

Following the `research-visuals` skill, there is a report, a graph (verdict
grid per cell and k; matrix size against k) and a PDF for every run. The
benchmarks/leaderboard rules apply only if a cost row ever comes of this,
and none is planned.
