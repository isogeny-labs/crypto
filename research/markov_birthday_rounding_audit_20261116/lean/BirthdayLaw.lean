import BirthdayWords

namespace MarkovBirthdayRoundingAudit
noncomputable section
open scoped BigOperators
open StaticHashCollisionAudit WalkWeightedGeneralAudit
variable (p : ℕ) [NeZero p]

/-- Exact actual iid survival probability, derived from the genuine embedding bijection. -/
theorem uniform_birthday_survival (x : ZMod p) (n : ℕ) :
    iidSurvival (uniformWeight p) (resetUpdate p) x n=
      ((p-1).descFactorial n : ℝ)*(1/(p : ℝ))^n := by
  have hw (w : Fin n → ZMod p) : weight (uniformWeight p) w=(1/(p : ℝ))^n := by
    simp only [weight,uniformWeight,Finset.prod_const,Finset.card_univ,Fintype.card_fin]
  have hc : (∑ w : Fin n → ZMod p,if SelfAvoiding (resetUpdate p) x w then (1 : ℝ) else 0)=
      (Fintype.card (avoidingWords p x n) : ℝ) := by
    change _=(Fintype.card {w : Fin n → ZMod p // SelfAvoiding (resetUpdate p) x w} : ℝ)
    rw [Fintype.card_subtype,Finset.card_filter,Nat.cast_sum]
    apply Finset.sum_congr rfl
    intro w _
    split_ifs <;> norm_num
  unfold iidSurvival expectation
  simp only [hw]
  rw [←Finset.mul_sum,hc,avoiding_words_card,mul_comm]

theorem uniform_birthday_cdf (x : ZMod p) (n : ℕ) :
    collisionCDF p x n=1-((p-1).descFactorial n : ℝ)*(1/(p : ℝ))^n := by
  rw [collision_cdf_complement,uniform_birthday_survival]

theorem birthday_survival_nonnegative (x : ZMod p) (n : ℕ) :
    0 ≤ iidSurvival (uniformWeight p) (resetUpdate p) x n := by
  rw [uniform_birthday_survival]
  positivity

theorem birthday_survival_zero (x : ZMod p) (n : ℕ) (hn : p≤n) :
    iidSurvival (uniformWeight p) (resetUpdate p) x n=0 := by
  rw [uniform_birthday_survival,Nat.descFactorial_of_lt (by have:=NeZero.pos p;omega),Nat.cast_zero,zero_mul]

theorem birthday_survival_recurrence (x : ZMod p) (n : ℕ) (hn : n+1≤p) :
    iidSurvival (uniformWeight p) (resetUpdate p) x (n+1)=
      (1-(n+1 : ℝ)/p)*iidSurvival (uniformWeight p) (resetUpdate p) x n := by
  rw [uniform_birthday_survival,uniform_birthday_survival,Nat.descFactorial_succ,Nat.cast_mul,pow_succ]
  have hc : ((p-1-n : ℕ) : ℝ)=(p : ℝ)-(n+1) := by
    rw [Nat.cast_sub (by omega),Nat.cast_sub (NeZero.pos p)]
    norm_num
    ring
  rw [hc]
  field_simp [ne_of_gt (modulus_positive p)]

#print axioms uniform_birthday_survival
#print axioms uniform_birthday_cdf
#print axioms birthday_survival_zero
#print axioms birthday_survival_recurrence
end
end MarkovBirthdayRoundingAudit
