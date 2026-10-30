import GuardedLastFall
import FirstFallCore
import Mathlib.RingTheory.MvPolynomial.Homogeneous

namespace GuardedTops
noncomputable section
open MvPolynomial CubicArithmetic CubicNormalization GuardedReduction GridLower

variable {K : Type*} [Field K]

/-- Extract the actual leading form from a homogeneous top plus a lower
remainder, proving its degree as well as its homogeneous component. -/
theorem top_and_degree (T L : P K) (d : ℕ) (hT : T.IsHomogeneous d)
    (hne : T ≠ 0) (hL : L.totalDegree < d) :
    (T+L).totalDegree = d ∧ homogeneousComponent d (T+L) = T := by
  have hd := hT.totalDegree hne
  refine ⟨?_,?_⟩
  · rw [totalDegree_add_eq_left_of_totalDegree_lt (hL.trans_eq hd.symm),hd]
  · rw [map_add,homogeneousComponent_eq_self hT,homogeneousComponent_eq_zero d L hL,add_zero]

def membershipLow (a b c : K) (j : Fin 5) : P K :=
  -C (a+b+c)*X j^2+C (a*b+a*c+b*c)*X j-C (a*b*c)

theorem membershipLow_degree (a b c : K) (j : Fin 5) :
    (membershipLow a b c j).totalDegree ≤ 2 := by
  have hm2 := totalDegree_mul (-C (a+b+c) : P K) (X j^2)
  have hm1 := totalDegree_mul (C (a*b+a*c+b*c) : P K) (X j)
  have hs := totalDegree_add (-C (a+b+c)*X j^2 : P K) (C (a*b+a*c+b*c)*X j)
  have hd := totalDegree_sub (-C (a+b+c)*X j^2+C (a*b+a*c+b*c)*X j : P K) (C (a*b*c))
  simp only [totalDegree_neg,totalDegree_C,totalDegree_X_pow,totalDegree_X] at hm2 hm1 hd
  unfold membershipLow
  omega

theorem membership_top (a b c : K) (j : Fin 5) :
    (GuardedModel.membership a b c (X j)).totalDegree = 3 ∧
    homogeneousComponent 3 (GuardedModel.membership a b c (X j)) = X j^3 := by
  have heq : GuardedModel.membership a b c (X j) = (X j : P K)^3+membershipLow a b c j := by
    simp [GuardedModel.membership,membershipLow,map_add,map_mul]; ring
  rw [heq]
  exact top_and_degree _ _ 3 (isHomogeneous_X_pow j 3)
    (pow_ne_zero 3 (X_ne_zero j)) (by have hh := membershipLow_degree a b c j; omega)

theorem guard_top : (GuardedModel.guard : P K).totalDegree = 2 ∧
    homogeneousComponent 2 (GuardedModel.guard : P K) = FirstFallCore.guardTop := by
  have hh : (FirstFallCore.guardTop : P K).IsHomogeneous 2 :=
    (isHomogeneous_X K 4).mul ((isHomogeneous_X K 1).sub (isHomogeneous_X K 2))
  have heq : (GuardedModel.guard : P K) = FirstFallCore.guardTop+(-1) := by
    simp [GuardedModel.guard,FirstFallCore.guardTop,sub_eq_add_neg]
  rw [heq]
  exact top_and_degree _ _ 2 hh FirstFallCore.guardTop_ne_zero (by simp)

def firstTop : P K :=
  (X 1-X 2)^2*X 0^2-C 2*((X 1+X 2)*(X 1*X 2)*X 0)+(X 1*X 2)^2
def firstLow (α β : K) : P K :=
  C (-2*α)*((X 1+X 2)*X 0+X 1*X 2)+C (-4*β)*(X 0+X 1+X 2)+C (α^2)

theorem firstTop_homogeneous : (firstTop : P K).IsHomogeneous 4 := by
  have h1 : ((X 1-X 2)^2 : P K).IsHomogeneous 2 :=
    ((isHomogeneous_X K 1).sub (isHomogeneous_X K 2)).pow 2
  have hu := isHomogeneous_X_pow (R := K) (0 : Fin 5) 2
  have hxy : (X 1*X 2 : P K).IsHomogeneous 2 := (isHomogeneous_X K 1).mul (isHomogeneous_X K 2)
  have hm : ((X 1+X 2)*(X 1*X 2)*X 0 : P K).IsHomogeneous 4 :=
    (((isHomogeneous_X K 1).add (isHomogeneous_X K 2)).mul hxy).mul
    (isHomogeneous_X K 0)
  exact ((h1.mul hu).sub (hm.C_mul 2)).add (hxy.pow 2)

theorem firstTop_ne_zero : (firstTop : P K) ≠ 0 := by
  intro h
  have hh := congrArg (eval (fun j : Fin 5 => if j=0 ∨ j=1 then (1 : K) else 0)) h
  simpa [firstTop] using hh

theorem firstLow_degree (α β : K) : (firstLow α β).totalDegree ≤ 2 := by
  have hxy := totalDegree_add (X 1 : P K) (X 2)
  have hm := totalDegree_mul (X 1+X 2 : P K) (X 0)
  have hxy' := totalDegree_mul (X 1 : P K) (X 2)
  have hs := totalDegree_add ((X 1+X 2)*X 0 : P K) (X 1*X 2)
  have ha := totalDegree_mul (C (-2*α) : P K) ((X 1+X 2)*X 0+X 1*X 2)
  have hu := totalDegree_add (X 0 : P K) (X 1)
  have hu' := totalDegree_add (X 0+X 1 : P K) (X 2)
  have hb := totalDegree_mul (C (-4*β) : P K) (X 0+X 1+X 2)
  have hs' := totalDegree_add (C (-2*α)*((X 1+X 2)*X 0+X 1*X 2) : P K)
    (C (-4*β)*(X 0+X 1+X 2))
  have hs'' := totalDegree_add
    (C (-2*α)*((X 1+X 2)*X 0+X 1*X 2)+C (-4*β)*(X 0+X 1+X 2) : P K) (C (α^2))
  simp only [totalDegree_X,totalDegree_C] at hxy hm hxy' ha hu hu' hb hs''
  unfold firstLow
  omega

theorem first_top (α β : K) : (GuardedModel.first α β).totalDegree = 4 ∧
    homogeneousComponent 4 (GuardedModel.first α β) = firstTop := by
  have heq : GuardedModel.first α β = firstTop+firstLow α β := by
    simp [GuardedModel.first,PrimeBitLocal.oddS3,firstTop,firstLow,
      map_add,map_mul,map_neg,map_pow,map_ofNat]
    ring
  rw [heq]
  exact top_and_degree _ _ 4 firstTop_homogeneous firstTop_ne_zero
    (by have hh := firstLow_degree α β; omega)

def secondTop : P K := X 3^2*X 0^2
def secondLow (α β r : K) : P K :=
  (C (r^2)+C (-2*r)*X 3)*X 0^2+poly (X 3) (q2B α β r)*X 0+poly (X 3) (q2D α β r)

theorem secondLow_degree (α β r : K) : (secondLow α β r).totalDegree ≤ 3 := by
  have hm := totalDegree_mul (C (-2*r) : P K) (X 3)
  have hs := totalDegree_add (C (r^2) : P K) (C (-2*r)*X 3)
  have hm' := totalDegree_mul (C (r^2)+C (-2*r)*X 3 : P K) (X 0^2)
  have hb := poly_degree (X 3 : P K) (q2B α β r) (by simp)
  have hd := poly_degree (X 3 : P K) (q2D α β r) (by simp)
  have hb' := totalDegree_mul (poly (X 3 : P K) (q2B α β r)) (X 0)
  have h1 := totalDegree_add ((C (r^2)+C (-2*r)*X 3)*X 0^2 : P K)
    (poly (X 3) (q2B α β r)*X 0)
  have h2 := totalDegree_add ((C (r^2)+C (-2*r)*X 3)*X 0^2+poly (X 3) (q2B α β r)*X 0 : P K)
    (poly (X 3) (q2D α β r))
  simp only [totalDegree_C,totalDegree_X,totalDegree_X_pow] at hm hs hm' hb'
  unfold secondLow
  omega

theorem second_top (α β r : K) : (GuardedModel.second α β r).totalDegree = 4 ∧
    homogeneousComponent 4 (GuardedModel.second α β r) = secondTop := by
  have heq : GuardedModel.second α β r = secondTop+secondLow α β r := by
    rw [second_family]
    simp [family,poly,q2A,secondTop,secondLow,map_ofNat,map_mul,map_neg]
    ring
  have hhom : (secondTop : P K).IsHomogeneous 4 :=
    (isHomogeneous_X_pow (R := K) 3 2).mul (isHomogeneous_X_pow 0 2)
  have hne : (secondTop : P K) ≠ 0 :=
    mul_ne_zero (pow_ne_zero 2 (X_ne_zero 3)) (pow_ne_zero 2 (X_ne_zero 0))
  rw [heq]
  exact top_and_degree _ _ 4 hhom hne (by have hh := secondLow_degree α β r; omega)

def generator (α β a b c r : K) (i : Fin 6) : P K :=
  if i=0 then GuardedModel.membership a b c (X 1) else
  if i=1 then GuardedModel.membership a b c (X 2) else
  if i=2 then GuardedModel.membership a b c (X 3) else
  if i=3 then GuardedModel.first α β else
  if i=4 then GuardedModel.second α β r else GuardedModel.guard
def weight (i : Fin 6) : ℕ := if i=5 then 2 else if i=3 ∨ i=4 then 4 else 3
def top (f : Fin 6 → P K) (i : Fin 6) : P K := homogeneousComponent (f i).totalDegree (f i)

theorem generator_degrees (α β a b c r : K) (i : Fin 6) :
    (generator α β a b c r i).totalDegree = weight i := by
  fin_cases i
  · simpa [generator,weight] using (membership_top a b c 1).1
  · simpa [generator,weight] using (membership_top a b c 2).1
  · simpa [generator,weight] using (membership_top a b c 3).1
  · simpa [generator,weight] using (first_top α β).1
  · simpa [generator,weight] using (second_top α β r).1
  · simpa [generator,weight] using (guard_top (K := K)).1

theorem generator_tops (α β a b c r : K) (i : Fin 6) :
    top (generator α β a b c r) i =
      if i=0 then X 1^3 else if i=1 then X 2^3 else if i=2 then X 3^3 else
      if i=3 then firstTop else if i=4 then secondTop else FirstFallCore.guardTop := by
  rw [top,generator_degrees]
  fin_cases i
  · simpa [generator,weight] using (membership_top a b c 1).2
  · simpa [generator,weight] using (membership_top a b c 2).2
  · simpa [generator,weight] using (membership_top a b c 3).2
  · simpa [generator,weight] using (first_top α β).2
  · simpa [generator,weight] using (second_top α β r).2
  · simpa [generator,weight] using (guard_top (K := K)).2

end
end GuardedTops
