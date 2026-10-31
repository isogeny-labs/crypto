import Std

/-!
Finite, aligned target-price vectors. The raw CSV and the conversion from
operation counters to priced values are checked outside Lean; this file proves
only the aggregation step used by a whole-cohort loss statement.
-/

namespace KummerCost

inductive PointwiseLt : List Nat → List Nat → Prop where
  | nil : PointwiseLt [] []
  | cons {candidate rho : Nat} {candidates rhos : List Nat}
      (head : rho < candidate)
      (tail : PointwiseLt candidates rhos) :
      PointwiseLt (candidate :: candidates) (rho :: rhos)

theorem sum_le_of_target_losses {candidate rho : List Nat}
    (h : PointwiseLt candidate rho) :
    rho.sum ≤ candidate.sum := by
  induction h with
  | nil => simp
  | cons hhead htail ih =>
      simp only [List.sum_cons]
      omega

theorem pooled_loss_of_target_losses {candidate rho : List Nat}
    (h : PointwiseLt candidate rho)
    (hne : candidate ≠ []) :
    rho.sum < candidate.sum := by
  cases h with
  | nil => contradiction
  | cons hhead htail =>
      have htail_le : _ := sum_le_of_target_losses htail
      simp only [List.sum_cons]
      omega

#print axioms sum_le_of_target_losses
#print axioms pooled_loss_of_target_losses

end KummerCost
