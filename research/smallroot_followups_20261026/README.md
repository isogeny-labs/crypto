# Small-root decomposition, follow-ups: unequal bounds, non-convex monomial sets, the joint (4,2) lattice

2026-10-26 · follow-up to [`../smallroot_optimal_reach_20261026/`](../smallroot_optimal_reach_20261026/README.md)
(PR #2612) · tools [`examples/smallroot_optimal_reach.rs`](../../examples/smallroot_optimal_reach.rs)
(`theorem` mode) and [`examples/smallroot_joint_lattice.rs`](../../examples/smallroot_joint_lattice.rs)
(std only) · records `theorem_check.jsonl`, `joint_nu.jsonl`, `joint_control.jsonl` ·
graph `followups.svg` · PDF `report.pdf`

**Class: analysis (a proof, an exact check of it, and a computation of an
ideal invariant). Not an index-calculus measurement, no leaderboard row.**

## Verdict

| open item from PR #2612 | result | status |
|:--|:--|:--|
| unequal per-variable bounds | any lattice from `(p,f)^t` needs `h_N(δ) = max_{v∈supp f} ⟨δ,v⟩ < 1`: **ε < 3/4** for `S₃` in 3 unknowns, **ε < 1/2** in 2 unknowns, against a demand of 8/9 | **proved** (§1); exact check 9,600 cases, 0 violations (§2) |
| monomial sets that are not convex | the same theorem holds for **every** monomial set `M` | **proved** (§1), same check on random non-convex `M` |
| one joint lattice for the three-equation `(4,2)` system | the joint lattice needs `ν_s < s` for some `s` (`ν_s` = least `δ`-weight in `(f̄₁,f̄₂,f̄₃)^s`); computed `ν_s/s` = `min_k deg_δ f_k` ≥ **2.08** in the demand region for `s ≤ 3` | **computed, not proved** (§3): upper bounds from truncated Macaulay matrices, stable in the truncation; positive control passes |

With PR #2612 this makes the windowed-decomposition (small-root) family closed
for every single-block Coppersmith lattice, for any bound shape and any monomial
set, by a proof; the joint three-block lattice is closed by computation for
`s ≤ 3` with a factor-two margin, not by proof.

## 1. Theorem (any monomial set, any bounds)

Let `f ∈ Z[x₁..x_n]` with `f̄ = f mod p ≠ 0`, Newton support `N`, and bounds
`|x_i| < p^{δ_i}`, `δ ≥ 0`. Let `L` be any lattice of polynomials in
`(p, f)^t` supported on a finite monomial set `M` (Jochemsz–May, Ryan's optimal
shifts, any strategy; `M` arbitrary), and `E = log_p [L : p^t Z^M]`. If the
Howgrave-Graham determinant condition `Σ_{μ∈M} ⟨δ, μ⟩ < E` holds, then

> **`h_N(δ) := max_{v∈N} ⟨δ, v⟩ < 1`.**

*Proof.* (i) Filter `L` by `L_i = L ∩ p^i Z^M`. `p, f` is a regular sequence, so
`((p,f)^t : p^i) = (p,f)^{t−i}` and `L_i/L_{i+1}` embeds, by `g ↦ g/p^i mod p`, into
`W_{t−i} = f̄^{t−i} F_p[x] ∩ span M`. Hence `E ≤ Σ_{s=1}^{t} dim W_s`.
(ii) Fix a monomial order refining the weight `δ` (ties broken by any monomial
order). An echelon basis of `W_s` has distinct leading monomials; each is the
leading monomial of some `h·f̄^s`, whose `δ`-weight is
`deg_δ(h f̄^s) = deg_δ h + s·deg_δ f̄ ≥ s·h_N(δ)` (`F_p[x]` is a domain, so
`δ`-initial forms multiply). So a monomial `μ` is a leading monomial in at most
`⌊⟨δ,μ⟩ / h_N(δ)⌋` of the layers, and `E ≤ Σ_{μ∈M} ⟨δ,μ⟩ / h_N(δ)`.
(iii) With the condition, `Σ⟨δ,μ⟩ < E ≤ Σ⟨δ,μ⟩ / h_N(δ)`, so `h_N(δ) < 1`. ∎

No convexity is used. For `S₃` in three unknowns the vertices `(2,2,0)`,
`(2,0,2)`, `(0,2,2)` give `2(δ_i + δ_j) < 1` for all pairs, so
`ε = Σδ_i < 3/4`; in two unknowns the vertex `(2,2)` gives `ε < 1/2`, which is
the value the PR #2612 optimiser approached (0.489). The bound is sharp in the
univariate case (`dδ < 1`, Coppersmith's `1/d`). For three unknowns and equal
bounds the Brunn–Minkowski bound of PR #2612 (0.329, convex `M`) is stronger.

Scope: the determinant (Howgrave-Graham) criterion that every Coppersmith
strategy, Ryan's included, is judged by; dimension factors are dropped, which
favours the attack. It is not a statement about lattices that happen to contain
unusually short vectors beyond the determinant estimate.

## 2. Exact check of the theorem's key inequality

`smallroot_optimal_reach theorem` draws random monomial sets (each monomial of a
box kept with probability 0.45–0.85, so almost never convex), computes `E(M)`
exactly by elimination over `Z/p^t` (`p = 1000003`, superset box four wider), and
checks `E(M) ≤ Σ_μ min(t, ⌊⟨w,μ⟩/h_N(w)⌋)` for 12 random weights each.

| f | unknowns | box | random sets | with E > 0 | weight checks | violations | max E / bound | max λ*·h_N |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| linear | 2 | 6 | 200 | 200 | 2400 | **0** | 0.8438 | 0.4599 |
| univariate-d2 | 1 | 8 | 200 | 190 | 2400 | **0** | 1.0000 | 0.8571 |
| s3 | 2 | 6 | 200 | 170 | 2400 | **0** | 0.6897 | 0.3896 |
| s3 | 3 | 4 | 200 | 68 | 2400 | **0** | 0.2373 | 0.1181 |

`λ*·h_N(w) ≤ 1` is the theorem's conclusion in scaled form (`λ* = E/Σ⟨w,μ⟩`); `max E/bound = 1.0000` for the univariate case shows the inequality is attained.

## 3. The joint lattice for the three-equation (4,2) system

`f₁ = S₃(x₁,x₂,u)`, `f₂ = S₃(x₃,x₄,v)`, `f₃ = S₃(u,v,x_R)`; bounds `p^β` on the
leaves, `p^{γ_u}`, `p^{γ_v}` on `u, v`. The demand (CLOSURE.md §1) is `β < 1/4`
and `3β + γ_u + γ_v > 5/2`. Steps (i)–(ii) of §1 carry over to the ideal
`(p, f₁, f₂, f₃)^t` (assuming `f̄₁, f̄₂, f̄₃` is a regular sequence, which the
tree structure suggests), with `W_s = Ī^s ∩ span M`: a monomial counted in layer
`s` has `δ`-weight at least `ν_s = min{deg_δ g : 0 ≠ g ∈ Ī^s}`. So the joint
lattice can meet the condition only if **`ν_s < s` for some `s`**.

For one block `ν_s = s·h_N(δ)`. Here the `δ`-initial forms of `f₁` and `f₃`
share `u²` (`u²(x₁−x₂)²` and `u²v²`), and those of `f₂` and `f₃` share `v²`, so
the single-block argument does not apply: combinations such as
`v²f₁ − (x₁−x₂)²f₃` lose their top weight. No weight `w ≤ δ` with regular initial
forms has `deg_w f_k ≥ 1` for all `k` (the leaves force `4β < 1`), so no simple
weight argument closes it. `smallroot_joint_lattice` computes `ν_s(T)`, the least
`δ`-weight in the row space of the Macaulay matrix `{x^a f^α : |α| = s,
deg ≤ T}` over `F_p`. Truncation only drops ideal elements, so `ν_s(T)` is an
**upper bound** on `ν_s`.

| β | γ_u | γ_v | in demand region | deg_δ f₁, f₂, f₃ | s | T | rows × cols | ν_s(T) | ν_s/s |
|--:|--:|--:|:--|:--|--:|--:|:--|--:|--:|
| 0.17 | 0.99 | 0.99 | no | 2.32, 2.32, 3.96 | 1 | 6 | 84 × 924 | 2.32 | 2.32 |
| 0.17 | 0.99 | 0.99 | no | 2.32, 2.32, 3.96 | 1 | 8 | 630 × 3003 | 2.32 | 2.32 |
| 0.17 | 0.99 | 0.99 | no | 2.32, 2.32, 3.96 | 1 | 10 | 2772 × 8008 | 2.32 | 2.32 |
| 0.17 | 0.99 | 0.99 | no | 2.32, 2.32, 3.96 | 2 | 8 | 6 × 3003 | 4.64 | 2.32 |
| 0.17 | 0.99 | 0.99 | no | 2.32, 2.32, 3.96 | 2 | 10 | 168 × 8008 | 4.64 | 2.32 |
| 0.17 | 0.99 | 0.99 | no | 2.32, 2.32, 3.96 | 2 | 12 | 1260 × 18564 | 4.64 | 2.32 |
| 0.17 | 0.99 | 0.99 | no | 2.32, 2.32, 3.96 | 3 | 12 | 10 × 18564 | 6.96 | 2.32 |
| 0.17 | 0.99 | 0.99 | no | 2.32, 2.32, 3.96 | 3 | 13 | 70 × 27132 | 6.96 | 2.32 |
| 0.2 | 0.95 | 0.95 | no | 2.30, 2.30, 3.80 | 1 | 6 | 84 × 924 | 2.30 | 2.30 |
| 0.2 | 0.95 | 0.95 | no | 2.30, 2.30, 3.80 | 1 | 8 | 630 × 3003 | 2.30 | 2.30 |
| 0.2 | 0.95 | 0.95 | no | 2.30, 2.30, 3.80 | 1 | 10 | 2772 × 8008 | 2.30 | 2.30 |
| 0.2 | 0.95 | 0.95 | no | 2.30, 2.30, 3.80 | 2 | 8 | 6 × 3003 | 4.60 | 2.30 |
| 0.2 | 0.95 | 0.95 | no | 2.30, 2.30, 3.80 | 2 | 10 | 168 × 8008 | 4.60 | 2.30 |
| 0.2 | 0.95 | 0.95 | no | 2.30, 2.30, 3.80 | 2 | 12 | 1260 × 18564 | 4.60 | 2.30 |
| 0.2 | 0.95 | 0.95 | no | 2.30, 2.30, 3.80 | 3 | 12 | 10 × 18564 | 6.90 | 2.30 |
| 0.2 | 0.95 | 0.95 | no | 2.30, 2.30, 3.80 | 3 | 13 | 70 × 27132 | 6.90 | 2.30 |
| 0.24 | 0.9 | 0.9 | yes | 2.28, 2.28, 3.60 | 1 | 6 | 84 × 924 | 2.28 | 2.28 |
| 0.24 | 0.9 | 0.9 | yes | 2.28, 2.28, 3.60 | 1 | 8 | 630 × 3003 | 2.28 | 2.28 |
| 0.24 | 0.9 | 0.9 | yes | 2.28, 2.28, 3.60 | 1 | 10 | 2772 × 8008 | 2.28 | 2.28 |
| 0.24 | 0.9 | 0.9 | yes | 2.28, 2.28, 3.60 | 2 | 8 | 6 × 3003 | 4.56 | 2.28 |
| 0.24 | 0.9 | 0.9 | yes | 2.28, 2.28, 3.60 | 2 | 10 | 168 × 8008 | 4.56 | 2.28 |
| 0.24 | 0.9 | 0.9 | yes | 2.28, 2.28, 3.60 | 2 | 12 | 1260 × 18564 | 4.56 | 2.28 |
| 0.24 | 0.9 | 0.9 | yes | 2.28, 2.28, 3.60 | 3 | 12 | 10 × 18564 | 6.84 | 2.28 |
| 0.24 | 0.9 | 0.9 | yes | 2.28, 2.28, 3.60 | 3 | 13 | 70 × 27132 | 6.84 | 2.28 |
| 0.2 | 1 | 0.9 | no | 2.40, 2.20, 3.80 | 1 | 6 | 84 × 924 | 2.20 | 2.20 |
| 0.2 | 1 | 0.9 | no | 2.40, 2.20, 3.80 | 1 | 8 | 630 × 3003 | 2.20 | 2.20 |
| 0.2 | 1 | 0.9 | no | 2.40, 2.20, 3.80 | 1 | 10 | 2772 × 8008 | 2.20 | 2.20 |
| 0.2 | 1 | 0.9 | no | 2.40, 2.20, 3.80 | 2 | 8 | 6 × 3003 | 4.40 | 2.20 |
| 0.2 | 1 | 0.9 | no | 2.40, 2.20, 3.80 | 2 | 10 | 168 × 8008 | 4.40 | 2.20 |
| 0.2 | 1 | 0.9 | no | 2.40, 2.20, 3.80 | 2 | 12 | 1260 × 18564 | 4.40 | 2.20 |
| 0.2 | 1 | 0.9 | no | 2.40, 2.20, 3.80 | 3 | 12 | 10 × 18564 | 6.60 | 2.20 |
| 0.2 | 1 | 0.9 | no | 2.40, 2.20, 3.80 | 3 | 13 | 70 × 27132 | 6.60 | 2.20 |
| 0.24 | 1 | 0.8 | yes | 2.48, 2.08, 3.60 | 1 | 6 | 84 × 924 | 2.08 | 2.08 |
| 0.24 | 1 | 0.8 | yes | 2.48, 2.08, 3.60 | 1 | 8 | 630 × 3003 | 2.08 | 2.08 |
| 0.24 | 1 | 0.8 | yes | 2.48, 2.08, 3.60 | 1 | 10 | 2772 × 8008 | 2.08 | 2.08 |
| 0.24 | 1 | 0.8 | yes | 2.48, 2.08, 3.60 | 2 | 8 | 6 × 3003 | 4.16 | 2.08 |
| 0.24 | 1 | 0.8 | yes | 2.48, 2.08, 3.60 | 2 | 10 | 168 × 8008 | 4.16 | 2.08 |
| 0.24 | 1 | 0.8 | yes | 2.48, 2.08, 3.60 | 2 | 12 | 1260 × 18564 | 4.16 | 2.08 |
| 0.24 | 1 | 0.8 | yes | 2.48, 2.08, 3.60 | 3 | 12 | 10 × 18564 | 6.24 | 2.08 |
| 0.24 | 1 | 0.8 | yes | 2.48, 2.08, 3.60 | 3 | 13 | 70 × 27132 | 6.24 | 2.08 |
| 0.2 | 0.5 | 0.5 | no | 1.40, 1.40, 2.00 | 1 | 6 | 84 × 924 | 1.40 | 1.40 |
| 0.2 | 0.5 | 0.5 | no | 1.40, 1.40, 2.00 | 1 | 8 | 630 × 3003 | 1.40 | 1.40 |
| 0.2 | 0.5 | 0.5 | no | 1.40, 1.40, 2.00 | 1 | 10 | 2772 × 8008 | 1.40 | 1.40 |
| 0.2 | 0.5 | 0.5 | no | 1.40, 1.40, 2.00 | 2 | 8 | 6 × 3003 | 2.80 | 1.40 |
| 0.2 | 0.5 | 0.5 | no | 1.40, 1.40, 2.00 | 2 | 10 | 168 × 8008 | 2.80 | 1.40 |
| 0.2 | 0.5 | 0.5 | no | 1.40, 1.40, 2.00 | 2 | 12 | 1260 × 18564 | 2.80 | 1.40 |
| 0.2 | 0.5 | 0.5 | no | 1.40, 1.40, 2.00 | 3 | 12 | 10 × 18564 | 4.20 | 1.40 |
| 0.2 | 0.5 | 0.5 | no | 1.40, 1.40, 2.00 | 3 | 13 | 70 × 27132 | 4.20 | 1.40 |

Positive control (`--control`): with generators `{f₁, f₂, f₁ + x₁}` the ideal
contains `x₁`, so `ν₁` must equal `β = 0.2`; the code returns 0.2000 at `T = 6`
and `T = 8` (`joint_control.jsonl`). Cancellation is visible to the instrument.

**Reading.** In every cell `ν_s(T) = s·min_k deg_δ f_k` exactly, for
`s = 1, 2, 3` and every truncation tried. Cancellation between blocks never
lowered the least weight. In the demand region that is `ν_s/s ≥ 2.08`, against
the `< 1` a joint lattice needs: a factor-two margin. This is evidence, not a
proof: the true `ν_s` could be lower beyond `T = 13`, or for `s > 3`
(`ν_s` is subadditive, so small `s` does not bound the limit from below).

## 4. Run record

- `theorem`: `rustc -O --edition 2021 -D warnings examples/smallroot_optimal_reach.rs`,
  `smallroot_optimal_reach theorem > theorem_check.jsonl`, seed `0x7EE02026`,
  5.6 s wall (`theorem_time.txt`).
- joint: `rustc … examples/smallroot_joint_lattice.rs`,
  `smallroot_joint_lattice > joint_nu.jsonl` (seed `0x4A012026`, 4.7 s,
  `joint_time.txt`) and `smallroot_joint_lattice --control > joint_control.jsonl`.
- Host: shared 4-core, 15 GB container; counts only, no timing claims.
- `scripts/checkpoint.sh push` cannot upload from this container (no S3 token).
