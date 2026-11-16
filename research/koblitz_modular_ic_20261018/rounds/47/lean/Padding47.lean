import Mathlib.Algebra.Group.Basic
import Mathlib.Tactic.Abel
namespace Padding47
variable {A : Type*} [AddCommGroup A]
def mixed (p : ℕ → A) : A :=
  4 • p 0+p 1-p 2+p 3-p 4-4 • p 5-p 6+p 7-p 8+p 9
/-- The common effective padding and reference class cancel integrally. -/
theorem common_padding_cancels (p : ℕ → A) (q : A) :
    mixed (fun i => p i+q)=mixed p := by
  simp only [mixed]
  abel
/-- The independently ordered native expressions compute the same group sum. -/
theorem paired_expression (p : ℕ → A) :
    mixed p=4 • (p 0-p 5)+(p 1-p 6)-(p 2-p 7)+(p 3-p 8)-(p 4-p 9) := by
  simp only [mixed]
  abel
end Padding47
