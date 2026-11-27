# Protocol: arm D2S — 2LP Weil descent + BFS forest + sparse LA, against matched rho

Study `b2lsp-20261127/v1`. Seed policy `isogeny-labs/experiment-seeds/v1`.
Status `[pcc:unverified]`, L0, one shared 4-vCPU / 15 GB cloud container.
**Single target only** (owner instruction): every solve is one cold solve of
one public `Q`; nothing is amortised across targets.

Builds on crypto PR #5291 (`research/binary-descent-2lp-20261127/`, arm D2,
study `b2lp-20261127/v1`, head `73bf5f4e`) and PR #5290
(`research/binary-descent-sparse-la-20261127/`, arm D-S, study
`bdsl-20261127/v1`, head `92f096c1`), both stacked on #5261 (arm D) and #5253.
This branch is based on #5291's head; #5290's study directory is copied
unchanged from its branch head (`git checkout 92f096c1 -- research/binary-descent-sparse-la-20261127/`),
and its `sparse_la.rs` is included by `#[path]`, unedited.

## What D2S is (fixed before the pilot)

`harness/src/main.rs` (`run_d2s`) and `harness/src/bfs.rs`. Reused unchanged
by `#[path]`: #5253's field, curves, factor base, A1 relation-target stream,
group-law resolution, echelon (cross-check only) and rho; #5261's
`descent.rs`; #5291's `graph.rs` (`Comb`, `inv_mod`; `LpGraph` only as an
uncounted shadow); #5290's `sparse_la.rs`.

1. **Relation phase = #5291's D2**: same oracle, same stream, small base =
   columns `1..fs`, large primes = the other subfield points; rows with 0 / 1
   / 2 large primes kept (full row / edge to SV / edge), 3 discarded.
2. **Relations are stored**, with a union-find that counts cycle-closing
   edges: `Y_ub = full rows + cycle-closing edges`, an upper bound on the
   yields (exact in SV's component, at most one too many per other cyclic
   component).
3. **Stopping rule = #5290's**, on `Y_ub` and `fs` in place of the rows and
   the column count: first checkpoint at `Y_ub ≥ ⌈0.70·fs⌉`, then every
   `⌈fs/256⌉` more. At each checkpoint:
   - **BFS forest** rebuilt from scratch: BFS from SV in SV's component, from
     the union-find root in every other component with a cycle; every vertex
     gets a shortest path to its root, so a non-tree edge `(u, w)` closes a
     cycle of length ≤ depth(u) + depth(w) + 1.
   - Each non-tree edge's relation is reduced **deepest column first** along
     BFS parent edges (each vertex eliminated once; signed-graph grounding as
     #5291: the first frustrated cycle of a component without SV grounds it).
     Yields = full rows + these combinations = `E − rank` (the same count as
     #5291's `LpGraph`, checked by an uncounted shadow at every checkpoint).
   - #5290's 2-core excess on the yields; if ≥ 1, #5290's SGE + Wiedemann
     (kmax 64, 4 attempts, |u| = 16, excess ≤ 2) on the yields over `fs + 1`
     columns. `d` found → stop; otherwise collection goes on and the next LA
     call is counted again.
4. **Counting**: all graph work in phase `graph` — classification,
   union-find, BFS adjacency and reduction steps one `h` each; mod-ℓ work by
   #5290's convention (product `z` unless an operand is ±1, addition `za`,
   inversion `2·bits(ℓ)` `z`, cached per build). The ±1 products and the
   LA's structural bookkeeping are counted and reported as sensitivities.
5. **D2Sall** (every class small, `fs = F − 1`) on #5290's LA stream
   `la-ds|n|ci|ti` must reproduce D-S bitwise (d, targets, relations, LA
   counters and report). Pre-freeze engineering check on #5290's own
   self-test instances (n = 27, 33; ti = 1..3): 6/6 identical in d, targets,
   relations, `la_report`, LA calls, LA phase counters and structural count.

## Part A — pilot rule (frozen before any pilot is run)

The pilot chooses only the small-base rule of D2S (exponent β and constant
θ in `fs = min(F − 1, ⌈θ·F^β⌉)`) and the rho lanes of rung n = 60. Held-out
instances only.

- **Small-base pilot.** Curves `d2spilot-curve|n|ci`, targets
  `d2spilot-target|n|ci|ti`, relation stream `d2spilot-stream|n|ci|ti`, LA
  stream `d2spilot-la|n|ci|ti|α`; n ∈ {42, 48, 54}, ci, ti ∈ {0, 1}
  (4 cases per rung); grid `fs = ⌈α·(F − 1)⌉`,
  α ∈ {0.045, 0.067, 0.1, 0.15, 0.22, 0.33, 0.5, 0.75, 1.0}.
  Rule (`b2lsp_harness pilot-summary`):
  1. per rung, `S(α) = ln gmean_cases Meq(s=1,u=1)`; `k*` = argmin on the
     grid (ties to the smaller α);
  2. if `k*` is interior, α̂ = vertex of the parabola through
     `(ln α, S)` at `k*−1, k*, k*+1`, clamped to that bracket; at an edge,
     α̂ = α_{k*};
  3. `fs*(n) = α̂·(F − 1)`; OLS of `ln fs*` on `ln F` over the three rungs
     gives β̂ and θ̂; β̂ is clamped to [0.5, 1.0] and, if clamped, θ̂ is
     refitted as the geometric mean of `fs*/F^β`;
  4. arm **D2S** uses (β̂, θ̂); arm **D2S23** uses β = 2/3 (the textbook k = 3
     2LP balance) with θ = geometric mean over the rungs of `fs*/F^{2/3}`.
- **Rho lanes.** n ≤ 54: unchanged references (part B). n = 57: W = 256,
  the argmin of #5291's complete held-out cells at n = 57
  (`pilot_rho_ext2.jsonl`: 256 → 1.718·10⁹, 128 → 1.822·10⁹, 64 → 2.045·10⁹
  Meq; 16 records each). n = 60: new held-out pilot `pilot-curve|60|ci`,
  `pilot-target|60|ci|ti`, `pilot-rho|60|ci|ti|W`, ci ∈ {0, 1}, 8 targets per
  curve, W ∈ {128, 256, 512}; W* = argmin of mean Meq(s=1,u=1).

Pilot outputs: `data/pilot_alpha.jsonl`, `data/pilot_rho_60.jsonl`. They are
not evaluation data. Part B (arms, rungs, references, predictions, analysis)
is written and frozen after the pilot and before any evaluation data.

### Amendment A1 (recorded after the first pilot attempt, before its rerun and before any evaluation data)

- **Defect seen in the first pilot attempt** (`data/pilot_alpha_v0_defect.jsonl`,
  70 records, n = 42 complete and n = 48 34/36; stopped by hand during n = 48/54):
  on one held-out case (n = 42, ci = 1, ti = 1) at α = 0.045 and 0.067 the
  solve needed 230 and 67 LA calls. Diagnosis on the same held-out case: one
  cycle combination cancels completely (an empty row `0 = 0`, i.e. the same
  relation stored twice); the empty row is a permanent kernel vector of the
  LA input, #5290's excess pruning never removes it (it drops the heaviest
  rows), so every Wiedemann attempt returned it (`den = 0`).
- **Fix:** empty cycle combinations are dropped before the LA (one `h` each,
  counted as `graph.empty_dropped`); the yield-count identity against #5291's
  `LpGraph` becomes `yields + empty_dropped = LpGraph yields`. Full rows are
  untouched, so D2Sall ≡ D-S is unaffected (re-checked: 6/6 identical,
  `data/selftest_prefreeze_A1.log`). `sparse_la.rs` is unchanged.
- The pilot is **rerun from scratch** with the fixed binary under the same
  part A rule; only the rerun (`data/pilot_alpha.jsonl`) chooses β, θ. The
  defective attempt is kept as a record and chooses nothing. Observed in it
  (not used): at n = 42 and 48 the cost falls monotonically with α, with
  α = 1 (the full base, i.e. D-S) cheapest.

### Pilot outcome (recorded before part B was frozen)

- **Small-base pilot** (`data/pilot_alpha.jsonl`, 108 records, all correct;
  rule output `data/pilot_alpha_summary.txt`):
  - n = 42: cost falls monotonically with α; argmin α = 1 (edge) → fs* = F − 1.
  - n = 48: monotone again; argmin α = 1 (edge) → fs* = F − 1.
  - n = 54: argmin α = 0.15 (1.578·10¹⁰ Meq; α = 1: 1.665·10¹⁰; 0.22:
    1.657·10¹⁰; 0.1: 1.923·10¹⁰); parabola α̂ = 0.1675, fs* = 22,022.
    One case at α = 0.75 needed 4 LA calls (Wiedemann found only useless
    dependencies three times; inherited #5290 behaviour, no empty rows) and
    costs 1.9·10¹¹; it does not touch the argmin or its neighbours.
  - OLS: raw β = 0.359 (outside [0.5, 1], because two rungs sit at the edge
    α = 1) → **D2S: β = 0.5, θ = 99.589**; **D2S23: β = 2/3, θ = 17.616**.
- **Rho lanes at n = 60** (`data/pilot_rho_60_complete.jsonl`, 48 records, all
  correct; `data/pilot_rho_60_summary.txt`): W = 128: 4.150·10⁹, **W = 256:
  3.731·10⁹**, W = 512: 3.883·10⁹ → **W = 256**.
- **Interruption.** A container restart killed the rho pilot at 43 of 48
  records (cell W = 512, ci = 1 had 3 of 8) and both prediction simulations
  before n = 60. The W = 512 cells were rerun from the same streams
  (`data/pilot_rho_60_resume.jsonl`): the 11 overlapping records are identical
  in every field but wall time, and the 5 missing ones complete the cell.
  The simulations were resumed for n = 60 only (deterministic per (n, rep)).
- Binaries: pilot and simulations ran `b2lsp_harness` sha256 `90a70114…`
  (amendment A1). The run binary adds only the analysis guard and is hashed
  in `SEEDS.sha256`; `run_d2s`, `bfs.rs` and every `#[path]` module are
  unchanged between the two.

## Part B — evaluation protocol (frozen before any evaluation data)

### Question

On binary composite `F_{2^n}`, n = 3n′, with #5261's oracle unchanged: does
combining #5291's 2LP relation phase with a BFS (shortest-path) forest and
#5290's sparse LA lower the cold single-target cost against D-S (#5290), D2
(#5291) and matched negation-map rho, and does its whole-solve slope fall below
rho's?

### Arms and references

- **D2S** (primary): `fs = min(F − 1, ⌈99.589·F^{0.5}⌉)`. Saturated (all
  classes small, i.e. the D-S relation set) at n ≤ 42; fs/F ≈ 0.78, 0.55,
  0.39, 0.28, 0.20, 0.14 at n = 45, 48, 51, 54, 57, 60.
- **D2S23** (secondary): `fs = min(F − 1, ⌈17.616·F^{2/3}⌉)`; fs/F ≈ 0.88,
  0.70, 0.55, 0.44, 0.35, 0.28, 0.22 at n = 42..60.
- **D2Sall** (control, n ≤ 39): every class small on #5290's LA stream; must
  equal D-S bitwise.
- **References, reused unchanged**: D-S (#5290 `runs_ds.jsonl`, arm DS), D2
  (#5291 `runs_d2.jsonl`, arm D2), rho as listed in `protocol.json`
  (`references`): n = 51 W = 128 (the wider-grid pilot argmin of #5291; 4
  cases from #5291, the other 8 of #5290's 12 cases run here), n = 54 W = 64
  (#5290, 4 cases), n = 57 W = 256, n = 60 W = 256 (new). #5290's n = 51
  W = 64 rho is a sensitivity (`D2S_over_rho_W64`).

### Rungs and cases

n = 18..48: #5261's 116 cases (4 curves × 3 targets; 2 × 2 at n = 33, 48);
n = 51: 4 × 3 (= #5290's cases); n = 54, 57, 60: 2 × 2. D2S and D2S23 at every
rung; D2Sall at n ≤ 39. One process per rung, 3 threads, `prlimit --as`
12 GiB, wall budgets in `protocol.json`; a killed process is a resource-limit
record and the run goes on. D-S and D2 are not run at n ≥ 57 (no reference
there: D-S would need its F² LA at F ≈ 2.6·10⁵, D2 its echelon).

### Checks (frozen)

As `protocol.json` → `checks`: every log (`[d]G = Q` in-solve, `d = d_true`);
D2S/D2S23 d = D-S d and = D2 d on shared cases; every row of every LA call
group-law checked; the yield-count identity against #5291's `LpGraph` at every
checkpoint; n ≤ 30: the dependency traced to the input rows and the echelon on
the same rows give the same d; D2Sall ≡ D-S (n ≤ 39); replay n = 30, ci = 0,
ti = 0, arm D2S, compared bitwise but wall times.

### Cost unit and band

#5253's Meq unchanged; mod-ℓ work as #5290 (u = 1 primary). Band per §5a:
non-field units at u ∈ {0, 0.1, 1}, squarings s ∈ {1, 0.25}; also the
±1-products-charged and ±1-plus-LA-bookkeeping-charged readings. Orders
between arms only where they hold across the band.

### Analysis (frozen; `b2lsp_harness analyze`)

Per case D2S/rho, D2S23/rho, DS/rho, D2/rho, D2S/DS, D2S/D2, D2S23/DS,
D2S23/D2S, D2Sall/DS (rho = mean of 16 runs); per rung geometric mean with a
95% hierarchical bootstrap (curves, targets, rho seeds), B = 10000 primary,
2000 band, stream `boot|b2lsp|main`; rho A/A (seeds 0–7 vs 8–15); phase split
(relation / graph / LA) and relation phase over rho; census (fs/F,
targets/q′, LA rows, yield weight, cycle length mean/max, BFS depth, LA/fs²,
LA calls); exponent fits (OLS of log₂ cost on log₂ ℓ, bootstrap within rung)
for rho, DS, D2, D2S, D2S23 on all rungs, n ≥ 36, n ≥ 48, and n = 36..54,
plus the slope difference D2S − rho (n ≥ 36, n ≥ 48) with a bootstrap CI;
crossover projection (a model, labelled so): targets/F = K(c + α)/(1 −
(1 − α)³) fitted on unsaturated D2S/D2S23 rows, relation + graph per target
linear in n′, LA either the fitted `2^{l₀}·fs^γ` or `κ·fs²`, rho
`S_top·√(2^{3n′}/h̄)`; first n′ ≤ 64 with ratio ≤ 1 for the frozen rules and
a base re-optimised per n′; straight-line extrapolation of measured slopes.
The gap is reported as the remaining distance, never as pass/fail.

### Predictions (written before evaluation data)

From the abstract simulation (`b2lsp_harness predict`, Poisson(1/6)
decompositions of uniform classes, the same `BfsLp`, stopping rule and
sparse LA; `data/predict.jsonl`, `data/predict23.jsonl`, table
`data/predict_summary.md`) priced with #5291's measured relation Meq per
target and rho S, and from the held-out pilot:

| n | D2S fs/F | pred D2S/rho | pred D2S/DS | pred weight | pred cycle len mean/max |
|---|---|---|---|---|---|
| ≤ 42 | 1 | = DS/rho (±2%) | 1.00 ± 0.02 | 3.0 | — |
| 45 | 0.78 | 21 | 1.00–1.15 | 3.5 | 2.4/6 |
| 48 | 0.55 | 23 | 1.05–1.25 | 4.3 | 2.9/8 |
| 51 | 0.39 | 29 | 1.1–1.5 | 5.1 | 3.6/11 |
| 54 | 0.28 | 29 | 0.9–1.25 | 6.0 | 4.4/15 |
| 57 | 0.20 | 32 | — | 7.1 | 5.4/17 |
| 60 | 0.14 | 32 | — | 8.3 | 6.5/22 |

D2S23 (simulation): 27, 22, 23, 29, 31, 36, 41 at n = 42..60.

- **H1 (exactness).** Every D2S, D2S23, D2Sall log correct; 0 group-law
  failures; 0 yield-identity mismatches; all track and echelon cross-checks
  pass; D2Sall ≡ D-S on every case n ≤ 39; replay identical.
- **H2 (row weight).** BFS keeps combined rows light: D2S mean yield weight
  ≤ 9 and mean cycle length ≤ 7 at every rung n ≤ 60, maximum cycle length
  ≤ 30 (D2 at n = 54: weight 81, paths 360–650).
- **H3 (LA).** LA/fs² for D2S rises with the row weight: 0.7–1.5 where
  saturated, 2–20 at n = 48..57 (simulation 2.9 → 14; the pilot's real rows
  are heavier at small α).
- **H4 (gap).** No parity anywhere: D2S/rho between 15 and 45 at every rung
  n ≥ 36. D2S/DS within the table's ranges; no rung n ≤ 54 where D2S beats D-S
  by more than 15% (u = 1).
- **H5 (D2).** D2S/D2 ≤ 0.35 at n = 48 and ≤ 0.2 at n = 51, 54 (the sparse LA
  removes D2's echelon cost).
- **H6 (exponent).** D2S's whole-solve slope on n ≥ 36 is 0.45–0.52
  (simulation 0.497; D2S23 0.509); its difference from rho's slope has a CI that includes 0
  — i.e. the slope does **not** measurably fall below rho's on these rungs.
- **H7 (crossover, model).** No frozen-rule crossover for n ≤ 90; any
  re-balanced crossover beyond n ≈ 100.

**Falsifier.** Parity at a rung if the upper 95% bound of D2S/rho or
D2S23/rho is ≤ 1 there (then the rho A/A floor and full verification are
required before it is stated). H2–H7 are falsified by their stated ranges.

### Not tested

Multi-target amortisation (excluded by instruction); block Wiedemann/Lanczos;
SGE tuned for heavy rows; a depth cap on cycles (BFS only); D-S and D2 at
n ≥ 57; isolab/L2; Koblitz rungs; GHS.
