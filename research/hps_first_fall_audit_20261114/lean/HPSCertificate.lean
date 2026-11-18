module
public import RankCounterfamily
public import GammaCounter

@[expose] public section
namespace HPSFirstFallAudit
noncomputable section
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩

/-- Full explicit source-bound refutation, exact repair, and domain controls. -/
theorem hps_first_fall_certificate :
    (∀ (K : Type) [Field K] (q : ℕ),
      iSupIndep (fun i : Fin (powerBasis K q).dim => piece K q i.val) ∧
      (⨆ i : Fin (powerBasis K q).dim, piece K q i.val) = ⊤ ∧
      (∀ i j : ℕ, ∀ a b : Truncated K q,
        a ∈ piece K q i → b ∈ piece K q j → a * b ∈ piece K q (i + j)) ∧
      (∀ a : Truncated K q, a ∈ positivePart K q → a ^ q = 0)) ∧
    (∀ (p : ℕ) [Fact p.Prime], 5 ≤ p →
      Fintype.card (ZMod p) = p ∧ Module.finrank (ZMod p) (Truncated (ZMod p) p) = p ∧
      (∀ k : ℕ, x (ZMod p) p ^ 2 ∈ piece (ZMod p) p k ↔ k = 2) ∧
      linearGeneratorRank (ZMod p) p (x (ZMod p) p ^ 2) = 1 ∧
      firstFallDegree (ZMod p) p 2 = p ∧
      (linearGeneratorRank (ZMod p) p (x (ZMod p) p ^ 2) * (p - 1) + 2 + 2 : ℕ) <
        2 * firstFallDegree (ZMod p) p 2) ∧
    (¬ (∀ d : ℕ, 1 < d → d < 5 →
      (firstFallDegree (ZMod 5) 5 d : ℚ) ≤
        ((linearGeneratorRank (ZMod 5) 5 (x (ZMod 5) 5 ^ d) : ℚ) * (5 - 1) + d + 2) / 2)) ∧
    gamma 5 1 2 4 2 = 0 ∧
    (¬ (∀ q n d : ℕ, q.Prime → 0 < n → 2 ≤ d → d ≤ n * (q - 1) →
      0 < gamma q n d (q - 1) ((n * (q - 1) - d + 2) / 2))) ∧
    (∀ (K : Type) [Field K] (q d : ℕ), 3 ≤ q → 2 ≤ d → d < q →
      firstFallDegree K q d = q ∧ linearGeneratorRank K q (x K q ^ d) = 1) ∧
    ((∀ k : ℕ, k < 3 → Subsingleton (sourceHomology (ZMod 5) 5 2 k)) ∧
      Nontrivial (sourceHomology (ZMod 5) 5 2 3)) ∧
    (firstFallDegree (ZMod 3) 3 2 = 3 ∧
      linearGeneratorRank (ZMod 3) 3 (x (ZMod 3) 3 ^ 2) = 1 ∧
      2 * firstFallDegree (ZMod 3) 3 2 =
        linearGeneratorRank (ZMod 3) 3 (x (ZMod 3) 3 ^ 2) * (3 - 1) + 2 + 2) ∧
    (x (ZMod 2) 2 ^ 2 = 0 ∧ piece (ZMod 2) 2 2 = ⊥) := by
  exact ⟨fun K _ q => ⟨finite_pieces_independent K q, finite_pieces_span K q,
      fun i j _ _ ha hb => piece_mul K q i j ha hb,
      fun _ ha => positivePart_pow_zero K q ha⟩,
    fun p _ hp => quadratic_counterfamily p hp,
    literal_rank_bound_false, gamma_five_zero, literal_gamma_positivity_false,
    fun K _ q d hq hd hdq => monomial_sharp_repair K q d hq hd hdq,
    five_homology_control, three_positive_control, binary_invalid_domain_control⟩

#print axioms hps_first_fall_certificate
end
end HPSFirstFallAudit
