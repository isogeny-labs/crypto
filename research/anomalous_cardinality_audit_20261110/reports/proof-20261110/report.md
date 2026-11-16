---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# The actual 160-bit curve has exactly p points

Lean proves the entire corrected curve has p rational points, using its
certified order-p point, a closed quadratic-nonresidue certificate and an
elementary point-count bound. No Hasse or CM point-count theorem is assumed.
The curve and prime are the same actual Mathlib objects as the parent point
and concrete Smart arithmetic certificates. [pcc:unverified]

```text
p = 730750818665451459112596905638433048232067471723
a = 425706413842211054102700238164133538302169176474
b = 203362936548826936673264444982866339953265530166
E: y² = x³ + ax + b over ZMod p
```

![Elementary actual-point counting and explicit negative control. [pcc:unverified]](proof.pdf){width=98%}

The kernel checks b^(p/2)=−1 and b≠0. Euler's criterion excludes x=0.
At most two y-values occur at every other x, so the actual point count N
satisfies N<2p. The certified point gives p divides N; finiteness and infinity
give N>0. Hence N=p. [pcc:unverified]

\newpage

# Exact proof obligations and negative controls

| Module | Checked obligation |
|:--|:--|
| `ShortPointCount` | Actual nonsingular-point equation; injective encoding into nonmissing x-values × Bool; strict count bound; arithmetic multiple lemma |
| `MissingFiber` | Closed Euler-power check, nonzero b and no point at x=0 |
| `CardinalityControls` | Actual F₅ curve has ten points, an order-five point and no missing x-fibre; direct universal negation |
| `AnomalousCardinality` | Finite positive actual point count, Lagrange composition, exact N=p and combined certificate |

Two actual controls prevent an unjustified identification of subgroup and
curve counts. Over F₃, y²=x³−x+1 has seven points, so a universal strict
N<2q bound fails. Over F₅, y²=x³+3x has ten points and an actual order-five
point; all x-fibres are nonempty. Lean directly refutes the inference that
an order-characteristic point alone forces total curve order equal to the
characteristic. These controls do not contradict the checked missing-fibre
argument. [pcc:unverified]

The generic encoding chooses one square root per nonempty fibre and records
whether a point uses that root or its negative. It is a noncomputable
mathematical injection, not a runtime or enumeration claim. Actual point
cardinality is connected through Mathlib's nonsingular-point equivalence;
no abstract lookup table or assumed curve cardinality is substituted.
[pcc:unverified]

Main binding:
`AnomalousCardinalityAudit.anomalous_curve_cardinality_certificate`.
Full gate: 17 declarations, zero failing and zero trusted-native exceptions;
generated equations are included, private proofs audited transitively.
Lean 4.35.0 / Mathlib 56a00585, only propext, Classical.choice and Quot.sound.
No holes, custom axioms, native_decide exception, linter waiver or added
elaboration setting. Failed extraction/choice/tuple proofs and the rejected
modern-header setup remain archived; legacy import advisories are retained.
[pcc:unverified]

Source commit `7783f692` precedes packet `PCC-20261110-541e6f`.
Full local verification passes at 10:39:11Z; signed standing is unverified.
Source pins, actual verdict and archive roundtrip checks are in
`../../sources.json` and `../../verification.json`. This closes the former
whole-curve-cardinality obligation, while the generic Smart p-adic transfer,
complexity/native compiler refinement and matched SafeCurves benchmark remain
open. The native execution packet remains separately `not_covered`.
NIST's screened recommendation is not refuted. [pcc:unverified]

Sources checked: corrected anomalous-curve construction and author erratum;
Smart HPL-97-128; Mathlib affine point, square-root, finite cardinality,
Lagrange and Euler-criterion declarations. Prior paper hashes/read scopes
remain in the parent plan. KB/Conductor unavailable; bounded primary,
repository and Mathlib fallback. No novelty, native experiment, sampling,
timing, security-cost or paid-compute claim. [pcc:unverified]
