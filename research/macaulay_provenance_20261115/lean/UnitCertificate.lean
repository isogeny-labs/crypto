import Mathlib.RingTheory.Ideal.Span
import Mathlib.Data.ZMod.Basic
import Mathlib.Algebra.Polynomial.Basic
import Mathlib.RingTheory.Polynomial.Basic
import Mathlib.Tactic.Ring

/-! Exact instance of the native unit certificate; no native-parser theorem. -/
namespace MacaulayCertificateInstances
open Polynomial
abbrev R := Polynomial (ZMod 5)

/-- The actual unit-row coefficients are 1 and 4 for x and x-1 over F5. -/
theorem unit_certificate_identity : (X : R)+4*((X : R)-1)=1 := by
  have h5 : (5 : R)=0 := by
    simpa only [Nat.cast_ofNat] using (CharP.cast_eq_zero R 5)
  calc
    (X : R)+4*((X : R)-1)=(5 : R)*((X : R)-1)+1 := by ring
    _=1 := by rw [h5,zero_mul,zero_add]

/-- The exact declared input ideal is unit, using Mathlib's ideal laws. -/
theorem unit_ideal_top : Ideal.span ({(X : R),(X : R)-1} : Set R)=⊤ := by
  apply (Ideal.eq_top_iff_one _).mpr
  have hx : (X : R) ∈ Ideal.span ({(X : R),(X : R)-1} : Set R) :=
    Ideal.subset_span (by simp)
  have hy : (X : R)-1 ∈ Ideal.span ({(X : R),(X : R)-1} : Set R) :=
    Ideal.subset_span (by simp)
  have hc := (Ideal.span ({(X : R),(X : R)-1} : Set R)).add_mem hx
    ((Ideal.span ({(X : R),(X : R)-1} : Set R)).mul_mem_left 4 hy)
  rwa [unit_certificate_identity] at hc

end MacaulayCertificateInstances
