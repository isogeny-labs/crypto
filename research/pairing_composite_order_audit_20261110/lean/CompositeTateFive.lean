import BilinearObstruction

namespace CompositeTateFiveAudit
open CompositeTateObstructionAudit
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev F := ZMod 5
abbrev curve := SkepticalAudit.rationalCurve

theorem curve_nonsingular : curve.Δ≠0 := SkepticalAudit.rationalCurve_nonsingular
theorem curve_cardinality : Nat.card curve.Point=6 := TorsionBoundaryAudit.small_curve_cardinality

def P : curve.Point := .some 0 1 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

theorem P_double : P+P= -P := by decide +kernel
theorem P_ne_zero : P≠0 := by decide +kernel
theorem P_three : (3:ℕ) • P=0 := by
  calc (3:ℕ) • P=(P+P)+P := by
         rw [show (3:ℕ)=2+1 by decide,add_nsmul,two_nsmul,one_nsmul]
       _ = 0 := by rw [P_double,neg_add_cancel]
theorem P_six : (6:ℕ) • P=0 := by
  rw [show (6:ℕ)=3+3 by decide,add_nsmul,P_three,add_zero]
theorem P_order : addOrderOf P=3 := addOrderOf_eq_prime P_three P_ne_zero

def sixTorsion : AddSubgroup curve.Point :=
  (nsmulAddMonoidHom 6 : curve.Point→+curve.Point).ker
abbrev Left := ↥sixTorsion
def sixMultiples : AddSubgroup curve.Point :=
  (nsmulAddMonoidHom 6 : curve.Point→+curve.Point).range
abbrev Right := curve.Point ⧸ sixMultiples

def torsionP : Left := ⟨P,by change (6:ℕ) • P=0;exact P_six⟩
theorem torsionP_double_ne_zero : torsionP+torsionP≠0 := by
  intro h
  have hh := congrArg Subtype.val h
  change P+P=0 at hh
  rw [P_double,neg_eq_zero] at hh
  exact P_ne_zero hh

abbrev U := Fˣ
def sixthPowers : Subgroup U := (powMonoidHom 6 : U→*U).range
abbrev Target := U ⧸ sixthPowers

def root : U := Units.mk0 (4:F) (by decide +kernel)
def witness : U := Units.mk0 (2:F) (by decide +kernel)
theorem source_hypotheses : Nat.Coprime 6 5 ∧ 6∣Nat.card curve.Point ∧
    root^6=1 ∧ root≠1 := by
  rw [curve_cardinality]
  decide +kernel

theorem unit_four : ∀ u : U,u^4=1 := by decide +kernel

theorem sixth_eq_square (u : U) : u^6=u^2 := by
  rw [show (6:ℕ)=4+2 by decide,pow_add,unit_four,one_mul]

theorem target_square (z : Target) : z^2=1 := by
  refine QuotientGroup.induction_on z (fun u => ?_)
  change ((QuotientGroup.mk' sixthPowers) u)^2=1
  have hmem : u^2∈sixthPowers := ⟨u,sixth_eq_square u⟩
  have hq : (QuotientGroup.mk' sixthPowers) (u^2)=1 :=
    (QuotientGroup.eq_one_iff (u^2)).mpr hmem
  simpa only [map_pow] using hq

theorem target_nontrivial : (QuotientGroup.mk' sixthPowers) witness≠1 := by
  intro h
  obtain ⟨u,hu⟩ := (QuotientGroup.eq_one_iff witness).mp h
  have hn : ∀ u : U,u^6≠witness := by decide +kernel
  exact hn u hu

theorem no_left_nondegenerate_bilinear_pairing :
    ¬∃ e : Left→Right→Target,LeftBilinear e ∧ LeftNondegenerate e :=
  nonzero_double_obstructs target_square torsionP torsionP_double_ne_zero

theorem all_rational_points_killed (Q : curve.Point) : (6:ℕ) • Q=0 := by
  have h := addOrderOf_dvd_natCard Q
  rw [curve_cardinality] at h
  exact addOrderOf_dvd_iff_nsmul_eq_zero.mp h

theorem left_cardinality : Nat.card Left=6 :=
  (kernel_card_of_nsmul_all_zero 6 all_rational_points_killed).trans curve_cardinality

theorem right_cardinality : Nat.card Right=6 :=
  (right_card_of_nsmul_all_zero 6 all_rational_points_killed).trans curve_cardinality

theorem primitive_root_absent : ¬∃ u : U,IsPrimitiveRoot u 6 := by
  rintro ⟨u,hu⟩
  have h := hu.dvd_of_pow_eq_one 4 (unit_four u)
  norm_num at h

theorem primitive_two_control : IsPrimitiveRoot root 2 := by
  apply (IsPrimitiveRoot.iff (by decide : 0<2)).mpr
  constructor
  · decide +kernel
  · intro m hm hn
    have he : m=1 := by omega
    subst m
    decide +kernel

theorem no_left_nondegenerate_on_curve :
    ¬∃ e : Left→curve.Point→Target,LeftBilinear e ∧ LeftNondegenerate e :=
  nonzero_double_obstructs target_square torsionP torsionP_double_ne_zero

#print axioms P_order
#print axioms target_square
#print axioms no_left_nondegenerate_bilinear_pairing
end CompositeTateFiveAudit
