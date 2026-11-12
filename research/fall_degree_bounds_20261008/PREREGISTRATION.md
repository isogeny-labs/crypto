# Preregistration: proving or refuting the fall-degree bounds for `m = 2` index calculus

**Written:** 2026-10-08, while the E1, E1b and E2 runs were in flight and
before any of their output was read. The bounds being tested are ledgered
in `research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md`.

**What was seen beforehand, disclosed:**

- The `lfd.sage` cells for `n = 8 … 20`, `TABLE.md`.
- An exploratory fit, `fit.sage`, of the constant term of the
  Kosters–Yeo trace relation on 189 draws at `n = 6 … 10`. Its seeds are
  `31000·n + s`, disjoint from every seed below.
- Three `n = 12` syzygy draws (seeds `60000–60002`) that motivated E1b.
  E1b uses seeds `5000·n + s`, so `n = 12` reuses `60000–60007`; those
  three draws are excluded from scoring.

## System

- Curve `y² + xy = x³ + b` over `F_{2^n}`, minimal-weight modulus.
- Target `x_3`: uniform random for `rand` cells. For `sat` cells it is a
  root of `S_3(v_1, v_2, T)` with `v_1, v_2 ∈ V`.
- Factor base `V ⊂ F_{2^n}` of dimension `n' = ⌊n/2⌋`. It is either
  random (`random`) or `F_{2^{n/2}}` (`subfield`).
- The system `F`: the `n` Weil-descended equations of `S_3(X_1, X_2, x_3)`
  with `X_i ∈ V`, in `N = 2n'` Boolean unknowns.
- `d_F`: the Huang–Kosters–Yeo last fall degree, computed by the mutant
  closure in the Boolean ring (definition and equivalence argument in the
  ledger, §0).
- `d_ff`: the operational first fall degree.

## E1: the trace relation, exactly

**Hypothesis H1 (literature-derived, sharpened).** For every draw with
`x_3 ≠ 0`, the polynomial

```
L = Σ_j Tr(v_j)·(y_{1j} + y_{2j}) + Tr(b / x_3²)
```

lies in `span_{F_2}(F)`, the degree-2 Macaulay space with no multipliers.
Here `v_j` is the basis of `V`, `y_{ij}` are the bits of `X_i`, and `Tr`
is the absolute trace.

Consequences, which become theorems once H1 is proved (§P1):

- **C1.** If `Tr|_V ≢ 0`, then `d_ff = 2`.
- **C2.** If `Tr|_V ≡ 0` and `Tr(b/x_3²) = 1`, then `1 ∈ span(F)`. The
  target is then not decomposable over `V`, and `d_F ≤ 2`.
  - The subfield base at `n = 2n'` always has `Tr|_V ≡ 0`.
  - So C2 predicts that about half of all subfield targets are refuted
    at degree 2 by one trace bit.
- **C3.** If `Tr|_V ≡ 0` and `Tr(b/x_3²) = 0`, then `L = 0` and the trace
  gives no fall. Any `d_ff = 2` there has another cause.

**Run.** `sage exp.sage trace n n/2 8 {random,subfield} rand 6` for
`n ∈ {8, 10, 12, 14, 16}`, and the random base at odd `n ∈ {9, 11, 13, 15}`.

**Falsified if** any of these holds:

- some draw has `L ∉ span(F)`;
- some draw with `Tr|_V ≢ 0` has `d_ff ≠ 2`;
- some subfield draw with `Tr(b/x_3²) = 1` is not refuted at degree 2.

**Registered side prediction.** Among draws with `Tr|_V ≢ 0`, the
degree-≤1 part of `span(F)` has dimension exactly 1 in every draw at
`n ≥ 12`.

## E1b: the "bounded-defect" syzygy is the trace relation

`RESEARCH_FFD_PROOF_COMPLEXITY.md` §3.4 found, in EXP-J, exactly one
excess degree-3 syzygy `ℓ·(Σ_{i∈S} f_i) ≡ 0`, with `ℓ` symmetric. It
proposed this as the antecedent of its conditional lower bound.

**Hypothesis H1b.** That syzygy is the trivial Boolean identity
`(L + 1)·L = 0` applied to the trace relation. Concretely:

- the space of degree-3 linear syzygies `Σ_i ℓ_i f_i = 0` has dimension
  exactly 1;
- its multiplier is `ℓ = L + 1`, and `Σ_{i∈S} f_i = L`.

**Run.** `sage exp.sage syz n n/2 8 {random,subfield} rand 0` at
`n ∈ {12, 14, 16, 18, 20}`, and the random base at odd `n ∈ {13 … 19}`.

**Falsified if** a random-base draw with `Tr|_V ≢ 0` and `n ≥ 12` has
`syz_dim ≠ 1`, or its multiplier is not `L + 1`.

**Consequence if it holds.** The bounded-defect lemma has a proof of
existence: Kosters–Yeo Cor. 4.11 plus the Boolean identity. Its
uniqueness half reduces to C1's dimension statement.

## E2: is the low `d_F` structural or generic?

**Controls.** Random Boolean quadratic systems with the same shape: `n`
equations in `n` unknowns. They come in two kinds:

- `plain`;
- `planted_linear`: the last equation is the sum of the others plus a
  random affine-linear form. This mimics the trace fall.

Each kind is run unsatisfiable-or-random (`rand`) and with a planted root
(`sat`). The run is `sage exp.sage control n n/2 4 kind {rand,sat} 6` at
`n ∈ {8, …, 18}`.

**H2a.** `d_F(Semaev) ≤ d_F(planted_linear control)` holds at every `n`,
comparing cell medians with the same satisfiability.

**H2b.** The gap `d_F(control) − d_F(Semaev)` is nondecreasing in `n`.

- The literature anchors this. The Kousidis–Wiemers F4 top degree, which
  is at least `d_F`, is 5 at `n = 48`. The semi-regular `D_reg` there is
  8.
- So a generic control must eventually sit above Semaev.

**Falsified if** Semaev's median `d_F` exceeds the control's at any `n`
(H2a fails), or the gap shrinks between consecutive `n` (H2b fails).
Equality at every `n ≤ 18` would not falsify H2a. It would mean the
structure becomes visible only beyond toy size, and E3 would decide.

## E3: scaling of `d_F` for `m = 2` (needs E5's engine)

Cells: `n ∈ {22, 24, 26, 28, 32}`, random base, 8 draws, rand and sat.

- **P3a.** `d_F ≤ 5` for every `n ≤ 32`. This follows from Kousidis–
  Wiemers' F4 data together with `d_F ≤ sd` (Caminata–Gorla). A violation
  would mean a definitional or engine bug, not a discovery.
- **P3b, the scientific question.** Between `n = 18` and `n = 32`,
  `d_F` steps from 4 to 5 at most once.
- Two models compete:
  - `LOG`: `d_F = a + b·log₂ n`, fitted on `n ≤ 22`;
  - `LIN`: `d_F = a + n/c`, fitted on the same points.
- The model with the smaller absolute error on `n ∈ {24 … 32}` is
  preferred. A tie is reported as a tie.

**Kill condition for "d_F bounded".** `d_F` reaches 5 by `n = 32` *and*
the step widths `n(3→4)`, `n(4→5)` grow less than geometrically.

## E4: `m = 3` (chained `S_3` with one auxiliary variable; needs E5)

The cells are the `dreg_ladder` cells `(n, ℓ) ∈ {(5,2), (7,2), (5,3), (7,3)}`,
on the same draws as their committed runs.

- **P4a (THEOREM check).** `d_F ≤ D*` on every draw, where `D*` is the
  ladder's plain refutation degree. A violation is a bug.
- **P4b.** `d_F < D*` on at least half the draws. The mutant closure is
  then strictly cheaper than the ladder's plain tower.
- **P4c.** The Kousidis–Wiemers bound `d_ff ≤ m² − m + 1 = 7` holds for
  the direct `S_4` descent at `n ≤ 15`.

## E5: native engine (prerequisite for E3, E4)

Port the closure `V_{F,c}` to Rust on the existing M4RI path. The port
must reproduce the per-degree dimension of `V_{F,c}` on every Sage cell
in `runs/` and `exp/`, with exact equality. Any disagreement blocks E3
and E4.

## Proof obligations

| id | statement | route | status at registration |
|---|---|---|---|
| P1 | H1 as a theorem for `a_1 = 1, a_3 = 0` curves | Kosters–Yeo Cor. 4.11 with `a_2` chosen so that `x_3` lies on `E_{a_2}`. Their constant is `Tr(x_3 + a_2)`, and `Tr(x_3 + a_2) = Tr(b/x_3²)` exactly when `(x_3, ·) ∈ E_{a_2}(F)`. `S_3` does not depend on `a_2`. | proof sketch, ledger §5 |
| P2 | C1–C3 | linear algebra from P1 | follows from P1 |
| P3 | `dim(span(F) ∩ R_{≤1}) = 1` when `Tr|_V ≢ 0`, `n' ≥ 6` | genericity of the remaining `n−1` quadratic parts over a random `V` | open, E1 and E1b evidence |
| P4 | `d_F ≥ 3` for random `V` with `Tr|_V ≢ 0`, `n ≥ 10` | show that `V_{F,2}` misses some degree-2 element of the ideal | open |
| P5 | `d_F = ω(1)`, or a uniform bound | PC-degree lower bound on the trace-quotiented system (Alekhnovich–Razborov immunity), or an explicit low-degree certificate | open, decided empirically by E3 at toy size only |

## Scoring

`score.py` (to be written after the runs, from committed JSONL only)
reports each hypothesis as **held**, **falsified**, or **not testable**,
quoting the cell that decides it. Results go in `RESULTS.md`. This file
is not edited after commit except by dated amendments.

## Amendment 1 (2026-10-08, after E1 output was read)

**What went wrong.** The as-run `exp.sage trace` built the predicted
relation with constant `Tr(x_3)`, the pre-fit guess. The registered
hypothesis H1 uses `Tr(b/x_3²)`, the constant found by `fit.sage`. The
syzygy mode (E1b) used the registered constant. So E1 as run tested the
wrong statement, and its H1 and C2 verdicts score that wrong statement.

**What changes.** `rescore_e1.sage` recomputes the registered constant
from each record's stored `x_3` and `b`. It keeps the stored linear part,
`predicted_L` minus its constant, and re-tests membership against the
stored echelon basis of `span(F) ∩ R_{≤1}`, `deg_le1`. No draw was rerun
and no seed changed. `score.py` reports both the as-run verdicts and the
Amendment 1 verdicts, and `RESULTS.md` quotes both.

## Amendment 2 (2026-10-12, before any E3 cell runs): E5 engine, its validation scope, E3 seeds

**Engine.** E5 is `examples/hky_last_fall_degree.rs`, native Rust.

- **Closure.** The mutant closure `V_{F,c}` uses the Boolean convention of §0 ("multiply
  `g` by `m` only if `deg m + deg g ≤ c`"). It is computed by a work queue over a
  leading-monomial echelon basis.
- **Ideal side.** It does not use Sage's Gröbner basis. With the field equations the
  Boolean ideal is radical, so `I = I(S)` for the exact solution set `S`. `S` is
  enumerated by solving `S₃ = 0` as a quadratic in `X₂` for each `X₁ ∈ V`. So
  `dim (I ∩ R_{≤c})` is a rank on `S`.
- **`DG`.** It is the top degree of the minimal non-standard monomials of `S` in Sage's
  deglex, found by a Buchberger–Möller scan.
- **Early stop.** The engine stops at `e₀`, where Sage continued to `cmax`. `d_last`
  depends only on degrees `≤ e₀`.

**Validation scope (a deviation from E5's text, disclosed).**
- The Sage records store `(n, x₃, b)`, but not the random subspace basis. So "reproduce
  every Sage cell" is possible only for the `subfield` cells, where `V = F_{2^{n/2}}` is
  fixed by `n`. Closure and ideal dimensions are invariant under a change of basis of `V`.
- On those 32 records (`n = 8, 10, 12, 14`; rand and sat) the engine matches Sage exactly:
  `DG`, `d_last`, `e₀`, and the closure, ideal and plain dimensions at every degree both
  computed. The modulus is NTL's sparse irreducible, which is Sage's `minimal_weight`.
  Output: `e5/replay_subfield.jsonl`.
- The random-base Sage cells are compared by distribution only: the native E3 run
  includes `n = 18, 20, 22`, overlapping `TABLE.md`.

**E3 as run.** The cells are as registered: random base, 8 draws each, `rand` and `sat`,
at `n ∈ {22, 24, 26, 28, 32}`. Overlap cells at `n ∈ {18, 20}` are added for the
comparison above; they are not part of P3a or P3b.
- Seeds: `hky_last_fall_degree cell --seed 20261012`, so draw `s` uses
  `20261012·1000 + s`.
- `cmax = 6`. A draw with no `e₀ ≤ 6` is reported censored.
- Scoring follows §E3 unchanged: P3a, P3b, the LOG/LIN comparison, and the kill
  condition. The LOG and LIN fits use the per-`n` median `d_F` on `n ≤ 22`, mixing the
  Sage `TABLE.md` cells at `n ≤ 16` with native ones at `n = 18–22`. They are scored on
  `n ∈ {24…32}`.
- Nothing from any random-base E3 cell has been seen. Two timing draws with seed 1 at
  `n = 22` and `26` were discarded unseen.

## Amendment 3 (2026-10-12, before any E4 draw is measured): E4 as run, and an exploratory extension

**E4 as registered.** E4 is the four `m = 3` cells, using their committed unsatisfiable
draws (`solutions = 0`), rebuilt exactly from the recorded `v_basis` and `x_r` with
`koblitz_groebner::build_decomposition_system` (`b = 1`, `find_irreducible_sparse(n)`):

| cell | records |
|:--|:--|
| (5, 2), (7, 2), (7, 3) | `research/dreg_fixed_surplus_20260923/runs/cell-*-7.jsonl` |
| (5, 3) | `research/dreg_ell_grid_20260925/runs/cell-5-3-7.jsonl` |

- **What is measured.** These systems have no solutions, so `I = (1)` and `d_F` is the
  first `c` with `1 ∈ V_{F,c}`, the polynomial-calculus refutation degree (ledger §1).
- **Engine.** The E5 engine computes it with the same closure, stopping once `1` enters.
- **Scoring.** P4a and P4b are scored exactly as registered. P4c (the direct-`S₄`
  first-fall bound) is not run here and is reported not tested.

**Exploratory extension (not registered for scoring; reported separately and labelled).**
The same measurement is taken on the committed unsatisfiable draws of:

| `m` | cells | `cmax` | records |
|:--|:--|:--|:--|
| 3 | (9, 3), (11, 4) | 6 | `dreg_fixed_surplus_20260923` |
| 3 | (13, 4) | 5 | `dreg_fixed_surplus_20260923` |
| 4 | (5, 2), (7, 2) | 6 | `research/dreg_m4_chain_20261006` |
| 4 | (9, 3) | 5 | `research/dreg_m4_chain_20261006` |

- **The question.** Does `d_F` stay below `D*` as `ℓ` grows (`D*` = 5, 6, 6 at `m = 3`;
  6 and `≥ 7` at `m = 4`)?
- **Reading rule.** Fixed now: "`d_F` flat while `D*` rises" needs `d_F ≤ D* − 2` at the
  largest `ℓ` measured, for both `m`. Otherwise the reading is "`d_F` tracks `D*`".
- **Censoring.** A draw not refuted by `cmax` is censored (`d_F > cmax`).

Nothing from any E4 or extension draw has been computed.

## Amendment 4 (2026-10-12, before any E6 draw is measured): E6, `d_F` beyond the ladder's reach

**Why.** On every E4 and extension draw (`m = 3`, ℓ = 2–4; `m = 4`, ℓ = 2–3; 32 draws,
each cross-checked by an independent implementation, `examples/hky_crosscheck.rs`), `d_F`
is 3 or 4. Meanwhile the plain refutation degree `D*` rises from 5 to `≥ 7`. E6 asks
whether `d_F` stays at 4 where `N`, `ℓ` and `m` are larger. It also asks the question that
matters for relation finding: satisfiable targets (HKY Prop 2.8 solves at
`max(d_F, #solutions)`).

**Part A (committed draws).** `m = 3`, cells (13, 5) and (15, 5), with `N = 28, 30`. These
are the four unsatisfiable draws each in `research/dreg_fixed_surplus_20260923/runs/cell-1{3,5}-5-6.u*.jsonl`,
with recorded `D* ≥ 7`. `cmax = 5`.

**Part B (new draws).** Drawn exactly as `dreg_ladder` draws them: `ladder_sample` with
`cell_seed(20261012, n, ℓ, m)`, keeping the first 4 draws with no solutions. `cmax = 4`;
a draw not refuted by degree 4 is reported as `d_F ≥ 5`.

| `m` | cell | `N` |
|--:|:--|--:|
| 3 | (18, 6) | 36 |
| 4 | (13, 4) | 42 |
| 5 | (10, 2) | 40 |

**Part C (satisfiable draws).** `m = 3`, using the committed satisfiable draws
(`solutions > 0`) of cells (7, 3), (9, 3) and (11, 4), up to 4 per cell.
- `d_F` uses the record's own definition: `d_last` from closure dimensions against
  `dim (I(S) ∩ R_{≤c})`, with `DG` from the standard monomials of `S`.
- `S` is enumerated from the chain and checked against every equation.
- `cmax = 5`.

**Reading rule (fixed now; exploratory, not part of the registered E3/E4 scoring).**

| reading | condition |
|:--|:--|
| **`d_F` bounded at toy size** | every A and B draw has `d_F ≤ 4`, **and** every C draw has `d_last ≤ 4` |
| **`d_F` grows** | the median `d_F` of some A or B cell is `≥ 5`, or some C cell's median `d_last` is `≥ 5` |
| **mixed** | otherwise |

Even "bounded" is evidence at toy size only. The ledger's conditional (§2.2) says `d_F`
must eventually be `ω(m²)` unless ECDLP over `F_{2^n}` is subexponential.

Nothing from any E6 draw has been computed. Part B's draws have not been generated.

## Amendment 5 (2026-10-12, before any control is measured): E6 Part D, structure-matched random controls

**Why.** Every Semaev draw measured so far has `d_F ∈ {3, 4}`. Without a control, "`d_F = 4`"
could be a property of any system of this shape at these sizes, rather than of Semaev
systems.

**Controls.** `koblitz_bench::random_control_system(n_vars, n_vars + 4, 3, terms_per_eq, seed)`,
the `dreg_ladder` control construction. It has degree 3 and the Semaev cell's own term
density. With 4 more equations than unknowns it is unsatisfiable with high probability:
- every control is checked unsatisfiable by exhaustive evaluation over `F₂^N`;
- a satisfiable one is skipped and logged.

The shapes are the committed ladder controls' `(n_vars, terms_per_eq)`:

| cell | `n_vars` | `terms_per_eq` |
|:--|--:|--:|
| `n5l2` | 11 | 14 |
| `n7l3` | 16 | 39 |
| `n11l4` | 23 | 83 |
| `n13l5` | 28 | `ladder_terms_per_eq(13, 5, 20261012)` |

- **Draws:** 4 controls per shape, seeds `20261012·100 + t`.
- **Measurement:** `d_F` is the first `c` with `1 ∈ V_{F,c}`, with `cmax = 5`.

**Reading rule (fixed now).** The control is more constrained than the Semaev system
(`N + 4` equations against Semaev's `2n`).

| reading | condition |
|:--|:--|
| **"Semaev-specific"** | the control's median `d_F` is `≥` the Semaev cell's median `d_F` + 1 at every shape |
| **"generic at toy size"** | the control's median `d_F ≤ 4` at every shape |
| **mixed** | otherwise |

Nothing from any control has been computed.

## Amendment 6 (2026-10-12, after Part D's controls turned out satisfiable; before any `d_last` on them)

**What happened.** All 12 Part D controls (shapes `n5l2`, `n7l3`, `n11l4`; 4 seeds each)
have 1 or 2 solutions by exhaustive evaluation. `random_control_system` plants a root. So
Part D as registered, which needs unsatisfiable controls, is **not testable**. The closure
dimensions were computed and are recorded (`e6/D_controls_small.jsonl`). The refutation
degree is undefined for these systems.

**Part D′ (replacement).** Use the same 12 controls, plus 4 at the `n13l5` shape. Compute
`d_last` exactly as Part C does for satisfiable Semaev draws:
- the solution set `S` by exhaustive evaluation over `F₂^N`;
- `dim (I(S) ∩ R_{≤c})` and `DG` from `S`;
- `e₀` and `d_last` as in `lfd_fast.sage`;
- `cmax = 6`.

The comparison is Part C's satisfiable Semaev draws, where `d_last = 4` on all 6.

**Reading rule (fixed now).**

| reading | condition |
|:--|:--|
| **"Semaev-specific"** | every shape has control median `d_last ≥ 5` |
| **"generic at toy size"** | every shape has control median `d_last ≤ 4` |
| **mixed** | otherwise |

## Amendment 7 (2026-10-18, before any E7 draw is generated): E7, `d_F` at `m ≥ 5` and `N` up to 60

**Why.** E6 reached `N = 40` at `m = 5` and only `ℓ = 2`. Every draw so far has `d_F ≤ 4`.
The HKYY conditional needs `d_F = ω(m²)` eventually. So the question is whether `d_F`
starts to move once `m ≥ 5` and `ℓ ≥ 3`, and as `N` grows toward the limit of dense
linear algebra.

**Engine.** `examples/hky_par.rs`. It computes the same mutant closure as
`hky_last_fall_degree` (Amendment 2), with different mechanics:
- columns are ranked by the combinatorial number system;
- candidates are reduced against the basis in parallel, chunk by chunk;
- the residues are eliminated against each other in parallel.

**Gate (must pass on the exact binary that runs E7, before any E7 draw):**
1. `e7/validate.sh` reproduces `d_F` and every per-degree closure dimension of the
   validated engine. That covers 24 committed draws: `m = 3` cells (5,2), (7,3), (11,4),
   (13,4), (13,5), and `m = 4` (9,3). Required: `AGREE 24 DISAGREE 0`.
2. `newcells --cells 5:10:2 --seed 20261012 --keep 4 --cmax 4` must reproduce
   `e6/B_5-10-2.rerun.jsonl`: the same four draws, the same `d_F` (4, 3, 4, 4) and the same
   closure dimensions.

If either check fails, E7 does not run.

**E6 Part B completion.** The m = 4 (13, 4) cell was lost to two container restarts. It
is rerun with this engine. The cell, seed, `keep` and `cmax` are as in Amendment 4, and
the output goes to `e6/B_4-13-4.par.jsonl`. It is scored under Amendment 4's rule.

**E7 cells.** Draws are made as in E6 Part B:
- `ladder_sample` with `cell_seed(20261012, n, ℓ, m)`;
- keep the first 4 draws with no solutions;
- `cmax = 4`, and a draw not refuted by degree 4 is reported as `d_F ≥ 5`.

Unknowns: `N = (m − 2)·n + m·ℓ`. Memory is dominated by the degree-4 echelon basis, about
`(#columns)²/16` bytes.

| tier | `m` | `(n, ℓ)` | `N` | columns `≤ 4` | basis memory |
|:--|--:|:--|--:|--:|--:|
| local (4 cores, 15 GB) | 6 | (8, 2) | 44 | 149 986 | ≈ 1.4 GB |
| local | 5 | (11, 3) | 48 | 213 053 | ≈ 2.8 GB |
| local | 6 | (9, 2) | 48 | 213 053 | ≈ 2.8 GB |
| large host (≥ 64 GB) | 5 | (13, 3) | 54 | 342 541 | ≈ 7.3 GB |
| large host | 6 | (10, 3) | 58 | 456 838 | ≈ 13 GB |
| large host | 5 | (15, 3) | 60 | 523 686 | ≈ 17 GB |

- The local tier runs now.
- The large-host tier runs only on a host with at least 64 GB of RAM, using the same
  binary build and the same gate.
- If no such host is available, the large tier is reported as **not run**. It is not
  extrapolated.
- `N ≈ 100` is out of reach of a dense degree-4 closure: about 4.1 million columns and a
  ≈ 1 TB basis. It is not registered here, and would need a sparse or structured method.

**Reading rule (fixed now; exploratory, like E6).**

| reading | condition |
|:--|:--|
| **`d_F` bounded through `m = 6`** | every E7 draw that ran has `d_F ≤ 4` |
| **`d_F` grows** | some E7 cell has median `d_F ≥ 5` |
| **mixed** | otherwise |

- Each tier is read separately, and so is the pooled set.
- "Bounded" here is still evidence at toy size, for the same reason as in E6. A bounded
  reading does not contradict the HKYY conditional, which constrains asymptotics only.
- Wall time and per-degree closure dimensions are recorded. They are not scored.

Nothing from any E7 cell has been computed, and no E7 draw has been generated.

## Amendment 8 (2026-10-19, after the E7 local tier; before any draw of the cell it moves)

- **Observation.** The local tier finished well inside its budget. The N = 48 cells took
  90–553 s per draw on 4 threads, and no draw needed more than 15 GB.
- **Change.** The m = 5, (13, 3), N = 54 cell moves from the large-host tier to the local
  tier. Its dense basis is ≈ 7.3 GB, which fits the 15 GB host.
- **Unchanged:**
  - the binary (sha256 `24cf10d6…`, which passed the Amendment 7 gate);
  - the draw rule, `keep = 4` and `cmax = 4`;
  - the reading rule.
- **Peak memory.** It is recorded with `/usr/bin/time -v` if available.
- **If the run is killed** by memory or by a container restart, the cell is reported as
  **not completed**, together with whatever draws finished. A finished draw is never
  rerun selectively.
- **Still large-host.** (6, 10, 3) N = 58 and (5, 15, 3) N = 60 stay in the large-host tier.

No draw of the (13, 3) m = 5 cell has been generated.

## Amendment 9 (2026-10-25, before any draw of the cells below): E8, `d_F` against `m` at fixed `(n, ℓ)`

**Why.** HKYY's growth requirement is in `m`, not in `N`. E3–E7 mostly raised `N` by raising
`n` or `ℓ`. E8 holds `(n, ℓ)` fixed and raises `m` alone. `N = (m − 2)·n + m·ℓ`, so the
(8, 2) ladder runs `N = 14, 24, 34, 44, 54` for `m = 3 … 7`. The (6, 2) ladder runs
`N = 10 … 52` for `m = 3 … 8`.

**Engine and gate.** The same binary as E7 (`hky_par`, sha256 `24cf10d6…`, which passed the
Amendment 7 gate); the gate is not rerun.

**Cells.** Draws as in E6 Part B: `ladder_sample` with `cell_seed(20261012, n, ℓ, m)`, keep
the first 4 draws with no solutions, `cmax = 4` (a draw not refuted by degree 4 is
`d_F ≥ 5`).

| ladder | `m` values | cells `(m:n:ℓ)` |
|:--|:--|:--|
| (8, 2) | 3, 4, 5 | 3:8:2, 4:8:2, 5:8:2 (6:8:2 is E7, not rerun) |
| (8, 2) | 7 | 7:8:2 (`N = 54`) |
| (6, 2) | 3 … 8 | 3:6:2, 4:6:2, 5:6:2, 6:6:2, 7:6:2, 8:6:2 (`N = 52`) |

**Reading rule (fixed now; exploratory).** Per ladder, using the median `d_F` of each `m`
(the E7 (8, 2), m = 6 median is 4):

| reading | condition |
|:--|:--|
| **`d_F` flat in `m`** | every median `d_F ≤ 4` and the medians are non-decreasing in `m` with a spread of at most 1 |
| **`d_F` grows in `m`** | some median `d_F ≥ 5`, or the medians rise by 2 or more across the ladder |
| **mixed** | otherwise |

- A cell that runs out of memory or time is reported as **not completed**, with any draws
  that finished. A finished draw is never rerun selectively.
- A cell with no draw free of solutions in 1000 draws is reported as such.

**Not in this amendment.**
- `N = 58–60` (large-host tier of Amendment 7) stays **not run**. `AWS_SECRET_ACCESS_KEY`
  and a region are absent. `e7/aws_large_tier.sh` is prepared but has not been run.
- `N ≈ 100` needs a sparse closure engine (about 4.1M columns at degree 4). It is scoped
  in `e8/N100_SCOPING.md` and is not measured.

Nothing from any E8 cell has been computed.

## Amendment 10 (2026-10-27, before any draw of the cells below): E9, the large-host tier; E10, the balanced diagonal `mℓ = n`

**Why.**
- **E9.** Amendment 7's large-host tier, (6, 10, 3) `N = 58` and (5, 15, 3) `N = 60`, was
  never run because no AWS route existed. Lab-account credentials are now present, and the
  `ephemeral` one-command Spot runner (`taskq/deploy/ephemeral`) is deployed in it.
- **E10.** HKYY's growth requirement is along `m ≈ n^{1/3}`, `n' = n/m`. That is, the
  factor base is balanced: `mℓ ≈ n`. E8 raised `m` at `n ≤ 8` with `ℓ = 2`, so `mℓ ≥ n`
  there, and the field was small. E10 instead holds `mℓ = n` and raises `n` along each `m`,
  as far as a dense degree-4 closure reaches (`N ≤ 60`).

**Route and measurement class.** The output is a count (`d_F`, closure dimensions), not a
timing, so this is a `search`-class job. It runs on `ephemeral` Spot instances in the lab
account. The Iso Lab EKS request contracts accept only `ecbench` specs from outside the
cluster, so they cannot run this binary. Wall times are recorded but not scored, and are
valid only as rough indications (no isolation level).

**Engine change.** `examples/hky_par.rs` now counts solutions with
`chained_solution_count_fast` (one Artin–Schreier lookup per link) instead of a scan of
`F_{2^n}`. The scan costs about `2^{(m−1)ℓ+n}`, which is infeasible at `n ≥ 20`. The count is
exact: the unit test `fast_chained_count_matches_the_reference` holds it to the scan at
`m = 3 … 6`. So the same draws are kept. The closure code is unchanged.

**Gate (on the exact binary each instance builds, before any E9/E10 draw on that
instance).** If any part fails, that instance runs no cell.
0. `cargo test --release --lib -- fast_chained_count_matches_the_reference chained_count`
   passes (the counter equals the scan; run on the instance, because the local host lacks
   the disk to build the test target).
1. `e7/validate.sh`: `AGREE 24 DISAGREE 0`.
2. `newcells --cells 5:10:2` reproduces `e6/B_5-10-2.rerun.jsonl` (draw, `d_F`,
   `v_basis`, `x_r`, closure dimensions).
3. New, for the counter change: `newcells --cells 3:18:6` reproduces
   `e6/B_3-18-6.jsonl`, and `newcells --cells 4:8:2,5:8:2` reproduces
   `e8/E8_4-8-2.jsonl` and `e8/E8_5-8-2.jsonl`, on the same fields.

**Draw rule.** As E6 Part B and E7–E8: `ladder_sample` with `cell_seed(20261012, n, ℓ, m)`;
keep the first 4 draws with no solutions; `cmax = 4`, and a draw not refuted at degree 4
is `d_F ≥ 5`.

**Cells.** `N = (m − 2)·n + m·ℓ`.

| experiment | `m` | `(n, ℓ)` | `N` | columns `≤ 4` | basis, formula |
|:--|--:|:--|--:|--:|--:|
| E9 | 6 | (10, 3) | 58 | 456,838 | ≈ 13 GB |
| E9 (also E10, `m = 5`) | 5 | (15, 3) | 60 | 523,686 | ≈ 17 GB |
| E10 | 3 | (21, 7) | 42 | 124,314 | ≈ 1.0 GB |
| E10 | 3 | (24, 8) | 48 | 213,053 | ≈ 2.8 GB |
| E10 | 3 | (27, 9) | 54 | 342,541 | ≈ 7.3 GB |
| E10 | 4 | (12, 3) | 36 | 66,712 | ≈ 0.3 GB |
| E10 | 4 | (16, 4) | 48 | 213,053 | ≈ 2.8 GB |
| E10 | 4 | (20, 5) | 60 | 523,686 | ≈ 17 GB |
| E10 | 6 | (12, 2) | 60 | 523,686 | ≈ 17 GB |

The measured peak is about 1.7× the formula (E7, `N = 54`). Instances need ≥ 64 GB.

Earlier diagonal points, read with E10 but not rerun:
- `m = 3`: (9, 3), (15, 5) and (18, 6);
- `m = 4`: (8, 2);
- `m = 5`: (10, 2).

**Reading rules (fixed now; exploratory).**
- **E9:** Amendment 7's rule, read for the large tier and pooled with E7.
- **E10**, per `m` ladder along the diagonal, by the median `d_F` of each cell:

| reading | condition |
|:--|:--|
| **`d_F` flat on the diagonal** | every median `d_F ≤ 4` |
| **`d_F` grows on the diagonal** | some median `d_F ≥ 5` |
| **mixed** | otherwise |

- **Cross-`m` reading at fixed `N`.** At `N = 48`, compare (3, 24, 8) and (4, 16, 4).
  At `N = 60`, compare (4, 20, 5), (5, 15, 3) and (6, 12, 2). The reading is "rises with
  `m`" if the median strictly increases with `m` at a fixed `N`. Otherwise it is "not
  rising".
- A cell that runs out of memory or time is **not completed**, with whatever draws finished.
- A Spot interruption is an infrastructure failure. The interrupted job is relaunched once,
  from scratch, on the same commit. The computation is deterministic, so the relaunch
  makes the same draws. The interrupted attempt's partial outputs are kept and must agree
  with the relaunch. A second interruption makes the remaining cells **not completed**.
- A cell with no draw free of solutions in 1000 draws is reported as such.
- Nothing is extrapolated beyond `N = 60` here. `N ≈ 100` is a separate engine (E11) with
  its own validation, and is registered separately before any `N ≈ 100` draw.

Nothing from any E9 or E10 cell has been computed, and no E9 or E10 draw has been generated.

## Amendment 11 (2026-10-27, after E9 and E10 were read; before any draw below): E12, the exact `d_F` at (3, 21, 7), and ℓ against `m` and `n`

**Why.** E10 read `d_F ≥ 5` on every draw of (3, 21, 7), (3, 24, 8) and (3, 27, 9). Every
other E9/E10 cell, up to `N = 60`, read `≤ 4`. That includes (4, 20, 5), (5, 15, 3) and
(6, 12, 2). The censored cells are exactly the cells with `ℓ ≥ 7`, and all of them have
`m = 3` and sit on the diagonal. Three questions follow:
1. What is the exact `d_F` there?
2. At `m = 3` and fixed `N`, is the jump driven by `ℓ` or by `n`?
3. Does `ℓ = 7` at `m = 4` also censor?

**Hypothesis registered now (H_ℓ):** the median `d_F` of a cell is `≥ 5` iff `ℓ ≥ 7`,
whatever `m` and `n` are, at the sizes below.

**Route.** Same as Amendment 10: `ephemeral` Spot in the lab account, source shipped as a
`git archive` of the frozen commit. Counts only; wall time is recorded and not scored.

**E12a: degree 5 at (3, 21, 7).**
- Engine: `hky_par`, the same closure, `newcells --cells 3:21:7 --seed 20261012 --keep 2
  --cmax 5`. These are the first two of the four E10 draws: draws 0 and 1, which are
  regenerated, not copied.
- Degree `≤ 5` has 974,982 columns, so the basis is ≈ 59 GB by formula and ≈ 100 GB
  measured-scaled. One 256 GB host.
- Output: per draw, `d_F = 5`, or `d_F ≥ 6` (the degree-5 closure stops without 1).
- The degree-4 dimensions must equal E10's record for the same draw. If not, the draw is
  void.
- Gate: Amendment 10's gates 0–3 on the exact binary.

**E12b: `m = 3`, ℓ against `n` at about fixed `N`.**
- Engine: `hky_par newcells`, `cmax = 4`, `keep = 4`, seed 20261012.
- Cells, below the diagonal (`mℓ < n`, so unsatisfiable draws are common):

| cell | `N` | `ℓ` |
|:--|--:|--:|
| 3:30:4 | 42 | 4 |
| 3:27:5 | 42 | 5 |
| 3:24:6 | 42 | 6 |
| 3:24:7 | 45 | 7 |

- Gate: Amendment 10's gates 0–3.

**E12c: ℓ = 7 at `m = 4`, (4, 28, 7), `N = 84`.**
- Engine: `hky_window` (E11), policy `w1x`, built with `--features wide-vars`.
  `newcells --cells 4:28:7 --seed 20261012 --keep 4 --policies w1x`.
- The engine now counts solutions with `chained_solution_count_fast`, because the scan is
  infeasible at `n = 28`. It is exact.
- Output per draw:
  - "`d_F ≤ 3`" if the full degree-3 closure holds 1;
  - "`d_F = 4` certified" if it does not, and the restricted degree-4 closure reaches 1;
  - otherwise "inconclusive (`≥ 4`)".
- The engine cannot certify `d_F ≥ 5`.
- W(1) has 817,860 columns, so the basis is ≈ 42 GB by formula. One 128 GB host.
- Gate, on the exact wide build:
  - unit tests (gate 0);
  - `ladder --policies w1x` reproduces `e11/validate_final/` (`d_f_le3`, `d_f_window`,
    `w1x.cols`, `w1x.dim`, degree-2/3 dims) on `B_3-18-6`, `B_4-13-4`, `E7_6-8-2` and
    `E8_7-8-2`;
  - `newcells --cells 4:16:4 --keep 4 --policies w1x` reproduces the draws and those
    fields of `e11/scaling/window_4-16-4.jsonl`. This also checks the counter change
    at `m = 4`.

**Reading rules (fixed now).**

| part | reading | condition |
|:--|:--|:--|
| **E12a** | **"`d_F = 5`"** | both draws read 5 |
| | **"`d_F ≥ 6`"** | both draws are censored at 5 |
| | **"split"** | otherwise |
| **E12b** | **"consistent with H_ℓ"** | 3:30:4, 3:27:5 and 3:24:6 have median `≤ 4`, and 3:24:7 has median `≥ 5` |
| | **"`n`-driven"** | 3:30:4 and 3:27:5 both have median `≥ 5` |
| | **"mixed"** | otherwise |
| **E12c** | **"H_ℓ refuted at `m = 4`"** | at least 2 of the finished draws are "`d_F = 4` certified" |
| | **"no reading"** | otherwise. An inconclusive draw is not evidence for `≥ 5`. |

- A cell or draw that runs out of time or memory is **not completed**, with whatever
  finished.
- A Spot interruption gets one relaunch from scratch on the same archive.

Nothing from any E12 cell has been computed, and no E12 draw has been generated.

## Amendment 12 (2026-10-27, after E12b r1 and r2 were read; before any draw below): finish E12b cell 3:24:7 On-Demand

**Why.** Both E12b attempts finished 3:30:4, 3:27:5 and 3:24:6, each with all four draws
at `d_F = 4`. Both were then Spot-reclaimed after draw 0 of 3:24:7, which read `≥ 5`.
r2 equals r1 on every non-timing field. Amendment 11's one relaunch is used, so 3:24:7 is
not completed, and the E12b reading cannot be decided (`RESULTS.md`, E12 section).
The user asked on 2026-10-27 to finish 3:24:7 On-Demand.

**Disclosed before this run:** the 13 r1/r2 draws above, including draw 0 of 3:24:7.

**Run (E12b-OD).**
- Cell 3:24:7 only (`N = 45`), from scratch: `hky_par newcells`, `cmax = 4`, `keep = 4`,
  seed 20261012. Same frozen archive `fd-src-371035216942.tgz` (sha256 `4ae808ac…`) and
  `e12/run_par.sh`, without the Lean step.
- Host: `ephemeral --on-demand`, m7i.4xlarge or any x86 type with ≥ 16 vCPU and ≥ 32 GB.
  Counts only; wall time is recorded and not scored.
- Gates: Amendment 10's gates 0–3 on the exact binary. **Gate 4:** draw 0 must equal the
  r1/r2 record on every non-timing field; otherwise the run is void.
- An interruption or failure of any kind ends the run as **not completed**. There is no
  further relaunch without a new amendment.

**Reading (fixed now).** Amendment 11's E12b rules, with 3:24:7's median taken over the
four E12b-OD draws and the other three cells as recorded in r2:
- **"consistent with H_ℓ":** 3:24:7 has median `≥ 5`. The median is `lowerMedian` in
  `lean/FallDegree/Rule.lean`, the 2nd smallest of 4, with a censored value recorded as the
  cap plus one. So this needs at least 3 of the 4 draws at `≥ 5`.
- **"mixed":** otherwise. "`n`-driven" is already excluded.

**Not in scope.** E12a and E12c stay not completed under Amendment 11. Rerunning them
needs its own amendment.

## Amendment 13 (2026-10-28 UTC, 2026-10-27 Pacific): one new E12a/E12c attempt

**Authorization and disclosure.** The user has now explicitly requested launch of
the unfinished E12 follow-ups. All earlier E9/E10 results, the E11 engineering
record, and the interrupted E12 attempts have been read. This amendment is frozen
and committed before any new draw. It permits one additional from-scratch attempt
for E12a and E12c only; earlier interruptions and results remain unchanged.

**Scientific contract.** Amendment 11's cells, seed `20261012`, sample counts,
engines, gates, and reading rules are unchanged. Source remains archive
`fd-src-371035216942.tgz`, SHA-256
`4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f`, source commit
`371035216942660c4615dfbb88d85a14afaf5c16`. Every host verifies the archive before
extracting it. The original `run_par.sh` and `run_window.sh` execute unchanged.

| Part | Cell | Engine and draws | Host | Hard lifetime |
| --- | --- | --- | --- | --- |
| E12a | `3:21:7`, N=42 | `hky_par`, cap 5, first 2 unsatisfiable draws | `r7i.8xlarge`, 32 vCPU, 256 GiB | 48 h |
| E12c | `4:28:7`, N=84 | `hky_window`, `wide-vars`, `w1x`, 4 draws | `r7i.4xlarge`, 16 vCPU, 128 GiB | 48 h |

**Execution.** Existing ephemeral Spot backend in `us-east-1`, one instance per
part, concurrent where admitted, no `--keep`, no automatic retry. This uses 48
vCPUs against the observed 64-vCPU regional standard-Spot quota. Hardware identity,
source, binary digests, gates, status, and raw outputs are retained. Values are
exact-computation observations; all times are diagnostic, with no performance or
statistical comparison. CPU changes cannot justify a speed claim.

**Interruption policy.** These frozen engines have no resumable mid-closure
checkpoint. The existing runner saves available outputs on exit or interruption;
this is partial-output preservation, not process recovery. A reclaim, timeout,
OOM, gate failure, or other failure ends that part as not completed. No further
relaunch is authorized by this amendment. The command stops ten minutes before
the hard lifetime to allow uploads; deadline shutdown and the existing reaper
remain enabled. The user-requested bounded attempt is not replaced by a new
checkpoint-engine development project.

**Admission.** Run each part's original gates before its cell. E12a still requires
degree-4 dimensions to equal the E10 records for draws 0 and 1. E12c window failure
remains inconclusive and cannot certify degree at least 5. A launcher receipt or
exit code is not evidence of completed draws; full output counts and the original
gates must be checked before any reading.

**E12b stays pending.** Amendment 12 requires On-Demand. The current role explicitly
denies it through `SpotOnlyEC2Launch`. The question of a new Spot attempt has been
put to the user and has not been answered when this amendment is written. E12b is
not authorized by Amendment 13; any change is recorded separately before launch.

Launch scripts, the resource plan and receipts are stored under
`e12/continuation-20261028/`. All prior attempts remain separate.

## Amendment 14 (2026-10-28 UTC): E12a pre-execution capacity alternatives

The Amendment 13 E12a submission
`20261028-020921-fd-e12a-20261028-r3-4358b0` exhausted the configured zones with
`InsufficientInstanceCapacity` or `Unsupported`. Its durable status is `error`;
EC2 enumeration found no instance for that job. No solver, gate, or draw ran.
E12c has been submitted separately and is not relaunched by this amendment.

To fulfill the same user-requested E12a launch, allow at most two additional
capacity submissions, in this fixed order: `r6a.8xlarge`, then `r6i.8xlarge`.
Both are x86 hosts with 32 vCPU and 256 GiB. Source archive, seed, two draws,
degree cap 5, gates, 48-hour lifetime, and scientific readings are unchanged.
This amendment is committed before either fallback submission.

Each alternative requires the preceding submission's durable `error` status
to identify a no-Spot-capacity failure and a fresh EC2 check to find no instance
for it. Preserve every failed submission in a separate receipt. If any instance
is created, stop trying alternatives; there is only one admitted solver attempt.
An ambiguous launch, any other error, or a started job's interruption/failure
does not authorize an alternative. No additional instances beyond this ordered
list are authorized here. The wrapper enforces separate receipt directories and
the absence-of-instance check. This infrastructure amendment adds no new draw,
changes no scientific acceptance rule, and makes no timing comparison.

## Amendment 15 (2026-10-27 Pacific, 2026-10-28 UTC): one E12b Spot continuation

The user explicitly selected one new Spot attempt for E12b after the current
role denied On-Demand. This supersedes Amendment 12's market requirement for
this attempt only. The earlier partial results and On-Demand denial stay in the
record. Current EC2 enumeration finds only the admitted E12a/E12c jobs, and the
two earlier E12b jobs have durable `interrupted` status. No On-Demand E12b job was
launched. This amendment is committed and pushed before the new submission.

- **One attempt:** cell `3:24:7` (N=45), from scratch, cap 4, four unsatisfiable
  draws, seed `20261012`, using the unchanged `e12/run_par.sh` without Lean.
- **Frozen source:** `fd-src-371035216942.tgz`, SHA-256
  `4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f`, commit
  `371035216942660c4615dfbb88d85a14afaf5c16`.
- **Host and budget:** one `m7i.4xlarge` Spot instance in `us-east-1`, 16 vCPU,
  64 GiB, maximum lifetime 24 hours, no `--keep`. Together with E12a/E12c this
  uses 64 vCPU against the observed 64-vCPU quota. Timing remains diagnostic.
- **Gates:** original gates 0–3 on the exact binary, then Amendment 12's gate 4:
  draw 0 must equal the r2 reference on every non-timing field. The known timing
  fields are the top-level and per-degree `secs`; all other fields are compared,
  including thread count. The r2 reference is
  `e12/run-b2/out/E_3-24-7.jsonl`, SHA-256
  `589c5045361c9a9138baecae629ea91512c35a87efbc022f31780d2d3a91574a`.
  Earlier reconciliation established r1/r2 non-timing equality. Reference bytes
  and a completion checker travel separately from the frozen source archive and
  are hashed in the launch receipt. Neither changes the solver.
- **Completion:** require all four draws numbered 0–3, the expected cell/seed,
  zero solutions for each, successful cell exit, original gate completion, and
  gate 4 equality. Missing or failed checks prevent a completed reading even if
  the original wrapper exits zero. Preserve all raw outputs and checker logs.
- **Reading:** retain Amendment 12's lower-median rule, substituting these four
  new Spot draws for the unrun On-Demand draws; the other three cells remain r2.
  At least three of four draws at `>=5` are required for consistency with H_ell.
  A partial cell receives no reading. No independent implementation claim is made.
- **Stopping:** no resumable mid-closure checkpoint and no automatic relaunch.
  Reclaim, timeout, OOM, gate failure, or another failure ends this attempt as
  not completed. The command stops ten minutes before the hard deadline for
  output upload; existing deadline shutdown/reaper protections remain enabled.

Submission uses selection `b-spot` in `e12/continuation-20261028/launch.sh`,
with a separate receipt directory. This amendment does not relaunch E12a/E12c
or authorize any further E12b attempt.

## Amendment 16 (2026-10-27 Pacific, 2026-10-28 UTC): persist live E12 output files

The user requested assurance that datasets are persisted to S3. Inspection found
the existing runner uploads its main log every 15 seconds, but uploads `/work/out`
only on exit or Spot notice. At inspection E12b already had partial draw data,
while E12a/E12c's new-cell output files were empty. Immediate file snapshots were
requested through SSM command `fe4fcb90-c33a-4440-a0cd-de464130bf0c` to protect
existing bytes before installing a periodic uploader. No solver was restarted.

For the three admitted continuation job IDs, install a low-priority systemd
oneshot/timer that captures `/work/out` and available host/resource metadata to
`s3://ephemeral-runs-104186811093-us-east-1/jobs/<job>/snapshots/` about once per
minute. Each unique archive has a SHA-256 manifest published after S3 verifies
the archive checksum; `snapshots/latest.json` points to the newest committed
snapshot. Archive and manifest writes are conditional on absence. The existing
final upload under `out/` is unchanged and cannot be overwritten by this helper.

The helper validates the local job ID, refuses other jobs, respects its original
deadline, and uses a lock to prevent overlapping uploads. Failed uploads are
reported in the existing job log and retried by the timer; this is an artifact
upload retry, never a solver retry. The timer does not extend instance lifetime.
No engine, draw, seed, gate, scientific reading, or compute budget changes.
All timings remain diagnostic; additional file I/O precludes a timing comparison.

Snapshots capture live files and may contain a partial last record. They are not
atomic solver-memory checkpoints and do not enable mid-closure resume. Each
saved archive's bytes are independently verifiable; only complete draw records
and all original gates can support a later scientific reading. Preserve the
initial snapshots, installation receipts, readback hashes, and uploader source
under `e12/continuation-20261028/`. The observed bucket lifecycle expires `cache/`
objects only; it defines no expiration for these `jobs/` keys.

## Amendment 17 (2026-11-07 UTC, before any draw below): E13, the exact `d_F` at (3, 21, 7), an independent-engine check of every E10 `d_F ≥ 5` draw, and (4, 28, 7)

**Why.**
- E12a (the degree-5 closure at (3, 21, 7)) and E12c ((4, 28, 7), `N = 84`) were
  Spot-reclaimed three times each and have no reading.
- The E10 `d_F ≥ 5` draws have been confirmed only by runs of the same closure code
  (`hky_par`). Both `hky_crosscheck` attempts were lost to container restarts.
- The user approved all three follow-ups on 2026-11-07, with a hard cap of **USD 100**.

**Route.**
- One RunPod CPU pod: `cpu3g`, 64 vCPU, about 256 GB, `interruptible: false` (On-Demand,
  so not reclaimable). This is the route the owner approved on 2026-11-06 for
  `dreg_guess_scaling_20261106`. The lab AWS account's `SpotOnlyEC2Launch` policy forbids
  On-Demand EC2.
- **Budget:**
  - The pod gets a hard wall-clock limit of `floor(100 / costPerHr)` hours, at most
    36 h, counted from pod creation.
  - It is deleted when the job ends or the limit passes, whichever comes first.
  - Nothing runs after the limit, and no second pod is created without a new amendment.
- **Source and outputs:**
  - The source is a `git archive` of main `a2be660914e8e485eefd45f2958eb6d18344dadf` over
    `e13/source_paths.txt`. The pod fetches it by a presigned S3 URL and checks its
    SHA-256 before building.
  - Outputs are tarred to S3 by presigned PUT every 10 minutes, and once at the end.
  - Counts only: wall time is recorded, not scored.

**Gates (on the pod's exact build; if one fails, the parts that depend on it do not run).**
- **G0:** `cargo test --release --lib -- fast_chained_count_matches_the_reference
  chained_count partial_enlargement` passes.
- **G1 (for E13a and E13b):** `dreg_linear_guess --engine pe --to 4 --max-cols 140000
  --draw 0 --sum 15=7+0 --fix 23=0,24=1` on
  `research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl` reproduces
  `research/dreg_invariant_guess_20261029/runs/S3.d0.b4.jsonl`: degree 4 saturated,
  not refuted, `closure_rank` 131,588. This is Gate A2 of `dreg_guess_scaling_20261106`.
- **G2 (for E13c):** the gates of `e12/run_window.sh`, unchanged.

**E13b: independent-engine check (runs first).**
- **Engine:** `dreg_linear_guess --engine pe --batch 64 --to 4 --max-cols 400000`, no
  guesses. This is the partial-enlargement closure of `src/cryptanalysis/partial_enlargement.rs`,
  a different implementation from `hky_par`. It shares only the system construction
  (`build_decomposition_system`).
- **Censored draws:** all 12 `d_F ≥ 5` draws of E10, from the committed records
  `e10/run-r3-B/out/E_3-21-7.jsonl`, `E_3-24-8.jsonl` and `E_3-27-9.jsonl`.
- **Controls with a known refutation:** the four E10 draws of (4, 12, 3) (`d_F = 4`) and
  the four E9 draws of (6, 10, 3) (`d_F` 4 3 4 4).
- **Reading:**

| reading | condition |
|:--|:--|
| **"independent agreement"** | every censored draw saturates unrefuted at degree 4 with `closure_rank` equal to `hky_par`'s degree-4 `dim`, AND every control refutes at its recorded `d_F` |
| **"disagreement"** | otherwise. The draws that differ are listed; the E10 `≥ 5` reading for those cells is then suspended until resolved |

**E13a: the exact `d_F` at (3, 21, 7).**
- **Engine:** the same `pe` engine, `--to 5 --max-cols 1000000` (degree ≤ 5 has 974,982
  columns at `N = 42`).
- **Draws:** the four E10 draws of (3, 21, 7) (draws 0, 1, 3 and 4), one at a time, in that
  order.
- **Per draw:**
  - "`d_F = 5`" if the degree-5 closure contains 1.
  - "`d_F ≥ 6`" if it saturates without 1.
  - "unmeasured" if it is capped or the time limit passes.
  - The draw's degree-4 entry must equal E13b's for the same draw. If not, the draw is void.
- **Cell reading:**

| reading | condition |
|:--|:--|
| **"`d_F = 5` at (3, 21, 7)"** | at least 3 of 4 draws read 5 |
| **"`d_F ≥ 6` at (3, 21, 7)"** | at least 3 of 4 draws read `≥ 6` |
| **"split"** | otherwise, with at least 2 draws measured |
| **"not completed"** | fewer than 2 draws measured |

**E13c: (4, 28, 7) with the E11 engine.** `e12/run_window.sh "$OUT" 4:28:7`, exactly as
E12c: `hky_window`, `w1x`, `--features wide-vars`, `keep = 4`.
- **Reading:** Amendment 11's E12c rule. "H_ℓ refuted at `m = 4`" if at least 2 completed
  draws are "`d_F = 4` certified"; otherwise "no reading".
- **Consistency check:** draw 0 was already computed by E12c r3 (`e12/interrupted-c-20261028/W_4-28-7.jsonl`).
  The new draw 0 must equal it on every non-timing field. A difference is reported as
  engine nondeterminism.

**Scheduling.**
- E13b runs first, with all threads.
- Then E13a (40 threads) and E13c (24 threads) run concurrently.
- A part cut off by the time limit is "not completed", with whatever finished.
- Nothing is retried or relaunched under this amendment.

**Prediction (recorded before any draw).**
- E13b: "independent agreement", probability 0.85.
- E13a: "`d_F = 5`", 0.6; "`d_F ≥ 6`", 0.3; "not completed", 0.1.
- E13c: "no reading", 0.8. The E12c r3 draw 0 was inconclusive, and E12b's
  "mixed" reading weakens H_ℓ.

Nothing from any E13 part has been computed, and no E13 draw has been generated.

## Amendment 18 (2026-11-07 UTC, before any E13 draw): E13 runs locally instead of on RunPod

**Why.** The RunPod pod of Amendment 17 was refused for low account balance, so no pod
existed and nothing was run. The user then asked for E13 to run locally. Nothing from any
E13 part has been computed.

**Route.**
- The authoring container: 4 vCPU and 15 GB RAM.
- The source is the same frozen archive (`e13/source_archive.sha256`, main `a2be6609`),
  extracted and built here.
- Every computation runs under an address-space cap of 13 GiB (`ulimit -v`). A run that
  exceeds it fails cleanly and is recorded as **"not completed (memory)"**.
- No cloud cost.

**Changes from Amendment 17 (everything else unchanged):**
- **Threads:** 4 for every part.
- **Order:** sequential: gates, then E13b (the censored draws first, then the controls,
  smallest `N` first), then E13a, then E13c. The engines, flags, gates and reading rules
  are as in Amendment 17.
- **Draw granularity:** every draw is a separate process (`--draw D`) with its own output
  file. A finished draw is never recomputed.
- **Container restarts:** after a restart, the runner resumes at the first draw without
  an output file. That draw is recomputed from scratch, which is allowed because the
  computation is deterministic. The killed attempt's partial `.err` file is kept with an
  `.attemptN` suffix.
- **Time limit:** none per draw. A part still unfinished when the user stops the run is
  "not completed", with whatever finished.

**Expected (recorded before any draw).**
- E13b's censored draws fit (degree 4, at most 342,541 columns).
- The `N = 58` controls, E13a (974,982 columns at degree 5) and E13c (`N = 84`) are
  likely to exceed 13 GiB. Probability "not completed (memory)":
  - `N = 58` controls: 0.5;
  - E13a: 0.85;
  - E13c: 0.95.

Nothing from any E13 part has been computed, and no E13 draw has been generated.

## Amendment 19 (2026-11-09 UTC, before any draw below): E14, the parts of E13 that need more than 13 GiB, on RunPod

**Why.**
- E13 ran locally under a 13 GiB cap (Amendment 18). Three parts ran out of memory:
  - the three `N = 58` controls 6:10:3 d1, d3 and d4;
  - all four E13a draws (degree 5 at (3, 21, 7));
  - the E13c gate build. The local runner put the cap on the build too.
- The owner topped up RunPod on 2026-11-09 and asked for the pod route.
- E13's recorded readings are unchanged. E14 is a new run with its own readings.

**Route.** As in Amendment 17:
- **Pod:** one RunPod CPU pod, `cpu3g`, 64 vCPU, `interruptible: false`.
- **Source:** the same frozen archive (main `a2be6609`, SHA-256 `18b14930…`), fetched by
  presigned URL and hash-checked.
- **Outputs:** tarred to S3 by presigned PUT every 10 minutes and after every draw.
- **Hard cap of USD 100:** the pod reads its own `costPerHr` from the RunPod API and sets
  its deadline to `min(36 h, floor(100 / costPerHr) h)` from start. It deletes itself at
  the end or at the deadline. No second pod is created without a new amendment.
- **Memory:** no `ulimit`.
- **Gates:** G0 and G1 of Amendment 17, unchanged. E14c also runs the gates of
  `e12/run_window.sh`.

**E14b: the three unmeasured E13b controls.**
- **Run:** 6:10:3 draws 1, 3 and 4 (recorded `d_F` = 4 each), from
  `e9/run-r3/out/E_6-10-3.jsonl`, with
  `dreg_linear_guess --engine pe --batch 64 --to 4 --max-cols 600000 --draw D`, 64 threads.
- **Reading:**
  - **"controls confirmed"** if all three refute at degree 4.
  - **"control differs"** if any of them saturates unrefuted at degree 4 or refutes at
    degree 3. The draw is listed.
  - **"not completed"** if any draw does not finish.
- **Combined with E13b:** if E14b reads "controls confirmed", E13b's condition holds over
  E13 and E14 together. All 12 censored draws agree, and all 8 controls refute at their
  recorded `d_F`. That combined statement is reported as **"independent agreement (E13b +
  E14b)"**. E13b's own recorded reading stays "disagreement".

**E14a: the exact `d_F` at (3, 21, 7).** Rules as E13a (Amendment 17):
- **Run:** `--to 5 --max-cols 1000000`, draws 0, 1, 3 and 4 in that order, 40 threads.
- **Per draw:** "`d_F = 5`", "`d_F ≥ 6`" or "unmeasured". The degree-4 entry must equal
  E13b's committed record for the same draw, or the draw is void.
- **Cell:** "`d_F = 5`" if ≥ 3 of 4 draws read 5; "`d_F ≥ 6`" if ≥ 3 of 4 read ≥ 6;
  "split" if ≥ 2 are measured; "not completed" otherwise.

**E14c: (4, 28, 7).** `e12/run_window.sh "$OUT/C" 4:28:7`, 24 threads, concurrent with E14a.
- **Reading:** Amendment 11's E12c rule. "H_ℓ refuted at `m = 4`" if at least 2 completed
  draws are "`d_F = 4` certified"; otherwise "no reading".
- **Consistency check:** draw 0 must equal `e12/interrupted-c-20261028/W_4-28-7.jsonl` on
  every non-timing field.

**Scheduling.** Gates first, then E14b, then E14a and E14c concurrently. A part cut off by
the deadline is "not completed". Nothing is retried.

**Prediction (recorded before any draw).**
- E14b "controls confirmed": 0.85.
- E14a: "`d_F = 5`" 0.55, "`d_F ≥ 6`" 0.3, "not completed" 0.15.
- E14c: "no reading" 0.75.

Nothing from any E14 part has been computed, and no E14 draw has been generated.

## Amendment 20 (2026-11-11 UTC, before any draw below): E14 relaunch (r2) after the zero-balance termination

**Why.**
- The Amendment 19 pod (`oubpztzdbbbo4s`) was terminated by the provider at zero account balance
  about 5 h after start (`e14/score.json`). It passed G0 and G1 and produced no draw. E14b's first
  control, 6:10:3 draw 1, had run for more than 16,400 s at 64 threads without output, against 190 s
  for the `hky_par` reference, and its `.err` stream was empty. E13's local timings put the `pe`
  engine at roughly 20× the `hky_par` time at matched column counts (E_3-24-8: 1,830 s for 186k
  columns at 4 threads against 90–124 s for `hky_par` at 213k columns). Dense elimination scales
  about cubically in the column count, so the registered E14a design (`pe`, 975k columns at degree
  5) had no realistic chance inside the USD 100 cap. This amendment keeps every reading rule and
  changes the engine for E14a, the transport, the order, and adds a scaling gate.
- AWS identity is unavailable in the authoring session (`aws` is not installed; the last check on
  2026-11-11 returned `InvalidClientTokenId`), so the S3 presigned transport of Amendments 17–19
  cannot be minted. The repository `isogeny-labs/crypto` is public, so the pod fetches the frozen
  commit directly from GitHub.
- The owner authorised the spend on 2026-11-11 ("a few hundred dollars of large-memory compute to
  finish E14 and the three missing controls, once credentials are restored").

**Admission (checked by `e14/launch_r2.sh`, recorded in `e14/pod-r2/launch.json`).**
- RunPod account balance ≥ USD 100 + 24 h × (sum of `costPerHr` of every other running pod). At
  authoring time the balance was USD 15.74 with another pod (`icb-k14d`, not this study's) running
  at USD 5.29/h, so the launch is **not admitted** until the account is topped up.
- No other pod whose name starts with `fd-e14` exists.
- `.resume/RESUME.md` filled by the launching session (repository rule). The S3 push needs a
  checkpoint token or AWS identity; absent both, the owner's waiver is recorded in the session log.
- The in-pod script `e14/run_pod_r2.sh` is passed as `SCRIPT_B64`; its sha256 at launch is recorded
  in `launch.json` and must equal the committed file's.

**Route.** One RunPod CPU pod, Secure Cloud, flavor `cpu3g` (fallback `cpu5g`), 64 vCPU,
`interruptible: false`, 150 GB container disk, `runpod/base:1.0.2-ubuntu2404`. Hard cap USD 100:
deadline `min(36 h, floor(100 / costPerHr) h)` from start, read from the pod's own record with the
key passed as `POD_CTL_KEY`; fallback 18 h if the cost cannot be read. The pod deletes itself at the
end or at the deadline. No second pod without a new amendment.

**Source.** Frozen commit `a2be660914e8e485eefd45f2958eb6d18344dadf` (main, 2026-11-07), as in
Amendments 17–19, fetched by sparse partial clone from `https://github.com/isogeny-labs/crypto` over
the E13 path list. Gates:
- `G-src-a`: `git rev-parse HEAD` equals the commit.
- `G-src-b`: sha256 of the path list fetched from the commit equals
  `97b6bbde009ee1e04884041280b1c5dcd9030024d95b940f7e4a12bd9bd28ecd`.
- `G-src-c`: `git ls-tree -r HEAD -- <paths> | sha256sum` equals
  `da6ee5f3f65fa864d7eddb1da48a6bb7d5d286e7072f8607cc1356e480c466ff` (4,501 entries). This is the
  tree identity; it does not depend on gzip. For the record, `git archive --format=tar <commit>
  <paths> | gzip -n` hashed `e9d4924a4cde92cc469a3f663eae25d29d12c0bb56a0cf68dc97482f7d8dd170` on the
  authoring host (32,129,557 bytes); Amendment 17's archive hash `18b14930…` was made with other gzip
  settings and is not re-checked.

**Outputs.** No S3. The pod serves `/workspace/out` over HTTP on port 8080 under a random path token
(`OUT_TOKEN`, 32 hex chars, generated at launch) through RunPod's proxy; `e14/pull_r2.sh` pulls
`out.tgz` every 10 minutes into `e14/pod-r2/pulls/` with its sha256, and the final pull into
`e14/pod-r2/out/`. The pod refreshes `out.tgz` every 10 minutes and after every draw, keeps serving
for 30 minutes after `final_status` is written, then deletes itself; the controller deletes it as
soon as the final pull is verified.

**Gates on the pod (all before any draw; any failure ends the run with that status).**
- `G0`, `G1`: as Amendment 17 (unit tests; `pe` reproduces `S3.d0.b4.jsonl` on the 3-22-8 draw).
- `G2` (new, `hky_par` identity): `hky_par newcells --cells 3:21:7 --seed 20261012 --keep 4 --cmax 4`
  at 64 threads must reproduce `e10/run-r3-B/out/E_3-21-7.jsonl` on every non-timing field (`draw`,
  `x_r`, `v_basis`, `solutions`, `d_f`, `d_f_at_least`, `per_degree`) for draws 0, 1, 3, 4.
- `G3` (new, `pe` scaling pilot): the G1 command at 8, 16 and 64 threads, wall time recorded in
  `g3.json`. `T_pe` = the fastest; `TH_pe` = its thread count, used for every `pe` run below.
  If `T_pe > 1,800 s`, E14b is skipped with reading "not completed (G3: pe too slow)", because the
  cubic extrapolation from 140k to 457k columns (×34) would exceed the cap per draw.

**E14a: the exact `d_F` at (3, 21, 7), engine `hky_par`.** `hky_par newcells --cells 3:21:7 --seed
20261012 --keep 4 --cmax 5`, 64 threads, default `HKY_CHUNK`. Per draw, in the engine's order
(expected 0, 1, 3, 4): the degree-≤4 `per_degree` entries must equal the E10 record for that draw or
the draw is **void**; then "`d_F = 5`" if `d_f = 5`, "`d_F ≥ 6`" if `d_f_at_least = 6`. Cell rule as
Amendment 19: "`d_F = 5`" if ≥ 3 of 4 read 5; "`d_F ≥ 6`" if ≥ 3 read ≥ 6; "split" if ≥ 2 are
measured; "not completed" otherwise. The engine processes all four draws in one invocation; its
per-draw records are read as they appear. Time box: if no draw has finished 12 h after E14a starts,
E14a is killed and reads "not completed".

**E14b: the three unmeasured E13b controls, engine `pe`.** 6:10:3 draws 1, 3, 4 from
`e9/run-r3/out/E_6-10-3.jsonl`, `dreg_linear_guess --engine pe --batch 64 --to 4 --max-cols 600000
--draw D`, `TH_pe` threads, sequential, 6 h per draw (a draw past 6 h is killed and reads "not
completed"). Readings exactly as Amendment 19, including the combined "independent agreement (E13b +
E14b)" statement.

**E14c: (4, 28, 7).** `e12/run_window.sh "$OUT/C" 4:28:7`, 24 threads, only after E14a and E14b, until
the deadline. Reading as Amendment 19.

**Order.** Gates; E14a; E14b; E14c. Nothing runs concurrently with E14a or E14b. A part cut off by
the deadline is "not completed". Nothing is retried.

**Prediction (recorded before any draw).**
- E14a: "`d_F = 5`" 0.50, "`d_F ≥ 6`" 0.25, "split" 0.05, "not completed" 0.20.
- E14b: "controls confirmed" 0.55, "not completed" 0.35, "control differs" 0.10.
- E14c: "no reading" 0.80.
- G2 passes (hky_par at `a2be6609` reproduces E10): 0.90.

Nothing from any E14 r2 part has been computed, and no E14 r2 draw has been generated.

## Amendment 21 (2026-11-12 UTC, before any draw below): owner override of the Amendment 20 admission rule; longer post-finish grace

**Owner decision (2026-11-12, about 02:05 UTC).** The RunPod balance was USD 99.23 with a sibling
pod (`icb-bin-deep2`, not this study's, USD 5.29/h, started 01:48 UTC) running, so Amendment 20's
admission condition (balance ≥ USD 100 + 24 h × other pods) was not met: USD 227 would have been
needed. The main session put three options to the owner (a capped 9 h run; wait for a top-up; the
full run now with the zero-balance risk). The owner chose **"Full run now anyway"**. The consequence
was stated before the choice and is recorded here: at the combined burn of about USD 7.9/h the
account reaches zero after about 12 h unless topped up, which would terminate both pods as it did
E14 r1. Nothing else in Amendment 20 changes: engine order, gates, readings, predictions and the
USD 100 cap stand.

**Changes.**
- `e14/launch_r2.sh` accepts `OWNER_OVERRIDE=1`, which bypasses only the balance condition (the
  "no other `fd-e14` pod" condition stays) and records `owner_override: true`, the balance and the
  other pods' rate in `launch.json`.
- Post-finish grace on the pod rises from 30 minutes to 6 hours (`e14/run_pod_r2.sh`), because the
  controller in a cloud session runs in windows of at most 2 hours and is re-armed by scheduled
  check-ins; the controller still deletes the pod as soon as the final pull verifies, so the grace
  costs nothing when the controller is up. Worst case USD 15.
- Controller cadence: `e14/pull_r2.sh` pulls every 10 minutes while it runs; the main session
  re-arms it with scheduled check-ins about every 100 minutes and records each pull's sha256 in
  `e14/pod-r2/pull.log`. Snapshots are kept in `e14/pod-r2/pulls/` so a termination between pulls
  loses at most 10 minutes of output.
- Resume checkpoint: `.resume/RESUME.md` is filled on the branch; the S3 push is unavailable in this
  session (no checkpoint token, no AWS identity), so the Git copy is the checkpoint, as the owner
  allowed for the 2026-11-11 oversampling run.

Nothing from any E14 r2 part has been computed, and no E14 r2 draw has been generated.

## Amendment 22 (2026-11-12 UTC, before any draw below): pod r2a failed to bootstrap; relaunch r2b with observability

**What happened.** Pod `vvek50lemifzcw` (`fd-e14-r2-20261112T0205Z`, created 02:05:40 UTC) ran for
13 minutes with the container reporting 0 % CPU and nothing listening on port 8080 (proxy 502, then
timeouts), so its in-pod script either stopped early (most likely in `finish`, idle in the
post-finish grace) or never started its HTTP server (if `python3` is absent from
`runpod/base:1.0.2-ubuntu2404`; the sibling pod on the same image installs Python packages through
`apt` as a fallback, which suggests exactly that). Its log could not be read, so the cause is **not
established**. It computed no gate and no draw. It was deleted at 02:21:09 UTC (about USD 0.66).
Its launch record is kept as `e14/pod-r2/launch-r2a.json`.

**Changes for r2b (`e14/run_pod_r2.sh`, sha256 recorded in `launch.json`).**
- `apt-get install python3 openssh-server …` runs **before** the HTTP server starts; the first
  `run.log` lines ("boot: script started", "boot: apt exit=…", "boot: sshd started") are written
  before any network-dependent step, and `pack` runs after the Rust toolchain and after the source
  check, so a failure in any later step leaves a readable log.
- `sshd` is started with RunPod's account `PUBLIC_KEY` (and `EXTRA_PUBKEY` if given; none given,
  the authoring session has no SSH client) and port `22/tcp` is exposed, so the owner can inspect
  the pod from the RunPod console if needed. Root login is key-only.
- Pod name prefix `fd-e14-r2b-`. Nothing else changes: engines, gates, readings, predictions, the
  USD 100 cap and Amendment 21's owner override all stand. No draw of r2a exists, so no draw is
  retried.

## Amendment 23 (2026-11-12 UTC, before any draw below): r2b also failed to bootstrap; cause established; relaunch r2c

**What happened.** Pod r2b (`vlydemuitde7d7`, created 02:21:55 UTC) behaved exactly like r2a: container
up, 0 % CPU, nothing on port 8080 after 11 minutes, no readable log. A diagnostic pod
(`fd-e14-diag-python-proxy`, `ar0ext66azn7u1`, 2 vCPU, USD 0.06/h, created 02:30 UTC) running the
same image with a start command that writes under `/srv` after `mkdir -p` served its status file
through the proxy **17 s after boot** and reported `python3=/usr/bin/python3`. So the image has
Python, the proxy path works, and r2a/r2b's script never ran: their start command was
`echo "$SCRIPT_B64" | base64 -d >/workspace/run.sh && bash /workspace/run.sh`, which fails at the
redirect because these CPU pods mount no volume and `/workspace` does not exist before the script's
own `mkdir -p`. The E14 r1 pod (Amendment 19) had created `/workspace` another way. r2b computed no
gate and no draw; it and the diagnostic pod were deleted at 02:33 UTC (r2b about USD 0.5, the
diagnostic pod under USD 0.01). Records: `e14/pod-r2/launch-r2b.json`.

**Change for r2c (`e14/launch_r2.sh`).** Start command
`mkdir -p /workspace /root && echo "$SCRIPT_B64" | base64 -d >/root/run.sh && exec bash /root/run.sh`;
pod name prefix `fd-e14-r2c-`. The in-pod script (`run_pod_r2.sh`, sha256 `5f49b8ac…`, Amendment 22)
is unchanged. Nothing else changes: engines, gates, readings, predictions, the USD 100 cap and the
Amendment 21 owner override stand. No draw exists, so no draw is retried.

## Amendment 24 (2026-11-12 UTC, before any draw below): r2c failed the same way; early-serving start command and a full trace; relaunch r2d

**What happened.** Pod r2c (`qoulmu5122514o`, created 02:33:06 UTC) showed the r2a/r2b signature
again (container up, 0 % CPU, nothing on 8080 after 5 minutes). The stored `SCRIPT_B64` decodes to
the committed script (sha256 `5f49b8ac…`, 7,981 bytes) and the sibling pod carries a 20 KB variable
the same way, so delivery is not the cause; the `/workspace` theory of Amendment 23 is therefore
**not confirmed** either. The cause is still not established. r2c was deleted at about 02:41 UTC
(about USD 0.35, no gate, no draw). Record: `e14/pod-r2/launch-r2c.json`.

**Changes for r2d.**
- Start command (`e14/launch_r2.sh`): creates the output and serve directories, writes "start command
  running" to `run.log`, starts `python3 -m http.server 8080` itself (Python is in the image, as the
  diagnostic pod showed), decodes the script, logs its sha256, then runs it. The log is therefore
  readable from the first second, whatever the script does.
- In-pod script (`e14/run_pod_r2.sh`): `set -x` with timestamps into `out/trace.log`; `apt` split into
  update, base tools and build tools, each with its exit code logged; `openssh-server` and `sshd`
  dropped (no SSH client in the authoring session; port 22 no longer exposed). Everything after the
  boot block is unchanged. The new script's sha256 is recorded in `launch.json`.
- Pod name prefix `fd-e14-r2d-`. Engines, gates, readings, predictions, the USD 100 cap and the
  Amendment 21 owner override stand. No draw exists, so no draw is retried. If r2d also fails to
  bootstrap, no further pod is launched without the owner's reading of `trace.log`.

## Amendment 25 (2026-11-12 UTC, before any draw below): cause of the r2a–r2d failures established (64 vCPU exceeds the provider maximum); relaunch r2e at 32 vCPU on a memory-optimized flavor

**What happened.** r2d (`9cfghisyn5cliy`) showed the same signature and was deleted at 02:55 UTC
(14 min, about USD 0.6; record `e14/pod-r2/launch-r2d.json`). Diagnostic pods settled the cause:
- D2 (`7u7h5lowmoe4f7`): the known-good simple start command with r2d's **full env block** served
  after 11 s with every variable present, so the env block (including the 10 KB `SCRIPT_B64`) is
  not the cause.
- D5 (`e1j3jly1yk1hf6`): a 2-vCPU pod on `cpu5c` served after about 15 s (EUR-IS-1).
- RunPod's flavor catalogue (GraphQL `cpuFlavors`, read 03:03 UTC): every CPU flavor has
  `maxVcpu = 32` (cpu3c/cpu5c 2 GB per vCPU, cpu3g/cpu5g 4 GB, cpu3m/cpu5m 8 GB; disk 10–15 GB per
  vCPU). r2a–r2d all requested **64 vCPU**; the API accepted the request and echoed
  `vcpuCount: 64`, but their containers never started (uptime counting, 0 % CPU, no listener).
  The E14 r1 pod of Amendment 19 ran with 64 vCPU and 528 GB on 2026-11-09, so the limit is new or
  was not enforced then; it is enforced now.
- D3 and D4 (2-vCPU probes created 02:53–02:55 on cpu3c/cpu3g) had not started after 10 min
  (one container with uptime 0, one with no container), so container start on some hosts is also
  slow; that is a second, independent effect and does not change the conclusion above.
  All probes were deleted at 03:05 UTC (total under USD 0.05).

**Changes for r2e.**
- `e14/launch_r2.sh`: `vcpuCount: 32`, flavors `["cpu5m","cpu3m"]` (8 GB per vCPU, so 256 GB RAM,
  which the E14a degree-5 closure needs: Amendment 13 sized it at 256 GiB), pod name prefix
  `fd-e14-r2e-`. Price about USD 4.16/h (cpu5m) or 3.52/h (cpu3m); the USD 100 cap therefore gives a
  deadline of about 24 h or 28 h instead of 36 h. The cap is unchanged.
- `e14/run_pod_r2.sh` (sha256 `6003ecbdbb92c6ef…`): every 64-thread setting becomes 32 (G1, G2, E14a, and the
  G3 pilot now times 8, 16 and 32 threads; `g3.json` records `t32`). Nothing else changes.
- Engines, gates, readings, predictions and the Amendment 21 owner override stand. No draw exists,
  so no draw is retried. If r2e has not written "start command running" to its served `run.log`
  within 20 minutes of creation, it is left running for a further 20 minutes before any action,
  because slow container starts were also observed.

*Note at launch (03:05:14 UTC):* the provider priced the 32-vCPU memory-optimized pod at USD 2.08/h
(below the catalogue estimate above), so the USD 100 cap gives the full 36 h deadline. Pod
`xkgx6mw9roo3zl`, `fd-e14-r2e-20261112T0305Z`, 32 vCPU, 256 GB; balance at launch USD 91.37; the
sibling pod was at USD 5.35/h.

## Amendment 26 (2026-11-12 UTC, before any draw below): r2e booted correctly and stopped at G-src-b; path list fetched from a commit that contains it; relaunch r2f

**What happened.** r2e (`xkgx6mw9roo3zl`, cpu5m/cpu3m 32 vCPU, 528 GB host, AMD EPYC 9575F) ran the
start command 2 s after creation, finished `apt` and the Rust toolchain by 03:05:33 UTC, and then
stopped at gate `G-src-b`: the path list was fetched from
`raw.githubusercontent.com/.../a2be6609.../e13/source_paths.txt`, but `e13/source_paths.txt` was
added to the repository after the frozen commit `a2be6609` (it is absent there; the raw URL returns
404), so the hash check refused the 404 page, as it should. The controller pulled the final output
(`e14/pod-r2/out-r2e/`: `run.log`, `trace.log`, `apt.log`, `host.txt`) and deleted the pod at
03:05:49 UTC (under USD 0.05). This confirms Amendment 25's cause: with 32 vCPU the pod starts at once.
No gate beyond G-src-a was reached; no draw exists.

**Change for r2f.** The path list is fetched from commit
`43438f5ab6eb6d1844b8d0465c933ad079099326` (main at the start of this session, where it exists with
sha256 `97b6bbde…`); `G-src-b` still pins its content by that hash, and `G-src-a`/`G-src-c` still
pin the source tree at `a2be6609`. Env `PATHS_COMMIT` carries the commit; the in-pod script
(sha256 `a6c9d90cec93b62f…`) defaults to the same value. Pod name prefix `fd-e14-r2f-`. Nothing else changes.

## Amendment 27 (2026-11-12 UTC, before any draw below): G2's comparison key excluded timings as Amendment 20 states; relaunch r2g

**What happened.** r2f (`vx5qv5kjfk21k0`) passed G-src-a/b/c (03:08:04 UTC), G0 and G1 (G1: 597 s at
32 threads for the 140k-column cell, against 511–586 s at 4 threads in E13, so `pe` gains nothing
from threads) and ran G2 at 03:26:33 UTC. G2 reported "differs" because the in-pod `jq` key compared
`per_degree` whole, and each degree entry carries a `secs` wall-time field (E10: 136.7, 140.5,
134.2, 139.0 s; r2f: 94.2, 90.0, 84.0, 88.0 s). Every non-timing field is identical on all four draws:
`draw`, `x_r`, `v_basis`, `solutions`, `d_f`, `d_f_at_least`, and `dim`, `of`, `rounds` at
degrees 2, 3 and 4 (`e14/pod-r2/out-r2f/g2.diff`, `g2.jsonl`). Amendment 20 defines G2 on
"every non-timing field"; the script's key was wrong, not the engine. The output was pulled, the pod
deleted at about 03:36 UTC (about USD 1), and the records archived under `e14/pod-r2/out-r2f/`.
No draw of E14a, E14b or E14c exists.

**Change for r2g.** The G2 key becomes
`{draw,x_r,v_basis,solutions,d_f,d_f_at_least,per_degree:(.per_degree|map_values(del(.secs)))}`
(script sha256 `e6e6d62754a76a66…`). Applied to E10's committed records and to r2f's `g2.jsonl` on the
authoring host, both sides hash identically. Pod name prefix `fd-e14-r2g-`. Nothing else changes.

## Amendment 28 (2026-11-12 UTC, before any draw below): r2g stalled in the build on a slow host; CPU probe and build time limit; relaunch r2h

**What happened.** r2g (`q0pylpkso8ofd1`, cpu5m, 32 vCPU, EPYC 9575F, 528 GB) verified its source
at 03:41:13 UTC and then spent more than 55 minutes on the final crate of the build
(`Compiling crypto v0.1.0`), with the container reporting about 200 % CPU, against 3.5 minutes for
the whole build on r2f (same host model, same 231 crates). It shared no machine with the sibling pods.
The container was effectively getting about two of its 32 cores; whether by a quota or by host
contention is not established. At that rate E14a would also run about 15 times slower than planned,
so the pod was pulled (`e14/pod-r2/out-r2g/`, key redacted) and deleted at 04:39:46 UTC
(about USD 2.2). No gate beyond the source checks ran; no draw exists.

**Change for r2h (`e14/run_pod_r2.sh`, sha256 `690f657cf36fe54b…`).** Before the build the script records
`/sys/fs/cgroup/cpu.max` and `/proc/pressure/cpu` in `host.txt`, runs a 32-way busy-loop probe and
logs the effective parallelism; below 12 of 32 the run ends with `cpu_probe_low_<value>` before any
build, so a bad host costs minutes, not an hour. The build is bounded at 25 minutes
(`build_failed_or_timeout`). Pod name prefix `fd-e14-r2h-`. Nothing else changes.
