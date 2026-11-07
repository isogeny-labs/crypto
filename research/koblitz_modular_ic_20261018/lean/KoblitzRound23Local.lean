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
  exact (Ideal.span_singleton_le_iff_mem I).mpr (mem_of_colon_unit I h q hu hc)

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

-- The actual F32 fiber and the exact inverse appearing in compact02.
def ssPoly (a : A) : A := a^5+a^3+1
def ssL (a n l : A) : A := l^4+(a+1)*l+(a+1)^2
def ssC (a n l : A) : A := n^4+(a+1)^3*n+(a+1)^4*(l+1)
def ssU (a n l : A) : A := n^2+(a+1)*n+(a+1)*l^3+(a+1)^2*l
def ssInv (a n l : A) : A :=
  (a^4+a)*n^2+(a^4+a^3+a^2+a+1)*n+
  (a^4+a^3+a^2+a+1)*l^3+a^3*l+a^3
def ssMultiplier (a n l : A) : A :=
  l^6+n^2*l^3+a*l^4+a^3*l+a*n^2*l+l^4+a^3+a^2*n+n^3+
  a*n*l+a*n+n^2+a*l+n*l+a+1
def ssEven (a n l : A) : A :=
  a^4*l^6+a^5*n*l^3+a^4*n^2*l^3+a^5*l^4+a^3*l^6+a^4*n^4+
  2*a^7*l+a^5*n^2*l+2*a^4*n*l^3+2*a^4*l^4+a^2*l^6+2*a^7+
  a^6*n+a^4*n^3+2*a^6*l+a^5*n*l+a^5*l^2+a^4*l^3+
  2*a^3*n*l^3+a^2*n^2*l^3+2*a^3*l^4+a*l^6+2*a^6+a^5*n+
  a^4*n^2+3*a^5*l+2*a^4*n*l+a^3*n^2*l+2*a^4*l^2+a^3*l^3+
  2*a^2*n*l^3+a*n^2*l^3+2*a^2*l^4+2*a^5+a^4*n+a^3*n^2+
  a^2*n^3+a*n^4+3*a^4*l+2*a^3*n*l+a^2*n^2*l+a^3*l^2+
  2*a*n*l^3+a*l^4+2*a^4+2*a^3*n+a^2*n^2+a*n^3+3*a^3*l+
  2*a^2*n*l+n*l^3+2*a^3+a^2*n+a*n^2+2*a^2*l+a*n*l+2*a^2

theorem ss5_unit_integer_lift (a n l : A) :
    a^3*l^2*ssL a n l+(a^4+a)*ssC a n l+ssInv a n l*ssU a n l+1=
      ssPoly a*ssMultiplier a n l+2*ssEven a n l := by
  unfold ssL ssC ssInv ssU ssPoly ssMultiplier ssEven
  ring

theorem ss5_explicit_inverse [CharP A 2] (a n l : A)
    (hq : ssPoly a=0) (hL : ssL a n l=0) (hC : ssC a n l=0) :
    ssInv a n l*ssU a n l=1 := by
  have h2 : (2:A)=0 := CharP.cast_eq_zero A 2
  have hx := ss5_unit_integer_lift a n l
  simp only [hL,hC,hq,h2,mul_zero,zero_mul,zero_add,add_zero] at hx
  linear_combination hx-h2

theorem ss5_unit_is_unit [CharP A 2] (a n l : A)
    (hq : ssPoly a=0) (hL : ssL a n l=0) (hC : ssC a n l=0) :
    IsUnit (ssU a n l) := by
  apply isUnit_iff_exists_inv.mpr
  exact ⟨ssInv a n l, by simpa only [mul_comm] using ss5_explicit_inverse a n l hq hL hC⟩

-- The geometric bridge identifies the native k-dimensions with e*f.
-- These implications do not assume that a nonnormal fiber is a DVR.
theorem z5_ramification_from_lengths (unramified ramified : ℕ)
    (hu : 2*unramified=2) (hr : 2*ramified=14) :
    unramified=1 ∧ ramified=7 := by omega

theorem z5_degree_accounting : 1*2+7*2=16 := by norm_num

theorem z5_separator_values (sep norm : ℤ)
    (hs : min 7 sep=4) (hn : min 7 norm=6) : sep=4 ∧ norm=6 := by omega

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
#print axioms KoblitzRound23Local.ss5_unit_integer_lift
#print axioms KoblitzRound23Local.ss5_explicit_inverse
#print axioms KoblitzRound23Local.ss5_unit_is_unit
#print axioms KoblitzRound23Local.z5_ramification_from_lengths
#print axioms KoblitzRound23Local.z5_degree_accounting
#print axioms KoblitzRound23Local.z5_separator_values
