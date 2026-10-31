import KoblitzRound22Base
import Mathlib.Data.Nat.GCD.Basic

/-! Exact algebra behind the explicit O charts. Native factorization and
Jacobian certificates, smooth/proper curve identification, Hensel lifting and
the geometric constant-field divisibilities are separate written bridges.
No theorem here assumes a nonnormal fiber to be a DVR. -/
namespace KoblitzRound24Geometry

variable {A : Type*} [CommRing A]

def infinityBase (t q : A) : A := q+1+(1+t)*t^3*q^2
def tower (z t a d e : A) : A := z^8+t^11*d*z^2+a^2*t^11*d*z+t^14*e

theorem infinity_base_clearing (t q : A) :
    infinityBase t q = q+t^3*q^2+1+t^4*q^2 := by
  unfold infinityBase
  ring

theorem unramified_chart_identity (t y a d e : A) :
    tower (t^3*y) t a d e =
      t^14*(t^10*y^8+t^3*d*y^2+a^2*d*y+e) := by
  unfold tower
  ring

theorem tame_chart_identity (w x a d e : A) :
    tower (w^11*x^3) (w^7*x^2) a d e =
      w^88*x^24*(1+a^2*d*x+w^11*d*x^4+w^10*e*x^4) := by
  unfold tower
  ring

theorem tame_inverse_cleared (w x : A) :
    (w^11*x^3)^2=w*(w^7*x^2)^3 ∧
      (w^7*x^2)^11=x*(w^11*x^3)^7 := by
  constructor <;> ring

theorem unramified_residue_equation (y : A) :
    (0:A)^10*y^8+(0:A)^3*1*y^2+1^2*1*y+1=y+1 := by ring

theorem tame_residue_equation (x : A) :
    1+1^2*1*x+(0:A)^11*1*x^4+(0:A)^10*1*x^4=1+x := by ring

variable {K : Type*} [Field K]

-- Both inverse formulas are in the actual tower variables t,Z.
theorem tame_rational_inverse (t z : K) (ht : t≠0) (hz : z≠0) :
    (z^2/t^3)^7*(t^11/z^7)^2=t ∧
      (z^2/t^3)^11*(t^11/z^7)^3=z := by
  constructor <;> field_simp <;> ring

theorem tame_parameter_inverse (w x : K) (hw : w≠0) (hx : x≠0) :
    (w^11*x^3)^2/(w^7*x^2)^3=w ∧
      (w^7*x^2)^11/(w^11*x^3)^7=x := by
  constructor <;> field_simp <;> ring

theorem two_lambda_residues [CharP K 2] (l : K) :
    l^2+l=0 ↔ l=0 ∨ l=1 := by
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  constructor
  · intro h
    have hh : l*(l-1)=0 := by linear_combination h-l*h2
    rcases mul_eq_zero.mp hh with hh | hh
    · exact Or.inl hh
    · exact Or.inr (sub_eq_zero.mp hh)
  · rintro (rfl | rfl)
    · ring
    · convert h2 using 1 <;> norm_num

-- Multiplicativity of orders in the actual smooth chart supplies these
-- equations. This proves their consequence, not existence of those DVRs.
theorem tame_orders_from_inverse (ot oz : ℤ)
    (hw : 2*oz-3*ot=1) (hx : 11*ot-7*oz=0) : ot=7 ∧ oz=11 := by omega

theorem infinity_fiber_degree : 1+1+7+7=16 := by norm_num

theorem infinity_nu_denominator (e : ℤ) : 2*0+3*(3*e)+2*e=11*e := by ring

-- The written geometry supplies both divisibilities and interprets
-- nonsplitting over F4 as exclusion of degree 2 for the constant field.
theorem constant_degree_candidates (d : ℕ) (h16 : d∣16) (h10 : d∣10) :
    d=1 ∨ d=2 := by
  have hd : d∣2 := by simpa using Nat.dvd_gcd h16 h10
  have hb : d≤2 := Nat.le_of_dvd (by decide) hd
  have hp : d≠0 := by
    rintro rfl
    norm_num at hd
  omega

theorem constant_degree_one (d : ℕ) (h16 : d∣16) (h10 : d∣10)
    (hnot : d≠2) : d=1 := by
  rcases constant_degree_candidates d h16 h10 with h | h
  · exact h
  · exact False.elim (hnot h)

end KoblitzRound24Geometry

#print axioms KoblitzRound24Geometry.infinity_base_clearing
#print axioms KoblitzRound24Geometry.unramified_chart_identity
#print axioms KoblitzRound24Geometry.tame_chart_identity
#print axioms KoblitzRound24Geometry.tame_inverse_cleared
#print axioms KoblitzRound24Geometry.unramified_residue_equation
#print axioms KoblitzRound24Geometry.tame_residue_equation
#print axioms KoblitzRound24Geometry.tame_rational_inverse
#print axioms KoblitzRound24Geometry.tame_parameter_inverse
#print axioms KoblitzRound24Geometry.two_lambda_residues
#print axioms KoblitzRound24Geometry.tame_orders_from_inverse
#print axioms KoblitzRound24Geometry.infinity_fiber_degree
#print axioms KoblitzRound24Geometry.infinity_nu_denominator
#print axioms KoblitzRound24Geometry.constant_degree_candidates
#print axioms KoblitzRound24Geometry.constant_degree_one
