import GuardedFibers

namespace GuardedLastFall
noncomputable section
open MvPolynomial CubicArithmetic CubicNormalization StagedResultant GuardedReduction
  RationalFibers GuardedFibers GridLower FilteredClosure

variable {K : Type*} [Field K]

/-- The actual six-input guarded model has a degree-five refutation whenever
its base points have rational ordinate lifts and it has no field solution.
No unit-ideal, resultant-nonzero, or degree-five closure premise is assumed.
Nonsingularity and a target ordinate are unnecessary for this stronger
polynomial-system statement. -/
theorem upper_five (α β a b c r : K) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hchar : (2 : K) ≠ 0)
    (hlift : ∀ x ∈ ({a,b,c} : Set K), ∃ Y : K, Y^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    Derivable (GuardedModel.inputs α β a b c r) 5 1 := by
  have hw := missing_degree a b c
  have hgw : Derivable (GuardedModel.inputs α β a b c r) 5
      (cubic (missing a b c) (rootsCoefficients a b c)) :=
    monotone (by omega) (missing_cubic_derivable α β a b c r)
  have hgz : Derivable (GuardedModel.inputs α β a b c r) 5
      (cubic (X 3) (rootsCoefficients a b c)) :=
    monotone (by omega) (z_cubic_derivable α β a b c r)
  have h1 : Derivable (GuardedModel.inputs α β a b c r) 5
      (monic (missing a b c) (X 0) (firstLinear α β a b c) (firstConstant α β a b c)) := by
    exact monic_normalization (GuardedModel.inputs α β a b c r)
      (q1A (rootsCoefficients a b c)) (q1B (rootsCoefficients a b c) α β)
      (q1D (rootsCoefficients a b c) α β) a b c hab hac hbc
      (q1A_ne_zero a b c a hab hac hbc (by simp))
      (q1A_ne_zero a b c b hab hac hbc (by simp))
      (q1A_ne_zero a b c c hab hac hbc (by simp))
      (missing a b c) (X 0) hw (by simp) hgw (q1_derivable α β a b c r)
  have hr := target_outside_base α β a b c r hab hlift hns
  have hA (x : K) (hx : x ∈ ({a,b,c} : Set K)) : val (q2A r) x ≠ 0 := by
    rw [q2A_value]
    apply pow_ne_zero
    apply sub_ne_zero.mpr
    intro h
    apply hr
    simpa [← h] using hx
  have h2 : Derivable (GuardedModel.inputs α β a b c r) 5
      (monic (X 3) (X 0) (secondLinear α β a b c r) (secondConstant α β a b c r)) := by
    exact monic_normalization (GuardedModel.inputs α β a b c r)
      (q2A r) (q2B α β r) (q2D α β r) a b c hab hac hbc
      (hA a (by simp)) (hA b (by simp)) (hA c (by simp))
      (X 3) (X 0) (by simp) (by simp) hgz (q2_derivable α β a b c r)
  apply monic_refutation (GuardedModel.inputs α β a b c r)
    (firstLinear α β a b c) (firstConstant α β a b c)
    (secondLinear α β a b c r) (secondConstant α β a b c r)
    a b c hab hac hbc hchar (missing a b c) (X 3) (X 0) hw (by simp) (by simp)
    hgw hgz h1 h2
  intro w hw z hz
  exact scalar_resultant_nonzero α β a b c r w z hab hac hbc hw hz hlift hns

def IsLastFallDegree (F : Set (P K)) (d : ℕ) : Prop :=
  LastFallBound F d ∧ ∀ e < d, ¬ LastFallBound F e

/-- Complete uniform exact last-fall theorem for the six original inputs.
The unit ideal is a conclusion of the constructed refutation, not a premise. -/
theorem exact_last_five (α β a b c r : K) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hchar : (6 : K) ≠ 0)
    (hlift : ∀ x ∈ ({a,b,c} : Set K), ∃ Y : K, Y^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    IsLastFallDegree (GuardedModel.inputs α β a b c r) 5 := by
  have h2 : (2 : K) ≠ 0 := by
    intro h
    apply hchar
    calc
      (6 : K) = 2*3 := by ring
      _ = 0 := by rw [h,zero_mul]
  have hu := upper_five α β a b c r hab hac hbc h2 hlift hns
  refine ⟨lastFallBound_of_refutation hu,?_⟩
  intro d hd
  exact GuardedModel.lastFall_lower α β a b c r hab hac hbc hchar (mem_span hu) d hd

end
end GuardedLastFall

#print axioms GuardedLastFall.upper_five
#print axioms GuardedLastFall.exact_last_five
