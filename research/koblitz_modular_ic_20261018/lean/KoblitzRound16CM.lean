import KoblitzRound15CM
import Mathlib.Algebra.Polynomial.Eval.Defs
namespace KoblitzRound16CM
instance : Fact (Nat.Prime 11) := ⟨by decide⟩
theorem actual_integer_Bezout {R:Type*} [CommRing R] (x:R) : ((22:R)*x^0 + (11:R)*x^1 + (11:R)*x^2 + (-2:R)*x^5 + (-1:R)*x^6 + (-1:R)*x^7)*((16:R)*x^0 + (-8:R)*x^1 + (-4:R)*x^2 + (6:R)*x^3 + (-1:R)*x^4 + (3:R)*x^5 + (-1:R)*x^6 + (-1:R)*x^7 + (1:R)*x^8)+((22:R)*x^0 + (-11:R)*x^1 + (11:R)*x^2 + (2:R)*x^5 + (-1:R)*x^6 + (1:R)*x^7)*((16:R)*x^0 + (8:R)*x^1 + (-4:R)*x^2 + (-6:R)*x^3 + (-1:R)*x^4 + (-3:R)*x^5 + (-1:R)*x^6 + (1:R)*x^7 + (1:R)*x^8)=(704:R) := by ring
noncomputable def pminus : Polynomial (ZMod 11) := let x := Polynomial.X; (16:Polynomial (ZMod 11))*x^0 + (-8:Polynomial (ZMod 11))*x^1 + (-4:Polynomial (ZMod 11))*x^2 + (6:Polynomial (ZMod 11))*x^3 + (-1:Polynomial (ZMod 11))*x^4 + (3:Polynomial (ZMod 11))*x^5 + (-1:Polynomial (ZMod 11))*x^6 + (-1:Polynomial (ZMod 11))*x^7 + (1:Polynomial (ZMod 11))*x^8
noncomputable def pplus : Polynomial (ZMod 11) := let x := Polynomial.X; (16:Polynomial (ZMod 11))*x^0 + (8:Polynomial (ZMod 11))*x^1 + (-4:Polynomial (ZMod 11))*x^2 + (-6:Polynomial (ZMod 11))*x^3 + (-1:Polynomial (ZMod 11))*x^4 + (-3:Polynomial (ZMod 11))*x^5 + (-1:Polynomial (ZMod 11))*x^6 + (1:Polynomial (ZMod 11))*x^7 + (1:Polynomial (ZMod 11))*x^8
theorem actual_common_root : Polynomial.eval 1 pminus=0 ∧ Polynomial.eval 1 pplus=0  := by
 constructor <;> norm_num [pminus,pplus] <;> decide +kernel
theorem no_integral_projector (A B:Polynomial (ZMod 11)) : A*pminus+B*pplus ≠ 1 := by
 intro h
 have hh := congrArg (Polynomial.eval 1) h
 simp only [Polynomial.eval_add,Polynomial.eval_mul,actual_common_root.1,actual_common_root.2,mul_zero,add_zero,Polynomial.eval_one] at hh
 have hn : (0:ZMod 11) ≠ 1 := by decide +kernel
 exact hn hh
end KoblitzRound16CM
#print axioms KoblitzRound16CM.actual_integer_Bezout
#print axioms KoblitzRound16CM.actual_common_root
#print axioms KoblitzRound16CM.no_integral_projector
