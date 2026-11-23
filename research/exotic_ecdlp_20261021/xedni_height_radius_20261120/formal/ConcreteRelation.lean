import UniformQuotient
import FreeTorsionQuotient

/-! A bounded relation in the explicit free-plus-torsion lift group. -/

namespace XedniConcreteRelation

open XedniUniformQuotient XedniFreeTorsionQuotient

variable {A : Type*} [AddCommGroup A] [Fintype A]

/-- The exact quotient construction and a rank gap give a bounded relation
under explicit canonical-norm laws. The norm laws are stated on the free
coordinates and can be instantiated by a canonical-height norm. -/
theorem bounded_relation_of_free_torsion_rank
    (d r c K N : ℕ) [NeZero N]
    (points : Fin r → LiftGroup (A := A) d)
    (freeNorm : (Fin d → ℤ) → ℝ) (m size : ℝ)
    (hdegree : d < r) (hN : N ≤ c * K)
    (hlarge : Fintype.card A * c ^ d < K)
    (htriangle : ∀ x y, freeNorm (x - y) ≤ freeNorm x + freeNorm y)
    (hshort : ∀ coeff ∈ coefficientBox r K,
      freeNorm (eval points coeff).1 ≤ size)
    (hscale : ∀ u, freeNorm (N • u) = (N : ℝ) * freeNorm u)
    (hkill : ∀ a : A, N • a = 0)
    (hmin : ∀ u, u ≠ 0 → m ≤ freeNorm u)
    (hgap : size + size < (N : ℝ) * m) :
    ∃ coeff, (∃ i, coeff i ≠ 0) ∧ Bounded (2 * K) coeff ∧
      eval points coeff = 0 := by
  let liftNorm : LiftGroup (A := A) d → ℝ := fun z => freeNorm z.1
  apply bounded_relation_of_rank_quotient points (quotientMap d N) liftNorm
    N (Fintype.card A) d K m size
  · intro x y
    exact htriangle x.1 y.1
  · exact hshort
  · exact kernel_divisible d N
  · intro u
    exact hscale u.1
  · intro u hu
    exact min_norm_of_nonzero_multiple d N freeNorm m hkill hmin u hu
  · exact hgap
  · exact le_of_eq (quotient_card (A := A) d N)
  · exact rank_quotient_count (Fintype.card A) d r c K N hdegree hN hlarge

#print axioms XedniConcreteRelation.bounded_relation_of_free_torsion_rank

end XedniConcreteRelation
