> Idea Generator lane report, wave 5, lane "pipeline" (2026-11-12). Proposals only: nothing was run, no file
> outside this one was edited, no status changed. Every number and claim below is `[pcc:unverified]`: either a
> quote of an unverified on-disk assertion (path given) or desk arithmetic done by hand in this lane (no shell was
> available, so no stdlib-Python check was run; every derived figure must be recomputed before use).

# Curve25519-family miniatures: whole-pipeline levers on the index-calculus baseline

## 0. The baseline, its phases, and the verdict on each candidate direction

**Baselines named.** No Curve25519-family row is on `docs/ic/LEADERBOARD.md` (none has passed the validation gate)
[pcc:unverified]. The frozen decomposition baseline is `proj-a` + `mitm:negation_folded=1`, S = 11.76 / 12.57 /
16.38 / 23.31 at k = 20 / 24 / 28 / 32, 7.8x / 10.6x / 18.1x / 24.7x rho-neg (`research/curve25519_ic_baseline_20261102/README.md`)
[pcc:unverified]. The operative ratchet is the autolab generic track, C = 2 seeds (owner ruling 2026-11-04):
`inc` S = 2.703 (box, k = 20) / 2.168 / 1.953 / 1.913, `inc-lazy` 2.096 / 1.932 / 1.908, and `tw2cj-lazy`
(classification pending) 2.529 / 1.915 / 1.756 / 1.758 against rho-neg 1.570 / 1.175 / 1.009 / 0.989
(`research/curve25519_autolab_20261104/snc-panel/README.md`, claim `PCC-20261110-978c29`, standing unverified).
The algebraic-track incumbent sits 1.83e4x (T13) to 6.06e4x (T10 winner) behind rho-neg at w = 0.1
(`research/curve25519_rho_parity_bound_20261104/README.md`, `research/curve25519_t10_winner_ecbench_20261104/README.md`) [pcc:unverified].

**Which phase dominates (all quoted, units of sqrt(r), w = 0)** [pcc:unverified]:

| row | k | dominant phase | other phases | source |
|:--|--:|:--|:--|:--|
| proj-a (frozen) | 20 / 32 | relations 57 % / 47 % | table 25 / 49 %, base 17 / 4 %, LA+verify 1 / 0 % | `curve25519_ic_baseline_20261102/analysis/summary.md` |
| box `inc` | 20 | table + rest 1.52 | probes 1.08 (3 hits, no free rows) | `autolab/ideas/OBSTRUCTION.md` |
| `tw2cj-lazy` | 20 | stored walk 1.738 | everything else about 0.79 (S minus stored entries, derived) | snc-panel |
| `twcj-cv0` (C = 1, boundary) | 20 / 24 | walk 1.378 / 1.363 | set-up 0.368 / 0.110, fixed (base, LA, [d]G) 0.129 / 0.040 | `curve25519_engine_items_20261110/README.md` §6 |
| `inc` (wwalk) | 28 / 32 | walk to rank 2 about 1.3 | probes 0.365 / 0.496, set-up 1-2 % | OBSTRUCTION.md |
| T10 winner (algebraic) | 20 | F4 per trial, about 100 % at w >= 0.1 | group work S = 26.4 at w = 0 | T12 README |

**Two desk findings this lane relies on** (derived here; recompute before use) [pcc:unverified]:

1. *The measured 3 % walk excess is first-order local anti-collision.* In a stored walk `E_{t+1} = E_t + J_{h_t}` with
   `h_t` i.i.d. uniform over j jumps, the pair (s, t+1) is the same event as (s-1, t) whenever `h_t = h_{s-1}`
   (same jump multiset), for both signs; so a fraction 1/j of pair tests repeats an earlier test and the K-th
   independent collision arrives later by `(1 - 1/j)^{-1/2}` (1.0328 at j = 16). This is the stored-walk analogue of
   the Brent-Pollard degree-1 local anti-collision discussed by Bernstein-Lange. Fit to
   `research/curve25519_tw_scaling_20261108/README.md` (twcj walk steps per sqrt(r)): walk(inf) = walk(16)/1.0328 =
   1.324 at both k = 28 and 32, which predicts j32 / j64 = 1.345 / 1.334 (k = 28, measured 1.351 / 1.329) and
   1.346 / 1.335 (k = 32, measured 1.344 / 1.343), all within 0.6 %; walk(inf) is 0.4 % below the K = 2 Poisson floor
   1.329. The law is first order: it under-predicts j = 4 (1.535 vs 1.86 measured, `curve25519_tw_jumps_20261108`).
2. *The target-free table-then-probe design has its own floor, 1.77, not 1.662.* With the table stopped at the
   rank-2 time `n2 = sqrt(rX)`, X ~ Gamma(2, 1), probes cost `E[r/(2 n2)] = sqrt(r) Gamma(3/2)/(2 Gamma(2)) = 0.443`,
   so table + probes = 1.329 + 0.443 = 1.772; truncating at T = 0.9 sqrt(r) gives 1.370 + 0.398 = 1.768 (flat in x, as
   round 35 measured; measured probes 0.365-0.393 at k = 24-28). Only a stored target (tw2, owner's ruling) reaches
   the K = 3 floor 1.662. The descent phase of the strict design therefore costs 6.4 % above the joint floor.

**Verdict on the candidate directions** [pcc:unverified]:

| direction | verdict | number it rests on |
|:--|:--|:--|
| linear algebra (SGE, filtering, block Lanczos) | dead: LA is about 10 row ops per run on the C = 2 arms and <= 1 % on proj-a | `row_ops_uncharged` 10.25 (engine_items §7); summary.md |
| large primes, partial relations, cycle-finding | dead as a lever: every extra large prime is an unknown, floor `Gamma(K+1/2)/Gamma(K)` rises; m = 2 cycles are collisions (SNC-3 filed) | OBSTRUCTION.md; SYNTHESIS-NC SNC-3 |
| multi-target reuse | excluded by the owner; tends to 1.0 of matched multi-target rho from above | STRUCTURAL-20261110 §1.1 |
| descent of the target | strict design at its own floor 1.77 (finding 2); below it only with the target stored (tw2) | snc-panel |
| factor-base size vs yield | exhausted: B, N, x grids flat (rounds 21, 26, 32, 35) | autolab README |
| Montgomery batching, x-only | GAE-neutral; in `bitop/v1` batching costs 3 mul > 1 inv; x-only needs the difference | SIBLING-POINTS §2; structure_vs_rho |
| 2-adic / cofactor | closed list plus `curve25519_miniature_descent_class_20261111` (both bases span both classes) | |
| algebraic-track pipeline | dominated at m <= 3 by enumeration on the same base (7.2 mults per base point vs 52,816 F4 mul-adds per target; F4 slope 2.2 > 1) | `curve25519_algebraic_oracle_search_20261107`; exponent study §6 |
| **fixed set-up, walk anti-collision, final check** | **open: the four records below** | |

---

```yaml
id: W5-PL-1-recycle-setup-points
title: Store every set-up point (doubling ladders, jump partial sums, jumps) in the collision table
obstruction: >
  Set-up points are computed, charged and thrown away. At k = 20 the non-walk share is 0.50 sqrt(r) for twcj-cv0 and about
  0.79 for tw2cj-lazy; the wwalk inc draws gamma_j uniform in [1, r) per jump (plugins.rs:2662, ic_boundary.rs:9251) and
  evaluates it by popcount over [2^i]G (eval, ic_boundary.rs:9395), "a few hundred group additions" (OBSTRUCTION.md).
  OBSTRUCTION.md calls set-up 1-2 % at k >= 28, which is true, and not worth a round there; at k = 20-24 it is the largest
  non-walk cost. [pcc:unverified]
claim: >
  On every stored-walk arm (wwalk g=1 grow=2, tw=1/2 with cj=1), inserting each distinct set-up point lying in <G> with its
  coefficient vector into the walk table (rows with zero seed part still dropped) lowers mean counted GAE per solve by
  rho_s x (set-up GAE of the same arm), with rho_s in [0.80, 1.00], at unchanged walk RNG. For tw arms the saving is
  pathwise: no run costs more at w = 0. [pcc:unverified]
mechanism: >
  The K-th independent collision is a count of pairs among stored points, so m extra points with distinct coefficient
  vectors substitute for m walk steps. For a pair with coefficient difference D, D_seed != 0 mod r, the probability over
  uniform seeds that <D, theta> = 0 mod r is exactly 1/r, so set-up points are as good as walk points in expectation.
  Seed-free points (G-, Q-, K-chain) pair usefully only with seed-bearing points, a loss of O(m^2/n^2). Adding rows never
  lowers rank, so a tw run stops no later than its unrecycled twin. [pcc:unverified]
prediction: >
  Saving / own set-up GAE = 0.90 [0.80, 1.00] at every k. In ratios: tw2cj-lazy-rec / tw2cj-lazy 0.83 [0.76, 0.90] at
  k = 20, 0.95 [0.92, 0.98] at k = 24, null (+-1 %) at k = 28, 32; wwalk inc-lazy-rec / inc-lazy 0.80 [0.72, 0.88] at
  k = 20, 0.93 [0.89, 0.97] at k = 24. Strict-IC k = 20 ratchet: wwalk-lazy-rec below the box incumbent (2.703),
  ratio 0.80 [0.70, 0.92]. Arm B (recycling makes jumps free up to the n_K budget): j = 128 cj-chained / j = 16, both
  recycled, 0.98 [0.97, 0.995] at k = 20 by finding 1. [pcc:unverified]
discriminating_test: >
  Stage 0 (no group operations): read setup_ops and the set-up point list from committed snc-panel and engine_items
  records and the wwalk k = 20 configuration; count distinct <G> points; this fixes the predicted saving per arm before
  data. Stage 1: `rec=1` on TargetWalk / WalkTable (ic_boundary.rs) via WWalkOracle (plugins.rs), set-up inserts carry
  the `counted:table_insert_attempts` marker; replay of committed sessions with rec=0 byte-identical. Single-target
  ecbench, 8 sessions x 64 rounds per size at k = 20, 24 (plus a k = 28 null control), arms rho-neg, rho-neg-rec,
  inc-lazy, inc-lazy-rec, tw2cj-lazy, tw2cj-lazy-rec, box at k = 20. Minutes of CPU on a 4-core box.
falsification: >
  rho_s < 0.70 at k = 20 on either arm (set-up points are not worth walk points); any rec run dearer than its twin at
  w = 0 on a tw arm (pathwise claim false); or wwalk-lazy-rec / box upper 95 % end >= 1 at k = 20 (no strict-IC ratchet).
cost_band: >
  Every arm at w = 0, 0.1, 1, table inserts priced (default since 2026-11-08). rho-neg: canonicalisations_uncharged about
  1.1 sqrt(r) (k = 20). IC arms: table_insert_attempts (m more, m fewer walk inserts), row_ops, sqrt/Legendre/projected
  base items. Unlisted on every arm (erratum E8): per-step coefficient mod-r additions and rho's partition hash; the w > 0
  comparison stays a lower bound until they are listed. rho-neg-rec adds one table lookup per step (priced at w).
controls: >
  Reference parity arm rho-neg-rec (rho checking each step against a table of its own ~136 set-up points at k = 20): a
  derived first-hit model, survival exp(-tau^2 - 2 mu tau) with mu = 0.375, puts its walk at 0.69x and S near 1.20 at
  w = 0 but about 2.7 at w = 1 (one lookup per step), so the band order against rho-neg should flip and rho-neg stays the
  reference; report IC distance against both. A/A: rec=0 twin. Positive: Stage 0 count equals measured saving / 0.9.
risks: >
  Set-up composition of the k = 20 wwalk arm is estimated, not read (Stage 0 fixes it). Recycled points beyond n_K would
  be waste (not at these sizes). The k = 20 gain is a fixed-cost effect that vanishes as log r / sqrt(r); it does not move
  the K = 3 floor. Part of the gap closure against rho-neg at k = 20 is a stored-method effect a table-augmented rho shares.
closest_baseline: >
  Baby-step tables (the precomputed points are the table); Bernstein-Lange 2012 precomputed DP tables. Neither stores the
  jump-construction ladder of a collision walk. Lab: SYN-R3 (cheap jumps), cj chaining (removes set-up rather than
  recycling it), tw_jumps (jump count vs set-up). Nearest atlas ids: LEAD-ICB-koblitz-setup-phase-cost,
  LEAD-ICB-free-relations-self-collision.
gap_to_rho_after: "k = 20: tw2cj-lazy-rec about 2.10 = 1.34x rho-neg (about 1.75x rho-neg-rec at w = 0); k = 24 about 1.55x [pcc:unverified]"
not_relabelling: "Group operations performed fall; the recycled inserts are counted and band-priced; rows are verified as before."
lean_obligation: "rank monotonicity under added rows (Submodule.span_mono) and the 1/r counting lemma for a linear form with a unit coefficient over Z/r; neither written."
prior: "0.75 Stage 1 meets rho_s >= 0.8 at k = 20; 0.5 the strict-IC k = 20 ratchet holds across the band."
```

```yaml
id: W5-PL-2-self-sum-walk
title: Jump-free stored walk E_{t+1} = E_t + E_{t-h}, h uniform in [1, H], all coefficients mod r
obstruction: >
  Two costs of the fixed jump table at once: its set-up, and the first-order anti-collision 1/j of a j-adding walk
  (finding 1: 3.3 % of the walk at j = 16, the whole measured walk excess at k = 28, 32). Memoryless rho cannot drop the
  jump table; a stored walk can. [pcc:unverified]
claim: >
  Replacing the 16 fixed jumps by "add a random one of the last H stored points" (start set {P1, P2, G} for the strict
  design, {P1, P2, G, Q} for tw2) brings stored points at the K-th independent row to within 1 % of the Poisson floor
  (1.329 for rank 2, 1.662 for rank 3) and removes the jump set-up, at one group addition per stored point.
mechanism: >
  The repeated-test event needs the new point's partner to be the same element as an earlier step's partner, which in a
  self-sum walk happens only for pairs within H steps: a fraction O(H/n) instead of 1/j. The 1/r pair lemma of PL-1 holds
  for distinct coefficient vectors mod r; coefficient growth is irrelevant mod r (r < 2^32 at k <= 32). [pcc:unverified]
prediction: >
  Stage 0: simulated stored points to rank K / floor = 1.000-1.006 for H in {16, 64}, against (1 - 1/j)^{-1/2} +- 0.5 %
  for j-adding walks with j in {16, 32, 64, 128}. Stage 1, S2W / incumbent at w = 0: tw2cj-lazy 0.78 [0.70, 0.86] (k = 20),
  0.92 [0.88, 0.96] (24), 0.95 [0.93, 0.98] (28), 0.96 [0.94, 0.985] (32); strict inc-lazy 0.76 [0.68, 0.86] (20),
  0.90 [0.86, 0.95] (24), 0.956 [0.93, 0.98] (28), 0.97 [0.95, 0.99] (32). [pcc:unverified]
discriminating_test: >
  Stage 0 (zero group operations, native): extend src/bin/nc2_stage0.rs with a coefficient-only walk (random seeds theta
  mod r, collisions decided by <V, theta>), 1e4 walks per cell on the r of the k = 20, 24, 28, 32 family curves.
  Stage 1 only if Stage 0 passes: `walk=selfsum,H=16` in TargetWalk/WalkTable, single-target ecbench panel as PL-1
  (5 fresh curves per size, 12 targets x 8 rounds as the snc-panel protocol), rho-neg in every session. Hours of CPU.
falsification: >
  Stage 0 S2W above 1.015 x floor at H = 16, or j-adding cells off the law by more than 1 % (mechanism wrong); Stage 1
  tw2cj-lazy-s2w / tw2cj-lazy upper end >= 1 at k = 28 and 32 together; or stored entries per sqrt(r) above 1.68 at k = 32.
cost_band: >
  As PL-1, w = 0, 0.1, 1, inserts priced. Per-step native work: 3-4 coefficient mod-r additions (the current walk already
  does 2-4, unlisted on every arm, erratum E8) plus one RNG draw; list them on every arm, rho-neg included, before a
  rank is claimed at w = 1.
controls: "j = 16 incumbent twin; A/A; rho-neg; S2W with H = 1 (degenerate Fibonacci recurrence, expected to fail by Pisano-period repeats) as a negative control."
risks: >
  Early small-coefficient duplicates (identical vectors in the first ~30 steps) waste steps: about 1 % at k = 20, measured
  in Stage 0. Higher-degree anti-collisions (Bernstein-Lange) are not modelled. Exponential coefficient growth requires
  mod-r coefficient fields in TargetWalk (today u < 2^32 is assumed by wwalk_plan_check). PL-1 and PL-2 overlap at k <= 24:
  S2W has no set-up left to recycle, so their gains do not add.
closest_baseline: >
  Teske r-adding and mixed walks; Brent-Pollard local anti-collision heuristic; Bernstein-Lange "Two grumpy giants and a
  baby" (ia.cr/2012/294): anti-collisions make rho walks less random. All concern memoryless walks with a fixed jump set;
  here the jump set is the walk's own history. Nearest atlas id: LEAD-RHO-r-adding-walk-mixing (rho side).
gap_to_rho_after: "k = 32: tw2 about 1.69 = 1.71x rho-neg, strict inc about 1.85 = 1.87x; floor ratios 1.68x / 1.79x of measured rho-neg [pcc:unverified]"
not_relabelling: "Fewer stored points per row and no set-up; same per-point cost; rows verified as before."
lean_obligation: "the 1/r lemma (shared with PL-1) and the repeat-event lemma for j-adding walks: P(same jump multiset for the shifted pair) = 1/j; neither written."
prior: "0.6 Stage 0 passes; 0.45 a resolved gain at k = 28 and 32; 0.6 at k = 20."
```

```yaml
id: W5-PL-3-check-from-stored-ladder
title: Final [d]G = Q from a stored [2^i]G ladder (NAF, negation free) instead of g.mul from scratch
obstruction: >
  The relation loop's recovery check calls g.mul(generator, d) (ic_boundary.rs:13302), rebuilding about log2 r doublings
  that the oracle's eval tables ([2^i]G, ic_boundary.rs:9395) already hold. [pcc:unverified]
claim: >
  Passing the oracle's G ladder (or building it first and storing it as recycled points, PL-1) cuts the check to
  NAF-weight(d) additions, saving about 16-19 GAE at k = 20 (0.045-0.052 sqrt(r)) and 20-23 at k = 24, with the walk
  unchanged. [pcc:unverified]
mechanism: "[d]G = sum over signed NAF digits of +-[2^i]G; negation is free in GAE; the ladder is already paid for."
prediction: >
  check-ladder / incumbent = 0.981 [0.975, 0.987] (tw2cj-lazy, k = 20), 0.993 [0.990, 0.996] (k = 24); identical walks,
  rows and logarithms on every paired run, so the paired interval is as tight as SNC-1's (0.9665 [0.966, 0.967]).
discriminating_test: >
  An optional fixed-base table handed from the oracle to the relation loop; replay of committed sessions without it
  byte-identical; 8 single-target sessions x 64 rounds at k = 20, 24; difference per run must equal the recorded check
  group_ops minus (NAF weight - 1). Minutes.
falsification: "Any paired-run difference other than the predicted check delta, or a mean saving below 10 GAE at k = 20."
cost_band: "Every arm at w = 0, 0.1, 1; rho-neg-ladder arm (rho reusing its own jump ladder for its check, 0.065 sqrt(r) today) priced the same way."
controls: "rho-neg-ladder as reference parity: both arms fall by about the same GAE, so the distance moves little; A/A."
risks: "Small (2 % at k = 20, under 1 % above); conflicts with PL-2, which has no ladder unless one is built and stored."
closest_baseline: "Fixed-base comb and NAF scalar multiplication (textbook); lab SNC-1 lazy certification (removes per-row recomputation, keeps this check)."
gap_to_rho_after: "unchanged within 0.5 % once rho-neg-ladder is adopted as reference [pcc:unverified]"
not_relabelling: "Same check, fewer operations; nothing moved to unpriced."
lean_obligation: "[d]G equals the signed-digit sum of [2^i]G (nsmul_add and zsmul over the NAF expansion); not written."
prior: "0.85."
```

```yaml
id: W5-PL-4-jump-count-at-the-law-optimum
title: Jump count j*(k) from the anti-collision law; configuration only on the existing binary
obstruction: "Fixed j = 16 everywhere; finding 1 says j* rises as r^(1/4) without recycling and is budget-limited with it."
claim: >
  S(j) = walk(inf) (1 - 1/j)^{-1/2} + c j (c = 3.5-4.8 GAE per jump, measured) gives j* = sqrt(walk(inf) sqrt(r) / (2c))
  = 8 / 16 / 30 / 57 at k = 20 / 24 / 28 / 32; at k = 32, j = 64 costs 0.983 [0.975, 0.992] of j = 16 at w = 0.
mechanism: "Finding 1 plus the measured marginal set-up per jump (tw_scaling: 4.31 GAE at k = 28, 4.8 at k = 32)."
prediction: "k = 32 j64/j16 0.983 [0.975, 0.992]; k = 28 1.000 +- 0.01; walk ratios on the law within 0.6 %. [pcc:unverified]"
discriminating_test: >
  No code: `jumps=64` vs `jumps=16` on twcj-cv0 and tw2cj-lazy at k = 32, enough single-target sessions for a 0.5 %
  half-width (about 32 sessions x 64 rounds; the k = 32 runs are cheap). Doubles as an on-curve check of PL-2's mechanism.
falsification: "j64/j16 upper end >= 1 at k = 32, or any walk ratio off the law by more than 1 %."
cost_band: "Every arm at w = 0, 0.1, 1, inserts priced; rho-neg in each session."
controls: "j = 32 middle arm; A/A."
risks: "Effect at the noise floor; moot if PL-2 lands (S2W removes the same 1/j)."
closest_baseline: "research/curve25519_tw_scaling_20261108 (j16/j32/j64, underpowered: 0.998 and 0.990) and curve25519_tw_jumps_20261108; atlas LEAD-RHO-r-adding-walk-mixing."
gap_to_rho_after: "k = 32 about 1.38x rho-neg for twcj (boundary), about 1.75x for tw2cj-lazy [pcc:unverified]"
not_relabelling: "Configuration of an existing walk; set-up of extra jumps is charged."
lean_obligation: "none beyond PL-2's repeat-event lemma."
prior: "0.4 for a resolved gain."
```

## Ranked by expected measured improvement per unit cost

Expected improvement = prior x predicted point gain on the strict-IC incumbent at the size where it is largest
(derived, [pcc:unverified]); cost = engineering plus CPU on a local 4-core box.

| rank | lead | largest gain (k) | gain at k = 32 | prior | cost | why this place |
|--:|:--|:--|:--|--:|:--|:--|
| 1 | PL-1 recycle set-up | 20 % (20), 7 % (24) | none | 0.75 | about a day of Rust, minutes of runs | pathwise for tw arms; the only candidate for a strict-IC k = 20 ratchet over the box |
| 2 | PL-3 check from ladder | 2 % (20) | none | 0.85 | hours | tiny but resolvable on paired-identical runs |
| 3 | PL-2 self-sum walk | 24 % (20), 4 % (28, 32) | 3-4 % | 0.45-0.6 | 2-3 days, Stage 0 first | the only lever left at k >= 28; overlaps PL-1 at k <= 24 (not additive) |
| 4 | PL-4 jump count | 1.7 % (32) | 1.7 % | 0.4 | configuration only | mechanism check for PL-2; moot if PL-2 lands |

After all four, the generic C = 2 track sits at its floor at k >= 28 (1.662 with tw2, 1.77 strict), about 1.7x-1.8x
measured rho-neg; what remains is the per-point price (SIBLING-POINTS, filed) and the owner's tw2 ruling (6.4 %)
[pcc:unverified]. Nothing here changes an exponent; all four are engineering-class.

## Sources checked

- Contract: `AGENTS.md` (Success criterion; Tracks; Preserve scope; Name the obstruction; No Python; boundary/table/ratio,
  §5a). Board: `docs/ic/LEADERBOARD.md` (lines 1-334; no Curve25519 row found by grep). `docs/ic/leaderboard.json` not read.
- Curve25519 studies: `research/curve25519_ic_baseline_20261102/README.md` and `analysis/summary.md`;
  `curve25519_ic_ratchet_20261102`, `curve25519_t10_winner_ecbench_20261104`, `curve25519_f1_targetwalk_20261108`,
  `curve25519_rho_parity_bound_20261104`, `curve25519_structure_vs_rho_20261107`, `curve25519_algebraic_oracle_search_20261107`,
  `curve25519_exponent_study_20261108`, `curve25519_exponent_study_ext_20261110` (lines 1-90), `curve25519_engine_items_20261110`,
  `curve25519_tw_scaling_20261108` (phases), `curve25519_tw_jumps_20261108` (lines 1-40), `curve25519_miniature_descent_class_20261111`,
  `curve25519_f4_trace_20261102` (grep), all `README.md`.
  `research/curve25519_ic_coverage_20261108/README.md` does not exist on disk.
- Autolab: `research/curve25519_autolab_20261104/README.md`, `snc-panel/README.md`, `snc-panel/data/diagnostics.json` (head),
  `ideas/OBSTRUCTION.md`, `ideas/STRUCTURAL-20261110.md`, `ideas/SIBLING-POINTS-20261110.md`,
  `ideas/SYNTHESIS-NC-20261108.md` (SNC-3), idea-record ids in `ideas/*.md` (grep), `round-031/README.md` (k = 20 rows).
- Code: `src/cryptanalysis/ic_framework/plugins.rs` (wwalk parameters 2659-2670, WSumOracle 2380-2500),
  `src/cryptanalysis/ic_boundary.rs` (IncrementalGauss 1087-1180, WalkTable jump draw 9234-9262, eval 9395-9417 and
  10955-10980, relation loop and recovery check 13225-13312), `src/cryptanalysis/ic_framework/mod.rs` (completion rule
  555-662); `src/bin/nc2_stage0.rs` exists (not read).
- Today's lanes: `research/leads_20261112/README.md`, `research/leads_20261112/atlas_lead_ids.txt`,
  `research/curve25519_leads_wave5_20261112/lanes/real_parameters.md` (no overlap: those are m = 3 algebraic-engine items).
- Not available in this lane: research-kb (no tool), `tools/lead_atlas.py` (no shell; atlas bodies unread, only ids),
  ecdb (not needed: every curve fact is quoted from the files above), Conductor.
- Literature: Bernstein, Lange, "Two grumpy giants and a baby", ANTS-X 2012, [ia.cr/2012/294](https://ia.cr/2012/294)
  (search-result abstract only: degree-1 local anti-collisions of Brent-Pollard, higher-degree anti-collisions). Teske's
  r-adding and mixed walks and Brent-Pollard 1981 are cited from memory, not fetched; a literature check of "jump-free
  self-sum walk" and "recycled set-up points" is owed before any task card.
