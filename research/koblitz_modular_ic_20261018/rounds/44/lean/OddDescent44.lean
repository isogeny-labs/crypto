import Mathlib.Logic.Function.Iterate
import Mathlib.Tactic.Ring
namespace OddDescent44

theorem fivefold_on_half_period {X : Type*} (F : X → X) (p : X) (d : ℕ)
    (h : (F^[2*d]) p=p) : (F^[5*d]) p=(F^[d]) p := by
  have he : 5*d=d+(2*d)*2 := by ring
  rw [he, Function.iterate_add_apply, Function.iterate_mul]
  rw [Function.iterate_fixed h 2]

theorem sign_fixed_after_fifth_descent {X : Type*} (F S : X → X)
    (p : X) (d : ℕ) (hf : (F^[5]) p=p)
    (hperiod : (F^[2*d]) p=p) (hs : S p=(F^[d]) p) : S p=p := by
  have hfixed : (F^[5*d]) p=p := by
    rw [Function.iterate_mul]
    exact Function.iterate_fixed hf d
  calc
    S p=(F^[d]) p := hs
    _=(F^[5*d]) p := (fivefold_on_half_period F p d hperiod).symm
    _=p := hfixed
end OddDescent44
