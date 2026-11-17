import PathAgreement

namespace StaticHashCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit
variable {G I : Type*} [Fintype G] [Fintype I] [DecidableEq G] [DecidableEq I]

/-- Probability of no state repeat, averaged over the actual static hash law. -/
def staticSurvival (p : I → ℝ) (U : G → I → G) (x : G) (t : ℕ) : ℝ :=
  expectation p (fun h : Hash G I => if StaticSurvives U h x t then 1 else 0)

/-- Probability of no state repeat under an actual iid chronological label word. -/
def iidSurvival (p : I → ℝ) (U : G → I → G) (x : G) (t : ℕ) : ℝ :=
  expectation p (fun w : Fin t → I => if SelfAvoiding U x w then 1 else 0)

theorem selfAvoiding_history_mass (p : I → ℝ) (hp : ∑ i, p i = 1)
    (U : G → I → G) (x : G) {t : ℕ} (w : Fin t → I) (hw : SelfAvoiding U x w) :
    expectation p (fun h : Hash G I => if observations U h x t = w then 1 else 0) =
      weight p w := by
  classical
  unfold weight
  calc
    _ = expectation p (fun h : Hash G I =>
        if ∀ i : Fin t, h (wordPath U x w i.val) = w i then 1 else 0) := by
      congr 1
      funext h
      by_cases he : observations U h x t = w
      · simp only [ite_eq_left he, ite_eq_left ((observations_eq_iff U h x w).mp he)]
      · have hh : ¬∀ i : Fin t, h (wordPath U x w i.val) = w i :=
          fun hc => he ((observations_eq_iff U h x w).mpr hc)
        simp only [ite_eq_right he, ite_eq_right hh]
    _ = _ := by
      have hc := cylinder_mass p hp (fun i : Fin t => wordPath U x w i.val)
        (selfAvoiding_queries_injective U x w hw) w
      convert hc using 1
      congr 1
      funext h
      split_ifs <;> rfl

omit [Fintype G] in
theorem survival_indicator_expansion (U : G → I → G) (h : G → I) (x : G) (t : ℕ) :
    (if StaticSurvives U h x t then (1 : ℝ) else 0) =
      ∑ w : Fin t → I, if SelfAvoiding U x w then
        (if observations U h x t = w then 1 else 0) else 0 := by
  classical
  symm
  calc
    _ = ∑ w : Fin t → I, if observations U h x t = w then
        (if SelfAvoiding U x w then (1 : ℝ) else 0) else 0 := by
      apply Finset.sum_congr rfl
      intro w _
      split_ifs <;> rfl
    _ = if SelfAvoiding U x (observations U h x t) then 1 else 0 := by simp
    _ = _ := by simp only [observed_selfAvoiding_iff]

/-- Exact deferred-decisions identity. The state rule is arbitrary;
independence is derived only at the actually distinct queried states. -/
theorem static_survival_eq_iid (p : I → ℝ) (hp : ∑ i, p i = 1)
    (U : G → I → G) (x : G) (t : ℕ) :
    staticSurvival p U x t = iidSurvival p U x t := by
  classical
  unfold staticSurvival
  simp_rw [survival_indicator_expansion]
  rw [expectation_sum]
  unfold iidSurvival expectation
  apply Finset.sum_congr rfl
  intro w _
  by_cases hw : SelfAvoiding U x w
  · simp only [hw, ite_true, mul_one]
    exact selfAvoiding_history_mass p hp U x w hw
  · simp [hw]

theorem actual_survival_pmf (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i) (hp : ∑ i, p i = 1)
    (U : G → I → G) (x : G) (t : ℕ) :
    (∑ h : Hash G I, (hashLaw p hp0 hp h).toReal *
      (if StaticSurvives U h x t then (1 : ℝ) else 0)) =
    (∑ w : Fin t → I, (hashLaw p hp0 hp w).toReal *
      (if SelfAvoiding U x w then (1 : ℝ) else 0)) := by
  classical
  simp only [hashLaw_real_mass]
  exact static_survival_eq_iid p hp U x t

#print axioms static_survival_eq_iid
#print axioms actual_survival_pmf
end
end StaticHashCollisionAudit
