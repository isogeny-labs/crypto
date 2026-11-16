module
public import Mathlib.RingTheory.AdjoinRoot
public import Mathlib.RingTheory.PowerBasis
public import Mathlib.RepresentationTheory.Submodule
public import Mathlib.GroupTheory.SpecificGroups.Cyclic
public import Mathlib.Tactic

@[expose] public section
namespace DiemCyclicModuleAudit
noncomputable section
open Polynomial
open scoped MonoidAlgebra

variable {K : Type*} [Field K] (f : K[X])

/-- The actual polynomial quotient, with no irreducibility assumption. -/
abbrev Quotient := AdjoinRoot f

def root : Quotient f := AdjoinRoot.root f

theorem quotient_dimension (hf : f.Monic) :
    Module.finrank K (Quotient f) = f.natDegree := by
  rw [Module.finrank_eq_card_basis (AdjoinRoot.powerBasis' hf).basis]
  simp

theorem root_power_eq_one_iff (n : ℕ) : root f ^ n = 1 ↔ f ∣ X ^ n - 1 := by
  have h : AdjoinRoot.mk f (X ^ n - 1) = root f ^ n - 1 := by
    simp [root]
  rw [← sub_eq_zero, ← h, AdjoinRoot.mk_eq_zero]

/-- A root which is an actual unit supplies an actual cyclic group and representation. -/
def cyclicRepresentation (u : (Quotient f)ˣ) :
    Representation K (Subgroup.zpowers u) (Quotient f) where
  toFun g := LinearMap.mul K (Quotient f) (g.val : Quotient f)
  map_one' := by ext x; simp
  map_mul' g h := by ext x; simp [mul_assoc]

theorem cyclicRepresentation_faithful (u : (Quotient f)ˣ) :
    Function.Injective (cyclicRepresentation f u) := by
  intro g h he
  apply Subtype.ext
  apply Units.ext
  have hh := congrArg (fun T : Module.End K (Quotient f) => T 1) he
  simpa [cyclicRepresentation] using hh

/-- Stability under the root propagates to every actual quotient element. -/
theorem mul_mem_of_root_invariant (U : Submodule K (Quotient f))
    (hU : ∀ a ∈ U, root f * a ∈ U) {a : Quotient f} (ha : a ∈ U)
    (b : Quotient f) : b * a ∈ U := by
  induction b using AdjoinRoot.induction_on with
  | ih p =>
    induction p using Polynomial.induction_on' with
    | add p q hp hq => simpa [map_add, add_mul] using U.add_mem hp hq
    | monomial n c =>
      have hn : root f ^ n * a ∈ U := by
        induction n with
        | zero => simpa using ha
        | succ n ih => simpa [pow_succ, mul_assoc, mul_comm, mul_left_comm] using hU _ ih
      simpa [← Polynomial.C_mul_X_pow_eq_monomial, root, Algebra.smul_def, mul_assoc]
        using U.smul_mem c hn

/-- Cyclicity is derived from the actual quotient, not assumed as a premise. -/
theorem orbit_spans (u : (Quotient f)ˣ) (hu : (u : Quotient f) = root f) :
    Submodule.span K (Set.range (fun g : Subgroup.zpowers u => cyclicRepresentation f u g 1)) = ⊤ := by
  let U := Submodule.span K
    (Set.range (fun g : Subgroup.zpowers u => cyclicRepresentation f u g 1))
  have hp (n : ℕ) : root f ^ n ∈ U := by
    have hm : u ^ n ∈ Subgroup.zpowers u :=
      Subgroup.pow_mem _ (Subgroup.mem_zpowers u) n
    have hs : cyclicRepresentation f u ⟨u ^ n, hm⟩ 1 ∈ U :=
      Submodule.subset_span (Set.mem_range_self (⟨u ^ n, hm⟩ : Subgroup.zpowers u))
    simpa [cyclicRepresentation, hu] using hs
  change U = ⊤
  apply Submodule.eq_top_iff'.mpr
  intro b
  induction b using AdjoinRoot.induction_on with
  | ih p =>
    induction p using Polynomial.induction_on' with
    | add p q hp hq => simpa [map_add] using U.add_mem hp hq
    | monomial n c =>
      simpa [← Polynomial.C_mul_X_pow_eq_monomial, root, Algebra.smul_def]
        using U.smul_mem c (hp n)

/-- Cyclicity also holds in the actual Mathlib group-algebra module. -/
theorem group_algebra_cyclic (u : (Quotient f)ˣ) (hu : (u : Quotient f) = root f) :
    Submodule.span K[Subgroup.zpowers u]
      {((cyclicRepresentation f u).asModuleEquiv.symm 1)} = ⊤ := by
  let ρ := cyclicRepresentation f u
  let P := Submodule.span K[Subgroup.zpowers u] {ρ.asModuleEquiv.symm 1}
  have hp : ρ.asModuleEquiv.symm 1 ∈ P := Submodule.subset_span (by simp)
  have hle : Submodule.span K (Set.range (fun g : Subgroup.zpowers u => ρ g 1)) ≤
      (P.restrictScalars K).comap ρ.asModuleEquiv.symm.toLinearMap := by
    apply Submodule.span_le.mpr
    rintro _ ⟨g, rfl⟩
    change ρ.asModuleEquiv.symm (ρ g 1) ∈ P
    rw [Representation.asModuleEquiv_symm_map_rho]
    exact P.smul_mem _ hp
  rw [orbit_spans f u hu] at hle
  change P = ⊤
  apply Submodule.eq_top_iff'.mpr
  intro a
  exact hle (Submodule.mem_top : ρ.asModuleEquiv a ∈ (⊤ : Submodule K (Quotient f)))

/-- A source-prime control: an irreducible actual quotient has no proper root-invariant space. -/
theorem invariant_eq_bot_or_top [Fact (Irreducible f)] (U : Submodule K (Quotient f))
    (hU : ∀ a ∈ U, root f * a ∈ U) : U = ⊥ ∨ U = ⊤ := by
  by_cases hz : U = ⊥
  · exact Or.inl hz
  · right
    obtain ⟨a, ha, hne⟩ := Submodule.exists_mem_ne_zero_of_ne_bot hz
    apply top_unique
    intro b _
    simpa [hne] using mul_mem_of_root_invariant f U hU ha (b / a)

#print axioms quotient_dimension
#print axioms cyclicRepresentation_faithful
#print axioms orbit_spans
#print axioms invariant_eq_bot_or_top
end
end DiemCyclicModuleAudit
