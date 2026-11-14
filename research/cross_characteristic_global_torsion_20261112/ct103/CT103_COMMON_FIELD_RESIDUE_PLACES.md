# CT103 — Two explicit unramified residue places in the same degree-120 torsion field

**2026-11-14. Scientific standing: [pcc:unverified].** This resolves a *fixed common-field prime-selection* step for the CT101–102 eleven-torsion fixture. It is not an online arbitrary-public-point lift, a cross-characteristic elliptic-curve morphism, or a cheaper DLP algorithm.

## Dependencies and prior literature

Local repository evidence: CT101's exact division polynomial, finite-field source and target factorization, and 121/363 group-operation tests; CT102's unconditional irreducibility over Q and negative discriminant norm; CT94's Galois-equivariant Frobenius caveat; the current AGENTS.md, session log and root PR #3877. External classical reference: [Snowden, Elliptic curves over DVRs, Lecture 8](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html), on prime-to-p finite étale torsion and reduction; [Sage finite-field Frobenius polynomial](https://sporadic.stanford.edu/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html). The research-KB MCP and repository CLI are not available in this connector environment; no novelty or absence-of-prior-art claim is made.

## A single global field and two concrete places

Use the **same** integral eleven-division polynomial F(X)=psi_11(X) as in CT101, with degree 60 and leading coefficient 11, for

    E: Y^2+Y = X^3-X^2-10X-20.

Write h(X)=X^3-X^2-10X-20 and form the finite free algebra

    R = Z[1/11][X,Y]/(F(X), Y^2+Y-h(X)).

Since 11 is invertible, F/11 is monic. R is finite free of rank 120 over Z[1/11]. CT102 rigorously proved F irreducible over Q and D(X)=1+4h(X) nonsquare in Q[X]/(F) by its **negative rational norm**. Therefore the generic fiber

    K=R tensor Q = Q(X,Y)/(F(X),Y^2+Y-h(X))

is a **field of degree 120** (not merely a 120-dimensional product ring).

Fix the source evaluation, including the ordinate,

    rho_59(X)=19, rho_59(Y)=22,   into F_59.

The independent verifiers reconstruct F and check F(19)=0 mod59, 22^2+22=h(19) mod59, and the derivative units F'(19)=17, 2*22+1=45 mod59. Hence this is a degree-one **unramified** maximal ideal of the displayed order, with no hidden need for scalar-dependent prime selection.

Fix the target polynomial

    g(X)=X^20+X^17+X^16+X^5+X^2+X+1  (mask 0x130027).

Both verifiers check g is irreducible over F2, and g is an exact factor of F mod2; the three CT101 binary factors multiply to F mod2. Let t=X mod g in k=F2[t]/(g). Put r=t^3+t^2. The explicit trace calculation gives Tr_(k/F2)(r)=1. Consequently

    L=k[u]/(u^2+u+r) = F_(2^40)

is a field. Define the second **fixed** evaluation

    rho_2(X)=t, rho_2(Y)=u,   into L.

The relations of R vanish under both evaluations, so these are actual surjective ring maps **from one and the same global order R**, not independently assigned geometric points. Their maximal ideals have residue degrees 1 and 40. At the target, F has distinct squarefree factors and d(Y^2+Y+r)/dY=1; the chosen prime is unramified and avoids the nonmaximal-order residue ambiguity seen in CT88's seven-torsion subgroup generator. This is a pair of distinct residue maps of K, **not** a map of fields F59 -> F_(2^40).

This construction fixes a global point T=(X,Y) in E(K) and its two specializations P=(19,22) and U=(t,u), with [11]T=O by the division polynomial and good reduction. For the selected fixed places, the map

    E(F59)[11] -> <U> subset E(F_(2^40))[11]

exists uniquely as an **abstract group homomorphism**: good-reduction injectivity of E(K)[11] into E(F59)[11], together with #E(F59)=55 and the order-11 generator P, shows E(K)[11]=<T> maps bijectively to E(F59)[11]. Reduce the unique lift of each input in E(K)[11] at rho_2. This is mathematically well-defined; **no online algorithm to recover that global lift from arbitrary encoded Q has been supplied**.

## Exact Frobenius and full oriented splitting of the common field

At p=59: trace t=59+1-55=5 and determinant 59=4 mod11. In a rational canonical basis on E[11], Frobenius acts by

    A59 = [[0,7],[1,5]] mod11.

Its eigenvalues are 1 and 4; 4 has multiplicative order five mod11. The 120 nonzero vectors decompose into **10 fixed vectors and 22 five-cycles**, giving the full oriented point field splitting 1^10 5^22. The corresponding x-field pattern 1^5 5^11 agrees with CT101.

At p=2: trace -2 and determinant 2. A companion matrix is

    A2 = [[0,9],[1,9]] mod11,

with characteristic polynomial Z^2+2Z+2, irreducible mod11. The 120 nonzero vectors form **three 40-cycles**; the full oriented field splits 40^3, matching CT101's three 20-degree x-factors each extended quadratically. All 120 vectors and their cycles are enumerated independently by Python and JavaScript. No full-GL2 Galois-image hypothesis is used.

## Reproduction and scope

From the repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct103
python3 -B "$D/ct103_fixed_places.py" >/tmp/ct103-py.json
python3 -O -B "$D/ct103_fixed_places.py" >/tmp/ct103-pyopt.json
node "$D/ct103_fixed_places_independent.js" >/tmp/ct103-js.json
node --no-opt "$D/ct103_fixed_places_independent.js" >/tmp/ct103-jsopt.json
cmp /tmp/ct103-py.json /tmp/ct103-pyopt.json
cmp /tmp/ct103-py.json /tmp/ct103-js.json
cmp /tmp/ct103-py.json /tmp/ct103-jsopt.json
sha256sum /tmp/ct103-py.json
```

Four development-local modes produced the same JSON digest **f6a26aa3c63ed6d2b7d1efb7c2c505cc76ecd5f39b60e6575c7ab489a65c4dc1**. Dedicated exact-head CI and signed scientific validation are separate. The two independently coded programs check generator relations, irreducible binary factors, trace, and complete companion-matrix cycles. They **do not** independently repeat CT102's degree-120 norm calculation, construct a normal integral basis of K, or solve arbitrary input logs.

### Next expensive step, now isolated

Obtain from arbitrary public Q=(x,y) in the source order-eleven subgroup a representation of its **unique** global lift in K (e.g. bounded certified polynomial/rational function in X,Y) *without* first determining Q=mP. Only then evaluate that representation under the fixed rho_2 and compute its target log. Freeze a complete cold protocol charging F, irreducibility, R and both primes, all p-adic or algebraic root recognition, certificate, target F_(2^40) operations, and final log; compare matched source rho/BSGS and admitted IC on equal public inputs. No EKS bill, protected merge, or complete ECDLP advantage is claimed.
