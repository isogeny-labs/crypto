import S4Degree
import Mathlib.LinearAlgebra.Matrix.Polynomial
import Mathlib.Algebra.MvPolynomial.CommRing

/-! General coefficient/degree control for polynomial determinants.
The determinant proof follows Mathlib/LinearAlgebra/Matrix/Polynomial.lean
(Copyright (c) 2021 Yakov Pechersky, Apache 2.0), with independently assigned
column bounds rather than a uniform bound of one. -/
namespace ResultantDegreeStepAudit
noncomputable section
open Polynomial Matrix
open S4DegreeAudit
variable {R : Type*} [CommRing R]

theorem coeff_prod_column_bounds {ι : Type*} (s : Finset ι)
    (p : ι→R[X]) (d : ι→ℕ) (h : ∀ i∈s, (p i).natDegree≤d i) :
    (∏ i∈s, p i).coeff (∑ i∈s, d i)=∏ i∈s, (p i).coeff (d i) := by
  classical
  revert h
  induction s using Finset.induction_on with
  | empty => intro h; simp
  | @insert a s ha ih =>
    intro h
    have hs : ∀ i∈s, (p i).natDegree≤d i := fun i hi=>h i (Finset.mem_insert_of_mem hi)
    have hd : (∏ i∈s, p i).natDegree≤∑ i∈s, d i := by
      exact (Polynomial.natDegree_prod_le s p).trans (Finset.sum_le_sum hs)
    simp only [Finset.prod_insert ha,Finset.sum_insert ha]
    rw [Polynomial.coeff_mul_add_eq_of_natDegree_le (h a (Finset.mem_insert_self a s)) hd,
      ih hs]

variable {n : Type*} [Fintype n] [DecidableEq n]

theorem determinant_natDegree_le_column_bounds (M : Matrix n n R[X]) (d : n→ℕ)
    (h : ∀ i j, (M i j).natDegree≤d j) : M.det.natDegree≤∑ j, d j := by
  rw [Matrix.det_apply]
  refine (Polynomial.natDegree_sum_le _ _).trans ?_
  refine Multiset.max_le_of_forall_le _ _ ?_
  simp only [forall_apply_eq_imp_iff,true_and,Function.comp_apply,
    Multiset.mem_map,exists_imp,Finset.mem_univ_val]
  intro g
  calc
    (Equiv.Perm.sign g • ∏ i, M (g i) i).natDegree≤(∏ i, M (g i) i).natDegree := by
      rcases Int.units_eq_one_or (Equiv.Perm.sign g) with hg | hg
      · rw [hg,one_smul]
      · rw [hg,Units.neg_smul,one_smul,Polynomial.natDegree_neg]
    _≤∑ i, (M (g i) i).natDegree := Polynomial.natDegree_prod_le _ _
    _≤∑ i, d i := Finset.sum_le_sum (fun i _=>h (g i) i)

set_option backward.isDefEq.respectTransparency false in
theorem determinant_coeff_column_bounds (M : Matrix n n R[X]) (d : n→ℕ)
    (h : ∀ i j, (M i j).natDegree≤d j) :
    M.det.coeff (∑ j, d j)=Matrix.det (fun i j=> (M i j).coeff (d j)) := by
  rw [Matrix.det_apply,Polynomial.finsetSum_coeff,Matrix.det_apply]
  refine Finset.sum_congr rfl ?_
  intro g hg
  rw [Polynomial.coeff_smul]
  rw [coeff_prod_column_bounds Finset.univ (fun i=>M (g i) i) d
    (fun i _=>h (g i) i)]

/-- The two explicit new variables are Z (inner X) and T (outer X).
The elimination variable of the resultant is a third polynomial layer. -/
def firstLift (f : R[X]) : Polynomial (Polynomial (Polynomial R)) :=
  f.map ((Polynomial.C : R[X]→+*R[X][X]).comp (Polynomial.C : R→+*R[X]))

def secondLift (b2 b4 b6 b8 : R) : Polynomial (Polynomial (Polynomial R)) :=
  let z : R[X][X] := C (X:R[X])
  let t : R[X][X] := X
  quad (qaRing z t) (qbRing (C (C b2)) (C (C b4)) (C (C b6)) z t)
    (qcRing (C (C b4)) (C (C b6)) (C (C b8)) z t)

def resultantStep (D : ℕ) (f : R[X]) (b2 b4 b6 b8 : R) : R[X][X] :=
  Polynomial.resultant (firstLift f) (secondLift b2 b4 b6 b8) D 2

def columnBound (D : ℕ) (j : Fin (D+2)) : ℕ :=
  j.addCases (fun _ : Fin D=>2) (fun _ : Fin 2=>0)

theorem columnBound_sum (D : ℕ) : ∑ j : Fin (D+2), columnBound D j=D*2 := by
  simp [columnBound,Fin.sum_univ_add]

theorem qaRing_poly (z : R) :
    qaRing (C z) (X:R[X])=quad 1 (-2*z) (z^2) := by
  simp [qaRing,quad,Polynomial.C_ofNat]
  ring

theorem qbRing_poly (b2 b4 b6 z : R) :
    qbRing (C b2) (C b4) (C b6) (C z) (X:R[X])=
      quad (-2*z) (-(2*z^2+b2*z+b4)) (-(b4*z+b6)) := by
  simp [qbRing,quad,Polynomial.C_ofNat]
  ring

theorem qcRing_poly (b4 b6 b8 z : R) :
    qcRing (C b4) (C b6) (C b8) (C z) (X:R[X])=
      quad (z^2) (-(b4*z+b6)) (-(b6*z+b8)) := by
  simp [qcRing,quad]
  ring

theorem quad_natDegree_le (a b c : R) : (quad a b c).natDegree≤2 := by
  unfold quad
  compute_degree!

theorem quad_coeff_two (a b c : R) : (quad a b c).coeff 2=a := by simp [quad]
theorem quad_coeff_one (a b c : R) : (quad a b c).coeff 1=b := by simp [quad]
theorem quad_coeff_zero (a b c : R) : (quad a b c).coeff 0=c := by simp [quad]

theorem secondLift_natDegree_le (b2 b4 b6 b8 : R) :
    (secondLift b2 b4 b6 b8).natDegree≤2 := quad_natDegree_le ..

theorem secondLift_coeff_degree (b2 b4 b6 b8 : R) (k : ℕ) :
    ((secondLift b2 b4 b6 b8).coeff k).natDegree≤2 := by
  have hd := secondLift_natDegree_le b2 b4 b6 b8
  by_cases hk2 : k=2
  · subst k
    simp only [secondLift,quad_coeff_two]
    rw [qaRing_poly]
    exact quad_natDegree_le ..
  by_cases hk1 : k=1
  · subst k
    simp only [secondLift,quad_coeff_one]
    rw [qbRing_poly]
    exact quad_natDegree_le ..
  by_cases hk0 : k=0
  · subst k
    simp only [secondLift,quad_coeff_zero]
    rw [qcRing_poly]
    exact quad_natDegree_le ..
  have hk : 2<k := by omega
  rw [Polynomial.coeff_eq_zero_of_natDegree_lt (hd.trans_lt hk)]
  simp

theorem secondLift_coeff_top (b2 b4 b6 b8 : R) (k : ℕ) :
    ((secondLift b2 b4 b6 b8).coeff k).coeff 2=
      ((X-C (X:R[X]))^2 : R[X][X]).coeff k := by
  have hq : ((X-C (X:R[X]))^2 : R[X][X])=
      quad (1:R[X]) (-2*(X:R[X])) ((X:R[X])^2) := by
    simp [quad,Polynomial.C_ofNat]
    ring
  rw [hq]
  by_cases hk2 : k=2
  · subst k
    simp only [secondLift,quad_coeff_two]
    rw [qaRing_poly,quad_coeff_two]
  by_cases hk1 : k=1
  · subst k
    simp only [secondLift,quad_coeff_one]
    rw [qbRing_poly,quad_coeff_two]
  by_cases hk0 : k=0
  · subst k
    simp only [secondLift,quad_coeff_zero]
    rw [qcRing_poly,quad_coeff_two]
  have hk : 2<k := by omega
  rw [Polynomial.coeff_eq_zero_of_natDegree_lt
    ((secondLift_natDegree_le b2 b4 b6 b8).trans_lt hk)]
  rw [Polynomial.coeff_eq_zero_of_natDegree_lt
    ((quad_natDegree_le (1:R[X]) (-2*(X:R[X])) ((X:R[X])^2)).trans_lt hk)]
  simp

theorem firstLift_coeff (f : R[X]) (k : ℕ) :
    (firstLift f).coeff k=C (C (f.coeff k)) := by
  simp [firstLift,RingHom.comp_apply]

theorem sylvester_column_degrees (D : ℕ) (f : R[X]) (b2 b4 b6 b8 : R) :
    ∀ i j : Fin (D+2),
      ((Polynomial.sylvester (firstLift f) (secondLift b2 b4 b6 b8) D 2) i j).natDegree≤
        columnBound D j := by
  intro i j
  induction j using Fin.addCases
  · simp only [Polynomial.sylvester,Matrix.of_apply,columnBound,Fin.addCases_left]
    split_ifs
    · exact secondLift_coeff_degree ..
    · simp
  · simp only [Polynomial.sylvester,Matrix.of_apply,columnBound,Fin.addCases_right]
    split_ifs
    · rw [firstLift_coeff]
      simp
    · simp

def leadingSylvester (D : ℕ) (f : R[X]) (b2 b4 b6 b8 : R) :
    Matrix (Fin (D+2)) (Fin (D+2)) R[X] :=
  fun i j=>((Polynomial.sylvester (firstLift f) (secondLift b2 b4 b6 b8) D 2) i j).coeff
    (columnBound D j)

set_option backward.isDefEq.respectTransparency false in
theorem leadingSylvester_eq (D : ℕ) (f : R[X]) (b2 b4 b6 b8 : R) :
    leadingSylvester D f b2 b4 b6 b8=
      Polynomial.sylvester (f.map (C : R→+*R[X]))
        ((X-C (X:R[X]))^2 : R[X][X]) D 2 := by
  apply Matrix.ext
  intro i j
  induction j using Fin.addCases
  · rename_i j0
    simp only [leadingSylvester,Polynomial.sylvester,Matrix.of_apply,
      columnBound,Fin.addCases_left]
    split_ifs
    · exact secondLift_coeff_top (R:=R) b2 b4 b6 b8 ((i:ℕ)-(j0:ℕ))
    · simp
  · rename_i j0
    simp only [leadingSylvester,Polynomial.sylvester,Matrix.of_apply,
      columnBound,Fin.addCases_right]
    split_ifs
    · simp [firstLift_coeff]
    · simp

theorem resultantStep_natDegree_le (D : ℕ) (f : R[X]) (b2 b4 b6 b8 : R) :
    (resultantStep D f b2 b4 b6 b8).natDegree≤D*2 := by
  change (Polynomial.sylvester (firstLift f) (secondLift b2 b4 b6 b8) D 2).det.natDegree≤D*2
  simpa only [columnBound_sum] using
    determinant_natDegree_le_column_bounds
      (Polynomial.sylvester (firstLift f) (secondLift b2 b4 b6 b8) D 2)
      (columnBound D) (sylvester_column_degrees D f b2 b4 b6 b8)

/-- Universal leading coefficient of the actual fixed-degree resultant.
The first size D is retained even if f loses degree under specialization. -/
theorem resultantStep_coeff_top (D : ℕ) (f : R[X]) (b2 b4 b6 b8 : R)
    (hf : f.natDegree≤D) :
    (resultantStep D f b2 b4 b6 b8).coeff (D*2)=f^2 := by
  have hc := determinant_coeff_column_bounds
    (Polynomial.sylvester (firstLift f) (secondLift b2 b4 b6 b8) D 2)
    (columnBound D) (sylvester_column_degrees D f b2 b4 b6 b8)
  rw [columnBound_sum] at hc
  change (resultantStep D f b2 b4 b6 b8).coeff (D*2)=
    (leadingSylvester D f b2 b4 b6 b8).det at hc
  rw [leadingSylvester_eq] at hc
  change (resultantStep D f b2 b4 b6 b8).coeff (D*2)=
    Polynomial.resultant (f.map (C : R→+*R[X]))
      ((X-C (X:R[X]))^2 : R[X][X]) D 2 at hc
  rw [hc,Polynomial.resultant_X_sub_C_pow_right _ _ _ _
    (Polynomial.natDegree_map_le.trans hf)]
  have hsign : (-1:R[X])^(D*2)=1 := by
    rw [Nat.mul_comm D 2,pow_mul]
    simp
  rw [hsign,one_mul,Polynomial.eval_map,Polynomial.eval₂_C_X]

theorem resultantStep_natDegree [IsDomain R] (D : ℕ) (f : R[X])
    (b2 b4 b6 b8 : R) (hf : f.natDegree≤D) (hn : f≠0) :
    (resultantStep D f b2 b4 b6 b8).natDegree=D*2 := by
  apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero (resultantStep_natDegree_le ..)
  rw [resultantStep_coeff_top D f b2 b4 b6 b8 hf]
  exact pow_ne_zero 2 hn

theorem resetting_declared_degree_changes_result [IsDomain R] (b2 b4 b6 b8 : R) :
    resultantStep 2 (1:R[X]) b2 b4 b6 b8≠resultantStep 0 (1:R[X]) b2 b4 b6 b8 := by
  intro he
  have hd2 := resultantStep_natDegree 2 (1:R[X]) b2 b4 b6 b8 (by simp) one_ne_zero
  have hd0 := resultantStep_natDegree 0 (1:R[X]) b2 b4 b6 b8 (by simp) one_ne_zero
  rw [he,hd0] at hd2
  norm_num at hd2

variable {K : Type*} [Field K]

abbrev ThreeInputs (K : Type*) [Field K] := MvPolynomial (Fin 3) K

/-- A faithful multivariate coefficient ring avoids expanding a long tower
of typeclass instances; each input remains an actual independent variable. -/
def multivariateS4 (W : WeierstrassCurve.Affine K) : Polynomial (ThreeInputs K) :=
  positionPolynomial (MvPolynomial.C W.b₂) (MvPolynomial.C W.b₄)
    (MvPolynomial.C W.b₆) (MvPolynomial.C W.b₈)
    (MvPolynomial.X 0) (MvPolynomial.X 1) (MvPolynomial.X 2) 3

def tripleProbe : ThreeInputs K→+*Polynomial (PairRing K) :=
  MvPolynomial.eval₂Hom
    ((Polynomial.C : PairRing K→+*Polynomial (PairRing K)).comp
      ((Polynomial.C : K[X]→+*PairRing K).comp (Polynomial.C : K→+*K[X])))
    (fun i : Fin 3=>if i=0 then C (pairX (K:=K)) else
      if i=1 then C (pairY (K:=K)) else X)

theorem s3Ring_map {S : Type*} [CommRing S] (f : R→+*S)
    (b2 b4 b6 b8 x y z : R) :
    f (s3Ring b2 b4 b6 b8 x y z)=
      s3Ring (f b2) (f b4) (f b6) (f b8) (f x) (f y) (f z) := by
  simp [s3Ring,qaRing,qbRing,qcRing,map_ofNat]

set_option backward.isDefEq.respectTransparency false in
theorem multivariate_triple_nonzero (W : WeierstrassCurve.Affine K) :
    s3Ring (MvPolynomial.C W.b₂ : ThreeInputs K) (MvPolynomial.C W.b₄)
      (MvPolynomial.C W.b₆) (MvPolynomial.C W.b₈)
      (MvPolynomial.X 0) (MvPolynomial.X 1) (MvPolynomial.X 2)≠0 := by
  intro hz
  have he := congrArg (tripleProbe (K:=K)) hz
  have hs := s3Ring_map (R:=ThreeInputs K) (S:=Polynomial (PairRing K))
    (tripleProbe (K:=K))
    (MvPolynomial.C W.b₂) (MvPolynomial.C W.b₄)
    (MvPolynomial.C W.b₆) (MvPolynomial.C W.b₈)
    (MvPolynomial.X 0) (MvPolynomial.X 1) (MvPolynomial.X 2)
  rw [hs,map_zero] at he
  apply formal_triple_nonzero W
  simpa [tripleProbe,RingHom.comp_apply,liftPair] using he

theorem multivariateS4_natDegree (W : WeierstrassCurve.Affine K) :
    (multivariateS4 W).natDegree=4 := by
  apply (positionPolynomial_natDegree_four_iff ..).mpr
  exact multivariate_triple_nonzero W

/-- Canonical first arity beyond S4: actual S4 in the elimination variable,
then a fresh pair of variables in S3. The result lies in K[x1]...[x5]. -/
def symbolicS5 (W : WeierstrassCurve.Affine K) :
    Polynomial (Polynomial (ThreeInputs K)) :=
  resultantStep 4 (multivariateS4 W)
    (MvPolynomial.C W.b₂) (MvPolynomial.C W.b₄)
    (MvPolynomial.C W.b₆) (MvPolynomial.C W.b₈)

theorem symbolicS5_last_natDegree (W : WeierstrassCurve.Affine K) :
    (symbolicS5 W).natDegree=8 := by
  have hn : multivariateS4 W≠0 := by
    intro hz
    have hd := multivariateS4_natDegree W
    rw [hz] at hd
    norm_num at hd
  exact resultantStep_natDegree 4 (multivariateS4 W) _ _ _ _
    (le_of_eq (multivariateS4_natDegree W)) hn

/-- Explicit packet binding: universal fixed-size step, a closed false
reset-of-size control, and the scoped canonical S5 last-input degree. -/
theorem resultant_degree_step_audit_certificate [IsDomain R]
    (b2 b4 b6 b8 : R) (W : WeierstrassCurve.Affine K) :
    (∀ (D : ℕ) (f : R[X]), f.natDegree≤D →
      (resultantStep D f b2 b4 b6 b8).coeff (D*2)=f^2 ∧
      (resultantStep D f b2 b4 b6 b8).natDegree≤D*2) ∧
    (∀ (D : ℕ) (f : R[X]), f.natDegree≤D → f≠0 →
      (resultantStep D f b2 b4 b6 b8).natDegree=D*2) ∧
    resultantStep 2 (1:R[X]) b2 b4 b6 b8≠resultantStep 0 (1:R[X]) b2 b4 b6 b8 ∧
    (symbolicS5 W).natDegree=8 := by
  exact ⟨fun D f hf=>⟨resultantStep_coeff_top D f b2 b4 b6 b8 hf,
      resultantStep_natDegree_le D f b2 b4 b6 b8⟩,
    fun D f hf hn=>resultantStep_natDegree D f b2 b4 b6 b8 hf hn,
    resetting_declared_degree_changes_result b2 b4 b6 b8,
    symbolicS5_last_natDegree W⟩

#print axioms determinant_coeff_column_bounds
#print axioms secondLift_coeff_top
#print axioms resultantStep_coeff_top
#print axioms symbolicS5_last_natDegree
#print axioms resultant_degree_step_audit_certificate
end
end ResultantDegreeStepAudit
