module
public import FirstFall
public import GeneratorRank
public import Homology
public import Mathlib.Data.ZMod.Basic

@[expose] public section
namespace HPSFirstFallAudit
noncomputable section
variable (K : Type*) [Field K]
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩

theorem monomial_sharp_repair (q d : ℕ) (hq : 3 ≤ q) (hd : 2 ≤ d) (hdq : d < q) :
    firstFallDegree K q d = q ∧ linearGeneratorRank K q (x K q ^ d) = 1 := by
  have ht : q ≤ d * (q - 1) := by
    calc
      q ≤ 2 * (q - 1) := by omega
      _ ≤ d * (q - 1) := Nat.mul_le_mul_right _ hd
  exact ⟨monomial_first_fall K q d (by omega) hdq ht,
    monomial_rank_one K q d (by omega) hdq⟩

theorem quadratic_counterfamily (p : ℕ) [Fact p.Prime] (hp : 5 ≤ p) :
    Fintype.card (ZMod p) = p ∧ Module.finrank (ZMod p) (Truncated (ZMod p) p) = p ∧
    (∀ k : ℕ, x (ZMod p) p ^ 2 ∈ piece (ZMod p) p k ↔ k = 2) ∧
    linearGeneratorRank (ZMod p) p (x (ZMod p) p ^ 2) = 1 ∧
    firstFallDegree (ZMod p) p 2 = p ∧
    (linearGeneratorRank (ZMod p) p (x (ZMod p) p ^ 2) * (p - 1) + 2 + 2 : ℕ) <
      2 * firstFallDegree (ZMod p) p 2 := by
  have hh := monomial_sharp_repair (ZMod p) p 2 (by omega) (by omega) (by omega)
  refine ⟨ZMod.card p, actual_dimension (ZMod p) p,
    monomial_piece_iff (ZMod p) p 2 (by omega), hh.2, hh.1, ?_⟩
  rw [hh.1, hh.2]
  omega

theorem literal_rank_bound_false :
    ¬ (∀ d : ℕ, 1 < d → d < 5 →
      (firstFallDegree (ZMod 5) 5 d : ℚ) ≤
        ((linearGeneratorRank (ZMod 5) 5 (x (ZMod 5) 5 ^ d) : ℚ) * (5 - 1) + d + 2) / 2) := by
  intro h
  have hh := h 2 (by omega) (by omega)
  have he := monomial_sharp_repair (ZMod 5) 5 2 (by omega) (by omega) (by omega)
  rw [he.1, he.2] at hh
  norm_num at hh

theorem five_homology_control :
    (∀ k : ℕ, k < 3 → Subsingleton (sourceHomology (ZMod 5) 5 2 k)) ∧
    Nontrivial (sourceHomology (ZMod 5) 5 2 3) := by
  refine ⟨fun k hk => homology_zero_before (ZMod 5) 5 2 k (by omega), ?_⟩
  exact homology_boundary_nonzero (ZMod 5) 5 2 (by omega) (by omega) (by omega)

/-- At q=3 the same source bound is attained, rather than violated. -/
theorem three_positive_control :
    firstFallDegree (ZMod 3) 3 2 = 3 ∧
    linearGeneratorRank (ZMod 3) 3 (x (ZMod 3) 3 ^ 2) = 1 ∧
    2 * firstFallDegree (ZMod 3) 3 2 =
      linearGeneratorRank (ZMod 3) 3 (x (ZMod 3) 3 ^ 2) * (3 - 1) + 2 + 2 := by
  have hh := monomial_sharp_repair (ZMod 3) 3 2 (by omega) (by omega) (by omega)
  exact ⟨hh.1, hh.2, by rw [hh.1, hh.2]⟩

/-- The same expression at q=2 is zero and is not a degree-two test case. -/
theorem binary_invalid_domain_control :
    x (ZMod 2) 2 ^ 2 = 0 ∧ piece (ZMod 2) 2 2 = ⊥ :=
  ⟨x_pow_q (ZMod 2) 2, piece_above (ZMod 2) 2 2 le_rfl⟩

#print axioms monomial_sharp_repair
#print axioms quadratic_counterfamily
#print axioms literal_rank_bound_false
#print axioms five_homology_control
#print axioms three_positive_control
#print axioms binary_invalid_domain_control
end
end HPSFirstFallAudit
