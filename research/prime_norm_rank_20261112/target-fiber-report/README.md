# A lossless target row with an explicit third-order parameter factor

2026-11-14. Source `d265c35a0efc1efb609a396e451fd543ab838c21`.
All claims unsigned [pcc:unverified].

The actually computed norm/membership remainder has a uniform third-order
factor after evaluation at the scaled target. The identity holds over
commutative rings, including a polynomial coefficient ring in the parameter.
Its specialization is exactly the existing field model. Replacing the
constant remainder equation by the normalized target row preserves the full
five-equation solution set for nonzero parameter. This is a concrete equation
transformation for subsequent closure analysis [pcc:unverified].

Central claims: factor `PCC-20261114-76f2d5`, polynomial divisibility
`PCC-20261114-af9c9e`, specialization `PCC-20261114-ca75bf`, lossless rows
`PCC-20261114-2ceac7`. [Lean source](../lean/NormTargetFiber.lean),
[contract](../TARGET_FIBER_PROTOCOL.md), [proof map](../target-fiber-evidence/claim-pins.json),
[editable diagram](target-row.dot), [SVG](target-row.svg), [PDF](report.pdf).

![The actual quotient and remainder produce the parameter factor; specialization and nonzero-parameter row equivalence are separate checked steps. Dashed complexity arrows remain open. Evidence unsigned.](target-row.pdf){width=90%}

## Construction and proof

Start with `f=X^3+a*X+b`, a public target `z^2=f(r)`,
`D=X^3+a2*X^2+a1*X`, and `B=c0+c1*X`. Complete the actual constant:
`A=D+z*B(r)-D(r)`. The actually computed support is
`U=(A^2-f*B^2) div (X-r)`. Exact norm division and differentiation give

```
U(r) = B(r)*J(r)
J(r) = 2*z*A'(r)-f'(r)*B(r)-2*z^2*B'(r).
```

Differentiate the polynomial norm identity first; substitute target values
afterwards. No derivative identity `A'=z*B'` is assumed from one point's
incidence. In this cubic/linear chart, both `B(r)` and `J(r)` are affine
expressions in the four retained coefficients. The underlying norm derivative
is established prior work; this component builds the computed quotient and
parameter-vanishing consequences on it [pcc:unverified].

Now use the explicit family with curve `f_e=X^3+e^4*a*X+e^6*b`, target
`(q_e,z_e)=(e^2*r,e^3*z)`, and the same actual target-completion operation.
The support evaluated at that target factors as

```
U_e(q_e) = e^3*(c0+c1*e^2*r)*J_e
J_e = 2*z*a1+4*e^2*z*r*a2+6*e^4*z*r^2
      -e*(3*r^2+a)*(c0+c1*e^2*r)-2*e^3*z^2*c1.
```

For depth n put `d=2^(n+1)`, `P_e=M_e,n-terminal*e^d`, and compute
`Q_e=P_e div U_e`, `R_e=P_e mod U_e`. The master recurrence gives
`P_e(q_e)=e^d*K_n`, where `K_n=master(2,n)(r)-terminal` is the original
public target evaluation. Evaluating actual division yields, for d>=3,

```
R_e(q_e) = e^3*T_e
T_e = e^(d-3)*K_n-(c0+c1*e^2*r)*J_e*Q_e(q_e).
```

The quotient is computed by monic division, with no free oracle or successful
coefficient witness. Norm/support degree proofs work over nontrivial rings;
actual monic remainder computation commutes with coefficient-ring maps.
Consequently this factorization is valid in `K[E]`, with E a polynomial
parameter and X the distinct geometric variable. Mapping `E` to a field
element recovers exactly the prior `NormWeightedLeading.scaledMembership`.
No agreement-on-finite-field-values shortcut is used [pcc:unverified].

## Exact equation replacement and scope

The actual remainder has degree below5. For any such remainder, vanishing
of its target evaluation and its four nonconstant coefficients is equivalent
to vanishing of the whole polynomial. This row replacement is valid over
the coefficient ring. Over a field, e!=0 makes e^3 nonzero, so the full
original five equations are equivalent to `T_e=0` together with those four
nonconstant coefficient equations [pcc:unverified].

At e=1 this is an equivalent equation system for the original symbolic
input. At e=0 the polynomial divisibility remains valid, but removal of the
factor is not an asserted solution-set equivalence. The statement is uniform
divisibility by e^3, not exact order3: characteristic2, z=0 or other vanishing
factors can increase the order. The support/row specialization uses this
degree5 chart; no growing-arity solving theorem is asserted [pcc:unverified].

The expected weighted degree29 for a normalized depth4 target row remains
a graded-polynomial proof obligation. A parameter factor, input degree or
first degree fall does not establish a low weighted last-fall/normal-form
bound. Nor is a fast solver supplied by introducing a name for the actual
quotient. The original W-jet family still needs its own faithful public map
[pcc:unverified].

## Validation, retained errors and next test

The module has26 statements. Final build007 passes; the whole-study gate
reports353 statements,0 additional trusted uses, exit0, and import-inclusive
fresh replay exits0. Standard foundations are `propext`, `Classical.choice`,
`Quot.sound`. Validation claim `PCC-20261114-7ad52b` binds the
[receipt](../target-fiber-evidence/validation-001.json), exact source/artifact
hashes and all retained exits. All28 new source/data claims remain unsigned;
default-PATH verdicts are `not_covered` (exit2) [pcc:unverified].

Dependency pins remain Lean4.35.0, CryptoLean revision
`148f5dc389b341401e06d76abe7783c4ccf42f2e`, and Mathlib revision
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`. The adapter uses Rust1.99.1.
No earlier mathematical source, dependency, checker/hash or gate changed
[pcc:unverified].

Literal certificate `IDC1h085fdef17210ed0e`, claim `PCC-20261114-365b89`,
seals only the unconditional evaluated derivative-factor step

```
2*Z*B*D-F*B^2-2*Z^2*B*C = B*(2*Z*D-F*B-2*Z^2*C).
```

Its syntactic degree is4, with32 statement-derived and16 extra replay points.
It does not encode the curve premise, parameter divisibility, specialization,
row equivalence or execution. The canonical adapter's22 tests, format,
release build and issuance pass; strict clippy records four inherited style
failures in unchanged canonical SHA256. Failed proof drafts002/004/006
retain export-body equality, mapped numeral and wrong factor-side errors.
Drafts001/003/005 and final007 pass. No check was suppressed
[pcc:unverified].

No native fixture/closure experiment, unknown-root search, F4/msolve,
paid/cloud/attack run or timing comparison was attempted. The earlier fixed
fixture flow remains1 planned,0 attempted,0 completed,0 excluded,1 admission-
blocked by unchanged checkpoint upload transport. Pure Lean and literal
certificate construction are separate. The old sparse benchrec1680/162
diagnostic is preserved; this change touches no benchmark records or
progress/gate generator [pcc:unverified].

Sources checked (KB): unavailable as already recorded; no unchanged retry.
The [bounded fallback](../target-fiber-evidence/prior-search-receipt.json)
records five queries and exact URLs. Read actual NormWronskian derivative,
public-jet rigidity and functional/polarization sources; earlier theory and
rationality reviews preserve scheme/field-selection limits. Their Kummer-
coset rationality lemma is not transferred to the Dickson input. Reuse the
primary weighted-GB and ML full-closure reviews; neither input degree nor
first-fall behavior supplies a solver bound. No novelty or exhaustive-
absence claim follows from this token-matching search [pcc:unverified].

Next, prove the graded interpretation of the normalized target row and
construct a bounded full-closure/normal-form certificate. Any computational
test needs its own unchanged acceptance criteria and checkpoint admission;
one successful cell would not prove growing-family scaling. The broad goal
still requires an efficient public-input finder and complete expected bit
cost, including field extraction, useful relations and verification
[pcc:unverified].

Global performance/curve/attack-route graphs remain unchanged: there is no
new measured speed or group-transport edge. Earlier source/evidence/claims
and canonical ledger bytes remain intact. This study's new diagram separates
the exact algebraic transformation from the open complexity arrow
[pcc:unverified].

## Exact proof map

Exact hypotheses and declaration hashes are in claim-pins.json. All entries
are unsigned [pcc:unverified].

| NormTargetFiber declaration | Unsigned claim |
|---|---|
| `normDerivativeRing` | `PCC-20261114-075987` |
| `linearQuotientValue` | `PCC-20261114-9bf216` |
| `normQuotientTargetFactor` | `PCC-20261114-25d287` |
| `supportTargetValue` | `PCC-20261114-20bbfa` |
| `remainderTargetIdentity` | `PCC-20261114-e5137c` |
| `scaledCurveEquation` | `PCC-20261114-e0d47c` |
| `scaledSupportTargetValue` | `PCC-20261114-1cd56e` |
| `masterAtScaledTarget` | `PCC-20261114-8d052a` |
| `membershipTargetFactor` | `PCC-20261114-76f2d5` |
| `membershipTargetDivisibility` | `PCC-20261114-7a2d78` |
| `polynomialParameterDivisibility` | `PCC-20261114-af9c9e` |
| `normWindowRing` | `PCC-20261114-cef811` |
| `supportWindowRing` | `PCC-20261114-5fb4da` |
| `scaledSupportWindowRing` | `PCC-20261114-4a71b9` |
| `targetRowsIff` | `PCC-20261114-5a50b9` |
| `membershipTargetRowsIff` | `PCC-20261114-18d3c7` |
| `mapActualA` | `PCC-20261114-d9cee2` |
| `mapSupport` | `PCC-20261114-ac74c3` |
| `mapMaster` | `PCC-20261114-3b0238` |
| `mapMembership` | `PCC-20261114-58ed53` |
| `scaledSupportField` | `PCC-20261114-76fbe7` |
| `masterField` | `PCC-20261114-f20511` |
| `membershipField` | `PCC-20261114-6be927` |
| `parameterSpecialization` | `PCC-20261114-ca75bf` |
| `normalizedTargetEquation` | `PCC-20261114-6f0e15` |
| `membershipNormalizedRowsIff` | `PCC-20261114-2ceac7` |
