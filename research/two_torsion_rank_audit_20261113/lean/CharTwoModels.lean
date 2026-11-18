import CharTwoCardinality

namespace TwoTorsionRankAudit
open Polynomial WeierstrassCurve.Affine
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 2]

def ordinaryModel (K : Type*) [Field K] : WeierstrassCurve.Affine K := ⟨1,0,0,0,1⟩
def supersingularModel (K : Type*) [Field K] : WeierstrassCurve.Affine K := ⟨0,0,1,0,0⟩

theorem ordinary_delta : (ordinaryModel K).Δ=1 := by
  rw [WeierstrassCurve.Δ_of_char_two]
  norm_num [ordinaryModel,WeierstrassCurve.b₈]

theorem supersingular_delta : (supersingularModel K).Δ=1 := by
  rw [WeierstrassCurve.Δ_of_char_two]
  norm_num [supersingularModel,WeierstrassCurve.b₈]

theorem ordinary_nonsingular : (ordinaryModel K).Nonsingular 0 1 := by
  apply ((ordinaryModel K).equation_iff_nonsingular_of_Δ_ne_zero
    (by rw [ordinary_delta]; exact one_ne_zero)).mp
  norm_num [WeierstrassCurve.Affine.equation_iff,ordinaryModel]

def ordinaryPoint (K : Type*) [Field K] [DecidableEq K] [CharP K 2] :
    (ordinaryModel K).Point := Point.some 0 1 ordinary_nonsingular

theorem ordinaryPoint_nonzero : ordinaryPoint K≠0 := Point.some_ne_zero _

theorem ordinaryPoint_killed : 2 • ordinaryPoint K=0 := by
  apply (char_two_killed_iff (ordinaryModel K) 0 1 ordinary_nonsingular).mpr
  norm_num [ordinaryModel]

theorem ordinaryPoint_order : addOrderOf (ordinaryPoint K)=2 := by
  apply addOrderOf_eq_prime
  · exact ordinaryPoint_killed
  · exact ordinaryPoint_nonzero

theorem ordinary_classification (P : (ordinaryModel K).Point) :
    2 • P=0 ↔ P=0 ∨ P=ordinaryPoint K := by
  constructor
  · intro hk
    by_cases hp : P=0
    · exact Or.inl hp
    · exact Or.inr (nonzero_killed_unique (ordinaryModel K) (by change (1:K)≠0; exact one_ne_zero)
        P (ordinaryPoint K) hp ordinaryPoint_nonzero hk ordinaryPoint_killed)
  · rintro (rfl | rfl)
    · exact smul_zero _
    · exact ordinaryPoint_killed

theorem ordinary_torsion_card : Nat.card (TwoTorsion (ordinaryModel K))=2 :=
  torsion_card_two_of_classification _ (ordinaryPoint K) ordinaryPoint_nonzero
    ordinaryPoint_killed (fun P h => (ordinary_classification P).mp h)

theorem supersingular_torsion_card : Nat.card (TwoTorsion (supersingularModel K))=1 :=
  char_two_torsion_card_one _ (by rw [supersingular_delta]; exact one_ne_zero) rfl

variable {L : Type*} [Field L] [DecidableEq L] [CharP L 2]

theorem ordinary_equation_map (f : K →+* L) (x y : K) (h : (ordinaryModel K).Equation x y) :
    (ordinaryModel L).Equation (f x) (f y) := by
  have he : y^2+x*y=x^3+1 := by
    simpa [ordinaryModel] using ((ordinaryModel K).equation_iff x y).mp h
  apply ((ordinaryModel L).equation_iff (f x) (f y)).mpr
  simpa [ordinaryModel] using congrArg f he

/-- Actual coordinatewise point extension on the constant curve. This is
used for field-of-definition inclusion; no unproved torsion basis is supplied. -/
def ordinaryMap (f : K →+* L) : (ordinaryModel K).Point → (ordinaryModel L).Point
  | .zero => .zero
  | .some x y h => Point.some (f x) (f y)
      (((ordinaryModel L).equation_iff_nonsingular_of_Δ_ne_zero
        (by rw [ordinary_delta]; exact one_ne_zero)).mp (ordinary_equation_map f x y h.1))

theorem ordinaryMap_zero (f : K →+* L) : ordinaryMap f 0=0 := rfl

theorem ordinaryMap_point (f : K →+* L) : ordinaryMap f (ordinaryPoint K)=ordinaryPoint L := by
  simp only [ordinaryMap,ordinaryPoint,map_zero,map_one]

theorem ordinary_all_torsion_in_image (f : K →+* L) (Q : (ordinaryModel L).Point)
    (hQ : 2 • Q=0) : ∃ P : (ordinaryModel K).Point,2 • P=0 ∧ Q=ordinaryMap f P := by
  rcases (ordinary_classification Q).mp hQ with h | h
  · exact ⟨0,smul_zero _,by rw [h,ordinaryMap_zero]⟩
  · exact ⟨ordinaryPoint K,ordinaryPoint_killed,by rw [h,ordinaryMap_point]⟩

theorem ordinary_two_divides_point_card [Finite K] : 2∣Nat.card (ordinaryModel K).Point := by
  have h := addOrderOf_dvd_natCard (ordinaryPoint K)
  rw [ordinaryPoint_order] at h
  exact h

#print axioms ordinary_all_torsion_in_image
#print axioms ordinary_torsion_card
end
end TwoTorsionRankAudit
