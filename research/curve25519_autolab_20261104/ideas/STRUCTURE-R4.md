# Curve25519-family index calculus, round 4 of proposals: a structure search (Idea Synthesist, 2026-11-04)

Status: **proposals only**. Nothing here is a measurement, a result or a status change. A number is one of:
quoted from a committed file (path given, tagged `[pcc:unverified]`: every autolab outcome is a `statement`
claim with standing `unverified`, and the other in-repo numbers have no claim id and are unverified
assertions); *derived* (arithmetic in this file on quoted numbers, or read from source code, not run); or
*predicted* (to be frozen by the Coordinator before any run). I had read, search and web tools only: no run,
no ecdb query, no Conductor check, no edit outside this file.

## 0. Bottom line

1. **No structure of these curves lowers counted GAE for index calculus without lowering matched rho by the
   same factor, or acting outside the order-r subgroup.** Section 3 goes lever by lever: x-only arithmetic,
   cofactor 8 and its torsion, the quadratic twist, Frobenius, endomorphisms, isogenies, the special prime,
   pairings, and summation polynomials restricted to x-only or twist data. The GAE unit counts one per
   addition or doubling (`docs/ecbench/README.md` lines 452-457), so field-level savings never reach `S`, and
   rho has the same additions to save on. This is a derived argument, not a theorem.
2. **Two levers are index-calculus-only, and both act on the algebraic track's native (w > 0) cost.**
   - The #3408 ingredient: a coordinate in which the rational 2-torsion acts linearly (Edwards `v -> -v`),
     with signed-product nodes (`research/ic_tree_gluing_20261103/round7_edwards/README.md`, arm E1Q).
   - Its extension through the 2-isogenous curve `E' = E/<T0>`. Derived in section 2: `E'(F_p)` has the
     2-part `Z/2 x Z/4`, so the Jacobi-intersection and Huff models, closed for `E` because its 2-part is
     cyclic, exist on `E'`.

   Neither can reach rho. The algebraic incumbent is already 18x rho on group operations alone, at w = 0
   (26.448 against 1.447 at k = 20, `ledger.json` tracks.algebraic) [pcc:unverified] (record I2).
3. **The cross-lane connection: the generic incumbent's headline S is one draw of a random walk.** The wwalk
   table is a deterministic function of the curve, `x`, `jumps` and `seed = 0`. A fresh-target holdout
   re-samples the probe phase only. The train panels behave the same way: the walk-probe and stride-probe arms
   at one `x` differ by the walk-probe set-up and little else, and the spread across `x` matches
   table-to-table variation. The Poisson model's mean is about 1.75-1.86 sqrt(r), against recorded holdouts of
   1.515 (round 33) and 1.352 (round 34 challenger) at k = 32 [pcc:unverified]. The second sits below
   SYNTHESIS section 4 item 8's alarm line (0.9 x 1.662 = 1.496), so two records disagree (section 1).
4. **Test first: record I1.**
   - Stage 0 reads committed session counters: zero compute.
   - Stage 1 runs the incumbent under 16 walk seeds. It needs no code: `seed` is an existing `wwalk` parameter.
   - Either outcome closes a question. If the seed-averaged S sits near 1.8, the expected distance to rho is
     about 1.8x, not 1.43x-1.55x [pcc:unverified], and the generic floor stands. If it stays near 1.5, the
     walk is below the generic Poisson count, and that is the one empirical lead toward "structure" to audit.

## 1. The connection not visible lane by lane

**What the source says** (read, not run):
- The two seeds are the first two cofactor-projected abscissae from `x = 1..scan_cap`, the same on every run
  (`src/cryptanalysis/ic_framework/plugins.rs` 348-373).
- The walk RNG is `seed ^ mix(key(P1)) ^ mix(key(P2))` (`src/cryptanalysis/ic_boundary.rs` 7996-7998), with
  `seed` defaulting to 0 (`plugins.rs` 2630-2632). No wwalk family in `families.json` sets it (grep).
- The run seed reaches `PipelineSpec.seed`, which drives the probes, not `oracle_params`
  (`src/cryptanalysis/ecbench/methods.rs` 5758-5781).

So jumps, steps, collisions, rows and `build_ops` repeat in every run of an arm on one curve. With
`g=1,grow=2` the table is about three quarters of S (model below).

**What the panels show** (quoted values [pcc:unverified]; differences derived):

| check | k = 24 | k = 28 | k = 32 |
|:--|--:|--:|--:|
| round 33 train: mean of (walk-probe S − stride-probe S) over the six `x` values [pcc:unverified] | 0.72 | 0.26 | 0.07 |
| SYNTHESIS R3's 16-jump walk set-up, 832-1408 GAE / sqrt(r) (derived) | 0.71 | 0.21 | 0.06 |
| round 33 train, walk-probe arms, mean and SD over the six `x` values [pcc:unverified] | 2.92, 0.47 | 2.17, 0.25 | 1.77, 0.34 |
| round 33 holdout of the selected arm (480 runs) [pcc:unverified] | 1.673 | 1.572 | 1.515 |
| round 34 holdout, incumbent and challenger [pcc:unverified] | 1.825 / none | 1.517 / 1.488 | 1.536 / 1.352 |
| round 34 holdout rho-neg [pcc:unverified] | 1.135 | 0.947 | 0.838 |

- The walk-probe and stride-probe arms at one `x` share a table. They differ by the probe set-up only, and
  that difference matches R3's estimate at all three sizes.
- Different `x` values give different `T` and `w = round(1.2 sqrt(T))`, so a different jump set and a fresh
  table draw. The spread across `x` is the table lottery, and train selection takes its minimum.

**Model (derived).** A pair of entries collides with probability 2/r, so the rows arrive as a Poisson process
with mean `n^2` at `n sqrt(r)` entries. The second row arrives at `n_2 = sqrt(Gamma(2))`, with mean 1.329. The
probes then need one hit at rate `2n/sqrt(r)`, which costs `1/(2n)`. So for one table draw:

`S = max(n_2, x) + 1/(2 max(n_2, x)) + set-up`, with a cap at `3x`; capped runs need 2-3 hits.

| quantity (derived) | value |
|:--|:--|
| mean at x = 0.6, walk probes, k = 32 | about 1.86 (table 1.29, probes 0.49, set-up and row checks 0.09) |
| mean at x = 0.9, stride probes, k = 28 | about 1.82 |
| SD across walk draws | about 0.35 |
| minimum over draws | sqrt(2) = 1.414, plus set-up |
| P3 outcome note, static `g=1,grow=2` table on train-k32 (16 runs) [pcc:unverified] | 1.77 |
| round 33 holdout 1.515 at k = 32 [pcc:unverified], read through the model | a table that reached rank 2 between 0.6 and 0.9 sqrt(r): about a 5th-20th percentile draw; 1 − 0.8^6 = 0.74 is the chance that the best of six draws is that low |

**The contradiction, stated without choosing a side.**
- SYNTHESIS section 0 and section 4 item 8 put the C = 2 all-pairs floor at `Gamma(3.5)/Gamma(3)` = 1.662.
  They read any S below 1.496 as an accounting error first. PROPOSALS-R3 section 0 says the same of anything
  below about 1.5.
- Round 34's k = 32 holdout challenger is 1.352 [pcc:unverified], classified null with band `sel < inc`.
  Round 33's 1.515 and round 34's 1.488 at k = 28 [pcc:unverified] sit below 1.662 but above 1.496.
- Rho-neg on the round 34 k = 32 holdout reads 0.838 [pcc:unverified], 5% under its expectation
  `sqrt(pi/4)` = 0.886. A common-mode low draw on that target set would put the challenger near 1.43,
  inside the model's range.

Record I1 decides which reading holds.

**Why no lane saw it.**
- The autolab compares arms paired on targets. A shared deterministic table cancels in `inc-aa / inc` (the
  A/A interval is exactly [1.000, 1.000] in rounds 33 and 34 [pcc:unverified]), but not between two wwalk
  configurations, which draw different tables.
- PROPOSALS-R3 modelled the walk's expected cost. SYNTHESIS set the floor. Neither looked at how the walk is
  seeded.

## 2. The curves, as built (from `research/curve25519_ic_baseline_20261102/analysis/family.json`)

Construction rule (same file):
- `p` is the largest prime below `2^k` with `p ≡ 5 (mod 8)`.
- `A ≡ 2 (mod 4)` is the smallest value with `A^2 − 4` a non-square, `#E = 8r`, `#E' = 4r'`, and both r and r'
  prime.
- The curve is the Montgomery form `y^2 = x^3 + A x^2 + x`. Its 2-Sylow is cyclic `Z/8`, with `T0 = (0,0)` and
  4-torsion at `x = 1` or `x = −1`.
- ecbench runs it in short Weierstrass form.

The curve facts in the table (p, A, r, r', t, 4-torsion, D_K, f) are as recorded by `curve25519_family family`
in that file [pcc:unverified]; the last three columns are derived here.

| k | p | 2^k − p | A | r | r' (twist) | trace t | 4-torsion at x | fundamental disc D_K, conductor f | least degree of a non-scalar endomorphism, abs(D_K)/4 (derived) | same / sqrt(r) (derived) | p − 1 (derived) |
|--:|--:|--:|--:|--:|--:|--:|--:|:--|--:|--:|:--|
| 20 | 1048573 | 3 | 926 | 131203 | 261881 | −1050 | −1 | −6388, 22 | 1597 | 4.4 | 4·3^3·7·19·73 |
| 24 | 16777213 | 3 | 318 | 2097349 | 4193909 | −1578 | −1 | −16154692, 2 | 4038673 | 2789 | 4·3·23·89·683 |
| 28 | 268435157 | 299 | 5330 | 33557809 | 67101961 | −27314 | −1 | −81921508, 2 | 20480377 | 3535 | 4·67108789 (not factored) |
| 32 | 4294967197 | 99 | 27842 | 536884639 | 1073714321 | −109914 | 1 | −1274695348, 2 | 318673837 | 13753 | 4·1073741799 (not factored) |

Derived facts used below:
- **Least endomorphism degree.** `D_K ≡ 0 (mod 4)` at every size, so every non-integer element of `O_K` has
  norm at least `abs(D_K)/4`. Since `End(E)` contains `Z[pi]` and lies in `O_K`, every endomorphism other
  than `[n]` has at least this degree.
- **No extra automorphisms.** No curve in the isogeny class has CM by `Z[i]` or `Z[zeta_3]`, so no automorphism
  of order above 2 exists anywhere in the class.
- **Small roots of unity.** `v2(p − 1) = 2` at every size, since `p ≡ 5 (mod 8)`: `mu_(2^t)` lies in `F_p`
  only for `t ≤ 2`.
- **The 2-isogenous curve.** For `Y^2 = X(X^2 + aX + b)`, the 2-isogenous curve `E' = E/<(0,0)>` is
  `Y^2 = X(X^2 − 2aX + a^2 − 4b)`. Its quadratic factor has discriminant 16b, a square here (b = B^2). So `E'`
  has full rational 2-torsion, and with `#E' = 8r` its 2-part is `Z/2 x Z/4`.
- **Twist pairs.** Every `x` with `f(x) ≠ 0` lies on exactly one of the curve and its twist, and
  `#E + #E'` equals `2p + 2` (checked at k = 20: 1049624 + 1047524 = 2097148).

## 3. Lever by lever

Rule applied throughout: a lever counts only if it lowers the method's counted cost and rho cannot use it as
well (AGENTS.md section 5a and the matched-reference rule). "Ratio" is `S(method)/S(rho-neg)` at w = 0.

| lever | mechanism | does rho get it? | estimate of the ratio change | cheapest test in this repository | falsification of "no lever" |
|:--|:--|:--|:--|:--|:--|
| x-only (Montgomery ladder, differential addition) | `xADD` needs `x(P − Q)`, so x-only arithmetic is limited to differential chains: ladders and progressions `Q + kG`. Every `xADD` or `xDBL` counts 1 GAE; a ladder costs 2 GAE per bit against the 1.5 log2 r convention, so it costs more GAE. Table walks and rho steps are not differential and need full additions. The honest x-only collision structure is the x-key itself: the class `{P, −P}`, used by both arms (gamma = 2). | Yes. Field savings apply to rho's additions too; rho cannot use x-only steps any more than the walk table can. | 1.00 in GAE. In `bitop/v1` about 1.0: a projective `xADD` plus batched normalisation for the hash key costs about as much as a batched affine addition (derived). | Optional: `ecbench bound fit --unit bitop/v1` on {rho.negation, incumbent} at k = 24. | The bitop ratio differs from the GAE ratio by more than 10%. That would be a field-level asymmetry, still priced outside GAE. |
| x(P+Q) and x(P−Q) from one square root (X ± Y, Galbraith-Wang-Zhang) | Two points per counted operation | Rho can use it equally (GWZ) | Accounting change: mitmbatch and mitmblocked recorded identical S (SYNTHESIS section 3, rounds 5 and 21) [pcc:unverified] | none | — |
| cofactor 8, `E[8]`, T0 on the Kummer line | Torsion translations are free but leave `<G>`. Keys modulo `±1` and a torsion subgroup H give about `8r/(2 abs(H))` classes, at least r/2, with equality exactly for the full rational torsion, which is projection. Keys in `xi = x + 1/x` identify `{±P, ±P + T0}`, which meets `<G>` only in `{±P}`. | Rho works in `<G>` natively | 1.00 beyond projection. Projection already recovers the factor 8: trials per relation 7.66-8.38 vs a cofactor-1 control at 1.00 (`curve25519_ic_baseline_20261102/README.md`) [pcc:unverified] | Done: baseline P1; `curve25519_t0_symmetry_20261102` | A torsion-invariant key with more than 2/r collisions per pair among subgroup elements |
| quadratic twist (shared x-line) | A mixed pair (P1 on E, P2 a twist point, that is `pi(P2) = −P2` over `F_(p^2)`) has `pi(P1 + P2) = P1 − P2`. The two roots of `S3(x1, x2, X)` are then Frobenius conjugates, rational only if `2P1 = 0` or `2P2 = 0`. Twist-twist relations live in the twist's group of order 4r', and r' ≠ r. | Rho does not use the twist | 1.00 | Optional unit test in `src/bin/curve25519_family.rs`: count the `F_p` roots of `S3` over all mixed pairs at k = 16; predicted 0 outside 2-torsion | Any rational mixed root off 2-torsion |
| Frobenius, small characteristic | `pi` acts as 1 on `E(F_p)`. Its other eigenline in `E[r]` is defined only over `F_(p^e)`, with e of order r. The characteristic is about 2^k, so no Koblitz or Weil-descent structure exists. | Not applicable | Not applicable | none | — |
| endomorphisms, GLV | Least non-scalar degree is 1597 to 3.2e8, that is 4.4 to 13,753 sqrt(r) (section 2). One evaluation costs at least of order that many field operations: at k = 20 a single application is comparable to all of matched rho. The eigenvalue on `<G>` has multiplicative order of size r, so no orbit folding. GLV would only shorten scalar multiplications (set-up, under 4% of S at k ≥ 28). | Yes: GLV and any automorphism fold rho as well (SYNTHESIS section 3) | 1.00 | none (derived from family.json) | A non-scalar endomorphism of degree below sqrt(r) on any miniature: excluded by `abs(D_K)/4` |
| isogenies to curves with cheaper arithmetic | `RESEARCH_ISOGENY_FIBERS_ECDLP.md` Theorem 15.1, as summarised in `ic_tree_gluing_20261103/round7_beyond/DIRECTIONS.md` item 3: rational isogeny maps add nothing in the generic model [pcc:unverified]. Cheaper formulas on an isogenous curve are field-level. | Yes: `transfer.isogeny` (ecbench registry) carries rho across too | 1.00 in GAE | none | — |
| special prime (`2^k − c` with c = 3, 3, 299, 99; `p ≡ 5 mod 8`) | Cheap reduction lowers the cost of a field multiplication. GAE cannot see it, since S counts group operations by definition. A gate unit sees it equally in every arm that does field arithmetic. Atkin's one-exponentiation square root helps only square-root-heavy oracles (base scan, rootfind), which measured 70x-320x worse than the incumbent (round 3) [pcc:unverified]. | Yes for reduction; the square-root saving is IC-only but a constant on native work | 1.00 in GAE; in a gate unit the IC/rho ratio, if anything, rises (IC carries more non-field native work) | none | — |
| pairings, anomalous | Embedding degree is at least 1000 at k ≥ 20 (432 at k = 16), and the trace is not 1 (family.json) [pcc:unverified] | — | closed | none | — |
| summation polynomials, x-only | `S3` in Montgomery x has the `T0` symmetry. Quotienting on the x-line (`xi = x + 1/x`, arm TQ) left D = 6 = TW (round 7c) [pcc:unverified]: the quotient pays only in a coordinate where `T0` acts linearly. | Rho runs no polynomial systems | Algebraic track only: record I2 | I2 Stage 0 | — |
| linear 2-torsion quotient (#3408) on E, and `(Z/2)^2` on `E' = E/<T0>` | Signed-product nodes: each 2-to-2 node becomes two quadrics. Round 7b measured D 6 -> 5 (m = 3) and 7 -> 5 (m = 4, t = 4), with msolve wall 2.5 s vs 697 s on a non-family prime with `p − 1 = 2^18·3` [pcc:unverified]. | No: IC-only | Native cost only. S at w = 0 is unchanged (same trial count). Parity is excluded by SYNTHESIS Table 2's budget `c*` | Record I2 | Record I2 |

Not a curve lever, and so not costed here: multi-target amortisation, PROPOSALS-R3 P4, which uses its own
gauge against pooled rho.

## 4. Idea records

```yaml
id: R4-I1-walk-seed-draw
title: "Re-sample the wwalk table, not only the targets: is the incumbent's S a property of the method or of one walk draw?"
track: generic decomposition (C = 2, owner-ruled 2026-11-04 to count); class if it moves numbers: accounting (the algorithm does not change)
obstruction_attacked: >
  The holdout rule varies the targets while the wwalk table (seeds, 16 jumps, step stream, rows, build_ops) is a
  deterministic function of (curve, x, jumps, seed = 0), so the table phase, about 3/4 of S, is never re-sampled
  and train selection over x picks the luckiest of six to twelve table draws (section 1).
claim: >
  (a) Within one wwalk arm on one curve, setup_ops, walk_ops, verify_ops, grown_steps and the free-row set are
  identical in every run. (b) Over walk seeds s = 1..16, the mean S(w = 0) of `wwalk:x=0.6,g=1,grow=2` (walk
  probes) at k = 32 is 1.80 [1.65, 1.95], and of `wwalk:x=0.9,g=1,grow=2` with `targets=stride-generator` at
  k = 28 is 1.82 [1.66, 1.98], against recorded holdouts 1.515 and 1.572 [pcc:unverified]; the across-seed SD
  is 0.25 to 0.45; at most 6 of the 16 seeds fall below the seed-0 value on the same targets.
mechanism: >
  Rows arrive as a Poisson process of mean n^2 (pairs times 2/r); the second row at n_2 = sqrt(Gamma(2)); one
  probe hit at rate 2n/sqrt(r) after both seed logs are pinned. S = max(n_2, x) + 1/(2 max(n_2, x)) + set-up,
  capped at 3x. Mean 1.82-1.86 and SD about 0.35 over draws; minimum sqrt(2) + set-up (derived, section 1). The
  round-33 train spread over x (SD 0.34 around 1.77 at k = 32) and the P3 unit trend (1.77) [pcc:unverified]
  match the model, the selected holdouts sit in its lower tail.
prediction:
  stage0: "zero variance of walk_ops and build_ops across the 480 runs of each wwalk arm in ECBS1h8b22612ef731 (k32), ECBS1hba478ffc1d23 (k28) and the round-34 k32 holdout; mean probes within 10% of sqrt(r)/(2n), n = entries/sqrt(r); phase sum = total_gae"
  stage1_mean_S_w0: {k32_x06_walk: "1.80 [1.65, 1.95]", k28_x09_stride: "1.82 [1.66, 1.98]"}
  stage1_seed0_rank: "seed 0 in the lowest 7 of 17 at both sizes"
  distance_to_rho_if_confirmed: "about 1.8x at k = 28 and 32 (1.80 / 0.975, 1.82 / 1.052, rho from round 33 holdout [pcc:unverified]), not 1.49x-1.55x"
discriminating_test: >
  Stage 0 (exact, no run): `python3 tools/artifacts/artifacts.py pull` the round-33 and round-34 holdout session
  records and read per run: setup_ops, walk_ops, verify_ops, steps, grown_steps, entries, free_rows_emitted,
  probe count, probe_hits, phase GAE, total_gae. Stage 1 (config only, no code): one ecbench panel per size,
  arms {inc (seed 0), the same oracle with `seed=1` .. `seed=16`, rho.negation, inc-aa}, 32 fresh targets each
  (a target_seed disjoint from `ledger.json` exposures), `ecbench verify --replay`, `ecbench band --json
  --ranks`. No seed may be selected from this panel (AGENTS.md "choosing favourable seeds" is inadmissible);
  the report is the seed-averaged S and its across-seed SD.
falsification: >
  Hypothesis dead if: Stage 0 shows walk_ops varying across runs of one arm (my reading of the seeding is
  wrong); or Stage 1 mean over 16 seeds below 1.62 at k = 32 or below 1.64 at k = 28; or across-seed SD below
  0.12. Then the incumbent is below the C = 2 all-pairs Poisson floor (1.662) in expectation, which SYNTHESIS
  section 4 item 8 reads as an accounting error first: next, the per-pair collision rate (collisions over
  steps^2/2 against 2/r) and a phase-sum recount. A rate above 2/r that survives the recount would be the
  first measured non-generic effect on this family, the lead this search did not find by reasoning.
cost_band: >
  Stage 0: no compute beyond the artifact pull. Stage 1: 19 arms x 32 targets x 2 sizes = 1,216 runs, about
  1.8 sqrt(r) GAE per IC run (2.5e7 GAE at k = 32, 6.3e6 at k = 28): minutes of CPU on the frozen route.
  Counted: walk steps, set-up multiples, row recomputation, probe additions, the internal [d]G = Q check.
  Unpriced: one hash insert per walk step, one lookup per probe, row operations on 3 columns; every arm,
  rho-neg included, priced at w = 0, 0.1 and 1. The seeds change no counter definition.
controls: [inc-aa (must be exactly 1 again), rho.negation on the same targets, "seed 0 arm (must reproduce the recorded value within probe noise: 1.515 +/- 0.1 at k = 32)", "wsum2g incumbent configuration (deterministic box table: no seed effect expected)"]
risks:
  - "If the oracle mixes `seed` weakly (seed ^ key mix), nearby seeds may correlate; use widely spaced seeds (for example s * 0x9E3779B97F4A7C15) and report the 16 jump sets' distinct count."
  - "Rho's own walk may also be seeded per curve; the matched reference is then one draw too. Stage 0 reads rho's per-run jump-table counters as well."
  - "A confirmed draw effect touches rounds 31-34 (wwalk arms); how the ledger treats it is the Coordinator's and owner's decision, not this record's."
closest_baseline: "rounds 31-34 (wwalk family); PROPOSALS-R3 P2 and its Outcomes note (1.77 unit trend); SYNTHESIS section 4 item 8"
code_needed: {files: ["none in source; the Coordinator adds 16 `families.json` entries or a panel with `wwalk:...,seed=s`"], ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=2,scan_cap=32, oracle=wwalk:x=0.6,g=1,grow=2,seed=S"}
lean_obligations: none (no mathematical claim; every recovered log is checked by ecbench)
parity_statement: "Moves no method. If confirmed, the expected distance to matched rho is about 1.8x at k = 28, 32 and the generic table-probe floor of about 1.41 + set-up holds."
reduces_to: "Poisson pair count of a stored random walk (Kuhn-Struik style); BSGS with a known-log table of random size"
```

```yaml
id: R4-I2-linear-2torsion-quotient-family
title: "Carry #3408's linear 2-torsion quotient to Curve25519-family primes, and to the 2-isogenous curve whose 2-part is Z/2 x Z/4"
track: algebraic (summation-polynomial decomposition); index-calculus-only (rho runs no polynomial systems); acts on native solver cost only
obstruction_attacked: >
  The algebraic incumbent's native F4 cost. k = 20 `pkm-iso-t6-g4-s1093` has S = 26.448 / 1.073e6 / 1.073e7 at
  w = 0 / 0.1 / 1 against rho-neg 1.447 / 1.547 / 2.448 (`ledger.json` tracks.algebraic; session
  ECBS1h405f20219036) [pcc:unverified]. Two transfer obstructions are named here. First, family primes have
  v2(p − 1) = 2, so round 7's base v in g·mu_(2^t) (built on p − 1 = 2^18·3) exists only for t ≤ 2. Second, the
  ecbench incumbent is m = 2, while round 7 measured m = 3, 4.
claim: >
  Arm A (on E): a complete twisted Edwards model of each miniature, with (a, d) = (A + 2, A − 2) or swapped by
  (u, v) -> (u, 1/v). For k = 20, 24, 28, A − 2 is the square (4-torsion at x = −1), so check which pair is
  complete. Base v in g·mu_(4d'), d' odd with d' | p − 1 (k = 20: d' = 27, 108 v-values closed under
  v -> −v), with cubic remainder towers. E1Q (signed products, W = w^2) then has D at most D_E1 − 1 at m = 3
  and 4, and quotient dimension exactly D_E1's / 2^(m−1). Arm B (on E' = E/<T0>): a twisted
  Jacobi-intersection model of E', where translations by the three rational 2-torsion points act by
  coordinate sign changes (literature, FGHR J. Cryptology 2014; re-derive in the generator). The (Z/2)^2
  quotient has quotient dimension divided by 4^(m−1) and D at most D_E1Q. Arm m2: at m = 2 (the ecbench
  incumbent's arity), E1Q lowers msolve's widest matrix by at least 2x.
mechanism: >
  Round 7c [pcc:unverified]: the D drop needs a coordinate where the 2-torsion acts linearly, so the quotient
  keeps signs as products and each node splits into SI(squares) − 2(a − d)·p = 0 and p^2 = product of squares
  (input degree 4 -> 3). The x-line quotient (TQ) did not drop D. E' has a second independent rational
  2-torsion point, so a model exists where (Z/2)^2 acts by signs: twice as many sign variables to quotient per
  summand. The lab closed Jacobi intersection and Huff for E only because E's 2-part is cyclic
  (`curve25519_frame_search_20261103/README.md` line 20; LEADS.md "Closed"). The 2-isogeny phi has degree
  prime to r, so it maps <G> isomorphically onto the order-r subgroup of E'; mapping base and target costs a
  few field operations per point.
prediction:
  stage0_D: {armA_m3: "D_E1Q = D_E1 − 1", armA_m4: "D_E1Q ≤ D_E1 − 1", armB_m3: "D_JIQ ≤ D_E1Q", m2: "widest(E1Q) ≤ 0.5 widest(E1)"}
  stage0_qdim: {armA: "ratio 2^(m−1) exactly", armB: "ratio 4^(m−1) against the unquotiented JI arm"}
  probability_stage0_passes: "armA 0.5 (the 2-power tower is gone), armB 0.3, m2 0.4 (judgements)"
  stage1_if_stage0_passes: "k = 20 algebraic arm with the quotient model: S(w = 0) within ±5% of 26.448; S(w = 0.1) down by 2x to 300x (round 7b wall ratios 24x-280x at m = 3, 4 [pcc:unverified], fewer at m = 2); band sel < inc at w = 0.1 and 1"
  distance_to_rho: "w = 0: about 18x, unchanged; w = 0.1: from 6.9e5x to at best about 2e3x. Parity excluded: SYNTHESIS Table 2 gives c*(m, k) ≤ 970 native operations per trial for m ≤ 6, far below any F4 run at D ≥ 4."
discriminating_test: >
  Stage 0 (no ecbench): a native Singular generator in the style of
  `research/ic_tree_gluing_20261103/round7_edwards/gen7.sing`, with a planted check before every write, on
  the k = 20 miniature (p = 1048573, A = 926) and its 2-isogenous curve. Cells m in {2, 3, 4} x arms {T, E1,
  E1Q, JI, JIQ} x 2 targets. msolve 0.6.5, `-t 1`, 2 GiB, 1,200 s, read with `../parse.sh`. Stage 1 only if
  Stage 0 passes: a model option on `pkm-semaev` (ecbench `ic.pipeline`, factor base `prime-isogeny:t=6,
  seed=1093`), panel {incumbent, quotient arm, rho.negation, inc-aa}, 6 targets x 2 rounds, as T5 did.
falsification: >
  Arm A dead if D_E1Q ≥ D_E1 in every comparable cell on the family-prime base. Arm B dead if the generator
  finds the 2-torsion action on E' is not by sign changes in any F_p-rational model, or D_JIQ ≥ D_E1Q
  everywhere. Stage 1 dead if paired sel/inc at w = 0.1 has 95% interval containing 1, or S(w = 0) rises by more
  than 5% (relabelling), or any unchecked recovered log.
cost_band: >
  Stage 0: at most 30 cells x 1,200 s on one core (under 10 h worst case; round 7's cells took 0.02-700 s),
  no GAE (a stage diagnostic, not a pipeline price). Stage 1: 4 arms x 12 runs at k = 20, minutes to hours per
  run (the incumbent's native cost). Counted: target and base points, the isogeny images (few field
  operations each, tallied as native), relation trials, the final [d]G = Q check. Unpriced at w = 0: F4
  multiply-adds, square roots, tower reductions; priced at w = 0.1 and 1 by `ecbench band`. Rho-neg priced
  identically. It gains nothing from the quotient, which is the point of the lever.
controls: [unquotiented E1 and JI arms on the same base, the Weierstrass T arm, the x-line TQ arm (negative control: no D drop predicted), a second target per cell, inc-aa in Stage 1]
risks:
  - "Cubic towers raise input degree; the round-7 drop may not survive them (the reason armA is 0.5)."
  - "At m = 2 the tree has one node; the quotient may only halve redundancy without lowering D."
  - "Equal-base comparisons need class counts, as round 7c corrected."
  - "The FGHR action on Jacobi intersections is quoted from memory; the generator must re-derive it."
closest_baseline: "round7_edwards arms E1, E1Q, TW, TQ; FGHR (J. Cryptology 2014); research/curve25519_t0_symmetry_20261102; LEADS.md T7"
code_needed:
  files:
    - "a new Singular generator under a new research directory (Stage 0)"
    - "src/cryptanalysis/ic_framework/pkm_semaev.rs: a quotient model option (Stage 1 only)"
    - "reuse of the 2-isogeny map behind `transfer.isogeny` for arm B"
lean_obligations: ["e1q_lift: every E1Q solution with W, Y nonzero lifts to exactly 2^(m−1) E1 solutions (round 7b sketch)", "isogeny_subgroup_iso: a degree-2 isogeny restricts to an isomorphism of the order-r subgroups (r odd)"]
parity_statement: "Cannot reach matched rho: the w = 0 term (18x, trial count) is untouched and c* is far below any F4 cost. At best it ratchets the algebraic track's own native cost."
reduces_to: "FGHR symmetry reduction; Theorem 15.1 for the isogeny transport (no generic gain, coordinate effect only)"
```

## 5. Ranking

The ranking orders levers by expected information times probability. None is expected to bring S to matched
rho.

1. **I1, walk-seed draw** (not a structure lever; it tests the baseline every structure lever is measured
   against). Zero code; Stage 0 needs no compute.
2. **I2, arm A**: the #3408 quotient on family primes. IC-only, algebraic, native cost only.
3. **I2, arm B**: the 2-isogenous `E'` with `Z/2 x Z/4` and a `(Z/2)^2` sign quotient. IC-only. It reopens
   models the lab closed for `E` alone.
4. **x-only, Kummer**: ratio 1.00 in GAE. Ladders count 2 GAE per bit, and only progressions are differential;
   progressions cluster their hits (rounds 29, 30) [pcc:unverified].
5. **Cofactor 8, torsion, T0 keys**: projection already gives the minimal class count r/2; ratio 1.00.
6. **Twist**: mixed `S3` roots are Frobenius conjugates, and twist relations live in a group of order 4r';
   ratio 1.00.
7. **Endomorphisms, GLV, isogenies**: least non-scalar degree `abs(D_K)/4` (1597 at k = 20, 3.2e8 at k = 32);
   no automorphism of order above 2 in the isogeny class; rho is granted the same transfers; ratio 1.00.
8. **Special prime, Frobenius, pairings**: wall-time or not applicable; the GAE ratio cannot move.

## 6. Obstruction to record

I may not write `problems/`. Suggested record: "On the Curve25519-family miniatures, every curve structure
(±1, `E[8]`, `T0` on the Kummer line, the twist, Frobenius, endomorphisms, isogenies, the special prime) is
either used by matched rho as well, or acts outside the order-r subgroup, or is priced as native work. The
generic-decomposition floor is therefore not lifted by structure, and index calculus can reach rho parity
only through an algebraic oracle meeting SYNTHESIS Table 2."
- Status: a derived argument, not a theorem.
- Exit criteria: SYNTHESIS section 6 items 1-3. Add: "I1 Stage 1 mean below 1.62 at k = 32 with a clean
  recount".

## 7. Evidence consulted, and what I did not do

**Read:**
- `research/curve25519_autolab_20261104/README.md`, `ideas/PROPOSALS-R3.md` (with its Outcomes section),
  `ideas/SYNTHESIS.md`, `round-033/README.md`, `round-034/README.md`, `ledger.json` (tracks), `families.json`
  (grep)
- `research/curve25519_ic_baseline_20261102/README.md`, `STRUCTURE.md`, `analysis/family.json`
- `research/curve25519_torsion_leads_20261103/LEADS.md`
- `research/curve25519_frame_search_20261103/README.md` (grep)
- `research/ic_tree_gluing_20261103/README.md`, `round7_edwards/README.md` (#3408), `round7_beyond/DIRECTIONS.md`
- `docs/ecbench/README.md` 430-519
- `AGENTS.md`: "Preserve scope", "Name the obstruction", "Intermediate-cost and asymmetric-access", "The rule:
  boundary, table, ratio"
- `tools/pcc/pcc.py` (the lint-prose word list)
- Source: `ic_boundary.rs` 7722-8099, `plugins.rs` 302-373 and 2596-2735, `ecbench/methods.rs` 5758-5898

**Not done:**
- No run, no ecdb query (no tool), no Conductor check.
- No session log: the brief limited edits to this file.
- The lead atlas is not on this machine: `/home/user/crypto-autoresearcher/tools/lead_atlas.py` does not
  exist, and a glob over `/home/user` timed out. `--match` was not run.
- Session records were not pulled, so I1 Stage 0 is unchecked. My reading of the walk seeding comes from
  source only.
- `p − 1` at k = 28 and 32 is not factored. The FGHR action on Jacobi intersections is from memory.
- The model numbers are hand arithmetic, with roughly ±0.05 numerical error.
