# CT336 — explicit bielliptic quotient maps from CT335's decomposable kernel

**2026-11-23 UTC. Source-bound, producer-executed Prime31 curve-map construction; unsigned [pcc:unverified].** This is a *deliberately decomposable positive control*, not an ordinary-prime-field ECDLP speedup. It does not interfere with the separately owned F3 E7 map lane [#5007](https://github.com/isogeny-labs/crypto/pull/5007) or original norm reuse.

## Actual two degree-two curve morphisms, not just kernels

Let `p=2147483647` and `C:y²=f(x)=∏_{a=1}^6(x-a)`, the original CT334 smooth genus-two model. The determinant-zero CT335 matching `(1,6)|(2,5)|(3,4)` reflects the explicit *extra* automorphism `σ:(x,y)→(7−x,y)`: six factors pair and `f(7−x)=f(x)`.

Define `t=x−7/2`, `u=t²`, `a=1/4`, `b=9/4`, `c=25/4` in `Fp`. Then the **polynomial identity**
```text
f(x) = (t² − a)(t² − b)(t² − c)
```
produces two rational, degree-two curve maps:
```text
pi_+: C -> E_+  , (x,y) -> (u=t², v=y)
       E_+: v² = (u−a)(u−b)(u−c)

pi_-: C -> E_-  , (x,y) -> (u=t², w=t*y)
       E_-: w² = u(u−a)(u−b)(u−c)
```
The deck involutions are `(t,y)↦(−t,y)` and `(t,y)↦(−t,−y)`, respectively. Both quotient curves are **smooth genus one**; `E_+` has rational 2-torsion, and `E_-` has the rational point `(u,w)=(0,0)`, allowing elliptic-curve group structures after choosing basepoints. These identities specify actual rational maps on the affine open, extending uniquely to morphisms of smooth projective curves. They are *not* yet implemented as a native Cantor/Mumford divisor algorithm.

The pulled-back invariant differentials satisfy
```text
pi_+^*(du/v) = 2t dx/y
pi_-^*(du/w) = 2 dx/y .
```
They are linearly independent in the two-dimensional space of regular differentials on `C`. Thus the induced pair of Jacobian pushforwards `J(C)→E_+×E_-` is a **separable isogeny**, giving two **nonzero rational factor maps** and an exact *structural* decomposability witness, not merely a symbolic Richelot determinant. The classical decomposable (2,2) Richelot kernel explains this product quotient; explicit Mumford images, its kernel and dual-composition are still unexecuted.

## Exact finite-field source and five public point images

In `Fp`, `7/2=1073741827`, and `(a,b,c)=(536870912,536870914,536870918)`. The actual elliptic-model polynomials (ascending coefficients) are:
```text
E_+ RHS: [1040187388, 402653200, 536870903, 1]
E_- RHS: [0, 1040187388, 402653200, 536870903, 1]
```
The frozen source tested full polynomial identities and smoothness, and **five explicit rational source affine points** with images on both quotients. One non-Weierstrass example is `(x,y)=(8,978299470)` mapping to
```text
E_+: (u,v)=(536870932,978299470)
E_-: (u,w)=(536870932,107380321)
```
The full five-point table, including negative-ordinate images, is in [CT336_RESULT.json](CT336_RESULT.json). This is genuine source-to-elliptic *point-map* checking at the curve level, but **not** Jacobian group-law or DLP checking. Three effective mutants (wrong half-shift, changed branch value, omitted `t` in `w=t*y`) were rejected.

## Provenance and test status

- [Preregistered protocol](CT336_PROTOCOL.md) committed first at `17af09e2e5c64223095a5a020fa1cdea8ae51a6d`, before the CT336 arithmetic run.
- The [executed exact BigInt JS source](ct336_bielliptic_maps.js), commit `439c9639c1fa0707773d5a08d50f19a12f088c4f`, Git blob `1e487d0d7774ac8416cb6a4964d1674c595d0cb4`, and [full 2,659-byte JSON](CT336_RESULT.json), commit `6743b9ff267dc5b3179aa1b6979fb58a4f83b8eb`, blob `c991383170dba7e9a1e17d210bffe9fd486e6605`, were read back from GitHub. The exact study function ran in the coordinator V8 tool; standalone `node research/chained_2power_descent_20261123/ct336_bielliptic_maps.js` is a **reproduction command**, not a claimed additional native CI run.
- A **separate SymPy finite-field implementation**, using `Poly`, `Matrix.det` and independently substituted polynomials, verified the two full identities, squarefree genus-one RHS, original Richelot `delta=0`, all 5/5 exact point-image records and 3/3 effective mutations in ordinary and `python3 -O` runs. Same coordinator, not an independently signed validator. Local checker SHA256 `3dff0bfcb84f35da89332b5454c368c83f13a1f3a971f588d2a8f07ca6807866`; it was not checked into GitHub.
- This is mathematical function-field algebra corroborated by two source paths. No Lean-kernel proof, original Rust/Cantor arithmetic, native same-host timing, PCC signatures, external cryptographic-size challenge or independent-person replay.

## Interpretation and next step

**This model is intentionally weak**: the rational automorphisms force an elliptic-product quotient, unlike a typical ordinary prime-field elliptic curve or genus-two Jacobian. It is only a positive control for a functioning cover/quotient chain, not evidence that a hard input ECDLP has been reduced. This distinction also prevents double-counting a trivial 2-isogeny reduction as an index-calculus speedup.

The next admissible component is an independent *Jacobian divisor* map for these explicitly verified curve morphisms (Cantor reduction, zero/degree-drop/infinity, additive composition and odd-order subgroup kernel), while **#5007 owns its separate F3/E7 native map** and must not be duplicated. The principal owner-reserved target still requires authenticated 695/701 Jacobian norm-point replay, `C169/C13a/C13b/N2_actual/D` physical pricing and full single-target same-host cold cost including setup/transfer/LA/memory against optimized rho, BSGS and strongest IC, plus signed PCC. No EKS/paid job, CI rerun, forbidden custody access, merge or complete ECDLP advantage occurred.

Background: [Magma Richelot isogenies](https://magma.maths.usyd.edu.au/magma/handbook/text/1615); [Kunzweiler 2024](https://link.springer.com/article/10.1007/s10623-024-01366-1). Repository KB/Conductor unavailable in the current tool runtime, so CT331–CT335 and cited primary sources served as fallback. Standard bielliptic-quotient mathematics; **no claim of novelty**.
