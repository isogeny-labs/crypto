import Mathlib.Tactic.Ring
import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.FieldSimp
/- Algebraic update rules only, not a proof of the Rust union-find algorithm or a speed claim. -/
namespace AnchorGraph
variable {K : Type*} [Field K]
theorem substitute_endpoints (u v ru rv au av bu bv cu cv h : K)
    (hu : u = au * ru + bu) (hv : v = av * rv + bv)
    (edge : cu * u + cv * v + h = 0) :
    (cu * au) * ru + (cv * av) * rv + (h + cu * bu + cv * bv) = 0 := by
  rw [hu, hv] at edge
  linear_combination edge

theorem attach_root (u v a b h : K) (ha : a ≠ 0)
    (edge : a * u + b * v + h = 0) : u = -(b / a) * v - h / a := by
  have hu : u = -(b * v + h) / a := by
    apply (eq_div_iff ha).2
    linear_combination edge
  calc
    u = -(b * v + h) / a := hu
    _ = -(b / a) * v - h / a := by field_simp [ha]; ring

theorem balanced_cycle (r a b h : K) (balance : a + b = 0)
    (edge : (a + b) * r + h = 0) : h = 0 := by simpa [balance] using edge

theorem unbalanced_cycle (r a b h : K) (nonzero : a + b ≠ 0)
    (edge : (a + b) * r + h = 0) : r = -h / (a + b) := by
  apply (eq_div_iff nonzero).2
  linear_combination edge
#print axioms attach_root
#print axioms balanced_cycle
#print axioms unbalanced_cycle
end AnchorGraph
