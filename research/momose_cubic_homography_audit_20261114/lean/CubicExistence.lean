module
public import HomographyBasics

@[expose] public section
namespace MomoseCubicHomographyAudit
noncomputable section
open Matrix
variable (K L : Type*) [Field K] [Field L] [Algebra K L] [FiniteDimensional K L]

/-- Constructive fractional-linear transitivity on the actual cubic extension. -/
theorem cubic_homography_exists (hd : Module.finrank K L = 3) {z w : L}
    (hz : outsideBase K L z) (hw : outsideBase K L w) :
    ∃ A : GL2 K, sends K L A.val z w := by
  obtain ⟨a, b, c, hc⟩ := cubic_coordinates K L hd hz w
  obtain ⟨u, v, t, ht⟩ := cubic_coordinates K L hd hz (z ^ 3)
  have hab : a ≠ 0 ∨ b ≠ 0 := by
    by_contra h
    have hh : a = 0 ∧ b = 0 := by tauto
    apply hw
    refine ⟨c, ?_⟩
    simpa [hh.1, hh.2] using hc.symm
  let A : Mat2 K := !![a^2*v + a*c - a*b*u - b^2, a^2*t - a*c*u - b*c;
                         a, -(a*u + b)]
  have hden : denominator K L A z ≠ 0 := by
    intro he
    have hh := linear_relation_zero K L hz a (-(a*u+b)) he
    have ha : a = 0 := hh.1
    have hb : b = 0 := by simpa [ha] using hh.2
    exact hab.elim (fun h => h ha) (fun h => h hb)
  have hs : sends K L A z w := by
    dsimp [sends, numerator, denominator, A]
    simp only [map_add, map_sub, map_mul, map_pow, map_neg]
    rw [hc]
    linear_combination -(algebraMap K L a)^2 * ht
  exact ⟨Matrix.GeneralLinearGroup.mkOfDetNeZero A (determinant_ne_zero K L hw A hden hs), hs⟩

/-- The source's explicit matrix in the depressed-cubic case. -/
theorem source_explicit_matrix (hd : Module.finrank K L = 3) {z : L}
    (hz : outsideBase K L z) (r e a b c : K)
    (hrel : z^3 = algebraMap K L r * z + algebraMap K L e)
    (hab : a ≠ 0 ∨ b ≠ 0) :
    let A : Mat2 K := !![a*(a*r+c)-b^2, a^2*e-b*c; a, -b]
    A.det ≠ 0 ∧ denominator K L A z ≠ 0 ∧
      fractional K L A z = algebraMap K L a*z^2 + algebraMap K L b*z + algebraMap K L c := by
  intro A
  have hw : outsideBase K L
      (algebraMap K L a*z^2 + algebraMap K L b*z + algebraMap K L c) := by
    rintro ⟨k, hk⟩
    have hh := quadratic_relation_zero K L hd hz a b (c-k) (by
      rw [map_sub]
      linear_combination -hk)
    exact hab.elim (fun h => h hh.1) (fun h => h hh.2.1)
  have hden : denominator K L A z ≠ 0 := by
    intro he
    have hh := linear_relation_zero K L hz a (-b) he
    exact hab.elim (fun h => h hh.1) (fun h => h (neg_eq_zero.mp hh.2))
  have hs : sends K L A z
      (algebraMap K L a*z^2 + algebraMap K L b*z + algebraMap K L c) := by
    dsimp [sends, numerator, denominator, A]
    simp only [map_add, map_sub, map_mul, map_pow, map_neg]
    linear_combination -(algebraMap K L a)^2 * hrel
  exact ⟨determinant_ne_zero K L hw A hden hs, hden,
    (sends_iff_fractional K L A _ _ hden).mp hs⟩

#print axioms cubic_homography_exists
#print axioms source_explicit_matrix
end
end MomoseCubicHomographyAudit
