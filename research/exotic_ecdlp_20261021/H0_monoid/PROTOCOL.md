# H0 — Additive-closure defect of a linear-size factor base (the C0 probe)

Pre-registration. Tests the organizing conjecture C0 empirically at small `p`.

| Field | Value |
|---|---|
| Object | `E/F_p` random, `p ∈ {1009, 2^16+k, 2^20+k}` (first good prime ≥ each). Factor base `F = {P ∈ E : 0 ≤ x(P) < ⌈p^{1/2}⌉}` (Semaev interval base), both signs of `y`. |
| Claim (C0 counterexample) | A size function closed under `+` with `≈√p` size-1 generators and `polylog`-length factorizations of random points. |
| Observable | (a) `|F|` vs `√p`. (b) For `m = 1..6`, the fraction `ρ_m` of `E(F_p)` reachable as a sum of `≤ m` elements of `F` (the `m`-fold sumset coverage). (c) The "factorization recursion defect": for a random point `R`, whether `R − Q` lands back in (a small neighborhood of) `F` for some `Q ∈ F` — i.e. whether greedy factorization makes progress, measured as the median `min_{Q∈F} rank(x(R−Q))` where `rank` is the position of `x` in sorted order. |
| Null (rho) | `ρ_m ≈ (|F|/ N)^? ` grows only as the sumset, with no recursive shortcut; greedy factorization does not reduce `x`-rank below uniform. Relation prob × solve ≥ `√N`. |
| Win | `ρ_m` reaches `Θ(1)` at `m = O(polylog)` **and** greedy factorization strictly decreases a size at each step (recursion works) → C0 counterexample candidate; escalate to full validation gate. |
| Kill | Greedy factorization shows no size decrease (x-rank of `R−Q` is uniform, median ≈ `p/2`), confirming quadratic-height non-recursiveness; `ρ_m` matches the dumb sumset bound. |
| Scale | Defect (uniformity of `R−Q` rank) must stay flat or worsen as `p` grows across the three sizes. |

## Method
1. Build `E`, `N`, a generator `G`. Build `F` by enumerating `x ∈ [0, ⌈√p⌉)`, lifting to points. Record `|F|`.
2. Sumset coverage: for `m = 1..6`, sample `T = 2000` random target points `R`; estimate `ρ_m` = fraction expressible as `±P_{i_1} ± … ± P_{i_j}`, `j ≤ m`, by randomized subset sums (not exhaustive): draw `K = 10^5` random signed `m`-subsets of `F`, hash their sums, measure target hit-rate; report the coverage curve.
3. Recursion defect: for `T = 2000` random `R`, compute `min over Q∈F` of the sorted-rank of `x(R−Q)` in `[0,p)`; report the distribution. Uniform ⇒ median ≈ `0.5` (normalized). Also report the autocorrelation: does iterating `R ← R − Q*` (best `Q`) reduce the rank monotonically over 10 steps?
4. Output JSON: `|F|`, `sqrt_p`, `rho_m` curve, rank-defect histogram summary (mean/median/min), recursion-progress slope.

## Decision rule
Kill if, at all three `p`, the normalized `x(R−Q)` rank is statistically uniform (KS test p-value > 0.01 against uniform) and the recursion slope is ≥ 0 (no decrease). Win only if a monotone size decrease appears and strengthens with `p`.

## Isolab
One job, `cpu 2 mem 4Gi seconds 1200`. Cheap. Deterministic seed recorded.
