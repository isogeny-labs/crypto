import KoblitzRound22SS
import Mathlib.RingTheory.Ideal.Colon
import Mathlib.RingTheory.Ideal.Span
import Mathlib.Algebra.BigOperators.Group.Finset.Basic

/-! Structural consequences of the exact source-neighborhood certificates.
The actual colon/lift matrices are native Singular records. This module does
not assert a scheme normalization or identify its places by assuming them.
-/
namespace KoblitzRound23Local
variable {A : Type*} [CommRing A]

theorem mem_of_unit_mul_mem (I : Ideal A) (h q : A)
    (hu : IsUnit h) (hm : h*q ∈ I) : q ∈ I := by
  rcases hu with ⟨u, rfl⟩
  have hh := I.mul_mem_left (↑u⁻¹ : A) hm
  simpa only [← mul_assoc, Units.inv_mul, one_mul] using hh

theorem mem_of_colon_unit (I : Ideal A) (h q : A)
    (hu : IsUnit h) (hc : h ∈ I.colon {q}) : q ∈ I := by
  apply mem_of_unit_mul_mem I h q hu
  exact Submodule.mem_colon_singleton.mp hc

theorem redundant_equation_of_colon_unit (I : Ideal A) (h q : A)
    (hu : IsUnit h) (hc : h ∈ I.colon {q}) :
    I ⊔ Ideal.span {q} = I := by
  apply sup_eq_left.mpr
  exact Ideal.span_singleton_le_iff_mem.mpr (mem_of_colon_unit I h q hu hc)

-- Local recovery of the Tate equation from an original cofactored residual.
theorem tate_equation_of_unit_cofactor (f c : A)
    (hu : IsUnit c) (h : c*f=0) : f=0 := by
  have hh : f ∈ (⊥ : Ideal A) :=
    mem_of_unit_mul_mem ⊥ c f hu (by simpa using h)
  simpa using hh

-- Written proper-curve degree accounting supplies the displayed hypotheses.
-- This lemma proves the exhaustion step, not the geometric degree formula.
theorem degree_exhaustion (known remaining degree : ℕ)
    (htotal : known+remaining=degree) (hknown : known=degree) : remaining=0 := by
  omega

theorem ss5_degree_accounting : 4*20=16*5 := by norm_num

-- The vanishing denominator of generic nu recovery does not imply a pole.
-- At an unramified SS5 place the base a1 has value1; b,delta0,nu are units.
theorem ss5_denominator_value (a b delta : ℤ)
    (ha : a=1) (hb : b=0) (hd : delta=0) : 2*a+3*b+delta=2 := by omega

theorem quotient_unit_cancellation (numerator denominator quotient : ℤ)
    (hq : quotient=numerator-denominator) (hu : quotient=0)
    (hd : denominator=2) : numerator=2 := by omega

theorem source_relative_jacobian_factor [CharP A 2]
    (fb q0n q0l q1n q1l b u : A)
    (hfb : fb=b^2*(b+1)^2)
    (hqr : q0n*q1l-q0l*q1n=b^4*u) :
    fb*(q0n*q1l-q0l*q1n)=b^6*(b+1)^2*u := by
  rw [hfb,hqr]
  ring

end KoblitzRound23Local
#print axioms KoblitzRound23Local.mem_of_unit_mul_mem
#print axioms KoblitzRound23Local.mem_of_colon_unit
#print axioms KoblitzRound23Local.redundant_equation_of_colon_unit
#print axioms KoblitzRound23Local.tate_equation_of_unit_cofactor
#print axioms KoblitzRound23Local.degree_exhaustion
#print axioms KoblitzRound23Local.ss5_degree_accounting
#print axioms KoblitzRound23Local.ss5_denominator_value
#print axioms KoblitzRound23Local.quotient_unit_cancellation
#print axioms KoblitzRound23Local.source_relative_jacobian_factor
