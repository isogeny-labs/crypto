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
