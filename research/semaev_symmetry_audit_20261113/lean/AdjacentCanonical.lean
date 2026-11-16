import CanonicalTower

namespace SemaevSymmetryAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
open SemaevReassociationAudit RecursiveSemaevTransportAudit RecursiveSemaevSymmetryAudit
noncomputable section
variable {R : Type*} [CommRing R]

def tripleS3 (b2 b4 b6 b8 : R) : Triple R :=
  s3Ring (tripleLift b2) (tripleLift b4) (tripleLift b6) (tripleLift b8)
    formalZ formalX formalY

theorem tripleS3_inner (b2 b4 b6 b8 : R) :
    innerSwap (tripleS3 b2 b4 b6 b8) = tripleS3 b2 b4 b6 b8 := by
  unfold tripleS3
  rw [s3Ring_map (R := Triple R) (S := Triple R) (innerSwap (R := R))
    (tripleLift b2) (tripleLift b4) (tripleLift b6) (tripleLift b8) formalZ formalX formalY]
  rw [innerSwap_scalar, innerSwap_scalar, innerSwap_scalar, innerSwap_scalar,
    innerSwap_Z, innerSwap_X, innerSwap_Y]
  unfold s3Ring qaRing qbRing qcRing
  ring

theorem tripleS3_outer (b2 b4 b6 b8 : R) :
    outerSwap (tripleS3 b2 b4 b6 b8) = tripleS3 b2 b4 b6 b8 := by
  unfold tripleS3
  rw [s3Ring_map (R := Triple R) (S := Triple R) (outerSwap (R := R))
    (tripleLift b2) (tripleLift b4) (tripleLift b6) (tripleLift b8) formalZ formalX formalY]
  rw [outerSwap_scalar, outerSwap_scalar, outerSwap_scalar, outerSwap_scalar,
    outerSwap_Z, outerSwap_X, outerSwap_Y]
  unfold s3Ring qaRing qbRing qcRing
  ring

theorem step_pairPoly (b2 b4 b6 b8 x y : R) :
    resultantStep 2 (pairPoly b2 b4 b6 b8 x y) b2 b4 b6 b8 =
      s4Ring (C (C b2)) (C (C b4)) (C (C b6)) (C (C b8))
        (C (C x)) (C (C y)) (C X) X := by
  have hf : firstLift (pairPoly b2 b4 b6 b8 x y) =
      quad (qaRing (C (C x)) (C (C y)))
        (qbRing (C (C b2)) (C (C b4)) (C (C b6)) (C (C x)) (C (C y)))
        (qcRing (C (C b4)) (C (C b6)) (C (C b8)) (C (C x)) (C (C y))) := by
    unfold firstLift pairPoly
    rw [SemaevReassociationAudit.quad_map]
    simp [qaRing, qbRing, qcRing, RingHom.comp_apply, map_ofNat]
  unfold resultantStep
  rw [hf]
  rfl

theorem pairPoly_source (b2 b4 b6 b8 x y : R) :
    pairPoly b2 b4 b6 b8 x y = s3Ring (C b2) (C b4) (C b6) (C b8) (C x) (C y) X := by
  rw [s3Ring, qaRing_C, qbRing_C, qcRing_C]
  rfl

variable {K : Type*} [Field K]

theorem towerCanonical_base_pair (W : WeierstrassCurve.Affine K) :
    (towerCanonical W 0 : Polynomial (Tower K 2)) =
      pairPoly (towerScalar 2 W.b₂) (towerScalar 2 W.b₄)
        (towerScalar 2 W.b₆) (towerScalar 2 W.b₈)
        (towerVariable 2 1) (towerVariable 2 0) := by
  change (basePolynomial W).map (RingHom.id (InputRing K 0)) = _
  rw [Polynomial.map_id, towerVariable_two_one, towerVariable_zero]
  rfl

theorem towerCanonical_base_triple (W : WeierstrassCurve.Affine K) :
    towerCanonical W 0 = tripleS3 W.b₂ W.b₄ W.b₆ W.b₈ := by
  rw [towerCanonical_base_pair, pairPoly_source, towerVariable_two_one, towerVariable_zero]
  rfl

theorem towerCanonical_base_adjacent (W : WeierstrassCurve.Affine K) (i : Fin 2) :
    adjacent 1 i (towerCanonical W 0) = towerCanonical W 0 := by
  fin_cases i
  · change outerSwap (towerCanonical W 0 : Triple K) = _
    rw [towerCanonical_base_triple]
    exact tripleS3_outer ..
  · change innerSwap (towerCanonical W 0 : Triple K) = _
    rw [towerCanonical_base_triple]
    exact tripleS3_inner ..

theorem towerCanonical_four (W : WeierstrassCurve.Affine K) :
    towerCanonical W 1 = s4Ring (towerScalar 4 W.b₂) (towerScalar 4 W.b₄)
      (towerScalar 4 W.b₆) (towerScalar 4 W.b₈)
      (towerVariable 4 3) (towerVariable 4 2) (towerVariable 4 1) (towerVariable 4 0) := by
  rw [towerCanonical_succ]
  norm_num only [Nat.reduceAdd, Nat.reducePow]
  rw [towerCanonical_base_pair]
  change resultantStep 2
    (pairPoly (towerScalar 2 W.b₂) (towerScalar 2 W.b₄) (towerScalar 2 W.b₆)
      (towerScalar 2 W.b₈) (towerVariable 2 1) (towerVariable 2 0))
    (towerScalar 2 W.b₂) (towerScalar 2 W.b₄) (towerScalar 2 W.b₆) (towerScalar 2 W.b₈) = _
  rw [step_pairPoly]
  simp only [towerVariable_two_one,
    towerVariable_four_one, towerVariable_four_two, towerVariable_four_three, towerVariable_zero]
  rfl

theorem adjacent_four_middle_scalar (a : K) :
    adjacent 2 (1 : Fin 3) (towerScalar 4 a) = towerScalar 4 a := adjacent_scalar 2 1 a

theorem adjacent_four_middle_coordinates :
    (adjacent 2 (1 : Fin 3) (towerVariable 4 3 : Tower K 4) = towerVariable 4 3) ∧
    (adjacent 2 (1 : Fin 3) (towerVariable 4 2 : Tower K 4) = towerVariable 4 1) ∧
    (adjacent 2 (1 : Fin 3) (towerVariable 4 1 : Tower K 4) = towerVariable 4 2) ∧
    (adjacent 2 (1 : Fin 3) (towerVariable 4 0 : Tower K 4) = towerVariable 4 0) := by
  rw [adjacent_two_one]
  simp only [towerVariable_four_one, towerVariable_four_two,
    towerVariable_four_three, towerVariable_zero]
  simp only [Polynomial.mapEquiv_apply, Polynomial.map_C, Polynomial.map_X,
    RingEquiv.coe_toRingHom, AlgEquiv.coe_toRingEquiv,
    Polynomial.Bivariate.swap_C_C, Polynomial.Bivariate.swap_X, Polynomial.Bivariate.swap_Y]
  trivial

theorem towerCanonical_four_middle (W : WeierstrassCurve.Affine K) :
    adjacent 2 (1 : Fin 3) (towerCanonical W 1) = towerCanonical W 1 := by
  rw [towerCanonical_four]
  change (adjacent (R := K) 2 (1 : Fin 3)).toRingHom
    (s4Ring (towerScalar 4 W.b₂) (towerScalar 4 W.b₄) (towerScalar 4 W.b₆)
      (towerScalar 4 W.b₈) (towerVariable 4 3) (towerVariable 4 2)
      (towerVariable 4 1) (towerVariable 4 0)) = _
  rw [s4Ring_map (R := Tower K 4) (S := Tower K 4) (adjacent 2 (1 : Fin 3)).toRingHom
    (towerScalar 4 W.b₂) (towerScalar 4 W.b₄) (towerScalar 4 W.b₆) (towerScalar 4 W.b₈)
    (towerVariable 4 3) (towerVariable 4 2) (towerVariable 4 1) (towerVariable 4 0)]
  change s4Ring ((adjacent (R := K) 2 (1 : Fin 3)) (towerScalar 4 W.b₂))
    ((adjacent (R := K) 2 (1 : Fin 3)) (towerScalar 4 W.b₄))
    ((adjacent (R := K) 2 (1 : Fin 3)) (towerScalar 4 W.b₆))
    ((adjacent (R := K) 2 (1 : Fin 3)) (towerScalar 4 W.b₈))
    ((adjacent (R := K) 2 (1 : Fin 3)) (towerVariable 4 3))
    ((adjacent (R := K) 2 (1 : Fin 3)) (towerVariable 4 2))
    ((adjacent (R := K) 2 (1 : Fin 3)) (towerVariable 4 1))
    ((adjacent (R := K) 2 (1 : Fin 3)) (towerVariable 4 0)) = _
  rw [adjacent_four_middle_scalar, adjacent_four_middle_scalar,
    adjacent_four_middle_scalar, adjacent_four_middle_scalar,
    adjacent_four_middle_coordinates.1, adjacent_four_middle_coordinates.2.1,
    adjacent_four_middle_coordinates.2.2.1, adjacent_four_middle_coordinates.2.2.2]
  have h := actual_S4_middle_swap (W.map (towerScalar 4))
    (towerVariable 4 3) (towerVariable 4 2) (towerVariable 4 1) (towerVariable 4 0)
  simpa only [WeierstrassCurve.map_b₂, WeierstrassCurve.map_b₄,
    WeierstrassCurve.map_b₆, WeierstrassCurve.map_b₈] using h.symm

theorem towerCanonical_outer (W : WeierstrassCurve.Affine K) (n : ℕ) :
    adjacent (n+2) 0 (towerCanonical W (n+1)) = towerCanonical W (n+1) := by
  rw [towerCanonical_succ]
  exact resultantStep_exchange ..

theorem towerCanonical_middle (W : WeierstrassCurve.Affine K) (n : ℕ) :
    adjacent (n+2) 1 (towerCanonical W (n+1)) = towerCanonical W (n+1) := by
  cases n with
  | zero => exact towerCanonical_four_middle W
  | succ n =>
    change innerSwap (towerCanonical W (n+2) : Triple (Tower K (n+2))) = _
    rw [towerCanonical_double]
    have h := doubleStep_innerSwap (2^(n+1)) (towerCanonical W n)
      (towerCanonical_degree W n).le (W.map (towerScalar (n+2)))
    rw [WeierstrassCurve.map_b₂, WeierstrassCurve.map_b₄,
      WeierstrassCurve.map_b₆, WeierstrassCurve.map_b₈] at h
    exact h

theorem towerCanonical_adjacent (W : WeierstrassCurve.Affine K) (n : ℕ)
    (i : Fin (n+2)) : adjacent (n+1) i (towerCanonical W n) = towerCanonical W n := by
  induction n with
  | zero => exact towerCanonical_base_adjacent W i
  | succ n ih =>
    refine Fin.cases (towerCanonical_outer W n) (fun j => ?_) i
    refine Fin.cases (towerCanonical_middle W n) (fun k => ?_) j
    rw [towerCanonical_succ]
    let p : Polynomial (Tower K (n+2)) := towerCanonical W n
    change (resultantStep (2^(n+1)) p
      (towerScalar (n+2) W.b₂) (towerScalar (n+2) W.b₄)
      (towerScalar (n+2) W.b₆) (towerScalar (n+2) W.b₈)).map
      (mapRingHom (adjacent n k).toRingHom) = _
    rw [resultantStep_map (A := Tower K (n+2)) (B := Tower K (n+2))
      (adjacent n k).toRingHom]
    have hp : p.map
        (adjacent n k).toRingHom = towerCanonical W n := ih k.succ
    rw [hp]
    have hs (a : K) : (adjacent n k).toRingHom (towerScalar (n+2) a) =
        towerScalar (n+2) a := adjacent_scalar n k a
    rw [hs,hs,hs,hs]

#print axioms towerCanonical_adjacent

end
end SemaevSymmetryAudit
