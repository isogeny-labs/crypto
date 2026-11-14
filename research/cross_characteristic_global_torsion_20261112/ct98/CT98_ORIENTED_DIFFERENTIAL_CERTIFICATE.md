# CT98 — Exact oriented torsion transport, with an explicit hidden-log certificate

**2026-11-14. [pcc:unverified].** Fixed 11a1, source place 37, target place 2, seven-torsion only. This completes a *toy oriented coordinate transport* for six public nonzero source subgroup points, but intentionally demonstrates that the chosen ordinate algorithm is **not scalar-independent**. It therefore does not resolve the decisive arbitrary-input transfer gate or establish any DLP advantage.

## Prior work and hypothesis

CT80–94 fix a characteristic-zero oriented seven-torsion generator T=(alpha,Y), L=Q(alpha) of degree 24 and K=L(Y) of degree 48, with compatible source specialization (alpha,Y)→(9,4) in E(F37) and binary specialization (alpha,Y)→(2,Y_2) with Y_2²+Y_2=12 in F_(2^24). CT97 globally recognizes one of three x-roots for each public source x in {9,27,28}, without supplying a scalar and without choosing a new target prime.

CT85 already proved global non-CM endomorphism rigidity and observed that an exact rational multiplication map reveals |m| through its pole coefficient. Here we test whether the *same x-root rational function* supports an exact oriented ordinate, including safe reduction at characteristic 2. The invariant differential identity is classical (Silverman, **Arithmetic of Elliptic Curves**, 2nd ed., chapter III on isogenies and chapter IV on invariant differentials: https://www.math.brown.edu/johsilve/AECHome.html). No novel differentiation theorem is claimed. Research-KB MCP and wrapper remain unavailable in this connector-only context; repo source CT80/84/85/87/91/93/94/95–97 and textbook were checked as fallback.

## Certificate and exact limitation

Write E: Y²+Y=f(X), f(X)=X³−X²−10X−20, D(X)=4f(X)+1.
Let the x-map recognized by CT97 be r=A/B in Q(X), with integral coprime A,B and deg A=deg B+1. **This test verifies rather than assumes** that r is a multiplication x-map.

Define

    m² = lc(B)/lc(A),    N=A'B−AB',
    H=4A³−4A²B−40AB²−79B³.

For all three recognized rational x-functions, the leading ratio is a positive integer square, and the exact polynomial identity is

    D N² = m² B H.                         (1)

Equation (1) is the invariant-differential certificate: [m]^*(dX/(2Y+1))=m dX/(2Y+1). The two possible global ordinates are obtained, provided the following actual integral polynomial divisions succeed, from

    V=N/m,  U=(V−B²)/2,
    Y_r = (U+VY)/B²,       Y_r^− = −1−Y_r.       (2)

Both V and U have integral coefficients in these three fixtures. Crucially, (2) uses **no reduction of an unverified half-integer modulo two**. We check the exact polynomial identity

    U²+U B²+V² f(X) = B (A³−A²B−10AB²−20B³). (3)

Therefore (r(alpha),Y_r(alpha,Y)) is an actual global point in K. At the source place the public input y selects (2) or its inverse. The same chosen algebraic element specializes at the pre-fixed binary prime; B(alpha) is a unit there, so the formula is valid in CT84's F_(2^12)[Y_2] presentation, including negative points.

**Failure of scalar-independence:** the exact x-coordinate rational map carries m² in its leading coefficient *before* the ordinate is constructed. Choosing the sign with the public source y returns m modulo 7 on all six inputs. This is a complete source DLP disclosure from the reconstructed global *multiplication map*, not a cryptanalytic speedup. It is a deterministic obstruction to claiming that this particular map-reconstruction-plus-differentiation pipeline solves the DLP faster than rho. It is **not** an information-theoretic obstruction to other, genuinely input-conditioned global point-recognition algorithms. [pcc:unverified]

## Exact public oriented transport

CT98 Python imports CT97's actual scalar-blind x-root recognizer, caches its three exact rational functions, and chooses the source x residue and public y. It derives m from the map and checks equations (1),(3), 2-integrality, both fixed denominator units, and the final binary curve equation. Its binary coordinates are in the CT84 basis: x is an F_(2^12) integer mask, y=(u,v) means u+vY_2 with Y_2²+Y_2=12.

| Source Q in E(F37)[7] | Binary x | Binary y=(u,v) | Disclosed m mod 7 |
|---|---:|---|---:|
| (9,4) | 2 | (0,1) | 1 |
| (9,32) | 2 | (1,1) | 6 |
| (27,9) | 17 | (77,1) | 2 |
| (27,27) | 17 | (76,1) | 5 |
| (28,0) | 1562 | (2879,1) | 3 |
| (28,36) | 1562 | (2878,1) | 4 |

Separately written JavaScript uses **BigInt integer polynomial arithmetic** to verify (1),(3) and numerator divisibility, then independently checks the binary F_(2^24) curve and group law; it verifies that these positive points are G,2G,3G and negative points are elliptic inverses. It does **not** independently discover CT97's global x-roots, and uses the classical [1],[2],[3] rational functions only as **independent after-the-fact witness fixtures**. The Python program is the integration test of CT97 source-root selection; the JS program is an independent mathematical certificate.

Local development checks ran Python against exact CT97-equivalent reference rational functions (not the full CT97 source module), Python -O, JS and JS --no-opt: canonical JSON agreed, SHA256 `3ccef4ce62b4631553ec52a0bbd3f2b21b04db0dc30b22794c655b42e5c049b3`. The **committed Python/CT97 integration** still requires its separate exact-head GitHub CI before being called fully replayed. Independent signed scientific PCC remains outstanding.

## Replay (bounded; no cloud spending)

From repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct98
python3 -m pip install 'sympy==1.14.0'
python3 -B "$D/ct98_oriented_certificate.py" >/tmp/ct98-py.json
python3 -O -B "$D/ct98_oriented_certificate.py" >/tmp/ct98-opt.json
node "$D/ct98_oriented_independent.js" >/tmp/ct98-js.json
node --no-opt "$D/ct98_oriented_independent.js" >/tmp/ct98-jsopt.json
cmp /tmp/ct98-py.json /tmp/ct98-opt.json
cmp /tmp/ct98-py.json /tmp/ct98-js.json
cmp /tmp/ct98-py.json /tmp/ct98-jsopt.json
printf '%s  %s\n' 3ccef4ce62b4631553ec52a0bbd3f2b21b04db0dc30b22794c655b42e5c049b3 /tmp/ct98-py.json | sha256sum -c
```

## Cost and next decision

This toy completes a six-point *coordinate* check but not a nontrivial solver transfer: it literally determines the source log as an intermediate value and has an input-independent three-point multiplication-map witness setup, not a uniform cryptographic-scale factorization. It cannot be entered as a cost advantage in the ECDLP ledger. The CT87 cubic setup, field-degree 24 and 48 constructions, CT97 nullspace and binary field arithmetic are positive costs; neither cold setup wall time nor optimized rho/IC matched reference have been measured in a controlled deployment. No paid EKS, full cargo/Lean/Conductor audit, protected merge or independent PCC signature was used. [pcc:unverified]

The next **genuinely scalar-independent** test is to take only the algebraic x-root beta and solve the ordinate quadratic in K by general algebraic factorization or square-root extraction in L (no rational multiplication map, no extraction of its leading coefficient). Fix the source y and binary place before factorization, require a globally certified y-root and 2-integral representation, then charge the complete cold cost. A second independent curve/prime fixture is necessary before attempting any generalization.
