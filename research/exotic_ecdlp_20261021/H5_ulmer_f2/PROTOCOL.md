# H5 — Ulmer Legendre at d = p^f+1, f=2: is the specialization kernel geometric

Pre-registration. Tests C5. The f=1 collapse does not settle f=2.

| Field | Value |
|---|---|
| Object | Ulmer Legendre curve `E_t: y^2 = x(x+1)(x+t)` over `F_p(u)` with `t = u^{d}`, `d = p^f+1`, `f=2`, `p ∈ {7, 11}`. Explicit sections `P_ζ` for `ζ ∈ μ_d`. Specialization at `u_0 ∈ F_{p^{2f}}` gives `λ` and lands sections in `E_λ(F_{p^{2f}})`; rank over the function field is `≈ p^f`. Kernel of specialization `K = ker(Z^{r} → E_λ(F_{p^{2f}}))`, index `≤ p^{2f}`. |
| Claim (C5 counterexample) | `K` (equivalently a spanning set of relations among the `P_ζ` in `E_λ`) is computable from the intersection form / Hermitian-surface geometry at corank `o(r)` and negligible cost, giving sub-rho relations in the fiber subgroup. |
| Observable | (a) the true `K` computed by linear algebra on specialized points: its index, a reduced basis, shortest-vector heights. (b) the geometric relation lattice `K_geo` spanned by (i) Ulmer's known Mordell–Weil relations and (ii) intersection-form/pencil identities; `dim K_geo` and whether `K_geo = K` or `K_geo ⊊ K`. (c) the corank `r − rank(K_geo)` = how many relations still need collection. |
| Null (C5) | `K_geo` captures only Ulmer's `O(1)` relations (and tautologies); `K/K_geo` has corank `Θ(r)`, so the rest needs relation collection at the rho rate of `E_λ(F_{p^{2f}})` — no shortcut. (This is the f=1 collapse generalized.) |
| Win | `K_geo = K` (or corank `o(r)`) at negligible cost → the fiber DLP relations are free from geometry; escalate. |
| Kill | corank `K/K_geo = Θ(r)` at both `p`; geometry gives `O(1)` relations only. |
| Scale | corank/`r` across `p ∈ {7,11}` (and the ratio's trend); `Θ(1)` ⇒ generic. |

## Method
1. Build `E_t` over `F_p(u)`, `t=u^{p^2+1}`. Construct the explicit sections `P_ζ = (ζ·g(u), …)` per Ulmer's formulas for the Legendre/Hermitian family (verify each is on the curve over `F_p(u)`).
2. Pick `u_0 ∈ F_{p^4}` generic (so `λ=u_0^{p^2+1}`); specialize all sections to `E_λ(F_{p^4})`. Compute the group `E_λ(F_{p^4})` structure and the map `Z^r → E_λ` (r = number of sections).
3. `K` = kernel of that map (Sage: build the matrix of discrete-log coordinates of each `P_ζ` w.r.t. generators of `E_λ(F_{p^4})`; `K = left kernel`). Record index, LLL-reduced basis, shortest heights.
4. `K_geo`: (i) encode Ulmer's two known MW relations; (ii) enumerate intersection-form relations from the Hermitian pencil (lines through fibers) restricted to these sections. Build `K_geo` as their span; compute `rank(K_geo)`, and `corank = r − rank(K_geo)` relative to `K` (i.e. `dim K − dim(K_geo ∩ K)`, with `K_geo ⊆ K` checked).
5. Output JSON per `p`: `r`, `|E_λ(F_{p^4})|` factorization, `index(K)`, `dim K`, `dim K_geo`, corank, shortest-vector heights, whether `K_geo=K`.

## Decision rule
Kill if corank `= Θ(r)` (specifically `≥ 0.5·r`) at both `p`, confirming geometry yields only `O(1)` free relations. Win if corank `= o(r)` and shrinks with `p`.

## Isolab
One job, `cpu 8 mem 16Gi seconds 3600`. `p=11, f=2` means `E_λ(F_{11^4})` of size `~11^4≈1.5e4` and `r≈121`; discrete logs there are cheap. `p=7` first.
