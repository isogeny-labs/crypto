import LocalSmooth
import Mathlib.Algebra.Polynomial.Derivative
import Mathlib.RingTheory.Coprime.Basic
set_option autoImplicit false
set_option maxHeartbeats 2000000
namespace SameUGlobal
open Polynomial MixedCancellation
variable {K : Type*} [Field K]

theorem coprime_excludes_common_root (f : K[X]) (h : IsCoprime f f.derivative) (x : K) :
 ¬(f.eval x=0 ∧ f.derivative.eval x=0) := by
 rintro ⟨hf,hd⟩
 obtain ⟨a,b,he⟩ := h
 have hx := congrArg (Polynomial.eval x) he
 simp only [eval_add,eval_mul,eval_one,hf,hd,mul_zero,zero_add] at hx
 exact zero_ne_one hx

theorem curve_eval (a0 a1 a2 a3 x : K) :
 (WeightedDoubling.curve a0 a1 a2 a3).eval x=WeightedDoubling.value a0 a1 a2 a3 x := by
 simp [WeightedDoubling.curve,WeightedDoubling.value]

theorem curve_derivative_eval (a0 a1 a2 a3 x : K) :
 (WeightedDoubling.curve a0 a1 a2 a3).derivative.eval x=WeightedDoubling.deriv a1 a2 a3 x := by
 simp [WeightedDoubling.curve,WeightedDoubling.deriv,derivative_add,derivative_mul,derivative_X_pow]
 <;> ring

theorem global_implies_local (a0 a1 a2 a3 : K)
 (h : IsCoprime (WeightedDoubling.curve a0 a1 a2 a3) (WeightedDoubling.curve a0 a1 a2 a3).derivative)
 (x : K) : SameULocal.LocalNonsingular a0 a1 a2 a3 x := by
 have hh := coprime_excludes_common_root _ h x
 simpa only [curve_eval,curve_derivative_eval,SameULocal.LocalNonsingular] using hh

theorem global_extracted_ordinate_nonzero (a0 a1 a2 a3 q r s1 t1 s2 t2 : K)
 (h1 : support q r ∣ WeightedDoubling.curve a0 a1 a2 a3-(ordinate s1 t1)^2)
 (h2 : support q r ∣ WeightedDoubling.curve a0 a1 a2 a3-(ordinate s2 t2)^2)
 (hk : s1-s2≠0) (hn : ¬(s1= -s2 ∧ t1= -t2))
 (hg : IsCoprime (WeightedDoubling.curve a0 a1 a2 a3) (WeightedDoubling.curve a0 a1 a2 a3).derivative) :
 s1*((t2-t1)/(s1-s2))+t1≠0 :=
 SameULocal.extracted_ordinate_nonzero a0 a1 a2 a3 q r s1 t1 s2 t2 h1 h2 hk hn (global_implies_local _ _ _ _ hg _)
end SameUGlobal
