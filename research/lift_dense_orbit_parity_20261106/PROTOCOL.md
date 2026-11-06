# Protocol: orbit-parity criterion and the descent case (frozen before data)

2026-11-06. This is the second piece checking the claims from another
session (an unverified assertion). The first piece was
[`lift_dense_torsion_pairing_20261106`](../lift_dense_torsion_pairing_20261106/README.md).
This file is committed before any code or computation in this directory.
Changes after the freeze go in a dated amendment below.

## Setting

Let p be an odd prime, q = p, K = F_{q^n}, and let E be the curve
y² = f(x) = x³ + a₂x² + a₄x + a₆ over K, with distinct roots e₁, e₂, e₃
in an extension of K.

A **line** is L(x₀, u) = {x₀ + u·t : t ∈ F_q}, with x₀, u ∈ K and u ≠ 0.
Write χ_K for the quadratic character of K. The line is:

- **all-lift** if χ_K(f(x)) ≠ −1 for every x on it;
- **all-twist** if χ_K(f(x)) ≠ +1 for every x on it;
- **constant** if it is either one.

The **shifted cubic** is F(t) = f(x₀ + ut)/u³, whose roots are
t_i = (e_i − x₀)/u. Let σ be the q-power Frobenius.

## The claim under test

> **Criterion.** A line is all-lift or all-twist exactly when, for each
> σ-orbit O of the roots of the shifted cubic,
> n·r_O/s_O is even. Here r_O is the number of roots of F in O and s_O is
> the size of O.

> **Descent case.** Half the lifted points sit on the twist and are dead
> columns for the target. (The source's Exp 2 reports 128 of 276 dead,
> predicted with 0 mismatches. That experiment is not available here.)

**Reading used here.** It is fixed before data, and alternatives are noted.

- For t ∈ F_q, χ_K(f(x₀ + ut)) = χ_q(N_{K/F_q}(u³F(t))). The norm is
  c·g(t) with g(t) = ∏_{j<n} F^{σ^j}(t) ∈ F_q[t].
- A root ρ of g in orbit O has multiplicity n·r_O/s_O. So the criterion
  says g is a constant times a square.
- "⇒" is then exact algebra.
- "⇐" can fail by chance when q is small compared with deg g = 3n (the Weil
  bound).
- **Descent case** means the root set {t₁, t₂, t₃} is stable under σ^{n/2}
  (n even). Then F ∈ K₀[t], with K₀ = F_{q^{n/2}}, and the criterion holds
  automatically.
- This needs E to be K-isomorphic to a twist of a curve over K₀, which
  happens exactly when j(E) ∈ K₀.
- On an all-lift descent line, E ≅ C : Y² = F(t) over K, with C defined
  over K₀, and #E(K) = #C(K₀)·#C′(K₀), where C′ is the quadratic twist of
  C over K₀.
- A lifted point with Y ∈ K₀ lies in the image of C(K₀). Otherwise it lies
  in the image of C′(K₀).
- Let ℓ be the largest prime factor of #E(K). A point is **dead** if its
  projection to the ℓ-primary part is O, that is [#E/ℓ^v]P = O. If ℓ
  divides exactly one of #C(K₀) and #C′(K₀), the points on the other side
  are dead, and only those (2-torsion included).

## Instrument

This is a new std-only Rust crate, `orbit/`, with no shared code. It runs
inside one big field B = F_p[z]/(m), with deg m = 6n, so B contains K and
every root of a cubic over K. The irreducible m is found by a seeded search
and checked with Rabin's test.

- **K and K₀.** These are the fixed fields of the p^n- and p^{n/2}-power
  Frobenius in B. Random elements come from the relative traces
  Tr_{B/K} and Tr_{B/K₀}.
- **Roots.** e_i are found by equal-degree splitting in B[x]. A root found
  this way is checked by evaluation, and each cubic must have exactly
  3 distinct roots.
- **Orbits.** Each orbit is computed by iterating t ↦ t^p in B.
- **χ_K.** v^{(p^n−1)/2}.
- **Point counts.** #C(K₀) and #C′(K₀) are counted by enumerating K₀. The
  identity #E(K) = #C(K₀)#C′(K₀) is checked by [#E]P = O on 8 random
  points of E(K). Group-law points are lifted with a square root in K
  (Tonelli–Shanks).

## Cells and arms

The cells are (p, n) ∈ {13, 101} × {2, 3, 4}. The descent arms apply only
when n is even.

| arm | curves per cell | lines per curve | role |
|:--|--:|--:|:--|
| `generic-random` | 8, with a₂, a₄, a₆ uniform in K (j ∉ K₀ recorded) | 200 random (x₀, u) | Criterion "⇐": chance-constant lines |
| `descent-built` | 8 descent curves (below) | 40 descent lines (x₀ = β + αγ, u = αν with γ uniform in K₀, ν uniform in K₀^*) | Criterion "⇒", descent dead-point test |
| `descent-random` | the same 8 descent curves | 40 random lines | negative control on the same curves |

**Descent curves.** Draw a monic cubic f₀ over K₀ with distinct roots
e⁰_i, and α ∈ K^*, β ∈ K. Then E has roots e_i = αe⁰_i + β, so
f(x) = ∏(x − e_i) has coefficients in K and j(E) ∈ K₀. Curves with
j(E) ∈ K₀ for n = 2 are curves whose j-invariant lies in F_q.

All lines are recorded:

- the criterion predicate, with the full orbit data (r_O, s_O);
- the counts of squares, nonsquares and zeros of f along the line;
- whether the root set is σ^{n/2}-stable.

Each cell uses a fixed seed string with SplitMix64.

## Predictions (decision rules, coded in the summarizer)

**O1 (⇒, exact).** Every line that satisfies the criterion is constant, in
every cell and arm: 0 exceptions. Any exception falsifies the criterion's
"⇒" as read here.

**O2 (⇐ at q = 101).** In the q = 101 cells, no line that fails the
criterion is constant: 0 of all random lines. One or more is a
counterexample to "exactly when" at a q where Weil leaves no room for
chance. See the note below for which cells qualify.

**O2′ (⇐ at q = 13, report only).** The number of chance-constant lines
that fail the criterion at q = 13, and at q = 101 with n = 4. A nonzero
count here does not falsify anything: it shows where "exactly" needs q
large compared with (3n)².

> Note, fixed before data: the Weil bound only rules out chance constancy
> when q − 3n > (3n − 1)√q. That holds for (101, 2) and (101, 3). It does
> not hold for (101, 4), where 101 − 12 = 89 < 11·10.05, nor for any
> q = 13 cell. **O2 is therefore decided on (101, 2) and (101, 3) only.**
> The other cells are reported under O2′.

**O3 (descent, exact).**

- Every `descent-built` line satisfies the criterion and has a
  σ^{n/2}-stable root set.
- It is all-lift iff χ_K(u³) = +1, all-twist otherwise, with 0
  mismatches.

**O4 (dead points, exact).** On every all-lift `descent-built` line in a
curve where ℓ divides exactly one of #C(K₀) and #C′(K₀):

- the predicted dead set (the C′ side if ℓ | #C(K₀), the C side otherwise;
  2-torsion always dead) equals the observed dead set [#E/ℓ^v]P = O, with
  0 mismatches;
- pooled per cell, the dead fraction lies in [0.35, 0.65] ("half").

Curves where ℓ divides both, or neither (#E(K) a prime power of 2 or 3),
are reported and excluded from O4.

**O5 (multiplicity formula, exact).** For every line, ∏ over orbits of
(t − ρ)^{n·r_O/s_O} for ρ ∈ O has degree 3n. Equivalently,
Σ_O n·r_O/s_O·s_O = 3n, and each n·r_O/s_O is an integer. Any failure means
the reading above is wrong.

## What would change the lead

- If O1–O4 hold, the criterion and the descent case stand as stated at the
  scope tested: toy q ≤ 101, n ≤ 4.
- In that case, the descent case is a subfield-curve phenomenon (j ∈ K₀).
  Its lifted half is dead, so it is no lever for the target subgroup. That
  is consistent with the n = 2 finding of the first piece in char 2.
- O2 failing would contradict "exactly when".

## Controls, A/A, analyst

- The `descent-built` lines are the positive control for the "⇒" direction
  and for the dead-point detector.
- The `generic-random` and `descent-random` lines are the negative
  controls.
- A/A: two runs, with byte-identical rows required.
- The analyst is the producer. The verdicts come from coded rules.

## Amendments

(none)
