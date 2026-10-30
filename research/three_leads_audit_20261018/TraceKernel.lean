/- Partial formalization of the subgroup argument, independent of Mathlib.
The elliptic-curve trace homomorphism and its kernel identification are the
Kosters–Yeo theorem, not assumptions we claim to have proved in this file. -/
import Std
namespace TraceKernel

def mult {G : Type} (zero : G) (add : G → G → G) : Nat → G → G
  | 0, _ => zero
  | n+1, p => add (mult zero add n p) p

variable {G : Type} (zero : G) (add : G → G → G) (bit : G → Bool)
variable (hzero : bit zero = false)
variable (hadd : ∀ p q, bit (add p q) = Bool.xor (bit p) (bit q))
include hadd

theorem same_coset_pair (p q : G) (h : bit p = bit q) :
    bit (add p q) = false := by
  rw [hadd, h]
  cases bit q <;> rfl

include hzero
theorem trace_even (p : G) (n : Nat) :
    bit (mult zero add (2*n) p) = false := by
  induction n with
  | zero => exact hzero
  | succ n ih =>
    have hn : 2*(n+1) = (2*n+1)+1 := by omega
    rw [hn]
    simp only [mult, hadd]
    rw [ih]
    cases bit p <;> rfl

theorem trace_odd (p : G) (n : Nat) :
    bit (mult zero add (2*n+1) p) = bit p := by
  simp only [mult, hadd]
  rw [trace_even zero add bit hzero hadd p n]
  cases bit p <;> rfl

theorem odd_order_in_kernel (p : G) (n : Nat)
    (horder : mult zero add (2*n+1) p = zero) : bit p = false := by
  have h := trace_odd zero add bit hzero hadd p n
  rw [horder, hzero] at h
  exact h.symm

end TraceKernel
#print axioms TraceKernel.same_coset_pair
#print axioms TraceKernel.trace_even
#print axioms TraceKernel.trace_odd
#print axioms TraceKernel.odd_order_in_kernel
