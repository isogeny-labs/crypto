module
public import Mathlib.Tactic
public section
namespace RankOneExtraction
variable {K I : Type*} [Field K]

/-- All two by two minors vanish; entries may be indexed by any type. -/
def MinorsVanish (M : I → I → K) : Prop :=
  ∀ i j k l, M i j * M k l = M i l * M k j

/-- A symmetric nonzero minor-vanishing matrix has a nonzero diagonal pivot. -/
theorem diagonalPivot (M : I → I → K)
    (hs : ∀ i j, M i j = M j i) (hm : MinorsVanish M)
    (hn : ∃ i j, M i j ≠ 0) : ∃ k, M k k ≠ 0 := by
  obtain ⟨i,j,hij⟩ := hn
  refine ⟨i, ?_⟩
  intro hii
  have h := hm i j j i
  rw [hs j i, hii, zero_mul] at h
  exact (mul_ne_zero hij hij) h

/-- Construct a scaled outer product using one column; no square root is needed. -/
theorem pivotFactor (M : I → I → K)
    (hs : ∀ i j, M i j = M j i) (hm : MinorsVanish M)
    (k : I) (hk : M k k ≠ 0) (i j : I) :
    M i j = (M k k)⁻¹ * (M i k * M j k) := by
  have h := hm i j k k
  rw [← hs j k] at h
  apply (mul_right_cancel₀ hk)
  calc
    M i j * M k k = M i k * M j k := h
    _ = ((M k k)⁻¹ * (M i k * M j k)) * M k k := by
      field_simp

/-- This is extraction after solving the constrained matrix problem. -/
theorem factorExists (M : I → I → K)
    (hs : ∀ i j, M i j = M j i) (hm : MinorsVanish M)
    (hn : ∃ i j, M i j ≠ 0) :
    ∃ (s : K) (w : I → K), s ≠ 0 ∧ (∃ k, w k ≠ 0) ∧
      ∀ i j, M i j = s * (w i * w j) := by
  obtain ⟨k,hk⟩ := diagonalPivot M hs hm hn
  exact ⟨(M k k)⁻¹, (fun i => M i k), inv_ne_zero hk,
    ⟨k,hk⟩, pivotFactor M hs hm k hk⟩

/-- The column vector inherits a linear branch constraint from M times ell = 0. -/
theorem extractBranch [Fintype I] (M : I → I → K) (ell : I → K)
    (hs : ∀ i j, M i j = M j i)
    (hbranch : ∀ i, ∑ j, M i j * ell j = 0) (k : I) :
    ∑ j, M j k * ell j = 0 := by
  simpa only [hs k] using hbranch k

/-- The extracted norm scalar remains nonzero. -/
theorem absorbScale {c s : K} (hc : c ≠ 0) (hs : s ≠ 0) : c / s ≠ 0 :=
  div_ne_zero hc hs
variable {V : Type*} [AddCommGroup V] [Module K V]

/-- Any linear norm-coefficient map respects the extracted outer product and free scale. -/
theorem extractLinearImage (L : (I → I → K) →ₗ[K] V)
    (M : I → I → K) (w : I → K) (s c : K) (g : V) (hs : s ≠ 0)
    (hf : M = s • (fun i j => w i * w j)) (hL : L M = c • g) :
    L (fun i j => w i * w j) = (c / s) • g := by
  have h := congrArg (fun z : V => s⁻¹ • z) hL
  rw [hf, L.map_smul] at h
  simpa [smul_smul, hs, div_eq_mul_inv, mul_comm] using h
end RankOneExtraction
#print axioms RankOneExtraction.diagonalPivot
#print axioms RankOneExtraction.pivotFactor
#print axioms RankOneExtraction.factorExists
#print axioms RankOneExtraction.extractBranch
#print axioms RankOneExtraction.absorbScale
#print axioms RankOneExtraction.extractLinearImage
