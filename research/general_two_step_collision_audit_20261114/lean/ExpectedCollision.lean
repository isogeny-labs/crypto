import PairLabelEquiv

namespace GeneralTwoStepCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit
variable {G I : Type*} [AddCommGroup G] [Fintype G] [Fintype I]
  [DecidableEq G] [DecidableEq I]

instance admissibleDecidable (w : Labels I) : Decidable (Admissible w) := by
  unfold Admissible
  infer_instance

instance matchDecidable (g : I → G) (h : G → I) (v : G) (w : Labels I) :
    Decidable (Match g h v w) := by
  unfold Match
  infer_instance

instance freshAtDecidable (g : I → G) (h : G → I) (v : G) (xy : G × G) :
    Decidable (FreshAt g h v xy) := by
  unfold FreshAt
  infer_instance

def pairCount (g : I → G) (h : G → I) (v : G) : ℝ :=
  ∑ xy : G × G, if FreshAt g h v xy then 1 else 0

theorem subtype_card_indicator {A : Type*} [Fintype A] (P : A → Prop) [DecidablePred P] :
    (Nat.card {a : A // P a} : ℝ) = ∑ a : A, if P a then (1 : ℝ) else 0 := by
  classical
  rw [Nat.card_eq_fintype_card, Fintype.card_subtype, Finset.card_filter, Nat.cast_sum]
  apply Finset.sum_congr rfl
  intro a _
  split_ifs <;> norm_num

theorem pair_count_expansion (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g)
    (h : G → I) (v : G) :
    pairCount g h v = ∑ w : Labels I,
      if Admissible w then (if Match g h v w then (1 : ℝ) else 0) else 0 := by
  classical
  calc
    _ = (Nat.card (PairAt g h v) : ℝ) := (subtype_card_indicator (FreshAt g h v)).symm
    _ = (Nat.card (WordAt g h v) : ℝ) := by
      exact_mod_cast Nat.card_congr (pairLabelEquiv g hg h v)
    _ = ∑ w : Labels I, if Admissible w ∧ Match g h v w then (1 : ℝ) else 0 :=
      subtype_card_indicator _
    _ = _ := by
      apply Finset.sum_congr rfl
      intro w _
      by_cases ha : Admissible w <;> by_cases hm : Match g h v w <;> simp [ha, hm]

theorem matching_mass (p : I → ℝ) (hp : ∑ i, p i = 1)
    (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g)
    (v : G) (w : Labels I) (hw : Admissible w) :
    expectation p (fun h : Hash G I => if Match g h v w then (1 : ℝ) else 0) =
      ∏ a : Fin 4, p (labelsAt w a) := by
  classical
  have hc := StaticHashCollisionAudit.cylinder_mass p hp (query g v w)
    (query_injective g hg v w hw) (labelsAt w)
  convert hc using 1
  congr 1
  funext h
  unfold Match
  split_ifs <;> rfl

theorem admissible_weight_sum (p : I → ℝ) :
    (∑ w : Labels I, if Admissible w then (∏ a : Fin 4, p (labelsAt w a)) else 0) =
      TwoStepCollisionAudit.labelCoefficient p := by
  simp [Labels, Fintype.sum_prod_type, Admissible, labelsAt,
    Fin.prod_univ_succ, TwoStepCollisionAudit.labelCoefficient, mul_assoc]

theorem expected_fixed_terminal (p : I → ℝ) (hp : ∑ i, p i = 1)
    (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g) (v : G) :
    expectation p (fun h : Hash G I => pairCount g h v) =
      TwoStepCollisionAudit.labelCoefficient p := by
  classical
  simp_rw [pair_count_expansion g hg]
  rw [expectation_sum]
  calc
    _ = ∑ w : Labels I, if Admissible w then (∏ a : Fin 4, p (labelsAt w a)) else 0 := by
      apply Finset.sum_congr rfl
      intro w _
      by_cases hw : Admissible w
      · simp only [hw, ite_true]
        exact matching_mass p hp g hg v w hw
      · simp [hw, expectation]
    _ = _ := admissible_weight_sum p

def Fresh (g : I → G) (h : G → I) (xy : G × G) : Prop :=
  Function.Injective (pairQueries g h xy) ∧
    step g h (step g h xy.1) = step g h (step g h xy.2)

instance freshDecidable (g : I → G) (h : G → I) (xy : G × G) : Decidable (Fresh g h xy) := by
  unfold Fresh
  infer_instance

def globalCount (g : I → G) (h : G → I) : ℝ :=
  ∑ xy : G × G, if Fresh g h xy then 1 else 0

omit [Fintype I] [DecidableEq I] in
theorem terminal_indicator_sum (g : I → G) (h : G → I) (xy : G × G) :
    (∑ v : G, if FreshAt g h v xy then (1 : ℝ) else 0) =
      if Fresh g h xy then 1 else 0 := by
  classical
  by_cases hf : Fresh g h xy
  · have he (v : G) : FreshAt g h v xy ↔ step g h (step g h xy.1) = v := by
      constructor
      · exact fun hh => hh.2.1
      · intro hv
        exact ⟨hf.1, hv, hf.2.symm.trans hv⟩
    simp only [he, hf, ite_true]
    simp
  · have hn (v : G) : ¬FreshAt g h v xy := fun hh =>
      hf ⟨hh.1, hh.2.1.trans hh.2.2.symm⟩
    simp [hn, hf]

omit [Fintype I] [DecidableEq I] in
theorem global_count_sum_terminals (g : I → G) (h : G → I) :
    globalCount g h = ∑ v : G, pairCount g h v := by
  unfold globalCount pairCount
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro xy _
  exact (terminal_indicator_sum g h xy).symm

theorem expected_global_count (p : I → ℝ) (hp : ∑ i, p i = 1)
    (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g) :
    expectation p (fun h : Hash G I => globalCount g h) =
      (Fintype.card G : ℝ) * TwoStepCollisionAudit.labelCoefficient p := by
  simp_rw [global_count_sum_terminals]
  rw [expectation_sum]
  simp only [expected_fixed_terminal p hp g hg, Finset.sum_const, Finset.card_univ, nsmul_eq_mul]

#print axioms expected_fixed_terminal
#print axioms expected_global_count
end
end GeneralTwoStepCollisionAudit
