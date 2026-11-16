# CT104 — Non-CM endomorphism rigidity and canonical rational-map leakage

**2026-11-14. Scientific standing: [pcc:unverified].** Exact, instance-specific obstruction for the fixed 11a1 curve and **algebraic self-endomorphisms**. It does *not* rule out correspondences, covers, different Jacobians, nonalgebraic computable transports, or the global lifting inverse in CT103.

## Starting point and sources

Read root crypto PR #3877, stacked cross-characteristic #4102 (CT95, CT102, CT103), today's `docs/session-logs/2026-11-14.md`, and `AGENTS.md`. The concurrent degree-13 Jacobian evaluator is in cryptanalysis PR #1523 and remains untouched. Crypto-autoresearcher is running unrelated transfer leads; no duplicate implementation was started.

Repository primary evidence: CT95's (4,3) Padé obstruction, CT102's irreducible degree-60 eleven-division polynomial / degree-120 oriented field proof, and CT103's explicit source F59 and target F2^40 places. Relevant classical sources: [Weierstrass invariants and j](https://mathworld.wolfram.com/j-Invariant.html), [singular moduli / integrality](https://arxiv.org/abs/1402.1632), [Galois image and division fields](https://arxiv.org/abs/2206.14959), and [GHS descent analysis](https://www.cambridge.org/core/journals/lms-journal-of-computation-and-mathematics/article/analysis-of-the-ghs-weil-descent-attack-on-the-ecdlp-over-characteristic-two-finite-fields-of-composite-degree/D4B264D082FF67354EE9968D9803102A). No new prior-art/novelty claim. **KB unavailable:** this runtime exposes neither research-KB MCP functions nor a repo checkout for the `scripts/research-kb` wrapper; repository reports and published sources were used instead.

## An exact non-CM certificate

For E: y²+y=x³−x²−10x−20 (11a1), its general Weierstrass invariants are

    (b₂,b₄,b₆,b₈)=(-4,-20,-79,-21)
    c₄=496,   Δ=−161051=−11⁵,
    j(E)=c₄³/Δ=−122023936/161051.

The numerator is coprime to 11, so j is **not** an integer. Since the j-invariant of a CM elliptic curve is an algebraic integer, and a rational algebraic integer is an ordinary integer, E has no complex multiplication. In characteristic zero,

    End_(overline Q)(E)=Z.

Thus every algebraic **origin-preserving self-morphism** of E over the algebraic closure is multiplication [n] by an integer. If an algebraic map E→E also has a translation, it is not origin-preserving unless that translation is zero.

**Scope:** this does not apply to endomorphisms of a *different* or higher-genus Jacobian; maps of E into another variety; non-morphism set maps of torsion; or the theoretical cross-characteristic inverse-specialization defined by CT103. It excludes an exotic algebraic *self-endomorphism* as the mechanism behind CT84-style source rational maps.

## Division-polynomial degree leakage and noncollision

For n≥1, standard division polynomials give

    x([n]T) = φₙ(X)/ψₙ(X)²,   deg φₙ=n²,   deg ψₙ²=n²−1.

For even n, ψₙ² means the polynomial in X obtained after eliminating Y via (2Y+1)²=4X³−4X²−40X−79.

Independent Python and BigInt JavaScript build ψ₃, ψ₄/ψ₂, ψ₅ and ψ₆/ψ₂ from the exact recurrence and construct φₙ and ψₙ² for n=1,…,5. Their reduction modulo 101 and modulo 103 has gcd 1, with leading coefficients 1 and n² preserved, certifying the rational functions are coprime over Q. Each x-map is the genuine global multiplication-by-n rational function.

| n | deg φₙ | deg ψₙ² | x([n](19,22)) mod 59 |
|---:|---:|---:|---:|
| 1 | 1 | 0 | 19 |
| 2 | 4 | 3 | 20 |
| 3 | 9 | 8 | 43 |
| 4 | 16 | 15 | 46 |
| 5 | 25 | 24 | 22 |

The code also checks **all 121 ordered addition pairs** in the order-eleven subgroup of E(F59); each rational x-map agrees with the independently implemented point law.

For any two different 1≤m<n≤5, the nonzero polynomial

    φ_m ψ_n² − φ_n ψ_m²

has degree at most m²+n²−1, in fact at most **40** over those ten pairs. CT102 establishes that X=x(T) has degree **60** over Q. Hence no two canonical x-maps collide when evaluated at X, and the low-degree rational certificates cannot be confused by reduction in the global x-field.

This makes an important interpretation precise: a recognizer that returns a **canonical multiplication rational function** [m] for a public Q has effectively identified the scalar up to sign already. CT84's successful [2] Padé formula is a valid exact toy reconstruction, but by itself does not exhibit a new non-scalar algebraic endomorphism. An arbitrary alternate representation in K need not expose m; this is **not** an unconditional DLP hardness proof.

## Cold-cost accounting gate

The fixed CT103 representation has exact x-field degree **60**, oriented global degree **120**, and characteristic-two target field degree **40**. A dense coefficient-vector representation of an oriented global element uses 120 rational slots, while a dense ψ₁₁ coefficient list has 61 integer slots. These are **representation-format counts**, not a lower bound for sparse/straight-line-program encodings or a same-host runtime speed result.

For a general prime ell with full GL₂(F_ell) Galois action, the oriented point field has degree ell²−1 and the x-field has degree (ell²−1)/2, because the Galois orbit of a nonzero torsion vector has that size and ± vectors share x. Producing a dense full-degree element would then take Ω(ell²) coefficient slots, while generic rho has expected O(sqrt(ell)) group operations; *slots and group operations are not comparable timing units*. The full-image clause is conditional, **not** deduced for all ell from CT102's ell=11 field irreducibility.

A frozen stage-by-stage cold-cost protocol is in `CT104_COLD_COST_PROTOCOL.json`; all unexecuted stages remain explicitly `not_measured`, particularly arbitrary-input global root recognition and target DLP. Do not use setup-free or multi-target amortized numbers as a cold single-target speed claim.

## Four-way independent reproduction

Run from the repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct104
python3 -B "$D/ct104_endomorphism_gate.py" >/tmp/ct104-py.json
python3 -O -B "$D/ct104_endomorphism_gate.py" >/tmp/ct104-pyopt.json
node "$D/ct104_endomorphism_gate_independent.js" >/tmp/ct104-js.json
node --no-opt "$D/ct104_endomorphism_gate_independent.js" >/tmp/ct104-jsopt.json
cmp /tmp/ct104-py.json /tmp/ct104-pyopt.json
cmp /tmp/ct104-py.json /tmp/ct104-js.json
cmp /tmp/ct104-py.json /tmp/ct104-jsopt.json
printf '%s  %s\n' 8c70653b82e879a6719b9beebbf3e62c5c861b4692c4d46dc1283213c1b9bbfa /tmp/ct104-py.json | sha256sum -c
```

All four commands passed locally against equivalent committed source prior to submission; output SHA-256 `8c70653b82e879a6719b9beebbf3e62c5c861b4692c4d46dc1283213c1b9bbfa`. Python and JS use separate source-group implementations and independently calculate b-invariants and division polynomials. Checks remain active under optimized Python. Exact-head CI and independent signed scientific PCC are separate gates. No root/full Rust/Conductor execution or paid EKS workload was launched.

## Remaining decisive route

A truly online, scalar-independent root recognition routine would receive only encoded public Q, the **fixed** K and the two **fixed** primes and output the globally certified oriented point of E(K)[11] reducing to Q. The image at the fixed binary prime must then be determined **without** an indexed mP→[m]U table. Test and price its actual field operations, precision, coefficient height and error rejection, before target DLP and matched source rho/BSGS/IC on the same single-target cold protocol. The present CT104 obstruction closes only the exotic-self-endomorphism and canonical rational-function loopholes; it does not finish that route.
