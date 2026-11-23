module
public import CubicGenerator

@[expose] public section
namespace MomoseCubicHomographyAudit
noncomputable section
open Matrix
variable (K L : Type*) [Field K] [Field L] [Algebra K L]

abbrev Mat2 := Matrix (Fin 2) (Fin 2) K
abbrev GL2 := Matrix.GeneralLinearGroup (Fin 2) K

def numerator (A : Mat2 K) (z : L) : L :=
  algebraMap K L (A 0 0) * z + algebraMap K L (A 0 1)

def denominator (A : Mat2 K) (z : L) : L :=
  algebraMap K L (A 1 0) * z + algebraMap K L (A 1 1)

/-- Cross-multiplied actual fractional-linear action; the denominator is proved nonzero. -/
def sends (A : Mat2 K) (z w : L) : Prop := numerator K L A z = w * denominator K L A z

def fractional (A : Mat2 K) (z : L) : L := numerator K L A z / denominator K L A z

theorem linear_relation_zero {z : L} (hz : outsideBase K L z) (a b : K)
    (h : algebraMap K L a * z + algebraMap K L b = 0) : a = 0 ∧ b = 0 := by
  have ha : a = 0 := by
    by_contra ha
    apply hz
    refine ⟨-b / a, ?_⟩
    have hn : algebraMap K L a ≠ 0 := (map_ne_zero (algebraMap K L)).mpr ha
    rw [map_div₀, map_neg, div_eq_iff hn]
    linear_combination -h
  refine ⟨ha, ?_⟩
  simp only [ha, map_zero, zero_mul, zero_add] at h
  apply (algebraMap K L).injective
  simpa using h

theorem denominator_ne_zero {z : L} (hz : outsideBase K L z) (A : GL2 K) :
    denominator K L A.val z ≠ 0 := by
  intro h
  have hh := linear_relation_zero K L hz (A 1 0) (A 1 1) h
  have hd := A.det_ne_zero
  apply hd
  simp [det_fin_two, hh.1, hh.2]

theorem sends_iff_fractional (A : Mat2 K) (z w : L) (hd : denominator K L A z ≠ 0) :
    sends K L A z w ↔ fractional K L A z = w := by
  exact (div_eq_iff hd).symm

/-- A fractional relation between non-base points with nonzero denominator forces invertibility. -/
theorem determinant_ne_zero {z w : L} (hw : outsideBase K L w) (A : Mat2 K)
    (hd : denominator K L A z ≠ 0) (h : sends K L A z w) : A.det ≠ 0 := by
  intro hz
  have hdet : algebraMap K L (A 0 0) * algebraMap K L (A 1 1) -
      algebraMap K L (A 0 1) * algebraMap K L (A 1 0) = 0 := by
    have hh := congrArg (algebraMap K L) hz
    simpa only [det_fin_two, map_sub, map_mul, map_zero] using hh
  change algebraMap K L (A 0 0) * z + algebraMap K L (A 0 1) =
    w * (algebraMap K L (A 1 0) * z + algebraMap K L (A 1 1)) at h
  have hm : (algebraMap K L (A 1 0) * w - algebraMap K L (A 0 0)) *
      denominator K L A z = 0 := by
    unfold denominator
    linear_combination -algebraMap K L (A 1 0) * h - hdet
  have hh := (mul_eq_zero.mp hm).resolve_right hd
  have hc := linear_relation_zero K L hw (A 1 0) (-A 0 0) (by
    simpa [map_neg, sub_eq_add_neg] using hh)
  have hbd : algebraMap K L (A 1 1) * w + algebraMap K L (-A 0 1) = 0 := by
    simp only [hc.1, neg_eq_zero.mp hc.2, map_zero, zero_mul, zero_add] at h
    rw [map_neg]
    linear_combination -h
  have hd0 := (linear_relation_zero K L hw (A 1 1) (-A 0 1) hbd).1
  apply hd
  simp [denominator, hc.1, hd0]

#print axioms linear_relation_zero
#print axioms denominator_ne_zero
#print axioms determinant_ne_zero
end
end MomoseCubicHomographyAudit
