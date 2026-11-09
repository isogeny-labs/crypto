# Curve25519-family, the 0.75x goal: can an IC relation cost less than a collision? (Idea Synthesist, 2026-11-08)

Status: **proposals only.** Nothing in this file is a measurement, a result or a status change. Each number is
one of three kinds. *Quoted* numbers come from a committed file, with the path given and the tag
`[pcc:unverified]`, because no number cited here has a signed claim. *Derived* numbers are arithmetic in this
file and were not run. *Predicted* numbers are for the Coordinator to freeze before any run. I had read, search
and web tools only: I ran nothing, queried no ecdb, made no Conductor check, and edited nothing outside this file.

## 0. Bottom line

1. **No mechanism I found makes an index-calculus relation cheaper than a collision at all three band
   weights.** The 0.75x goal is out of reach for every IC design the repository has built or proposed:
   - At k = 24, 28 and 32 it is closed at every weight for collision-based IC with at least one unknown seed,
     whatever the design (section 1).
   - At k = 20 it is closed at w = 1 on the five-curve mean. A sliver stays open on the curve least favourable
     to rho.
   - Non-collision (decomposition) IC can pass only with an oracle costing a few native operations per trial.
     The cheapest known oracle is about 40x over that budget at k = 20, and the gap grows with k (record SNC-3).
2. **The connection.** The four lanes each saw one side of the same constraint (section 2):
   - The generic track has few unknowns and unstructured abscissae, so its only relations are collisions.
   - The algebraic track has structured abscissae but many independent unknowns, so it pays in trials, linear
     algebra or native work.
   - A relation cheaper than a collision needs a point set with both properties: few unknown logarithms and a
     cheap algebraic membership test. No curve structure of this family supplies one.
3. **The w = 1 point of the band binds every route.** Each method pays about one listed native operation per
   touched element. Rho-neg pays its canonicalisation, and table methods pay their inserts (priced since
   2026-11-08). At w = 1 the ratio to rho therefore tends to the ratio of touched elements. With K = 2 unknowns
   the touched-element bound is 1.01 sqrt(r) (incidence, NONCOLLISION NC-2), against rho-neg's roughly 1.1 to
   1.5 sqrt(r) including set-up.
4. **Records.**
   - Two engineering records (SNC-1, SNC-2) lower the C = 2 main-track baseline. They share one panel, the
     cheapest experiment here that closes two open questions.
   - One card (SNC-3) carries the double-large-prime cycle elimination over to prime-field m = 2 and prices it
     across the band. It corrects SYNTHESIS Table 2's m = 2 row, which says "excluded by linear algebra"; under
     the band, w = 1 is the binding constraint.
   - None can reach 0.75. The probability is below 0.01 for each, under the assumptions stated.
5. **An existing 0.75 artifact.**
   - The progress headline `s_twcj_w1` reads 0.75 of rho at k = 24 (`research/curve25519_f1_targetwalk_20261108/README.md`, Answer and section 5)
     [pcc:unverified].
   - That README itself identifies the value as an unpriced-insert artifact. With inserts priced it is 1.39x.
   - It must not be read as meeting the goal (section 3).

## 1. Targets in the band, and what the incidence bound leaves

The rho-neg columns are the five fresh curves per size from `../c1/band-confirm-k{20,24,28,32}.txt` (means
derived here) [pcc:unverified]. The two right-hand columns give the derived lower bound for any collision-based
IC with K = C + 1 = 2 unknowns:
- the stored-element bound E >= 1.0095 sqrt(r) (NONCOLLISION NC-2, incidence count for oblivious designs; for
  adaptive generic algorithms the same first-moment union bound is a heuristic here, not proved);
- plus the charged final check [d]G = Q at 1.5 log2 r GAE (0.070 / 0.022 / 0.0065 / 0.0019 sqrt(r));
- at w > 0, plus w per stored element for its hash insert.

| k | rho-neg S, w = 0 / 0.1 / 1 | 0.75x target, w = 0 / 0.1 / 1 | K = 2 collision floor, w = 0 / 0.1 / 1 | closed at |
|--:|:--|:--|:--|:--|
| 20 | 1.590 / 1.705 / 2.745 | 1.19 / 1.28 / 2.06 | 1.08 / 1.18 / 2.09 | w = 1 (mean only) |
| 24 | 1.141 / 1.244 / 2.166 | 0.86 / 0.93 / 1.62 | 1.03 / 1.13 / 2.04 | every weight |
| 28 | 0.999 / 1.093 / 1.934 | 0.75 / 0.82 / 1.45 | 1.02 / 1.12 / 2.03 | every weight |
| 32 | 0.928 / 1.019 / 1.840 | 0.70 / 0.76 / 1.38 | 1.01 / 1.11 / 2.02 | every weight |

Qualifications, quoted and not re-scored:
- The k = 20 rho references disagree:
  - 1.531 at w = 0 in `research/curve25519_f1_targetwalk_20261108/README.md`;
  - 1.590 in `../c1/confirm-summary.json`;
  - 1.620 in `../bsgs/README.md`.
  So the k = 20 target ranges from 1.15 to 1.22.
- Rho-neg at w = 1 ranges over 2.524 to 2.948 across the five k = 20 curves. On the 2.948 curve the w = 1
  target is 2.21, above the 2.09 floor, so k = 20 is closed only on the mean.
- The c1 and targetwalk bands are the pre-2026-11-08 reading (AGENTS.md section 5a, last bullet). Their IC rows
  at w > 0 omit the walk's table inserts and understate IC cost; their rho-neg rows are unaffected. The floor
  column assumes the current reading.
- The w = 1 floor assumes at least one listed native operation per stored element, the same per-step price
  rho-neg pays. Measured (S(1) − S(0))/S(0) for rho-neg is 0.73 at k = 20 and 0.98 at k = 32 (derived from the
  table above).

The rest of the generic design space is closed as well:
- C = 0 is rho or BSGS, which is excluded.
- C >= 2 has a higher bound.
- A structured BSGS-like C = 1 design does not open the k = 20 sliver either. Derived here: a baby set {±iP}
  with two giant streams G + jβP and Q + kγP has three useful pair families, each covering a fraction
  p = (2/9) t^2 at symmetric split, and it needs two of the three. That gives
  E = 1.0607 · (2 − 6/5 + 4/7) = 1.455 sqrt(r), above the 1.329 Poisson value. It is also BSGS-class by structure.

## 2. The connection no single lane saw

| lane | what it holds fixed | what it shows | what it cannot see |
|:--|:--|:--|:--|
| autolab generic (rounds 1-35, `../README.md`) | few unknowns (C = 1..3), random abscissae | every relation is a collision; the plateau is 1.8x-1.9x (multicurve) | that the plateau is the K-collision floor in touched elements, and that this survives w = 1 |
| NC-1/NC-2 (`NONCOLLISION-20261108.md`, `../nc2/RESULTS.md`) | oblivious designs at w = 0 | the K = 2 incidence bound is 1.01; one-addition walk designs are random-like (D3/D0 0.974-1.027) | the band: at w = 1 every stored element costs about 2 |
| targetwalk (`research/curve25519_f1_targetwalk_20261108/`) | C = 1, all pairs | all-pairs collection works (walk 1.39 sqrt(r), 4-6% over 1.329); the w = 1 insert asymmetry | the main track (C = 2) and the verification overhead there |
| algebraic (`problems/curve25519-family-algebraic-ic-tower.md`, SYNTHESIS Table 2) | structured abscissae (towers, bit intervals) | native cost 10^5-10^6x at w >= 0.1; w = 0 trial count about 18x | that its unknowns (N columns) are what force the trial count, independent of solver speed |

The two tracks stall for the same reason seen from opposite sides.
- **Generic.** A point set with few unknown logarithms (seed multiples, walk entries) has abscissae with no
  algebraic structure. Membership can only be tested by storing the set, so a relation is a coincidence among
  stored elements and pays the collision count.
- **Algebraic.** A point set with algebraic membership (small x, x in g·μ_d, zeros of an isogeny tower) has
  independent unknown logarithms, one per element. N columns need about N relations (or N/2 with cycle
  elimination at m = 2), and each relation costs a decomposition trial.
- **What both would need.** A relation cheaper than a collision needs a set S in `<G>` with:
  - (i) |S| >= r^(1/2);
  - (ii) logarithms in a known affine family of dimension O(1);
  - (iii) a membership or decomposition test costing O(1) native operations, with no table.
- **Candidates checked against (i)-(iii)**, all closed:
  - torsion Möbius orbits (x -> 1/x): satisfy (ii) and (iii) but leave `<G>` (STRUCTURE-R4 row "cofactor 8");
  - endomorphism orbits: degree >= 1597 at k = 20 (STRUCTURE-R4 section 2);
  - isogeny images: only relocate the r points (Theorem 15.1, NONCOLLISION row "isogenies");
  - seed multiples {[i]P}: abscissae are degree-i^2 division-polynomial values, which give no handle on i;
  - the "subfield factor base after lifting to F_{p^n}" variant: the lab's Lean round 6 states that such a base
    generates only E(F_q) (session log 2026-11-08T06:30Z, `research/ecdlp_weakness_lean_20261104/lean/SubfieldFactorBase.lean`,
    PCC-20261108-8e873f, unsigned; I did not read the Lean file).

I propose this as the obstruction to record. I may not write `problems/`. Exit criterion: a set meeting (i)-(iii)
on any family miniature, checked by a planted-log test.

## 3. Ruled out already, and accounting traps (flagged, not proposals)

- **Closed by OBSTRUCTION.md and NONCOLLISION section 1:** cofactor 8, Montgomery/Edwards, twists, Frobenius,
  endomorphisms, isogenies, xedni, and m >= 3 summation polynomials with MITM. Nothing below reopens them.
  SNC-1 and SNC-2 sit inside the obstruction: they approach the Poisson floor and do not break it.
- **NC-1 has no results file.** The brief cites `nc1/RESULTS.md`, but that path does not exist. NC-1's
  designed-slope Stage 0 (D2/D3 at K = 1) was never run. Only its K = 1 controls exist: D0i 0.886-0.887 and D0
  0.918-0.923, in `../nc2/RESULTS.md` [pcc:unverified]. I report the gap and do not infer an NC-1 outcome.
- **C = 1 classification disagrees between two records:**
  - `../c1/README.md` calls C = 1 "a labelled boundary result outside the ratchet";
  - the targetwalk README calls its C = 1 walk "not index calculus in the decomposition sense ... does not
    belong on the IC leaderboard".
  The brief's "best IC ratio 1.40" (c1, k = 20) and twcj's 1.40 at k = 20 are of this class. The owner's ruling
  decides which applies.
- **Accounting or relabelling, not improvements:**
  - The `s_twcj_w1` headline, 0.83 at k = 20 and 0.75 at k = 24: unpriced inserts (see section 0).
  - X ± Y from one inversion or square root (Galbraith-Wang-Zhang): counts a paired operation as one. Rounds 5
    and 21 measured identical S.
  - Native S3 tests or x(P ± Q) by field formulas in place of counted additions. This is a relabelling, as in
    the §10.5 triple oracle: rootfind-pairs measured 70x-320x the incumbent (round 3).
  - Unlisted distinguished-point checks or per-step canonicalisation. AGENTS.md section 5a requires them listed;
    targetwalk README section 5 already notes unlisted per-step work.
  - w = 0-only algebraic arms (NONCOLLISION section 3 item 2): unpriced shifting.
  - Batched affine arithmetic: field-level, so GAE is unchanged.
  - Multi-target and online-clock readings: other gauges.

## 4. Idea records

```yaml
id: SNC-1-lazy-certification-main-track
title: "Certify collision rows by their exact key and the charged final check; recompute only when [d]G = Q fails (carry targetwalk open item (a) to the C = 2 main-track wwalk incumbents)"
track: generic decomposition, C = 2 main track (owner ruling 2026-11-04); C = 1 boundary as a side row
obstruction_attacked: >
  Fixed per-collision recomputation (verify_ops, `src/cryptanalysis/ic_boundary.rs` WalkTable/TargetWalk doc
  comments around 8040-8063 and 8596-8617). Measured on the C = 1 target walk: 0.255 sqrt(r) at k = 20 and 0.079 at
  k = 24 ("about two collisions x two recomputations"), out of S = 2.138 / 1.623 (targetwalk README section 3 phase
  table) [pcc:unverified]. Keys are exact abscissae with a sign bit (`map` doc: "Key (abscissa and sign bit,
  shifted)"), so a key collision already certifies E_t = +/-E_s; the recomputation only re-checks bookkeeping.
what_is_new: "targetwalk README section 6(a) lists this for its C = 1 walk only and did not measure it. The C = 2 incumbents (`wwalk:g=1,grow=2,seed=run`) recompute every collision too; their verify share has never been read out."
claim: >
  An option `lazy=1` that skips verify, keeps rows, and on a failed internal [d]G = Q recomputes (charged) only the
  rows used, drops any that fail and continues, lowers S at k = 20 by the incumbent's verify share with identical
  walk and probe counters (same seed=run draw), with zero wrong answers accepted.
mechanism: "Certification moves from a group recomputation per row to a key comparison (already done) plus one final check (already charged). The work is skipped, not uncounted: this is engineering, not accounting, provided the re-verification path is implemented and charged."
prediction:
  stage0_verify_share_C2_incumbent: {k20: "0.30 [0.15, 0.45] sqrt(r)", k24: "0.08 [0.04, 0.12]", k28: "0.02", k32: "0.006"}
  S_over_rho_w0: {k20: "1.57 -> 1.39 [1.30, 1.48]", k28: "1.80 -> 1.78 [1.77, 1.79]"}   # multicurve incumbents 2.601 / 1.869, ratios 1.57 / 1.80
  C1_boundary_side_row: {k20: "1.396 -> 1.230 (derived: (2.138 - 0.255) / 1.531)", k24: "1.411 -> 1.343"}
  band: "the same absolute GAE saving at w = 0, 0.1, 1 (unpriced counters unchanged); w = 1 ratio falls by about 0.30 / S(w = 1) at k = 20"
  reaches_0.75: "no; probability < 0.01 under the section 1 assumptions (K = 2/3 collision floor)"
discriminating_test: >
  Stage 0 (zero compute): read verify_ops / build_ops per run from the committed multicurve and rebaseline-seedrun
  session records of the C = 2 incumbents at k = 20, 24. This sizes the lever only; a subtraction is not a result.
  Stage 1: the shared panel below (with SNC-2).
shared_panel: >
  One frozen panel per size k = 20, 24, 28, 32, seed=run, fresh target seeds:
  {inc, inc+lazy, tw2cj, tw2cj+lazy, rho-neg}, 8 single-target sessions x 64 rounds as in the targetwalk protocol,
  `ecbench verify --replay`, `ecbench band --ranks` under the 2026-11-08 pricing. The 2 x 2 factorial separates the
  lazy effect from the design effect and closes SNC-1 and SNC-2 together.
falsification: >
  Dead if Stage 0 shows verify below 0.10 sqrt(r) at k = 20 for the C = 2 incumbent. Dead at Stage 1 if inc+lazy /
  inc has its 95% upper end >= 1 at k = 20, if walk or probe counters differ from inc on the same draw (the option
  changed more than verification), or if any accepted answer fails ecbench's check.
cost_band: >
  About 40 lines of Rust (WalkTable and TargetWalk option plus plumbing). Panel: about 10k runs over four sizes,
  about 2 sqrt(r) GAE each (about 1.2e8 GAE at k = 32); minutes of CPU on the route the director freezes. Counted:
  walk, set-up, probes, any re-verification, final check. Unpriced: one insert per stored entry, one lookup per
  probe, row operations; rho-neg's canonicalisations; all priced at 0, 0.1, 1.
controls: [inc vs inc+lazy on the same seed=run draw (exact pairing), rho-neg, "fault injection: corrupt one stored coefficient in a test build; the lazy path must reject it via the final check and the re-verification"]
distinct_from_obstruction: "No: inside it. Engineering on set-up/verification, floor unchanged."
probability: {measured_improvement_k20_and_k24: 0.65, reaches_0.75: "< 0.01"}
```

```yaml
id: SNC-2-two-seed-target-walk
title: "The C = 2 target walk: every stored entry carries s1, s2 and log Q, every collision is a row, no probe phase (the targetwalk design carried to the main track)"
track: generic decomposition, C = 2 (classification: same class as the wwalk C = 2 arms; the owner rules, since the targetwalk README calls the C = 1 version 'rho with one superfluous unknown')
obstruction_attacked: >
  The table-probe pairing loss in the C = 2 incumbents. The probes cost 0.39 / 0.37 / 0.50 sqrt(r) at k = 24 / 28 / 32
  (OBSTRUCTION.md headroom table, multicurve) [pcc:unverified], and probe-probe pairs carry no seed part. NC-2 D0 (an
  all-pairs walk with rows of nonzero seed part) gives E = 1.684-1.740 sqrt(r) at C = 2 on nine curves at
  k = 20/24/28 (`../nc2/RESULTS.md`) [pcc:unverified], against the incumbents' table plus probes.
what_is_new: "targetwalk measured the all-pairs design at C = 1 only (twcj/f1-x07 0.909 and 0.888 at k = 20, 24). NC-2 measured the C = 2 all-pairs control but only with jumps uniform in Z_r^4, which need full-size scalar multiplications, so its 1.72 is a prior, not a Stage 0 for a buildable design."
claim: >
  TargetWalk with two seeds and chained jumps (J_j = [a_j]P1 + [a'_j]P2 + K_j, K_j = K_(j-1) + K_(j-2) over
  [c]G + [b]Q), stopping at rank 3 over (s1, s2, log Q), has mean walk length 1.72-1.80 sqrt(r) and, with lazy
  certification, S below inc+lazy at k = 24, 28 and 32.
mechanism: "Store every point; all pairs are rows (K = 3 Poisson floor 1.662, plus the 3.5-6% walk excess seen in NC-2 D0 and in targetwalk). Set-up: four doubling chains plus chained jumps, about 0.42 / 0.13 / 0.04 / 0.02 sqrt(r)."
prediction:
  stage0_E_chained_C2: {k20: "1.75 [1.70, 1.82]", k24: "1.75 [1.70, 1.82]", k28: "1.75 [1.70, 1.82]"}
  S_w0: {k20: "2.29 [2.15, 2.45]", k24: "1.90 [1.82, 2.00]", k28: "1.78 [1.72, 1.86]", k32: "1.76 [1.70, 1.84]"}
  S_over_rho_w0: {k20: "1.38 [1.30, 1.48]", k28: "1.71 [1.66, 1.79]"}     # rho from multicurve: 1.657 / 1.038 (derived S_inc / ratio)
  tw2cj_lazy_over_inc_lazy: {k20: "0.99 [0.93, 1.06]", k24: "0.94 [0.89, 0.99]", k28: "0.97 [0.93, 1.01]", k32: "0.92 [0.88, 0.97]"}
  band: "one insert per stored entry (about 1.75 sqrt(r)) against inc's inserts plus probe lookups (about 1.9 sqrt(r)): w = 1 ratio within 0.03 of w = 0"
  reaches_0.75: "no: the floor 1.662 is 1.66x-1.79x fresh rho at k = 28, 32; probability < 0.01"
discriminating_test: >
  Stage 0 (zero group operations): extend `src/bin/nc2_stage0.rs` with the chained-jump C = 2 structure and the rank-3
  stop rule (with guard), 2,000 samples at the nine NC-2 curves. Stage 1: the shared panel in SNC-1.
falsification: >
  Dead at Stage 0 if chained E > 1.82 at two of three sizes (no room under the incumbents' model). Dead at Stage 1 if
  tw2cj+lazy / inc+lazy has upper end >= 1 at two of k = 24, 28, 32; if the measured walk mean is outside [1.66, 1.85];
  if any row with zero seed part is used; or if any answer is unverified.
cost_band: >
  Stage 0: about 120 lines of Rust and seconds of CPU. Stage 1: about 200 lines (TargetWalk with two seeds; WWalkOracle
  option) and the shared panel. Counted: chains, jumps, walk steps, final check. Unpriced: one insert per entry, row
  operations on 3 columns, per-step canonicalisation (to be listed: targetwalk README section 5 notes it is unlisted).
controls: [inc and inc+lazy, rho-neg, "Stage 0 D0i (iid) must give 1.662 +/- 0.01", "randomised-e control as in NC-2"]
distinct_from_obstruction: "No: approaches the K = 3 Poisson floor (OBSTRUCTION table row K = 3), does not lower it."
probability: {improvement_over_inc_lazy_at_two_of_k24_28_32: 0.45, reaches_0.75: "< 0.01"}
```

```yaml
id: SNC-3-m2-graph-cycle-budget
title: "m = 2 decompositions with cycle elimination (double large primes carried from RESIDUAL_WALKS 11.7 to prime-field m = 2): the one relation type cheaper than a collision in touched elements, priced across the band"
track: algebraic or generic decomposition (by oracle); a bound card with two zero-group-operation checks
obstruction_attacked: >
  SYNTHESIS Table 2 excludes m = 2 because linear algebra on N about sqrt(r/2) columns costs N^2. An m = 2 relation
  R = aG + bQ = +/-P_i +/- P_j is an edge of a graph on the base, and d follows from the first solvable (sign-balanced,
  nonzero total b) cycle, found by union-find in about one operation per edge. Under that elimination the LA objection
  falls. The technique is in the lab (RESEARCH_RESIDUAL_WALKS.md section 11.7, Koblitz double large primes); the
  transfer to prime-field m = 2 and its band price is the contribution.
claim: >
  Let N be the number of base classes, c_t the listed native operations per decomposition trial (each trial is one
  GAE walk step), c_e >= 1 per edge, and gamma N the edges at the first solvable cycle (random-graph theory:
  P(forest at cN edges) -> sqrt(1 - 2c) exp(c + c^2), so gamma is about 0.45-0.5). Edges per trial are N^2/(4r) for a
  base in E(F_p) and 2N^2/r for a base inside <G>. Minimising over N (derived):
    full-group base: S(w) >= 4 gamma sqrt(w c_e (1 + w c_t))
    base in <G>:     S(w) >= gamma sqrt(2 w c_e (1 + w c_t))
  At w = 0 both tend to 0 as N grows (unpriced shifting). The 0.75 targets of section 1 then require:
    full-group base: c_t < 0.06 at k = 20 at w = 1, impossible at k >= 24;
    base in <G> (and <G>-membership free): c_t <= 7.5 / 4.3 / 3.2 / 2.8 at k = 20 / 24 / 28 / 32 at w = 1, and
    c_t <= about 300 (k = 20) to 100 (k = 32) at w = 0.1.
  One S3 evaluation already costs about 10 field multiplications, and no method finds (x1, x2) in V^2 with
  S3(x1, x2, x_R) = 0 for a structured V of size about sqrt(r) in O(1) evaluations.
prediction:
  gamma_montecarlo: "0.47 [0.42, 0.52] at N = 1e3..1e5"
  existing_oracles_c_t: "rootfind-type: about N sqrt-solves per trial (>= 3e2 at k = 20); pkm-iso m = 2: about 7e5 F4 multiply-adds per target (SYNTHESIS, isogeny-seed engine table) [pcc:unverified]"
  S_over_rho_w1_best_conceivable_c_t_10: {full_group: {k20: "2.4", k28: "3.4"}, base_in_G: {k20: "0.86", k28: "1.22"}}
  S_over_rho_w1_existing_oracles: ">= 4.5 (base in <G>, c_t = 300) and >= 12 (full-group base) at k = 20; larger at higher k, where N and hence c_t grow"
  reaches_0.75: "probability < 0.01: needs an O(1)-evaluation m = 2 decomposition oracle over a <G>-only structured base, an open problem (Hu, ePrint 2024/1923, as quoted in SYNTHESIS section 2)"
discriminating_test: >
  (a) A native Monte Carlo of the first solvable cycle in a random signed multigraph with edge labels (a, b) uniform
  mod r: 1e4 graphs per N in {1e3, 1e4, 1e5}; zero group operations; seconds. (b) Read decompositions per trial and
  native operations per trial from committed counters of `rootfind-mitm:m=2` (round 3 sessions) and the pkm-iso m = 2
  incumbent (ECBS1h405f20219036) at k = 20, and place both on the budget curve. No ecbench run.
falsification: >
  The card is wrong if gamma < 0.30 (every bound falls by 40% or more), or if a committed m = 2 oracle shows c_t <= 10
  with at least half the predicted edges per trial, or if any IC arm under the 2026-11-08 pricing measures
  S(w = 1)/rho(w = 1) <= 0.75 at any k and survives the accounting audit (base-free rows, unlisted counters, replay).
cost_band: "About 100 lines of Rust for (a); (b) is reading committed records. Counted/unpriced split as in the claim; every bound is stated at w = 0, 0.1, 1 with rho-neg's measured band."
controls: ["(a) at gamma's known limit: the fraction of forests at cN edges must match sqrt(1 - 2c) exp(c + c^2) within 2% for c = 0.25, 0.4"]
distinct_from_obstruction: "Distinct from OBSTRUCTION (not collision-based) and from NONCOLLISION's closed MITM row (that is m = 4). It corrects SYNTHESIS Table 2's reason for m = 2; it does not reopen m = 2 under the band."
probability: {card_holds: 0.9, reaches_0.75: "< 0.01"}
```

## 5. Ranking and the cheapest experiment

1. **The shared SNC-1/SNC-2 panel.** One 2 x 2 factorial plus rho-neg settles both records. SNC-1 Stage 0 costs
   nothing and should run first; if verify is below 0.10 sqrt(r) at k = 20, drop the lazy arms. The expected gain
   is 10-12% at k = 20 and 5-9% at k = 28 to 32. These are measured improvements to the C = 2 baseline. The
   distance after them is about 1.4x / 1.7x rho at k = 20 / 28.
2. **SNC-3 checks (a) and (b).** Both cost nothing in group operations. They turn the m = 2 row of Table 2 into a
   budget card and fix the band reason in the obstruction record.
3. **Nothing toward 0.75.** Reaching it needs a set meeting section 2's (i)-(iii). I found none and do not propose
   a run to look for one without a candidate.

## 6. Sources checked and limits

- **research-kb MCP: unavailable in this runtime** (no tool loaded). The lead atlas is absent: a glob of
  `/home/user/crypto-autoresearcher/knowledge/leads/ecdlp/*.md` returned nothing, so `--match` was not run. No ecdb
  query was made: curve facts come from STRUCTURE-R4 section 2 and the c1 files. No Conductor check, no session
  log (the brief limits edits to this file), and no run.
- **Read:**
  - `ideas/OBSTRUCTION.md`, `ideas/NONCOLLISION-20261108.md`, `ideas/SYNTHESIS.md`, `ideas/STRUCTURE-R4.md`,
    `ideas/PROPOSALS-R3.md` (with its Outcomes section);
  - `../README.md` (rounds table), `../nc2/RESULTS.md`, `../c1/README.md`, `../c1/confirm-summary.json`,
    `../c1/band-confirm-k{20,24,28,32}.txt`, `../bsgs/README.md`, `../r4-i1/README.md`;
  - `research/curve25519_f1_targetwalk_20261108/README.md`, `problems/curve25519-family-algebraic-ic-tower.md`,
    `research/ic_arity_sweep_20261027/README.md` lines 1-90;
  - grep hits in `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` (sections 11.7, 10.5) and
    `RESEARCH_RHO_PARITY_PROGRAMME.md` (double large primes);
  - `src/cryptanalysis/ic_boundary.rs` 8040-8109 and 8596-8645 (verify ledgers, keys);
  - `docs/ecbench/README.md` 440-457 and 1048-1056 (the GAE unit);
  - AGENTS.md: success criterion, tracks, preserve scope, name the obstruction, intermediate-cost, boundary/table/ratio
    sections 1-6 and 5a;
  - `docs/session-logs/2026-11-08.md`: entries on NC-2, the BSGS panel and Lean round 6.
- **Derived here, by hand, unrun (about ±5%):**
  - the band targets and floors in section 1;
  - the three-stream E = 1.455 (independence approximation, symmetric split, not shown optimal);
  - the SNC-3 bounds (they assume random-graph edges, c_e >= 1, and one GAE per trial);
  - the SNC-1 verify share for C = 2 (scaled from the C = 1 measurement, not read from records).
- **Literature from memory, not re-fetched:** Shoup 1997; Lenstra-Manasse two large primes (as cited in
  RESIDUAL_WALKS); random-graph acyclicity at cN edges (Erdős-Rényi; constant quoted from memory, checked by SNC-3
  control (a)).
