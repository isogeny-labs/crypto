import BilinearObstruction

namespace CompositeTateThreeAudit
open CompositeTateObstructionAudit
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev F := ZMod 3
def curve : WeierstrassCurve.Affine F := ⟨0,0,0,1,1⟩

theorem curve_nonsingular : curve.Δ≠0 := by
  unfold curve WeierstrassCurve.Δ WeierstrassCurve.b₂ WeierstrassCurve.b₄
    WeierstrassCurve.b₆ WeierstrassCurve.b₈
  decide +kernel

theorem curve_cardinality : Nat.card curve.Point=4 := by
  rw [Nat.card_congr curve.nonsingularPointEquiv]
  change Nat.card (Option {xy : F×F // curve.Nonsingular xy.1 xy.2})=4
  let : DecidablePred (fun xy : F×F => curve.Nonsingular xy.1 xy.2) :=
    fun xy => decidable_of_iff (xy.2^2=xy.1^3+xy.1+1) (by
      change xy.2^2=xy.1^3+xy.1+1 ↔ curve.Nonsingular xy.1 xy.2
      erw [←WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
        curve_nonsingular,WeierstrassCurve.Affine.equation_iff]
      simp [curve])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

def P : curve.Point := .some 0 1 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)
def T : curve.Point := .some 1 0 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

theorem P_double : P+P=T := by decide +kernel
theorem T_double : T+T=0 := by decide +kernel
theorem T_ne_zero : T≠0 := by decide +kernel
theorem P_four : (4:ℕ) • P=0 := by
  simp only [show (4:ℕ)=2+2 by decide,add_nsmul,two_nsmul,P_double,T_double]
theorem P_order : addOrderOf P=4 := by
  apply addOrderOf_eq_prime_pow (p:=2) (n:=1)
  · simpa only [pow_one,two_nsmul,P_double] using T_ne_zero
  · simpa only [show (2:ℕ)^(1+1)=4 by decide] using P_four

def fourTorsion : AddSubgroup curve.Point :=
  (nsmulAddMonoidHom 4 : curve.Point→+curve.Point).ker
abbrev Left := ↥fourTorsion
def fourMultiples : AddSubgroup curve.Point :=
  (nsmulAddMonoidHom 4 : curve.Point→+curve.Point).range
abbrev Right := curve.Point ⧸ fourMultiples

def torsionP : Left := ⟨P,by change (4:ℕ) • P=0;exact P_four⟩
theorem torsionP_double_ne_zero : torsionP+torsionP≠0 := by
  intro h
  have hh := congrArg Subtype.val h
  change P+P=0 at hh
  rw [P_double] at hh
  exact T_ne_zero hh

abbrev U := Fˣ
def fourthPowers : Subgroup U := (powMonoidHom 4 : U→*U).range
abbrev Target := U ⧸ fourthPowers

def root : U := Units.mk0 (2:F) (by decide +kernel)
theorem source_hypotheses : Nat.Coprime 4 3 ∧ 4∣Nat.card curve.Point ∧
    root^4=1 ∧ root≠1 := by
  rw [curve_cardinality]
  decide +kernel

theorem unit_square : ∀ u : U,u^2=1 := by decide +kernel

theorem target_square (z : Target) : z^2=1 := by
  refine QuotientGroup.induction_on z (fun u => ?_)
  change ((QuotientGroup.mk' fourthPowers) u)^2=1
  simpa only [map_pow,map_one] using congrArg (QuotientGroup.mk' fourthPowers) (unit_square u)

theorem target_nontrivial : (QuotientGroup.mk' fourthPowers) root≠1 := by
  intro h
  obtain ⟨u,hu⟩ := (QuotientGroup.eq_one_iff root).mp h
  have hn : ∀ u : U,u^4≠root := by decide +kernel
  exact hn u hu

theorem no_left_nondegenerate_bilinear_pairing :
    ¬∃ e : Left→Right→Target,LeftBilinear e ∧ LeftNondegenerate e :=
  nonzero_double_obstructs target_square torsionP torsionP_double_ne_zero

theorem all_rational_points_killed (Q : curve.Point) : (4:ℕ) • Q=0 := by
  have h := addOrderOf_dvd_natCard Q
  rw [curve_cardinality] at h
  exact addOrderOf_dvd_iff_nsmul_eq_zero.mp h

theorem left_cardinality : Nat.card Left=4 :=
  (kernel_card_of_nsmul_all_zero 4 all_rational_points_killed).trans curve_cardinality

theorem right_cardinality : Nat.card Right=4 :=
  (right_card_of_nsmul_all_zero 4 all_rational_points_killed).trans curve_cardinality

theorem primitive_root_absent : ¬∃ u : U,IsPrimitiveRoot u 4 := by
  rintro ⟨u,hu⟩
  have h := hu.dvd_of_pow_eq_one 2 (unit_square u)
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
end CompositeTateThreeAudit
