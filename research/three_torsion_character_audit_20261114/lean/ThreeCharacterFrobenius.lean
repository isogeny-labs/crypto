import ThreeCharacterModule

namespace ThreeTorsionCharacterAudit
noncomputable section
open Matrix Module ThreeTorsionRankAudit.General BKFrobeniusBridgeAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

theorem primitive_exponent {L : Type*} [Field L] (w : L) (hw : w^3=1) (hn : w≠1)
    (c : ZMod 3) (n : ℕ) (he : w^c.val=w^n) : c=(n:ZMod 3) := by
  have ho : orderOf w=3 := orderOf_eq_prime hw hn
  have hp : IsPrimitiveRoot w 3 := IsPrimitiveRoot.iff_orderOf.mpr ho
  have hm : w^n=w^(n%3) := by rw [←ho,pow_mod_orderOf]
  have hc : c.val=n%3 := hp.pow_inj (ZMod.val_lt c) (Nat.mod_lt _ (by decide)) (he.trans hm)
  apply ZMod.val_injective
  simpa only [ZMod.val_natCast] using hc

theorem third_coefficient_ne_zero {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0) (hΔ : W.Δ≠0) : W.a₃≠0 := by
  rcases W with ⟨a,a2,b,a4,a6⟩
  change a2=0 at h2
  change a4=0 at h4
  change a6=0 at h6
  subst a2
  subst a4
  subst a6
  intro hb
  change b=0 at hb
  apply hΔ
  change (curve a b).Δ=0
  rw [discriminant_value,hb,zero_pow (by decide : (3:ℕ)≠0),zero_mul]

variable {F L : Type*} [Field F] [Fintype F] [Field L] [Algebra F L]
  [DecidableEq F] [DecidableEq L] [IsAlgClosed L] [NeZero (3:L)]

omit [DecidableEq F] in
theorem actual_tate_frobenius_determinant (W : WeierstrassCurve.Affine F)
    (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0) (hb : W.a₃≠0) (hΔ : W.Δ≠0)
    (b : Basis (Fin 2) (ZMod 3) (ThreeTorsion (W.baseChange L))) :
    (frobeniusMatrix W b).det=(Fintype.card F : ZMod 3) := by
  have h2' : (W.baseChange L).a₂=0 := by change algebraMap F L W.a₂=0; rw [h2,map_zero]
  have h4' : (W.baseChange L).a₄=0 := by change algebraMap F L W.a₄=0; rw [h4,map_zero]
  have h6' : (W.baseChange L).a₆=0 := by change algebraMap F L W.a₆=0; rw [h6,map_zero]
  have hb' : (W.baseChange L).a₃≠0 := (map_ne_zero (algebraMap F L)).mpr hb
  have hd' : (W.baseChange L).Δ≠0 := by
    change (W.map (algebraMap F L)).Δ≠0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap F L)).mpr hΔ
  obtain ⟨c,⟨hN,hc0⟩,hn,hw,hrepr⟩ := model_character_basis (W.baseChange L) h2' h4' h6' hb' hd'
  have hfix : linearFrobenius W (c 0)=c 0 := by
    apply Subtype.ext
    change FrobeniusPointDescentAudit.pointFrobenius W (c 0).val=(c 0).val
    rw [hc0]
    change WeierstrassCurve.Affine.Point.some _ _ _=WeierstrassCurve.Affine.Point.some 0 0 hN
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    exact ⟨map_zero _,map_zero _⟩
  have h00 : frobeniusMatrix W c 0 0=1 := by
    rw [frobeniusMatrix,LinearMap.toMatrix_apply]
    rw [hfix]
    simp
  have h10 : frobeniusMatrix W c 1 0=0 := by
    rw [frobeniusMatrix,LinearMap.toMatrix_apply]
    rw [hfix]
    simp
  have he : (modelValue (W.baseChange L) (c 1).val)^(c.repr (linearFrobenius W (c 1)) 1).val=
      (modelValue (W.baseChange L) (c 1).val)^Fintype.card F := by
    rw [←hrepr]
    exact modelValue_frobenius W (c 1).val
  have h11 : frobeniusMatrix W c 1 1=(Fintype.card F : ZMod 3) := by
    rw [frobeniusMatrix,LinearMap.toMatrix_apply]
    exact primitive_exponent _ hw hn _ _ he
  have hdet : (frobeniusMatrix W c).det=(Fintype.card F : ZMod 3) := by
    rw [Matrix.det_fin_two,h00,h10,h11]
    ring
  exact (determinant_basis_independent W b c).trans hdet

omit [Fintype F] in
theorem rational_three_divides (W : WeierstrassCurve.Affine F)
    (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0) (hb : W.a₃≠0) :
    3 ∣ Nat.card W.Point := by
  rcases W with ⟨a,a2,b,a4,a6⟩
  change a2=0 at h2
  change a4=0 at h4
  change a6=0 at h6
  subst a2
  subst a4
  subst a6
  have ho : addOrderOf (origin a b hb)=3 := addOrderOf_eq_prime
    (origin_three a b hb) (origin_nonzero a b hb)
  rw [←ho]
  exact addOrderOf_dvd_natCard _

theorem actual_tate_extension_criterion {K : Type*} [Field K] [Fintype K]
    [Algebra F K] [Algebra K L] [IsScalarTower F K L] [DecidableEq K]
    (W : WeierstrassCurve.Affine F) (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0)
    (hb : W.a₃≠0) (hΔ : W.Δ≠0) (hq : (Fintype.card F : ZMod 3)≠1)
    (k : ℕ) (hc : Fintype.card K=Fintype.card F^k) :
    (∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (pointInclusion F K L W)) ↔ 3 ∣ Fintype.card F^k-1 := by
  let b := actualGeometricBasis (L:=L) W hΔ
  exact actual_extension_criterion_of_det W b (rational_three_divides W h2 h4 h6 hb) hq
    (actual_tate_frobenius_determinant W h2 h4 h6 hb hΔ b) k hc

#print axioms actual_tate_frobenius_determinant
#print axioms actual_tate_extension_criterion
end
end ThreeTorsionCharacterAudit
