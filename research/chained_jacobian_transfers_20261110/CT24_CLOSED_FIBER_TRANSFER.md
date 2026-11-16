# CT24: explicit closed-divisor transfer into two degree-seven target Jacobians

2026-11-11, `[pcc:unverified]`. Stacked on CT23 #3927, CT22 #3926, CT20–CT21 and root crypto #3877. This work evaluates a **genuine geometric elliptic-to-Jacobian map** at each of the 109 source rational points, rather than only a product-coordinate proxy. It does **not** construct the full source-Jacobian-to-target-Jacobian (7,7) isogeny, balanced Cantor reduction or a faster DLP algorithm.

## Prior work / source custody

Reviewed original PR #3877, CT20 plan and HLP chart inventory, CT22 actual E[4] oracle, CT23 rational-product regression, cryptanalysis #1483 and crypto-autoresearcher transfer lead inventory. Relevant established gluing work: Howe–Leprévost–Poonen (2000) §3.2 (https://math.mit.edu/~poonen/papers/large.pdf). The pullback/norm identity is standard functoriality of degree-2 curve morphisms, not a new theorem. The crypto-autoresearcher lead atlas sources are vault-locked (UTF-8 decode failed); research-KB/conductor unavailable. No global novelty claim.

## Explicit geometrical construction

Fix `p=101` and the source elliptic curve `E0:y²=x³+x+46`, with generator `G=(2,37)` of order 109. Reuse the existing **degree-seven isogeny** `phi_B:E0 -> E1:y²=x³+26x+58` whose monic 3-kernel x-polynomial is `h=x³+35x²+100x+72` (CT19/CT23).

There are two accepted genus-two target models:

| chart | C: `y²=t⁶+A t⁴+B t²+b` | reciprocal quotient short coefficients | isomorphism scale |
| --- | --- | --- | --- |
| B+A | (A,B,b)=(92,44,33) | (34,9) | s=7 |
| B+B | (A,B,b)=(26,42,68) | (69,5) | s=21 |

On each genus-two curve, define the degree-two morphism

```
pi(t,y) = (U,V) = (b/t^2, b*y/t^3)
D0 = (0,+sqrt(b)) + (0,-sqrt(b)) = pi^*(O).
```

Its elliptic codomain is the **long** cubic `V²=U³+B U²+b A U+b²`, which is isomorphic over F101 to E1 by `(X,Y)=(s²(U+B/3),s³ V)`. Note the `B/3` shift uses the **quadratic** genus-two coefficient, not the quartic one; reversing these produced an explicit failing development assertion and was corrected before committing.

For a finite quotient point `Q=(U,V)` with `U != 0`, a rational closed degree-two fiber has the exact support representation

```
u(t)=t²-b/U,       v(t)=(V/U)t,
D(Q) = [(u,v)] - D0.
```

Indeed `v² - (t⁶ + A t⁴ + B t² + b)` is divisible by `u`. Its two points are rational if `b/U` is a square and form a closed F101² point otherwise. They are automatically evaluated over F101 as a single rational divisor even for nonsplit fibers.

**Exceptional points must not be discarded:** the quotient's two finite points `Q=(0,±b)` pull back to the two **ramified points at infinity**, each with multiplicity two. Therefore `D((0,±b))=2 infinity_± - D0`. The elliptic identity maps to the zero Jacobian class. Crucially, the reference divisor `D0` lies at **t=0**, not at genus-two infinity; using the latter would be a false normalization.

The map
```
T(P) = [ pi^*(iota_s^{-1}(phi_B(P)) - O) ] in Jac(C)(F101)
```
is a defined homomorphism by Picard functoriality. Since `pi_*pi^*=[2]` and `phi_B` has 7-power degree coprime to 109, its restriction to the order-109 source is injective. Thus it preserves the logarithm. No balanced-divisor group law is used to evaluate `T(P)`; the output is an honest **degree-zero divisor difference**, not a canonical Mumford class or CT20 `Phi`.

## Bounded producer-run tests

Run from repo root with Python optimization disabled:

```sh
python3 -S research/chained_jacobian_transfers_20261110/ct24_closed_fiber_transport.py
python3 -S research/chained_jacobian_transfers_20261110/ct24_fiber_replay.py
python3 -m py_compile research/chained_jacobian_transfers_20261110/ct24_closed_fiber_transport.py research/chained_jacobian_transfers_20261110/ct24_fiber_replay.py
```

For **each chart**, the first oracle tests all 109 source scalars, Kohel evaluation, short/long chart conversion and its inverse, the explicit fiber support polynomial, the complete `pi_*D=[2]Q` norm reconstruction and inverse-2 recovery, inverse divisor signs, distinct quotient-normalized images, and end-to-end target-elliptic BSGS scalar recovery from the **divisor data**. The frozen observations are **58 split + 48 nonsplit + 2 ramified infinity = 108 nonidentity** fibers, with exceptional source logs B+A `(4,-),(105,+)` and B+B `(18,-),(91,+)`. Across 109 cold target BSGS runs it counts 1,685 elliptic additions per chart, **not** the whole transport cost.

The second, separately written F101² polynomial/point oracle independently enumerates all 109 *target* elliptic points (no Kohel import), checks both geometric support points including nonsplit roots `t=zeta*w` with `zeta²=2`, verifies the target sextic and reciprocal quotient image **pointwise**, and separately identifies the two infinity cases. Both local producer executions passed. `python -O` disables the assertion gates and is forbidden. Intentionally wrong chart scale is rejected.

A local producer test remains `[pcc:unverified]`, even with two independently coded algorithms. Exact-head CI and external signed PCC acceptance are separate mandatory gates.

## Limits, costs and next steps

This advance makes the **E109 -> Jac(C1)** leg explicit, including irreducible closed fibers and infinity ramification. The full `J0 -> J1` CT20 map is still missing: implement `N0` for arbitrary balanced genus-two Mumford classes and `P1` as a **sum** of both elliptic quotient pullbacks, including its full even-sextic infinity corrections; then evaluate `Phi=P1 alpha (N0/2)` and the dual on all 12,317 classes. Compare against the direct `phi_B` shortcut, optimized rho and independent index-calculus baselines only after complete same-host cost accounting and approved compute admission.

Do not merge while ancestor #3877 frozen PCC-ledger/archive, CI and independent review gates remain blocked. No EKS Spot or paid compute was used and no DLP speedup is claimed.
