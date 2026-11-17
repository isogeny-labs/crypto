import GuardedReduction
import Mathlib.Tactic.FieldSimp

namespace RationalFibers
noncomputable section
open MvPolynomial CubicArithmetic GuardedReduction GridLower

variable {K : Type*} [Field K]

def middle (α β x y : K) : K := (x+y)*(x*y+α)+2*β
def constantTerm (α β x y : K) : K := (x*y-α)^2-4*β*(x+y)
def minusRoot (α β x y Y Z : K) : K := (middle α β x y-2*Y*Z)/(x-y)^2
def plusRoot (α β x y Y Z : K) : K := (middle α β x y+2*Y*Z)/(x-y)^2

theorem discriminant_identity (α β x y : K) :
    (middle α β x y)^2-4*(x^3+α*x+β)*(y^3+α*y+β) =
      (x-y)^2*constantTerm α β x y := by
  unfold middle constantTerm
  ring

/-- Rational lifts split the specialized monic quadratic. No division by
a point ordinate occurs, so repeated roots and two-torsion are included. -/
theorem split_monic (α β x y Y Z : K) (hxy : x ≠ y)
    (hY : Y^2=x^3+α*x+β) (hZ : Z^2=y^3+α*y+β) :
    ∃ ρ τ : K, -2*middle α β x y/(x-y)^2 = -(ρ+τ) ∧
      constantTerm α β x y/(x-y)^2 = ρ*τ := by
  let d := (x-y)^2
  let m := middle α β x y
  have hd : d ≠ 0 := pow_ne_zero 2 (sub_ne_zero.mpr hxy)
  refine ⟨minusRoot α β x y Y Z, plusRoot α β x y Y Z, ?_, ?_⟩
  · simp only [minusRoot,plusRoot,div_eq_mul_inv]
    ring
  · have hp : (Y*Z)^2=(x^3+α*x+β)*(y^3+α*y+β) := by rw [mul_pow,hY,hZ]
    have hnum : (m-2*Y*Z)*(m+2*Y*Z) = d*constantTerm α β x y := by
      calc
        _ = m^2-4*(Y*Z)^2 := by ring
        _ = d*constantTerm α β x y := by
          rw [hp]
          simpa only [mul_assoc] using discriminant_identity α β x y
    change constantTerm α β x y/d = ((m-2*Y*Z)/d)*((m+2*Y*Z)/d)
    rw [div_mul_div_comm,hnum]
    field_simp [hd]

theorem raw_monic (α β x y u : K) (hxy : x ≠ y) :
    PrimeBitLocal.oddS3 α β x y u = (x-y)^2*
      (u^2+(-2*middle α β x y/(x-y)^2)*u+constantTerm α β x y/(x-y)^2) := by
  unfold PrimeBitLocal.oddS3 middle constantTerm
  field_simp [sub_ne_zero.mpr hxy]
  <;> ring

theorem has_rational_root (α β x y Y Z : K) (hxy : x ≠ y)
    (hY : Y^2=x^3+α*x+β) (hZ : Z^2=y^3+α*y+β) :
    ∃ u : K, PrimeBitLocal.oddS3 α β x y u = 0 := by
  obtain ⟨ρ,τ,hA,hB⟩ := split_monic α β x y Y Z hxy hY hZ
  refine ⟨ρ,?_⟩
  rw [raw_monic α β x y ρ hxy,hA,hB]
  ring

theorem swap_first_last (α β x y u : K) :
    PrimeBitLocal.oddS3 α β x y u = PrimeBitLocal.oddS3 α β u y x := by
  unfold PrimeBitLocal.oddS3
  ring

def assignment (x y z u : K) : Fin 5 → K := fun j =>
  if j=0 then u else if j=1 then x else if j=2 then y else if j=3 then z else (x-y)⁻¹

def NoSolution (α β a b c r : K) : Prop :=
  ¬ ∃ ρ : Fin 5 → K, ∀ p ∈ GuardedModel.inputs α β a b c r, MvPolynomial.eval ρ p = 0

/-- Two scalar summation-polynomial zeros on the guarded base give an
actual solution of all six original multivariate generators. -/
theorem solution_of_common (α β a b c r x y z u : K)
    (hx : x ∈ ({a,b,c} : Set K)) (hy : y ∈ ({a,b,c} : Set K))
    (hz : z ∈ ({a,b,c} : Set K)) (hxy : x ≠ y)
    (h1 : PrimeBitLocal.oddS3 α β x y u = 0)
    (h2 : PrimeBitLocal.oddS3 α β u z r = 0) :
    ∃ ρ : Fin 5 → K, ∀ p ∈ GuardedModel.inputs α β a b c r, MvPolynomial.eval ρ p = 0 := by
  refine ⟨assignment x y z u,?_⟩
  intro p hp
  rcases hp with rfl | rfl | rfl | rfl | rfl | rfl
  · rcases hx with rfl | rfl | rfl <;> simp [GuardedModel.membership,assignment]
  · rcases hy with rfl | rfl | rfl <;> simp [GuardedModel.membership,assignment]
  · rcases hz with rfl | rfl | rfl <;> simp [GuardedModel.membership,assignment]
  · simpa [GuardedModel.first,PrimeBitLocal.oddS3,assignment] using h1
  · simpa [GuardedModel.second,PrimeBitLocal.oddS3,assignment] using h2
  · simp [GuardedModel.guard,assignment,inv_mul_cancel₀ (sub_ne_zero.mpr hxy)]

theorem target_outside_base (α β a b c r : K) (hab : a ≠ b)
    (hlift : ∀ x ∈ ({a,b,c} : Set K), ∃ Y : K, Y^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) : r ∉ ({a,b,c} : Set K) := by
  intro hr
  obtain ⟨y,hy,hry⟩ : ∃ y ∈ ({a,b,c} : Set K), r ≠ y := by
    by_cases hra : r=a
    · exact ⟨b,by simp,by simpa [hra] using hab⟩
    · exact ⟨a,by simp,hra⟩
  obtain ⟨Y,hY⟩ := hlift r hr
  obtain ⟨Z,hZ⟩ := hlift y hy
  obtain ⟨u,hu⟩ := has_rational_root α β r y Y Z hry hY hZ
  apply hns
  apply solution_of_common α β a b c r r y y u hr hy hy hry hu
  rw [swap_first_last]
  exact hu

end
end RationalFibers

#print axioms RationalFibers.split_monic
#print axioms RationalFibers.target_outside_base
