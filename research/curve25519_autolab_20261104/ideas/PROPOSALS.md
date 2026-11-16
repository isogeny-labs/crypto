# Curve25519-family index calculus: proposals to close the gap to rho (Idea Generator, 2026-11-04)

Status: **proposals only**. Nothing here is a measurement, a result, or a status change. Every number is
either quoted from a committed file (path given) or marked *derived* (arithmetic on quoted numbers) or
*predicted* (preregistered below, to be frozen by the Coordinator before any run). Claim records: none; all
quoted autolab outcomes are `[pcc:unverified]` in their own READMEs and are used here as such.

## 0. Read this first: what I found, and what I did not find

**I did not find a representation-level lever (coordinates, summation polynomials, the field, the curve's
structure) that I can defend with a plausible, priced, measured gain.** I looked for one in the evidence on
disk and in the literature, and each candidate fell to one of three obstructions (section 5). The three
proposals below are therefore **generic-track engineering inside the table family** the ledger already
uses. They behave identically on a random base: they do not use Curve25519's structure, and they cannot beat
the generic-group floor. I include them because (a) the owner's success criterion counts any verified
improvement over the incumbent, (b) they are the only proposals I can price in group operations, and (c) a
calibrated cost model (section 1) says the incumbents are far above that floor for reasons that are not
structural. If the Coordinator holds the "must use the representation" requirement strictly, none of the
three qualifies and the honest answer is "no such proposal yet"; the obstruction is recorded in section 5
and should go to `problems/` (I may not write there).

Ranking is by expected value per unit of implementation work: **P1 (config only) > P2 (about 300 lines) >
P3 (about 450 lines)**.

### Policy risk that applies to P2 and P3 (flag, not a verdict)

`research/ic_sweeper_20261024/README.md` section 2.4 records an owner criterion dated 2026-10-24 that an IC
pipeline "counts only when its relations come from solving summation-polynomial systems ... without table
collisions", noting it is not in `AGENTS.md` on `main`. The autolab ledger already ratchets table methods
(`ksum`, `mitm`), so I treat table methods as in scope as the task states. P2 reads collisions inside the
table as relations; P3 shrinks the factor base until the method approaches baby-step giant-step. The owner
decides whether those rows ratchet; I recommend reporting the number of unknown columns `C` as a column in
every table so the classification is visible.

## 1. The cost model (derived; calibrated on the committed incumbents)

Incumbent mechanism (read from source): `ksum:m` builds a table of every signed m-sum of the N-column
projected base, one entry per negation orbit (`KSumTable::build_negation_folded`,
`src/cryptanalysis/ic_boundary.rs:7196`), keyed by the abscissa. Per target, `ic.pipeline` walks
`R_t = [a]G + [b]Q`, probes the table once per trial, and each hit is one row of an incremental Gauss
elimination over `N + 1` columns (`collect_and_solve_relations_scaled_touching`,
`ic_boundary.rs:8639`, `CompletionRule::TargetPinned`). The table is rebuilt in every solve
(`solve_ic_prime`, `ecbench/methods.rs:5794`), so table cost is inside every per-target `S`.

```
adds_build(N, m=4) = N(N-1) + 4 C(N,3) + 8 C(N,4)          (leaf entries T = 8 C(N,4))
adds_build(N, m=3) = N(N-1) + 4 C(N,3)                      (leaf entries T = 4 C(N,3))
p_hit              = 2 T / r                                (T distinct x-classes of (r-1)/2)
trials             = (N + 1) * r / (2 T)
cost(w = 0)        = adds_build + trials + residual          S = cost / sqrt(r)
```

Calibration against the ledger incumbents (`research/curve25519_autolab_20261104/ledger.json`,
`round-005/README.md` ledger table; `sqrt(r)` = 362.2, 1448.2, 5792.9, 23170.3 for k = 20, 24, 28, 32):

| k | incumbent | model adds + trials | model S | measured S (w = 0) | residual (GAE) |
|--:|:--|--:|--:|--:|--:|
| 20 | `ksum3-n11` | 770 + 1,193 = 1,963 | 5.42 | 8.987 | +1,292 |
| 24 | `ksum4-n12` | 4,972 + 3,443 = 8,415 | 5.81 | 6.985 | +1,700 |
| 28 | `ksum4-n18` (round 8) | 28,050 + 13,023 = 41,073 | 7.09 | 7.445 | +2,053 |
| 28 | `ksum4-n21` (round 4, superseded) | 53,620 + 7,709 = 61,329 | 10.59 | 10.939 | +2,041 |
| 32 | `ksum4-n26` | 130,650 + 60,602 = 191,252 | 8.254 | 8.296 | +968 |

The model reproduces the k = 32 incumbent to 0.5% and the k = 28 `n18` incumbent to 5%; the model was
written down from the k = 28 `n21` and k = 32 numbers and then checked against `n18`, which it had not
been fitted to (it also says `n21` at k = 28 was not the optimum, which round 8 then found). The
**unexplained residual is +1.0k to +2.1k GAE per solve, 17% to 40% of the incumbent's total at k = 20 and
24.** My hypothesis (unverified; see P1): the residual is the walk's set-up, `draw_jumps`
(`ic_boundary.rs:8319`) costs two scalar multiplications and one addition per jump, and the pipeline
draws a pool of jumps per solve; 32 scalar multiplications at about 40 GAE each is the right size.

Unpriced work at w = 0.1 and 1 (hash inserts, lookups, row operations): the incumbents' S is linear in w
(k = 20: 8.987, 9.710, 16.217), giving unpriced counts `U = (S(1) - S(0)) sqrt(r)` of 2,619 / 7,655 /
26,966 / 119,325 at k = 20 / 24 / 28 / 32. `U` is about the number of table inserts plus lookups at k = 24
and 20% to 35% below it at k = 28 and 32, so **I do not have a calibrated w model**; the w = 0.1 and 1
predictions below scale the incumbent's `U` by `(T + trials)_new / (T + trials)_inc`, stated as crude.

Floor for the table family (derived, generic): with negation, `T + r/(2T)` is minimised at
`T = sqrt(r/2)`, total `sqrt(2) sqrt(r)`, `S >= 1.41` before set-up. Matched rho-neg at the same sizes is
S = 1.58 / 1.16 / 1.02 / 0.91 (derived from the incumbent distances 5.7x / 6.0x / ... and the round
tables), so **the table family cannot come closer than about 0.9x, 1.2x, 1.4x, 1.55x of rho at k = 20,
24, 28, 32**. A measured table-family `S` below 1.3 should be treated as an accounting error, not a result.

---

## P1 (rank 1): set the walk's start cost aside: `targets=stride` on the ksum incumbents

```yaml
id: P1-stride-start
title: "Replace the 16-jump walk start by a one-stride start on the ksum incumbents (config only)"
track: generic (table family); class if it works: engineering
claim: >
  On the incumbents ksum3-n11 (k=20) and ksum4-n12 (k=24), `targets=stride` lowers the paired
  sel/inc cost at w=0 to at most 0.80 (k=20) and at most 0.95 (k=24), because the per-solve residual of
  +1,292 and +1,700 GAE is walk-jump set-up that a one-joint-multiplication stride start does not pay.
mechanism: >
  The default `targets=walk` draws jumps `[aj]G + [bj]Q` (two scalar multiplications each) before the
  first probe. A stride start (`TargetSource::Stride`, `ic_framework/mod.rs:514`) pays one joint
  multiplication, then adds one point per trial. Table methods need only that successive targets are
  distinct points with known (a, b); the table does not care how they are generated.
  Uses no curve structure (a random base has it too). The ledger's panels vary N and the oracle family
  only: no spec under research/curve25519_autolab_20261104/*/specs/ sets `targets`, so this knob is untried.
prediction:
  derived_from: "cost - residual + 100 GAE (two scalar mults and the stride), residual hypothesised = walk set-up"
  S_w0:   {k20: 5.70, k24: 5.88, k28: 7.11, k32: 8.27}      # inc: 8.987, 6.985, 7.445, 8.296
  ratio_sel_over_inc_w0: {k20: "0.63 [0.55, 0.80]", k24: "0.84 [0.75, 0.95]", k28: "0.95 [0.90, 1.02]", k32: "1.00 [0.97, 1.03]"}
  S_w01_and_w1: "set-up is counted group work, unpriced work unchanged: k20 w=1 ratio 0.80, k24 w=1 ratio 0.91 (derived)"
  distance_to_rho_w0: {k20: "5.7x -> 3.6x", k24: "6.0x -> 5.1x"}
  band: "order sel < inc expected at k20 and k24 at all three weights; no claim at k28, k32"
discriminating_test: >
  Stage 0 (exact, no run): `tools/artifacts/artifacts.py pull` the existing round-002 (k24) and the
  `ksum3-n11` k20 session records and split each solve's group operations into table build, walk set-up
  (scalar_mults and the adds merged with them), relation trials, verification. The residual hypothesis
  predicts walk set-up >= 80% of the residual in the table above. Stage 1: one ecbench panel
  {inc, inc with targets=stride, rho.negation, inc-aa}, k = 20 and 24, fresh target_seed, 32 targets x 2
  rounds, run through the autolab freeze/holdout rule.
falsification: >
  Stage 0: scalar-mult share of the residual below 50% (the hypothesis is wrong; residual stays
  unexplained). Stage 1: paired sel/inc 95% interval containing 1 at k20 or above 0.95 point estimate at
  k24, or any unverified solve. Also kills it: stride probes visibly dependent (hits per trial more than 1.3x
  or less than 0.7x the model p_hit = 2T/r, which would show the arithmetic progression correlates with
  the table keys).
cost_band: >
  Arms (all priced at w = 0, 0.1, 1 through `ecbench band --ranks`): inc, inc+stride, rho-neg, inc-aa.
  Group operations dominate; unpriced = table inserts + lookups + guard probes. The stride arm removes
  the walk pool's operations from the group count and keeps the guard (`is_guarded`, Stride is guarded),
  so its unpriced count is unchanged. Rho references priced by the same band.
controls: [inc-aa (A/A), rho.negation, a cofactor-1 control curve per the baseline study, ksum4-n12 at the
  k = 28/32 curves as a no-effect check (predicted ratio ~1)]
risks:
  - "Residual may be something else (rows needed beyond N+1, base scan, verification mult); Stage 0 decides cheaply."
  - "The stride progression R0 + t*S is an arithmetic progression; probe independence is an assumption."
  - "`Targets::parse` spelling must be confirmed (planner.rs uses 'walk' and 'stride-renew'; 'stride' is the enum variant)."
closest_baseline: "ledger incumbents ksum3-n11 / ksum4-n12; published: no direct citation (a table method does not need a random-walk start)."
code_needed:
  files: [research/curve25519_autolab_20261104/families.json (new arm template only)]
  ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=12,scan_cap=192, oracle=ksum:m=4, targets=stride"
  lean_obligation: none (no mathematical claim; the solve is checked by ecbench against the target)
```

---

## P2 (rank 2): turn the table's own collisions into relations (free rows)

```yaml
id: P2-free-rows
title: "Table self-collision rows for ksum: use the duplicate keys the build already discards"
track: generic (table family); class if it works: engineering (an advance only if the ratio to the floor falls; it does not)
claim: >
  `KSumTable::build_negation_folded` discards every entry whose key already exists
  (`self.map.entry(key).or_insert(packed)`, ic_boundary.rs:7263). Two distinct signed m-sums with equal
  abscissa satisfy S = +/-S', i.e. a verified homogeneous relation sum (e -/+ e')_i s_i = 0 (mod r) among
  the base logs, with no probe. The expected number of such pairs is D = T(T-1)/(r-1) (heuristic: entries
  behave as random x-classes). Injecting them as rows `Relation::Row{a:0, b:0}` cuts the probes needed from
  N+1 to max(2, N+1-rank) and moves the optimal N down. Predicted sel/inc at w=0: 0.85 (k20), 0.78 (k24),
  0.70 (k28), 0.68 (k32).
mechanism: >
  A random base has this too (it is the birthday effect that makes rho and baby-step giant-step work);
  nothing about Montgomery or Curve25519 is used. At the incumbents D is already of the order of N:
  k=24 n12: D = 7.5 against N+1 = 13; k=28 n18: 17.9 against 19; k=32 n26: 26.6 against 27 (derived).
  The build pays for the table once; the relations are present in it unread. The free rows are
  homogeneous so rank <= N-1; the scale and the target still need two probe hits (G is not a column of the
  projected base), hence the max(2, .).
prediction:
  derived_cost_w0: "adds_build + max(2, N+1-D) * r/(2T) + residual (P1 not assumed; additive residual from section 1)"
  best_config_by_model: {k20: "ksum:m=3, N=13", k24: "ksum:m=4, N=12", k28: "ksum:m=4, N=17", k32: "ksum:m=4, N=25"}
  S_w0:   {k20: 7.6, k24: 5.4, k28: 5.25, k32: 5.6}          # inc: 8.987, 6.985, 7.445, 8.296
  ratio_sel_over_inc_w0: {k20: "0.85 [0.75, 0.95]", k24: "0.78 [0.70, 0.88]", k28: "0.70 [0.60, 0.82]", k32: "0.68 [0.58, 0.80]"}
  S_w01_w1_crude: "k20 8.1 / 13.0 (inc 9.71 / 16.2); k24 5.9 / 9.4 (inc 7.51 / 12.27): ratios 0.83/0.80 and 0.79/0.77 (scaled-U, not calibrated)"
  distance_to_rho_w0: {k20: "5.7x -> 4.8x", k24: "6.0x -> 4.6x", k28: "7.3x -> 5.2x", k32: "9.1x -> 6.1x"}
  band: "order sel < inc expected at k28, k32; uncertain at k20 (small D, set-up dominates) and at w=1 (larger table => more unpriced inserts)"
discriminating_test: >
  Exact preflight, no ecbench run: new native subcommand `curve25519_family ksum-dups <k> <N> <m> <inner>`
  builds the family miniature and the projected base exactly as `PrimeProjectedAbscissaBase` does, builds the
  table with duplicate capture, and prints T (attempts), distinct keys, D, the expected T(T-1)/(r-1), and the
  rank over F_r (Gaussian elimination mod r on u64) of the collision rows, after re-checking every row by
  group arithmetic (2(m-1) additions each, counted). It then prints the exact predicted cost from the
  exact D and rank. Preregistered: pooled over at least 16 (k, N, m) cells, sum D / sum E in [0.7, 1.3]
  and rank >= min(D, N-1) - 1 in every cell. Then one ecbench panel {inc, P2 at model-best N, rho-neg,
  inc-aa} per size through the autolab freeze/holdout rule, with an ablation arm
  `free=1,inner=1` (also store the depth < m partial sums, which the DFS already computes: +9% to +25% entries).
falsification: >
  Preflight: pooled D/E outside [0.5, 1.6] (collisions are not random: either structure or an indexing
  bug), or rank deficiency beyond 1 in a cell with D >= N+1 (rows are dependent: the collisions are
  trivial), or any row failing the group check. End to end: sel/inc >= 1 at w=0 with the A/A containing 1,
  or any unverified solve. A measured S below 1.3 is an accounting error (section 1 floor), not a result.
cost_band: >
  Arms: inc (ksum, no rows), P2 (free=1), P2 (free=1, inner=1), rho-neg, inc-aa. P2 adds: row verification
  (about 2(m-1) additions per row, counted group ops, a few hundred at most) and one capture per duplicate
  insert (unpriced; the inserts are already counted). The `Relation::Row` rows enter the elimination as
  unpriced row operations like any other row. Rho priced identically.
controls: [inc-aa, rho-neg, cofactor-1 control curve (D should follow T^2/r there too), shuffled-key control:
  permute the table's key->entry map after build (D unchanged, rows now wrong): every such row must fail the
  group check, which tests the verifier]
risks:
  - "D is deterministic per curve and base, not a random variable per target: all 32 targets share one table and one set of rows. The test is per-curve; pool across (k, N, m) cells."
  - "Rank of sparse +/-1 rows on N columns may fall short of N-1; the model assumes it does not (preflight measures it)."
  - "Policy: collisions-as-relations is a collision-based method (see section 0); report C and D in every table."
  - "At w=1 the larger table costs more unpriced inserts; the band may not order at k=20, 24."
  - "Free rows involve no target, so a cold per-solve S pays the table every time; any caching across targets is a separate (amortisation) claim and is not made."
closest_baseline:
  in_repo: ["research/pdp_preregistrations_20261007/PROTOCOL-E3-index-self-collisions.md (stage diagnostic, 'priced by memory', Koblitz root index)",
            "src/cryptanalysis/koblitz_index_calculus.rs:5884 `free_relations` (Koblitz Frobenius pair table; not on prime ksum)",
            "ic_boundary.rs:9170 `repeated_column_rows` / `pinned_by_repeated_row` (probe-side repeats pin d like a rho collision)"]
  published: ["Amadori-Pintore-Sala; Mueller (random factor base, no Groebner basis; known only from the Caen 2018 slides found by search, not read in full)",
              "Galbraith-Gaudry, Recent progress on the ECDLP, DCC 2016 (survey of summation-polynomial index calculus)",
              "baby-step giant-step / rho with negation (Bernstein-Lange-Schwabe 2011; cited in STRUCTURE.md, not re-read): the birthday mechanism P2 imports"]
  difference: "the collisions are harvested from a k-sum table of unknown-log columns on a prime curve; the in-repo precedents are a diagnostic and a Koblitz-only pair-table path."
code_needed:
  files:
    - "src/cryptanalysis/ic_boundary.rs: `KSumTable` gains `dups: Vec<(u64, u64)>` filled where `or_insert` finds an occupied entry (replace with a `match map.entry(..)`), and `inner: bool` to insert the partial sums at depth < m"
    - "src/cryptanalysis/ic_framework/plugins.rs: `KSumOracle::decompose_relations` override that, on its first call, returns the verified dup rows as `Relation::Row { columns, a: 0, b: 0 }` ahead of the trial's own relation; parameters `free`, `inner` added to `parameters()`"
    - "src/bin/curve25519_family.rs: `ksum-dups` preflight subcommand (native Rust, no Python)"
  ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=12,scan_cap=192, oracle=ksum:m=4,free=1   (ablation: ksum:m=4,free=1,inner=1)"
  lean_obligations:
    - "`xclass_eq_pm`: on y^2 = f(x), x(P) = x(Q) implies Q = P or Q = -P (standard; needed so a key match gives a +/- relation)."
    - "`collision_row_sound`: in an additive group with P_i = [s_i]G, G of prime order r, S = sum a_i P_i = +/- sum b_i P_i = S' implies sum (a_i -/+ b_i) s_i = 0 in Z/r. Pure linear algebra over Z/r."
    - "`pin_after_two_probes`: over the field Z/r, if the homogeneous rows have solution space spanned by v and two probe rows (e.v, b), (e'.v, b') with e.v b' != e'.v b, then d is determined. The heuristic D ~ T^2/r is NOT a Lean claim; it is the preflight's tested assumption."
```

---

## P3 (rank 3): weighted seeds (shrink the number of unknown columns)

```yaml
id: P3-weighted-seeds
title: "Coefficient-box table over C seed points: unknown columns drop from N to C"
track: generic (table family); class if it works: engineering; degenerates to baby-step giant-step as C -> 1
claim: >
  The IC cost is about sqrt(2(N+1)(1+o)) sqrt(r) because it needs N+1 relations. A table of all
  sum_c e_c P_c with |e_c| <= B over C seeds (one entry per negation orbit, T = ((2B+1)^C - 1)/2, one
  addition per entry by depth-first enumeration, [mu]P_c precomputed for mu <= B) has the same one-addition-
  per-entry cost and hit rate 2T/r but only C+1 unknowns. With P2's collision rows it needs max(2, C+1-rank)
  probes. Predicted S at w=0: k=20 6.1, k=24 3.9, k=28 2.5, k=32 2.2.
mechanism: >
  Not curve-specific. The incumbent restricts the table to sums of m DISTINCT base points, which forces
  N = 12..26 columns to reach a table of 4k..120k entries. Repeated points (coefficients) reach the same
  table size with C = 3..4 columns. `Relation::Summands` already accumulates repeated indices per column
  (`addmod(row[c], coef_of[i])`, ic_boundary.rs:9142-9146), so the pipeline needs no change.
prediction:
  configs_by_model: {k20: "C=3, B=5, T=665", k24: "C=4, B=4, T=3280", k28: "C=3, B=12, T=7812", k32: "C=3, B=20, T=34460"}
  S_w0: {k20: 6.1, k24: 3.9, k28: 2.5, k32: 2.2}              # inc: 8.987, 6.985, 7.445, 8.296
  ratio_sel_over_inc_w0: {k20: "0.68 [0.45, 0.85]", k24: "0.57 [0.40, 0.65]", k28: "0.34 [0.25, 0.50]", k32: "0.27 [0.20, 0.40]"}
  S_w01_w1_crude: "k20 6.4 / 9.5 (inc 9.71 / 16.2); k24 4.2 / 6.7 (inc 7.51 / 12.27): ratios ~0.66/0.59 and 0.56/0.55"
  distance_to_rho_w0: {k20: "5.7x -> 3.9x", k24: "6.0x -> 3.4x", k28: "7.3x -> 2.5x", k32: "9.1x -> 2.4x"}
  caveat: "k=20 and 24 are set-up-bound (residual 1.3k-1.7k of 2.2k-5.7k): P1 matters most there. The k=28/32 numbers use an additive residual of 2.0k / 1.0k."
discriminating_test: >
  Exact preflight (same `ksum-dups` binary extended with a `box` mode): for each (k, C, B) print T, the number
  of box points that are distinct mod r (collisions among them are free rows), D against T(T-1)/(r-1), the
  rank of the free rows, and the shortest relation vector found by lattice reduction of the log lattice
  {e : sum e_i s_i = 0 mod r} using the TRUE seed logs (preflight only; the oracle never sees them) to show
  that collisions are the short vectors. Preregistered: D/E in [0.7, 1.3]; rank >= C-1 whenever D >= C+1.
  Then ecbench panels with the reference ladder below.
  Reference ladder (priced identically, same targets): rho-neg; inc; P2 best; P3 at C = 4, 3, 2; and the
  existing known-log base with the ksum table (`ic.known_log_ksum_v1` / `known-log` base) as the table-family
  floor arm (predicted S about 1.5 to 2.0, not 1.41, after set-up and base scalar mults).
falsification: >
  Preflight: D/E outside [0.5, 1.6] or rank < C-2 with D >= C+1. End to end: P3 at the model's best (C, B)
  not below inc at w=0 with the A/A containing 1; or the ladder shows S non-monotone in C in a way the
  model cannot place (e.g. C=2 worse than C=4 by more than 1.5x). A measured S below 1.3 is an accounting
  error. Verification failure on any solve kills the arm.
cost_band: >
  Arms: inc, P2, P3 at three (C, B), known-log floor, rho-neg, inc-aa. Group ops: table additions, seed
  multiples, base scalar multiplications, probe additions, row verification. Unpriced: inserts, lookups,
  guard probes, row operations on at most C+1 columns (cheaper than the incumbent's N+1 = 13..27). The
  floor arm pays N scalar multiplications for its random scalars; all priced at the band.
controls: [inc-aa, rho-neg, known-log floor arm, cofactor-1 control curve, a wrong-logs control: give the box
  oracle seed logs permuted so the rows are false; every solve must fail verification]
risks:
  - "Classification: at C <= 2 this is a baby-step giant-step in a 2-D lattice, not index calculus; report C, and let the owner rule on ratcheting (section 0)."
  - "Forced collisions: seeds whose logs have a very short relation make the box redundant (fewer distinct entries). Preflight counts distinct entries; the oracle must dedupe."
  - "Weight sum can reach C*B (up to 60 for C=3, B=20), so a hit's confirmation costs up to C*B additions unless [e]P_c is precomputed; the model charges a few hundred GAE for this and it should be counted."
  - "The model's residual (section 1) is unexplained; if it scales with the table or with N rather than being fixed, the k=28/32 ratios shift."
closest_baseline:
  in_repo: ["`ksum:m` (distinct points only, ic_boundary.rs:7182); research/ic_arity_sweep_20261027 (arity m only, N grows with m); `ic.known_log_ksum_v1` (ecbench/known_log.rs) as the floor"]
  published: ["Amadori-Pintore-Sala / Mueller random factor base without linear algebra (the C -> N, known-log end; slides only)", "baby-step giant-step with negation"]
  difference: "interpolates between the incumbent (many unknown columns, distinct points) and a known-log table (zero unknowns) by choosing the number of unknown seeds; not found in the repo's sweeps, which vary N and m but never allow repeated points."
code_needed:
  files:
    - "src/cryptanalysis/ic_boundary.rs: `WeightedSumTable` next to `KSumTable` (DFS over coordinates, first nonzero coefficient positive, dedupe on key, optional dup capture as in P2); `probe` returns the summand list with repeated indices"
    - "src/cryptanalysis/ic_framework/plugins.rs: `WeightedSumOracle` next to `KSumOracle` (line ~2282), `parameters()` = coef (B), free"
    - "src/cryptanalysis/ecbench/methods.rs: one match arm `\"wsum\" => &mut wsum` in `solve_ic_prime` (line ~5824) and the method's param declaration"
    - "src/bin/curve25519_family.rs: `box` mode of the preflight"
  ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=4,scan_cap=64, oracle=wsum:coef=4,free=1"
  lean_obligations:
    - "`box_count`: the number of nonzero vectors in {-B..B}^C modulo +/-1 is ((2B+1)^C - 1)/2 (finite combinatorics)."
    - "`box_collision_iff_short_relation`: two box vectors give equal points iff their difference is a vector with sup-norm <= 2B in the log lattice; plus P2's `collision_row_sound` and `pin_after_two_probes`."
```

---

## 4. Order of work and what each outcome changes

1. P1 Stage 0 first (a read of existing records; no run): it prices the residual that every other estimate
   uses. If the residual is not walk set-up, the section 1 residual is an open item and the `S` predictions
   of P2 and P3 for k = 20 and 24 move by up to +1.3k to +1.7k GAE (reported as the interval widths above).
2. P2 preflight (exact D, rank) before any ecbench panel; it is a counting test and kills P2 and P3 cheaply.
3. P1 Stage 1, P2 panel, P3 ladder as autolab rounds with the frozen decision rule and a fresh `target_seed`;
   quote results with their claim record and standing, per "Sabotage is assumed".

## 5. Considered and not proposed (each with the obstruction and evidence)

| candidate | obstruction | evidence |
|:--|:--|:--|
| Closed cofactor fiber / unprojected T0-closed table | projection already gets all 8 cosets for 3 doublings per base point; T0 keys give 2 of 8 | `curve25519_t0_symmetry_20261102/README.md` (projected / T0 = 3.3 to 4.0), `curve25519_torsion_leads_20261103/LEADS.md` T1-T8 |
| Key folds by E[2], E[4], E[8] translations or by the 2-isogeny `x -> x + 1/x + A` (the dual's Klein-4 on E') | act only on cofactor cosets; the r-subgroup sees only +/-1 (every F_p-rational endomorphism acts as a scalar on E(F_p) because pi acts as 1) | derived; `STRUCTURE.md` section 2(b),(c) |
| Legendre / 2-descent or 4-descent filter (`chi(x)`, Tate pairing into mu_4 since 4 divides p-1) | kernel contains the r-subgroup: information about the Z/8 part only | `LEADS.md` T2 (coverage x2.0, F4 per relation 1.28x to 1.71x worse); derived |
| `x -> -x` (p = 5 mod 8, so -1 is a square) | gives E = E_{-A}, an isomorphism of Montgomery models, not an automorphism | derived |
| Coppersmith / lattice small roots of `S_{m+1}` | the existence threshold (B^2 near p) is far above the lattice bound for a bidegree-(2,2) polynomial; the search found no published use of small-roots methods on summation polynomials | the search below returned none; derived, not proved |
| Sqrt-based table or probe stream (both `x(P+Q)` and `x(P-Q)` from one root of `S3`; x-only differential walks) | priced field work: a square root is about 40 field multiplications against about 6 for a batched affine addition (my estimate, not measured here), and the lab counts batched and unbatched x-only table builds at exactly the same S | `round-005/README.md`: `mitmbatch-n512` and `mitmblocked-n512` are 23.193 / 25.381 / 45.069, identical to `inc`; wall-time only (L2) |
| Algebraic track (PKM tower, S3, in-house F4) | F4 multiply-adds are unpriced native work: 18x rho at w=0, 6.9e5x at w=0.1 | `curve25519_isogeny_seed_20261103/README.md` distance table |
| Isogeny-class transport to a curve with extra automorphisms or small endomorphisms | all curves in the class share End inside the order of Q(sqrt(t^2-4p)); j = 0 or 1728 needs a special discriminant; F_p-rational endomorphisms act as scalars of large norm | derived; `STRUCTURE.md` 2(c) |
| Embedding degree / MOV / Frey-Ruck | needs small embedding degree; the family's p^e = 1 mod r has e of the order of r | `STRUCTURE.md` section 1 (255-bit bound); not rechecked at the miniatures |

Obstruction to record (for the Coordinator): "no representation-level group-operation lever on a prime-field
Montgomery curve with cyclic Z/8 cofactor beyond projection and +/-1". Type: untested hypothesis plus
derived arguments, **not a proved obstruction**; what would unblock it is a key symmetry or relation source
acting nontrivially on the r-subgroup (none known) or an algebraic decomposition whose per-relation cost is
a few group operations (the F4 route is 10^5x to 10^7x too expensive at w >= 0.1).

## 6. Literature searched (2026-11-04)

- Galbraith, Gaudry, "Recent progress on the elliptic curve discrete logarithm problem", DCC 2016:
  [eprint 2015/1022](https://eprint.iacr.org/2015/1022). No prime-field subexponential method.
- Petit, Kosters, Messeng, PKC 2016 (prime-field towers): [Springer chapter](https://link.springer.com/chapter/10.1007/978-3-662-49387-8_1);
  summarised by the lab as `pkm-semaev`.
- Amadori, Pintore, Sala and Mueller (random factor base, no Groebner step, still exponential):
  [Mueller, Caen 2018 slides](https://nitaj.users.lmno.cnrs.fr/Caen2018/Caen2018Slides/Caen2018Mueller.pdf),
  [Mueller, Caen 2018](https://nitaj.users.lmno.cnrs.fr/Caen2018/DanielaMuellerCaen2018.pdf).
- Kummer line of a Montgomery curve as the x-line translated by a 2-torsion point, Edwards y-line
  isomorphism: [Kummer-line paper from oldweb.isical.ac.in found by search](https://oldweb.isical.ac.in/~skarati/pub_pdf/BKL251.pdf)
  (checked only through the search summary; authors not verified); no result on the x -> 1/x symmetry in a
  summation-polynomial index calculus on Curve25519 was returned. The lab's own derivation is
  `STRUCTURE.md` 2(b).
- Coppersmith small roots for summation polynomials: no result returned (the search says so). Unsearched
  beyond one query; treat the Coppersmith row of section 5 as unsettled.

The search tool returned summaries, not the papers; every "closest published" statement above rests on those
summaries and on the lab's own reading in `STRUCTURE.md`, and should be rechecked against the PDFs before a
novelty statement is made.

## 7. Evidence consulted (exact paths)

- `/home/user/crypto/research/curve25519_ic_baseline_20261102/README.md`, `STRUCTURE.md`
- `/home/user/crypto/research/curve25519_autolab_20261104/README.md`, `ledger.json`, `round-004/README.md`, `round-005/README.md`
- `/home/user/crypto/research/curve25519_torsion_leads_20261103/LEADS.md`, `curve25519_t0_symmetry_20261102/README.md`, `curve25519_isogeny_seed_20261103/README.md`
- `/home/user/crypto/research/ic_sweeper_20261024/README.md` (section 2.3 falsified list, section 2.4 owner criterion), `ic_arity_sweep_20261027/README.md`, `prime_ecdlp_three_leads_20261029/README.md`
- Source read: `src/cryptanalysis/ic_boundary.rs` (7165-7314 `KSumTable`; 8319 `draw_jumps`; 8446 `Relation`; 8538-9230 relation loop), `ic_framework/plugins.rs` (2279-2347 `KSumOracle`), `ic_framework/stages.rs` (214-314 oracle trait), `ic_framework/mod.rs` (512-640), `ecbench/methods.rs` (5721-5857), `ecbench/known_log.rs` (header)

## 8. What I did not do

I ran nothing (no tool for execution was available to me): the section 1 model is arithmetic on committed
numbers, not a run. I did not read the per-phase counters of any session (the records live in the artifact
store), so the residual hypothesis is unchecked. I did not verify that `Targets::parse` accepts `stride`, that
`PrimeProjectedAbscissaBase` assigns `col_of` and `coef_of` as P2 and P3 assume, or the round 7, 10 and 11
outcomes (the ledger shows round 11 frozen for k = 24 and round 7 / 10 ratcheted without a history entry, so
the k = 20, 24 and 32 incumbents above are as of the committed README and `ledger.json` incumbents block and
must be re-read at freeze time). No code, result or status file was edited.
