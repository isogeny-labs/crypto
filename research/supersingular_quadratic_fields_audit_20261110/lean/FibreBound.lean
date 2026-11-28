import QuadraticModels
import Mathlib.Algebra.Polynomial.Roots

namespace WeierstrassFibreBoundAudit
noncomputable section
open Polynomial SmallCharacteristicCensusAudit
variable {K : Type*} [Field K] [Fintype K] [DecidableEq K]

theorem fibre_card_le_two (E : WeierstrassCurve.Affine K) (x : K) :
    Fintype.card {y : K // equationTest E (x,y)}≤2 := by
  let p : K[X] := X^2+C (E.a₁*x+E.a₃)*X-C (x^3+E.a₂*x^2+E.a₄*x+E.a₆)
  have hd : p.natDegree=2 := by unfold p;compute_degree!
  have hp : p≠0 := by intro h;rw [h,Polynomial.natDegree_zero] at hd;omega
  let S : Finset K := Finset.univ.filter (fun y=>equationTest E (x,y))
  have hr : S.val⊆p.roots := by
    intro y hy
    apply (Polynomial.mem_roots hp).mpr
    have he:=(Finset.mem_filter.mp hy).2
    dsimp only [equationTest] at he
    simp only [Polynomial.IsRoot,p,eval_sub,eval_add,eval_mul,eval_pow,eval_X,eval_C]
    linear_combination he
  have hc:=Polynomial.card_le_degree_of_subset_roots hr
  rw [hd] at hc
  simpa only [Fintype.card_subtype,S] using hc

def equationSigmaEquiv (E : WeierstrassCurve.Affine K) :
    {xy : K×K // equationTest E xy}≃Σ x : K,{y : K // equationTest E (x,y)} where
  toFun xy := ⟨xy.val.1,⟨xy.val.2,xy.property⟩⟩
  invFun xy := ⟨(xy.1,xy.2.val),xy.2.property⟩
  left_inv _ := rfl
  right_inv _ := rfl

theorem counted_le_two_card_add_one (E : WeierstrassCurve.Affine K) :
    counted E≤2*Fintype.card K+1 := by
  unfold counted
  rw [Fintype.card_option,Fintype.card_congr (equationSigmaEquiv E),Fintype.card_sigma]
  have hs : (∑ x : K,Fintype.card {y : K // equationTest E (x,y)})≤∑ _x : K,2 :=
    Finset.sum_le_sum (fun x _=>fibre_card_le_two E x)
  simpa only [Finset.sum_const,Finset.card_univ,smul_eq_mul,Nat.mul_comm] using Nat.succ_le_succ hs

#print axioms counted_le_two_card_add_one
end
end WeierstrassFibreBoundAudit
