---
title: "Why a cover, an isogeny or a Jacobian chain does not move an elliptic-curve discrete log: the obstruction, stated once"
date: 2026-11-12
---

# Chained-transfer obstruction: proof note

Status: mathematical note, step 5 of `research/chained_prime_transfer_20261111/NEXT_STEPS_20261112.md`.
It gives an argument for or a citation of each step of the obstruction, marks assumptions, and ties
every step to the code or measurement in this repository that checked it.
It settles nothing about cross-characteristic maps (section 5).

Every number that comes from an experiment carries `[pcc:unverified]`: no pcc
claim has been signed for any of them. Theorems proved or cited here are not
experimental claims.

**Knowledge-base step.** `scripts/research-kb` was run and failed with
"no executable found; set RESEARCH_KB_BIN or put research-kb on PATH". No
`source_id` was obtained, and no novelty or "not tried" claim is made here.
The literature is cited from the author's knowledge; theorem and section
numbers marked (n) are from memory and should be checked against the source
before the note is quoted.

Notation. `p` is a prime, `k = F_p`, `k̄` an algebraic closure. `E/k` is an
elliptic curve with origin `O`; `G ∈ E(k)` has prime order `r`; `T = [x]G`.
`C/k` is a smooth projective geometrically irreducible curve of genus `g`;
`c: C → E` is a finite morphism over `k` of degree `d`. `J_C = Pic⁰(C)`.
`c^*` and `c_*` are pull-back and push-forward on divisors and, by
functoriality, on `Pic⁰`.

## 0. Summary of statuses

| # | Statement | Status |
|---|---|---|
| T1 | `c_* c^* = [d]` on `E` | proved (divisor level, standard) |
| T2 | `J_C ~ E × P`, `dim P = g−1`, `#J_C(F_p) = #E(F_p)·#P(F_p)` | cited (Poincaré reducibility, Tate) plus short proof |
| T3 | `#P(F_p) ∈ [(√p−1)^{2(g−1)}, (√p+1)^{2(g−1)}]` | proved from Weil's bound (cited) |
| T4 | `c^*` is injective on the prime-to-`d` part, so the DLP of order `r ∤ d` transfers | proved |
| T5 | Fibre rigidity: a reduced divisor of weight `≤ g` in the class of `c^*Q` is the fibre `c^*Q`, for all `Q` outside an explicit finite set (hyperelliptic `C`, `d ≤ g`, hypothesis H0) | proved |
| T6 | Same conclusion for non-hyperelliptic `C` when `g > d² − d + 1` and `c` is indecomposable | proved from Castelnuovo–Severi (cited) |
| T7 | Collision equivalence: every dependency among fibre relations is a collision `Q_i = ±Q_j` on `E` | proved from T5 and H2 |
| T8 | `φ2_* φ1^* = 0` for the two quotients of a bielliptic curve | proved |
| T9 | Isogenies, pairings and Weil restriction keep the characteristic; isogenies keep `#E` | cited |
| O1 | No efficient cross-characteristic DLP-preserving map exists | open, not claimed |
| O2 | Lifting to characteristic 0 beats `C₀/p` | open; the only bound is conditional |
| O3 | An IC that draws relations from all of `J_C(F_p)` rather than from the transferred subgroup | not covered by T5–T7; known cost is worse, but that is a complexity comparison, not a theorem here |

## 1. The cover decomposes the Jacobian; pull-back preserves the order-`r` DLP

**T1 (`c_* c^* = [d]`).** For every divisor `D` on `E`, `c_* c^* D = d·D`.

*Proof.* By linearity take `D = P` a closed point. `c^*P = Σ_{Q ↦ P} e_Q Q`
where `e_Q` is the ramification index of `Q` over `P`, and `c_* Q = f_Q P`
with `f_Q` the residue degree. So `c_* c^* P = (Σ_Q e_Q f_Q) P = d·P` by the
fundamental identity `Σ e_Q f_Q = d` for a finite morphism of smooth curves
(Hartshorne, *Algebraic Geometry*, II.6, proof of Prop. 6.9; Stichtenoth,
*Algebraic Function Fields and Codes*, Thm 3.1.11 (n); Silverman, *AEC*,
Prop. II.3.6 (n)). Both maps respect linear equivalence, so the identity holds
on `Pic⁰(E) = E`. ∎

Code check: `Bielliptic::push` applied to `Bielliptic::pull(Q)` equals `[2]Q`
and the cross composite is zero, on arbitrary points at `p ∈ {101, 211, 1009}`
(`bielliptic.rs`, test `push_pull_identities_on_arbitrary_divisors`);
`pushforward.rs`, test `pullback_pushes_to_double_and_cross_composite_vanishes`;
`pushforward_deg3.rs`, test `push_pull_is_multiplication_by_three`. The
degree-3 binary also checked the pull-back homomorphism for every `k ∈ 1..r`
in each of 24 instances (854 checks) [pcc:unverified].

**T2 (decomposition).** Write `ε = c^* c_* ∈ End(J_C)`. By T1,
`ε² = c^*(c_* c^*)c_* = d·ε`. Put `P = (ker c_*)⁰`, an abelian subvariety over
`k` (`c_*` is defined over `k`). Since `c_* c^* = [d]` is surjective, `c_*`
is surjective onto `E`, so `dim ker c_* = g − 1` and `dim P = g − 1`.
The addition map `μ: E × P → J_C`, `(e, x) ↦ c^* e + x`, is defined over `k`.
Its kernel is `{(e, −c^* e) : c^* e ∈ P}`. For such `e`, `c_* c^* e = d e = 0`,
so the kernel is finite, contained in `E[d]` in its first coordinate.
A finite-kernel homomorphism between abelian varieties of equal dimension is
surjective, so `μ` is an isogeny over `k`:

> `J_C ~ E × P` over `F_p`, with `ker μ ⊂ E[d] × P[d]`.

This is Poincaré's complete reducibility theorem applied to the abelian
subvariety `c^*(E)`, made explicit with the norm endomorphism `ε/d`
(Milne, *Abelian Varieties*, section on Poincaré's complete reducibility; Birkenhake–Lange,
*Complex Abelian Varieties*, Ch. 5 §3 and Ch. 12 on norm endomorphisms and Pryms (n)).
Isogenous abelian varieties over `F_p` have equal Frobenius characteristic
polynomials, hence equal point counts (Tate, *Endomorphisms of abelian
varieties over finite fields*, Invent. Math. 2 (1966), Thm 1). So

> `#J_C(F_p) = #E(F_p) · #P(F_p)`.

Code check: for genus 2 bielliptic curves the library computes
`jacobian_order() = e1.order() * e2.order()` (`bielliptic.rs`, same test, last
assertion), consistent with `P ~ E2` [pcc:unverified]. For genus 2, `J_C ~ E1 × E2`
for a bielliptic curve is classical (Kani–Rosen, *Idempotent relations and
factors of Jacobians*, Math. Ann. 284 (1989)).

**T3 (size of `P`).** Let `α_1, …, α_{2(g−1)}` be the Frobenius eigenvalues of
`P`. Weil's Riemann hypothesis for abelian varieties gives `|α_i| = √p`
(Weil 1948; Milne, *Abelian Varieties*, on the Riemann hypothesis; for `E` it
is Hasse's bound). Then `#P(F_p) = Π(1 − α_i)` and
`(√p − 1)^{2(g−1)} ≤ #P(F_p) ≤ (√p + 1)^{2(g−1)}`, that is

> `#P(F_p) = p^{g−1} (1 + O_g(p^{−1/2}))`. ∎

Consequence: `#J_C(F_p) ≈ p^g`, of which only the factor `#E(F_p) ≈ p` carries
the instance (see T4). A DLP of order `r ≤ #E(F_p)` has generic cost `Θ(√r)`;
embedding it in a group of order `≈ p^g` does not shrink it.

**T4 (pull-back preserves the order-`r` DLP).** Suppose `r ∤ d`. Then `c^*`
is injective on `E[r]`: if `c^* Q = 0` with `Q ∈ E[r]`, then
`dQ = c_* c^* Q = 0`, and `gcd(r, d) = 1` gives `Q = 0`. So `D := c^*(G − O)`
has order `r` in `J_C(F_p)`, and `c^*([x]G − O) = [x]D`: the map
`⟨G⟩ → ⟨D⟩` is a group isomorphism computable by one fibre computation
(a degree-`d` polynomial factoring or root-finding over `F_p` plus Cantor
reduction). In general `ker c^* ⊂ E[d]`, and `ker c^* = 0` iff `c` does not
factor through a nontrivial isogeny onto `E` (the classical criterion for
complex curves, Birkenhake–Lange Ch. 11–12 (n); it is not needed below because
`r ∤ d`). ∎

Code check: `Hop::pullback` runs `check_dlp` on construction (order of the image
equals `r`, `h([x]P) = [x]h(P)` for sampled `x`) and rejects the hop otherwise
(`chain.rs`). The hop's comment records the same argument for odd `r`.

## 2. The transferred subgroup lies in `c^*(E)`, and its relations are fibres

The transferred subgroup is `S = c^*(E(F_p)) ⊂ J_C(F_p)`. An index-calculus
(IC) relation for the instance `(D, T = [x]D)` is an effective rational
divisor `Σ m_P P` supported on a factor base of places such that
`[Σ m_P P − w ∞_C] = [α]D + [β]T ∈ S`, with `w = Σ m_P` and `∞_C` a rational
reference point. The measurements ask whether such relations carry anything
beyond the point `Q = [α]G + [β]T_E ∈ E(F_p)`.

Hypotheses.

* **H0** (reference). `c^*(O) ~ d·∞_C`. In the degree-3 experiment
  `c^*(O) = 3∞_C` as divisors (`c` is totally ramified over `O`). In the
  bielliptic experiment `c^*(O) = ∞⁺ + ∞⁻`, two hyperelliptic conjugates, which is
  linearly equivalent to `2·(Weierstrass point)`, the reference of the quintic
  model (`bielliptic.rs`, module comment).
* **H1** (hyperelliptic). `C: y² = f(x)` with `∞_C` a rational Weierstrass
  point at infinity of the imaginary model and `d ≤ g`. This holds in both
  experiments (genus 2 with `d = 2`, genus 4 with `d = 3`).
* **H2** (sign). `c ∘ ι_h = [−1] ∘ c`, where `ι_h` is the hyperelliptic
  involution. This holds for `φ1` and for `c(x, y) = (h(x), y)`.

A divisor `Σ_{i≤w} P_i` of weight `w ≤ g` is *reduced* if it avoids `∞_C`,
contains no pair `P, ι_h P` (counting a Weierstrass point at most once).

**Mumford–Cantor uniqueness** (Mumford, *Tata Lectures on Theta II*, IIIa §2;
Cantor, Math. Comp. 48 (1987)). For hyperelliptic `C` with rational
Weierstrass point `∞_C`, every class in `J_C(k)` is `[Σ P_i − w ∞_C]` for a
unique reduced rational divisor `Σ P_i` of weight `w ≤ g`.

**T5 (fibre rigidity).** Assume H0 and H1. Let `Z = {P ∈ C(k̄): c(ι_h P) = c(P)}`,
let `Ram` be the ramification points of `c`, and
`N = c(Z) ∪ {c(∞_C)} ∪ c(Ram ∩ Weier)` where `Weier` is the set of Weierstrass points.
Then `N` is finite, and for every `Q ∈ E(k) \ N` the reduced representative of
the class `c^*[Q − O]` is the fibre `c^*Q` itself, of weight exactly `d`.
Consequently, every IC relation of weight `≤ g` whose class is `c^*[Q − O]`
is the full fibre `c^*Q`, with each place `P` appearing with its
ramification multiplicity `e_P ≤ d`.

*Proof.* Finiteness. `Ram` and `Weier` are finite. Suppose `Z` were infinite.
Then `c ∘ ι_h` and `c` agree on infinitely many points, hence everywhere, so
`c` is constant on `ι_h`-orbits and factors through `C/ι_h = P¹`, which is
impossible, since every morphism `P¹ → E` is constant. So `Z` is finite
(it is the zero set of the nonconstant morphism `P ↦ c(P) − c(ι_h P)` to `E`, which
is nonconstant: were it a constant `e_0`, applying `ι_h` gives `2e_0 = 0`, and
`π: E → E/⟨e_0⟩` would satisfy `π c ι_h = π c`, so `π c` would factor
through `P¹` and be constant, which is impossible).

Reducedness. Let `Q ∉ N`. The effective divisor `c^*Q` has degree `d ≤ g`. By
H0, `[c^*Q − d∞_C] = [c^*Q − c^*O] = c^*[Q − O]`. The divisor `c^*Q`
avoids `∞_C` (else `Q = c(∞_C) ∈ N`), contains no pair `P, ι_h P` (else
`c(P) = c(ι_h P) = Q`, so `Q ∈ c(Z)`), and has no Weierstrass point with
multiplicity `≥ 2` (else `c` ramifies there). So it is reduced, and by
Mumford–Cantor uniqueness it is *the* reduced representative of the class.
A relation of weight `≤ g` with that class is a reduced divisor (the
decomposition of the reduced representative over the factor base), hence equals `c^*Q`. ∎

*Where it can fail.* (a) H0 fails: the class of the relation is then
`c^*[Q − O]` shifted by `κ = [c^*O − d∞_C] ≠ 0`, and the divisor
`c^*Q + (d∞_C − c^*O)` has no reason to be a fibre. (b) `Q ∈ N`: finitely many
points, among them `c(∞_C)`; for such `Q` the reduced representative is
shorter than `d`, and the relation is not a full fibre. (c) Weight `> g`: a
relation with `m > g` places has a linear system of dimension `≥ m − g`, with
many non-fibre members; T5 does not constrain it, and the standard IC does not
use such relations. (d) `d > g` or non-hyperelliptic `C`: use T6. (e) The
relation is drawn from outside `S`: see O3.

**T6 (non-hyperelliptic version via Castelnuovo–Severi).** Let `C` be any
curve, `c: C → E` of degree `d` over `k̄`, and suppose

* `g > d² − d + 1`, and
* `c` is indecomposable: it does not factor as `C → C' → E` with
  `C' ≠ C` and `C' ≠ E` (in the sense of both degrees `> 1`).

Then `ℓ(c^*Q) = 1` for every `Q ∈ E(k̄)`; so any effective divisor `D`
with `D ~ c^*Q` equals `c^*Q`.

*Proof.* Suppose `ℓ(c^*Q) ≥ 2`. Two independent sections give a nonconstant
`f: C → P¹` of degree `e ≤ d`. Let `K_c = k̄(E)` and `K_f = k̄(f)`, subfields of
`k̄(C)` of indices `d` and `e`. If their compositum is `k̄(C)`, the
Castelnuovo–Severi inequality (Stichtenoth, Thm 3.11.3 (n)) gives
`g ≤ d·1 + e·0 + (d−1)(e−1) ≤ d + (d−1)² = d² − d + 1`, contradicting the
hypothesis. Otherwise the compositum is `k̄(C'')` for a curve `C''` with
`C → C''` of degree `> 1`, and `c` factors through `C''`. By indecomposability
`C'' ≅ E`, so `f = u ∘ c` with `u: E → P¹` nonconstant of degree `≥ 2`, and
`e = d·deg u ≥ 2d > d`, a contradiction. ∎

T6 does not apply to either experiment (`g = 2, d = 2` needs `g > 3`;
`g = 4, d = 3` needs `g > 7`). It is the cleanest criterion for the
non-hyperelliptic setting and shows the failure mode is a pencil, not an
arithmetic accident.

Two sharper facts for the genus-2 bielliptic case, `d = 2`, `g = 2`: a degree-2
effective divisor on a genus-2 curve moves iff it is canonical, so
`ℓ(c^*Q) = 2` iff `c^*Q ~ K_C`. Here `c^*O = ∞⁺ + ∞⁻ ~ K_C`, so `Q = O` is
a point with a moving fibre, and the identity class is not a relation
anyway. If `c^*Q ~ K_C` and `c^*Q' ~ K_C` then `Q − Q' ∈ ker c^* ⊂ E[2]`;
the kernel meets `E × 0` trivially in Kani's description of `J_C` as the
quotient of `E1 × E2` by the graph of an isomorphism `E1[2] ≅ E2[2]`, so
`Q = O` is the only moving fibre. (The kernel claim is cited from Kani–Rosen
and the description of split genus-2 Jacobians, Howe–Leprévost–Poonen,
Forum Math. 12 (2000) (n); it is not used by T5.)

**T7 (collision equivalence).** Assume H0, H1, H2, and `r > d` prime. Let the
IC collect rows `i = 1, …, n` with `Q_i = [α_i]G + [β_i]T_E ∉ N`, each row
being the fibre `c^*Q_i` written on the factor-base columns, where a place
`P` and `ι_h P` are identified up to sign (as the code identifies places with
`y ≤ (p−1)/2`). Then:

1. The columns supporting the row of `Q_i` are exactly the places of
   `c^{-1}(Q_i)` and their `ι_h`-images, which by H2 are the places of
   `c^{-1}(−Q_i)`. Rows with `Q_i ≠ ±Q_j` have disjoint column supports.
2. Every nonzero entry of a row is a ramification multiplicity in `1..d`, a
   unit mod `r`. So the rows with distinct classes `±Q` are linearly
   independent over `Z/r`, and rows with `Q_j = ±Q_i` are `±1` multiples of
   one another.
3. Every dependency `Σ λ_i row_i = 0` over `Z/r` is therefore a sum of
   pairwise dependencies between rows with `Q_j = ε Q_i`, `ε = ±1`. Such a pair
   gives `(α_i − εα_j) + (β_i − εβ_j) x ≡ 0 (mod r)`, which is exactly the
   statement `Q_i = ε Q_j` on `E`.
4. Hence the first dependency, and any logarithm the IC extracts, requires a
   repeated `±Q` among the accepted trials. The accepted trials are a
   subsequence of the unfiltered stream `Q_1, Q_2, …` on `E`, so on the same
   random stream the first repeat of an unfiltered birthday search on `E`
   occurs no later. ∎

*Reading.* Under these hypotheses, index calculus restricted to the transferred
subgroup, over any factor base of places of `C`, is a birthday collision search
on `E` with an acceptance filter ("the fibre `c^{-1}(Q)` factors over the
allowed places") that can only discard trials. The same holds if the factor
base contains places of degree `> 1` (large primes): a fibre that factors into
places of several degrees is still the single fibre over a single `Q ∈ E(F_p)`.

Measurements that tested T5 and T7 [pcc:unverified]:

* `research/cover_pushforward_20261112/PUSHFORWARD.md` (degree 2, genus 2, 7
  primes 211 to 12841, 3 seeds, plus a 15-seed run): 37,866 of 37,866
  collected relations are fibre type; 189,330 of 189,330 in the 15-seed run;
  0 invalid pushed relations on `E1` or `E2`; columns used by the genus-2 arm
  are exactly twice those of the native `E1` arm (each place `P` and `ι P`
  over the same `X`); the control arm A6 (birthday search with no filter)
  finds its first collision at or before the cover arm in every cell.
  Code: `pushforward.rs`, `collect_jc`, `push_check`, `solve_rows`.
* `research/cover_pushforward_deg3_20261112/PUSHFORWARD_DEG3.md` (degree 3,
  genus 4, 8 primes 53 to 251, 24 cells, 360 seed-cells): 28,815 of 28,815
  relations are a single full fibre (0 unions, 0 other), including 1,317 rows
  with a repeated root; in 159,508 of 159,508 trials the Cantor-reduced class
  equals the Mumford pair of `Q` computed on `E` alone; the J(C) arm and the
  native `E` arm have identical counted operations and recover the same log in
  360 of 360 seed-cells. Code: `pushforward_deg3.rs`.

These counts are consistent with T5 and T7 but are not their justification; T5 and T7
have their own arguments above, and the counts check that the code implements the setting of the
proof (hypotheses H0 to H2, `d ≤ g`, weight `≤ g`).

**Consequence stated once.** For the instance transferred along `c`, an IC
that takes its relations in `S` and in reduced weight `≤ g` cannot do better
than the matched collision search on `E`, up to the discarded trials. This
does not exclude other relation sources (O3) nor any method that is not an IC.

## 3. The complement receives nothing: `φ2_* φ1^* = 0`

Setting: `C: y² = F(x²)`, `σ(x, y) = (−x, y)`, `φ1(x, y) = (x², y)` to
`E1: Y² = F(X)`, and `φ2(x, y) = (1/x², y/x³)` to `E2: Y² = X³F(1/X)`.
Then `φ1 ∘ σ = φ1` and `φ2 ∘ σ = [−1] ∘ φ2`, the latter because
`φ2(−x, y) = (1/x², −y/x³)` is the negation on `E2`.

**T8.** `φ2_* φ1^* = 0` as a homomorphism `E1 → E2`.

*Proof.* Put `ψ = φ2_* φ1^*`. From `φ1 σ = φ1` we get `σ_* φ1^* = φ1^*`
(pull-back by `φ1` is `σ`-invariant, and `σ` is an involution, so
`σ^* = σ_*` on `J_C`). From `φ2 σ = [−1] φ2` we get
`φ2_* σ_* = [−1] φ2_*`. Therefore
`ψ = φ2_* φ1^* = φ2_* σ_* φ1^* = −φ2_* φ1^* = −ψ`,
so `2ψ = 0` and `ψ(E1) ⊂ E2[2]`. The image of a connected variety under a
morphism is connected; a finite connected set is a point; so `ψ = 0`. ∎

Reading. `S ⊂ φ1^*(E1)` is invisible to `E2`: any relation pushed to `E2` is
`Σ = O` whatever `(α, β)` are, so nothing about the target is encoded there. For a
general cover `c`, `c_*` kills `P` by definition, and `c_* c^* = [d]` shows `c_*`
does not kill `c^*(E)`, so the same conclusion holds for `P` as a whole: the
target-carrying part is the `E`-isotypic part.

Code and measurement: `bielliptic.rs` asserts `b.push(other, &d) == None`
(identity) for the pull-back of generic points of both quotients, on
`p ∈ {101, 211, 1009}`. `PUSHFORWARD.md`: `φ2_*(G)` and `φ2_*(T)` are the identity in
37,866 of 37,866 rows and 0 pushed `E2` rows carry the target [pcc:unverified].

## 4. Isogenies, pairings and Weil restriction keep the characteristic

* **Isogenies.** An `F_p`-isogeny `φ: E → E'` has `#E(F_p) = #E'(F_p)` and
  conversely two curves over `F_p` with the same number of points are
  `F_p`-isogenous (Tate, Invent. Math. 2 (1966), Thm 1; Silverman, *AEC*, Ch. V
  and Exercise 5.4 (n)). The order `r` and the embedding degree
  `k = min{k : r | p^k − 1}` depend only on `(p, r)`, so an isogeny keeps both.
  An isogeny transfers the DLP exactly when `ker φ ∩ ⟨G⟩ = 0`, in particular
  when `r ∤ deg φ`.
* **Precisely what the Tate statement gives.** An algebraic group
  homomorphism `E → E'` over `F_p` that is nonzero is an isogeny, so it can
  exist only if `#E = #E'`. That direction is established by the cited theorem. The converse gives an
  isogeny, not an isogeny of degree prime to `r`; I do not claim that one
  exists in every isogeny class. In particular, "DLP-preserving maps exist
  iff `#E = #E'`" (as the wording of `ATLAS.md` and the earlier notes reads)
  is true only for algebraic homomorphisms, and its "if" direction is not
  shown by Tate's theorem alone. Set-theoretic isomorphisms of cyclic groups
  of order `r` always exist and are not efficiently computable.
* **Into higher genus.** If `ψ: E → A` is an `F_p`-homomorphism with finite
  kernel, `ψ(E)` is an elliptic curve isogenous to `E`, so the image of `⟨G⟩`
  sits in a group of order `#E(F_p)` (Tate for abelian varieties). This is the
  reason the transferred subgroup lives in the `E`-factor of T2.
* **Pairings.** The Menezes–Okamoto–Vanstone reduction (IEEE Trans. IT 39
  (1993)) and the Frey–Rück reduction (Math. Comp. 62 (1994)) embed `⟨G⟩` into
  `μ_r ⊂ F_{p^k}^*` via the Weil or Tate pairing. The target is a multiplicative
  group of a field of characteristic `p` and of size `≈ p^k`. For a random curve
  `k` is of the order of `r` (Balasubramanian–Koblitz, J. Cryptology 11
  (1998)), so the reduction is useful only for small `k`.
* **Weil restriction and descent.** Weil restriction `Res_{F_{p^n}/F_p}`
  (Frey 1998; Gaudry–Hess–Smart, J. Cryptology 15 (2002)) changes the
  base field but not its characteristic. A curve over the prime field `F_p`
  has no proper subfield to descend to, so the construction is vacuous there. Descent
  for extension fields is a separate question and its known limits are surveyed
  by Diem, Compositio Math. 147 (2011) (n).
* **Smart's anomalous attack** (J. Cryptology 12 (1999)) works by lifting to
  `Q_p` and is available only when `#E(F_p) = p`; it does not apply to a
  curve of other trace. It shows that the characteristic-0 route is real
  but special.

Code and measurement: `chain.rs` (`Hop::isogeny`) checks order and the
homomorphism property for each Vélu isogeny hop and rejects it otherwise;
the reachable-set census is in `atlas.rs` and
`research/chained_prime_transfer_20261111/ATLAS.md` [pcc:unverified].

## 5. What is not established

* **O1.** That no efficient cross-characteristic map from `⟨G⟩` to a group in
  which the DLP is easier exists is open. T1 to T9 only show that the maps
  built here (covers, isogenies, pairings, Weil restriction) stay in
  characteristic `p` and carry the instance in the `E`-factor. They are not a
  barrier theorem against an unknown map. Nothing in this repository
  constructs a cross-characteristic map.
* **O2.** The only known route out of characteristic `p`, lifting to
  characteristic 0 (the xedni calculus, Silverman 1999), has a success bound
  of about `C₀/p` per trial in Jacobson–Koblitz–Silverman–Stein–Teske,
  *Analysis of the Xedni Calculus Attack*, Designs, Codes and Cryptography 20
  (2000), 41–64, Thm 4.1, which begins "under certain plausible assumptions" and
  depends on a height-versus-discriminant comparison and Lang's height
  conjecture. It is conditional. The repository's Lean audit
  (`research/jksst_xedni_audit_20261101/`, `PROOF_MAP.md`) proves the
  conditional theorem from its stated hypotheses and refutes several
  overbroad variants; it does not discharge Lang's conjecture, and
  it does not show the algorithm fails.
* **O3.** T5 to T7 apply to relations in the transferred subgroup `S`. An IC
  in the style of Gaudry (Eurocrypt 2000) draws relations from random elements
  of the whole `J_C(F_p)` and computes logarithms of factor-base elements in
  the full group, of order `≈ p^g` (T3). Its known cost is `Õ(p^{2−2/g})`
  (Gaudry–Thomé–Thériault–Diem, Math. Comp. 76 (2007)), against `√r ≤ √p`
  for rho on the target subgroup: worse for every `g ≥ 2`. That is a comparison of
  best-known algorithms, not a lower bound, and the `tower_large` and
  `hyper64.rs` notes measure it only at toy size.
* **Measured results are toy-size.** Every experiment cited has `p` at most
  12,841 (degree 2) or 251 (degree 3), a handful of covers per prime, and no
  signed pcc claim: they are `[pcc:unverified]`, and none is a row in the
  validation gate. They check that the code implements the hypotheses of the
  proofs; they do not extend the proofs to larger `p`.
* T5 to T7 do not cover non-hyperelliptic covers with `g ≤ d² − d + 1`
  (T6 is silent), relations of weight `> g`, references with `κ ≠ 0`, or
  the finite exceptional set `N`.

## 6. Claims in earlier notes that this note narrows

1. `NEXT_STEPS_20261112.md`, step 5: "effective divisors of degree `deg c` in
   `c^*(E)` are fibres". Established only under H0 plus either T5 (hyperelliptic
   `C`, `d ≤ g`, weight `≤ g`, `Q ∉ N`) or T6 (`g > d² − d + 1`,
   indecomposable `c`). For non-hyperelliptic curves of small genus and for
   `Q = O` in the bielliptic case it can fail.
2. `ATLAS.md`: "a DLP-preserving homomorphism `E → E'` exists iff `E` and `E'`
   are isogenous, and over `F_p` iff `#E = #E'`". The "only if" is shown above. The "if"
   gives an isogeny whose degree may be divisible by `r` (section 4).
3. `PUSHFORWARD.md`: "the reduced class of `[α]G + [β]T` is a fibre" holds for
   `Q ∉ N`; the finite set `N` (fibres through `∞_C`, through a conjugate
   pair, or through a ramified Weierstrass point) is excluded, and the
   `Q = O` class is the identity. This matches `Bielliptic::fiber_generic`
   in the code, which excludes the ramification locus.
4. "IC on the transferred subgroup is a collision search on `E`" is a statement
   about relations taken in `S` (O3). It is not a statement about IC on all of
   `J_C`.
5. `c^*` is injective only on the prime-to-`d` part in general (T4); in the
   bielliptic experiment it is injective for the odd `r` used.

## 7. Lean candidates

Search results. `/home/user/crypto` has Lean projects, e.g.
`research/jksst_xedni_audit_20261101/lean/lakefile.toml` (toolchain
`leanprover/lean4:v4.34.1`),
`research/prime_square_kernel_20261106/lean/lakefile.toml`, and the
`research/koblitz_modular_ic_20261018/rounds/*/lean/` projects.
`lake` and `lean` are not on `PATH` in this container, and no built Mathlib is
present (only a partial source cache under
`research/pencil_lifecycle_20261030/.../cache-sources/Mathlib`, without
`RamificationInertia`). `/home/user` holds only `crypto` and `cryptanalysis`;
`crypto-lean` is not available locally. So **the statements below are text
only and nothing here was type-checked.**

The divisor-level core of T1 can be stated without algebraic geometry beyond
the fundamental identity. Candidate formulation (abstract, to be instantiated
by a Dedekind-domain extension, where Mathlib has the fundamental identity
`Ideal.sum_ramification_inertia` (n)):

```lean
-- c_* c^* = [deg c] at the level of divisors (free abelian groups on places).
-- Hypothesis `fund`: for every place P of E, Σ_{Q | P} e(Q|P) f(Q|P) = d.
theorem pushforward_pullback_eq_smul
    {Pl_C Pl_E : Type*} [DecidableEq Pl_C] [DecidableEq Pl_E]
    (over : Pl_C → Pl_E) (e f : Pl_C → ℕ) (d : ℕ)
    (fund : ∀ P : Pl_E, ∑ Q ∈ (Finset.univ.filter fun Q => over Q = P), e Q * f Q = d) :
    ∀ D : Pl_E →₀ ℤ, pushDiv over f (pullDiv over e D) = d • D := by
  sorry
```

where `pullDiv over e D = Σ_P D P • Σ_{Q | P} e Q • [Q]` and
`pushDiv over f Q' = Σ f Q • [over Q]`. The statement is a finite sum
interchange once `fund` is a hypothesis; the geometric content (that `fund`
holds for a finite morphism of smooth curves) is the cited fundamental identity.
A second formulation, on class groups, would add that both maps respect
principal divisors; Mathlib lacks function fields of curves with `Pic`.

The bielliptic cross-map vanishing (T8) in abstract form: for abelian groups
`A` (standing for `J_C`), `B1`, `B2`, homomorphisms `a: B1 → A`, `b: A → B2`,
and an automorphism `s: A → A` with `s ∘ a = a` and `b ∘ s = −b`, then
`b ∘ a = −(b ∘ a)`, that is `2 • (b ∘ a) = 0`:

```lean
theorem cross_map_two_torsion
    {A B1 B2 : Type*} [AddCommGroup A] [AddCommGroup B1] [AddCommGroup B2]
    (a : B1 →+ A) (b : A →+ B2) (s : A →+ A)
    (h1 : ∀ x, s (a x) = a x) (h2 : ∀ y, b (s y) = - b y) :
    ∀ x, b (a x) + b (a x) = 0 := by
  intro x; have := h2 (a x); rw [h1] at this; ...  -- b (a x) = - b (a x)
  sorry
```

The upgrade from `2ψ = 0` to `ψ = 0` uses connectedness of `E1` (a continuous
image of a connected variety is connected), which has no existing Lean
formulation for abelian varieties; the algebraic statement above is the
"Lean-ready" part. The T4 step (`c^*` is injective on `E[r]` for `gcd(r, d) = 1`)
is a one-line group-theory lemma from `c_* c^* = [d]`:
`∀ x, r • x = 0 → c_pull x = 0 → d • x = 0 → x = 0` for coprime `r, d`.

## 8. Files and verification

* This note: `research/chained_transfer_proof_20261112/PROOF.md`;
  rendered copy `PROOF.pdf` in the same directory.
* Code referred to: `src/cryptanalysis/chained_transfer/bielliptic.rs`,
  `chain.rs`, `pushforward.rs`, `pushforward_deg3.rs`, `atlas.rs`.
* Not run: no experiment was run for this note; no test was run. The code
  checks cited are the repository's existing tests and the recorded
  measurement notes.
