module
public import Mathlib.Algebra.Polynomial.Eval.Defs
public import Mathlib.Tactic.FieldSimp
public import Mathlib.Tactic.Ring
public section
namespace PoleAnchor
theorem two_pole (c t d : ℚ) (ht : t-d ≠ 0) (hd : 1-d ≠ 0) :
    c/(t-d)-c/(1-d) = c*(1-t)/((t-d)*(1-d)) := by
  field_simp
  ring
theorem node_preserved (w c h : Polynomial ℚ) (t : ℚ)
    (hc : c.eval t = 0) :
    (w+c*h).eval t = w.eval t := by
  simp [hc]
#print axioms two_pole
#print axioms node_preserved
end PoleAnchor
