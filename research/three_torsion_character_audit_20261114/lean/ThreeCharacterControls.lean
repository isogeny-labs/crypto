import ThreeCharacterFrobenius

namespace ThreeTorsionCharacterAudit
noncomputable section
open ThreeTorsionRankAudit.General WeierstrassCurve.Affine
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

theorem singular_origin (K : Type*) [Field K] :
    (curve (1:K) 0).Equation 0 0 ∧ ¬(curve (1:K) 0).Nonsingular 0 0 := by
  constructor
  · simp [WeierstrassCurve.Affine.equation_iff,curve]
  · simp [WeierstrassCurve.Affine.nonsingular_iff',curve]

theorem nonzero_b_singular_curve (K : Type*) [Field K] :
    (1:K)≠0 ∧ (curve (3:K) 1).Δ=0 ∧ (curve (3:K) 1).Nonsingular 0 0 := by
  refine ⟨one_ne_zero,?_,origin_nonsingular 3 1 one_ne_zero⟩
  rw [discriminant_value]
  norm_num

section CharacteristicThree
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 3]

omit [DecidableEq K] in
theorem ordinary_discriminant : (curve (1:K) 1).Δ≠0 := by
  rw [discriminant_value]
  have h3 : (3:K)=0 := CharP.cast_eq_zero K 3
  have h27 : (27:K)=0 := by linear_combination 9*h3
  simp [h27]

omit [DecidableEq K] in
theorem ordinary_division (x : K) : (curve (1:K) 1).Ψ₃.eval x=x^3 := by
  rw [division_value]
  have h3 : (3:K)=0 := CharP.cast_eq_zero K 3
  simp only [cubic,h3,zero_mul,zero_add,add_zero,one_pow,one_mul]
  ring

theorem ordinary_coordinates (x y : K) (hN : (curve (1:K) 1).Nonsingular x y)
    (ht : (3:ℕ) • Point.some x y hN=0) : x=0 ∧ (y=0 ∨ y=-1) := by
  have hx : x=0 := by
    have hp := (ThreeTorsionRankAudit.General.three_point_iff (curve (1:K) 1) hN).mp ht
    have hv : (curve (1:K) 1).Ψ₃.eval x=0 := hp
    rw [ordinary_division] at hv
    by_contra hn
    exact (pow_ne_zero 3 hn) hv
  refine ⟨hx,?_⟩
  have he : y*(y+1)=0 := by
    have h := hN.1
    rw [WeierstrassCurve.Affine.equation_iff] at h
    simp only [curve,hx,mul_zero,zero_mul,zero_pow (by decide : (3:ℕ)≠0),
      zero_pow (by decide : (2:ℕ)≠0),add_zero,one_mul] at h
    linear_combination h
  exact (mul_eq_zero.mp he).imp id eq_neg_of_add_eq_zero_left

def ordinaryCode : ThreeTorsion (curve (1:K) 1) → Option Bool
  | ⟨.zero,_⟩ => none
  | ⟨.some _ y _,_⟩ => some (if y=0 then false else true)

omit [DecidableEq K] [CharP K 3] in
theorem ordinary_minus_nonsingular : (curve (1:K) 1).Nonsingular 0 (-1) := by
  rw [WeierstrassCurve.Affine.nonsingular_iff']
  constructor
  · norm_num [WeierstrassCurve.Affine.equation_iff,curve]
  · right
    norm_num [curve]

omit [CharP K 3] in
theorem ordinary_minus_three : (3:ℕ) • Point.some 0 (-1) (ordinary_minus_nonsingular (K:=K))=0 := by
  apply (ThreeTorsionRankAudit.General.three_point_iff (curve (1:K) 1) ordinary_minus_nonsingular).mpr
  change (curve (1:K) 1).Ψ₃.eval 0=0
  rw [division_value,zero_mul]

def ordinaryPoint : Option Bool → ThreeTorsion (curve (1:K) 1)
  | none => 0
  | some false => torsionOrigin 1 1 one_ne_zero
  | some true => ⟨.some 0 (-1) ordinary_minus_nonsingular,ordinary_minus_three⟩

def ordinaryTorsionEquiv : ThreeTorsion (curve (1:K) 1) ≃ Option Bool where
  toFun := ordinaryCode
  invFun := ordinaryPoint
  left_inv P := by
    rcases P with ⟨P,hP⟩
    cases P with
    | zero => rfl
    | some x y hN =>
      obtain ⟨hx,hy⟩ := ordinary_coordinates x y hN hP
      rcases hy with hy | hy
      · subst x
        subst y
        apply Subtype.ext
        simp [ordinaryCode,ordinaryPoint,torsionOrigin,origin]
      · subst x
        subst y
        apply Subtype.ext
        simp [ordinaryCode,ordinaryPoint]
  right_inv Q := by
    cases Q with
    | none => rfl
    | some b => cases b <;> simp [ordinaryPoint,ordinaryCode,torsionOrigin,origin]

theorem ordinary_three_card : Nat.card (ThreeTorsion (curve (1:K) 1))=3 := by
  rw [Nat.card_congr ordinaryTorsionEquiv]
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem ordinary_three_finrank : Module.finrank (ZMod 3) (ThreeTorsion (curve (1:K) 1))=1 := by
  let : Finite (ThreeTorsion (curve (1:K) 1)) := Finite.of_equiv (Option Bool) ordinaryTorsionEquiv.symm
  have hc := Module.natCard_eq_pow_finrank (K:=ZMod 3) (V:=ThreeTorsion (curve (1:K) 1))
  rw [ordinary_three_card] at hc
  have hn : Nat.card (ZMod 3)=3 := by simp
  rw [hn] at hc
  apply Nat.pow_right_injective (by decide : 2≤3)
  exact hc.symm
end CharacteristicThree

abbrev OrdinaryClosure := AlgebraicClosure (ZMod 3)
private instance : DecidableEq OrdinaryClosure := Classical.decEq _
private instance : CharP OrdinaryClosure 3 := charP_of_injective_algebraMap' (ZMod 3) 3
def ordinaryBase : WeierstrassCurve.Affine (ZMod 3) := curve 1 1

theorem ordinary_baseChange_eq {K : Type*} [Field K] [Algebra (ZMod 3) K] :
    ordinaryBase.baseChange K=curve (1:K) 1 := by
  ext <;> simp [ordinaryBase,curve,WeierstrassCurve.baseChange,WeierstrassCurve.map]

theorem ordinary_model_coordinates {K : Type*} [Field K] [DecidableEq K] [CharP K 3]
    (W : WeierstrassCurve.Affine K) (hW : W=curve (1:K) 1) (x y : K)
    (hN : W.Nonsingular x y) (ht : (3:ℕ) • Point.some x y hN=0) : x=0 ∧ (y=0 ∨ y=-1) := by
  subst W
  exact ordinary_coordinates x y hN ht

theorem ordinary_all_torsion_in_extension {K : Type*} [Field K] [Algebra (ZMod 3) K]
    [Algebra K OrdinaryClosure] [IsScalarTower (ZMod 3) K OrdinaryClosure] [DecidableEq K] :
    ∀ P : (ordinaryBase.baseChange OrdinaryClosure).Point, (3:ℕ) • P=0 →
      P∈Set.range (BKFrobeniusBridgeAudit.pointInclusion (ZMod 3) K OrdinaryClosure ordinaryBase) := by
  intro P ht
  cases P with
  | zero => exact ⟨0,rfl⟩
  | some x y hN =>
    obtain ⟨hx,hy⟩ := ordinary_model_coordinates _ ordinary_baseChange_eq x y hN ht
    have hn0 : (ordinaryBase.baseChange K).Nonsingular 0 0 := by
      rw [ordinary_baseChange_eq]
      exact origin_nonsingular 1 1 one_ne_zero
    have hn1 : (ordinaryBase.baseChange K).Nonsingular 0 (-1) := by
      rw [ordinary_baseChange_eq]
      exact ordinary_minus_nonsingular
    rcases hy with hy | hy
    · refine ⟨.some 0 0 hn0,?_⟩
      change Point.some ((Algebra.ofId K OrdinaryClosure) 0) ((Algebra.ofId K OrdinaryClosure) 0) _=Point.some x y hN
      rw [Point.some.injEq]
      exact ⟨(map_zero _).trans hx.symm,(map_zero _).trans hy.symm⟩
    · refine ⟨.some 0 (-1) hn1,?_⟩
      change Point.some ((Algebra.ofId K OrdinaryClosure) 0) ((Algebra.ofId K OrdinaryClosure) (-1)) _=Point.some x y hN
      rw [Point.some.injEq]
      constructor
      · exact (map_zero _).trans hx.symm
      · rw [map_neg,map_one]
        exact hy.symm

theorem characteristic_three_cyclotomic_failure (k : ℕ) (hk : 0<k) : ¬3 ∣ 3^k-1 := by
  intro hd
  have hp : 1≤3^k := pow_pos (by decide : (0:ℕ)<3) k
  have hz : ((3^k-1:ℕ):ZMod 3)=0 := (ZMod.natCast_eq_zero_iff _ _).mpr hd
  rw [Nat.cast_sub hp,Nat.cast_pow] at hz
  norm_num only [Nat.cast_ofNat,Nat.cast_one] at hz
  have h3 : (3:ZMod 3)=0 := by decide +kernel
  rw [h3,zero_pow (Nat.ne_of_gt hk),zero_sub] at hz
  exact neg_ne_zero.mpr one_ne_zero hz

theorem deleting_coprimality_inclusion_false :
    ¬((∀ P : (ordinaryBase.baseChange OrdinaryClosure).Point, (3:ℕ) • P=0 →
      P∈Set.range (BKFrobeniusBridgeAudit.pointInclusion (ZMod 3) (ZMod 3) OrdinaryClosure ordinaryBase)) ↔
        3 ∣ 3^1-1) := by
  intro h
  exact characteristic_three_cyclotomic_failure 1 (by decide)
    (h.mp (ordinary_all_torsion_in_extension (K:=ZMod 3)))

theorem actual_char_three_rank_control : ordinaryBase.Δ≠0 ∧
    Nat.card (ThreeTorsion (curve (1:OrdinaryClosure) 1))=3 ∧
    Module.finrank (ZMod 3) (ThreeTorsion (curve (1:OrdinaryClosure) 1))=1 :=
  ⟨ordinary_discriminant,ordinary_three_card,ordinary_three_finrank⟩

#print axioms ordinary_three_card
#print axioms ordinary_three_finrank
#print axioms actual_char_three_rank_control
end
end ThreeTorsionCharacterAudit
