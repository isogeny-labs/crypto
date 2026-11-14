import ThreeCharacterPoint

namespace ThreeTorsionCharacterAudit
noncomputable section
open Matrix Module ThreeTorsionRankAudit.General
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

theorem matrix_first_column : ∀ v : Fin 2 → ZMod 3, v≠0 →
    ∃ A : Matrix (Fin 2) (Fin 2) (ZMod 3), A.det≠0 ∧ A *ᵥ ![1,0]=v := by
  decide +kernel

theorem basis_with_first {V : Type*} [AddCommGroup V] [Module (ZMod 3) V]
    (b : Basis (Fin 2) (ZMod 3) V) (P : V) (hP : P≠0) :
    ∃ c : Basis (Fin 2) (ZMod 3) V, c 0=P := by
  have hv : b.equivFun P≠0 := by
    intro hz
    exact hP (b.equivFun.injective (hz.trans (map_zero b.equivFun).symm))
  obtain ⟨A,hdet,hA⟩ := matrix_first_column (b.equivFun P) hv
  let f : V →ₗ[ZMod 3] V := A.toLin b b
  have hu : IsUnit (LinearMap.toMatrix b b f).det := by
    rw [LinearMap.toMatrix_toLin]
    exact hdet.isUnit
  let e : V ≃ₗ[ZMod 3] V := LinearEquiv.ofIsUnitDet hu
  refine ⟨b.map e,?_⟩
  change e (b 0)=P
  apply b.equivFun.injective
  change (b.repr (A.toLin b b (b 0)) : Fin 2 → ZMod 3)=b.equivFun P
  rw [Matrix.repr_toLin]
  have hzero : (b.repr (b 0) : Fin 2 → ZMod 3)= ![1,0] := by
    ext i
    fin_cases i <;> simp
  rw [hzero]
  exact hA

theorem basis_second_not_first_line {V : Type*} [AddCommGroup V] [Module (ZMod 3) V]
    (b : Basis (Fin 2) (ZMod 3) V) :
    b 1≠0 ∧ b 1≠b 0 ∧ b 1≠-b 0 := by
  constructor
  · exact b.ne_zero 1
  constructor
  · exact b.injective.ne (by decide : (1:Fin 2)≠0)
  · intro h
    have hh := congrArg (fun P => b.repr P 1) h
    simp at hh

variable {K : Type*} [Field K] [DecidableEq K] [NeZero (3:K)] [IsAlgClosed K]

abbrev torsionOrigin (a b : K) (hb : b≠0) : ThreeTorsion (curve a b) :=
  ⟨origin a b hb,origin_three a b hb⟩

theorem actual_basis_origin (a b : K) (hb : b≠0) (hΔ : (curve a b).Δ≠0) :
    ∃ c : Basis (Fin 2) (ZMod 3) (ThreeTorsion (curve a b)), c 0=torsionOrigin a b hb := by
  apply basis_with_first (geometricThreeBasis (curve a b) hΔ)
  intro hz
  exact origin_nonzero a b hb (congrArg Subtype.val hz)

omit [IsAlgClosed K] in
theorem actual_second_value_nontrivial (a b : K) (hb : b≠0)
    (c : Basis (Fin 2) (ZMod 3) (ThreeTorsion (curve a b))) (hc : c 0=torsionOrigin a b hb) :
    pointValue a b (c 1).val≠1 := by
  intro hv
  have hh := (pointValue_kernel a b hb (c 1).val (c 1).property).mp hv
  obtain ⟨h0,h1,h2⟩ := basis_second_not_first_line c
  rcases hh with hh | hh | hh
  · exact h0 (Subtype.ext hh)
  · apply h1
    rw [hc]
    exact Subtype.ext hh
  · apply h2
    rw [hc]
    exact Subtype.ext hh

omit [IsAlgClosed K] [NeZero (3:K)] in
theorem zmod_smul_val (a b : K) (i : ZMod 3) (P : ThreeTorsion (curve a b)) :
    (i • P).val=i.val • P.val := rfl

omit [IsAlgClosed K] [NeZero (3:K)] in
theorem origin_scalar_value (a b : K) (hb : b≠0) (i : ZMod 3) :
    pointValue a b (i • torsionOrigin a b hb).val=1 := by
  fin_cases i
  · change pointValue a b ((0:ZMod 3) • torsionOrigin a b hb).val=1
    rw [zero_smul]
    rfl
  · change pointValue a b ((1:ZMod 3) • torsionOrigin a b hb).val=1
    simpa only [one_smul,torsionOrigin] using pointValue_origin a b hb
  · change pointValue a b ((2:ZMod 3) • torsionOrigin a b hb).val=1
    rw [show (2:ZMod 3)=-1 by decide,neg_one_smul]
    change pointValue a b (-origin a b hb)=1
    rw [pointValue_neg a b _ (origin_three a b hb),pointValue_origin,inv_one]

omit [IsAlgClosed K] [NeZero (3:K)] in
theorem basis_second_scalar_value (a b : K)
    (c : Basis (Fin 2) (ZMod 3) (ThreeTorsion (curve a b))) (j : ZMod 3) :
    pointValue a b (j • c 1).val=(pointValue a b (c 1).val)^j.val := by
  fin_cases j
  · change pointValue a b ((0:ZMod 3) • c 1).val=(pointValue a b (c 1).val)^0
    rw [zero_smul,pow_zero]
    rfl
  · change pointValue a b ((1:ZMod 3) • c 1).val=(pointValue a b (c 1).val)^1
    rw [one_smul,pow_one]
  · change pointValue a b ((2:ZMod 3) • c 1).val=(pointValue a b (c 1).val)^2
    have hw := pointValue_cube a b (c 1).val (c 1).property
    have hn : pointValue a b (c 1).val≠0 := by
      intro hz
      rw [hz,zero_pow (by decide : (3:ℕ)≠0)] at hw
      exact zero_ne_one hw
    rw [show (2:ZMod 3)=-1 by decide,neg_one_smul]
    change pointValue a b (-(c 1).val)=(pointValue a b (c 1).val)^2
    rw [pointValue_neg a b _ (c 1).property]
    field_simp
    linear_combination -hw

omit [IsAlgClosed K] in
theorem basis_second_scalar_nontrivial (a b : K) (hb : b≠0)
    (c : Basis (Fin 2) (ZMod 3) (ThreeTorsion (curve a b))) (hc : c 0=torsionOrigin a b hb)
    (j : ZMod 3) (hj : j≠0) : pointValue a b (j • c 1).val≠1 := by
  have hn := actual_second_value_nontrivial a b hb c hc
  fin_cases j
  · exact (hj rfl).elim
  · change pointValue a b ((1:ZMod 3) • c 1).val≠1
    simpa only [one_smul] using hn
  · change pointValue a b ((2:ZMod 3) • c 1).val≠1
    rw [show (2:ZMod 3)=-1 by decide,neg_one_smul]
    change pointValue a b (-(c 1).val)≠1
    rw [pointValue_neg a b _ (c 1).property]
    intro h
    apply hn
    simpa only [inv_inv,inv_one] using congrArg Inv.inv h

omit [IsAlgClosed K] in
theorem basis_character_coordinates (a b : K) (hb : b≠0)
    (c : Basis (Fin 2) (ZMod 3) (ThreeTorsion (curve a b))) (hc : c 0=torsionOrigin a b hb)
    (i j : ZMod 3) :
    pointValue a b (i • c 0+j • c 1).val=(pointValue a b (c 1).val)^j.val := by
  by_cases hj : j=0
  · simp only [hj,zero_smul,add_zero]
    rw [hc,origin_scalar_value]
    simp
  · have hn := basis_second_scalar_nontrivial a b hb c hc j hj
    change pointValue a b ((i • c 0).val+(j • c 1).val)=_
    rw [hc,zmod_smul_val,add_comm]
    change pointValue a b ((j • c 1).val+i.val • origin a b hb)=_
    rw [pointValue_add_origin_nsmul a b hb _ (j • c 1).property hn]
    exact basis_second_scalar_value a b c j

omit [IsAlgClosed K] in
theorem basis_character_repr (a b : K) (hb : b≠0)
    (c : Basis (Fin 2) (ZMod 3) (ThreeTorsion (curve a b))) (hc : c 0=torsionOrigin a b hb)
    (Q : ThreeTorsion (curve a b)) :
    pointValue a b Q.val=(pointValue a b (c 1).val)^(c.repr Q 1).val := by
  have hr : (c.repr Q 0) • c 0+(c.repr Q 1) • c 1=Q := by
    simpa only [Fin.sum_univ_two] using c.sum_repr Q
  calc pointValue a b Q.val=pointValue a b ((c.repr Q 0) • c 0+(c.repr Q 1) • c 1).val :=
         congrArg (fun T : ThreeTorsion (curve a b) => pointValue a b T.val) hr.symm
       _=(pointValue a b (c 1).val)^(c.repr Q 1).val := basis_character_coordinates a b hb c hc _ _

theorem model_character_basis (W : WeierstrassCurve.Affine K)
    (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0) (hb : W.a₃≠0) (hΔ : W.Δ≠0) :
    ∃ c : Basis (Fin 2) (ZMod 3) (ThreeTorsion W),
      (∃ hN : W.Nonsingular 0 0, (c 0).val=WeierstrassCurve.Affine.Point.some 0 0 hN) ∧
      modelValue W (c 1).val≠1 ∧
      (modelValue W (c 1).val)^3=1 ∧
      ∀ Q : ThreeTorsion W, modelValue W Q.val=(modelValue W (c 1).val)^(c.repr Q 1).val := by
  rcases W with ⟨a,a2,b,a4,a6⟩
  change a2=0 at h2
  change a4=0 at h4
  change a6=0 at h6
  subst a2
  subst a4
  subst a6
  obtain ⟨c,hc⟩ := actual_basis_origin a b hb hΔ
  refine ⟨c,⟨origin_nonsingular a b hb,?_⟩,actual_second_value_nontrivial a b hb c hc,
    pointValue_cube a b (c 1).val (c 1).property,basis_character_repr a b hb c hc⟩
  exact congrArg Subtype.val hc

#print axioms basis_with_first
#print axioms actual_basis_origin
#print axioms actual_second_value_nontrivial
end
end ThreeTorsionCharacterAudit
