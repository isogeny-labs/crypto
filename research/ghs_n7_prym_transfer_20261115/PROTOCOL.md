# Frozen protocol: symmetric W1 Prym transfer

15 November 2026 UTC. Status: **proposed construction and verification
protocol**. This study asks whether a symmetric slice of the *published*
Menezes--Teske degree-seven W1 family admits an explicit subgroup-preserving
transfer to a genus-seven hyperelliptic Jacobian through a dihedral closure.
It does not presume a new eligible family or a faster ECDLP attack.

## Literature gate and claim boundary

Read [Menezes--Teske §5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf),
[Hess Theorems 12, 14--16 and §§8.3--8.4](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf),
[Roth--Raviv--Tamo Lemma 25](https://arxiv.org/pdf/1705.04560), and
[Dalaljan/Prym treatment](https://arxiv.org/pdf/2003.07774) before any
novelty claim. The last source explicitly works in characteristic zero;
the wild characteristic-two specialization must be checked separately.
The earlier W1 membership study is
[`../ghs_n7_membership_20261114/`](../ghs_n7_membership_20261114/).

Separate labels: new exact equation, new eligible family, new transfer
method, and demonstrated full attack advantage. Only the third is a
candidate here. A successful point map and a genus-seven model do not
establish the fourth without a full same-subgroup GHS-versus-rho cost.

## Parameterized source and candidate target

Let `k=F_q`, `K=F_(q^7)`, `q=2^ell`, `sigma(z)=z^q`, and let
`p=T^3+T+1` or `T^3+T^2+1`. Choose `gamma∈ker p(sigma)\{0}`,
`c,d∈k*`, and `a∈k` with `Tr_(k/F2)(a)=1`. Set

    beta = gamma*(c*gamma+d),    b=beta^2,
    E: y^2+x*y=x^3+a*x^2+b.

The first factor has exact Frobenius order `p`; the second has exact
order `(T+1)p`, so this is a symmetric slice of published W1. The
standard Hess cover `C/k` has genus 14, subject to verifying its exact
construction and the map on the selected source subgroup.

Over `K`, write the Artin--Schreier source as

    v^2+v = gamma/x + a + (c*gamma+d)*x.

Define `t=1/x+c*x`, `T=t/c`, and the rank-three AS field
`H=K(t,z_0,z_1,z_2)` with
`z_i^2+z_i=sigma^i(gamma)*t` for a Frobenius-stable basis. Define
`D=H(x)` by `x+1/(c*x)=T` and `C=D(v)` by `v^2+v=d*x+a`.
Add `u^2+u=d*T` to obtain `Z=H(x,v,u)`. Candidate involutions are
`A(v)=v+1`, `B(u)=u+1`, and
`S(x)=1/(c*x), S(u)=u, S(v)=v+u`; verify every defining equation and
`B*S*B=A*S`. The proposed genus-seven target is `X=Z/<S>`.
With `z=x/T`, `W=v+u*z`, and `e=d/sqrt(c)`, AS reduction predicts

    W'^2+W' = a + e/(u+1)

over `H(u)`. Verify that `H(u)` is rational, that the eight preimages of
`u=1` are simple poles, and that `g(X)=7`. Verify the Frobenius descent
of `X`, `C`, and the correspondence to `k`; an equation over `K` alone is
not sufficient.

Candidate Jacobian correspondence:

    M = Norm_(Z/X) o Con_(Z/C) : J_C(k) -> J_X(k).

On the anti-invariant Prym of `C→D`, verify
`M^t M=[2]` (or the weaker odd-torsion version). Independently verify
that Hess's source map `phi:E(K)→J_C(k)` is nonzero on each selected
odd-order subgroup. The subgroup map is `M o phi`. A field or divisor
identity without a checked point image is an incomplete map receipt.

## Fixed finite calibration and candidate selection

First instance: `K=F2[z]/(z^7+z+1)`, `gamma=z`, `c=d=a=1`, `b=0x14`.
The source equation is `y^2+xy=x^3+x^2+0x14`.
The proposed target model over `F2` is

    X: W^2+W = 1 + 1/(s^8+s^4+s+1).

For this exact instance, set `L(s)=s^8+s^4+s` and
`T(s)=L(s)^2+L(s)=s^16+s^4+s^2+s`. An explicit candidate closure
over `F2` is

    Z: x^2+T(s)*x+1=0,  v^2+v=x+1.

Its proposed `F2` automorphisms are `A:v↦v+1`, `B:s↦s+1`, and
`S:x↦1/x, v↦v+L(s)`; other coordinates are fixed. In particular,
`L(s+1)=L(s)+1`, `T(s+1)=T(s)`, and `B*S*B=A*S` must be checked.
The fixed field of `B` is the genus-14 Hess cover; the fixed field of
`S` is `X`. Verify source embedding using
`A_gamma(s)=gamma*s+(gamma+gamma^2)*s^2+
(gamma+gamma^2+gamma^4)*s^4+(gamma+gamma^4)*s^8`, for which
`A_gamma^2+A_gamma=gamma*T` and `A_gamma(s+1)=A_gamma(s)`.

Before timing, a native Rust implementation must verify the field,
curve order, Hess membership, every D4 field identity, genus by the
separable denominator, the target zeta data through degree seven,
and nonzero images of points in the source order-five and order-thirteen
subgroups. Preliminary Sage calculations of `#E(K)=130` and
`#J_X(F2)=130` are hypotheses to reproduce, not accepted run evidence.

Next finite workload: `q=16`, `K=F2[w]/(w^28+w+1)` (mask
`0x10000003`), `c=d=1`. Construct `k=F16⊂K` explicitly. Choose
lexicographically first `a∈k` of absolute trace one. Enumerate
`gamma` in ascending polynomial-basis integer encoding among nonzero
members of each `ker p(sigma)`. For each resulting `E`, compute its
exact group order with a native method and independently verify it;
retain the first curve in each cubic arm with `#E=2r`, `r` prime and
`r≥2^26`, or record exhaustion. This selection is fixed before any
attack timings. Verify the corresponding target `X` and its subgroup
image before pricing algorithms.

## Full same-subgroup cost comparison

For each admitted q=16 candidate, select the first affine source point
whose cofactor-cleared image has exact order `r`; select a target scalar
by a fixed SHA-256 seed committed before the run. Run a complete target
recovery through the genus-seven map, relation collection, linear algebra,
descent and verification. Count setup, map, relation, rank/LA, descent,
verification, and any failed/censored stages. Compare with a matched
source-subgroup rho run on the same `(E,P,Q,r)`, charging both operation
counts and wall/CPU time. A completed component is not an attack-cost win.

Use native Rust for field, curve, function-field, divisor, solver and
benchmark code; Sage may be an independent library-backed cross-check but
is not the research implementation. Freeze source hashes, host, isolation,
seeds, stop limits, and outputs before each run. Before launching, update
`.resume/RESUME.md`, push an S3 checkpoint and pass
`scripts/checkpoint.sh gate`. Every run gets raw records, a quantitative
SVG, Markdown report, and visually inspected PDF under `reports/`.

## Immediate falsifiers

1. Any defining equation or D4 relation fails in the exact finite field.
2. `X` does not descend to `k`, has genus other than seven, or lacks the
   selected order-`r` subgroup.
3. `M o phi` maps the selected source generator to zero, fails scalar
   compatibility, or fails independent replay.
4. A primary source already gives this characteristic-two W1 Prym map.
5. The complete charged target recovery is slower than matched rho.
