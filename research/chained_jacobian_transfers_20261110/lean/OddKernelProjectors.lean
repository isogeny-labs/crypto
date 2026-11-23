import Init.Grind.Ring.Basic

/-! Coprime Frobenius-factor algebra and fixed quadratic root certificates.
No finite-field/torsion-space/geometric isogeny construction is formalized. -/
set_option autoImplicit false
namespace OddKernelProjectors
open Lean.Grind

def chi {K : Type} [CommRing K] (p t x : K) : K := x^2-t*x+p

theorem bezout_identity {K : Type} [CommRing K] (p t₁ t₂ x : K) :
    (x-t₂)*chi p t₁ x - (x-t₁)*chi p t₂ x = p*(t₁-t₂) := by
  unfold chi
  grind

theorem coprime_certificate {K : Type} [CommRing K]
    (p t₁ t₂ x c : K) (h : c*(p*(t₁-t₂)) = 1) :
    (c*(x-t₂))*chi p t₁ x + (-c*(x-t₁))*chi p t₂ x = 1 := by
  have identity : (c*(x-t₂))*chi p t₁ x + (-c*(x-t₁))*chi p t₂ x =
      c*((x-t₂)*chi p t₁ x - (x-t₁)*chi p t₂ x) := by
    clear h
    grind
  rw [identity, bezout_identity, h]

theorem projector_product {K : Type} [CommRing K] (p t₁ t₂ x c : K) :
    ((c*(x-t₂))*chi p t₁ x) * ((-c*(x-t₁))*chi p t₂ x) =
      -(c^2*(x-t₁)*(x-t₂)) * (chi p t₁ x * chi p t₂ x) := by
  grind

theorem complementary_idempotents {K : Type} [CommRing K] (e f : K)
    (hs : e+f = 1) (hz : e*f = 0) : e^2 = e ∧ f^2 = f := by grind

def roots (ell p t : Nat) : List Nat :=
  (List.range ell).filter fun x => (x*x + ell - (t*x)%ell + p)%ell == 0

theorem fixed_roots17 :
    roots 3 2 2 = [] ∧ roots 3 2 0 = [1,2] ∧
    roots 5 2 4 = [] ∧ roots 5 2 3 = [1,2] ∧
    roots 7 3 6 = [] ∧ roots 7 3 3 = [4,6] := by decide

theorem fixed_roots101 :
    roots 3 2 2 = [] ∧ roots 3 2 1 = [] ∧
    roots 5 1 3 = [4] ∧ roots 5 1 4 = [] ∧
    roots 7 3 0 = [2,5] ∧ roots 7 3 3 = [4,6] := by decide

end OddKernelProjectors
#print axioms OddKernelProjectors.bezout_identity
#print axioms OddKernelProjectors.coprime_certificate
#print axioms OddKernelProjectors.projector_product
#print axioms OddKernelProjectors.complementary_idempotents
#print axioms OddKernelProjectors.fixed_roots17
#print axioms OddKernelProjectors.fixed_roots101
