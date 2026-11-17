# Curve25519-family index calculus, round 3 of proposals: below the C = 2 table-plus-probe model (Idea Generator, 2026-11-04)

Status: **proposals only**. Nothing here is a measurement, a result or a status change. A number is one of:
quoted from a committed file with its path and tagged `[pcc:unverified]` (every autolab outcome is a
`statement` claim with standing `unverified`); *derived* (arithmetic in this file on quoted numbers); or
*predicted* (to be frozen by the Coordinator before any run). No code, ledger, result or status file was edited,
no run was made, and I had no execution tool (read, search, web only).

## 0. Bottom line

1. **I found no curve-specific lever.** Montgomery/x-only arithmetic, the quadratic twist, the cofactor-8
   torsion, the prime-field Frobenius and F_p-rational endomorphisms each fall to the same obstruction: they act
   on the cofactor cosets or as scalars of norm about p on the r-subgroup, so on that subgroup only +/-1
   survives (section 6; the earlier records are `research/curve25519_ic_baseline_20261102/STRUCTURE.md` 2(b), 2(c)
   and `research/curve25519_torsion_leads_20261103/LEADS.md` T1-T8). This is a derived argument, not a proved
   obstruction.
2. **What the generic model does ignore is not the curve but the pair count and the order of work.** The model
   `T + (C+1) r/(2T)` uses only table-by-probe pairs, a fixed table, a fixed probe count, three probe hits, and
   one set-up cost per solve. Four proposals attack those, each with an exact count and a cheap test:
   - **P1** `stride-generator` probes on the wsum incumbent: removes the 16-jump walk set-up. Config only. It
     reaches the model, it does not go below it.
   - **P2** a walk-built table whose key collisions are target-free rows of full rank: three probe hits become two.
     Floor 2.13 (fixed table) or 2.05 (grow to the first row) against 2.449.
   - **P3** interleaved growth of table and probes in one hash map: floor 2.350 against 2.449.
   - **P4** multi-target amortisation of the two seeds, on its own gauge against pooled rho.
3. **Ranking by expected headline-S reduction times probability: P1 > P2 > P3 > P4** (the last is a different
   gauge, section 5). P2 and P3 are the ones that go below the model. **Test first: P1** (no code, one panel; it
   settles the standing set-up contradiction and re-bases every other prediction at k = 20 and 24); build P2's
   Stage 0 preflight in parallel.
4. Honest reductions to known generic bounds are marked per proposal and collected in section 7. None of the four
   gets within 1.5x of matched rho; the all-pairs floor `Gamma(C+3/2)/Gamma(C+1)` = 1.66 at C = 2 stays below
   everything here (`ideas/SYNTHESIS.md` section 0, R1), and a table-probe S below about 1.5 is an accounting
   error first.

## 1. The measured state, restated, and where the excess sits

Ledger after round 26 (`round-026/README.md`, ledger table) [pcc:unverified, `PCC-20261104-f9ed8c`]: incumbents
`wsum2-b15 / b30 / b60 / b120` (`wsum:coef=B,free=1`, `prime-projected-abscissa:size=2`) at S(w=0) =
4.860 / 3.318 / 2.576 / 2.519 for k = 20 / 24 / 28 / 32; S(w=1) = 7.092 / 5.863 / 4.746 / 4.915; distance to
rho 3.2x / 2.7x / 2.8x / 2.7x. Round 26 found no neighbour of the k = 28 or 32 incumbents better: `wsum2-b50-n2`
regressed (1.066 [0.990, 1.148], band `inc < sel`), and `wsum2-b100-n2` at k = 32 was 2.682 against 2.581 in
train [pcc:unverified]. So B is tuned.

Note on the B coefficient: the four incumbents have B / r^(1/4) = 0.788 at every size (15, 30, 60, 120 over
19.03, 38.06, 76.1, 152.2; derived from `sqrt(r)` = 362.2, 1448.2, 5792.9, 23170.3). The 0.47 r^(1/4) in the
brief is the same box expressed in p^(1/4) (0.788 / 8^(1/4) = 0.468). The box has T = 2B^2 = 1.24 sqrt(r)
entries against the model's T* = sqrt(3r/2) = 1.225 sqrt(r).

**Model.** C = 2 seeds, K = C + 1 = 3 unknown logs (d, s1, s2), one box entry per add, hit rate 2T/r per probe:
`E[GAE] = T + 3 r/(2T)`, minimised at T* = sqrt(3r/2), S = sqrt(6) = 2.449.

**Excess over the model** (measured S minus 2.449, times sqrt(r); derived):

| k | S measured [pcc:unverified] | excess in S | excess in GAE | 1-sigma sampling noise on the excess (derived, Gamma(3) probe count, 480 holdout runs) | R3's 16-jump set-up estimate (SYNTHESIS R3, derived, not re-checked) |
|--:|--:|--:|--:|--:|--:|
| 20 | 4.860 | 2.411 | 873 | about 12 | 832 |
| 24 | 3.318 | 0.869 | 1,259 | about 46 | 1,024 |
| 28 | 2.576 | 0.127 | 736 | about 190 | 1,216 |
| 32 | 2.519 | 0.070 | 1,622 | about 740 | 1,408 |

The excess is 70% / 38% / 5% / 3% of the incumbent's S at k = 20 / 24 / 28 / 32. At k = 28 and 32 the arm is
already on the model (k = 32 within noise), so **no set-up change can help there; only a change of pair
structure (P2, P3) can**. At k = 20 and 24 the excess is the larger term, and its source is the open item
that P1 decides (R3 says it is the walk's 16 two-scalar jumps; round 12 measured `ksum4s` (the `stride` source)
1.08x worse than the walk on `ksum4-n12`, 7.413 vs 6.857 in train [pcc:unverified, `PCC-20261104-4afdff`], which
R3 flags as a contradiction).

**Source facts used below** (read from source, not from memory):
- Default probe source is `TargetSource::Walk`: 16 jumps `[a_j]G + [b_j]Q`, each two scalar multiplications and
  one addition, drawn per solve (`src/cryptanalysis/ic_boundary.rs` `draw_jumps`, line 8805; set-up at lines
  9308-9320).
- `TargetSource::StrideGenerator` (spelled `targets=stride-generator` by `Targets::parse`,
  `ic_framework/stages.rs:771`): `R_0 = Q`, then `R_{k+1} = R_k + G`; "no draw, no scalar multiplication", one
  addition per probe, unguarded, stops at k = r (`ic_boundary.rs` lines 9289-9292 and 9539-9563). No spec in
  `families.json` uses it (only `targets: stride` on the ksum `s` families, lines 96 and 107).
- `Relation::Row { columns, a, b }` already carries a row with explicit `(a, b)`; the wsum `free=1` path emits
  target-free rows as `Row { a: 0, b: 0 }` (`plugins.rs` 2511-2529, `WSumTable::free_rows`, `ic_boundary.rs`
  7659). `TargetPinned` completion is a plain elimination test, `matrix.pinned(d_col)` (`ic_boundary.rs` 9675).
- The box table's free rows have rank 0 in more than 96% of bases because colliding pairs arrive as one burst
  of order T/4 that repeats one row (`ideas/SYNTHESIS.md` section 1.1, derived; round 15 saw the same on ksum
  [pcc:unverified, `PCC-20261104-836de4`]). P2 removes that cause.

## 2. P1 (rank 1): `stride-generator` probes on the wsum incumbent

```yaml
id: P1-wsum-stride-generator
title: "Run the wsum incumbents on the b = 1 generator progression Q + kG, which has no walk set-up"
track: generic decomposition (table family), C = 2 reported
obstruction_attacked: >
  The excess over the model at k = 20 and 24 (873 and 1,259 GAE, 70% and 38% of S), if it is the walk's
  16-jump set-up. Not a model-beating idea: it moves the arm onto the model. Listed because it is the largest
  measured lever at the two small sizes and because every other prediction at k = 20, 24 is additive on it.
claim: >
  With oracle `wsum:coef=B,free=1` unchanged and `targets=stride-generator`, paired sel / inc at w = 0 is at
  most 0.75 at k = 20 and at most 0.85 at k = 24 (95% upper bound below 0.90), and S(w=0) lands within 7% of
  2.449 at k = 24, 28, 32.
mechanism: >
  The progression Q + kG costs zero set-up (R_0 = Q) and one add per probe. A probe's hit probability is exactly
  2T/r by linearity over the random (d, s), whatever the correlation between probes; only the variance can
  change. A row (a = k, b = 1) pins d by elimination as soon as three independent rows exist (columns s1, s2, d;
  the 3x3 coefficient block is generically invertible), so the doc comment "pins the logarithm only beside a
  base column whose logarithm is known" (`ic_boundary.rs` 8666-8669) concerns tables with many columns, not
  this one (checked by reading `matrix.pinned`, not by running; the Stage 0 below checks it). The guard is
  dropped (the progression repeats only after r steps), which also removes one hash insert per probe.
prediction:
  S_w0:  {k20: "2.60 [2.40, 3.00]", k24: "2.55 [2.40, 2.80]", k28: "2.44 [2.30, 2.60]", k32: "2.50 [2.40, 2.65]"}
         # incumbent (round 26 ledger, [pcc:unverified]): 4.860, 3.318, 2.576, 2.519
  ratio_sel_over_inc_w0: {k20: "0.53 [0.45, 0.75]", k24: "0.77 [0.72, 0.85]", k28: "0.95 [0.90, 1.00]", k32: "0.99 [0.96, 1.02]"}
  S_w01_w1: >
    The guard (one counted hash insert per probe, about 1.2 sqrt(r) probes) disappears: at w = 1,
    k = 32 4.915 -> about 3.7, k = 28 4.746 -> about 3.6 (derived: minus 1.22 sqrt(r)/sqrt(r)); at w = 0.1 minus 0.12.
  k28_k32_note: >
    R3's 16-jump estimate (1,216 at k = 28) exceeds the measured excess (736 +/- 190) and would put S below the
    model; that is a signal, not a prediction. The set-up hypothesis is therefore predicted to explain the excess
    fully at k = 20, 24 and the model to bind at 28, 32.
discriminating_test: >
  Stage 0 (exact, from existing session records via the artifact store): read the setup-phase GAE
  (`walk_jumps`, `walk_pool_ops`, scalar-multiplication counts) of one k = 20 and one k = 24 incumbent holdout.
  Predicted: set-up plus pool is at least 80% of the 873 / 1,259 GAE excess. Stage 1: one ecbench panel at
  k = 20 and 24 (and 28, 32 as no-effect checks): {inc, inc with targets=stride-generator, rho.negation, inc-aa},
  32 train targets then the frozen holdout (480 runs) with a fresh target_seed.
falsification: >
  Stage 0: set-up plus pool below 50% of the excess (the hypothesis is dead; the excess stays an open item and
  P2, P3 numbers at k = 20, 24 keep their additive excess). Stage 1: paired sel / inc interval contains 1 at
  k = 24, or point estimate above 0.90 at k = 24 or above 0.80 at k = 20; or trials per relation more than 1.25x
  3r/(2T) / 3 (probes correlated: the round 12 failure mode); or any unverified solve. A measured S below 2.30
  at k >= 28 is an accounting error or a positive correlation to explain, not a result.
cost_band: >
  Arms: inc, inc + stride-generator, rho.negation, inc-aa, each at w = 0, 0.1, 1 through `ecbench band --ranks`.
  Counted: table adds, probe adds, row-confirm adds, the internal [d]G = Q check. Unpriced: table inserts, probe
  lookups, row operations. The stride arm has no guard probes; the saving is real unpriced work, not relabelling,
  and is reported separately from the counted saving (the w = 0 ratio is the counted one).
controls: [inc-aa, rho.negation, k = 28 and 32 no-effect arms (ratio about 1), ksum4-n12 with stride-generator
  as the round 12 replication]
risks:
  - "Round 12 (stride on ksum) was worse than the walk; the cause is unexplained. If it is progression correlation with structured tables, wsum's box may or may not share it."
  - "If the excess is something else (rows beyond K = 3, base scan and projection, internal verification), the ratio stays near 1 and the open item remains."
  - "The progression restarts only at k = r; at k = 20 with M about 1.8 sqrt(r) probes the sequence is far from r, fine."
closest_baseline: >
  In repo: round 12 (`ksum4s-n12`, stride, worse) and `planner.rs:302` (Koblitz stride-renew 0.40-0.75 of the
  walk at log2 r <= 21 [unverified assertion, no claim id read]). Published: none; a table method needs only
  distinct probes with known (a, b).
code_needed: {files: ["research/curve25519_autolab_20261104/families.json (new arm template: oracle wsum:coef=B,free=1 plus \"targets\": \"stride-generator\")"], ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=2,scan_cap=32, oracle=wsum:coef=30,free=1, targets=stride-generator"}
lean_obligations: none (no mathematical claim; every solve is checked by ecbench against its target)
parity_statement: "Reaches the model; 2.7x -> about 2.1x of the matched rho at k = 24 if the set-up hypothesis holds (matched rho 1.22, derived 2.55 / 1.22). No change at k >= 28."
```

## 3. P2 (rank 2): a walk-built table with target-free full-rank rows (three probe hits become two)

```yaml
id: P2-sidon-walk-table
title: "Build the C = 2 table as a random-jump walk over the seeds so key collisions between entries are independent homogeneous rows"
track: generic decomposition (table family), C = 2 reported; class if it works: engineering below the model
obstruction_attacked: >
  The "+1" and the third hit in K = C + 1 = 3. The box table cannot use its own collisions because they come in
  one burst (rank 0 in more than 96% of bases; SYNTHESIS 1.1). A table whose entries have pairwise-distinct
  difference vectors gets T^2/r independent collisions, and one of them is enough: a homogeneous row
  u s1 + v s2 = 0 fixes the ratio of the seeds, after which two probe hits pin (d, s1).
claim: >
  A table of T = x sqrt(r) entries E_t = E_{t-1} + J_{h_t}, one add each, h_t drawn independently of the point from
  16 jumps J_j = [alpha_j] P1 + [beta_j] P2 with alpha_j, beta_j uniform in [1, w], w about 1.2 T^(1/2), has
  a verified target-free row with probability 1 - exp(-x^2) (mean x^2 rows). With the row, two probe hits suffice.
  Expected cost E[GAE]/sqrt(r) = x + (2 + exp(-x^2)) / (2x) for a fixed table: minimum 2.132 at x = 1.2 (2.449 for
  the box). Growing the table past x0 = 1.0 until the first row, then stopping, gives 2.048.
mechanism: >
  Two stored entries collide as x-classes with probability 2/(r-1), independently across pairs whose
  difference vectors differ: n^2/2 pairs give n^2/r rows (here T^2/r). For lags tau >= 12 the difference
  Sum of tau jumps is a multiset sum; with 16 jumps there are C(tau+15, 15) >= 5e6 distinct multisets, so
  distinct windows give distinct vectors (the burst of the box, where a single difference vector repeats T/4
  times, cannot occur). A collision is E_t = +/-E_s, i.e. (u_t -/+ u_s) s1 + (v_t -/+ v_s) s2 = 0 mod r. The row
  has no (a, b) part, so it cannot be a rho collision (the guard's concern, `is_guarded`, `ic_boundary.rs` 8717).
  Rows are then handed to the loop as `Relation::Row { a: 0, b: 0 }`, exactly as wsum `free=1` does; probe hits
  are `Row { columns: [(col_1, e1 coef_1), (col_2, e2 coef_2)], a, b }` with the trial's own (a, b), because a
  hit's coefficients (u_t, v_t) reach about T w / 2 (millions) and must not be expanded into repeated summands.
prediction:
  free_row_fraction: "1 - exp(-1.44) = 0.76 [0.70, 0.82] of bases at x = 1.2; mean rows 1.44 [1.0, 1.9] (preflight, per base)"
  box_control_same_T: "fraction of bases with at least one free row 0.009 at k = 24 and 0.002 at k = 28 (SYNTHESIS 1.1, derived); a measured value above 0.20 refutes SYNTHESIS 1.1, not P2"
  S_w0_fixed_x1.2:  {k20: "4.89 [4.6, 5.3]", k24: "3.11 [2.95, 3.30]", k28: "2.29 [2.15, 2.45]", k32: "2.21 [2.10, 2.35]"}
  S_w0_grow_x0_1.0: {k24: "3.03 [2.88, 3.22]", k28: "2.21 [2.08, 2.38]", k32: "2.14 [2.03, 2.28]"}
  # incumbent [pcc:unverified]: 4.860 / 3.318 / 2.576 / 2.519. Built as model value x sqrt(r) + the incumbent's own
  # excess (873, 1,259, 736, 1,622 GAE) + 125-200 GAE for 16 jump set-ups (about 4-8 adds each from doubled seed
  # multiples) and verifying about 1.4 rows by recomputation (two short scalar mults each). If P1 holds the excess
  # is removed and the S values drop by 0.87 (k = 24) and 0.13 (k = 28), to about 2.24 and 2.17 (fixed).
  ratio_sel_over_inc_w0: {k20: "1.01 [0.93, 1.10]", k24: "0.94 [0.89, 0.99]", k28: "0.89 [0.83, 0.95]", k32: "0.88 [0.82, 0.94]"}
  S_w01_w1: "ratios within +/-0.03 of the w = 0 ratio (table inserts T and probe lookups change by less than 3%; fewer probes when a row exists lowers lookups slightly)"
  distance_to_rho_w0: "k24 2.7x -> 2.5x, k28 2.8x -> 2.3x, k32 2.7x -> 2.3x (rho 1.22, 0.99, 0.94 as in the brief and round 26)"
discriminating_test: >
  Stage 0 (exact preflight, no ecbench): native subcommand `curve25519_family walk-rows <k> <x> <jumps> <bases>`
  in `src/bin/curve25519_family.rs`. For each of 1,000 bases (the projected-abscissa seed pair of independent
  curves/seed offsets as the family tool already enumerates), build the walk table at T = x sqrt(r), record every
  key collision, recompute both entries by scalar multiplication of (u, v) (counted, preflight only) and verify
  E_t = +/-E_s, then print: fraction of bases with at least one verified row, mean rows, number of distinct
  difference vectors among colliding pairs, and the same for the wsum box at the same T (control). At k = 20,
  brute-force the seed logs and check u s1 + v s2 = 0 mod r. Controls: permuted (u, v) labels (every row must
  fail verification), and the box. Preregistered: fraction 0.76 [0.70, 0.82] at x = 1.2 on 1,000 bases at k = 24
  and 28; mean rows 1.44 [1.0, 1.9]; box fraction at most 0.03. Stage 1 (only if Stage 0 passes): ecbench panel
  {inc, P2 fixed, P2 grow, rho.negation, inc-aa} at k = 24, 28, 32 through the frozen holdout rule (480 runs).
falsification: >
  Stage 0: fraction of bases with a verified row below 0.50 at x = 1.2 (the walk does not mix; rows are not
  independent), or mean rows outside [0.6, 2.3], or any row failing the recomputation check, or the permuted
  control verifying any row. Stage 1: paired sel / inc interval containing 1 at k = 28 or 32 or point estimate
  above 0.95 there; mean probe hits to pin d above 2.6 when a row exists (the 2-hit claim is false); any
  unverified solve. A measured S below 1.9 is an accounting error (it would sit below the K = 2 table-probe
  floor).
cost_band: >
  Arms: inc, P2 fixed, P2 grow, rho.negation, inc-aa, priced at w = 0, 0.1, 1 by `ecbench band --ranks`.
  Counted: the T table adds, 16 jump set-ups, probe adds, and the recomputation that verifies the row and each
  hit (stored points would make this free, but the verification is then of bookkeeping only; use recomputation
  in the measured arm). Unpriced: one hash insert per entry (collision capture is the entry API's result, no
  extra operation), one lookup per probe, row operations on 3 columns. The box arm and the walk arm have the
  same T and the same unpriced counts to first order; rho priced identically.
controls: [inc-aa, rho.negation, box table at the same T (wsum), permuted-label verification control, a
  cofactor-1 control curve (row fraction should follow 1 - exp(-x^2) there too)]
risks:
  - "Jump lattice bias: a jump set with a small determinant alpha_i beta_j - alpha_j beta_i makes Delta lie in a sublattice; draw jumps independently and report the determinant gcd with r."
  - "The walk visits the same point twice by chance only at the predicted rate; a deterministic step map would merge trajectories (standard rho effect). h_t must be independent of the point."
  - "At k = 20 the set-up and verification (about 125 GAE) cancel the gain: null expected there."
  - "If P1 fails, the k = 24 ratio is diluted by the unexplained excess; if P1 succeeds the same absolute saving is a larger ratio."
  - "Classification: the table-table rows are collision-based (owner criterion of 2026-10-24 recorded in research/ic_sweeper_20261024/README.md 2.4, not in AGENTS.md on main). wsum `free=1` is already ratcheted, and these rows are target-free, so they cannot be rho; the owner rules. Report C = 2 and the free-row count in every table."
closest_baseline:
  in_repo: ["wsum `free=1` (round 16-26 incumbents): same row type, box-shaped table, rank 0 (SYNTHESIS 1.1)",
            "ideas/PROPOSALS.md P2 (ksum duplicate keys) and round 15 (ksum free rows: low rank, [pcc:unverified, PCC-20261104-836de4])",
            "ideas/SYNTHESIS.md R1 (all-pairs walk over G, Q and the seeds): P2 is its target-free half, with the probe side left as the table-probe design"]
  published: ["Pollard rho / Kuhn-Struik multi-collision (SAC 2001): the birthday count of stored points", "Amadori-Pintore-Sala, Mueller (one relation over a random factor base; slides only)"]
  difference: "I found no published method that builds the factor-base combination table as a random-jump walk so that its own collisions are the relations among unknown seed logs; the searches returned the generic rho collision framework only."
code_needed:
  files:
    - "src/cryptanalysis/ic_boundary.rs: `WalkTable<E>` next to `WSumTable` (7422): 16 jumps from doubled multiples of the two seeds, `build` = T adds, key -> (u, v) packed u32 pair, entry-API collision capture, `free_rows` (recompute both entries, check E_t = +/-E_s, emit the two-column row), `probe` returns (u, v)"
    - "src/cryptanalysis/ic_framework/plugins.rs: `SWalkOracle` next to `WSumOracle` (2410), parameters `x` (table size in sqrt(r)/100), `jumps`, `grow` (0/1), `free`; decompose_relations returns the free row once, then `Row { a, b }` on a hit"
    - "src/cryptanalysis/ecbench/methods.rs: one match arm \"swalk\" in `solve_ic_prime` (5867)"
    - "src/bin/curve25519_family.rs: `walk-rows` preflight (native Rust)"
  ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=2,scan_cap=32, oracle=swalk:x=120,jumps=16,free=1"
lean_obligations:
  - "walk_row_sound: in Z/r, with P_i = [s_i]G and x(E_t) = x(E_s), E_t = +/-E_s gives (u_t -/+ u_s) s1 + (v_t -/+ v_s) s2 = 0 (needs `xclass_eq_pm`: x(P) = x(Q) on y^2 = f(x) implies Q = +/-P; neither lemma name exists in a .lean file in this repo, grep returned none)"
  - "pins_after_row_and_two_probes: if u s1 + v s2 = 0 with v a unit, and probe rows a_i + b_i d = f_i s1 (f_i = e_i1 - u e_i2 / v) have a nonzero 2x2 determinant, then (d, s1, s2) is unique (linear algebra over the field Z/r)"
  - "The independence heuristic mu = T^2/r is NOT a Lean claim; it is the preflight's tested assumption."
parity_statement: "Narrows 2.5 -> 2.2-2.3 (S) at k = 28, 32; the 2-hit floor is 2.05 against rho 0.94-1.03, so about 2.1x remains."
reduces_to: >
  The Shoup / birthday count of pairs (T^2/2 pairs, probability 2/r each) with one row needed instead of
  three; K_eff = 2 is the minimum for any design with one random unknown seed beyond the target. The P2 floor
  (2.05) sits above the all-pairs floor 1.66 (R1) because probes are not stored against each other.
```

## 4. P3 (rank 3): interleave table growth and probing in one hash map

```yaml
id: P3-interleaved-single-map
title: "Grow the box table and probe at equal rates, storing probes, so the stopping time adapts to the K = 3 hits"
track: generic decomposition (table family), C = 2 reported
obstruction_attacked: >
  The fixed-table design pays T before it knows when the third hit will come. Pollard's interleaving of baby and
  giant steps is the known remedy; it lowers the Poisson-time cost of K = 3 hits from sqrt(2K) = 2.449 to
  sqrt(2) Gamma(K + 1/2) / Gamma(K) = 2.350.
claim: >
  One hash map holds both table entries and probes. At each trial the pipeline's probe (one add) is inserted or
  matched against the table entries stored so far, and the oracle adds exactly one new box entry (one add, in
  shell order) inserted or matched against stored probes. Every match is a row. With equal rates the pair
  count after tau adds is tau^2/4 and the K-th hit arrives at E[tau] = sqrt(2 r) Gamma(K + 1/2) / Gamma(K) =
  2.350 sqrt(r) (K = 3). Paired sel / inc at w = 0 is 0.96 [0.93, 0.99] at k = 28 and 32 and at w = 1.
mechanism: >
  Pairs between a stored table entry and a stored probe are table-probe pairs (valid rows, not rho: the probe
  has (a, b) and the entry has seed coefficients). The hit count is a Poisson process in the cumulative pair count
  tau^2/(2r); no table size has to be chosen before the first hit time is known. Entries cost one add in any order
  once [mu] P_c multiples up to the largest shell are available (the DFS already relies on them, `WSumTable::build`,
  7492-7506), so shell-by-shell growth costs the same adds per entry as the DFS. The single entry-API map gives
  one hash operation per element, the same unpriced operation count per add as the static design's one insert per
  entry plus one lookup per probe plus one guard insert.
prediction:
  S_w0: {k20: "4.76 [4.5, 5.1]", k24: "3.22 [3.05, 3.40]", k28: "2.48 [2.35, 2.62]", k32: "2.42 [2.30, 2.55]"}
        # incumbent [pcc:unverified]: 4.860 / 3.318 / 2.576 / 2.519; derived as incumbent minus (2.449 - 2.350) = 0.099
  ratio_sel_over_inc_w0: {k20: "0.98 [0.94, 1.02]", k24: "0.97 [0.94, 1.00]", k28: "0.96 [0.93, 0.99]", k32: "0.96 [0.93, 0.99]"}
  S_w1: "ratio 0.96 [0.92, 1.00] (unpriced operations per add unchanged to first order)"
  power_note: "per-run cost has a coefficient of variation of roughly 0.3, so a 4% paired effect needs about 480-960 holdout runs for a 95% interval below 1; freeze 960."
discriminating_test: >
  No preflight needed (the hit statistics are the model's). Stage 1: ecbench panel {inc, P3 with rate = 1,
  P3 with table:probe rates 1:2 and 2:1, rho.negation, inc-aa} at k = 28 and 32, 960 holdout runs, fresh
  target_seed. Preregistered: mean total adds at the K-th hit within 5% of 2.350 sqrt(r) + the incumbent's
  excess; rates 1:2 and 2:1 both worse than 1:1 (the symmetric split maximises pairs).
falsification: >
  Paired interval at k = 28 or 32 contains 1 or its point estimate exceeds 0.99; or mean adds at the K-th hit
  differ from the model by more than 10% (the pair process is not Poisson); or unpriced operations per add exceed
  the static design's by more than 10% (then w = 1 will not order it); or any unverified solve.
cost_band: >
  Arms: inc, P3 (1:1, 1:2, 2:1), rho.negation, inc-aa at w = 0, 0.1, 1. Counted: probe adds, table adds,
  multiples. Unpriced: one map operation per element; stored probes add one memory word each. The static arm
  already pays one guard insert per probe, which the shared map replaces rather than adds to.
controls: [inc-aa, rho.negation, static arm at the model-optimal B, the 1:2 and 2:1 rate arms]
risks:
  - "The gain is 4%; a null at 480 runs is the expected outcome of an underpowered test, hence 960."
  - "If a table entry equals a stored probe in the same trial (both sides present) the row is still valid; order of insert and match must be fixed before the run."
  - "Classification: the same C = 2 boundary as the incumbents (owner ruling pending), no new issue."
closest_baseline:
  in_repo: ["wsum incumbents (static box + probe stream)"]
  published: ["Pollard's interleaved baby-step giant-step (average 1.5 sqrt(r) -> 4/3 sqrt(r)), as credited in Galbraith's ECC 2015 slides (lfant.math.u-bordeaux.fr/ecc2015/documents/galbraith.pdf); Galbraith-Wang-Zhang, ePrint 2015/605 (interleaved BSGS and grumpy giants beat rho on average once negation and the X +/- Y trick are in; the +/- trick is an accounting change here and is NOT used)"]
  difference: "the same adaptive-stopping idea applied to K = C + 1 = 3 unknown logs of a coefficient-box table, with rows rather than a single hit"
code_needed:
  files:
    - "src/cryptanalysis/ic_boundary.rs: `WSumTable::grow_shell(&mut self, g, fb, ops) -> Option<(key, packed)>` (shell s adds 4s entries, one add each, lazily extends the multiples)"
    - "src/cryptanalysis/ic_framework/plugins.rs: `WSumOracle` option `grow=1` plus `rate`; a map key -> {Entry(packed), Probe(point, a, b)}; `decompose_relations` does both inserts and emits `Row { a, b }` for each match"
  ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=2,scan_cap=32, oracle=wsum:grow=1,rate=1,free=0"
lean_obligations:
  - "pair_row_sound: a stored probe R = [a]G + [b]Q matching a box entry gives a + b d = +/- (e1 s1 + e2 s2) in Z/r (same lemma as wsum's probe row; needs `xclass_eq_pm`). The Gamma formula is a heuristic, not a Lean claim."
parity_statement: "Narrows by 4%: 2.7x -> 2.6x of rho. A proof-of-method for adaptivity that composes with P2."
reduces_to: "Pollard's interleaved BSGS with K = 3 hits: the generic table-probe pair-count bound, tight at 2.350 sqrt(r)."
```

## 5. P4 (rank 4 on headline S, own gauge): amortise the two seeds across L targets

```yaml
id: P4-wsum-pooled-targets
title: "One C = 2 table, L targets, L + 2 hits: the seed unknowns are paid once per batch"
track: generic decomposition, a batch gauge. AGENTS.md "Primary comparison uses one target": the headline single-target S is unchanged; this is reported separately and against rho.multitarget_pooled.
obstruction_attacked: "The C unknown seed columns. For L targets the system has L + C unknowns, so the per-target relation count falls from C + 1 to 1 + C/L."
claim: >
  A batch of L targets Q_1..Q_L solved against one table T* = sqrt((C+L) r/2) costs sqrt(2 (C+L)) sqrt(r) in total
  adds: 3.46 / 4.47 / 6.00 sqrt(r) at L = 4 / 8 / 16 (C = 2), which is 0.866 / 0.559 / 0.375 sqrt(r) per target,
  against 2.449 at L = 1. The matched pooled-rho reference (Kuhn-Struik sum, `docs/ecbench/README.md` line 573)
  is 1.94 / 2.78 / 3.97 sqrt(r), so the distance to matched rho falls from 2.76x (L = 1, derived 2.449 / 0.886)
  to 1.79x / 1.61x / 1.51x, tending to sqrt(2)/1.0 = 1.41x as L grows. It never goes below pooled rho.
mechanism: >
  Rows are a + b_j d_j = e . s with a different target column per row; every target needs one hit, and two
  more hits from any targets pin (s1, s2), so L + 2 hits in total at rate 2T/r per probe. Probe each uncovered
  target once, retire it on its first hit, spend the last two hits anywhere. The table is shared, so T is
  paid once. With P1 in place there is no per-target walk set-up (with the default walk each target pays 16
  jumps and the amortisation is lost).
prediction:
  total_over_sqrt_r: {L4: "3.46 + excess/sqrt(r)", L8: "4.47", L16: "6.00"}
  per_target_S_k28_k32: {L4: "0.87 [0.8, 1.0]", L8: "0.56 [0.52, 0.66]", L16: "0.38 [0.35, 0.45]"}
  per_target_S_k24: {L8: "0.57 [0.52, 0.70] if P1 holds, 0.69 if not (the 1,259 GAE excess shared over L)"}
  ratio_to_pooled_rho_KS: {L4: "1.79 [1.7, 2.0]", L8: "1.61 [1.5, 1.8]", L16: "1.51 [1.4, 1.7]"}
  ratio_to_L_independent_rho: "L x S_rho.negation (0.944-0.996 at k = 28 [pcc:unverified]): 0.91 / 0.59 / 0.40 at L = 4 / 8 / 16"
discriminating_test: >
  Stage 0 (exact): a native count in `curve25519_family` of the hit-allocation rule on simulated (planted) logs at
  k = 20, 24, 28: total hits to pin all L logs, over 10,000 simulated batches, against L + 2 and the coupon bound
  (retire-on-first-hit makes it (L + 2)/p). Stage 1: new ecbench method `ic.pooled_wsum` (registered `applies:
  prime_only`, `batch` parameter, the same companion-target law as `rho.multitarget_pooled`: companions hashed to
  <G> by the public-target law, logs checked inside the run, record target solved last, `S` = whole batch over
  sqrt(r)); panel {rho.multitarget_pooled, L x rho.negation, P4} at L = 4, 8, 16, k = 24 and 28.
falsification: >
  Total adds at L = 8 more than 1.15x sqrt(2 (C + L)) sqrt(r) + excess, or hits needed above L + 3 on average (the
  allocation or the rank assumption is wrong); or any companion log fails the in-run check; or the batch S
  falls below the pooled-rho Kuhn-Struik reference (accounting error).
cost_band: >
  Arms: rho.multitarget_pooled, L x rho.negation, P4, at w = 0, 0.1, 1. Counted: the one table, every probe, row
  confirmations, the L internal [d_j]G = Q_j checks. Unpriced: table inserts, probe lookups, guard probes (none
  with the generator progression), row operations on L + 2 columns (grows with L: report it; at L = 16 the
  elimination is 18 columns). Rho priced identically.
controls: [rho.multitarget_pooled at the same L, L independent rho.negation runs, P4 at L = 1 (must equal the single-target arm), inc-aa]
risks:
  - "Not the headline metric; a reader must not compare per-target S with the single-target table. Report both, as AGENTS.md 'Cold-start and genuinely amortized costs' requires."
  - "Needs new ecbench method code (about 300 lines, the largest of the four)."
  - "Row operations grow with L; at w = 1 the band may not order P4 against pooled rho for large L."
  - "Companion logs unknown to the solver: the construction follows rho.multitarget_pooled so the comparison is like for like."
closest_baseline:
  in_repo: ["`rho.multitarget_pooled` (docs/ecbench/README.md 508, 563-576)", "ideas/SYNTHESIS.md section 3 row 'Multi-target amortisation' (derived sqrt(1 + C/L), declined there as a separate question)"]
  published: ["Kuhn-Struik, Random walks revisited, SAC 2001 (about sqrt(L) for L logs; the sqrt(L n) figure is from memory, the search returned only the bibliographic record)", "multi-target BSGS (Bernstein-Lange) not read"]
  difference: "the seed unknowns of a C = 2 coefficient-box table are shared rather than the walk or the table alone"
code_needed:
  files:
    - "src/cryptanalysis/ecbench/methods.rs: `ic.pooled_wsum` next to the `rho.multitarget_pooled` registration; companion law reused"
    - "src/cryptanalysis/ic_boundary.rs: a pooled variant of the relation loop (columns s1, s2, d_1..d_L; retire-on-first-hit allocation)"
lean_obligations:
  - "pooled_system_pins: rows of the form a_i + d_{j(i)} b_i = e_i . s over Z/r with at least one row per target and L + 2 rows of generic rank determine every d_j (linear algebra over a field). The L-fold amortisation formula is a heuristic, not a Lean claim."
parity_statement: "Distance to the matched pooled rho: 2.76x -> 1.5x-1.8x. It cannot pass pooled rho (floor sqrt(2)/1.0 = 1.41x)."
reduces_to: "Kuhn-Struik pooled rho and multi-target BSGS: the table-probe floor sqrt(2 (C+L)) against rho's sum_{i<L} C(2i,i) 4^(-i) sqrt(pi/4)."
```

## 6. Ranking by expected S reduction times probability

Reductions are against the round 26 ledger incumbents (k = 24: 3.318, k = 28: 2.576) [pcc:unverified];
probabilities are my judgements, not measurements.

| rank | proposal | goes below 2.449? | predicted S k=24 / k=28 | delta S k=24 / k=28 | probability of passing its test | expected delta S k=24 / k=28 | cost to test |
|--:|:--|:--|:--|:--|--:|:--|:--|
| 1 | P1 stride-generator | no, reaches the model | 2.55 / 2.44 | 0.77 / 0.14 | 0.45 | 0.35 / 0.06 | one panel, no code |
| 2 | P2 walk table, free full-rank rows | yes (2.13, or 2.05 grown) | 3.11 / 2.29 | 0.21 / 0.29 | 0.50 | 0.10 / 0.15 | about 400 lines + preflight |
| 3 | P3 interleaved single map | yes (2.350) | 3.22 / 2.48 | 0.10 / 0.10 | 0.60 | 0.06 / 0.06 | about 150 lines |
| 4 | P4 pooled targets | yes per target, own gauge | 0.57 / 0.56 per target at L = 8 | not comparable | 0.70 | not comparable | about 300 lines, new method |

If P1 holds, P2 and P3 compose on top of it. Reading S additively in sqrt(r) units, P1 + P2 gives about 2.24 at
k = 24 and 2.17 at k = 28 and P1 + P2 + P3 would remove about another 0.1; I do not claim a composite number
(three untested effects). The bundled test must be done one at a time, in rank order, with the A/A arm each time.

## 7. Where each reduces to a known generic bound

| proposal | known bound it reduces to | what is new |
|:--|:--|:--|
| P1 | none; it is an accounting-clean removal of counted set-up (scalar multiplications that were GAE) | a probe source the wsum arms never used |
| P2 | Shoup/birthday pair count, the Poisson row count of Kuhn-Struik; K_eff = 2 is the least possible with one random unknown seed | a table shape whose own collisions are full-rank; target-free so the rho guard does not apply |
| P3 | Pollard's interleaved BSGS (average 1.5 -> 4/3 sqrt(r)) at K = 3 hits | applied to a rows-not-hit design |
| P4 | Kuhn-Struik pooled rho and multi-target BSGS | the seed unknowns shared across a batch of a coefficient-box table |

None of the four breaks the generic-group floor. Any table-probe S below about 1.5 sqrt(r)-units is an
accounting error first (SYNTHESIS 4, item 8 and section 6 item 1).

## 8. Considered and not proposed (each with the obstruction)

| candidate | obstruction | evidence |
|:--|:--|:--|
| C = 1 rung (a table over one random seed, or over {G, P1}) | K = 2, static 2.00, interleaved 1.88: below the model, but it is a baby-step giant-step in a two-dimensional lattice with one unknown base. The brief excludes relabelling BSGS; the owner ruling on C = 2 is pending (round 22-25 rows) | derived; ideas/SYNTHESIS.md 4, item 1 |
| R1 all-pairs walk over G, Q and the seeds | already filed (SYNTHESIS R1), untested, floor 1.66; strictly worse than rho (needs 3 collisions where rho needs 1), so its IC content is the owner's call. P2 is its target-free half | SYNTHESIS R1; not repeated here |
| R3 cheap-walk set-up (`jumps=cheap`) | already filed; P1 is the cheaper first test of the same hypothesis. If P1 fails and Stage 0 shows set-up still dominates, R3 is the next step | SYNTHESIS R3 |
| Unprojected seeds with 8-torsion translated probes | each entry still matches one coset; hits per add are unchanged. A projected table already puts every entry in the r-subgroup | derived; `curve25519_t0_symmetry_20261102/README.md` (projected / T0 = 3.3-4.0 [unverified assertion]) |
| Quadratic twist, x-only twist points | twist points are in a different group; no map to the r-subgroup of E(F_p) | derived |
| Frobenius over F_p | acts as 1 on E(F_p); the other eigenline of E[r] lives over F_(p^e), with e of the order of r | derived; STRUCTURE.md section 1 |
| F_p-rational endomorphisms / GLV / isogeny transport | every non-scalar endomorphism of an ordinary curve has norm at least about |D|/4 (for pi - 1 it is #E = 8r), so evaluating it is degree about p; the units are +/-1 | derived; STRUCTURE.md 2(c) |
| Both x(P+Q) and x(P-Q) from one slope inversion or one square root (Galbraith-Wang-Zhang) | doubles the entries per counted add only by counting a paired operation as one group addition; the counted work of each of the two points is unchanged, so it is an accounting change | ePrint 2015/605; round 5 `mitmbatch` equals `inc` in S [pcc:unverified] |
| Summation-polynomial tests S3(x_R, x_i, x_j) in place of adds | priced native field work; rootfind-pairs measured 70x and 320x the incumbent at k = 20, 24 [pcc:unverified, round 3] | round-003/README.md |
| Probe-probe collisions (store the probes and use R_i = +/-R_j) | pins d with no seed part: rho with extra steps, the guard's stated reason (`ic_boundary.rs` 8717-8728) | `is_guarded` doc |
| Cheaper hits by LLL on two short relation vectors | two rows leave a one-dimensional family of (d, s1, s2) over Z/r with no further constraint to prune it | derived |

Obstruction to record (I may not write `problems/`): "the table-probe design with K = C + 1 unknown logs has
floor sqrt(2K) sqrt(r) (static) or sqrt(2) Gamma(K+1/2)/Gamma(K) sqrt(r) (adaptive); the all-pairs floor is
Gamma(K+1/2)/Gamma(K) sqrt(r); C = 2 gives 2.449, 2.350 and 1.66". It is a derived generic-group bound with a
Poisson heuristic, not a proved theorem. SYNTHESIS section 6 already suggests
`problems/curve25519-family-generic-ic-floor.md`; this file adds the P2 route (K_eff = 2 at 2.05-2.13) to that
record's exit criteria.

## 9. Literature searched (2026-11-04)

- Galbraith, Wang, Zhang, "Computing elliptic curve discrete logarithms with improved baby-step giant-step
  algorithm" ([ePrint 2015/605](https://eprint.iacr.org/2015/605)); interleaving credited to Pollard in
  [Galbraith's ECC 2015 slides](https://lfant.math.u-bordeaux.fr/ecc2015/documents/galbraith.pdf)
  (1.5 sqrt(r) to 4/3 sqrt(r)). Seen through search summaries only; the PDFs were not read.
- Kuhn, Struik, "Random walks revisited: extensions of Pollard's rho algorithm for computing multiple discrete
  logarithms", SAC 2001 (LNCS 2259, pp. 212-229): the search returned the bibliographic record
  ([bib](https://cr.yp.to/bib/2001/kuhn-rho.bibtex)) but not the sqrt(L) statement, which I give from memory
  and which `docs/ecbench/README.md` 573 restates as the Kuhn-Struik sum.
- Index calculus on elliptic curves over prime fields: McGuire-Mueller, Caen 2018 slides
  ([PDF](https://nitaj.users.lmno.cnrs.fr/Caen2018/Caen2018Slides/Caen2018Mueller.pdf)); a search for a
  published method using a stored random-jump walk over seed points whose own collisions give the relations
  returned only the generic rho collision framework. "No such method found" is a search result, not a
  novelty claim; recheck before any novelty statement.

## 10. Evidence consulted (exact paths)

- `/home/user/crypto/research/curve25519_autolab_20261104/README.md`, `ideas/PROPOSALS.md`, `ideas/SYNTHESIS.md`
- `.../round-012/README.md`, `.../round-026/README.md`, `.../families.json`
- `/home/user/crypto/src/cryptanalysis/ic_boundary.rs`: `WSumTable` 7404-7721, `TargetSource` 8625-8741,
  `draw_jumps` 8805, `Relation` 8932, set-up 9270-9336, `StrideGenerator` 9289 and 9539, pinned test 9675
- `/home/user/crypto/src/cryptanalysis/ic_framework/plugins.rs` `WSumOracle` 2407-2539;
  `ic_framework/stages.rs` `Targets::parse` 769-771; `ecbench/methods.rs` `solve_ic_prime` 5831-5896
- `/home/user/crypto/docs/ecbench/README.md` 508, 563-576

## 11. What I did not do

- Ran nothing: all S values are model arithmetic on the quoted ledger numbers, with the incumbent's excess
  carried additively (the assumption most likely to fail, and the one P1 tests).
- The lead atlas in the sibling crypto-autoresearcher checkout (`python3 tools/lead_atlas.py --brief`,
  `knowledge/leads/ecdlp/`) is not present on this machine (a glob of `/home/user/*/knowledge/leads/ecdlp/*`
  returned nothing), so I could not check what was tried there; the in-repo `ideas/` and the rounds were the
  only prior-art check. Run `--match` before freezing any of P1-P4.
- Did not verify that `ic.pipeline` accepts `targets=stride-generator` together with the wsum oracle (the
  parser has the spelling; `ecbench plan` will tell), nor the default `targets` of the wsum arms (assumed
  `walk`; confirm in a plan output).
- Did not read session records, so P1's Stage 0 and the excess attribution are unchecked.
- No Lean proof was written; the obligations are named per proposal. No `problems/` file, ledger or result
  was edited.

## Outcomes (2026-11-04)

- P1 (stride-generator probes): ratcheted k = 20, 24, 28 in rounds 27 and 28. It lost at k = 32 because the seeds always satisfy a short relation w·S = [m]G, which repeats hits along lines and gives dependent rows (test `wsum_stride_dependence_tests`). Rounds 29 and 30 showed that any arithmetic-progression probe stream clusters its hits the same way [pcc:unverified].
- P2 (walk-built table, `wwalk`): ratcheted k = 24, 28, 32 in round 31. The extension with known-log G jumps grown to rank-2 seed rows (`g=1,grow=2`) ratcheted them again in round 33, to S 1.67, 1.57, 1.52 (1.43x to 1.55x matched rho) [pcc:unverified].
- P3 (interleaved table and probes over one map, `wwalk ... interleave=1`): falsified in a unit trend before any ecbench round. On train-k32, 16 runs per arm, S was 2.15 / 2.10 / 2.11 at walk-to-probe ratio 1 / 2 / 4, against 1.77 for the static `g=1,grow=2` table. With unknown seeds, a hit counts only once both seed logs are pinned, so probes made during the interleave are mostly wasted. The code was not merged.
- P4 (multi-target amortisation): not tried.
