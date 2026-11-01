# K conversion: frozen protocol (2026-10-18)

Frozen before any Callgrind run. Requested by the owner after
`research/ic_leads_review_20261018/README.md` (section 1 item 4 and
section 3.2, "Measure K"). Classification of the claim: a **measurement**
(a ratio of two instruction counts), plus a **derived conversion** of two
priced rows. It is not a new algorithm, not a degree result, and not a
beat-rho claim.

## 1. What is converted

`research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` prices the
ECC2K-130 m = 4 rows in *monomial-column touches*: the oracle cost of one
target is `C(N, <=D)^w` with `w = 1`, i.e. one touch per column of the
degree-`D` Macaulay matrix, and rho in *group operations* (`2^60.81`). The
note leaves `K` "not measured". This study measures it.

Direction. The note's `K` is touches per group operation. The owner's
formula below is the reciprocal: rho steps per touch. This protocol uses the
owner's, called `K` throughout:

    K = (instructions per column touch) / (instructions per rho step)

so `cost in rho-step units = cost in touches x K` and
`log2(converted) = log2(cost) + log2(K)`. `K > 1` raises the rows,
`K < 1` lowers them. The note's `K_note = 1/K`.

## 2. The two units

### 2.1 Column touch (numerator)

There is no touch counter in the closure engine and none is added. The
engine's own counters are `ClosureDegree.columns` (`closed.columns()`,
which is `C(N, <=D)` for the built degree-`D` matrix),
`closure_products` and `closure_word_ops`
(`examples/dreg_closure.rs`, `closure_refutation_scan` in
`src/cryptanalysis/koblitz_bench.rs`). The note's oracle unit *is* the
column count, so:

- **one column touch = one column of the degree-`D` Macaulay matrix**,
  taken from the engine's `columns` field;
- **instructions in the oracle** = Callgrind `Ir` inside
  `ReducedBasis::from_system_closed_full(system, N, D, 1_000_000)` (the
  dense full-MutantXL closure, matrix build included), collected with
  `--toggle-collect`. The plain XL basis that `closure_refutation_scan`
  also builds, parsing, and the committed-draw replay are outside the
  interval;
- `I_col = Ir_oracle / columns`.

`closure_word_ops` is reported alongside as `Ir_oracle / closure_word_ops`
(instructions per engine word-XOR), as a cross-check only. It does not enter K.

`I_col` is not size independent: a dense elimination does more work per
column as the matrix grows. The note prices `w = 1`, which assumes it does
not. K is therefore measured at the sizes that fit, its growth with
`columns` is reported, and any converted row says so.

### 2.2 Rho step (denominator)

One walk step of `rho.signed_frobenius_strong`
(`src/cryptanalysis/koblitz_strong_rho.rs`, `StrongRhoG::solve`):
`I_step = Ir inside solve / walk_steps`, where `walk_steps` is the
solver's own outcome counter. Everything inside `solve` (canonicalization,
partition hash, distinguished-point table, walk starts, collision check) is
amortised into the step, as the ecbench profile of
`research/ecbench_rho_optimized_20261014` does (1,597,499,923 Ir over
1,591,865 steps = 1003.5 Ir/step; recorded there with a different binary
and compiler, so it is a cross-check only and is never mixed into K).

Fields. The binary supports a packed `u64` field (n <= 63) and a `u128`
field (n <= 127). **n = 131 (ECC2K-130) is not supported** (the group order
exceeds `u128`), so the ECC2K-130 field cannot be measured. Two arms:

- **n = 83 (primary denominator)**, the widest field available:
  `examples/k_conversion_rho.rs n83`, a walk capped at 10^6 steps
  (`step_cap_factor = 0` leaves the floor of 10^6), so `walk_steps` is 10^6
  to within one 32-lane batch (reported as 10^6; error under 0.004%).
- **n = 61 (secondary)**, the ecbench fixture field: complete solves,
  `walk_steps` from the outcome, answer verified against the planted scalar.

## 3. Cells (committed draws, fit in 15 GB)

All draws are unsatisfiable, committed, and replayed from the committed
file with `examples/dreg_closure.rs --max-cols 70000 --to 3 --draw K`
(degree 3 only: the degree-4 dense closure at N >= 34 costs 90 s or more
native, 40 to 100 times that under Callgrind). Cell labels follow the draw
files: `m<m>-<n>-<ell>`.

| Cell | m | n | ell | N | Source | Draws |
|---|--:|--:|--:|--:|---|---|
| A: m4-11-5 | 4 | 11 | 5 | 42 | `research/dreg_closure_driver_20261016/draws/m4-11-5.jsonl` | 8, 67, 107, 143 |
| B: m4-12-3 | 4 | 12 | 3 | 36 | `research/dreg_closure_scale_20261016/draws/m4-12-3.jsonl` | 1, 2, 3, 4 |
| B': m4-12-5 | 4 | 12 | 5 | 44 | `research/dreg_closure_driver_20261016/draws/m4-12-5.jsonl` | 24, 26, 47, 52 |
| C: m3-16-6 | 3 | 16 | 6 | 34 | `research/dreg_closure_scale_20261016/draws/m3-16-6.jsonl` | 0, 1, 2, 3 |

"(11,4)" and "(12,4)" in the request are read as n = 11 and n = 12 at
m = 4; the draws exist at ell = 5 (n = 11), and ell = 3 and 5 (n = 12),
and all three are run. "(16,6)" is m = 3, n = 16, ell = 6.

Growth cells (degree 4, for the I_col-versus-columns trend only, not for the
conversion): m = 3, n = 11, ell = 5, N = 26, committed draws 56 and 10 from
`research/dreg_surplus_control_20260925/runs/cell-11-5-6.u3.jsonl` and
`.u0.jsonl` (replayed with `--to 4`). If a degree-4 run exceeds the time
budget it is dropped and reported as not run.

## 4. Measurement

- Host: this container (no hardware PMU, so `perf-ipc` refuses by
  design); `uname`, `lscpu`, rustc, valgrind version and the binary SHA-256
  are recorded in `PROVENANCE.txt` before and after. Isolation tier: none
  (shared container, other jobs running); Callgrind counts simulated
  user-space instructions, which do not depend on contention, so the tier
  limits wall-clock claims only, and there are none.
- Unit: `callgrind.Ir`, `valgrind --tool=callgrind --cache-sim=no
  --branch-sim=no --dump-instr=no --toggle-collect=<symbol>`. Oracle symbol
  `*ReducedBasis*from_system_closed_full*`; rho symbol `*k_rho_solve*`. The
  total process `Ir` is recorded as well, so the collected share is visible.
- Build: `cargo build --release --example dreg_closure --example
  k_conversion_rho`, default release profile, one binary for all runs.
- Repeats: Callgrind is deterministic, so the spread is **across draws**
  (closure: 4 draws per cell) and **across seeds** (rho: seeds 1, 2, 3 at
  each field). A/A control: draw 0 of cell C and seed 1 of rho n = 61 are
  each run twice; the two `Ir` must agree to within 0.01% (ASLR and
  allocator noise only).
- Positive control: across all closure runs, `Ir_oracle` must rise with
  `closure_word_ops` (Spearman rank correlation reported). If the
  collected interval does not track the engine's own work counter, the
  toggle is wrong and the cell is not reported.
- Null control: the collected share of total `Ir` must be strictly between 0
  and 1, and the skipped part (parse, plain XL basis) is reported.
- Reported per cell: `columns`, `Ir_oracle`, `I_col`, `K_draw = I_col /
  I_step`, and the cell's median with min and max over draws. Spread is the
  range, since there are 4 values per cell, not an interval.

## 5. Conversion of the two rows

Rows, from the descent-degree note: `2^68.43` (m = 4, degree 4, w = 1,
ell = 33, +7.62 over rho `2^60.81`) and `2^71.13` (m = 4, degree 5, w = 1,
ell = 34, +10.32).

- Primary K: the median over the m = 4 degree-3 draws (cells A, B, B'
  pooled), over the n = 83 denominator. Band: the smallest and largest cell
  median, and the n = 61 denominator as the second end.
- Converted row = `old row + log2 K`; converted gap to rho =
  `old gap + log2 K` (rho stays `2^60.81`, a group-operation count at
  n = 131; its own per-step cost at n = 131 is unmeasured, and the n = 83
  denominator is a lower bound on it, so K is an upper bound for that
  reason).
- The conversion is **derived**: it is arithmetic on two measured ratios
  and on rows the note priced. The rows themselves, the degree question
  (does the closure degree stay at 4, or 5, up to N near 394) and the
  extrapolation from N of 34 to 44 to N = 394 are **unchanged**. K measured
  at small `columns` understates I_col at `columns` near 2^49; the converted
  rows are therefore lower bounds on the oracle side.
- Classification: accounting. It changes the unit of a priced row; it
  does not lower any cost, and is not an improvement.

## 6. What this does not say

No claim about beating rho; no claim about n = 131 rho step cost; no
wall-clock claim; no claim that `w = 1` is attainable; no cross-experiment
mixing (all numbers in the K ratio come from one binary in this study).
