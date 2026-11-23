module
public import Mathlib.Algebra.Polynomial.Coeff
public import Mathlib.Data.ZMod.Basic
public import Mathlib.Tactic
public section
namespace NormRank
open Polynomial

@[expose] noncomputable def cubic {R : Type*} [CommRing R] (a b : R) : Polynomial R :=
  X^3+C a*X+C b

@[expose] noncomputable def basePair {R : Type*} [CommRing R] (x₁ x₂ : R) : Polynomial R :=
  (X-C x₁)*(X-C x₂)

@[expose] noncomputable def trial {R : Type*} [CommRing R] (q y v : R) : Polynomial R :=
  C v*(X-C q)+C y

/-- Exact defect of the normalized two-support norm equation. -/
theorem defectIdentity {R : Type*} [CommRing R] (a b q y x₁ x₂ v : R) :
    (trial q y v)^2-cubic a b+basePair x₁ x₂*(X-C q)=
      C (v^2-(q+x₁+x₂))*(X-C q)^2+
      C (y^2-(q^3+a*q+b))+
      (X-C q)*C (2*y*v-(q^2+a+q*(q+x₁+x₂)-x₁*x₂)) := by
  simp only [trial,cubic,basePair,map_sub,map_add,map_mul,map_pow,map_ofNat]
  ring

/-- The linear coefficient equations leave a rank-one constraint. -/
theorem defectFromConstraints {R : Type*} [CommRing R] (a b q y x₁ x₂ v : R)
    (hy : y^2=q^3+a*q+b)
    (hv : 2*y*v=q^2+a+q*(q+x₁+x₂)-x₁*x₂) :
    (trial q y v)^2-cubic a b+basePair x₁ x₂*(X-C q)=
      C (v^2-(q+x₁+x₂))*(X-C q)^2 := by
  rw [defectIdentity,hy,hv]
  simp

theorem normEquationIff {R : Type*} [CommRing R] (a b q y x₁ x₂ v : R)
    (hy : y^2=q^3+a*q+b)
    (hv : 2*y*v=q^2+a+q*(q+x₁+x₂)-x₁*x₂) :
    (trial q y v)^2-cubic a b+basePair x₁ x₂*(X-C q)=0 ↔ v^2=q+x₁+x₂ := by
  rw [defectFromConstraints a b q y x₁ x₂ v hy hv]
  constructor
  · intro hz
    have hc := congrArg (fun P : Polynomial R => P.coeff 2) hz
    have hp : ((X-C q)^2 : Polynomial R)=X^2-C (2*q)*X+C (q^2) := by
      simp only [map_mul,map_pow,map_ofNat]
      ring
    have hcoeff : ((X-C q)^2 : Polynomial R).coeff 2=1 := by
      rw [hp]
      simp only [coeff_add,coeff_sub,coeff_X_pow,coeff_C_mul,coeff_C,coeff_X]
      norm_num
    rw [coeff_C_mul,hcoeff] at hc
    simpa only [mul_one,coeff_zero,sub_eq_zero] using hc
  · intro hr
    simp [hr]

theorem fixtureNonsingular : (4:ZMod 17)*2^3+27*2^2≠0 := by decide

theorem fixturePointCount :
    (Finset.univ.filter (fun xy : ZMod 17 × ZMod 17 =>
      xy.2^2=xy.1^3+2*xy.1+2)).card+1=19 := by decide

theorem fixturePoints :
    (1:ZMod 17)^2=5^3+2*5+2 ∧
    (3:ZMod 17)^2=6^3+2*6+2 ∧
    (6:ZMod 17)^2=0^3+2*0+2 := by decide

theorem fixtureLinear : (2:ZMod 17)*6*9=0^2+2+0*(0+5+6)-5*6 := by decide

theorem fixtureRankDefect : (9:ZMod 17)^2-(0+5+6)=2 := by decide

theorem fixtureResidual :
    (trial (0:ZMod 17) 6 9)^2-cubic 2 2+basePair 5 6*(X-C 0)=C 2*X^2 := by
  have h := defectFromConstraints (2:ZMod 17) 2 0 6 5 6 9
    (by decide) fixtureLinear
  simpa only [fixtureRankDefect,C_0,sub_zero] using h

theorem fixtureResidualNonzero :
    (trial (0:ZMod 17) 6 9)^2-cubic 2 2+basePair 5 6*(X-C 0)≠0 := by
  rw [fixtureResidual]
  intro hz
  have hc := congrArg (fun P : Polynomial (ZMod 17) => P.coeff 2) hz
  have hn : (2:ZMod 17)≠0 := by decide
  apply hn
  simpa using hc

theorem fixtureUniqueLinear (v : ZMod 17)
    (hv : 2*6*v=0^2+2+0*(0+5+6)-5*6) : v=9 := by
  have hall : ∀ v : ZMod 17, 2*6*v=0^2+2+0*(0+5+6)-5*6 → v=9 := by decide
  exact hall v hv

/-- The linear relaxation is satisfiable, but the normalized norm problem is not. -/
theorem fixtureNoNorm : ¬∃ v : ZMod 17,
    2*6*v=0^2+2+0*(0+5+6)-5*6 ∧
    (trial 0 6 v)^2-cubic 2 2+basePair 5 6*(X-C 0)=0 := by
  rintro ⟨v,hv,hn⟩
  have he := fixtureUniqueLinear v hv
  subst v
  exact fixtureResidualNonzero hn

end NormRank
#print axioms NormRank.defectIdentity
#print axioms NormRank.normEquationIff
#print axioms NormRank.fixtureNoNorm
