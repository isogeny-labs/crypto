# H116 truncated Frobenius-fiber traces

H116 tests a component needed by H115: whether the fixed low-order moments of
the degree-`r` fiber of `phi = V - 1` can be recovered from a short Laurent jet
of the `x`-coordinate map, without constructing or enumerating that fiber.

The proposed identities are frozen as hypotheses until proved and independently
checked. In particular, an efficient moment evaluator is not by itself an ECDLP
algorithm. The separate digit-predicate and decoder obligations remain open.
[pcc:unverified]

## Requested scope and evidence plan

| Requirement | Frozen evidence |
| --- | --- |
| A different non-generic lead | Relative fiber traces/residues of the special endomorphism `V-1`, not a generic walk or relation collection |
| Rigorous algorithm | Native candidate with exact field-operation counters and a direct degree-`r` enumeration reference |
| Controls | identical A/A reference, planted polynomial positive control, characteristic-bound rejection control, coordinate isomorphisms and shuffled target order |
| Scaling | five deterministically selected public toy curves with `p > 19`, all nonidentity targets and three generator labels |
| Independent check | separate cryptanalysis verifier source and tamper tests before scoring |
| Formal proof | Lean identity layer for logarithmic derivatives/Newton sums and residue extraction, with the Frobenius-jet and cost premises kept explicit if their library instances are unavailable |
| Subexponential claim | admissible only if the moment construction, one fixed exact digit predicate, the complete decoder and every setup/recovery cost are all bounded by `r^o(1)` |

## Frozen algebraic target

Let a separable odd-degree isogeny have coordinate map

```text
phi(x,y) = (R(x), y R'(x) / a),   R = N/D,
F_u = N - uD.
```

For a non-2-torsion target `Q=(u,v)`, H116 tests the following coefficient
identities, with fiber points counted with their algebraic multiplicities:

```text
X_j(Q)-X_j(Q0)
  = [x^(-j-1)] d/dx log((R-u)/(R-u0)),

Y_j(Q)
  = a v [x^(-1)] x^(j-1)/(R-u).
```

For `phi=V-1` on `E/F_p`, the formal endomorphism identity

```text
V - 1 = [t-1] - pi,    pi(z)=z^p
```

predicts that the required fixed-order jet is computable from a truncated
formal-group calculation. For the first implementation, the Bostan et al.
normalized-isogeny recurrence is used only when its denominators are units;
the frozen cohort requires `p > 2J+3` with `J=8`. The exact general small-
characteristic formal-group construction is an open successor, not silently
substituted into this run. [pcc:unverified]

The implemented normalization and exact truncated formulas are recorded in
`ALGORITHM.md`. In particular, the recurrence target x-coordinate is
`U=(t-1)^2u`, not raw `u`; this scaling is required to make the isogeny
normalized before applying the Bostan recurrence.

The Lean layer now also defines the exact formal series
`F_U(w)=1-Uw+h₁w²+h₂w³+...` and proves its coefficients, derivative, unit-
constant reciprocal recurrence and finite derivative-convolution used by the
native eight-coefficient loop. The `w=1/x` isogeny substitution and the
Bostan/`V-1` jet constructor remain explicit open obligations.
[pcc:unverified]

For prime `p>19`, the Lean theorem `bostanDenominatorsNonzero` proves that all
eleven denominators used to construct `h₁,...,h₇` are nonzero in
`ZMod p`. This discharges the characteristic-range side condition for the
frozen prime-field cohort; it does not prove recurrence (16) itself.
[pcc:unverified]

The exact finite recurrence is now encoded as `BostanJetRecurrence`. Lean
proves that its seven output coefficients are unique and that any two solutions
induce identical `F_U` coefficients through degree eight. The remaining bridge
is no longer ambiguity about the native formula: it is the geometric statement
that the actual normalized isogeny Laurent jet satisfies this recurrence.
[pcc:unverified]

The coefficient-extraction part of that bridge is now formalized as well.
`NormalizedIsogenySecondOrderEquation` is equation (15) of the cited Bostan et
al. construction after `R(x)=xU(1/x)`. Lean proves that this PowerSeries
equation plus the two initial coefficients implies the exact seven-term native
recurrence, with every division discharged for prime `p>19`. What remains is
to obtain this formal-series equation from the actual normalized isogeny map.
The preceding paper step is now formalized separately:
`normalizedMapEquation_implies_secondOrder` differentiates the normalized
rational-map curve equation and cancels its explicitly nonzero derivative to
obtain equation (15). The transformed equation (14) is also formalized as
`NormalizedIsogenyFirstOrderEquation`, and Lean proves it implies the existing
second-order PowerSeries equation. Its cancelled factor is automatically
nonzero because its constant coefficient is one. The same first-order identity
forces the two initial coefficients `h₁=(A-Atilde)/5` and
`h₂=(B-Btilde)/7`. Thus, in prime characteristic `p>19`, it now implies the
complete frozen recurrence without extra coefficient hypotheses. What remains
is to construct the actual normalized isogeny at-infinity series and prove that
it satisfies this first-order PowerSeries equation; no theorem here constructs
that map. [pcc:unverified]

The open geometric bridge now has an explicit finite proof-carrying interface.
`NormalizedRationalMapPolynomialCertificate` accepts reversed numerator and
denominator polynomials `N,D`, checks the quotient relation and the single
cleared-denominator curve identity, and Lean proves that certificate implies
the first-order equation and hence the complete recurrence for prime `p>19`.
This moves the remaining obligation to producing and independently checking
the actual rational-map polynomials; it does not treat a supplied polynomial
pair as an isogeny without that identity. [pcc:unverified]

The separate [map-certificate control](MAP_CERTIFICATE_EXPERIMENT.md) exercised
this interface on the excluded H115 `p=31`, `r=37` fixture. Rust reconstructed
the full Vélu map, and an independent Python verifier reproduced the cleared
identity, the coefficient prefix `[1,0,20,0,13,7,2,4,8]` and rejection of a
one-coefficient mutation. This is proof/control evidence only: the degree-`r`
construction is not charged as the fixed-length candidate and no gated H116
target was read. [pcc:unverified]

The follow-on [fixed low-jet experiment](JET_CERTIFICATE_EXPERIMENT.md) removes
the full polynomial degree from the **verification** interface. Lean proves
that `D*U=N mod X^11` plus the cleared curve identity modulo `X^10` forces the
nine coefficients needed by the complete recurrence for prime `p>19`. The
excluded `p=31`, `r=37` replay used 22 numerator/denominator coefficient slots
instead of all 75, and passed in-window, derivative-guard and out-of-window
mutation controls. This does not remove the degree-`r` cost of constructing the
correct low jet or certify its intended kernel semantics; those remain the
active algorithmic obligations. [pcc:unverified]

At the formal-series level, the target/reference denominators are now proved
to share every coefficient except `-Uw`: their difference is exactly
`(U0-U)w`. This is conditional on one supplied normalized jet `h`; identifying
that jet with the actual isogeny is still open. [pcc:unverified]

Target generation and moment production are process-separated. The target
preparer applies the frozen per-curve seeds and emits (1) coordinates plus an
opaque SHA-256 digest and (2) a separate scalar-label map. The moment producer
accepts only the first artifact, checks that it is a complete nonidentity
subgroup census, and serializes only the digest. The label map is reserved for
the independent scorer after identity verification.

## Cost boundary

The candidate target for fixed `J` is `O(M(J) log p)` base-field operations,
including construction of the scalar `[t-1]` jet. The direct control performs
`Theta(rJ)` extension-field work after constructing the kernel generator. The
run records cold setup, genuinely reusable setup, per-target work, recovery and
verification separately. No stage ratio will be reported as an end-to-end
ECDLP ratio. [pcc:unverified]

Files are added in this order: literature and protocol freeze, producer and
Lean source, independent verifier binding, checkpointed EKS execution, raw
export, report/diagram/PDF, and only then any claim record.

The independent verifier source is now bound in
`INDEPENDENT_VERIFIER_GATE.md`. Its target-admission run has not started
because the mandatory cryptanalysis checkpoint upload credential is absent;
this blocks the producer run rather than weakening the gate. [pcc:unverified]
