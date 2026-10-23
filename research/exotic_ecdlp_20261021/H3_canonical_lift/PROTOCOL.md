# H3 — Canonical-lift p-adic digits as a candidate size (does any digit functional carry structure)

Pre-registration. Tests C3.

| Field | Value |
|---|---|
| Object | Ordinary `E/F_p`, `p ∈ {2^12, 2^16, 2^20}` (nearest prime, ordinary). Canonical lift `Ẽ/Z_p` to precision `p^3` (Satoh/Harley: lift `j` via the modular equation `Φ_p(J, J^σ)=0` fixed point, or use Sage's `E.canonical_lift` where available; else AGM for `p=2` is excluded since `p` odd — use the `lift_x`/Teichmüller of `j` + Newton on `Φ_ℓ`). Frobenius-fixed lift `P ↦ P̃` of each point, digits `x_0,x_1,x_2 ∈ F_p` of `x(P̃)`. |
| Claim (C3 counterexample) | An `F_p`-linear (or low-degree) functional `L(x_1,x_2)` that (i) is a group homomorphism `E(F_p) → (F_p,+)` beyond the trivial `0`, or (ii) has a sublevel set `{P : L small}` closed under `+` and of size `≈√p` — a "size". |
| Observable | (a) Homomorphism test: regress `d_k(P̃+Q̃)` against `d_k(P̃), d_k(Q̃)` for `k=1,2` over many random pairs; residual rank. (b) For every `F_p`-linear functional `L` of `(x_1,x_2)` (a 2-param family mod scaling), the additive defect `Δ(L)=#{(P,Q): L(P+Q)≠L(P)+L(Q)} / #samples`, minimized over `L`. (c) Mutual information between `(x_1 mod small)` and `log_G P` (bucketed). (d) closure: for the smallest-`|L|` sublevel set, the sumset escape rate. |
| Null (T3) | Digits of `P̃+Q̃` are the formal-group polynomial in digits of `P̃,Q̃` (group law), so a full-rank nonlinear dependence exists but **no** linear functional is additive except `0`; `Δ(L) ≈ 1−1/p` for all `L≠0`; MI ≈ 0; sublevel sets escape immediately. |
| Win | Some `L≠0` with `Δ(L)=o(1)` (an approximate additive character on digits), or MI bounded away from 0 growing with `p`, or a closed sublevel set → C3 counterexample; escalate. |
| Kill | `min_{L≠0} Δ(L)` stays `≈1−1/p`; MI consistent with 0 (within sampling); sublevel sets escape at the random rate — all three `p`. |
| Scale | `min Δ(L)` and MI vs `log p`; must stay at the null. |

## Method
1. Compute the canonical lift to precision `p^3`. Validation: check `#Ẽ mod p^3` consistency and that `φ̃` fixes the lifted points (`φ̃(P̃)=P̃` to precision). Use a mid-size `p` first; if `p=2^20` lift is too slow, cap at `p=2^18` and report.
2. For `S = min(N, 20000)` points `P=kG`, record `(k, x_0,x_1,x_2)` of `P̃`.
3. Homomorphism/additive-defect scan: sample `M=50000` random pairs `(P,Q)`; for a grid of `L=(α,β)` over `P^1(F_p)` (sample `≤ 4000` directions), compute `Δ(L)`. Report `min Δ`, the argmin `L`, and the distribution.
4. MI: bucket `x_1` into `√p` bins and `log_G P = k` into `√p` bins; estimate MI; compare to a shuffled-label null (permutation test, 200 shuffles).
5. Closure: take the argmin-`L` sublevel set `{P: L(P) < p/√p}` (~√p points); measure fraction of pairwise sums landing back in it vs the base-rate `1/√p`.
6. Output JSON per `p`: lift precision, validation flags, `min Δ(L)`, argmin, MI estimate + null band, closure ratio vs base-rate.

## Decision rule
Kill if at all `p`: `min_{L≠0} Δ(L) ≥ 1 − c/√p` (no approximate additivity), MI within the permutation null band, closure ratio within 2σ of base-rate. Win otherwise, with the scale trend confirming.

## Isolab
One job, `cpu 4 mem 8Gi seconds 3600`. Canonical lift at `p=2^20` is the risk; fall back to `2^18` and mark the largest cell censored if needed.
