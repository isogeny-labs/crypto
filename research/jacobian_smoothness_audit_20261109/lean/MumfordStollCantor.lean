import MumfordStoll
import Mathlib.Algebra.GCDMonoid.Basic

/-! Explicit nonzero-v Cantor certificates for Stoll's order-five cycle.
No geometric Picard identification is asserted. -/
namespace MumfordStollCantorAudit
noncomputable section
open Polynomial MumfordStollAudit
set_option backward.isDefEq.respectTransparency false
set_option maxHeartbeats 1200000

/-- Bezout/common-divisor certificates identify the normalized full gcd.
The monic composition quotient and numerator remainder are explicit.
One reduction suffices for these genus-two constant-v intermediates. -/
def ExplicitCantorStep (P Q R : ReducedPair) : Prop :=
  ∃ d a b c U V q A B C F : K[X],
    d.Monic ∧ U.Monic ∧ d*A=P.u ∧ d*B=Q.u ∧ d*C=P.v+Q.v ∧
    a*P.u+b*Q.u+c*(P.v+Q.v)=d ∧ d^2*U=P.u*Q.u ∧
    a*P.u*Q.v+b*Q.u*P.v+c*(P.v*Q.v+curvePolynomial)=d*(V+U*q) ∧
    V.degree<U.degree ∧ U*F=curvePolynomial-V^2 ∧
    ((U.natDegree≤2 ∧ R.u=U ∧ R.v=V) ∨
      (2<U.natDegree ∧ U*R.u=curvePolynomial-V^2 ∧ R.v=-V))

attribute [local simp] Polynomial.coeff_X
  Polynomial.coeff_C Polynomial.coeff_one Polynomial.coeff_zero
  Polynomial.coeff_X_pow Polynomial.coeff_mul_C Polynomial.coeff_C_mul
  Polynomial.coeff_add Polynomial.coeff_sub Polynomial.coeff_neg

@[local simp] theorem coefficient_constant_power (a : K) (m n : ℕ) :
    ((Polynomial.C a)^m).coeff n=(Polynomial.C (a^m)).coeff n := by
  rw [←map_pow]

macro "f3_poly" : tactic =>
  `(tactic| (ring_nf <;> ((try simp only [←map_ofNat (Polynomial.C : K →+* K[X])]); ext n; (try simp); all_goals (try split_ifs) <;> norm_num <;> (try simp_all) <;> decide)))

def cyclePoint (i : Fin 5) : ReducedPair := point ⟨i.val,by omega⟩

theorem cycle_all_pairs (i j : Fin 5) :
    ExplicitCantorStep (cyclePoint i) (cyclePoint j) (cyclePoint (i+j)) := by
  fin_cases i <;> fin_cases j
  · refine ⟨1,1,0,0,1,0,0,1,1,0,curvePolynomial,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
  · refine ⟨1,1,0,0,X,1,0,1,X,1,X^4,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,1,0,0,X^2,1,0,1,X^2,1,X^3,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,1,0,0,X^2,-1,0,1,X^2,-1,X^3,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,1,0,0,X,-1,0,1,X,-1,X^4,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,1,0,X,1,0,X,1,1,X^4,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,0,-1,X^2,1,(-1:K[X])*X^3,X,X,2,X^3,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,0,-1,X^3,1,(-1:K[X])*X^2,X,X^2,2,X^2,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inr ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨X,1,0,0,X,-1,0,1,X,0,X^4,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨X,1,0,0,1,0,-1,1,1,0,curvePolynomial,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,1,0,X^2,1,0,X^2,1,1,X^3,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,0,-1,X^3,1,(-1:K[X])*X^2,X^2,X,2,X^2,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inr ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,0,-1,X^4,1,(-1:K[X])*X,X^2,X^2,2,X,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inr ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨X^2,1,0,0,1,0,-1,1,1,0,curvePolynomial,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨X,0,1,0,X,1,0,X,1,0,X^4,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,1,0,X^2,-1,0,X^2,1,-1,X^3,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨X,0,1,0,X,-1,0,X,1,0,X^4,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨X^2,1,0,0,1,0,1,1,1,0,curvePolynomial,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,0,1,X^4,-1,(1:K[X])*X,X^2,X^2,-2,X,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inr ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,0,1,X^3,-1,(1:K[X])*X^2,X^2,X,-2,X^2,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inr ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,1,0,X,-1,0,X,1,-1,X^4,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨X,1,0,0,1,0,1,1,1,0,curvePolynomial,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨X,1,0,0,X,1,0,1,X,0,X^4,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,0,1,X^3,-1,(1:K[X])*X^2,X,X^2,-2,X^2,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inr ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly
  · refine ⟨1,0,0,1,X^2,-1,(1:K[X])*X^3,X,X,-2,X^3,
      ?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,Or.inl ⟨?_,?_,?_⟩⟩
    all_goals norm_num [cyclePoint,point,representative,candidatePair,
      uPolynomial,vPolynomial,curvePolynomial]
    all_goals f3_poly

#print axioms cycle_all_pairs
end
end MumfordStollCantorAudit
