---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Complete F₂/F₃ models and actual degree-two disproofs

Lean checks all 32 binary and 243 ternary general Weierstrass coefficient
tuples. Under nonzero discriminant and the exact retained source trace
predicate, actual point counts belong to {1,3,5} over F₂ and {1,4,7} over F₃.
Every prime divisor has modular embedding degree at most six. Two actual
nonsingular curves have subgroup primes 5/7 and exact degrees 4/6, directly
disproving universal exact degree two without the characteristic restriction.
[pcc:unverified]

![Kernel finite checks, actual point-count bridge and explicit counterexamples. [pcc:unverified]](proof.pdf){width=98%}

| Actual curve | Field | #E | Trace | Prime r | k |
|:--|:--|--:|--:|--:|--:|
| y²+y=x³+x | F₂ |5|−2|5|4|
| y²=x³−x+1 | F₃ |7|−3|7|6|

Actual points of orders five and seven exist in the actual Mathlib groups;
they are not assumed abstract subgroups. The direct negation still holds
with p>2 and r>2, witnessed by the characteristic-three row. The qualified
p≥5 result is not refuted. [pcc:unverified]

\newpage

# Proof map, assurance and scope

The executable count enumerates polynomial-equation solutions plus the
point at infinity. `actual_cardinality_eq_counted` proves an explicit
subtype/Option equivalence to the actual Mathlib nonsingular-point set when
the discriminant is nonzero. Kernel computation proves the finite universal
statements over all five coefficients a₁,a₂,a₃,a₄,a₆. No short-model
restriction, random sample or native arithmetic is substituted.
[pcc:unverified]

| Module | Proved obligation |
|:--|:--|
| `PrimeFieldControls` | Actual curves, nonzero discriminants, complete point counts, trace predicate, actual prime-order points, exact modular orders and direct universal negations |
| `AllPrimeFieldModels` | Generic count equivalence, tuple counts 32/243, closed finite-forall checks, actual count classifications, prime-divisor bound six and combined binding |

Main theorem:
`SmallCharacteristicCensusAudit.all_prime_field_models_audit_certificate`.
Whole gate: 28 public declarations, zero failing and zero trusted-native.
Lean 4.35.0 / Mathlib 56a00585 are pinned. Only propext, Classical.choice,
Quot.sound; no holes, custom axioms, native_decide exception, linter waiver
or added elaboration setting. Gate receipts and all 11 intermediate source/log
files are archived with byte-equal unpack checks. [pcc:unverified]

The source predicate is exactly
`Int.gcd (p+1−Nat.card E.Point) p ≠ 1`; modular embedding degree is
`orderOf (p : ZMod r)`. This does not silently identify modular order with
rationality of the full r-torsion. The all-model result covers extension
degree one over F₂ and F₃ only. Geometric supersingularity equivalence,
higher extension fields and the full torsion-field bridge remain separate.
The general Hasse/classification proof is not inferred from these fields.
[pcc:unverified]

Source commit `c1ca2f5f` precedes theorem packet `PCC-20261110-2661fc`.
Full local verification passes at 10:10:52Z; signed standing remains unverified. Sources and actual verdict/standing are recorded in
`../../sources.json` and `../../verification.json`. The first Cauchy wrapper
failed because its classical equality choice differed from the caller's
computable point-group instance; an explicit equality parameter repairs the
proof without changing the group or claim. Earlier linter warnings were
removed without waivers; unsuccessful sources/logs remain archived.
[pcc:unverified]

Sources checked: Freeman–Scott–Teske (2010), Definition 2.1/Remark 2.2
pp 230–231 and trace-square list p 237; Waterhouse (1969), Theorem 4.1
pp 536–537, with prior visual/read receipts retained in census source plans.
Mathlib affine equation/nonsingular-point equivalence, finite-order and
Cauchy declarations were read. KB/Conductor unavailable; bounded primary,
repository and Mathlib fallback. No novelty, timing, native census, sampling,
security-cost or paid-compute claim. The original fourteen-item audit remains
partial. [pcc:unverified]
