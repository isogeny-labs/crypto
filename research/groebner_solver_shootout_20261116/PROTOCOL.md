# Gröbner / algebraic solver shootout on binary Semaev systems: protocol v1

Protocol id `GB-SOLVER-SHOOTOUT-20261116`. The machine-readable record is
`protocol.json` (`experiment.protocol/v1`). Its `freeze` block holds the
canonical SHA-256
(`116b33f754becbd9d7e76cbd80bfff8e323cb79ab0ee0c57481c0616d3d9bfee`, frozen
2026-11-16T16:55:09Z with the vendored checker
`research/primefield-companion-followup-20261102/tools/experiment_protocol.py`,
sha256 0f46a2b8…84e4, because the canonical crypto-autoresearcher checkout is
absent here). The `freeze.commit` field is the branch HEAD at freeze
(66d8635ef2), which does NOT yet contain this study or the harness files; the
commit that first adds them is the source revision. Status: **frozen, not launched**. No main-run data exists.
Nothing in this file is a result. The preflight figures below are
single-instance development timings on a shared container, and they are
labelled as such.

## Question

The systems are Weil-descended Semaev summation-polynomial decomposition
systems over GF(2^n) on the Koblitz curves y² + xy = x³ + a·x² + 1. They come
in four variants: m = 3 plain chained-S3, m = 3 symmetrised S4 (x and e
variables), m = 3 symmetrised e-only, and m = 4 plain chained-S3. Every arm
gets the same seeded targets and the same budget. The study asks how the
following arms compare:

- msolve F4,
- CryptoMiniSat with native XOR,
- a fixed-schedule F4 replay,
- Magma F4, if a licensed binary is reachable.

The comparison covers re-added (certified) relations per CPU-second, observed F4 step
degree, largest matrix, wall time and peak RSS. For each arm, the study also
reports how far its cost per relation is from matched Pollard rho across the
§5a band.

## Claims and their classes

| id | class | what decides it |
|:--|:--|:--|
| C-certified | exact computation | Every counted relation passes group re-addition and appears in the oracle's decomposition set |
| C-telemetry | exact computation | Per-row degree, matrix, CPU, wall and RSS, as the tool reported them in this run |
| C-solver-order | comparative | C-msolve-cms, C-msolve-repo, C-cms-repo, C-trace-warm, C-trace-msolve, C-magma-msolve |
| C-vs-rho | comparative | C-rho-msolve, C-rho-cms, C-rho-trace (absolute, band + clock) |
| C-dreg | heuristic estimate | Observed degree read against the semi-regular yardstick; descriptive only |
| C-out | out of scope | See "What this does not show" |

## Instance families (grid.tsv, sha256 666b262e…db3e)

All cells use b = 1. The factor base is V = ⟨1, z, …, z^{l−1}⟩ in the
polynomial basis of `find_irreducible_sparse(n)` order. l is close to n/m.
Within each cell, a is chosen so that the factor base has at least m + 2
liftable abscissae, and so that the estimated rate of decomposition sums per
group element lies between 0.18 and 2.7. Those fields are exact computations
made before any data, recorded in `grid.tsv`.

| m | variant | (n, l, a) | vars |
|:-:|:--|:--|:--|
| 3 | plain (3l + n vars, cubic) | (11,4,0) (13,5,0) (15,5,1) (17,6,0) | 23, 28, 30, 35 |
| 3 | symmetrised (x + e, 9l − 3) | same | 33, 42, 42, 51 |
| 3 | symmetrised-e (6l − 3, quadratic) | same | 21, 27, 27, 33 |
| 4 | plain (4l + 2n, cubic) | (9,3,1) (11,3,0) (13,4,0) | 30, 34, 42 |

The repository has no binary symmetrised S5. The generator raises an error
for that case, so m = 4 is chained-S3 only. This is a stated gap; no S5 was
improvised.

The largest prime factor r of the group order varies non-monotonically with
n: 23, 2003, 211, 239, 37. The study therefore makes no claim about how
results scale with n (lesson L-12d575623c2f).

Each cell contains these populations. All of them are identical bytes for
every arm, and their seeds come from the protocol hash. The derivation is in
`protocol.json` `randomization.seed_derivation`.

- **40 natural targets.** Each target R is a uniformly random point with
  x_R ∉ V, x_R ≠ 0 and R ≠ O. This is the population that relation collection
  actually meets. These targets give the primary endpoint.
- **5 planted targets (positive control).** Each is the sum of m factor-base
  points with distinct abscissae.
- **Oracle-empty natural targets (negative control).** These are the natural
  targets for which the fb-enum oracle finds no decomposition. Each arm-cell
  also gets 2 tampered witnesses, which the verifier must reject.
- **1 warm-up instance per arm per Job.** It is discarded.

## Arms

| arm | role | instrument | availability at freeze |
|:--|:--|:--|:--|
| msolve-f4 | candidate | msolve 0.6.5-1build2, sha256 c2722288…59fa, `-g 2 -v 2 -t 1` plus in-house root extraction | available |
| msolve-f4-aa | A/A | same call, fresh process, other position | available |
| cms-xor | candidate | cryptominisat5 5.11.15, sha256 00db7e59…7207, `--threads 1 --maxsol 1`, seed 0 | available |
| cms-xor-aa | A/A | same, solver `--random` seed derived per target | available |
| trace-f4 | candidate | Groebner.jl learn/apply (worker.jl), learn cost amortised per cell | **UNAVAILABLE**: no Julia (proxy 403), and supports differ per target |
| julia-f4-warm | baseline for trace-f4 | Groebner.jl ordinary solve | **UNAVAILABLE**: no Julia |
| magma-f4 | candidate | licensed Magma only | **UNAVAILABLE**: no binary or licence |
| repo-boolean-f4 | baseline | groebner_compare BooleanF4 prototype | available (30 s watchdog hit in the smoke run) |
| fb-enum-oracle | reference, not ranked | complete factor-base multiset × sign enumeration, counted group additions | prerequisite P2 |
| rho-matched | reference | expected signed-Frobenius steps on r, plus measured repository rho, 40 DLPs per curve | prerequisite P5 |

msolve's own GF(2) solving output (sparse FGLM or rational parametrisation)
is never used. On toy GF(2) systems with more than one solution, the scouts
saw it return degenerate or wrong output. Roots come only from the reduced
basis.

An arm marked UNAVAILABLE produces rows with status `unavailable` and a
reason. It is never filled with another tool or with published numbers.

Harness files at freeze are pinned by SHA-256 in
`execution.placement.harness_files_sha256_at_freeze`. They are
`groebner_compare/semaev.py`, `external_worker.py`, `semaev_verifier.py`,
`runner.py` and `certificate.py`. They were uncommitted when this protocol
was frozen. The commit that first contains them and this file is the source
revision. A change to them before launch is recorded as an amendment.

## Metrics

**Primary.**

1. **`rel_per_cpu_s`.** The number of natural targets for which the arm
   returned at least one re-added decomposition, divided by the charged CPU
   spent on those targets.
   - Charged CPU is the solver child's user + sys time, plus msolve root
     extraction, plus the trace-f4 learn cost.
   - A censored call is charged at the 300 s cap.
   - Verification CPU is excluded for every arm and reported separately.
2. **`cost_per_relation_vs_rho`.** Two ratios are reported:
   - the cost per relation in GAE divided by rho's cost,
   - the projected relation phase, (fb_abscissae + 10) relations with linear
     algebra uncharged (so a lower bound), divided by rho's cost.

   GAE here means CPU time divided by t_GA, the same-host CPU time of one
   native Koblitz point addition. Rho's cost is computed at four readings:
   - E_steps(r, n)·(1 + w·U_rho) for w = 0, 0.1 and 1, where E_steps =
     √(πr/2)/√(2n),
   - the measured rho CPU time per DLP on the same host.

   The result is always reported as a log2 distance remaining, never as
   pass or fail.

**Secondary.** Charged CPU per target (the input to the paired test), the
maximum F4 step degree, the largest matrix in rows × cols, solver wall time,
solver peak RSS, recall against the oracle, and planted recovery.

Peak RSS is the sampled `/proc` VmHWM of the solver process. ru_maxrss is not
used, because the forking worker's RSS leaks into it; the build smoke
observed that fault [pcc:unverified].

## Order, budgets, controls

- **Order.** Solver arms are interleaved per target in a Williams-balanced
  order. The rotation is derived from the protocol hash. Every call runs in a
  fresh process, single-threaded, one call at a time per Job.
- **Budget.** Every arm gets 300 CPU-seconds and 8 GiB per call.
- **Positive control first.** The 5 planted targets of each cell run before
  the natural targets.
  - An available arm must return a re-added decomposition on 5 of 5 planted
    targets.
  - A complete arm must also return the planted decomposition itself.
  - A miss that is not censored makes the arm-cell invalid.
  - If 3 planted calls of an arm-cell are censored, that arm-cell stops as
    "censored at cap" and its natural targets are not run.
- **Negative control.** An arm-cell is invalid if any of these happens:
  - a relation on an oracle-empty target,
  - an accepted tampered witness,
  - a complete arm's root set that differs from the oracle's.
- **A/A.** C-msolve-aa measures clock and RSS noise. msolve is deterministic
  at `-t 1`. C-cms-aa measures the spread caused by the CDCL seed. The noise
  floor is the larger |bound| of the two A/A 95% bootstrap intervals in that
  cell.

## Decision rule

Comparisons between solvers work as follows:

- The test is an exact two-sided Wilcoxon signed-rank test on paired log2
  charged CPU over the natural targets.
- The family is 315 tests: 21 label pairs × 15 cells, with a Bonferroni
  threshold of 0.05/315 = 1.59e-4.
- "A uses less CPU per target than B" requires p below that threshold and a
  |median log2 ratio| above the cell's A/A floor.
- If fewer than 14 pairs are decided (n_eff < 14), the threshold cannot be
  reached, and the comparison is reported as "unresolved".

Against rho, an arm is "below" or "above" matched rho only when the
projected ratio is on the same side of 1 at all four readings. Otherwise it
is "unresolved in band". If rho does not emit a count for one of its
unpriced counters, the rho side becomes a lower bound and the pair is not
ranked.

The full vocabulary is in `protocol.json` `decision_rule`.

## Isolation tier

The default route is isolab-eks. These solvers can only run there as command
Jobs (`isolab.eks.command-request/v1`), and command Jobs have no L2, replay
or A/A admission (taskq/deploy/eks-karpenter/ACCESS.md). Every clock-based
verdict is therefore labelled **diagnostic tier**. No row goes to the IC
leaderboard or to `benchmarks/README.md` without `VALIDATION.json`. An
admitted path, such as adding the solvers as an ecbench method, would be a
new protocol version.

Local container runs are development checks only.

## Stopping and budget

The design is fixed-N: 15 cells × (1 warm-up + 5 planted + 40 natural) × the
available solver arms, plus tamper checks, rho and t_GA. Limits:

- a study ceiling of USD 15,
- at most 24 vCPU running at once,
- 2 vCPU per Job,
- at most 3000 s per Job, with one Job per cell, split by target range where
  needed,
- at most 2 infrastructure retries per Job, each rerunning the whole shard
  with the same seeds; rows from a preempted Job are excluded under the rule
  "preempted".

Running this protocol needs owner authorization. Writing it is not
authorization to run.

## Prerequisites (must exist and be hash-pinned before launch)

None of these may change the design. Any that does needs an amendment.

- **P1. Natural-target mode in `semaev.py`.** It must implement the seed rule
  exactly as frozen.
- **P2. fb-enum oracle.** It enumerates every m-multiset and sign pattern,
  counts group additions, and returns the exact decomposition set.
- **P3. `rows.jsonl` emitter.** It must use the schema in `analyze.py`.
  - extract_cpu_s and verify_cpu_s must be separate fields.
  - It parses degree and matrix size from the msolve log.
  - It passes CMS `--random` and `--maxsol 1` through to the solver.
- **P4. Tamper check.**
- **P5. Calibration and rho reference.** Same-host t_GA calibration, and the
  matched rho reference with every unpriced counter emitted, written to
  `cells.json`.
- **P6. Solver image.** An image in `isolab/tasks` with msolve and
  cryptominisat5 at the pinned SHA-256s, recorded by digest, with the
  check-experiment-git receipt.
- **P7. Run gates.** Pushed source, `scripts/checkpoint.sh gate`, an open
  `bench.record/v1`, an owner-approved budget, and the director's task card.
- **P8. Trace arm (gating only).** Julia and Groebner.jl pinned, plus a
  same-support template encoding. Without P8, trace-f4 and julia-f4-warm stay
  UNAVAILABLE.

If any of P1–P7 is missing, the study does not launch.

## Preflight (development timing, not data)

The preflight ran on 2026-11-16 in the local container: Intel Xeon @ 2.10
GHz, 4 vCPU, shared, with 3 calls in parallel. It used one planted instance
per cell (seed 1, index 0), which is **not** a frozen seed, and a 120 s kill
cap. Raw output is in `preflight/timings.tsv`, produced by `preflight/gen.py`
and `preflight/run.sh`.

- **msolve -g 2.** It finished in 0.58–44.2 s user time on 8 cells, with
  maximum step degree 4–6 and largest matrix up to 57970×58975. It was killed
  at 120 s on 7 cells: symmetrised n ≥ 13, symmetrised-e n ≥ 13, and m = 4
  n = 13.
- **CMS --maxsol 1.** It returned SAT in at most 12.5 s on all 15 cells.

These observations shaped the 300 s cap and the rule for censored planted
targets. Disclosed: the preflight shows CMS deciding planted instances faster
than msolve computes a full basis on every finished cell [pcc:unverified]. That is in-sample
for the planted population only. It is not a prediction and it carries no
criterion.

Runtime estimate: about 410 CPU-s per natural-target index across all cells
and arms, × 40 = 16.4k s. Fixed costs are about 37.5k s: the planted phase,
probes of censored cells, repo-boolean-f4 probes, rho, t_GA and generation.
The total is about 54k CPU-s, roughly 15 CPU-hours. The actual figure is to
be recorded next to this estimate.

The dry run of `analyze.py` (sha256 0b0eee39…7929) on synthetic scratch rows
ran both phases in 0.8 s.

## What this does not show

The study does not establish any of the following:

- any gain over the current index-calculus baseline, because the
  in-repository binary IC baseline is not an arm;
- a leaderboard row;
- a scaling law or exponent;
- anything at cryptographic sizes;
- any Magma timing;
- any statement about msolve's GF(2) FGLM beyond "not used".

Prior results it builds on are all `[pcc:unverified]` or carry no pcc id
(`protocol.json` `literature`). research-kb and the lead atlas were not
available in this container, so the protocol makes no novelty claim.
