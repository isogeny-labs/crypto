# A prime-five smooth Cantor cycle over F3

Lean proves `cycleFraction≠ambientFraction` for the actual polynomial-pair
model of y²=x⁵+1/F3: the complete model has10pairs/8split-u pairs, while
a proper five-pair cyclic arithmetic image has all5members smooth.
The actual generator is(u,v)=(X,1), with order5different from characteristic3.
All25additions in that image match explicit Cantor certificates.
[pcc:unverified]

The full tracked project gate passes348declarations,0failures and0trusted-native
exceptions, including generated declarations. The only accepted foundations
are propext, Classical.choice and Quot.sound. Lean4.35.0/Mathlib56a00585
are pinned. No sorry, custom mathematical axiom or native evaluation is
used. See [full gate](evidence/stoll-cycle-gate.log). [pcc:unverified]

The complete coefficient classification ranges over all91bounded shapes,
then proves a bijection to every actual monic-u pair with degreeu≤2,
degreev<degreeu and u dividing v²−f. It is not a count of an assumed list.
Exactly two of the ten pairs have nonsplit u=X²+X+2; their v are±X.
Actual Polynomial.Splits is used throughout. [pcc:unverified]

The cyclic image contains(1,0),(X,±1),(X²,±1). For every ordered input
pair, the proof supplies common-divisor and Bezout witnesses identifying
the actual normalized full gcd d=gcd(gcd(uP,uQ),vP+vQ), then verifies the
monic composition quotient, numerator division/remainder and required
reduction. Only after matching all25steps is the Fin5group transported
to these actual pairs. This establishes generation/order5and fraction1,
compared with ambient8/10=4/5. [pcc:unverified]

| Obligation | Lean declaration |
|:--|:--|
| Every bounded actual pair is classified | `MumfordStollAudit.canonical_valid_shape` |
| Complete actual counts10and8 | `reducedPair_cardinality`, `smooth_reducedPair_cardinality` |
| Curve degree5/monic/separable | `curve_degree`, `curve_monic`, `curve_separable` |
| Every actual cycle addition | `MumfordStollCantorAudit.cycle_all_pairs` |
| Actual normalized gcd from certificates | `MumfordStollCycleGroupAudit.normalized_full_gcd`, `step_has_actual_full_gcd` |
| Actual final remainder | `reduction_matches_remainder` |
| Transferred addition equals actual arithmetic | `addition_is_Cantor` |
| Actual generator coordinates/order/generation | `generator_coordinates`, `generator_order`, `generated_by_actual_point` |
| Proper image/cardinality/splitting | `cycle_is_proper`, `cycle_cardinality`, `cycle_smooth_cardinality` |
| Exact unequal fractions | `exact_proportion_inequality` |
| Combined binding | `stoll_cycle_audit_certificate` |

The formal full ten-pair ambient group operation and geometric
Picard/Jacobian identification remain separate. This result proves a
finite arithmetic cycle inequality. It does not refute Gaudry's
fixed-genus limit or a qualified large-prime/typical-subgroup heuristic.
The original fourteen-item audit remains partial. [pcc:unverified]

Source/plan commit327d3183precedes scalar supplementIDC1hb7102c15384f2648
and theorem packetPCC-20261109-d8b77b. The supplement checks32derived and
11extra points for the scalar quadratic quotient/remainder identity only:
5variables/degree5over2⁶¹−1. Its raw4117bytes are preserved exactly.
Counts, splitting, actual gcd, all25steps, order and fraction inequality
rest on Lean. Signed standing remains unverified. [pcc:unverified]

The full local PCCcheck passes23:00:19Zon producerhost762f5fc120c9;
standing remains unsigned/unverified. The final two-page report and diagram
were visually inspected. See [report PDF](reports/proof-20261109-stoll-cycle/report.pdf). See [source plan](stoll-sources.json),
[actual count source](lean/MumfordStoll.lean),
[Cantor certificates](lean/MumfordStollCantor.lean),
[cycle group source](lean/MumfordStollCycleGroup.lean),
[proof diagram](reports/proof-20261109-stoll-cycle/proof.svg), and
[lossless build-attempt archive](evidence/stoll-build-attempts.tar.gz).
Failed and warning-bearing attempts are not passing proof evidence.
[pcc:unverified]

Primary source: Stoll2014Example4.19pp25–26was read in full, together
with the representation/addition sections recorded in cantor-sources.json.
Its rational worked example motivates the cycle; the characteristic3and
complete ten-pair assertions are separately checked here. Original
Cantor1987remains unread after publisher403. No novelty claim is made.
[pcc:unverified]
