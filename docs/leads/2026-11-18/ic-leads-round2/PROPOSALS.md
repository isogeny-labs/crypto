# k-sum IC pipeline: round-2 leads (Idea Generator, 2026-11-18)

Status: **proposals only.** Nothing here is a measurement, a result, a status change or an assignment. Every number is one of: *quoted* (path given, `[pcc:unverified]` as in its source), *derived* (arithmetic on quoted numbers, not run), or *predicted* (preregistered here, to be frozen by the Coordinator before any run). I had read, grep and web tools only: no run, no `tools/lessons.py brief` (no shell in this session), no research-kb (unavailable), no edit outside this file. Deliverables are Rust or spec-only; no Python is proposed.

## 0. Anchors (quoted)

Source: `research/ic_ksum_sibling_20261118/README.md`, `band-P24.txt`, `sessions/P24/records.jsonl`, `specs/P24.json`. P24: p = 10,935,329, r = 10,933,753, sqrt(r) = 3,306.6, base `prime-abscissa:size=11` (22 points, 11 abscissae), oracle `ksum:m=5,sib=1`.

| quantity | value | where |
|:--|--:|:--|
| table entries (insert attempts) | 7,392 = 2^4 C(11,5) | records, `table_insert_attempts` |
| build adds, sib / non-sib | 7,106 / 10,802 | records `adds` (setup phase), README |
| relation phase | 8,540 trials, 12 relations, 9,019 adds, 876 doubles, 40 scalar mults | records |
| S(w = 0 / 0.1 / 1) ksum5-sib, ksum5, rho-neg | 5.173 / 6.041 / 13.853; 6.290 / 7.047 / 13.853; 1.133 / 1.235 / 2.154 | `band-P24.txt` |
| `prune=1` alone, P24 | S 6.38 -> 5.98 (w = 0), 13.73 at w = 1; "build adds saved 12 %" | `docs/ic/LEADERBOARD.md` line 337-339 (validated, VALIDATION.json named there) |

Derived decomposition of one P24 solve (not in any record as such): table = 7,106 adds (3,696 last-level sibling pairs + 3,410 partial-sum adds at depths 2-4); walk set-up = 40 scalar mults = 876 doubles + about 480 adds = about 1.35k GAE (8 % of 17.1k); trials 8,540 at about 1.02 GAE each at w = 0, 1.22 at w = 0.1, 3.07 at w = 1 (the w = 1 total 45.8k minus table 18.2k minus set-up, over 8,540; includes one lookup and one walk mix hash per trial). Model hit rate p = 2E/(r - 1) for E distinct entries: 1/p = 739.6 trials per relation; observed 8,540/12 = 711.7 (one run). **Unit-test fact** (`ic_boundary.rs` `ksum_wide_index_tests` / `pruned_build_is_the_same_table_with_fewer_adds`): `prune` and `sib` produce the same table, so they stack, but no record measures them together.

Floor, for orientation (quoted from the brief): generic table-plus-walk cost is at least about 2 sqrt((F+1) r / 2) = 16.2k at F = 11, in **stored points at one add each**. Three of the proposals below lower the *price per stored point* (as `sib` did), which a point-count floor does not forbid; none lowers the point floor. Pricing at w = 1 charges inserts and lookups, which brings the floor back for them.

## 1. Ranking and the one-line verdict

Ranked by (expected effect) / (cost of the cheapest discriminating test). "Wall" = can it beat the generic sqrt(r) wall; **none of 1-5 can**. [pcc:unverified]

| # | id | lever | w = 0 effect (P24, predicted) | survives w = 1 | cheapest test | beats sqrt(r) wall |
|--:|:--|:--|:--|:--|:--|:--|
| 1 | L2-FEWCOL | unknown columns 11 -> 2-3 (`wsum` C = 3, `wwalk` C = 2) on Table B | S 4.78 -> 2.3 to 3.4 (0.48-0.71) | yes, predicted 0.25-0.5 | spec only, about 20 min | no: constant, BSGS-class, floor 1.5x rho at C = 1 |
| 2 | L2-STACK | `prune=1` + `sib=1` together | 4.78 vs 5.98 pruned incumbent (0.80) | prune part only (0.98), sib ties | spec only, about 10 min | no: constant |
| 3 | L2-FREE | `free=1` table collisions as rows, base size 11 and 12 | 0.78-0.97 of L2-STACK | yes, about equal | spec only, about 15 min | no: moves along the birthday floor |
| 4 | L2-INNER | keep the depth < k partial sums as table entries (`inner=1`) | 0.88 of L2-STACK | yes, 0.92 | about 40 lines Rust, about 1 h | no: price per point |
| 5 | L2-WSIB | x-only siblings T +- J in the target walk | 0.75 of L2-STACK | partial, 0.89-1.0 | native stage-0 probe about 1 h | no: price per probe |
| 6 | L2-HRATE | hit-rate excess census on records already on disk | none (diagnostic) | n/a | about 15 min, records only | **only candidate that could**; prior at most 3 % |

The new reference for 3-5 is `ksum:m=5,prune=1,sib=1` ("ksum5-ps"), derived S(0) = 4.78 at P24; L2-STACK measures it.

## 2. Records

```yaml
id: L2-FEWCOL
title: "Transfer the C = 2/3 column arms (wsum, wwalk g=1 grow=2) from the Curve25519 autolab to Table B"
obstruction_attacked: >
  The k-sum pipeline needs F+1 = 12 independent relations (11 unknown base logs plus d), each costing r/(2E) = 740
  trials, so the relation phase is 12x what a one-relation method pays. problems/prime-divisor-uniform-solver.md is the
  non-generic obstruction (no fixed-base relation supply); this lead does not touch it. It attacks the *generic*
  multiplier (F+1).
claim: >
  On Table B curves P20/P22/P24 (48 public targets, 2 rounds, L0, counted), `prime-abscissa:size=2` with
  `wwalk:x=0.9,g=1,grow=2,seed=run` (and `targets=stride-generator` as in autolab round 33) costs at most 0.60 of
  `ksum5-ps` at w = 0 and 0.1 and at most 0.55 at w = 1, every solve verified by [d]G = Q.
mechanism: >
  Same plugins, fewer unknowns: with C = 2 seed columns and known-log G jumps one hit solves (rank-2 rows), so the
  floor is Gamma(F+3/2)/(Gamma(F+1) Gamma(3/2)) x rho at F = 2 (1.88x), against about 4.4x at F = 11. The autolab measured
  this family on the Curve25519 miniatures: S 2.11 at k = 24 seed=run (1.94x matched rho), `wsum` C = 3 3.68, C = 2 3.32
  (research/curve25519_autolab_20261104/README.md rounds 20, 24, 31, 33, re-baseline). Table B has never been run with
  these oracles: grep of docs/ic/LEADERBOARD.md and research/ic_tableb*, research/ic_ksum_sibling_20261118 finds no wsum
  or wwalk arm.
prediction:
  S_w0_P24: {ksum5_ps_derived: 4.78, wsum_C3_B10: "3.4 [2.9, 4.0]", wwalk_C2_x09_seedrun: "2.3 [2.0, 2.7]", rho_neg: 1.133}
  ratio_to_ksum5_ps: {wsum_C3: "0.71 [0.60, 0.84]", wwalk_C2: "0.48 [0.42, 0.57]"}
  distance_to_rho_w0: "4.2x -> 2.0x [1.8, 2.4] (wwalk), 3.0x (wsum)"
  w_1: "wwalk ratio to ksum5-ps 0.35 [0.25, 0.50] (table inserts about 0.9 sqrt(r) priced at w; autolab: order held across the band)"
  same_direction_at_20_and_22_bits: "ratios within +-0.12 of the 24-bit ratio"
discriminating_test: >
  Step 0 (2 min): one-target dry run of each arm on P24 to confirm `prime_explicit` accepts the strings (the autolab used
  `prime-projected-abscissa`; the risk is a parameter rejection, not a result). Step 1: copy research/ic_ksum_sibling_20261118/specs/P24.json
  (new directory, frozen copy), arms {rho-neg, ksum5 (A), ksum5-ps (baseline), wsum3-b8, wsum3-b10, wsum3-b12,
  wwalk2-x09-sr, ksum5-ps-aa}, `ecbench run --cpus none --allow-busy`, `ecbench compare`, `ecbench band`. Runtime: each solve is
  10k-17k GAE, 48 x 2 x 8 arms is minutes on 4 cores.
falsification: >
  wwalk2 ratio to ksum5-ps above 0.75 at w = 0 on P24 with the A/A at 1, or any unverified solve, or order not holding at
  w = 0.1 and 1, or a table-draw component larger than the effect (re-run 16 seeds as autolab R4-I1 requires: the fixed-table
  figure alone is not evidence).
cost_band: >
  All arms at w = 0, 0.1, 1 via `ecbench band`: table inserts and lookups priced (default since 2026-11-08), walk set-up
  scalar mults counted, rho-neg priced the same way (S 1.133 / 1.235 / 2.154).
controls: [ksum5-ps-aa (A/A), rho-neg, wwalk with seed fixed vs seed=run (R4-I1 table-draw control), P20 and P22]
risks:
  - "Classification: autolab labels C = 2 and C = 1 rows 'BSGS boundary, pending an owner ruling'. The success criterion counts them only if the owner does; report the column count C in every row."
  - "Not evidence of a non-generic source: it is the generic floor family at fewer unknowns."
  - "Seed-averaging: report seed=run means, not a fixed draw."
closest_baseline: "research/curve25519_autolab_20261104 rounds 16-33 and re-baseline (S 2.11, 1.94x rho at k = 24); research/ic_c1_headroom_20261115 (C = 1 floor 1.5x rho, remaining headroom about 10 %); published: Bernstein-Lange, Two grumpy giants and a baby (ANTS X, 2012), eprint 2012/294. Different here: prime_explicit Table B curves with the prime-abscissa base, measured against the k = 5 baseline the sibling study left at 4.6-5.3x rho."
beats_wall: "no. At C = 1 the floor is 1.5x rho; at C = 2 about 1.9x."
lean_obligation: "none new (no new mathematical claim; rows are accepted only after the final [d]G = Q check)."
```

```yaml
id: L2-STACK
title: "ksum:m=5,prune=1,sib=1: the two validated build savings measured together"
obstruction_attacked: "Dead-end partial sums (prune) and a second inversion per last-level pair (sib) are independent build wastes; no record measures them stacked, so the incumbent for every later lead is unknown."
claim: >
  On P20/P22/P24, `ksum:m=5,prune=1,sib=1` has build adds equal to the sib build minus the prune saving, exactly
  (P24: 7,106 - 1,338 = 5,768 +- 40), the same trials, relations and keys as `ksum:m=5`, and S(0) = 4.78 [4.70, 4.85] at P24.
mechanism: >
  prune acts at depths 2-4 (skips partials that cannot reach 5 summands: 1,338 of 3,410 adds at F = 11), sib acts at depth 5
  (3,696 pairs, one inversion each). Disjoint sets of adds. The unit test already shows both leave the table unchanged.
prediction:
  build_adds_P24: "5,768 [5,730, 5,810]"
  S_P24_w0_w01_w1: "4.78 / 5.65 / 13.46 (derived: sib 5.173 / 6.041 / 13.853 minus 1,338 GAE / 3,306.6 at every w)"
  ratio_to_pruned_incumbent: "0.80 [0.79, 0.81] at w = 0 (4.78 / 5.98); 0.84 at w = 0.1; 0.98 at w = 1"
  distance_to_rho_P24_w0: "5.5x (pruned incumbent) -> 4.2x"
discriminating_test: "Spec only: specs/P24.json with arms ksum5, ksum5-prune, ksum5-sib, ksum5-ps, ksum5-ps-aa, rho-neg; P20, P22 likewise. Identity check: per-target trials and relations equal across all four ksum arms."
falsification: "Build adds differ from sib-minus-prune by more than 1 %, any trial/relation mismatch across the arms, or ratio to ksum5-prune at w = 0 above 0.88 at any size."
cost_band: "Every arm at w = 0, 0.1, 1; sibling_second_points priced at w, table inserts priced at w, as in band-P24.txt."
controls: [A/A, ksum5 unpruned, rho-neg]
risks:
  - "At w = 1 the sib part ties by pricing (README P-c failure); only the prune part (about 0.98) survives. Not an improvement across the band beyond that."
  - "Pure stacking: report as a baseline refresh, not a discovery."
closest_baseline: "docs/ic/LEADERBOARD.md section 'k-sum table build without dead-end partial sums' (prune, validated); research/ic_ksum_sibling_20261118 (sib, unvalidated)."
beats_wall: "no."
lean_obligation: "none."
```

```yaml
id: L2-FREE
title: "free=1 on the stacked Table B arm, base sizes 11 and 12: spend the build's own key collisions as relations"
obstruction_attacked: "Of the 12 relations a solve needs, all 12 are bought by walk trials at about 740 each (8,540 trials, 10.1k of 17.3k GAE). A table of E entries contains about E^2/(r - 1) key collisions for free."
claim: >
  On P24 with `ksum:m=5,prune=1,sib=1,free=1` (F = 11), the table yields D = 5.0 expected collisions (C(7392,2)/((r - 1)/2));
  if their rows have rank >= 3, mean trials per solve fall from 8,540 to at most 6,400 and S(0) to at most 0.90 of L2-STACK.
  At F = 12 (E = 12,672, D = 14.7) the solve needs only 2 probe hits and S(0) <= 4.8.
mechanism: >
  Two distinct signed 5-sums with equal abscissa satisfy S = +-S', a verified homogeneous row among the base logs (`KSumTable::free_rows`,
  8 adds per row, `Relation::Row`). The system has 12 unknowns, so r_f free rows (rank <= F - 1 = 10) leave 12 - r_f probe
  relations, at least 2. The plugin exists (`KSumOracle` param `free`); it has been run only on Curve25519 projected bases
  (autolab rounds 13-15: "no 4-sum collisions", "90 free rows per run but low rank: colliding sums share summands"). Table B prime-abscissa k = 5 has never been run with it.
prediction:
  D_per_curve: "P20 4.1, P22 3.9, P24 5.0 expected (Poisson; D is one fixed number per curve and base, since the table is identical for all 48 targets)"
  rank_over_D: "0.6-1.0 (autolab round 15 warns of low rank; F = 11 with k = 5 forces overlaps of about 2.3 summands, so I do not expect the rank to be as bad as at k = 4 with large F)"
  trials_P24: "6,400 [5,000, 7,500] against 8,540"
  S_P24_w0: "4.3 [3.9, 4.8] at F = 11 (0.90 of 4.78); F = 12: 4.0 [3.7, 4.8]"
  w_1: "0.88 [0.78, 0.97] of L2-STACK (a saved trial is worth 3.07 GAE)"
discriminating_test: >
  Two-stage. Stage A (preflight, minutes, no comparison): ecbench run of one arm per curve; read `free_collisions_checked`,
  `free_rows_emitted` from `relation_counters` and the recovered rank (rows accepted vs trials needed). Pool at least 8 fresh
  24-bit curves (one fixed D per curve). Stage B: panel {ksum5-ps, ksum5-ps-free F=11, F=12, ksum5-ps-aa, rho-neg}.
falsification: >
  Stage A: pooled sum D / sum E outside [0.7, 1.4] (collisions not random, or an indexing bug), mean accepted free rows
  below 2, or any row failing verification. Stage B: ratio to ksum5-ps above 0.97 at w = 0, or A/A not 1.
cost_band: "Arms priced at w = 0, 0.1, 1; free-row verification (8 adds per collision, counted); the larger F = 12 table pays its inserts at w."
controls: [shuffled-key control: permute the key -> entry map after build, every row must fail the group check; A/A; rho-neg]
risks:
  - "D is deterministic per curve and base: variance across curves is Poisson(5), so a single curve can show D = 1 or 10. Pool curves."
  - "Low rank (autolab round 15) is the known failure."
closest_baseline: "research/curve25519_autolab_20261104 rounds 13-15 and ideas/PROPOSALS.md P2 (same mechanism, Curve25519 projected base); published: none for IC (birthday collisions as relations are the mechanism behind rho and BSGS). Different here: the prime-abscissa Table B base at k = 5 and a stacked, cheaper table."
beats_wall: "no. It substitutes table-table collisions for table-walk collisions along the same birthday floor, which at F = 11 is about 4.4x rho in points."
lean_obligation: "none new. The row soundness lemma (equal abscissa implies S = +-S', so the signed sum of coefficients is zero mod r) is the one `free_rows` already checks by recomputation."
```

```yaml
id: L2-INNER
title: "inner=1: keep the depth < k partial sums as table entries (they are already computed)"
obstruction_attacked: "The build adds 2,072 partial sums at depths 2-4 (after prune) and discards their keys; a hit on any of them is a valid, shorter relation. Entries per add is 1.04 today; a point-count floor assumes 1."
claim: >
  On P24 with `ksum:m=5,prune=1,sib=1,inner=1`, the table holds 7,392 + 2,079 = 9,471 entries for the same 5,768 adds, trials per
  relation fall by 0.78 +- 0.04 (model 7,392/9,471) and S(0) is at most 0.92 of L2-STACK.
mechanism: >
  Insert x(S) of every partial sum at depth 1-4 under the negation fold as well as the leaves. `probe` returns a shorter
  summand list; `Relation::Summands` already takes any length. Leaf-level `sib` is unchanged. Keep `prune=1`: the 1,338 dead-end partials cost 1 add per
  entry, which is worth about 0.8 trials at the margin (derived: 12 x 5.47e6 / E^2 x 1.02 at E = 9.5k), below their cost at w = 0.
  The hit rate is linear in entries, which L2-HRATE tests first. Autolab ideas/PROPOSALS.md P2 listed `inner=1` as an
  ablation (+9-25 % entries at m = 4); it is not in the plugin's parameter list (plugins.rs line 2396-2402) and no record uses it. At k = 5 and F = 11 it is +28 %.
prediction:
  entries_P24: "9,471 exactly (7 + 56 + 336 + 1,680 partials with enough abscissae left, plus 7,392 leaves)"
  trials_P24: "6,660 [6,200, 7,200] against 8,540 (calibrated model: 12 x 739.6 x 0.78 x 0.962)"
  S_P24: "w = 0: 4.19 (0.88 [0.84, 0.92] of L2-STACK); w = 0.1: 0.89 [0.85, 0.93]; w = 1: 0.92 [0.88, 0.95] (saved trials 1.88k x 3.07 = 5.8k against 2.1k extra inserts)"
  distance_to_rho_w0: "4.2x -> 3.7x"
discriminating_test: >
  About 40 lines in `KSumTable::extend` (insert at depth < k, pad unused packed fields with 4095, which is not a valid index because the
  table refuses bases of 4096 or more points; teach `unpack` and `free_rows` to stop at the pad) and a unit test in the style of
  `pruned_build_is_the_same_table_with_fewer_adds`: with `inner=0` the map is bit-identical; with `inner=1` every old entry is present and every
  new key is verified by recomputing its summands. Then paired run on P24 {ksum5-ps, ksum5-ps-inner, A/A, rho-neg}. About 1 h including the build.
falsification: >
  Entries differ from 9,471, or trials fall by less than 12 % (hit rate not linear in entries), or ratio to ksum5-ps at w = 0.1 above 0.95,
  or any unverified solve, or any key mismatch in the identity test.
cost_band: "Inserts of the new entries priced at w (table_insert_attempts, +2,079); adds unchanged by construction; lookups as before."
controls: [A/A, inner=0 identity, rho-neg, base size 10 and 12 to see the optimum move]
risks:
  - "Short rows may be linearly dependent on earlier ones more often (a 2-sum hit is a rank-2 row); measure the fraction of accepted rows."
  - "Combines with L2-FREE (inner entries add collisions, D = 8.7 at E = 9,471): measure separately first."
closest_baseline: "KSumTable as built (all signed k-sums only); autolab ideas/PROPOSALS.md P2 ablation `inner=1` (proposed, never implemented); published: Schroeppel-Shamir style subset-sum tables store only the full k-sums, so the idea is a bookkeeping change, not a published method."
beats_wall: "no. It lowers the add price per stored point from 0.96 to 0.61; the entry count that decides the floor in points still pays 1 insert each at w = 1."
lean_obligation: "none new."
```

```yaml
id: L2-WSIB
title: "Sibling probes in the target walk: T + J and T - J from one inversion"
obstruction_attacked: "The relation phase buys 8,540 trials at one group add each. The table build already shares one inversion per +-P pair; the probe walk does not (the sibling walk exists only in `wwalk`, `ic_boundary.rs` lines 11051 and 11839, not in the `Targets` sources of the pipeline)."
claim: >
  A pipeline target source `walk-sib` that probes both x(T + J) and x(T - J) at every step (one `CountedGroup::sibling_pair`, charged one add plus one
  `sibling_second_points`, the table build's convention) needs 0.55 [0.50, 0.60] of the steps for the same 12 relations, with
  the same hit rate per probe (2E/(r - 1) within 5 %) and independent rows.
mechanism: >
  Probes are the cost; an x-only sibling is a second valid probe with known coefficients (a - aj, b - bj mod r) for one extra squaring and two
  additions. Waste: T - J equals the previous point when J is the previous jump, probability 1/16 with 16 jumps. Rows from the two probes of one step are
  independent (their difference is the fixed 2J relation, which involves no table column).
prediction:
  probes_to_12_relations_P24: "8,540 +- 5 % unchanged (stage 0); steps 4,550 [4,300, 4,800]"
  S_P24: "w = 0: 0.75 [0.70, 0.80] of L2-STACK (4.0k adds saved); w = 0.1: 0.80; w = 1: 0.89 [0.82, 1.00] (second point priced 1 GAE, offset by one shared mix hash per step; the decomposition of the 3.07 GAE per trial at w = 1 between hash, lookup and mod-r adds is derived, not recorded)"
  distance_to_rho_w0: "4.2x -> 3.2x"
discriminating_test: >
  Stage 0 (native Rust probe, no pipeline change, about 1 h): build the P24 table with the existing `KSumTable`, run the existing walk and the
  sibling walk over 96 independent starts each, count probes per hit and per rank-increase; record hits per probe for the +J and -J halves
  separately and the rank of rows from the two halves. Stage 1 (only if Stage 0 holds): `Targets` variant (about 120 lines) and a paired
  ecbench run.
falsification: >
  Hits per probe for the -J half differ from the +J half or from 2E/(r - 1) by more than 8 %, or the step count falls by less than 35 %, or the
  w = 0.1 ratio is above 0.90.
cost_band: "sibling_second_points priced at w (and, if lazy coefficients are not used, two extra mod-r adds per step); lookups and hashes as in the incumbent."
controls: [A/A, the non-sibling walk on the same targets, rho-neg]
risks:
  - "w = 1 may tie, as P2 did; the claim is stated for w <= 0.1 and w = 1 is reported as measured."
  - "Autolab SIBLING-POINTS-20261110.md has a stage-0 on the Poisson law for the stored-table walk (`wwalk sib=m`); the probe-side variant is not covered there."
closest_baseline: "wwalk sib=m (ic_boundary.rs 11051-11996) and research/curve25519_autolab_20261104/ideas/SIBLING-POINTS-20261110.md; ksum5-sib (P2, research/ic_ksum_sibling_20261118). Different here: the probe stream of the k-sum pipeline."
beats_wall: "no. It lowers the add price per probe (to 0.5 at w = 0), not the probe count."
lean_obligation: "none new; the x-only sibling identity is the one `sibling_pair` and the P2 unit test already exercise."
```

```yaml
id: L2-HRATE
title: "Hit-rate excess census: is the k-sum table hit rate exactly 2E/(r - 1)?"
obstruction_attacked: "Every proposal above and the generic floor assume that the probability a walk point's abscissa is in a table of E x-classes is 2E/(r - 1), i.e. that a 5-sum of prime-abscissa points is a random x-class. A non-generic relation source would show as an excess. The brief rules out the algebraic track (17x coverage gap); this is the cheap check of the generic side."
claim: >
  On the three Table B sessions on disk, H = (sum relations / sum trials) / (2E/(r - 1)) is 1.00 [0.94, 1.06] at each of P20, P22, P24
  (1,152 relations per curve, relative standard error 3 %). The single record read gives 12 / 8,540 against 1/739.6, H = 1.04.
mechanism: >
  Nothing is expected. The ksum5 and ksum5-sib arms make identical trials, so each curve has 96 + 96 + 96 identical records; use one arm. A real
  excess (H >= 1.15, stable across curves and base sizes) would mean x-classes of base-point sums are not equidistributed, for instance
  because small abscissae or prime-abscissa structure correlates with the group law.
prediction: "H in [0.94, 1.06] at all three curves, probability 0.97. H >= 1.15 at two curves, probability at most 0.03."
discriminating_test: >
  Read `trials`, `relations` and `table_insert_attempts` from `sessions/P20,P22,P24/records.jsonl` (ksum5-sib records), compute H per curve and per base-size
  stratum; native Rust or an existing `ecbench` summary, not Python; no run needed. If H is outside the band, stratify by number of table columns in the
  accepted row and by sign pattern before any further claim.
falsification: "H inside [0.94, 1.06] on all three curves closes the lead (it also confirms the hit-rate model L2-INNER and L2-FREE rely on). An H above 1.15 that disappears with seed=run or with a fresh base is a fixed-draw artefact, not a source."
cost_band: "None (records on disk); a follow-up census would run the same arms."
controls: [rho-neg trials per hit (no table), a ksum arm on a fresh base with the same E, a table-free sampler of E random x-classes]
risks:
  - "Dependent or repeated rows are discarded by the elimination, so H measured on `relations` underestimates the raw hit rate; use raw `lookups` hits if the record has them."
  - "Prior is low: structure of this kind is what the 17x algebraic coverage gap says is absent."
closest_baseline: "research/ic_c1_headroom_20261115 (coverage gap, 17x); docs/leads/2026-11-18/novel-decomposition-oracle (short-box lattice, active lane, not duplicated)."
beats_wall: "it is the only entry that could point at a non-generic source; it does not itself beat anything."
lean_obligation: "none."
```

## 3. Not proposed, with the reason

- **Cheaper walk jump pool** (40 scalar mults = 876 doubles + about 480 adds = 1.35k GAE, 8 % at w = 0): the stride and short-jump pools were falsified in autolab rounds 12, 29, 30 (clustering on short seed relations; 1.08x worse). A jump pool built from doubling chains has the same low-weight structure; I do not expect a different outcome and did not write a record.
- **Larger arity or smaller base at fixed table size**: F = 11 is the discrete optimum of E(F) + (F + 1) r/(2E(F)) (F = 10: 19k, F = 12: 18.3k, derived at 1 add per entry); L2-FREE and L2-INNER move that optimum and the tests include F = 12.
- **Weierstrass, isogeny, twist, endomorphism, descent and xedni leads**: closed in research/curve25519_autolab_20261104/ideas/NONCOLLISION-20261108.md section 1 (derived there); Table B's random curves have End = Z, so I found no new one.
- **Semaev/F4, the hashed network, curve transfer and hyperelliptic IC**: other lanes (PRs #4679, #4682, #4683, #4676).

## 4. Order of work I would suggest to the Coordinator

L2-STACK and L2-FEWCOL first (spec only, minutes, and they set the new reference and the biggest gap closure), then L2-FREE (spec only), L2-HRATE (records only, in parallel), then L2-INNER (Rust), then L2-WSIB (Rust, w = 1 uncertain). L2-INNER and L2-FREE should be preregistered with their stacked reference, not the old ksum5.

## 5. Sources checked

Files read: `AGENTS.md` (lines 479-557, 869-901, 1128-1186); `research/ic_ksum_sibling_20261118/{README.md, PROTOCOL.md, band-P24.txt, compare-P24-rho-neg-vs-ksum5-sib.txt, specs/P24.json}` and key counters from `sessions/P24/records.jsonl` (grep, not read in full); `research/ic_tableb_ksum_20261115/README.md`; `research/ic_c1_headroom_20261115/README.md`; `research/curve25519_autolab_20261104/README.md` (rounds table), `ideas/{PROPOSALS.md, NONCOLLISION-20261108.md, SIBLING-POINTS-20261110.md}`, `families.json` and `round-033/panel.json` (grep); `docs/ic/LEADERBOARD.md` lines 320-341; `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md` lines 560-780; `docs/leads/2026-11-18/novel-decomposition-oracle/REPORT.md`; `docs/ACTIVE.md` (grep for #4683); `problems/README.md` (grep); `src/cryptanalysis/ic_boundary.rs` lines 9420-9800, 13700-13760 and grep of `sibling`/`prune`; `src/cryptanalysis/ic_framework/plugins.rs` lines 1795-1830, 2330-2600 and grep of parameter lists. Greps with no hit: `inner=1` (implemented), `prune=1,sib=1` (measured together), `wsum`/`wwalk` in Table B studies, `free=1` with `ksum:m=5` on prime-abscissa, walk-side `sib` in `Targets`. Not read: `docs/session-logs/*` (too large; one grep for #4683 only), the other `docs/leads/2026-11-1[5-7]` reports, `docs/ic/factor-base-encyclopedia/` (not opened: no new base is proposed). research-kb unavailable.

Literature (web search, two queries, titles and abstracts only): Galbraith-Gaudry, Recent progress on the ECDLP (DCC 2016); Petit-Kosters-Messeng, PKC 2016 (prime-field summation polynomials); Bernstein-Lange-Schwabe, On the correct use of the negation map in the Pollard rho method (PKC 2011, eprint 2011/003); Bernstein-Lange, Two grumpy giants and a baby (ANTS X, 2012). The searches found no published hybrid of index calculus and table collisions as relations, so "closest published method" for L2-FREE and L2-FEWCOL is the generic collision literature above, and the closest results are in-repo.
