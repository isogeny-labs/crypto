# H116 selected quotient-channel derivation

Lean source SHA-256:
`363e188374111e9a81c31b1b9267398d41c1f2230855fccf22197a90f451a51e`

Proof map SHA-256:
`8a2c2992ddc8c5f42b87a19835ca2cb6d3ea00087875c151d0d2e7058e7f0cb7`

## Audited identity

For the producer denominator

```text
F_u(w)=1-u*w+h1*w^2+h2*w^3+...
```

and any reciprocal `G_u` satisfying `F_u*G_u=1`, Lean proves

```text
[w^0]G_u = 1
[w^1]G_u = u
[w^1](F'_u*G_u) = 2*h1-u^2.
```

For target `u` and reference `u0` sharing the same jet, the relative producer
orientation is therefore

```text
[w^1](F'_(u0)*G_(u0)-F'_u*G_u) = u^2-u0^2.
```

The mutation theorem keeps unequal first jet coefficients and exposes the
uncancelled correction `2*(h1_0-h1)`. Thus the shared-jet premise is used
exactly, while every coefficient above `h1` is absent from the selected
channel. [pcc:unverified]

In the Rust producer, `u=delta^2*x` with `delta=trace-1`. Hence

```text
X1 = delta^4 * (x^2-x0^2)
IX1 = delta^24 * (x^2-x0^2)^6 / D^2,
D = 4*A^3+27*B^2.
```

Under the locked coordinate change, `X1` has weight four and `D` weight
twelve, so `IX1` is invariant. For a fixed curve and generator the factors
`delta^24/D^2` and `x0` are public constants. The replicated rule therefore
ranks six public x-coordinate square differences after a sixth-power residue
map; it is not extracting information from the higher formal jet.
[pcc:unverified]

## Audited quotient arithmetic

For every additive commutative monoid, point `P` and natural scalar `k`, Lean
proves

```text
k • P = 6 • ((k/6) • P) + (k%6) • P.
```

In an additive commutative group it proves

```text
(k • P) - (k%6) • P = 6 • ((k/6) • P),
```

and `k/6<k` whenever `k>=6`. These theorems establish termination and
correctness of quotient recursion conditional on an exact digit. They do not
make `IX1-argmin` exact; the five-curve OOS data explicitly refute exactness.

## Verification

```text
ELAN_HOME=/workspace/.elan-h113 /workspace/.elan-h113/bin/lake build
Build completed successfully (3164 jobs).

ELAN_HOME=/workspace/.elan-h113 /workspace/.elan-h113/bin/lake env lean \
  --run /workspace/h114/tools/lean/AuditAxioms.lean -- H116FiberTrace
audited 215 theorems in [H116FiberTrace]; 0 failing, 0 trusted (native_decide)
```

The seven new audited theorems are
`fiberDenominatorReciprocal_coeff_zero_one`,
`fiberLogDerivative_coeff_one`,
`fiberRelativeLogDerivative_coeff_one`,
`fiberRelativeLogDerivative_coeff_one_unshared`,
`quotientDigit_nsmul_identity`, `quotientDigit_sub_identity`, and
`quotientDigit_strict_decrease`.

## Algorithmic boundary

An exact digit rule would recover the scalar in logarithmically many quotient
steps, with polynomial public group work. The measured rule has only a fixed
prediction advantage and abstains on some targets. Such a heuristic can
reorder a six-way search tree, but without an error-correction theorem whose
surviving path entropy is `o(log r)`, it does not imply subexponential work in
the subgroup order. [pcc:unverified]
