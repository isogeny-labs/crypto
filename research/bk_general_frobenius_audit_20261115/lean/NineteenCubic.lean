import SourceGeometric
import BKGeneralImports

namespace BKGeneralFrobeniusAudit.SourceNineteen
noncomputable section
open Polynomial
private instance : Fact (Nat.Prime 19) := ⟨by decide⟩

def cubic : (ZMod 19)[X] := X^3 + C 12 * X^2 + C 13 * X + C 9

theorem cubic_monic : cubic.Monic := by unfold cubic; monicity!
theorem cubic_degree : cubic.natDegree = 3 := by unfold cubic; compute_degree!
theorem cubic_no_root : ∀ x : ZMod 19, cubic.eval x ≠ 0 := by
  simp only [cubic, eval_add, eval_pow, eval_mul, eval_X, eval_C]
  decide +kernel

theorem cubic_irreducible : Irreducible cubic :=
  Polynomial.irreducible_of_degree_le_three_of_not_isRoot
    (by rw [cubic_degree]; decide) cubic_no_root

theorem psi_factor : curve.Ψ₃ = 3 * (X - C 12) * cubic := by
  norm_num [curve, WeierstrassCurve.Ψ₃, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈, cubic, Polynomial.C_ofNat]
  have h19 : (19 : (ZMod 19)[X]) = 0 := CharP.cast_eq_zero _ 19
  linear_combination (21 * X^2 + 27 * X + 17) * h19

abbrev CubeField := AdjoinRoot cubic
private instance : Fact (Irreducible cubic) := ⟨cubic_irreducible⟩
private instance : Finite CubeField := Finite.of_equiv (Fin cubic.natDegree → ZMod 19)
  (AdjoinRoot.powerBasis cubic_monic.ne_zero).basis.equivFun.symm.toEquiv
private instance : Fintype CubeField := Fintype.ofFinite _

theorem cube_field_dimension : Module.finrank (ZMod 19) CubeField = 3 :=
  (AdjoinRoot.powerBasis cubic_monic.ne_zero).finrank.trans cubic_degree

theorem cube_field_card : Fintype.card CubeField = 19^3 := by
  rw [Module.card_eq_pow_finrank (K := ZMod 19) (V := CubeField),
    cube_field_dimension, ZMod.card]

theorem cubic_root_frobenius {L : Type*} [Field L] [Algebra (ZMod 19) L]
    (x : L) (hx : cubic.eval₂ (algebraMap (ZMod 19) L) x = 0) : x^(19^3) = x := by
  let f := AdjoinRoot.lift (algebraMap (ZMod 19) L) x hx
  have h := FiniteField.pow_card (AdjoinRoot.root cubic)
  rw [cube_field_card] at h
  have hh := congrArg f h
  simpa only [f, map_pow, AdjoinRoot.lift_root] using hh

theorem psi_root_frobenius {L : Type*} [Field L] [Algebra (ZMod 19) L]
    (x : L) (hx : (curve.baseChange L).Ψ₃.IsRoot x) : x^(19^3) = x := by
  have hroot : ((curve.map (algebraMap (ZMod 19) L)).Ψ₃).eval x = 0 := hx
  rw [WeierstrassCurve.map_Ψ₃, psi_factor] at hroot
  have hfac : (3 : L) * (x - 12) * cubic.eval₂ (algebraMap (ZMod 19) L) x = 0 := by
    simpa only [Polynomial.eval_map, eval₂_mul, eval₂_sub, eval₂_X,
      eval₂_C, eval₂_ofNat, map_ofNat] using hroot
  have h3 : (3 : L) ≠ 0 := by
    simpa only [map_ofNat] using
      (map_ne_zero (algebraMap (ZMod 19) L)).mpr (by decide : (3 : ZMod 19) ≠ 0)
  rcases mul_eq_zero.mp hfac with hleft | hright
  · have hx12 : x = 12 := sub_eq_zero.mp ((mul_eq_zero.mp hleft).resolve_left h3)
    rw [hx12]
    have hb : (12 : L)^19 = 12 := by
      have hh := congrArg (algebraMap (ZMod 19) L) (FiniteField.pow_card (12 : ZMod 19))
      simpa only [map_pow, map_ofNat, ZMod.card] using hh
    calc
      (12 : L)^(19^3) = (((12 : L)^19)^19)^19 := by simp only [← pow_mul]; rfl
      _ = 12 := by rw [hb, hb, hb]
  · exact cubic_root_frobenius x hright

#print axioms cubic_irreducible
#print axioms cube_field_card
#print axioms psi_root_frobenius
end
end BKGeneralFrobeniusAudit.SourceNineteen
