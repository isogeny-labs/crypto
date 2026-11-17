import RecursiveSemaevSymmetry

/-! Degree of a surviving old coordinate through the actual fixed-size
resultant, and its canonical third-newest input corollary. Transport of
every older canonical input remains a separate obligation. -/
namespace RecursiveSemaevFocusAudit
noncomputable section
open Polynomial S4DegreeAudit ResultantDegreeStepAudit
variable {S : Type*} [CommRing S]

/-- Read an old coefficient variable after the two new variables:
(old,Z,T) becomes (Z,T,old). -/
def rotate : S[X][X][X] →+* S[X][X][X] :=
  Polynomial.Bivariate.swap.toRingHom.comp
    (Polynomial.mapRingHom Polynomial.Bivariate.swap.toRingHom)

theorem rotate_scalar_polynomial (a : S[X]) :
    rotate (C (C a))=a.map ((C : S[X]→+*S[X][X]).comp (C : S→+*S[X])) := by
  simp [rotate,RingHom.comp_apply,Polynomial.Bivariate.swap_C,Polynomial.map_map]

theorem rotate_Z : rotate (C (X:S[X][X]))=C (C (X:S[X])) := by
  simp [rotate,RingHom.comp_apply,Polynomial.Bivariate.swap_Y,
    Polynomial.Bivariate.swap_C_C]

theorem rotate_T : rotate (X:S[X][X][X])=C (X:S[X][X]) := by
  simp [rotate,RingHom.comp_apply,Polynomial.Bivariate.swap_Y]

theorem rotate_scalar (a : S) : rotate (C (C (C a)))=C (C (C a)) := by
  rw [rotate_scalar_polynomial]
  simp

theorem coeff_swap (f : S[X][X]) (i j : ℕ) :
    ((Polynomial.Bivariate.swap f).coeff i).coeff j=(f.coeff j).coeff i := by
  induction f using Polynomial.induction_on' with
  | add f g hf hg => simp only [map_add,coeff_add]; rw [hf,hg]
  | monomial n a =>
    rw [Polynomial.Bivariate.swap_monomial]
    simp only [coeff_mul_C,coeff_map,coeff_C_mul]
    by_cases h : n=j
    · subst j
      simp
    · simp [coeff_monomial,h,Ne.symm h]

theorem old_coeff_degree_le (f : S[X][X]) (k : ℕ) :
    (f.coeff k).natDegree≤(Polynomial.Bivariate.swap f).natDegree := by
  apply Polynomial.natDegree_le_iff_coeff_eq_zero.mpr
  intro d hd
  rw [←coeff_swap f d k,
    Polynomial.coeff_eq_zero_of_natDegree_lt hd,Polynomial.coeff_zero]

theorem rotated_first_coeff (f : S[X][X]) (k : ℕ) :
    ((firstLift f).map rotate).coeff k=
      (f.coeff k).map ((C : S[X]→+*S[X][X]).comp (C : S→+*S[X])) := by
  rw [coeff_map,firstLift_coeff,rotate_scalar_polynomial]

theorem rotated_first_coeff_degree (f : S[X][X]) (k : ℕ) :
    (((firstLift f).map rotate).coeff k).natDegree≤
      (Polynomial.Bivariate.swap f).natDegree := by
  rw [rotated_first_coeff]
  exact Polynomial.natDegree_map_le.trans (old_coeff_degree_le f k)

theorem map_qa {A B : Type*} [CommRing A] [CommRing B] (φ : A→+*B) (x y : A) :
    φ (qaRing x y)=qaRing (φ x) (φ y) := by simp [qaRing]

theorem map_qb {A B : Type*} [CommRing A] [CommRing B] (φ : A→+*B)
    (b2 b4 b6 x y : A) :
    φ (qbRing b2 b4 b6 x y)=qbRing (φ b2) (φ b4) (φ b6) (φ x) (φ y) := by
  simp [qbRing,map_ofNat]

theorem map_qc {A B : Type*} [CommRing A] [CommRing B] (φ : A→+*B)
    (b4 b6 b8 x y : A) :
    φ (qcRing b4 b6 b8 x y)=qcRing (φ b4) (φ b6) (φ b8) (φ x) (φ y) := by
  simp [qcRing]

theorem map_quad {A B : Type*} [CommRing A] [CommRing B] (φ : A→+*B) (a b c : A) :
    (quad a b c).map φ=quad (φ a) (φ b) (φ c) := by simp [quad]

theorem rotated_second (b2 b4 b6 b8 : S) :
    (secondLift (C b2) (C b4) (C b6) (C b8)).map rotate=
      (secondLift b2 b4 b6 b8).map (C : S[X][X]→+*S[X][X][X]) := by
  dsimp only [secondLift]
  rw [map_quad (A:=S[X][X][X]) (B:=S[X][X][X]) (rotate (S:=S)),
    map_quad (A:=S[X][X]) (B:=S[X][X][X]) C]
  simp only [map_qa (A:=S[X][X][X]) (B:=S[X][X][X]) rotate,
    map_qb (A:=S[X][X][X]) (B:=S[X][X][X]) rotate,
    map_qc (A:=S[X][X][X]) (B:=S[X][X][X]) rotate,
    map_qa (A:=S[X][X]) (B:=S[X][X][X]) C,
    map_qb (A:=S[X][X]) (B:=S[X][X][X]) C,
    map_qc (A:=S[X][X]) (B:=S[X][X][X]) C,rotate_Z (S:=S),rotate_T (S:=S),
    rotate_scalar (S:=S)]

theorem rotated_first_top (f : S[X][X]) (k d : ℕ) :
    (((firstLift f).map rotate).coeff k).coeff d=
      (firstLift ((Polynomial.Bivariate.swap f).coeff d)).coeff k := by
  rw [rotated_first_coeff (S:=S) f k]
  simp only [coeff_map,firstLift_coeff,coeff_swap,RingHom.comp_apply]

def focusColumnBound (D d : ℕ) (j : Fin (D+2)) : ℕ :=
  j.addCases (fun _ : Fin D=>0) (fun _ : Fin 2=>d)

theorem focusColumnBound_sum (D d : ℕ) :
    ∑ j : Fin (D+2),focusColumnBound D d j=2*d := by
  simp [focusColumnBound,Fin.sum_univ_add,Nat.mul_comm]

theorem rotated_column_degrees (D : ℕ) (f : S[X][X]) (b2 b4 b6 b8 : S) :
    ∀ i j : Fin (D+2),
      ((Polynomial.sylvester ((firstLift f).map rotate)
        ((secondLift (C b2) (C b4) (C b6) (C b8)).map rotate) D 2) i j).natDegree≤
      focusColumnBound D (Polynomial.Bivariate.swap f).natDegree j := by
  intro i j
  induction j using Fin.addCases
  · simp only [Polynomial.sylvester,Matrix.of_apply,focusColumnBound,Fin.addCases_left]
    split_ifs
    · rw [rotated_second,coeff_map]
      simp
    · simp
  · simp only [Polynomial.sylvester,Matrix.of_apply,focusColumnBound,Fin.addCases_right]
    split_ifs
    · exact rotated_first_coeff_degree f _
    · simp

theorem old_resultant_degree_le (D : ℕ) (f : S[X][X]) (b2 b4 b6 b8 : S) :
    (rotate (resultantStep D f (C b2) (C b4) (C b6) (C b8))).natDegree≤
      2*(Polynomial.Bivariate.swap f).natDegree := by
  unfold resultantStep
  rw [←Polynomial.resultant_map_map (R:=S[X][X][X]) (S:=S[X][X][X])
    (firstLift f) (secondLift (C b2) (C b4) (C b6) (C b8)) D 2 rotate]
  exact (determinant_natDegree_le_column_bounds _ _
    (rotated_column_degrees D f b2 b4 b6 b8)).trans_eq (focusColumnBound_sum ..)

theorem old_resultant_coeff_top (D : ℕ) (f : S[X][X]) (b2 b4 b6 b8 : S) :
    (rotate (resultantStep D f (C b2) (C b4) (C b6) (C b8))).coeff
        (2*(Polynomial.Bivariate.swap f).natDegree)=
      resultantStep D ((Polynomial.Bivariate.swap f).leadingCoeff) b2 b4 b6 b8 := by
  have hc := determinant_coeff_column_bounds _ _
    (rotated_column_degrees D f b2 b4 b6 b8)
  rw [focusColumnBound_sum] at hc
  unfold resultantStep
  rw [←Polynomial.resultant_map_map (R:=S[X][X][X]) (S:=S[X][X][X])
    (firstLift f) (secondLift (C b2) (C b4) (C b6) (C b8)) D 2 rotate]
  simp only [Polynomial.resultant]
  rw [hc]
  apply congrArg Matrix.det
  apply Matrix.ext
  intro i j
  induction j using Fin.addCases
  · simp only [Polynomial.sylvester,Matrix.of_apply,focusColumnBound,Fin.addCases_left]
    split_ifs
    · rw [rotated_second,coeff_map]
      simp
    · simp
  · simp only [Polynomial.sylvester,Matrix.of_apply,focusColumnBound,Fin.addCases_right]
    split_ifs
    · exact rotated_first_top f _ _
    · simp

theorem old_leading_degree (D : ℕ) (f : S[X][X]) (hf : f.natDegree≤D) :
    (Polynomial.Bivariate.swap f).leadingCoeff.natDegree≤D := by
  apply Polynomial.natDegree_le_iff_coeff_eq_zero.mpr
  intro k hk
  change ((Polynomial.Bivariate.swap f).coeff (Polynomial.Bivariate.swap f).natDegree).coeff k=0
  rw [coeff_swap,Polynomial.coeff_eq_zero_of_natDegree_lt (hf.trans_lt hk),coeff_zero]

theorem old_leading_nonzero (f : S[X][X]) (hn : f≠0) :
    (Polynomial.Bivariate.swap f).leadingCoeff≠0 := by
  apply Polynomial.leadingCoeff_ne_zero.mpr
  intro hz
  have hh := congrArg (Polynomial.Bivariate.swap (R:=S)) hz
  apply hn
  simpa only [Polynomial.Bivariate.swap_swap_apply,map_zero] using hh

theorem old_resultant_degree [IsDomain S] (D : ℕ) (f : S[X][X])
    (b2 b4 b6 b8 : S) (hf : f.natDegree≤D) (hn : f≠0) :
    (rotate (resultantStep D f (C b2) (C b4) (C b6) (C b8))).natDegree=
      2*(Polynomial.Bivariate.swap f).natDegree := by
  apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
    (old_resultant_degree_le D f b2 b4 b6 b8)
  rw [old_resultant_coeff_top (S:=S) D f b2 b4 b6 b8]
  intro hz
  have ht := resultantStep_coeff_top (R:=S) D (Polynomial.Bivariate.swap f).leadingCoeff
    b2 b4 b6 b8 (old_leading_degree D f hf)
  rw [hz,coeff_zero] at ht
  exact pow_ne_zero 2 (old_leading_nonzero f hn) ht.symm

/-- Fully quantified actual determinant theorem. The coefficients b_i are
constant with respect to the surviving old variable. The map rotate really
moves that variable outside the new Z,T variables. -/
theorem surviving_input_degree_audit_certificate :
    ∀ (A : Type*) [CommRing A] [IsDomain A] (D : ℕ) (f : A[X][X])
      (b2 b4 b6 b8 : A), f.natDegree≤D → f≠0 →
      (rotate (resultantStep D f (C b2) (C b4) (C b6) (C b8))).natDegree=
        2*(Polynomial.Bivariate.swap f).natDegree ∧
      (rotate (resultantStep D f (C b2) (C b4) (C b6) (C b8))).coeff
        (2*(Polynomial.Bivariate.swap f).natDegree)=
        resultantStep D (Polynomial.Bivariate.swap f).leadingCoeff b2 b4 b6 b8 := by
  intro A _ _ D f b2 b4 b6 b8 hf hn
  exact ⟨old_resultant_degree D f b2 b4 b6 b8 hf hn,
    old_resultant_coeff_top D f b2 b4 b6 b8⟩

open RecursiveSemaevDegreeAudit RecursiveSemaevSymmetryAudit

set_option backward.isDefEq.respectTransparency false in
theorem recursive_third_newest_degree {K : Type*} [Field K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (rotate (S:=InputRing K n) (recursivePolynomial W (n+2))).natDegree=2^(n+3) := by
  change (rotate (S:=InputRing K n)
    (resultantStep (2^(n+2)) (recursivePolynomial W (n+1))
      (C (coefficientEmbedding n W.b₂)) (C (coefficientEmbedding n W.b₄))
      (C (coefficientEmbedding n W.b₆)) (C (coefficientEmbedding n W.b₈)))).natDegree=_
  have hf : (recursivePolynomial W (n+1)).natDegree≤2^(n+2) := by
    exact le_of_eq (by simpa only [Nat.add_assoc] using recursivePolynomial_natDegree W (n+1))
  rw [old_resultant_degree (S:=InputRing K n) _ _ _ _ _ _ hf
    (recursivePolynomial_nonzero W (n+1))]
  have hd := recursive_penultimate_degree W n
  change (Polynomial.Bivariate.swap (recursivePolynomial W (n+1))).natDegree=2^(n+2) at hd
  rw [hd,show n+3=(n+2)+1 by omega,pow_succ]
  ring

theorem focus_degree_audit_certificate :
    (∀ (A : Type*) [CommRing A] [IsDomain A] (D : ℕ) (f : A[X][X])
      (b2 b4 b6 b8 : A), f.natDegree≤D → f≠0 →
      (rotate (resultantStep D f (C b2) (C b4) (C b6) (C b8))).natDegree=
        2*(Polynomial.Bivariate.swap f).natDegree ∧
      (rotate (resultantStep D f (C b2) (C b4) (C b6) (C b8))).coeff
        (2*(Polynomial.Bivariate.swap f).natDegree)=
        resultantStep D (Polynomial.Bivariate.swap f).leadingCoeff b2 b4 b6 b8) ∧
    (∀ (K : Type*) [Field K] (W : WeierstrassCurve.Affine K) (n : ℕ),
      (rotate (S:=InputRing K n) (recursivePolynomial W (n+2))).natDegree=2^(n+3)) := by
  constructor
  · exact surviving_input_degree_audit_certificate
  · intro K _ W n
    exact recursive_third_newest_degree W n

#print axioms focus_degree_audit_certificate
#print axioms recursive_third_newest_degree
#print axioms surviving_input_degree_audit_certificate
#print axioms old_resultant_degree
#print axioms old_resultant_degree_le
#print axioms old_resultant_coeff_top
#print axioms coeff_swap
#print axioms old_coeff_degree_le
#print axioms rotated_second
end
end RecursiveSemaevFocusAudit
