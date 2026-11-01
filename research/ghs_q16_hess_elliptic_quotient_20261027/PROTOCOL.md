# Four exact Hess elliptic-quotient witnesses

27 October 2026. This is an exploratory algebraic follow-up to the four already
measured `c=1` Hess genus-four controls. The immutable source is the exact-model
ledger `../ghs_q16_4126_hess_g4_20261027/EXACT_MODELS.md` and its four Magma
attack receipts. The goal is to test whether the observed elliptic complement
is realized by an explicit characteristic-two curve quotient.

For each `H: y²+h(x)y=f(x)` over `k=GF(16)`, solve over GF(2) for `B` of degree
at most six and `F` of degree at most three satisfying
`f+B²+hB=F(L)`, where `L=h-h(0)` is additive of degree four. Retain an output
only if `L'(x)≠0`, `deg F=3`, the normalized elliptic curve is nonsingular,
and its independently enumerated order equals the already recorded elliptic
complement order `#Jac(H)(k)/#E(K)`.

An independent four-bit GF(16) implementation must replay all four polynomial
identities, elliptic equations, point counts, and source-model encodings. Lean
must check the finite identities from the retained coefficient vectors. The
algebraic proof of the degree-four quotient uses the four translations by the
roots of `L` over the splitting field; it is stated separately from the
finite computations. Do not infer a map to a genus-three quartic or an attack
cost from the elliptic quotient.

Prior-work boundary: Hess (2004), Theorem 15, already describes factorization
of the descent curve's L-polynomial; the direct GHS map is implemented in
Magma. The exact equations here are new computations within this project.
The literature searches recorded in `REPORT.md` are bounded and do not prove
publication novelty of the quotient construction.
