import CubicArithmetic

namespace CubicNormalization
noncomputable section
open MvPolynomial CubicArithmetic FilteredClosure

variable {K σ : Type*} [Field K]

def family (t u : MvPolynomial σ K) (A B D : Coeff K) : MvPolynomial σ K :=
  poly t A*u^2+poly t B*u+poly t D

theorem family_degree (t u : MvPolynomial σ K) (A B D : Coeff K)
    (ht : t.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1) :
    (family t u A B D).totalDegree ≤ 4 := by
  have ha := poly_degree t A ht
  have hb := poly_degree t B ht
  have hd := poly_degree t D ht
  have hu2 := totalDegree_pow u 2
  have hm := totalDegree_mul (poly t A) (u^2)
  have hm' := totalDegree_mul (poly t B) u
  have hs := totalDegree_add (poly t A*u^2) (poly t B*u)
  have hs' := totalDegree_add (poly t A*u^2+poly t B*u) (poly t D)
  unfold family
  omega

theorem monic_degree (t u : MvPolynomial σ K) (B D : Coeff K)
    (ht : t.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1) :
    (u^2+poly t B*u+poly t D).totalDegree ≤ 3 := by
  have hb := poly_degree t B ht
  have hd := poly_degree t D ht
  have hu2 := totalDegree_pow u 2
  have hm := totalDegree_mul (poly t B) u
  have hs := totalDegree_add (u^2) (poly t B*u)
  have hs' := totalDegree_add (u^2+poly t B*u) (poly t D)
  omega

def topRow (A B D : Coeff K) : Coeff K := fun i =>
  if i=0 then D 2 else if i=1 then B 2 else A 2

theorem rotation_identity (s A B D : Coeff K) (t u : MvPolynomial σ K) :
    t*family t u A B D-poly u (topRow A B D)*cubic t s =
      family t u (rotate s A) (rotate s B) (rotate s D) := by
  simp [family, poly, topRow, cubic, rotate]
  ring

/-- One legal degree-five multiplication followed by cubic reduction.
The resulting family returns to degree at most four. -/
theorem rotation_derivable (F : Set (MvPolynomial σ K)) (s A B D : Coeff K)
    (t u : MvPolynomial σ K) (ht : t.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1)
    (hg : Derivable F 5 (cubic t s)) (hq : Derivable F 5 (family t u A B D)) :
    Derivable F 5 (family t u (rotate s A) (rotate s B) (rotate s D)) := by
  have hqdeg := family_degree t u A B D ht hu
  have hrow := poly_degree u (topRow A B D) hu
  have hgdeg := cubic_degree t s ht
  have hmul : (t*family t u A B D).totalDegree ≤ 5 :=
    (totalDegree_mul _ _).trans (by omega)
  have hcor : (poly u (topRow A B D)*cubic t s).totalDegree ≤ 5 :=
    (totalDegree_mul _ _).trans (by omega)
  rw [← rotation_identity s A B D t u]
  exact FilteredClosure.sub (Derivable.mul t hq hmul)
    (Derivable.mul (poly u (topRow A B D)) hg hcor)

theorem normalization_identity (s A B D I : Coeff K) (t u : MvPolynomial σ K) :
    I 0 • family t u A B D+
      I 1 • family t u (rotate s A) (rotate s B) (rotate s D)+
      I 2 • family t u (rotate s (rotate s A)) (rotate s (rotate s B))
        (rotate s (rotate s D)) =
      family t u (mulReduced s A I) (mulReduced s B I) (mulReduced s D I) := by
  simp only [family, mulReduced, poly_add, poly_smul, smul_eq_C_mul]
  ring

/-- Complete staged monic normalization, with a constructed interpolation
inverse and with both transient multiplications checked at degree five. -/
theorem monic_normalization (F : Set (MvPolynomial σ K)) (A B D : Coeff K)
    (a b c : K) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (ha : val A a ≠ 0) (hb : val A b ≠ 0) (hc : val A c ≠ 0)
    (t u : MvPolynomial σ K) (ht : t.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1)
    (hg : Derivable F 5 (cubic t (rootsCoefficients a b c)))
    (hq : Derivable F 4 (family t u A B D)) :
    let s := rootsCoefficients a b c
    let I := inverseCoeff A a b c
    Derivable F 5 (u^2+poly t (mulReduced s B I)*u+poly t (mulReduced s D I)) := by
  let s := rootsCoefficients a b c
  let I := inverseCoeff A a b c
  have hq₀ : Derivable F 5 (family t u A B D) := monotone (by omega) hq
  have hq₁ := rotation_derivable F s A B D t u ht hu hg hq₀
  have hq₂ := rotation_derivable F s (rotate s A) (rotate s B) (rotate s D)
    t u ht hu hg hq₁
  have hlin := Derivable.add (Derivable.add (Derivable.smul (I 0) hq₀)
    (Derivable.smul (I 1) hq₁)) (Derivable.smul (I 2) hq₂)
  rw [normalization_identity] at hlin
  have hInv : mulReduced s A I = oneCoeff :=
    inverseCoeff_normalizes A a b c hab hac hbc ha hb hc
  simpa only [family, hInv, poly_one, one_mul] using hlin

end
end CubicNormalization

#print axioms CubicNormalization.rotation_derivable
#print axioms CubicNormalization.monic_normalization
