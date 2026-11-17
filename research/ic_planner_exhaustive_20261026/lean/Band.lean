/-!
# The AGENTS.md §5a pricing band is affine in the weight

ecbench prices a run at weight `w` (GAE per unpriced native operation) as
`priced(w) = c + w · n`, with `c = G − C` the charged GAE and `n = U + n_C` the
native operations (`src/cryptanalysis/ecbench/band.rs`, `priced_gae`). Over an
arm's verified runs the sums keep the form: `Σ priced(w) = charged_sum + w ·
native_sum`, the two exact sums `ecbench band` now reports. A weight is a
non-negative rational `p / q`; every statement below is multiplied through by
`q > 0`, so it lives in `Int` and needs no rational library.

* `order_at`: at `w = p/q`, arm `A` is cheaper than `B` iff
  `p · (nA − nB) < q · (cB − cA)`.
* `crossing`: if `A` is cheaper at `w = 0` and dearer at `w = 1`, then
  `nA > nB` and the orders on each side of the unique crossing weight
  `w* = (cB − cA) / (nA − nB) ∈ (0, 1)` are fixed: so "not ranked across the
  band" is forced, not a statistical accident.
* `optimal_native_nonincreasing`: if arm `i` is cheapest at `w` and arm `j` at
  `v > w`, then `n_j ≤ n_i`: raising the weight can only move the optimum to an
  arm with no more native work. Prime bases grow (fewer trials, each a lookup
  and a guard probe); Koblitz bases shrink (cheaper canonicalisation and
  probes): each direction is the one that lowers `n`.
-/

namespace Band

/-- `q · priced(p/q)` for an arm with charged sum `c` and native sum `n`. -/
def scaled (c n p q : Int) : Int := q * c + p * n

theorem order_at (cA nA cB nB p q : Int) :
    scaled cA nA p q < scaled cB nB p q ↔ p * (nA - nB) < q * (cB - cA) := by
  unfold scaled
  rw [Int.mul_sub, Int.mul_sub]
  generalize q * cA = a; generalize q * cB = b; generalize p * nA = x; generalize p * nB = y
  omega

theorem order_at_le (cA nA cB nB p q : Int) :
    scaled cA nA p q ≤ scaled cB nB p q ↔ p * (nA - nB) ≤ q * (cB - cA) := by
  unfold scaled
  rw [Int.mul_sub, Int.mul_sub]
  generalize q * cA = a; generalize q * cB = b; generalize p * nA = x; generalize p * nB = y
  omega

/-- A reversal between `w = 0` and `w = 1` forces a crossing strictly inside. -/
theorem crossing (cA nA cB nB : Int) (h0 : cA < cB) (h1 : cB + nB < cA + nA) :
    0 < cB - cA ∧ cB - cA < nA - nB := by
  omega

/-- Below the crossing weight `A` is cheaper, above it `B` is: for `q > 0`,
`A < B` at `p/q` exactly when `p · (nA − nB) < q · (cB − cA)`, and the
reverse strict order exactly when `>`. -/
theorem sides (cA nA cB nB p q : Int) :
    (scaled cA nA p q < scaled cB nB p q ↔ p * (nA - nB) < q * (cB - cA)) ∧
    (scaled cB nB p q < scaled cA nA p q ↔ q * (cB - cA) < p * (nA - nB)) := by
  refine ⟨order_at cA nA cB nB p q, ?_⟩
  unfold scaled
  rw [Int.mul_sub, Int.mul_sub]
  generalize q * cA = a; generalize q * cB = b; generalize p * nA = x; generalize p * nB = y
  omega

/-- The optimum's native count is non-increasing in the weight. Weights are
`w / q` and `v / q` on any common denominator `q` (it cancels). -/
theorem optimal_native_nonincreasing (ci ni cj nj w v q : Int) (hwv : w < v)
    (hi : scaled ci ni w q ≤ scaled cj nj w q)
    (hj : scaled cj nj v q ≤ scaled ci ni v q) : nj ≤ ni := by
  unfold scaled at hi hj
  -- hi + hj: w·ni + v·nj ≤ w·nj + v·ni, i.e. (v − w)·(nj − ni) ≤ 0.
  have key : (v - w) * (nj - ni) ≤ 0 := by
    rw [Int.sub_mul, Int.mul_sub, Int.mul_sub]
    generalize q * ci = a at hi hj; generalize q * cj = b at hi hj
    generalize w * ni = x1 at hi hj ⊢; generalize w * nj = x2 at hi hj ⊢
    generalize v * ni = y1 at hi hj ⊢; generalize v * nj = y2 at hi hj ⊢
    omega
  have hpos : 0 < v - w := by omega
  by_cases h : nj ≤ ni
  · exact h
  · have : 0 < nj - ni := by omega
    have := Int.mul_pos hpos this
    omega

end Band
