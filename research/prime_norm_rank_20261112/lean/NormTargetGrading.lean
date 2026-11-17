module
public import NormTargetFiber
import all NormTargetFiber
import all NormPoleTransport
import all NormWeightedLeading
import all NormMembershipElimination
public section

namespace NormTargetGrading
open Polynomial

section Ring
variable {R : Type*} [CommRing R]

/-- The explicit normalized row, with the actual monic quotient retained. -/
noncomputable def normalized (e a b r z a1 a2 c0 c1 terminal : R) (n : ℕ) : R :=
  e^(2^(n+1)-3)*((NormMembershipElimination.master (2 : R) n).eval r-terminal)-
    (c0+c1*e^2*r)*NormTargetFiber.scaledPolar e a r z a1 a2 c0 c1*
      (NormTargetFiber.quotient e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)

/-- Factorization uses the computed remainder, with no quotient witness. -/
theorem actualTargetFactor (e a b r z a1 a2 c0 c1 terminal : R)
    (hp : z^2=r^3+a*r+b) (n : ℕ) (hd : 3≤2^(n+1)) :
    (NormTargetFiber.membership e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)=
      e^3*normalized e a b r z a1 a2 c0 c1 terminal n :=
  NormTargetFiber.membershipTargetFactor _ _ _ _ _ _ _ _ _ _ hp n hd

/-- At parameter one the normalized row is exactly the original target row. -/
theorem normalizedOne (a b r z a1 a2 c0 c1 terminal : R)
    (hp : z^2=r^3+a*r+b) (n : ℕ) (hd : 3≤2^(n+1)) :
    normalized 1 a b r z a1 a2 c0 c1 terminal n=
      (NormTargetFiber.membership 1 a b r z a1 a2 c0 c1 terminal n).eval r := by
  simpa using (actualTargetFactor 1 a b r z a1 a2 c0 c1 terminal hp n hd).symm

end Ring

section Field
variable {K : Type*} [Field K]

/-- Bind parameter-one membership to the exact support used in prior transport. -/
theorem membershipOne (a b r z a1 a2 c0 c1 terminal : K) (n : ℕ) :
    NormTargetFiber.membership 1 a b r z a1 a2 c0 c1 terminal n=
      (NormMembershipElimination.master (2 : K) n-C terminal) %ₘ
        NormPoleTransport.actualSupport a b a1 a2 c0 c1 r z := by
  have hm : NormTargetFiber.master (1 : K) n=NormMembershipElimination.master (2 : K) n := by
    induction n with
    | zero => rfl
    | succ n ih => simp [NormTargetFiber.master,NormMembershipElimination.master,ih]
  unfold NormTargetFiber.membership NormTargetFiber.dividend
  rw [hm]
  simp only [one_pow,mul_one,NormTargetFiber.scaledSupport,NormTargetFiber.support,one_mul]
  rfl

/-- Evaluate full actual-remainder transport at the actual scaled target. -/
theorem targetValueScale (e l a b r z a1 a2 c0 c1 terminal : K)
    (he : e*l=1) (hp : z^2=r^3+a*r+b) (n : ℕ) :
    e^(2^(n+1))*(NormTargetFiber.membership 1 a b r z
      (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) terminal n).eval r=
      (NormTargetFiber.membership e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r) := by
  have h := congrArg (fun P : K[X] => P.eval (e^2*r))
    (NormPoleTransport.membershipPoleScale e l a b a1 a2 c0 c1 r z terminal he hp n)
  have ht : l^2*(e^2*r)=r := by
    calc
      l^2*(e^2*r)=(e*l)^2*r := by ring
      _ = r := by rw [he]; ring
  rw [membershipOne,NormTargetFiber.membershipField]
  simpa only [eval_mul,eval_C,eval_comp,eval_X,ht] using h

/-- The normalized row has the exact nominal d-3 inverse-pair transformation.
This identity alone is not a multivariate degree or closure theorem. -/
theorem normalizedScale (e l a b r z a1 a2 c0 c1 terminal : K)
    (he : e*l=1) (hp : z^2=r^3+a*r+b) (n : ℕ) (hd : 3≤2^(n+1)) :
    e^(2^(n+1)-3)*normalized 1 a b r z
      (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) terminal n=
      normalized e a b r z a1 a2 c0 c1 terminal n := by
  have hen : e≠0 := by intro hz; simp [hz] at he
  apply mul_left_cancel₀ (pow_ne_zero 3 hen)
  rw [← mul_assoc,← pow_add,Nat.add_sub_of_le hd,
    normalizedOne _ _ _ _ _ _ _ _ _ hp n hd,
    targetValueScale _ _ _ _ _ _ _ _ _ _ _ he hp n,
    actualTargetFactor _ _ _ _ _ _ _ _ _ _ hp n hd]

end Field
end NormTargetGrading

#print axioms NormTargetGrading.actualTargetFactor
#print axioms NormTargetGrading.normalizedOne
#print axioms NormTargetGrading.membershipOne
#print axioms NormTargetGrading.targetValueScale
#print axioms NormTargetGrading.normalizedScale
