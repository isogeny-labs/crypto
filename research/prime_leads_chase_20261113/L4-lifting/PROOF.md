# Lifting with growing rank: paper proof (L4-lifting, 2026-11-13)

Status: paper proof, **not machine-checked**, `[pcc:unverified]`. Cited theorems marked
"(cited, not re-read)" were recalled from memory in this lab; nothing here was checked against a
primary source in this session (no research-kb, no downloads).

## 0. Setting and hypotheses

- `p` prime, `E/F_p`, `G_q ⊂ E(F_p)` cyclic of prime order `q`, generator `P`, target `Q = xP`.
- **Lift data.** A number field `K` of degree `d` (`K = Q` is the classical case), a prime `𝔭 | p`
  of any residue degree `f`, an elliptic curve `E'/K` with good reduction at `𝔭`, and an injective
  homomorphism `ι : E(F_p) → Ẽ'(F_𝔭)` (from an `F_𝔭`-isomorphism `E ⊗ F_𝔭 ≅ Ẽ'`).
  Reduction `red : E'(K) → Ẽ'(F_𝔭)` is a group homomorphism (good reduction).
- **Heights.** `ĥ` = Néron–Tate height on `E'(K)`, in any fixed normalisation (absolute or relative;
  every statement below uses only ratios of heights of points on the same curve, which are
  normalisation-free). `‖P‖ := ĥ(P)^{1/2}` is a Euclidean norm on `E'(K) ⊗ R` (Néron–Tate:
  positive definite on `E'(K)/tors ⊗ R`; cited, standard).
  `λ(Λ)` := minimum of `ĥ` over non-torsion elements of a subgroup `Λ`.
- **H1 (torsion coprime to q).** `q ∤ #E'(K)_tors`. For `K = Q` this holds for `q > 7` (Mazur; cited);
  for degree `d` for `q > (3^{d/2}+1)^2` (Merel–Oesterlé; cited).
- **FM (fresh-mask model).** Each trial draws `k` independent uniform masks `(s_i,t_i) ∈ (Z/q)^2` and
  gives the algorithm `R_i = s_i P + t_i Q`. The algorithm may be computationally unbounded, may try
  any number of lifts per mask, and sees the masks only through the `R_i`. Every mask draw counts
  as a trial (including aborted ones).
- **Success (Model X, xedni).** The algorithm outputs a lift with `P_i ∈ E'(K)`, `red P_i = ι R_i`,
  and `m ∈ Z^k` with `Σ m_i P_i ∈ E'(K)_tors` and `Σ m_i t_i ≢ 0 (mod q)` (useful relation).
- `γ_s` = Hermite's constant (`γ_1..γ_8` known exactly; `γ_s ≤ 1 + s/4` for all `s`; cited).

## 1. Three lemmas

**Lemma 1 (exact 1/q).** For fixed `m ∈ Z^k`, `m ≢ 0 (mod q)`, `Pr_FM[Σ m_i R_i = 0] = 1/q`.
*Proof.* `R_i = u_i P` with `u_i = s_i + t_i x` i.i.d. uniform on `Z/q`. If `m_j ≢ 0`, `Σ m_i u_i` is
uniform. ∎

**Lemma 2 (relations reduce).** Under H1, if `Σ m_i P_i ∈ E'(K)_tors` then `Σ m_i R_i = 0` in `G_q`.
*Proof.* `ι(Σ m_i R_i) = red(Σ m_i P_i)` has order dividing `#E'(K)_tors` and lies in `ι(G_q)`, whose
order is `q`; coprime, so it is `0`; `ι` is injective. ∎
(Note: no torsion factor `T` appears in the count below; the closure note's pigeonhole carried `T`.)

**Lemma 3 (short relation: Cramer–Hadamard–Minkowski).** Let `P_1..P_{s+1} ∈ E'(K)` span a rank-`s`
subgroup `Λ` of `E'(K)/tors`. Then there is `m ∈ Z^{s+1} \ {0}` with `Σ m_i P_i ∈ tors` and, for every `j`,
`|m_j| ≤ Π_{i≠j} ‖P_i‖ / covol(Λ) ≤ (γ_s · H/λ(Λ))^{s/2}`, `H = max ĥ(P_i)`.
*Proof.* Fix a `Z`-basis `b_1..b_s` of `Λ`; write `P_i ≡ Σ_l a_{li} b_l`, `A ∈ Z^{s×(s+1)}` of rank `s`.
`m_j := (−1)^j det A_{−j}` (delete column `j`) is a nonzero integer vector with `A m = 0`
(generalised cross product), so `Σ m_i P_i ∈ tors`. `|det A_{−j}| = vol(P_i : i≠j)/covol(Λ)`;
Hadamard gives `vol ≤ Π_{i≠j}‖P_i‖ ≤ H^{s/2}`; Hermite/Minkowski gives
`λ(Λ) ≤ γ_s covol(Λ)^{2/s}`, i.e. `covol(Λ) ≥ (λ(Λ)/γ_s)^{s/2}`. ∎

## 2. Model X (xedni, lift per trial) with any rank

**Theorem X.** In FM, let `Succ(ρ₀, S)` be: success, and the output points satisfy
`ρ := max ĥ(P_i)/λ(⟨P_1..P_k⟩) ≤ ρ₀` and `rank⟨P_1..P_k⟩ ≤ S`. Assume H1 and `(γ_s ρ₀)^{s/2} < q`.
Then
`Pr[Succ(ρ₀,S)] ≤ |𝓜|/q`,  `|𝓜| = Σ_{s=0}^{min(S,k−1)} C(k,s+1) · ((2⌊(γ_s ρ₀)^{s/2}⌋+1)^{s+1} − 1)/2`.

*Proof.* On success the points are dependent, so `s := rank < k`. Choose a maximal independent
subset `I` (`|I| = s`) and `j ∉ I`. Lemma 3 on `I ∪ {j}` (whose span has rank `s` and minimum
`≥ λ(⟨P⟩)`) gives a nonzero `m'` supported on `s+1` indices with `|m'|_∞ ≤ (γ_s ρ₀)^{s/2}` and
`Σ m'_i P_i ∈ tors`; by Lemma 2, `Σ m'_i R_i = 0`. Hence `Succ ⊆ ⋃_{m' ∈ 𝓜} {Σ m'_i R_i = 0}`
(`±m'` give the same event). Each `m' ≢ 0 (mod q)`; Lemma 1 and the union bound finish. ∎

Remarks. (a) The bound depends on `(k, S, ρ₀)` only, not on the lift, the field degree `d`, the residue
degree `f`, the number of lifts tried, or the computing power. (b) It covers Model X with extra
**fixed** factor-base points (sub-question (iv), mixed form): if every lift also contains points
`P'_j` with `red P'_j = F_j` for `r'` fixed points `F_j ∈ G_q` of unknown log, and a useful relation
must involve some masked point, replace `k` by `k + r'` and restrict to `m` with masked part `≠ 0`;
Lemma 1 still applies (masked part uniform), so the same count with `k + r'` holds.
(c) Fixed `S` recovers the closure note's Theorem 2 with exponent `S(S+1)/2` in `ρ₀` and no `T`.

**Corollary X1 (threshold).** `log₂|𝓜| = (S(S+1)/2)·log₂(γ_S ρ₀) + O(S log k)`. The bound stops
excluding per-trial success `≥ q^{−1/2}` (rho's rate) once `log₂|𝓜| ≥ ½ log₂ q`, i.e. at

`S*_X(q, ρ₀) ≈ ( log₂ q / log₂(γ_S ρ₀) )^{1/2} = Θ( (log p / log(ρ₀ log p))^{1/2} )`.

Numerically (code/bounds.py, `k = S+1`): at 256-bit `q`, `S*_X = 10, 8, 6, 4` for
`ρ₀ = 1.5, 10, 100, 10⁴`; at 2048 bits `24, 19, 15, 12`.

**Unconditional ρ₀.** For lifts whose coefficients and point coordinates have at most `β` bits:
`ĥ(P_i) ≤ ½h(x(P_i)) + c(E') ≤ O(β)` (Silverman 1990 difference bound; cited, constants not re-read,
may carry `b₂`/`2*` terms), and `λ ≥ log|N Δ_min| /(10^15 d³ σ⁶ log²(104613 d σ²))` with Szpiro ratio
`σ ≤ log|Δ_min|/log 11` (Petsche 2006 Thm 2; cited via the closure-note checker). So
`ρ₀ ≤ poly(dβ)` and, for polynomial-size lifts, `S*_X = Θ((log p / log log p)^{1/2})`. At 256 bits the
cited constants give `ρ₀ ≈ 10^{35}` (code/bounds.py), i.e. the unconditional concrete bound
excludes nothing beyond `S = 2`; concrete exclusion needs a measured or certified `ρ`.

## 3. Model F (fixed lift: Silverman–Suzuki index calculus, other rational points)

**Theorem F.** Let `𝓛` be a set of at most `L` lifts (curves `E'/K` with `ι`), chosen independently of
the masks. Per trial (one mask, `R = sP + tQ`), the algorithm succeeds if it outputs some `E' ∈ 𝓛` and
`P' ∈ E'(K)` with `red P' = ι R` and `ĥ(P') ≤ B`. With `r = max rank`, `T = max #tors`,
`λ = min λ(E'(K))`, `Pr[success] ≤ L · T · (1 + 2√(B/λ))^r / q`.
*Proof.* `R` is uniform on `G_q` and independent of `𝓛`. Success forces `ι R ∈ red(S_B(E'))`,
`S_B = {ĥ ≤ B}`. Points of `S_B` modulo torsion are lattice points of norm `≤ √B` with pairwise
distance `≥ √λ`; disjoint balls of radius `√λ/2` inside the ball of radius `√B + √λ/2` give
`#S_B ≤ T(1 + 2√(B/λ))^r`. Union bound over `E' ∈ 𝓛`. ∎

**Corollary F1 (threshold).** Beating rho with poly-cost trials needs `L·T(1+2√ρ)^r ≥ q^{1/2}`,
`ρ = B/λ`, i.e. `r ≥ r*_F ≈ ½ log₂(q/L²T²) / log₂(1+2√ρ) = Θ(log p / log ρ)`:
`Θ(log p/log log p)` for `ρ = polylog`, `Θ(log p)` for `ρ = O(1)`. At 256 bits:
`r*_F = 70, 44, 29, 17` for `ρ = 1.5, 10, 100, 10⁴` (L = 1, T = 16).

**Remark F2 (the enumeration algorithm is BSGS).** The natural way to use Model F — enumerate
`S_B(E')`, reduce, and collide with masked targets — costs `≥ #S_B + q/#S_B ≥ 2√q` reductions. Beating
rho needs a *decoder* for `red` restricted to small-height points (find `P'` from `R` in time
`≪ #S_B`). No such decoder is known; nothing here proves one cannot exist.

**Remark F3 (compact representations, (iv)).** If the algorithm never writes coordinates but
manipulates `Σ n_j G_j` only through `red(Σ n_j G_j) = Σ n_j red(G_j)`, it is a generic algorithm on
`G_q` with extra known elements of unknown log, and Shoup's `Ω(√q)` applies (cited). Hybrids that
compute rational points of moderate height **and** do generic work mod `p` are not covered by a
proof here.

## 4. Can lifts of the required rank exist? Combining with rank results

| Hypothesis | Model X (needs `rank ≥ S*_X − 1` and a dependent `k`-tuple) | Model F (needs `rank ≥ r*_F`) |
|---|---|---|
| Unconditional | **Open.** No upper bound on ranks over `Q` is known; ranks `≥ 28` exist (Elkies 2006; `≥ 29`, Elkies–Klagsbrun 2024; cited) and families over `Q(t)` of rank `≥ 12..19` (Mestre, Nagao, Kihara, Elkies; cited) have polynomial-size specialisations. Bound is vacuous above `S*_X`. | **Open.** Same. |
| GRH (+ algebraic rank ≤ analytic rank, i.e. part of BSD) | Mestre 1986: `rank ≤ O(log N / log log N)` (cited). For `β`-bit lifts `log N ≤ log|Δ| = O(β)`, ceiling `O(β/log β) ≫ S*_X`. **Not excluded.** | Ceiling `O(β/log β)` vs `r*_F = Θ(log p/log ρ)`: same order when `β = Θ(log p)` and `ρ = polylog`, so exclusion would hinge on unknown constants; with larger `β` or `ρ` the ceiling exceeds `r*_F`. **Not excluded.** |
| Bounded rank over `Q` (`rank ≤ R_max`; implied by Park–Poonen–Voight–Wood 2016 heuristic: finitely many curves of rank `> 21`; cited) | `S ≤ R_max` constant ⇒ per-trial `≤ k^{R_max+1}(polylog)^{O(1)}/q` ⇒ **excluded asymptotically** for `K = Q`. | `#S_B ≤ T(1+2√ρ)^{R_max}` = polylog ⇒ **excluded asymptotically** for `K = Q`. At 256 bits rank-20 lifts (heuristically existent among lifts of a given `E` once coefficients reach `~p^{48}`, from the PPVW count `X^{(21−r)/24}`) pass the count only if `ρ ≳ 1.3·10³` (`20·log₂(1+2√ρ) ≥ 124`): **not excluded at 256 bits by counting.** |
| Number-field lifts of growing degree `d` (sub-question (iii)) | Theorem X is independent of `d` and `f`; only `ρ₀ ≤ poly(dβ)` enters. | Theorem F likewise. **But** bounded rank over `Q` says nothing over growing `K`: e.g. over multiquadratic `K` of degree `2^m`, `rank E'(K) = Σ_χ rank E'^χ(Q)` (sum over `2^m` quadratic twists), which under the parity conjecture is `≥ ~d/2`. So the asymptotic exclusion under bounded rank **does not extend to (iii)**; what stops this route is small-height points, not rank: generators of twists have heights that are heuristically polynomial in the conductor, i.e. exponential in `log p`, so `λ`, and hence `#S_B` for polynomial `B`, collapse. That is a heuristic, not a proof. |

## 5. What is and is not proved

Proved (paper): Theorems X, F, Lemmas 1–3, Corollaries X1, F1 (given cited inputs). These are
lift-independent upper bounds; they say nothing about lifts achieving them.
Not proved: any lower bound on `ρ` for actual xedni lifts (the concrete `S ≥ S*_X` regime);
any statement about decoders for `red` (Remark F2); hybrids (Remark F3); the regulator heuristic in §4.
The bounds are essentially the best possible *for arguments that use only heights*:
`Pr[∃ m ∈ 𝓜 : Σ m_i R_i = 0] ≈ min(1, |𝓜|/q)` for uniform `R_i`, so above `S*_X` an exclusion must use
arithmetic of lifts (how often they realise mod-`p` relations), which the toy (REPORT §3) measures.
