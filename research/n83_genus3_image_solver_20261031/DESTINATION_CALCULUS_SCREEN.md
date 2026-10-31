# Destination-calculus applicability screen

Date: 2026-10-31  
Source revision screened: `3f77530aacb4283ac5b2d48eed013984a6f5f315`  
Decision: **the standard genus-three hyperelliptic index calculus is not an
applicable candidate solver for this transported image.**

This is a candidate rejection, not an impossibility result. It prevents a
full-Jacobian relation generator, or a solver that secretly returns to the
source elliptic curve, from being labelled a destination calculus.

## Exact problem being screened

The source subgroup has prime order

```text
r = 2417851639230796216685689, log2(r) = 81.00000000000092.
```

It is pulled into the Jacobian of

```text
C: v^2 + u^2 v = u^7 + u over GF(2^83)
```

by the same-field degree-three cover. The exact certificate proves that the
image generator has order `r` and that `phi_* phi^* = [3]`. It does not prove
that this cyclic image is the whole Jacobian; a genus-three Jacobian over
`GF(q)` has ambient scale `q^3`, whereas this image has scale `r ~= q`.

## Named candidate and quantitative gate

The screened candidate is the Gaudry–Thomé–Thériault–Diem double-large-prime
index calculus for a small-genus hyperelliptic Jacobian. Its familiar genus-3
heuristic cost is

```text
q^(2 - 2/g) = q^(4/3).
```

At `q = 2^83`, this is `2^110.666...` in the field-size exponent. Direct
generic rho in the actual order-`r` source subgroup is `sqrt(r) = 2^40.500...`.
The exponent ledger is therefore already about **70.17 bits against** this
transfer, before constants, individual logarithms, pullbacks, or linear
algebra storage are charged. The usual statement that genus-3 index calculus
beats rho compares against rho in a group of ambient scale `q^3`; it cannot be
reused for an order-`q` image inside that ambient group.

Gaudry's general abelian-variety construction makes the missing hypothesis
explicit. Its relation-probability analysis assumes the selected point
generates the whole abelian variety, estimates `#A ~= q^n`, and warns that the
analysis fails when the factor base lies in a strict abelian subvariety. The
transported order-`r` image is precisely a strict, much smaller target for the
ambient genus-three formulas.

## Reachability gate

The current implementation supplies:

- typed point pullback `phi^*: E -> Jac(C)`;
- generic Cantor arithmetic on the resulting Mumford classes;
- generic BSGS and rho that stay in the cyclic image; and
- typed push/pull composition for certified source points and exceptional
  fibers.

It does **not** supply a norm/pushforward for an arbitrary Jacobian divisor
class at `n=83`, an image-membership projector, or an image-compatible
factor-base decomposition. Consequently, an ambient factor-base relation
cannot currently be shown to land in the transported image, be projected to
it, or yield a scalar modulo `r`. Relation collection alone would not close
the DLP pipeline.

An adapter that pulls the target back, calls an elliptic-curve solver, and
re-encodes the answer would be a transported-data control. It is not a
destination solver and is prohibited by this screen.

## Admission criteria for the next candidate

A future non-generic adapter may be implemented only after its design answers
all of the following in the actual order-`r` image, not merely in `Jac(C)`:

1. **Domain:** define the factor base or search space and prove how its
   generated relations reach the image.
2. **Distribution:** state and test the relation probability for samples from
   the cyclic image; an ambient-uniform heuristic is insufficient.
3. **Projection:** provide an explicit image-membership/projection or a
   relation construction that is closed in the image.
4. **Closure:** include relation collection, rank/linear algebra, individual
   logarithm or descent, scalar recovery modulo `r`, destination replay, and
   independent source replay.
5. **Cost:** express the exponent in both `q` and `r`, charge the factor base,
   memory, linear algebra, transfer, and individual-log stages, and predict a
   finite crossover against the strongest source control.
6. **Controls:** retain source rho, destination generic rho or BSGS, a
   transported-data negative control, and A/A calibration.

Until those conditions are met, `jacobian_image.bsgs` and
`jacobian_image.rho3` remain the only legitimate destination adapters for this
image, and both are generic controls.

## Sources and reproducible evidence

- Pierrick Gaudry, *Index calculus for abelian varieties and the elliptic
  curve discrete logarithm problem*, IACR ePrint
  [2004/073](https://eprint.iacr.org/2004/073), especially sections 3.2–3.5
  and Remark 2. The retrieved PostScript had SHA-256
  `4a8a33f429715f065f2988dc1f03f721189635657f9622128171d02e61a82fbd`.
- P. Gaudry, E. Thomé, N. Thériault, C. Diem, *A double large prime
  variation for small genus hyperelliptic index calculus*, Math. Comp. 76
  (2007), DOI
  [10.1090/S0025-5718-06-01900-4](https://doi.org/10.1090/S0025-5718-06-01900-4).
  The publisher metadata states that its genus-three comparison is for
  hyperelliptic curves, not a small transported subgroup.
- The repository's derivation and implementation history:
  [`RESEARCH_HYPERELLIPTIC_IC_RHO.md`](../notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md),
  [`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`](../notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md),
  and [`JACOBIAN_TRANSFERS.md`](../../docs/curves/JACOBIAN_TRANSFERS.md).
- Exact transfer/image certificate:
  [`runs/exact-n83-post-panel-20261031T044500Z/certificate.json`](runs/exact-n83-post-panel-20261031T044500Z/certificate.json),
  SHA-256
  `911fdab8e1d2978fb995de90b743dd1a3e97719e960515f5668e53dcdb51e819`.

The AMS PDF endpoint returned HTTP 403 from this execution environment. The
failed fetch was retained in the session log; no theorem text was inferred
from the failed response.
