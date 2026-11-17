# Protocol: wide-modulus sparse linear algebra, cost per column against modulus width and size

**Status: frozen 2026-11-12, NOT RUN.** No measurement has been taken under
this protocol. Changes after data go in a numbered `PROTOCOL_AMENDMENT_<n>.md`
committed before the data they govern, never edited in here.

Origin: `docs/ic/IC_TECHNIQUE_INTEGRATION_20261012.md` §9 says the generic
modular word in `src/cryptanalysis/koblitz_sparse_la.rs` (`u64` fast path,
`Wide<2|3|4>` = `U128`/`U192`/`U256` for odd moduli up to
`MAX_MODULUS_BITS = 255`) makes ECC2K-130's sparse linear algebra possible,
promises this benchmark pack, and states that the 10⁹-column cost "remains an
extrapolation until the benchmark pack lands". This protocol is that pack's
measurement. It prices one phase (filter + Krylov solve + reconstruction) of
an index-calculus logarithm solve; no ECDLP speedup claim follows from it.

## Questions

1. **Size law.** For the `full-bw` (filter + block Wiedemann 4×4 +
   reconstruct) and `full-bl` (filter + block Lanczos 4 + reconstruct)
   solvers, how does the counted work per column grow with the number of
   columns D, over D = 2¹²…2²⁰? Fit a law with an interval.
2. **Width law.** At fixed D, how does the cost of one counted operation
   (`row_ops`, one multiply-subtract on a matrix entry) grow with the word
   width, `u64` (61-bit control) → `U128` (89, 127 bits) → `U192` (129, 190
   bits) → `U256` (255 bits)?
3. **Extrapolation.** Combining 1 and 2, what does the fitted law predict for
   the ECC2K-130 order (129-bit, `U192`) at D = 10⁹ columns, with what
   interval, and does the law survive its own falsification tests (below)?
   This replaces the §9 extrapolation by a fitted, falsifiable one; it is
   still an extrapolation beyond D = 2²⁰ and is labelled so.

## Instrument

- `examples/la_engine_bench.rs`, `ours --modulus-bits B` (added 2026-11-12;
  see its header). Synthetic relation systems: D columns, rows of
  m = `--weight` = 3 summands at uniform columns with coefficients ±1, drawn
  until every column occurs, then `--excess` = 32 more rows; a planted
  uniform logarithm vector in [1, r) fixes each right-hand side.
  Deterministic per (D, m, seed).
- Primes, fixed per B, primality checked in-program (Miller–Rabin, bases
  2…71; a proof below 3.317·10²⁴, i.e. for B = 61; a strong probable prime
  to 20 bases above):

  | B | prime | word |
  |:--|:--|:--|
  | 61 | 2⁶¹ − 1 | `u64` (control) |
  | 61 | 2⁶¹ − 1, `--word u128` | `U128` (limb-cost control) |
  | 89 | 2⁸⁹ − 1 | `U128` |
  | 127 | 2¹²⁷ − 1 | `U128` |
  | 129 | ECC2K-130 subgroup order 680564733841876926932320129493409985129 (log₂ r ≈ 129.0, bit length 130) | `U192` |
  | 190 | 2¹⁹⁰ − 11 | `U192` |
  | 255 | 2²⁵⁵ − 19 | `U256` |

- **Paired matrices.** For a given (D, seed) every width solves the same
  matrix pattern: the wide generator replays the 61-bit control's column and
  sign stream and draws planted values from a separate stream
  (`seed ^ WIDE_VALUE_STREAM`). The 2026-11-12 devcheck shows equal `rows`
  and `nonzeros` across all B for each (D, seed) (`devcheck.jsonl`). Width
  is therefore the only factor that changes between arms of one cell.
- Every run checks its solution against the planted vector (`verified`). An
  unverified run is reported, never dropped.

## Arms and grid

| factor | levels |
|:--|:--|
| width arm | `u64@61` (control), `U128@61`, `U128@89`, `U128@127`, `U192@129`, `U192@190`, `U256@255` |
| solver | `full-bw`, `full-bl` (block size 4, `--block 4`) |
| D (`--cols`) | 2¹², 2¹³, …, 2²⁰ (9 sizes) |
| seed | 1, 2, 3 |
| m (`--weight`) | 3 |
| coefficients | ±1 (`unit`, primary); `--coeffs wide` at D = 2¹⁶ only, all widths, seeds 1–3 (secondary: does a dense-coefficient row change the counted work?) |
| filter | legacy defaults (no `--filter-strategy`, no `--filter-preset`), as in the devcheck |
| threads | 4 Rayon threads (`RAYON_NUM_THREADS=4`) on a 4-vCPU L2 worker |

Primary grid: 7 × 2 × 9 × 3 = 378 runs; secondary: 7 × 2 × 3 = 42 runs;
A/A: below.

**Order.** For each D ascending, for each seed, the 14 (width, solver) arms
run in a rotated order: arm list as in the table, rotated left by
`(log2 D + seed) mod 14`, so no arm always runs first or last after a size
change.

## Counted units (primary) and wall time (secondary)

Primary, host-independent and deterministic per (arm, D, seed), all read
from the record the harness prints:

- `row_ops` (top level; `report.row_ops`): multiply-subtracts on a matrix
  entry over the whole solve, filter merges, folding, every Krylov product
  (non-zeros × lanes), core and final checks, reconstruction.
- `krylov_products`, `krylov_lanes` (block Wiedemann `products`, `block_n`;
  block Lanczos `sparse_products`, `block_size`), `core_dimension`,
  `core_nonzeros`, `filter_row_ops`, `rows`, `nonzeros`.

`row_ops` counts operations, not limbs: it is (by construction) almost the
same for every width at fixed (D, seed) (devcheck, B = 61…255: max/min
`row_ops` across widths ≤ 1.028 at D = 1024 and ≤ 1.053 at D = 256; the
fold and Krylov draws are random per width, so not exactly equal). It answers question 1. Question 2 needs a
per-operation cost, which only time (or a cycle counter) measures.

Secondary, host-dependent: `solve_s`, `filter_s` and `core_s_derived` on an
Iso Lab EKS L2 worker (native L2 admission, `cpu-l2-spot`). Derived:
`ns_per_row_op = 10⁹ · solve_s / row_ops`, labelled derived. A cycle-counter
pass (`perf-ipc` skill) is optional and only where the worker exposes a
hardware PMU; never estimated otherwise.

## Controls

- **A/A (noise floor).** `full-bw` at `U192@129`, D = 2¹⁶, seed 1, run
  three times per session: before the grid, at its midpoint, after it; the
  same for `u64@61`. Counted units must be bit-identical across the three
  (a difference is a determinism failure and voids the session). The
  max/min ratio of `solve_s` across the three is the session's wall-time
  band; a width-ratio inside that band is reported as "not resolved".
- **Limb control.** `U128@61` against `u64@61`: same modulus, same matrix,
  different word; isolates the word cost from the modulus. `U128@61`
  against `U128@89` and `U128@127`: same word, different modulus; the
  per-operation time should not depend on the modulus within a word
  (a ratio outside the A/A band falsifies "cost depends on L only").
- **Positive control.** Every arm must reproduce the same planted vector;
  a "fast" arm that solves nothing cannot pass.

## Acceptance and stop rules

- A run is valid when it exits 0, prints one JSON record, and
  `verified: true`. Invalid runs stay in the raw data with their reason.
  A cell with an invalid run is repeated once with the same arguments; a
  second failure marks the cell `censored`.
- Per-run timeout 3,600 s; a timeout or an out-of-memory kill ends that
  (width, solver) arm's sweep upward, and is reported with the last size
  reached. Pod memory 16 GiB (estimate at D = 2²⁰, `U256`: about 4.8·10⁶
  rows × 3 entries × 40 bytes ≈ 0.6 GB per copy of the rows, several copies
  live during filter and solve).
- Budget: finite dollar ceiling, CPU/RAM, deadline and retry allowance are
  recorded in the task card before launch (AGENTS.md §9a; ceilings in
  `taskq/deploy/eks-karpenter/COST_CONTROLS.md`). Proposed: 4 vCPU, 16 GiB,
  one worker, USD 25 total including build and failed attempts, 2 retries,
  deadline 7 days from launch. The experiment director confirms or lowers
  these; they are not raised after spending starts.
- The session stops early if any A/A determinism check fails, or if the
  node is not admitted at native L2.

## Analysis (frozen)

1. **Size law, counted.** Per (width, solver), with y = `row_ops / D`:
   fit log₂ y = a + (β − 1) log₂ D by ordinary least squares over the nine
   sizes, the three seeds as separate points. Interval: 95% percentile
   bootstrap over seeds within size (10,000 resamples, generator seed
   20261112). Expectation from the Krylov method: β ∈ [1.9, 2.1]
   (about 2D/b products of the block size b, each costing core non-zeros
   × lanes, with the core proportional to D). Fit the same for
   `core_dimension / D` and `core_nonzeros / D`.
2. **Width law, timed.** At each D ≥ 2¹⁶, ns_per_row_op per width arm,
   median over seeds with min–max. Fit t(L) = c₀ + c₂ L² over
   L ∈ {1, 2, 3, 4} limbs (schoolbook Montgomery: about 2L² + L word
   products per multiply-reduce), and report the ratios t(L)/t(1).
3. **Extrapolation.** Predicted solve time at the ECC2K-130 order and
   D = 10⁹: `row_ops(10⁹) · t(3)`, with `row_ops(10⁹)` from the
   `U192@129` size law. Interval: propagate the bootstrap intervals of
   (a, β) and of t(3). Reported as "extrapolated, about 3 decades beyond
   the largest measured size", in single-host CPU seconds; it does not
   model communication, distribution or memory beyond one host.

### How the extrapolation model is falsified

The law is rejected (and the extrapolation reported as unsupported, with the
data) if any of these holds:

- **Hold-out.** Refit (1) on D = 2¹²…2¹⁹; the observed `row_ops` at D = 2²⁰
  falls outside the 95% prediction interval for any seed.
- **Curvature.** Adding a (log₂ D)² term to (1) over D ≥ 2¹⁴ gives a
  coefficient whose 95% bootstrap interval excludes 0.
- **Width independence of the count.** `row_ops` at fixed (D, seed, solver)
  differs between width arms by more than 10%, which would mean the
  counted unit is not a width-free measure of work.
- **Width law.** At any D ≥ 2¹⁶, ns_per_row_op of two arms with the same
  word (`U128@61/89/127`, or `U192@129/190`) differs beyond the A/A band.
- **Planted check.** Any `verified: false` that survives its repeat in a
  cell used for a fit.

## Exact commands

Build (the image's build step; the tested commit is the one this protocol
is committed in, recorded in the task card):

```sh
CARGO_INCREMENTAL=0 cargo build --release --example la_engine_bench
B=target/release/examples/la_engine_bench
```

One run (template):

```sh
RAYON_NUM_THREADS=4 $B ours --arm <full-bw|full-bl> --cols <D> --weight 3 \
    --seed <s> --modulus-bits <61|89|127|129|190|255> [--word u128] [--coeffs wide]
```

Primary grid (appends one JSON line per run to `raw/grid.jsonl`):

```sh
ARMS=("full-bw 61 u64" "full-bl 61 u64" "full-bw 61 u128" "full-bl 61 u128"
      "full-bw 89 -" "full-bl 89 -" "full-bw 127 -" "full-bl 127 -"
      "full-bw 129 -" "full-bl 129 -" "full-bw 190 -" "full-bl 190 -"
      "full-bw 255 -" "full-bl 255 -")
for k in $(seq 12 20); do D=$((1 << k)); for s in 1 2 3; do
  r=$(( (k + s) % 14 ))
  for i in $(seq 0 13); do
    set -- ${ARMS[$(( (i + r) % 14 ))]}
    W=(); [ "$3" = u128 ] && W=(--word u128)
    RAYON_NUM_THREADS=4 timeout 3600 $B ours --arm "$1" --cols "$D" --weight 3 \
      --seed "$s" --modulus-bits "$2" "${W[@]}" >> raw/grid.jsonl \
      || echo "{\"failed\":true,\"arm\":\"$1\",\"cols\":$D,\"seed\":$s,\"modulus_bits\":$2,\"word\":\"$3\",\"exit\":$?}" >> raw/grid.jsonl
  done
done; done
```

Secondary (`--coeffs wide`, D = 2¹⁶) and A/A use the same template with the
arguments given above, written to `raw/wide_coeffs.jsonl` and `raw/aa.jsonl`.
The per-arm stop rule (timeout/OOM ends the arm's upward sweep) is applied by
the analysis, which ignores a larger D of an arm after its first timeout or
OOM; the launcher may skip those runs to save budget.

## Records, checkpoint and landing

Before launch (AGENTS.md "Resume checkpoints in S3 before every run"):

```sh
scripts/checkpoint.sh init      # fill .resume/RESUME.md: goal, this protocol, grid, commands, stop rules
scripts/checkpoint.sh push
scripts/checkpoint.sh gate      # must pass before any launcher
```

Benchmark record (AGENTS.md "Benchmark records"; `docs/benchmark-records.md`):

```sh
python3 tools/benchrec.py init --slug wide-sparse-la --problem kernel:ic_sparse_la_wide \
    --entry benchmarks/<YYYYMMDD>-wide-sparse-la
python3 tools/benchrec.py run benchmarks/records/<id>.json -- <the isolab submission or grid launcher>
python3 tools/benchrec.py add-raw benchmarks/records/<id>.json \
    research/ic_techniques_20261012/wide_sparse_la/raw/*.jsonl
python3 tools/benchrec.py metric benchmarks/records/<id>.json --name row_ops_per_col_D2^20_u192_129_bw \
    --value <v> --unit count --n 3
python3 tools/benchrec.py metric benchmarks/records/<id>.json --name solve_s_D2^20_u192_129_bw \
    --value <median> --unit wall_s --n 3
python3 tools/benchrec.py close benchmarks/records/<id>.json --status measured \
    --report <report.pdf> --svg <graph.svg> --summary "<one line>"
```

Landing, in the same PR: `raw/` (JSONL, plus S3 by SHA-256), `results.json`
(fits, intervals, falsification outcomes, A/A bands), `README.md` updated
with findings, the research-visuals report and graphs (row_ops/D against D
per width; ns_per_row_op against limbs), the `benchmarks/<date>-wide-sparse-la/`
copies and `benchmarks/README.md` row, and the regenerated
`benchmarks/progress/`. Not a leaderboard row: there is no matched rho for a
phase cost, so no `VALIDATION.json` row is claimed; a phase result goes in
the leaderboard's phase/component section only if it is a speed-up, which
this measurement is not.

## Why this was not run here

- The authoring container (2026-11-12) has AWS credentials in its
  environment. With an AWS CLI installed in the session scratchpad,
  `aws sts get-caller-identity` resolves them to the lab account
  `104186811093` (an IAM user). The remaining checks in
  `taskq/deploy/eks-karpenter/ACCESS.md` (EKS `can-i create jobs`,
  `remote/check-ready`, the CodeBuild launcher) were not run, and nothing
  was submitted: the run spends money under the budget proposed below, so
  it waits for the owner's go-ahead and the experiment director's card.
- The GitHub route, `.github/workflows/isolab-experiment.yml`, takes a
  `spec_path` that must be an `ecbench.spec/v1` JSON file and builds an
  `isolab.eks.request/v1` from it. This harness is a Cargo example, not an
  ecbench method, so that route cannot run it as is.
- A lab run therefore needs either (a) an executor with working Iso Lab
  access that submits this grid as a request-v1 job (image built from the
  tested commit, command as above), or (b) an ecbench method wrapping this
  harness (`ecbench-extend` skill) so the existing workflow can carry it.
- What was run here is a local development check only (`devcheck.jsonl`):
  D = 256 and 1024, m = 3, seeds 1–2, every B, both solvers, plus
  `U128@61` and `--coeffs wide` at 129 bits; all 52 runs `verified: true`.
  Its timings are from a shared container and are not measurements.
