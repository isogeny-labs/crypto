# Conditional trace-class bound and a composite-order control

The Lean theorem `trace_classification_audit_certificate` in
SupersingularTraceClassesAudit proves the following statement for an actual
Weierstrass curve over any finite field K. Write q=|K|, N=#E(K) and
integer t=q+1−N. Given prime r dividing N and the explicit classification
`t² ∈ {0,q,2q,3q,4q}`, the multiplicative order of q modulo r belongs to
{1,2,3,4,6}, hence is at most 6. In addition r is coprime to q and an
actual Mathlib curve point has additive order r. [pcc:unverified]

The classification is a premise. This file does not prove the classification
from geometric supersingularity, or identify modular order with the degree
of the actual field adjoining μr. The older actual-field r=2 source
counterexample and prime-field Hasse-conditional integer-order theorem
remain separate, unchanged results. [pcc:unverified]

## Proof map

| Declaration | Exact role |
|:--|:--|
| TraceSquares | Explicit integer trace-square predicate |
| trace_powers | In a field, a+1=b and the five square cases imply a power 2,3,4,or6 is one |
| power_order_cases | Order divides one of those exponents; kernel-checked small divisor table |
| cast_trace_squares | Images of the integer equalities under the integer ring homomorphism |
| trace_count_order_cases | Reduce actual point-count divisibility modulo the prime r |
| actual_curve_classified_order_cases / actual_curve_classified_bound | Instantiate with actual q,N,t; retain classification premise |
| trace_count_coprime | Zero residue has order zero, incompatible with the proved positive cases |
| actual_prime_order_point_exists | Finite point equivalence plus Mathlib's additive Cauchy theorem |
| composite_numeric_order / composite_count_only_counterexample | q=289,N=324,r=81: coprime, divisibility and trace class hold, but modular order is 9 |
| trace_classification_audit_certificate | Fully quantified combined conditional theorem and numerical negative control |

For the control t=−34 and t²=4q. It is an arithmetic-only counterexample
to dropping primality from this count-based argument. No curve or point of
order 81 is constructed, so it is not claimed as a counterexample to a
qualified statement about actual cyclic subgroups. [pcc:unverified]

## Sources and reproducibility

Freeman–Scott–Teske, *A taxonomy of pairing-friendly elliptic curves* (2010),
Definition 2.1/Remark 2.2 on pp230–231 and the trace-square list on p237 were
read in the retained primary PDF, SHA-256
`2acd937be8567abfb26bb3861e3aaf34312270c36df9a86f91cf9676c3caf785`.
Their definitions require coprime characteristic and actual subgroup order;
the discussion uses prime-order subgroups. The trace classification is
quoted there from Waterhouse Theorem4.1. The first Waterhouse fetch timed out after45seconds with a partial file.
A resumed fetch subsequently completed. Original Theorem4.1 and the proof
beginning on pp536–537 (PDF pages17–18) were read and visually inspected.
The original includes parity, characteristic and congruence restrictions;
the Lean predicate keeps only the necessary square-list consequence.
The full original PDF SHA-256 is
`7be2bf9dde45454afa3dd9a0ea1ba05acbe952b6b573a0c37c2b1b8f0cccbb47`.
This source review does not constitute a formal proof of the classification. [pcc:unverified]

Mathlib revision `56a005855ea17e0b0958f73ec801ae140b0a1679` and Lean4.35.0
are pinned. Reviewed code includes Affine/Point.lean's
nonsingularPointEquiv and group construction, and Perm/Cycle/Type.lean's
exists_prime_addOrderOf_dvd_card'. Point finiteness is constructed through
the actual equivalence, not assumed. Local classical decidability supplies
the coordinate group-law instances. [pcc:unverified]

Evidence supersingular-trace-build-01 through06 preserves unsuccessful and
successful attempts. Attempts01/04/05 fail with instance errors; they are not
accepted proofs. Attempt06 exits0, with only propext/Classical.choice/
Quot.sound in the reported declarations. No custom axioms, sorry, native
decision exception or linter waiver. The whole-project gate passes331declarations with zero failures and zero
trusted-native exceptions. The exact source
binding and producer-local packet are recorded separately in
supersingular-trace-sources.json and supersingular-trace-verification.json.
Signed standing remains separate from producer-local checks. No native
experiment or paid compute. [pcc:unverified]

Source commit `c60c141e` precedes packet `PCC-20261110-9c7d48`. Its full
producer-local check passes at02:34:04UTC on host762f5fc120c9. Signed
standing remains unverified. See [actual binding](supersingular-trace-verification.json),
[inspected two-page PDF](reports/proof-20261110-supersingular-trace/report.pdf)
and its [editable report](reports/proof-20261110-supersingular-trace/report.md).
[pcc:unverified]
