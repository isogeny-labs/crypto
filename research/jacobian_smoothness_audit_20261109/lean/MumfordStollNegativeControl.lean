import MumfordStollProofRules

/-! A source-known negative control: omitting the nonzero normalization
factor from a high-degree reduction changes the actual polynomial equation. -/
namespace MumfordStollNegativeControlAudit
noncomputable section
open Polynomial MumfordStollAudit

theorem omitting_normalization_factor_false :
    (X^3+X^4)*(point 8).u ≠ curvePolynomial-(1+X^3)^2 := by
  intro h
  have he := congrArg (Polynomial.eval (1 : K)) h
  norm_num [point,candidatePair,representative,uPolynomial,vPolynomial,
    curvePolynomial,Polynomial.eval_add,Polynomial.eval_sub,
    Polynomial.eval_mul,Polynomial.eval_pow] at he
  exact (by decide : (8 : K) ≠ -2) he

#print axioms omitting_normalization_factor_false
end
end MumfordStollNegativeControlAudit
