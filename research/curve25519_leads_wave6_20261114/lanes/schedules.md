> Idea Generator lane report, wave 6, lane "schedules" (2026-11-14). Proposals only: nothing was run, no file outside
> this one was edited, no status changed. Every number is `[pcc:unverified]`: a quote of an unverified on-disk assertion
> (path given), a quote of a fetched paper (page given), or hand arithmetic done here. **No simulation was run**: this
> lane had no shell, and AGENTS.md "Implementation language: no Python" bars Python for research result generation even
> where the task allowed stdlib Python. The abstract-walk checks are therefore specified (section 3) as an extension of
> the existing Rust tool `src/bin/nc2_stage0.rs`; the "results" below are closed-form or Simpson integrals by hand,
> validated once against an exact value (random K = 3: Simpson 1.6616 vs Gamma(7/2)/Gamma(3) = 1.6617).

# C = 2 table-then-walk arms: schedule-level levers after wave 5

## 0. Where the arms sit, and the constraint every schedule lever meets

Quoted [pcc:unverified], `research/curve25519_autolab_20261104/snc-panel/README.md` (claim `PCC-20261110-978c29`):
`tw2cj-lazy` S = 2.529 / 1.915 / 1.756 / 1.758 at k = 20 / 24 / 28 / 32, stored entries 1.738 / 1.677 / 1.686 / 1.738
per sqrt(r), rho-neg 1.570 / 1.175 / 1.009 / 0.989; `inc-lazy` 2.096 / 1.932 / 1.908 (k = 24 / 28 / 32). So the
non-entry part of S is 0.791 / 0.238 / 0.070 / 0.020 (derived) and entries sit 0.9 to 4.6 % above the K = 3 Poisson
floor 1.662 (SE about 1.3 % per cell, derived from Var(sqrt X), X ~ Gamma(3), over 480 runs). The ratio of floors is
1.662 / 0.886 = 1.876; the measured 1.74-1.78x is lower only because rho-neg itself runs 12-14 % above 0.886.

Already on disk and not repeated: W5-PL-1..4 (`research/curve25519_leads_wave5_20261112/lanes/pipeline.md`); NC-1 / NC-2
designed oblivious sequences (`research/curve25519_autolab_20261104/ideas/NONCOLLISION-20261108.md`) with Stage-0
**nulls**: NC-2 best held-out designed E / random-walk E = 0.974-1.027 over 18 cells (`.../nc2/RESULTS.md`); NC-1 best
K = 1 design 0.927 held-out against i.i.d. 0.887 (`.../nc1/RESULTS.md`). NC-2's incidence bound (E >= 1.26 at K = 3)
is the only proved-style room below 1.662, and no construction is known.

**Obstruction this lane names (derived here, not proved in Lean).** For an oblivious walk (jump stream independent of
the points, as `wwalk` is), the collision events of two pairs whose difference functionals over (s1, s2, d) are linearly
independent are *exactly independent* under uniform (s1, s2, d) (each value pair (a, b) has probability 1/r^2). Row
arrival can be made sub-Poisson only by concentrating many pairs in one functional class, and one class yields at most
one independent row. Bulk same-class pairs exist only on lines or planes of coefficient space; lines in a known (G)
direction waste their within-line pairs, planes collapse to one row each. Section 2, C1, computes what that costs.
Consequence: with K = 3 forced by C = 2 plus the target, schedule levers can recover only the walk's local excess
(about 3.5 %, PL-2/PL-4) and fixed costs (PL-1/PL-3); everything else in this lane is either rejected or a
classification-dependent variant. Suggested `problems/` wording for the Coordinator (I may not write it): "C = 2
schedules are bounded by K = 3 Poisson arrival; pairwise independence of independent-functional collisions leaves
only class-concentrated designs, which lose pairs. Exit: an nc2_stage0 plane-design cell below 0.98 x i.i.d."

## 1. Literature, fetched and quoted (constants with page), atlas status

| source (fetched) | constants quoted | atlas id (bodies unread) | what is new here |
|:--|:--|:--|:--|
| Bernstein-Lange, "Two grumpy giants and a baby", ANTS-X (antsmath.org/ANTSX/bernstein/paper.pdf; ePrint 2012/294 abstract) | p.15: BSGS-then-giants "(1.5+o(1))sqrt(l)" average, interleaved "(4/3+o(1))sqrt(l) = (1.3333...)", rho "(sqrt(pi/2)+o(1))sqrt(l) = (1.2533...)"; grumpy experimental chains "(1.23046...)l^1.5" (l = 65537, n = 146), "(1.20545...)l^1.5" (l = 1000003); slopes 23n^2/8 from 3n mults, "(0.71875+o(1))l"; p.3 "low overhead but high memory. We have not found a low-memory variant"; p.2 "all of the ideas can be adapted to the inverting case" (no inverting constant printed) | LEAD-RHO-grumpy-store-all-schedules; lab NC-1 | the K = 3 line/plane analysis (C1); BL grumpy realises 2.875 / 4.5 = 0.639 of ideal pair coverage (derived from p.14-15) |
| same, sections 3-4 (p.6-10) | p.7: average "sqrt(pi/2) sqrt(l) / sqrt(1 - sum p_i^2)", "The sqrt(1 - 1/r) formula" for adding walks (credited to Teske and Blackburn-Murphy); p.9: degree<=2 factor "1/sqrt(1-1/r-1/r^2+1/r^3)", degree<=3 "1/sqrt(1-1/r-1/r^2-2/r^3+7/r^4-4/r^5)"; Hildebrand: uniform "within O(l^(2/(r-1))) steps" | LEAD-RHO-r-adding-walk-mixing | W5 finding 1 is this published formula, not new; degree<=3 gives 1.0352 at j = 16 (NC-2 D0/i.i.d. at C = 2 averages 1.038, derived from its table) |
| Bernstein-Lange, "Computing small discrete logarithms faster", ePrint 2012/458 (abstract) | group of order l: "1.77*l^{1/3}" per log "using a table of size l^{1/3} precomputed with 1.24*l^{2/3}" | LEAD-RHO-advice-table-variants, LEAD-RHO-dp-multitarget-precomputation | single target: precomputation 1.24 l^(2/3) >> sqrt(l), so nothing transfers |
| Galbraith-Ruprai, ePrint 2010/615 (PKC 2010 full version, abstract); Ruprai thesis 2009 (math.auckland.ac.nz/~sgal018/Ruprai-thesis.pdf, p.3) | kangaroo "approximately 2 sqrt(N)"; Pollard "less than 1.71 sqrt(N)" (one inversion); thesis "less than 1.72 sqrt(N) ... generic group and 1.47 sqrt(N) ... fast inversion"; PKC: "close to 1.36 sqrt(N)" with fast inversion (abstract summary) | LEAD-RHO-kangaroo-interval; `ideas/STRUCTURAL-20261110.md` line 41 (clamped scalars: loss) | Gaudry-Schost lens on strict vs tw2 (C6) |
| Teske 2001 (Math. Comp. 70; via Bos ANTS-IX slides p.4, abstract snippet) | slides: "[Teske-01]: r=20 performance close to a random walk"; snippet: r >= 16 "hardly distinguishable" from random | LEAD-RHO-r-adding-walk-mixing | none (mixing is not the bottleneck: l^(2/15) is about 19 steps at r = 2^32) |
| Bos-Kleinjung-Lenstra, ANTS-IX slides (antsmath.org/ANTSIX/slides/bos.pdf) | p.6: fruitless 2-cycle probability "1/(2r)"; table p.14 best negation speed-up 1.29 at r = 128; p.15 "speedup of at most: 1.29 < sqrt(2)" | LEAD-RHO-negation-map-fruitless-cycles | inapplicable to the lab's oblivious stored walk (no iteration function); it prices only the rho-neg reference |
| Cheon-Hong-Kim tag tracing (J. Cryptol. 2012; PDF 403; BL p.2 read) | BL p.2: "look ahead one step in the rho method for F_p* and combine two multiplications into one ... memory usage increases"; search snippet of an SNU thesis: on EC it "does not effect so much" | LEAD-RHO-cheaper-step-primitives (assumed match) | no EC analogue under counted GAE (C7) |
| Holec, CTU master's thesis 2018 (secondary, p.21-22) | negation BSGS average "sqrt N", interleaved with negation "2 sqrt2/3 = 0.9428" | LEAD-RHO-bsgs-family-memory | BSGS-neg layout = NC-1 D1 1.00 |

## 2. Candidates

```yaml
id: W6-SC-1-grumpy-interleaved-C2
verdict: REJECT (predicted regression or null); optional zero-group-op closure cell, section 3
mechanism: >
  BL's gain at K = 1 comes from all pair slopes being one functional (d), so disjoint coverage is a uniform hit time.
  At C = 2 rows must span 3 dimensions; same-class coverage needs lines/planes (section 0). Ideal disjoint coverage
  assumed throughout (optimistic). L lines {v_i + zG}, n/L points each, one known line: edge hit prob 2(n/L)^2/r.
  L = 4 (known, P1, P2, Q lines): rank 3 iff the hit edges connect K4; P(conn) = 16p^3 - 33p^4 + 24p^5 - 6p^6,
  E = sqrt(8) x 0.66067 = 1.869 sqrt(r). L = 5 (one generic line added): E = 1.817 (Simpson, h = 0.4). Strict design
  phase 1 with 3 lines (known, P1, P2): E(rank 2) = sqrt(4.5) x 0.68571 = 1.455 vs Poisson 1.329. Three planes
  {a v_i + bG} (one row each, uniform hit, p = x^2/9) plus random cross-plane pairs (Poisson, mean 2x^2/3): 1.636
  with ideal in-plane coverage (-1.6 %), about 1.74 with BL's realised 0.639 coverage (derived, +-0.03).
claim: "No grumpy/BSGS-interleaved C = 2 schedule has entries/i.i.d. below 0.98; the best class (planes, BL coverage) sits at 1.04 [1.02, 1.08]."
prediction_S_w0: "S = E + non-entry part of tw2cj-lazy: k20 2.53 [2.49, 2.59], k24 1.98 [1.94, 2.04], k28 1.81 [1.77, 1.87], k32 1.76 [1.72, 1.82]; vs incumbent 1.00-1.03"
simulation: "none run; section 3 cells P4, P5, H3 reproduce 1.869 / 1.817 / 1.636 (ideal) as their check values"
cheapest_on_curve_test: "none recommended; a designed-jump option in WWalkOracle (plugins.rs:2657-2668, new `design=` param) only if H3 < 0.98 x i.i.d."
falsification: "any section-3 plane cell with held-out E/i.i.d. < 0.98 at k = 20 and 28 kills the rejection"
not_relabelling: "same counted points; it fails on the counting, not the accounting"
nearest_atlas: LEAD-RHO-grumpy-store-all-schedules; lab NC-2
prior: "0.03 that any realisable plane design beats 0.98 x i.i.d."
lean_obligation: "pairwise independence: for linearly independent f, g on (Z/r)^3, P(f = a, g = b) = r^-2 under uniform theta; rank of {v_i - v_j : ij in E} = |V| - #components for affinely independent v_i"
```

```yaml
id: W6-SC-2-multi-walk-shared-table
verdict: REJECT (dominated by W5-PL-4 by one start point)
mechanism: >
  All stored points pair with all others whichever walk made them, so W walks change only the local repeat structure.
  With disjoint jump sets, cross-walk pairs have no degree-1 repeat (J^A_h = J^B_h' never); within-walk pairs still do,
  so the repeat fraction is 1/(jW): W = 2, j = 16 equals one walk with j = 32 (BL p.7 formula), plus one extra start
  (1.5 log2 r GAE). The lab's exact-key table is DP-free, so the usual multi-walk benefit (parallelism) earns no GAE.
claim: "2 walks x 16 disjoint jumps / 1 walk x 32 jumps = 1.000 +- 0.003 on entries; S higher by one start point."
prediction_S_w0: "vs tw2cj-lazy (j = 16): k20 1.027 [1.01, 1.05], k24 0.998 [0.99, 1.01], k28 0.987 [0.975, 1.0], k32 0.983 [0.975, 0.992]"
cheapest_on_curve_test: "not worth code; PL-4's `jumps=32` arm answers it (plugins.rs:2660, jumps <= 64)"
falsification: "entries(W=2,j=16) / entries(W=1,j=32) outside [0.99, 1.01] in the section-3 cell M2"
not_relabelling: "n/a (rejected)"
nearest_atlas: LEAD-RHO-correlated-walks-shared-sums
prior: "0.9 that it equals PL-4 within 0.5 %"
```

```yaml
id: W6-SC-3-self-sum-vs-fresh-jumps
verdict: NOT A NEW LEVER (W5-PL-2); sharpens PL-2's Stage-0 reference line
mechanism: >
  Mixing (Teske, Hildebrand) is not what costs: an r-adding walk is uniform within O(l^(2/(r-1))) steps, about 19 at
  j = 16, r = 2^32. The cost is local anti-collision: degree<=3 factor (BL p.9) 1.0352 / 1.0165 / 1.0080 / 1.0040 for
  j = 16 / 32 / 64 / 128 (derived; degree-1 alone 1.0328 / 1.0160 / 1.0079 / 1.0039). A self-sum walk (jump = a recent
  stored point) repeats only within its window H, O(H/n). Fresh G/Q jumps are a j-adding walk and pay the factor.
claim: "NC-2 D0 (16 uniform jumps) / i.i.d. = 1.0352 +- 0.004 at C = 2 (on-disk cells average 1.038); self-sum H = 16 <= 1.005."
prediction_S_w0: "as PL-2 (tw2 0.95 [0.93, 0.98] at k28); replace PL-2's falsification law (1 - 1/j)^-1/2 by the degree<=3 factor"
cheapest_on_curve_test: "PL-2 Stage 0 then `walk=selfsum,H=16` in TargetWalk (PL-2's plan)"
falsification: "section-3 cell J: any j in {16, 32, 64} off the degree<=3 factor by > 0.7 %"
nearest_atlas: LEAD-RHO-r-adding-walk-mixing
prior: "0.8 the degree<=3 law fits; PL-2's own prior unchanged"
```

```yaml
id: W6-SC-4-adaptive-table-size
verdict: REJECT (adaptive policy = fixed x* = 1/sqrt(2); gain <= 0.3 %; strict arms only)
mechanism: >
  Strict design (`inc-lazy`): table to rank 2 at n2, then probes with hit rate 2y/r for table size y. After n2 the
  only state that matters is y, so the optimal continuation minimises y + r/(2y) over y >= n2: y* = max(n2, sqrt(r/2)).
  Observed collisions add no information beyond n2, so "online x" is the fixed truncation x = 0.707. Expected cost:
  T = 0 1.772, T = 0.6 1.763, T = 0.707 1.762, T = 0.9 1.768 (W5 finding 2) (Gamma(2) integrals by hand). tw2 has no x.
claim: "inc-lazy(x = 0.71) / inc-lazy(x = 0.9) = 0.997 [0.990, 1.004]; vs x = 0.6 0.9995 (flat, as round 35 measured)"
prediction_S_w0: "k24 2.090, k28 1.926, k32 1.907 (from 2.096 / 1.932 / 1.908); k20 not applicable (box incumbent)"
falsification: "a resolved ratio below 0.99 would mean the probe phase is not Poisson in y"
nearest_atlas: LEAD-ICB-generic-table-oracle
prior: "0.05 resolvable"
lean_obligation: "argmin over y >= a of y + c/y is max(a, sqrt c) (elementary)"
```

```yaml
id: W6-SC-5-target-multiples-as-jumps
verdict: REJECT (is SNC-2 `tw=2,cj=1` plus W5-PL-1)
mechanism: >
  Jumps [2^i]Q put Q in every entry: that is the stored-target walk, rows over (s1, s2, d), K = 3, floor 1.662, already
  measured (tw2cj-lazy). It does not reach "rank 2 sooner": Q adds a column, so rank 3 is needed. The [2^i]Q ladder
  (log2 r doublings) is set-up that PL-1 already recycles; any coefficient choice obeys the same 1/r pair lemma.
prediction_S_w0: "identical to tw2cj-lazy within 1 % (k28 1.756, k32 1.758); ladder recycling worth about 0.02 sqrt(r) at k20 (PL-1)"
nearest_atlas: LEAD-ICB-free-relations-self-collision
prior: "n/a"
```

```yaml
id: W6-SC-6-kangaroo-gaudry-schost
verdict: REJECT (no interval in the C = 2 pipeline); one useful reading
mechanism: >
  Interval methods beat rho only when the unknown is confined: 2 sqrt(N), 1.71, 1.72 / 1.47 (fast inversion), 1.36
  (Galbraith-Ruprai, fast inversion). With N = r: >= 1.36 against the 0.886 rho-neg floor; with a half-size interval
  (negation, or clamped Curve25519 scalars): 1.36 x 0.707 = 0.96 > 0.886, and ecbench targets are uniform anyway
  (STRUCTURAL-20261110 line 41: kangaroo 1.41 on clamped scalars, a loss). s1, s2, d are uniform in Z_r; choosing seeds
  in a short interval would make them partially known-log, i.e. relabel C downward. Useful reading: the strict design
  is Gaudry-Schost-shaped (tame table, wild probes, only tame-wild pairs count): floor 1.772; tw2 counts all pairs: 1.662.
prediction_S_w0: "no arm proposed"
nearest_atlas: LEAD-RHO-kangaroo-interval
prior: "n/a"
```

```yaml
id: W6-SC-7-tag-tracing-and-negation-cycles
verdict: REJECT (both)
mechanism: >
  Tag tracing defers full computation of intermediate F_p* elements; on EC every point must be computed in full to be
  keyed, and the stored-table design pays for unstored points quadratically (pairs scale as the square of the stored
  fraction). Fruitless cycles need an iteration function; the lab walk draws h_t from a stream (plugins.rs:2663 `seed`),
  so a +-collision is detected by the exact key and no cycle can recur. BKL's 1/(2r) and 1.29 price only rho-neg.
nearest_atlas: LEAD-RHO-cheaper-step-primitives; LEAD-RHO-negation-map-fruitless-cycles
```

```yaml
id: W6-SC-8-late-target-stored-walk
title: Strict table to rank 2 target-free, then continue the same walk with chained [beta_h]Q parts in the jumps, stored
verdict: PROPOSE (classification-dependent variant of tw2; the one schedule change with a resolvable predicted gain on the strict arm)
obstruction: >
  The strict design pays the Gaudry-Schost penalty: probes Q + kG pair only with table entries (probe-probe differences
  are pure G), floor 1.772 vs 1.662 (W5 finding 2, 6.4 %). tw2 removes it but puts the target in the walk from step 1,
  which the snc-panel README leaves to the owner ("classification of tw=2 is the owner's call").
claim: >
  Switching on Q parts only after rank 2 gives the same third-row time as tw2 in distribution: the first two rows come
  from the same pair process; after the switch every new pair involves a new Q-bearing entry and is useful provided the
  post-switch jumps carry varying Q parts (a single +Q would make new-new pairs Q-free and return to tame-wild).
  Entries to solve: 1.662 x walk factor, as tw2.
mechanism: "Poisson pair process; new-old and new-new pairs both carry db != 0 and du != 0 after the switch."
prediction_S_w0: >
  late / inc-lazy: k24 0.92 [0.88, 0.96], k28 0.92 [0.88, 0.96], k32 0.93 [0.89, 0.97]; late / tw2cj-lazy 1.00
  [0.97, 1.03] at every k; k20 late / box 0.94 [0.88, 1.00]. Gap after (k32): about 1.76 = 1.78x rho-neg.
simulation: "none run; section 3 cell LT must give rank-3 entries/i.i.d. = D0/i.i.d. within 1 %"
discriminating_test: >
  Stage 0: cell LT. Stage 1: new `late=1` (needs tw=2, cj=1) in WWalkOracle/TargetWalk: run the tw=2 walk with beta = 0
  until rank 2 on (s1, s2), then turn on the chained [gamma]G + [beta]Q parts; replay of committed tw=2 sessions with
  late=0 byte-identical. Arms inc-lazy, tw2cj-lazy, late, rho-neg, A/A; the snc-panel protocol (5 fresh curves, 12
  targets x 8 rounds) at k = 24, 28, 32.
falsification: "late / inc-lazy upper 95 % end >= 1 at two sizes, or late / tw2cj-lazy outside [0.95, 1.05]"
cost_band: >
  w = 0, 0.1, 1 on every arm, table inserts priced (counted:table_insert_attempts); late stores its post-switch points
  (inserts priced) where inc-lazy only looks probes up (lookups listed); rho-neg canonicalisations; per-step mod-r
  coefficient additions listed on every arm (erratum E8).
controls: "tw2cj-lazy (same floor); inc-lazy (parent); late with a single +Q (negative control, must return to about 1.77)"
risks: "It is tw2 under another order; if the owner rules tw2 out for 'target in the walk', this variant probably falls with it. Post-switch rows are rho on Q with known seeds (seed parts cosmetic): relabelling risk the owner must rule on."
not_relabelling: "counted points fall against inc-lazy; nothing moves to unpriced; but see risks on classification"
closest_baseline: "SNC-2 tw=2 (snc-panel); Gaudry-Schost tame/wild; W5 finding 2"
nearest_atlas: LEAD-ICB-generic-table-oracle
lean_obligation: "the 1/r pair lemma (PL-1) and: rank(rows) = 3 implies d determined when every row has du != 0 (linear algebra over Z/r)"
prior: "0.6 resolved gain over inc-lazy at two sizes; 0.85 null against tw2cj-lazy"
```

## 3. Stage-0 specification (zero group operations; Rust, extending `src/bin/nc2_stage0.rs`; not run)

Model as NC-2 Stage 0 (scalar coefficients mod r, x-class collision on c_t = +-c_u, rank over (d, s1, s2), zero-seed rows
dropped). Seeds: `--seed 20261114`, held-out = seed ^ 0xAAAA0001; r = 131203, 2097349, 33558163 (the NC-2 sizes);
n = 20000 samples per cell (SE about 0.0035), cap 10 sqrt(r). Pseudo-code of the new design cells:

```
cell P4 / P5 (lines): L in {4,5}; line i has base vector v_i (v0 = 0, v1 = e_s1, v2 = e_s2, v3 = e_d, v4 = (1,1,1));
  points of line i: v_i + z*G with z from a per-line Sidon-like schedule z_i(t) = t * m^i (m = ceil(sqrt(r)/L)),
  lines grown round-robin; check value E (ideal coverage): 1.869 (L=4), 1.817 (L=5).
cell H3 (planes): plane i = {a*v_i + b*G}, in-plane points by BL grumpy layout per plane (baby (0,j), giant1 (1, n*j),
  giant2 (2, -(n+1)*j)), planes round-robin; check: >= 1.70; ideal-coverage oracle variant 1.636.
cell M2: two walks, 16 disjoint uniform jumps each, alternate steps; compare with one walk, 32 jumps (ratio 1.000+-0.003).
cell J: one walk, j in {16, 32, 64, 128} uniform jumps; check D0/i.i.d. against 1.0352 / 1.0165 / 1.0080 / 1.0040.
cell LT: tw2 walk with beta_h = 0 until rank 2 on (s1, s2), then beta_h uniform; check = D0 (C = 2) within 1 %;
  negative control LT1: a single +Q at the switch, expected about 1.77 x i.i.d.-scale (tame-wild).
```

Readout: E / i.i.d. per cell with SE; a cell is "below Poisson" only if held-out E / i.i.d. < 0.98 at k = 20 and 28.

## 4. Ranked table (expected measured improvement on a strict-IC or C = 2 incumbent, per unit cost)

| rank | id | verdict | predicted effect (k = 28 / 32) | prior | cost | note |
|--:|:--|:--|:--|--:|:--|:--|
| 1 | W6-SC-8 late-target walk | propose | 0.92 / 0.93 of inc-lazy; 1.00 of tw2cj-lazy | 0.6 | 1-2 days Rust; Stage 0 minutes | value only if the owner accepts it where tw2 is doubted |
| 2 | W6-SC-3 degree<=3 reference | sharpen PL-2 | none of its own | 0.8 | Stage 0 cell J, minutes | makes PL-2/PL-4 falsification exact |
| 3 | W6-SC-1 plane/grumpy closure | reject; closure cell | 1.03 / 1.00 (no gain) | 0.03 | Stage 0 cells P4, P5, H3, about a day | closes the grumpy route at C = 2 with numbers |
| 4 | W6-SC-2 multi-walk | reject | 0.987 / 0.983 (= PL-4) | 0.9 equal | none (PL-4 arm) | dominated by PL-4 |
| 5 | W6-SC-4 adaptive x | reject | 0.997 / 0.9995 | 0.05 | none | adaptive = fixed x = 0.707 |
| - | W6-SC-5, -6, -7 | reject | none | - | - | duplicates or inapplicable |

Reading (labelled interpretation): after PL-1..4 and SC-8, the C = 2 track sits at entries about 1.662 x walk factor
(about 1.68-1.69 with PL-2) plus fixed costs, i.e. about 1.7x measured rho-neg at k >= 28. No schedule lever found
here moves the K = 3 floor; the only open route below it remains a construction approaching NC-2's incidence bound, and
the pairwise-independence argument says such a construction must be class-concentrated (planes), which C1 prices at a
loss. All findings are engineering-class or closures; none is an advance.

## 5. Sources checked

- Contract: `AGENTS.md` sections "Preserve scope and report evidence", "Name the obstruction", "Implementation language:
  no Python", "The rule: boundary, table, ratio" (incl. 5a). Not written: session log and `problems/` (the caller
  restricted edits to this file); no Conductor check (no shell; this lane adds one research file).
- On disk: `research/curve25519_leads_wave5_20261112/lanes/pipeline.md`; `research/curve25519_autolab_20261104/README.md`
  (lines 60-150, owner ruling line 99); `.../snc-panel/README.md`; `.../bsgs/README.md`; `.../ideas/NONCOLLISION-20261108.md`;
  `.../ideas/LITERATURE-20261110.md` (lines 15-230); `.../nc1/RESULTS.md`; `.../nc2/RESULTS.md`; grep of `.../ideas/` for
  kangaroo/adaptive/multi-walk; `src/cryptanalysis/ic_framework/plugins.rs` lines 2650-2668 (wwalk parameters);
  `research/leads_20261112/atlas_lead_ids.txt` (ids only; atlas bodies unreadable, `tools/lead_atlas.py` not run).
- Not available: research-kb MCP (no tool in this runtime), ecdb (no curve fact needed beyond quoted files), shell.
- Web (fetched): antsmath.org/ANTSX/bernstein/paper.pdf pp. 2-3, 6-7, 9-10, 13-15; eprint.iacr.org/2012/294 (abstract);
  eprint.iacr.org/2012/458 (abstract); eprint.iacr.org/2010/615 (abstract); math.auckland.ac.nz/~sgal018/Ruprai-thesis.pdf
  p.3; antsmath.org/ANTSIX/slides/bos.pdf pp. 3-8, 12-15; eprint.iacr.org/2015/605 (abstract; PDF 403);
  dspace.cvut.cz Holec thesis pp. 19-22 (secondary). Search snippets only: Teske 2001 abstract, Cheon-Hong-Kim and the
  SNU EC tag-tracing thesis. Galbraith-Wang-Zhang's grumpy-with-negation constant remains unread (403).
