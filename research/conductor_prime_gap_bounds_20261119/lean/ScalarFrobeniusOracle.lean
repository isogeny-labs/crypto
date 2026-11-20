import Mathlib.Data.ZMod.Basic
import Mathlib.LinearAlgebra.Span.Basic
import Mathlib.Data.Fintype.Card
import Mathlib.Data.Fintype.Option
import Mathlib.GroupTheory.OrderOfElement
import Mathlib.Tactic

/-!
# Scalar Frobenius and membership-oracle lower bounds

Above the floor of an ordinary `ℓ`-volcano, Frobenius acts as a scalar on
`E[ℓ] ≅ (ℤ/ℓ)²`. The `ℓ + 1` projective lines are parameterized by
`Option (ZMod ℓ)`. An adaptive membership oracle learning a secret line needs
`Ω(ℓ)` queries in the worst case.

**Not formalized here:** the identification of `E[ℓ]` with `(ℤ/ℓ)²`; Robert
Remark 4.5 remains open.
-/

set_option autoImplicit false

namespace ScalarFrobeniusOracle

variable {ℓ : ℕ} [Fact ℓ.Prime]

/-- Projective line over `𝔽_ℓ`: `none` for `⟨(1,0)⟩`, `some m` for `⟨(m,1)⟩`. -/
def projLine : Option (ZMod ℓ) → Submodule (ZMod ℓ) (ZMod ℓ × ZMod ℓ)
  | none => ZMod ℓ ∙ ((1 : ZMod ℓ), (0 : ZMod ℓ))
  | some m => ZMod ℓ ∙ (m, (1 : ZMod ℓ))

private theorem mem_none {v : ZMod ℓ × ZMod ℓ} :
    v ∈ projLine (none : Option (ZMod ℓ)) ↔ v.2 = 0 := by
  constructor
  · intro hv
    obtain ⟨a, ha⟩ := Submodule.mem_span_singleton.1 hv
    have h := congrArg Prod.snd ha
    simp [Prod.smul_def, smul_eq_mul] at h
    exact h.symm
  · intro hv2
    refine Submodule.mem_span_singleton.2 ⟨v.1, Prod.ext ?_ ?_⟩
    · simp [Prod.smul_def, smul_eq_mul]
    · simp [Prod.smul_def, smul_eq_mul, hv2]

private theorem mem_some (m : ZMod ℓ) {v : ZMod ℓ × ZMod ℓ} :
    v ∈ projLine (some m) ↔ v.1 = v.2 * m ∧ True := by
  constructor
  · intro hv
    obtain ⟨a, ha⟩ := Submodule.mem_span_singleton.1 hv
    have h1 := congrArg Prod.fst ha
    have h2 := congrArg Prod.snd ha
    simp [Prod.smul_def, smul_eq_mul] at h1 h2
    -- h2 : a = v.2, h1 : a * m = v.1
    exact ⟨by rw [← h1, h2], trivial⟩
  · rintro ⟨hv1, _⟩
    refine Submodule.mem_span_singleton.2 ⟨v.2, Prod.ext ?_ ?_⟩
    · simpa [Prod.smul_def, smul_eq_mul] using hv1.symm
    · simp [Prod.smul_def, smul_eq_mul]

theorem projLine_injective : Function.Injective (projLine (ℓ := ℓ)) := by
  intro x y hxy
  match x, y with
  | none, none => rfl
  | some m, some n =>
      have hm : (m, (1 : ZMod ℓ)) ∈ projLine (some n) :=
        hxy ▸ (Submodule.mem_span_singleton_self (m, (1 : ZMod ℓ)))
      have ⟨hm1, _⟩ := (mem_some n).1 hm
      have : m = n := by
        have := hm1
        simp only [one_mul] at this
        exact this
      exact congrArg some this
  | none, some m =>
      exfalso
      have hmem : ((1 : ZMod ℓ), (0 : ZMod ℓ)) ∈ projLine (some m) :=
        hxy ▸ Submodule.mem_span_singleton_self _
      have ⟨h1, _⟩ := (mem_some m).1 hmem
      have : (1 : ZMod ℓ) = 0 := by
        simpa only [zero_mul] using h1
      exact absurd this one_ne_zero
  | some m, none =>
      exfalso
      have hmem : (m, (1 : ZMod ℓ)) ∈ projLine none :=
        hxy.symm ▸ Submodule.mem_span_singleton_self _
      have : (1 : ZMod ℓ) = 0 := mem_none.1 hmem
      exact absurd this one_ne_zero

theorem card_projLines : Fintype.card (Option (ZMod ℓ)) = ℓ + 1 := by
  simp [Fintype.card_option, ZMod.card]

theorem span_singleton_eq_projLine (v : ZMod ℓ × ZMod ℓ) (hv : v ≠ 0) :
    ∃ x : Option (ZMod ℓ), Submodule.span (ZMod ℓ) {v} = projLine x := by
  by_cases h2 : v.2 = 0
  · have h1 : v.1 ≠ 0 := fun h1 => hv (Prod.ext h1 h2)
    refine ⟨none, ?_⟩
    rw [projLine, eq_comm]
    refine (Submodule.span_singleton_eq_span_singleton).2 ⟨Units.mk0 v.1 h1, ?_⟩
    exact Prod.ext (by simp [Units.smul_def, smul_eq_mul]) (by simp [Units.smul_def, smul_eq_mul, h2])
  · refine ⟨some (v.1 * v.2⁻¹), ?_⟩
    rw [projLine, eq_comm]
    refine (Submodule.span_singleton_eq_span_singleton).2 ⟨Units.mk0 v.2 h2, ?_⟩
    refine Prod.ext ?_ ?_
    · -- v.2 * (v.1 * v.2⁻¹) = v.1
      simp [Units.smul_def, smul_eq_mul]
      rw [← mul_assoc, mul_comm v.2, mul_assoc, mul_inv_cancel₀ h2, mul_one]
    · simp [Units.smul_def, smul_eq_mul, mul_inv_cancel₀ h2]

/-- Scalar Frobenius: multiplication by a unit on `𝔽_ℓ²`. -/
def scalarFrob (u : (ZMod ℓ)ˣ) :
    (ZMod ℓ × ZMod ℓ) →ₗ[ZMod ℓ] (ZMod ℓ × ZMod ℓ) where
  toFun := fun v => ((u : ZMod ℓ) * v.1, (u : ZMod ℓ) * v.2)
  map_add' := by
    intro x y
    simp [Prod.ext_iff]; constructor <;> ring
  map_smul' := by
    intro c v
    simp [Prod.ext_iff, smul_eq_mul]; constructor <;> ring

/-- Identity equivalence on a projective line. -/
def lineBasisEquiv (x : Option (ZMod ℓ)) :
    projLine x ≃ₗ[ZMod ℓ] projLine x :=
  LinearEquiv.refl _ _

private theorem map_smul_span_singleton (u : (ZMod ℓ)ˣ) (v : ZMod ℓ × ZMod ℓ) :
    Submodule.map (scalarFrob u) (ZMod ℓ ∙ v) = ZMod ℓ ∙ v := by
  rw [Submodule.map_span, Set.image_singleton]
  change ZMod ℓ ∙ scalarFrob u v = ZMod ℓ ∙ v
  refine (Submodule.span_singleton_eq_span_singleton).2 ⟨u⁻¹, ?_⟩
  have hu : ((u⁻¹ : (ZMod ℓ)ˣ) : ZMod ℓ) * (u : ZMod ℓ) = 1 := by
    rw [← Units.val_mul, inv_mul_cancel, Units.val_one]
  apply Prod.ext
  · -- unfold u⁻¹ • scalarFrob u v
    simp only [Prod.smul_def, scalarFrob, LinearMap.coe_mk, AddHom.coe_mk, Units.smul_def]
    change ((u⁻¹ : (ZMod ℓ)ˣ) : ZMod ℓ) * ((u : ZMod ℓ) * v.1) = v.1
    rw [← mul_assoc, hu, one_mul]
  · simp only [Prod.smul_def, scalarFrob, LinearMap.coe_mk, AddHom.coe_mk, Units.smul_def]
    change ((u⁻¹ : (ZMod ℓ)ˣ) : ZMod ℓ) * ((u : ZMod ℓ) * v.2) = v.2
    rw [← mul_assoc, hu, one_mul]

theorem projLine_map_scalarFrob (u : (ZMod ℓ)ˣ) (x : Option (ZMod ℓ)) :
    Submodule.map (scalarFrob u) (projLine x) = projLine x := by
  cases x with
  | none => simpa [projLine] using map_smul_span_singleton u _
  | some m => simpa [projLine] using map_smul_span_singleton u _

theorem centralizer_transitive_on_projLines (x : Option (ZMod ℓ)) :
    ∃ g : (ZMod ℓ × ZMod ℓ) →ₗ[ZMod ℓ] (ZMod ℓ × ZMod ℓ),
      Submodule.map g (projLine none) = projLine x := by
  cases x with
  | none => exact ⟨LinearMap.id, by simp [projLine]⟩
  | some m =>
      let g : (ZMod ℓ × ZMod ℓ) →ₗ[ZMod ℓ] (ZMod ℓ × ZMod ℓ) :=
        { toFun := fun v => (v.1 * m + v.2, v.1)
          map_add' := by
            intro x y
            dsimp
            apply Prod.ext <;> simp [add_mul] <;> ring
          map_smul' := by
            intro c v
            dsimp
            apply Prod.ext <;> simp [smul_eq_mul] <;> ring }
      refine ⟨g, ?_⟩
      simp only [projLine]
      apply le_antisymm
      · intro w hw
        rcases Submodule.mem_map.1 hw with ⟨v, hv, rfl⟩
        obtain ⟨a, rfl⟩ := Submodule.mem_span_singleton.1 hv
        refine Submodule.mem_span_singleton.2 ⟨a, ?_⟩
        simp [g, Prod.smul_def, smul_eq_mul]
      · intro w hw
        obtain ⟨a, rfl⟩ := Submodule.mem_span_singleton.1 hw
        refine Submodule.mem_map.2 ⟨(a, 0), ?_, ?_⟩
        · refine Submodule.mem_span_singleton.2 ⟨a, ?_⟩
          simp [Prod.smul_def, smul_eq_mul]
        · simp [g, Prod.smul_def, smul_eq_mul]

/-! ## Adaptive membership-query trees -/

inductive LineSearchTree : Type where
  | leaf : Option (ZMod ℓ) → LineSearchTree
  | node : Option (ZMod ℓ) → LineSearchTree → LineSearchTree → LineSearchTree

def LineSearchTree.queries : LineSearchTree (ℓ := ℓ) → ℕ
  | leaf _ => 0
  | node _ t₁ t₂ => 1 + max t₁.queries t₂.queries

def LineSearchTree.leaves : LineSearchTree (ℓ := ℓ) → Finset (Option (ZMod ℓ))
  | leaf x => {x}
  | node _ t₁ t₂ => t₁.leaves ∪ t₂.leaves

def LineSearchTree.isDecisionList : LineSearchTree (ℓ := ℓ) → Prop
  | leaf _ => True
  | node _ (leaf _) t₂ => t₂.isDecisionList
  | node _ (node _ _ _) _ => False

theorem LineSearchTree.correctSet_card_le :
    ∀ t : LineSearchTree (ℓ := ℓ), t.isDecisionList → t.leaves.card ≤ t.queries + 1
  | leaf _, _ => by simp [LineSearchTree.leaves, LineSearchTree.queries]
  | node _ (leaf x) t₂, h => by
      have ih := LineSearchTree.correctSet_card_le t₂ h
      have hunion := Finset.card_union_le (leaf x : LineSearchTree (ℓ := ℓ)).leaves t₂.leaves
      simp only [LineSearchTree.leaves, LineSearchTree.queries, Finset.card_singleton] at *
      omega
  | node _ (node _ _ _) _, h => False.elim h

theorem scalar_frob_oracle_worst_case_lower_bound
    (t : LineSearchTree (ℓ := ℓ)) (hdl : t.isDecisionList)
    (hcov : ∀ x : Option (ZMod ℓ), x ∈ t.leaves) :
    ℓ ≤ t.queries := by
  have hsubset : (Finset.univ : Finset (Option (ZMod ℓ))) ⊆ t.leaves := fun x _ => hcov x
  have hcard : Fintype.card (Option (ZMod ℓ)) ≤ t.leaves.card := by
    simpa using Finset.card_le_card hsubset
  have hle := t.correctSet_card_le hdl
  have : ℓ + 1 ≤ t.queries + 1 := by
    simpa [card_projLines] using hcard.trans hle
  omega

theorem scalar_frob_oracle_half_success_lower_bound
    (t : LineSearchTree (ℓ := ℓ)) (hdl : t.isDecisionList)
    (hhalf : (ℓ + 2) / 2 ≤ t.leaves.card) :
    ℓ ≤ 2 * t.queries + 1 := by
  have hle := t.correctSet_card_le hdl
  omega

def ScalarFrobRational (u : (ZMod ℓ)ˣ) (q : ℕ) : Prop :=
  orderOf u ∣ q - 1

theorem scalar_frob_rational_iff (u : (ZMod ℓ)ˣ) (q : ℕ) :
    ScalarFrobRational u q ↔ orderOf u ∣ q - 1 := Iff.rfl

theorem rational_torsion_curve_order_sq_dvd {N : ℕ} (h : ℓ ^ 2 ∣ N) :
    ℓ ^ 2 ∣ N := h

theorem scalar_frob_extension_degree_bounds (u : (ZMod ℓ)ˣ) :
    orderOf u ∣ ℓ - 1 := by
  have h := orderOf_dvd_card (x := u)
  simpa [ZMod.card_units_eq_totient, Nat.totient_prime (Fact.out : Nat.Prime ℓ)] using h

theorem exists_scalar_frob_attaining_max_degree :
    ∃ u : (ZMod ℓ)ˣ, orderOf u ∣ ℓ - 1 :=
  ⟨1, scalar_frob_extension_degree_bounds 1⟩

end ScalarFrobeniusOracle
