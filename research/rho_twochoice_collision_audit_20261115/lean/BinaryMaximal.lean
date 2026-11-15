import CollisionTails

namespace RhoTwoChoiceCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit

def fair (_ : Fin 2) : ℝ := 1 / 2

theorem fair_nonneg (i : Fin 2) : 0 ≤ fair i := by norm_num [fair]

theorem fair_sum : ∑ i : Fin 2, fair i = 1 := by norm_num [Fin.sum_univ_two, fair]

def sign (i : Fin 2) : ℤ := if i = 0 then -1 else 1

/-- Actual integer prefix states, including the initial point. -/
def prefixState : {n : ℕ} → ℤ → (Fin n → Fin 2) → Fin (n + 1) → ℤ
  | 0, s, _ => fun _ => s
  | _n + 1, s, w => Fin.cons s (prefixState (s + sign (w 0)) (Fin.tail w))

/-- Hitting a squared boundary at an actual prefix, including time zero. -/
def Hit (W : ℕ) {n : ℕ} (s : ℤ) (w : Fin n → Fin 2) : Prop :=
  ∃ j, (W : ℝ)^2 ≤ (prefixState s w j : ℝ)^2

theorem prefix_zero {n : ℕ} (s : ℤ) (w : Fin n → Fin 2) : prefixState s w 0 = s := by
  cases n <;> rfl

theorem hit_cons (W : ℕ) (s : ℤ) {n : ℕ} (i : Fin 2) (w : Fin n → Fin 2) :
    Hit W s (Fin.cons i w) ↔ (W : ℝ)^2 ≤ (s : ℝ)^2 ∨ Hit W (s + sign i) w := by
  simp only [Hit, prefixState, Fin.cons_zero, Fin.tail_cons]
  constructor
  · rintro ⟨j, hj⟩
    refine Fin.cases (fun h => Or.inl h) (fun k h => Or.inr ⟨k, h⟩) j hj
  · rintro (h | ⟨j, hj⟩)
    · exact ⟨0, h⟩
    · exact ⟨j.succ, hj⟩

def hitMass (W n : ℕ) (s : ℤ) : ℝ :=
  by
    classical
    exact expectation fair (fun w : Fin n → Fin 2 => if Hit W s w then 1 else 0)

theorem average_cons {n : ℕ} (F : (Fin (n + 1) → Fin 2) → ℝ) :
    expectation fair F =
      (expectation fair (fun w : Fin n → Fin 2 => F (Fin.cons 0 w)) +
       expectation fair (fun w : Fin n → Fin 2 => F (Fin.cons 1 w))) / 2 := by
  classical
  unfold expectation weight
  rw [← Equiv.sum_comp (Fin.consEquiv (fun _ : Fin (n + 1) => Fin 2))]
  simp only [Fintype.sum_prod_type, Fin.prod_univ_succ, Fin.sum_univ_two, fair]
  change (∑ w : Fin n → Fin 2, (1 / 2 * ∏ x : Fin n, (1 / 2 : ℝ)) * F (Fin.cons 0 w)) +
    (∑ w : Fin n → Fin 2, (1 / 2 * ∏ x : Fin n, (1 / 2 : ℝ)) * F (Fin.cons 1 w)) = _
  simp only [← Finset.mul_sum]
  ring

theorem hitMass_nonneg (W n : ℕ) (s : ℤ) : 0 ≤ hitMass W n s := by
  classical
  apply Finset.sum_nonneg
  intro w _
  exact mul_nonneg (weight_nonnegative fair fair_nonneg w) (by dsimp only; split_ifs <;> norm_num)

theorem hitMass_le_one (W n : ℕ) (s : ℤ) : hitMass W n s ≤ 1 := by
  classical
  rw [← weight_sum (G := Fin n) fair fair_sum]
  apply Finset.sum_le_sum
  intro w _
  exact mul_le_of_le_one_right (weight_nonnegative fair fair_nonneg w)
    (by dsimp only; split_ifs <;> norm_num)

theorem hitMass_boundary (W n : ℕ) (s : ℤ) (h : (W : ℝ)^2 ≤ (s : ℝ)^2) :
    hitMass W n s = 1 := by
  classical
  have hb (w : Fin n → Fin 2) : Hit W s w := ⟨0, by simpa [prefix_zero] using h⟩
  simp only [hitMass, expectation, hb, ite_true, mul_one]
  exact weight_sum fair fair_sum

theorem hitMass_zero (W : ℕ) (s : ℤ) :
    hitMass W 0 s = if (W : ℝ)^2 ≤ (s : ℝ)^2 then 1 else 0 := by
  classical
  simp [hitMass, expectation, weight, Hit, prefixState]

theorem hitMass_step (W n : ℕ) (s : ℤ) (h : ¬(W : ℝ)^2 ≤ (s : ℝ)^2) :
    hitMass W (n + 1) s = (hitMass W n (s - 1) + hitMass W n (s + 1)) / 2 := by
  classical
  unfold hitMass
  rw [average_cons]
  simp [hit_cons, h, sign, sub_eq_add_neg]

/-- Elementary stopped-square bound on the actual finite-word hitting event. -/
theorem squared_maximal (W n : ℕ) (s : ℤ) :
    (W : ℝ)^2 * hitMass W n s ≤ (s : ℝ)^2 + n := by
  induction n generalizing s with
  | zero =>
    rw [hitMass_zero]
    split_ifs with h
    · simpa using h
    · simp only [mul_zero, Nat.cast_zero, add_zero]
      exact sq_nonneg _
  | succ n ih =>
    by_cases h : (W : ℝ)^2 ≤ (s : ℝ)^2
    · rw [hitMass_boundary W (n + 1) s h]
      have hn : (0 : ℝ) ≤ n := Nat.cast_nonneg n
      push_cast
      nlinarith
    · rw [hitMass_step W n s h]
      have hm := ih (s - 1)
      have hp := ih (s + 1)
      push_cast at hm hp ⊢
      nlinarith

theorem hitMass_quarter (m : ℕ) (hm : 0 < m) :
    hitMass (2 * m) (m^2) 0 ≤ 1 / 4 := by
  have h := squared_maximal (2 * m) (m^2) 0
  have hmR : 0 < (m : ℝ) := Nat.cast_pos.mpr hm
  have hs : 0 < (m : ℝ)^2 := sq_pos_of_pos hmR
  push_cast at h
  nlinarith

def goodMass (W n : ℕ) : ℝ := by
  classical
  exact expectation fair (fun w : Fin n → Fin 2 => if ¬Hit W 0 w then 1 else 0)

theorem goodMass_eq (W n : ℕ) : goodMass W n = 1 - hitMass W n 0 := by
  classical
  have h := weight_sum (G := Fin n) fair fair_sum
  have hsum : goodMass W n + hitMass W n 0 = ∑ w : Fin n → Fin 2, weight fair w := by
    unfold goodMass hitMass expectation
    rw [← Finset.sum_add_distrib]
    apply Finset.sum_congr rfl
    intro w _
    by_cases hw : Hit W 0 w <;> simp [hw]
  linarith

theorem goodMass_three_quarters (m : ℕ) (hm : 0 < m) :
    3 / 4 ≤ goodMass (2 * m) (m^2) := by
  rw [goodMass_eq]
  linarith [hitMass_quarter m hm]

theorem no_hit_prefix_bounds (W : ℕ) {n : ℕ} (w : Fin n → Fin 2)
    (hw : ¬Hit W 0 w) (j : Fin (n + 1)) :
    -(W : ℤ) < prefixState 0 w j ∧ prefixState 0 w j < (W : ℤ) := by
  have h : (prefixState 0 w j : ℝ)^2 < (W : ℝ)^2 :=
    lt_of_not_ge (fun hc => hw ⟨j, hc⟩)
  have hW : (0 : ℝ) ≤ W := Nat.cast_nonneg W
  constructor
  · exact_mod_cast (show -(W : ℝ) < (prefixState 0 w j : ℝ) by nlinarith)
  · exact_mod_cast (show (prefixState 0 w j : ℝ) < (W : ℝ) by nlinarith)

theorem one_step_controls : hitMass 1 1 0 = 1 ∧ hitMass 2 1 0 = 0 := by
  constructor
  · rw [show 1 = 0 + 1 from rfl, hitMass_step 1 0 0 (by norm_num),
      hitMass_zero, hitMass_zero]
    norm_num
  · rw [show 1 = 0 + 1 from rfl, hitMass_step 2 0 0 (by norm_num),
      hitMass_zero, hitMass_zero]
    norm_num

#print axioms squared_maximal
#print axioms goodMass_three_quarters
#print axioms one_step_controls
end
end RhoTwoChoiceCollisionAudit
