# H2 — Hermitian-surface Néron–Severi relation spectrum (is the pushed-down matrix generic)

Pre-registration. Tests C2. This is the "one afternoon of Sage" falsifier.

| Field | Value |
|---|---|
| Object | Hermitian surface `H: x_0^{p+1}+x_1^{p+1}+x_2^{p+1}+x_3^{p+1}=0` over `F_{p^2}`, `p ∈ {7,11,13,31}`. Its lines (each `H` line is a `q+1`-secant structure; `H(3,q)` with `q=p^2` has `(p+1)(p^3+1)` lines (the count `(p^3+1)(p^2+1)` is the number of points) forming the generalized quadrangle `GQ(p^2,p)`... use the standard `GQ(q,p)=GQ(p^2,p)` incidence). A Legendre fiber `E_λ/F_{p^2}` with `λ = N(u_0)`, and the rational map `H ⇢ E_λ` restricting to points. |
| Claim (C2 counterexample) | The relation matrix `M` (rows = lines of `H` restricted to the fiber, cols = points of `E_λ(F_{p^2})` or factor-base classes) has rank deficit growing with `p`, or a spectral gap, enabling a solve in `o(|cols|^ω)` / `o(rho)`. |
| Observable | (a) `rank(M)` over `F_ℓ` (ℓ the large prime factor of `#E_λ`) and over `Q`; the corank vs trivial-relation count. (b) singular-value / eigenvalue spectrum of `M Mᵀ`; compare to (i) a random 0/1 matrix of equal density, (ii) the known `GQ(p^2,p)` incidence eigenvalues `{(p^2+1)(p+1), p^2-p-1 (mult), -(p+1) (mult)}`. (c) number of non-tautological relations (those not equal to a pencil = hyperplane-class identity). |
| Null (rho) | `rank(M)` = full minus the tautological pencil relations; spectrum = exactly the `GQ` incidence spectrum (strongly regular), which encodes no extra structure usable below `|cols|`. |
| Win | corank grows super-linearly in `p` beyond tautological, or an extra eigenvalue multiplicity outside the SRG spectrum → implicit fast solve; escalate. |
| Kill | `rank(M)` and spectrum match the `GQ(p^2,p)` prediction to numerical tolerance at all four `p`; the only relations are tautological + the two Ulmer Mordell–Weil ones. |
| Scale | corank/|cols| and spectral residual vs the SRG prediction, across `p ∈ {7,11,13,31}`. Flat/zero residual ⇒ generic. |

## Method
1. Build `F_{p^2}`, the Hermitian surface, enumerate its lines (use the unitary group action / standard parametrization; for small `p` brute-force over `P^3(F_{p^2})` lines is fine: `~p^6` lines total, filter those lying on `H`). Verify count `(p^3+1)(p^2+1)`.
2. Build the fiber curve `E_λ` and the restriction map (points of `H` whose coordinates satisfy the Legendre model after the standard quotient; concretely use the norm-circle model `N(x)=λ` ↔ `x_0^{p+1}+... ` relation). For each line, its `≤ p+1` points push to a multiset of `E_λ` points summing to a fixed class (the hyperplane class image).
3. Assemble `M`: rows = lines, columns = factor-base = representatives of `E_λ(F_{p^2})` points (or their `⟨±1,Frob⟩`-classes). Entry = signed multiplicity.
4. Compute `rank(M)` over `F_ℓ` and `Q` (Sage `M.rank()`), the tautological-relation subspace (span of pencil=hyperplane identities), the corank beyond it, and `eigenvalues(M*M.T)` (or singular values). Compare to the SRG spectrum and to a density-matched random matrix (10 samples, report rank/spectrum distribution).
5. Output JSON per `p`: line count, |cols|, rank, tautological dim, non-taut corank, top eigenvalues, SRG-predicted eigenvalues, random-matrix rank mean/sd, residual.

## Decision rule
Kill if, at all `p`, `(rank beyond tautological) = min(rows,cols) − tautological` (i.e. generic full rank of the non-trivial part) and the spectrum residual vs SRG < 1e-6. Win if non-tautological corank/|cols| grows with `p` or an off-SRG eigenvalue appears.

## Isolab
One job, `cpu 8 mem 16Gi seconds 3600`. `p=31` is the heavy cell (`|cols|~p^2≈10^3`, lines `~p^5≈3·10^7` — enumerate by unitary orbits, not brute P^3, at p=31; brute force only p≤13). If `p=31` exceeds memory/time, report it censored and keep `7,11,13` as the scale trend.


> Erratum (2026-10-23, found by the H2 runner): lines of H(3,p^2) number (p+1)(p^3+1); (p^3+1)(p^2+1) counts points. N N^T has spectrum {(p+1)(p^2+1), p^2+p, 0}. No faithful map from H to a Legendre fibre exists, so H2 tests a norm-circle surrogate.
