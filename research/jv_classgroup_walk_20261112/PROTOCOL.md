# Protocol: a class-group isogeny walk into the JV weak class (kernel-polynomial odd isogenies)

Frozen before any measurement run of this study. Smoke runs and correctness unit tests are disclosed in the
README and are not data. Every result is an **unverified assertion** `[pcc:unverified]`. Scope: the cubic
norm-one branch of the JV weak class over F_{p⁶} (`jv_cover`'s route); the quadratic-h branch is out of scope.

## Problem

`research/jv_weak_coverage_20261111` found that the implemented walk reaches a weak curve from only ~1% of
curves at p = 1009 (3 of 60 admitted classes) and 0 of 60 at p = 1511, 1823. The cause: the odd-degree jumps
(`walk_to_weak_record`) used `roots_q3(ψ_ℓ)`, i.e. only ℓ-isogenies whose kernel contains a point whose
x-coordinate is **rational** over F_{q³}. A cyclic ℓ-subgroup that is Galois-stable but has no rational point
x-coordinate is a rational ℓ-isogeny the walk could not take. This study implements those isogenies and
re-measures coverage.

## Construction (to implement and certify before any coverage run)

For E: y² = x³ + a₂x² + a₄x over F_{q³} = F_{p⁶} (the walk's translated full-2-torsion model), and ℓ ∈ {3,5,7}:

1. **Kernel polynomials.** Factor ψ_ℓ (degree (ℓ²−1)/2) over F_{q³}. A monic degree d = (ℓ−1)/2 factor
   h | ψ_ℓ is the kernel polynomial of a cyclic ℓ-isogeny iff the multiplication-by-2 x-map
   X₂(x) = (4xF − ψ₃)/(4F) permutes its roots, tested entirely in F_{q³}[x] by
   `h(x) | Σ_j c_j N^j D^{d−j}` with N = 4xF − ψ₃, D = 4F, h = Σ c_j x^j.
2. **Vélu through h (symmetric functions, no individual roots).** With F' the derivative of F,
   R₁ = (F'·h') mod h and P = (4F·h') mod h, the x-map is
   X(x) = [x·h² + 2R₁h + (P h' − P' h)] / h²  =  A(x) / h(x)².
3. **Self-certification.** The construction is accepted for a given h only if deg A = ℓ and gcd(A, h) = 1
   (the map has degree exactly ℓ with denominator h², so the kernel has order ℓ). The image curve's three
   full-2-torsion x-coordinates are X(0), X(e₁−e₀)+…, i.e. the images of the model's 2-torsion, which stay
   in F_{q³}.

## Correctness gates (must all pass before any coverage run; reported in the README)

- **C1 (agreement).** On split kernels (h with rational roots), symmetric-function Vélu equals the existing
  per-root Vélu of `ell_targets_with_kernels`, image curve identical, ≥ 200 cases across p ∈ {37, 1009}.
- **C2 (real-point image).** For ≥ 500 random affine points P on E and every certified kernel poly, φ(P)'s
  x-coordinate is the x of a real point on the image curve (its 2-torsion RHS is a square), p ∈ {37, 1009, 1511}.
- **C3 (degree).** Every certified isogeny has deg A = ℓ and gcd(A, h) = 1; every rejected h fails X₂-closure.
- **C4 (dual).** φ̂ ∘ φ = [ℓ] on ≥ 200 random points (compose the isogeny with a certified dual and compare to
  scalar ℓ on the curve), p ∈ {37, 1009}. If a certified dual is not implemented, C4 is replaced by C4′:
  the image curve is ℓ-isogenous by the modular polynomial Φ_ℓ(j(E), j(E')) = 0, ≥ 100 cases.
- **C5 (count).** The number of certified rational ℓ-isogenies from a curve equals the number predicted by the
  split type of t²−4q³ mod ℓ (0, 1, or 2 for inert/ramified/split), on ≥ 100 curves per ℓ.

## Measurement (only after C1–C5 pass)

Re-run `research/jv_weak_coverage_20261111`'s W measurement with the class-group walk in place of the
rational-root walk: 60 admitted full-2-torsion curves per prime, p ∈ {37, 53, 1009, 1511, 1823}, cap 10,000
curves, jumps 3,5,7. Report, per prime:

- **W′**: fraction of admitted classes from which the class-group walk reaches a weak curve, Wilson 95%.
- The operational coverage P_U(admitted) × W′, against the rational-root walk's W.
- The cost (F_p multiplications) of a successful walk, against the matched rho and the `jv_walk_charge` figures.
- Walk path lengths and the share of odd kernel-polynomial steps among all steps.

## Decision rule (fixed now)

- **"Class-group walk reaches the class"** if W′ > W at p = 1009 with non-overlapping Wilson intervals, and
  W′ ≥ 0.5 at p = 1511 or 1823.
- **"No material gain"** if W′'s interval overlaps W's at every prime.
- Otherwise **"partial"**, reported per prime.

## Predictions (before the runs)

- C1–C5 all pass: 0.8 (the construction is standard Kohel/Vélu; the risk is an implementation bug caught by
  the gates, not the mathematics).
- W′ ≥ 0.9 at every prime: 0.6. A Galois-stable cyclic ℓ-subgroup exists whenever ℓ splits or ramifies in the
  order, which for a full class should connect the volcano; the residual risk is the 2-isogeny structure and
  the weak locus, not the odd steps.
- Class-group walk reaches the class at p = 1009: 0.75.

## Cost note

The kernel-polynomial step costs one ψ_ℓ factorization over F_{q³} per curve per ℓ (a ppowmod of degree
(ℓ²−1)/2, i.e. 12 or 24), heavier than the rational-root scan. The walk-cost comparison is reported, not gated.

## Amendment 1 (2026-11-12, before any p ≥ 1009 class-group walk finished)

The class-group walk factors ψ_ℓ at every curve met, so a failed walk (exploring up to the cap) is far costlier
than the rational-root walk. To keep the large-prime sample tractable on the 4-core container, p ∈ {37, 53}
use the preregistered walk-limit 60; **p ∈ {1009, 1511, 1823} use walk-limit 20** (a bounded sample). The cap
(10,000 curves), primes, seeds and jumps are unchanged, so W′ is comparable to the rational-root W of
`research/jv_weak_coverage_20261111` (which measured 60, 60, 60, 60, 60 admitted walks). A 20-walk sample still
separates W′ from the rational-root W at p = 1009 (0.05) by the Wilson interval. Disclosed before any large-prime
run completed past its first walks.

## Amendment 2 (2026-11-12, before any p ≥ 1009 class-group walk completed)

A class-group walk that fails explores up to the cap with a ψ_ℓ factorization at every curve; at cap 10,000 and
p ≥ 1009 this did not complete on the container (two walks at p = 1009 ran for the whole budget). The large-prime
comparison is therefore made **paired at a reduced cap**: p ∈ {1009, 1511, 1823}, cap 3,000, walk-limit 30, and
**both** the rational-root walk (`examples/jv_weak_coverage`, `walk_to_weak_record`) and the class-group walk
(`walk_to_weak_classgroup`) are run on the **same seeded admitted curves** (identical `random_curve` sequence,
seed 20261111). W (rational-root) and W′ (class-group) are then compared per curve at the same cap, so the
comparison is valid independent of the cap. The small primes p ∈ {37, 53} keep the preregistered cap 10,000 and
walk-limit 60. The published rational-root W of `research/jv_weak_coverage_20261111` (cap 10,000) is reported
alongside as the reference, not the paired baseline. Disclosed before any large-prime walk completed.
