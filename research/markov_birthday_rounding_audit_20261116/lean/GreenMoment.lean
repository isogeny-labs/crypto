import UniformChain

namespace MarkovBirthdayRoundingAudit
noncomputable section
open scoped BigOperators
variable (p : ℕ) [NeZero p]

/-- Exact source truncated Green sum, including time zero. -/
def green (T : ℕ) (x y : ZMod p) : ℝ :=
  ∑ i ∈ Finset.range (T+1),(transitionMatrix p^i) x y

def rowEnergy (T : ℕ) (x : ZMod p) : ℝ := ∑ y : ZMod p,green p T x y^2
def columnEnergy (T : ℕ) (x : ZMod p) : ℝ := ∑ y : ZMod p,green p T y x^2
def rowMaximum (T : ℕ) : ℝ := Finset.univ.sup' Finset.univ_nonempty (rowEnergy p T)
def columnMaximum (T : ℕ) : ℝ := Finset.univ.sup' Finset.univ_nonempty (columnEnergy p T)

theorem green_one (x y : ZMod p) : green p 1 x y=(if x=y then 1 else 0)+1/(p : ℝ) := by
  simp only [green,Finset.sum_range_succ,Finset.sum_range_zero,zero_add,pow_zero,pow_one,
    Matrix.one_apply,transition_entry]

theorem row_energy_one (x : ZMod p) : rowEnergy p 1 x=1+3/(p : ℝ) := by
  have he (y : ZMod p) : green p 1 x y^2=
      (if y=x then 1+2/(p : ℝ) else 0)+(1/(p : ℝ))^2 := by
    rw [green_one]
    by_cases h : x=y
    · subst y
      simp only [ite_true]
      ring
    · simp only [ite_eq_right h,ite_eq_right (Ne.symm h),zero_add]
  calc
    _=∑ y : ZMod p,((if y=x then 1+2/(p : ℝ) else 0)+(1/(p : ℝ))^2) :=
      Finset.sum_congr rfl (fun y _=>he y)
    _=1+2/(p : ℝ)+(p : ℝ)*(1/(p : ℝ))^2 := by
      simp only [Finset.sum_add_distrib,Finset.sum_ite_eq',Finset.mem_univ,ite_true,
        Finset.sum_const,Finset.card_univ,ZMod.card,nsmul_eq_mul]
    _=_ := by field_simp [ne_of_gt (modulus_positive p)];ring

theorem green_one_symmetric (x y : ZMod p) : green p 1 x y=green p 1 y x := by
  rw [green_one,green_one]
  by_cases h : x=y
  · subst y;rfl
  · rw [ite_eq_right h,ite_eq_right (Ne.symm h)]

theorem column_energy_one (x : ZMod p) : columnEnergy p 1 x=1+3/(p : ℝ) := by
  unfold columnEnergy
  simp_rw [green_one_symmetric p _ x]
  exact row_energy_one p x

theorem row_maximum_one : rowMaximum p 1=1+3/(p : ℝ) := by
  simp only [rowMaximum,row_energy_one,Finset.sup'_const]

theorem column_maximum_one : columnMaximum p 1=1+3/(p : ℝ) := by
  simp only [columnMaximum,column_energy_one,Finset.sup'_const]

theorem uniform_mixing_one (x y : ZMod p) :
    (1 : ℝ)/p≤(transitionMatrix p^1) x y ∧ (transitionMatrix p^1) x y≤(1 : ℝ)/p := by
  rw [pow_one,transition_entry]
  exact ⟨le_rfl,le_rfl⟩

/-- The literal real-valued time expression in source Theorem3.2. -/
def sourceBound (T : ℕ) (m M c : ℝ) : ℝ :=
  4*c*(M/m)^2*(Real.sqrt ((2*(p : ℝ)/M)*max (rowMaximum p T) (columnMaximum p T))+(T : ℝ))

#print axioms green_one
#print axioms row_maximum_one
#print axioms column_maximum_one
#print axioms uniform_mixing_one
end
end MarkovBirthdayRoundingAudit
