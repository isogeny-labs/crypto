import MumfordStollProofRules

/-! Candidate certificates are accepted only by the Lean kernel below.
Bezout choices are fixed before defining the deterministic arithmetic. -/
namespace MumfordStollFullCertificatesAudit
noncomputable section
open Polynomial MumfordStollAudit MumfordStollGeneralAudit
set_option backward.isDefEq.respectTransparency false
set_option maxHeartbeats 1200000

attribute [local mumford_stoll_coeff] Polynomial.coeff_C Polynomial.coeff_one Polynomial.coeff_zero
  Polynomial.coeff_X_pow Polynomial.coeff_X Polynomial.coeff_mul_C Polynomial.coeff_C_mul
  Polynomial.coeff_add Polynomial.coeff_sub Polynomial.coeff_neg
  MumfordStollCantorAudit.coefficient_constant_power
  MumfordStollProofRulesAudit.coeff_constant_product
  MumfordStollProofRulesAudit.coeff_right_constant_power
  MumfordStollProofRulesAudit.coeff_left_constant_power

def repPerm (i : Fin 10) : Fin 10 :=
  match i with
  | 0 => 0
  | 1 => 6
  | 2 => 2
  | 3 => 8
  | 4 => 4
  | 5 => 5
  | 6 => 1
  | 7 => 9
  | 8 => 3
  | _ => 7

def expectedIndex (i j : Fin 10) : Fin 10 := repPerm (repPerm i+repPerm j)

-- Candidate Bezout choices only. Their equations must be proved by Lean.
def bezoutA (i j : Fin 10) : K[X] :=
  match i,j with
  | 0,0 => 0
  | 0,1 => 0
  | 0,2 => 0
  | 0,3 => 0
  | 0,4 => 0
  | 0,5 => (1)
  | 0,6 => (1)
  | 0,7 => (1)
  | 0,8 => (1)
  | 0,9 => (1)
  | 1,0 => 0
  | 1,1 => 0
  | 1,2 => 0
  | 1,3 => (1)
  | 1,4 => 0
  | 1,5 => 0
  | 1,6 => (1)
  | 1,7 => 0
  | 1,8 => (1+X)
  | 1,9 => (1+X)
  | 2,0 => 0
  | 2,1 => 0
  | 2,2 => 0
  | 2,3 => 0
  | 2,4 => 0
  | 2,5 => 0
  | 2,6 => (Polynomial.C (2:K))
  | 2,7 => 0
  | 2,8 => (Polynomial.C (2:K)+X)
  | 2,9 => (Polynomial.C (2:K)+X)
  | 3,0 => 0
  | 3,1 => 0
  | 3,2 => 0
  | 3,3 => 0
  | 3,4 => 0
  | 3,5 => 0
  | 3,6 => 0
  | 3,7 => (Polynomial.C (2:K))
  | 3,8 => (Polynomial.C (2:K)+X)
  | 3,9 => (Polynomial.C (2:K)+X)
  | 4,0 => 0
  | 4,1 => 0
  | 4,2 => (1)
  | 4,3 => 0
  | 4,4 => 0
  | 4,5 => 0
  | 4,6 => 0
  | 4,7 => (1)
  | 4,8 => (1+X)
  | 4,9 => (1+X)
  | 5,0 => 0
  | 5,1 => 0
  | 5,2 => 0
  | 5,3 => 0
  | 5,4 => 0
  | 5,5 => 0
  | 5,6 => 0
  | 5,7 => 0
  | 5,8 => (X)
  | 5,9 => (X)
  | 6,0 => 0
  | 6,1 => 0
  | 6,2 => (1)
  | 6,3 => 0
  | 6,4 => 0
  | 6,5 => 0
  | 6,6 => 0
  | 6,7 => 0
  | 6,8 => (1)
  | 6,9 => 0
  | 7,0 => 0
  | 7,1 => 0
  | 7,2 => 0
  | 7,3 => (1)
  | 7,4 => 0
  | 7,5 => 0
  | 7,6 => 0
  | 7,7 => 0
  | 7,8 => 0
  | 7,9 => (1)
  | 8,0 => 0
  | 8,1 => (Polynomial.C (2:K))
  | 8,2 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 8,3 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 8,4 => (Polynomial.C (2:K))
  | 8,5 => (Polynomial.C (2:K))
  | 8,6 => (Polynomial.C (2:K))
  | 8,7 => 0
  | 8,8 => 0
  | 8,9 => 0
  | 9,0 => 0
  | 9,1 => (Polynomial.C (2:K))
  | 9,2 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 9,3 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 9,4 => (Polynomial.C (2:K))
  | 9,5 => (Polynomial.C (2:K))
  | 9,6 => 0
  | 9,7 => (Polynomial.C (2:K))
  | 9,8 => 0
  | _,_ => 0

def bezoutB (i j : Fin 10) : K[X] :=
  match i,j with
  | 0,0 => (1)
  | 0,1 => 0
  | 0,2 => 0
  | 0,3 => 0
  | 0,4 => 0
  | 0,5 => 0
  | 0,6 => 0
  | 0,7 => 0
  | 0,8 => 0
  | 0,9 => 0
  | 1,0 => 0
  | 1,1 => 0
  | 1,2 => 0
  | 1,3 => 0
  | 1,4 => (1)
  | 1,5 => 0
  | 1,6 => 0
  | 1,7 => 0
  | 1,8 => (Polynomial.C (2:K))
  | 1,9 => (Polynomial.C (2:K))
  | 2,0 => 0
  | 2,1 => 0
  | 2,2 => 0
  | 2,3 => (1)
  | 2,4 => (1)
  | 2,5 => 0
  | 2,6 => (1)
  | 2,7 => 0
  | 2,8 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 2,9 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 3,0 => 0
  | 3,1 => (1)
  | 3,2 => (1)
  | 3,3 => 0
  | 3,4 => 0
  | 3,5 => 0
  | 3,6 => 0
  | 3,7 => (1)
  | 3,8 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 3,9 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 4,0 => 0
  | 4,1 => (1)
  | 4,2 => 0
  | 4,3 => 0
  | 4,4 => 0
  | 4,5 => 0
  | 4,6 => 0
  | 4,7 => 0
  | 4,8 => (Polynomial.C (2:K))
  | 4,9 => (Polynomial.C (2:K))
  | 5,0 => (1)
  | 5,1 => 0
  | 5,2 => 0
  | 5,3 => 0
  | 5,4 => 0
  | 5,5 => (1)
  | 5,6 => 0
  | 5,7 => 0
  | 5,8 => (Polynomial.C (2:K))
  | 5,9 => (Polynomial.C (2:K))
  | 6,0 => (1)
  | 6,1 => (1)
  | 6,2 => (Polynomial.C (2:K))
  | 6,3 => 0
  | 6,4 => 0
  | 6,5 => 0
  | 6,6 => 0
  | 6,7 => (1)
  | 6,8 => (Polynomial.C (2:K))
  | 6,9 => 0
  | 7,0 => (1)
  | 7,1 => 0
  | 7,2 => 0
  | 7,3 => (Polynomial.C (2:K))
  | 7,4 => (1)
  | 7,5 => 0
  | 7,6 => (1)
  | 7,7 => 0
  | 7,8 => 0
  | 7,9 => (Polynomial.C (2:K))
  | 8,0 => (1)
  | 8,1 => (1+X)
  | 8,2 => (Polynomial.C (2:K)+X)
  | 8,3 => (Polynomial.C (2:K)+X)
  | 8,4 => (1+X)
  | 8,5 => (X)
  | 8,6 => (1)
  | 8,7 => 0
  | 8,8 => (Polynomial.C (2:K))
  | 8,9 => (1)
  | 9,0 => (1)
  | 9,1 => (1+X)
  | 9,2 => (Polynomial.C (2:K)+X)
  | 9,3 => (Polynomial.C (2:K)+X)
  | 9,4 => (1+X)
  | 9,5 => (X)
  | 9,6 => 0
  | 9,7 => (1)
  | 9,8 => (1)
  | _,_ => (Polynomial.C (2:K))

def bezoutC (i j : Fin 10) : K[X] :=
  match i,j with
  | 0,0 => 0
  | 0,1 => (1)
  | 0,2 => (1)
  | 0,3 => (Polynomial.C (2:K))
  | 0,4 => (Polynomial.C (2:K))
  | 0,5 => 0
  | 0,6 => 0
  | 0,7 => 0
  | 0,8 => 0
  | 0,9 => 0
  | 1,0 => (1)
  | 1,1 => (Polynomial.C (2:K))
  | 1,2 => (Polynomial.C (2:K))
  | 1,3 => 0
  | 1,4 => 0
  | 1,5 => (1)
  | 1,6 => (Polynomial.C (2:K))
  | 1,7 => (Polynomial.C (2:K))
  | 1,8 => 0
  | 1,9 => 0
  | 2,0 => (1)
  | 2,1 => (Polynomial.C (2:K))
  | 2,2 => (Polynomial.C (2:K))
  | 2,3 => 0
  | 2,4 => 0
  | 2,5 => (1)
  | 2,6 => (Polynomial.C (2:K))
  | 2,7 => (Polynomial.C (2:K))
  | 2,8 => 0
  | 2,9 => 0
  | 3,0 => (Polynomial.C (2:K))
  | 3,1 => 0
  | 3,2 => 0
  | 3,3 => (1)
  | 3,4 => (1)
  | 3,5 => (Polynomial.C (2:K))
  | 3,6 => (1)
  | 3,7 => (1)
  | 3,8 => 0
  | 3,9 => 0
  | 4,0 => (Polynomial.C (2:K))
  | 4,1 => 0
  | 4,2 => 0
  | 4,3 => (1)
  | 4,4 => (1)
  | 4,5 => (Polynomial.C (2:K))
  | 4,6 => (1)
  | 4,7 => (1)
  | 4,8 => 0
  | 4,9 => 0
  | 5,0 => 0
  | 5,1 => (1)
  | 5,2 => (1)
  | 5,3 => (Polynomial.C (2:K))
  | 5,4 => (Polynomial.C (2:K))
  | 5,5 => 0
  | 5,6 => (1)
  | 5,7 => (Polynomial.C (2:K))
  | 5,8 => 0
  | 5,9 => 0
  | 6,0 => 0
  | 6,1 => (Polynomial.C (2:K))
  | 6,2 => (Polynomial.C (2:K))
  | 6,3 => (1)
  | 6,4 => (1)
  | 6,5 => (1)
  | 6,6 => (Polynomial.C (2:K))
  | 6,7 => 0
  | 6,8 => 0
  | 6,9 => (1)
  | 7,0 => 0
  | 7,1 => (Polynomial.C (2:K))
  | 7,2 => (Polynomial.C (2:K))
  | 7,3 => (1)
  | 7,4 => (1)
  | 7,5 => (Polynomial.C (2:K))
  | 7,6 => 0
  | 7,7 => (1)
  | 7,8 => (Polynomial.C (2:K))
  | 7,9 => 0
  | 8,0 => 0
  | 8,1 => 0
  | 8,2 => 0
  | 8,3 => 0
  | 8,4 => 0
  | 8,5 => 0
  | 8,6 => 0
  | 8,7 => (Polynomial.C (2:K))
  | 8,8 => (Polynomial.C (2:K)+Polynomial.C (2:K)*X)
  | 8,9 => 0
  | 9,0 => 0
  | 9,1 => 0
  | 9,2 => 0
  | 9,3 => 0
  | 9,4 => 0
  | 9,5 => 0
  | 9,6 => (1)
  | 9,7 => 0
  | 9,8 => 0
  | _,_ => (1+X)

macro "stoll_full_poly" : tactic =>
  `(tactic| (ring_nf <;> ((try simp only [mul_assoc,←map_ofNat (Polynomial.C : K →+* K[X]),←map_mul (Polynomial.C : K →+* K[X]),←map_pow (Polynomial.C : K →+* K[X])]) <;> (first | exact Or.inl True.intro | (ext n <;> ((try simp only [mumford_stoll_coeff]) <;> ((try split_ifs) <;> norm_num <;> (try simp_all) <;> decide)))))))

theorem cert_0_0 : GeneralCantorRun (bezoutA 0 0)
    (bezoutB 0 0) (bezoutC 0 0) (point 0) (point 0)
    (point (expectedIndex 0 0)) := by
  refine ⟨(1),(1),0,0,(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_1 : GeneralCantorRun (bezoutA 0 1)
    (bezoutB 0 1) (bezoutC 0 1) (point 0) (point 1)
    (point (expectedIndex 0 1)) := by
  refine ⟨(1),(X),(1),(X^4),(1),(X),(1),(X^4),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_2 : GeneralCantorRun (bezoutA 0 2)
    (bezoutB 0 2) (bezoutC 0 2) (point 0) (point 2)
    (point (expectedIndex 0 2)) := by
  refine ⟨(1),(X^2),(1),(X^3),(1),(X^2),(1),(X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_3 : GeneralCantorRun (bezoutA 0 3)
    (bezoutB 0 3) (bezoutC 0 3) (point 0) (point 3)
    (point (expectedIndex 0 3)) := by
  refine ⟨(1),(X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)*X^3),(1),(X^2),(Polynomial.C (2:K)),(X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_4 : GeneralCantorRun (bezoutA 0 4)
    (bezoutB 0 4) (bezoutC 0 4) (point 0) (point 4)
    (point (expectedIndex 0 4)) := by
  refine ⟨(1),(X),(Polynomial.C (2:K)),(Polynomial.C (2:K)*X^4),(1),(X),(Polynomial.C (2:K)),(X^4),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_5 : GeneralCantorRun (bezoutA 0 5)
    (bezoutB 0 5) (bezoutC 0 5) (point 0) (point 5)
    (point (expectedIndex 0 5)) := by
  refine ⟨(1),(1+X),0,0,(1),(1+X),0,(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3+X^4),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1+X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1+X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_6 : GeneralCantorRun (bezoutA 0 6)
    (bezoutB 0 6) (bezoutC 0 6) (point 0) (point 6)
    (point (expectedIndex 0 6)) := by
  refine ⟨(1),(X+X^2),(1+X),0,(1),(X+X^2),(1+X),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_7 : GeneralCantorRun (bezoutA 0 7)
    (bezoutB 0 7) (bezoutC 0 7) (point 0) (point 7)
    (point (expectedIndex 0 7)) := by
  refine ⟨(1),(X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),0,(1),(X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_8 : GeneralCantorRun (bezoutA 0 8)
    (bezoutB 0 8) (bezoutC 0 8) (point 0) (point 8)
    (point (expectedIndex 0 8)) := by
  refine ⟨(1),(Polynomial.C (2:K)+X+X^2),(X),0,(1),(Polynomial.C (2:K)+X+X^2),(X),(Polynomial.C (2:K)+Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)+X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((Polynomial.C (2:K)+X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_0_9 : GeneralCantorRun (bezoutA 0 9)
    (bezoutB 0 9) (bezoutC 0 9) (point 0) (point 9)
    (point (expectedIndex 0 9)) := by
  refine ⟨(1),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)*X),0,(1),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)*X),(Polynomial.C (2:K)+Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)+X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)*X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((Polynomial.C (2:K)+X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_0 : GeneralCantorRun (bezoutA 1 0)
    (bezoutB 1 0) (bezoutC 1 0) (point 1) (point 0)
    (point (expectedIndex 1 0)) := by
  refine ⟨(1),(X),(1),(X^4),(X),(1),(1),(X^4),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_1 : GeneralCantorRun (bezoutA 1 1)
    (bezoutB 1 1) (bezoutC 1 1) (point 1) (point 1)
    (point (expectedIndex 1 1)) := by
  refine ⟨(1),(X^2),(1),(Polynomial.C (2:K)*X^3),(X),(X),(Polynomial.C (2:K)),(X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_2 : GeneralCantorRun (bezoutA 1 2)
    (bezoutB 1 2) (bezoutC 1 2) (point 1) (point 2)
    (point (expectedIndex 1 2)) := by
  refine ⟨(1),(X^3),(1),(Polynomial.C (2:K)*X^2),(X),(X^2),(Polynomial.C (2:K)),(X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),0,(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_3 : GeneralCantorRun (bezoutA 1 3)
    (bezoutB 1 3) (bezoutC 1 3) (point 1) (point 3)
    (point (expectedIndex 1 3)) := by
  refine ⟨(X),(X),(Polynomial.C (2:K)),0,(1),(X),0,(X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_4 : GeneralCantorRun (bezoutA 1 4)
    (bezoutB 1 4) (bezoutC 1 4) (point 1) (point 4)
    (point (expectedIndex 1 4)) := by
  refine ⟨(X),(1),0,(1),(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_5 : GeneralCantorRun (bezoutA 1 5)
    (bezoutB 1 5) (bezoutC 1 5) (point 1) (point 5)
    (point (expectedIndex 1 5)) := by
  refine ⟨(1),(X+X^2),(1+X),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2+X^3),(X),(1+X),(1),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_6 : GeneralCantorRun (bezoutA 1 6)
    (bezoutB 1 6) (bezoutC 1 6) (point 1) (point 6)
    (point (expectedIndex 1 6)) := by
  refine ⟨(1),(X^2+X^3),(1+Polynomial.C (2:K)*X^2),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2),(X),(X+X^2),(Polynomial.C (2:K)+X),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_7 : GeneralCantorRun (bezoutA 1 7)
    (bezoutB 1 7) (bezoutC 1 7) (point 1) (point 7)
    (point (expectedIndex 1 7)) := by
  refine ⟨(X),(1+X),0,(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3),(1),(1+X),(Polynomial.C (2:K)),(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3+X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1+X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1+X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_8 : GeneralCantorRun (bezoutA 1 8)
    (bezoutB 1 8) (bezoutC 1 8) (point 1) (point 8)
    (point (expectedIndex 1 8)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+X^2+X^3),(1+Polynomial.C (2:K)*X^2),(1),(X),(Polynomial.C (2:K)+X+X^2),(1+X),(X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_1_9 : GeneralCantorRun (bezoutA 1 9)
    (bezoutB 1 9) (bezoutC 1 9) (point 1) (point 9)
    (point (expectedIndex 1 9)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+X^2+X^3),(1+X+Polynomial.C (2:K)*X^2),(Polynomial.C (2:K)),(X),(Polynomial.C (2:K)+X+X^2),(1+Polynomial.C (2:K)*X),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_0 : GeneralCantorRun (bezoutA 2 0)
    (bezoutB 2 0) (bezoutC 2 0) (point 2) (point 0)
    (point (expectedIndex 2 0)) := by
  refine ⟨(1),(X^2),(1),(X^3),(X^2),(1),(1),(X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_1 : GeneralCantorRun (bezoutA 2 1)
    (bezoutB 2 1) (bezoutC 2 1) (point 2) (point 1)
    (point (expectedIndex 2 1)) := by
  refine ⟨(1),(X^3),(1),(Polynomial.C (2:K)*X^2),(X^2),(X),(Polynomial.C (2:K)),(X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),0,(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_2 : GeneralCantorRun (bezoutA 2 2)
    (bezoutB 2 2) (bezoutC 2 2) (point 2) (point 2)
    (point (expectedIndex 2 2)) := by
  refine ⟨(1),(X^4),(1),(Polynomial.C (2:K)*X),(X^2),(X^2),(Polynomial.C (2:K)),(X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),0,(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_3 : GeneralCantorRun (bezoutA 2 3)
    (bezoutB 2 3) (bezoutC 2 3) (point 2) (point 3)
    (point (expectedIndex 2 3)) := by
  refine ⟨(X^2),(1),0,(1),(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_4 : GeneralCantorRun (bezoutA 2 4)
    (bezoutB 2 4) (bezoutC 2 4) (point 2) (point 4)
    (point (expectedIndex 2 4)) := by
  refine ⟨(X),(X),(1),0,(X),(1),0,(X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_5 : GeneralCantorRun (bezoutA 2 5)
    (bezoutB 2 5) (bezoutC 2 5) (point 2) (point 5)
    (point (expectedIndex 2 5)) := by
  refine ⟨(1),(X^2+X^3),(1+Polynomial.C (2:K)*X^2),(1+Polynomial.C (2:K)*X+X^2),(X^2),(1+X),(1),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_6 : GeneralCantorRun (bezoutA 2 6)
    (bezoutB 2 6) (bezoutC 2 6) (point 2) (point 6)
    (point (expectedIndex 2 6)) := by
  refine ⟨(1),(X^3+X^4),(1+X^3),(1+Polynomial.C (2:K)*X),(X^2),(X+X^2),(Polynomial.C (2:K)+X),(1+Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(1+Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_7 : GeneralCantorRun (bezoutA 2 7)
    (bezoutB 2 7) (bezoutC 2 7) (point 2) (point 7)
    (point (expectedIndex 2 7)) := by
  refine ⟨(X),(X+X^2),(1+X),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2),(X),(1+X),(Polynomial.C (2:K)),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_8 : GeneralCantorRun (bezoutA 2 8)
    (bezoutB 2 8) (bezoutC 2 8) (point 2) (point 8)
    (point (expectedIndex 2 8)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X^2+X^3+X^4),(1+Polynomial.C (2:K)*X^2),(1),(X^2),(Polynomial.C (2:K)+X+X^2),(1+X),(1+X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(Polynomial.C (2:K)+X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_2_9 : GeneralCantorRun (bezoutA 2 9)
    (bezoutB 2 9) (bezoutC 2 9) (point 2) (point 9)
    (point (expectedIndex 2 9)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X^2+X^3+X^4),(1+X^3),(Polynomial.C (2:K)),(X^2),(Polynomial.C (2:K)+X+X^2),(1+Polynomial.C (2:K)*X),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(1+Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_0 : GeneralCantorRun (bezoutA 3 0)
    (bezoutB 3 0) (bezoutC 3 0) (point 3) (point 0)
    (point (expectedIndex 3 0)) := by
  refine ⟨(1),(X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)*X^3),(X^2),(1),(Polynomial.C (2:K)),(X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_1 : GeneralCantorRun (bezoutA 3 1)
    (bezoutB 3 1) (bezoutC 3 1) (point 3) (point 1)
    (point (expectedIndex 3 1)) := by
  refine ⟨(X),(X),(Polynomial.C (2:K)),0,(X),(1),0,(X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_2 : GeneralCantorRun (bezoutA 3 2)
    (bezoutB 3 2) (bezoutC 3 2) (point 3) (point 2)
    (point (expectedIndex 3 2)) := by
  refine ⟨(X^2),(1),0,(Polynomial.C (2:K)),(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_3 : GeneralCantorRun (bezoutA 3 3)
    (bezoutB 3 3) (bezoutC 3 3) (point 3) (point 3)
    (point (expectedIndex 3 3)) := by
  refine ⟨(1),(X^4),(Polynomial.C (2:K)),(X),(X^2),(X^2),(1),(X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),0,(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_4 : GeneralCantorRun (bezoutA 3 4)
    (bezoutB 3 4) (bezoutC 3 4) (point 3) (point 4)
    (point (expectedIndex 3 4)) := by
  refine ⟨(1),(X^3),(Polynomial.C (2:K)),(X^2),(X^2),(X),(1),(X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),0,(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_5 : GeneralCantorRun (bezoutA 3 5)
    (bezoutB 3 5) (bezoutC 3 5) (point 3) (point 5)
    (point (expectedIndex 3 5)) := by
  refine ⟨(1),(X^2+X^3),(Polynomial.C (2:K)+X^2),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2),(X^2),(1+X),(Polynomial.C (2:K)),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_6 : GeneralCantorRun (bezoutA 3 6)
    (bezoutB 3 6) (bezoutC 3 6) (point 3) (point 6)
    (point (expectedIndex 3 6)) := by
  refine ⟨(X),(X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1+Polynomial.C (2:K)*X+X^2),(X),(1+X),(1),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_7 : GeneralCantorRun (bezoutA 3 7)
    (bezoutB 3 7) (bezoutC 3 7) (point 3) (point 7)
    (point (expectedIndex 3 7)) := by
  refine ⟨(1),(X^3+X^4),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^3),(Polynomial.C (2:K)+X),(X^2),(X+X^2),(1+Polynomial.C (2:K)*X),(1+Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(Polynomial.C (2:K)+X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_8 : GeneralCantorRun (bezoutA 3 8)
    (bezoutB 3 8) (bezoutC 3 8) (point 3) (point 8)
    (point (expectedIndex 3 8)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X^2+X^3+X^4),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^3),(1),(X^2),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)+X),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(Polynomial.C (2:K)+X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_3_9 : GeneralCantorRun (bezoutA 3 9)
    (bezoutB 3 9) (bezoutC 3 9) (point 3) (point 9)
    (point (expectedIndex 3 9)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X^2+X^3+X^4),(Polynomial.C (2:K)+X^2),(Polynomial.C (2:K)),(X^2),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1+X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(1+Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_0 : GeneralCantorRun (bezoutA 4 0)
    (bezoutB 4 0) (bezoutC 4 0) (point 4) (point 0)
    (point (expectedIndex 4 0)) := by
  refine ⟨(1),(X),(Polynomial.C (2:K)),(Polynomial.C (2:K)*X^4),(X),(1),(Polynomial.C (2:K)),(X^4),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_1 : GeneralCantorRun (bezoutA 4 1)
    (bezoutB 4 1) (bezoutC 4 1) (point 4) (point 1)
    (point (expectedIndex 4 1)) := by
  refine ⟨(X),(1),0,(Polynomial.C (2:K)),(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_2 : GeneralCantorRun (bezoutA 4 2)
    (bezoutB 4 2) (bezoutC 4 2) (point 4) (point 2)
    (point (expectedIndex 4 2)) := by
  refine ⟨(X),(X),(1),0,(1),(X),0,(X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_3 : GeneralCantorRun (bezoutA 4 3)
    (bezoutB 4 3) (bezoutC 4 3) (point 4) (point 3)
    (point (expectedIndex 4 3)) := by
  refine ⟨(1),(X^3),(Polynomial.C (2:K)),(X^2),(X),(X^2),(1),(X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),0,(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_4 : GeneralCantorRun (bezoutA 4 4)
    (bezoutB 4 4) (bezoutC 4 4) (point 4) (point 4)
    (point (expectedIndex 4 4)) := by
  refine ⟨(1),(X^2),(Polynomial.C (2:K)),(X^3),(X),(X),(1),(X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_5 : GeneralCantorRun (bezoutA 4 5)
    (bezoutB 4 5) (bezoutC 4 5) (point 4) (point 5)
    (point (expectedIndex 4 5)) := by
  refine ⟨(1),(X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3),(X),(1+X),(Polynomial.C (2:K)),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_6 : GeneralCantorRun (bezoutA 4 6)
    (bezoutB 4 6) (bezoutC 4 6) (point 4) (point 6)
    (point (expectedIndex 4 6)) := by
  refine ⟨(X),(1+X),0,(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2+X^3),(1),(1+X),(1),(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3+X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1+X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1+X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_7 : GeneralCantorRun (bezoutA 4 7)
    (bezoutB 4 7) (bezoutC 4 7) (point 4) (point 7)
    (point (expectedIndex 4 7)) := by
  refine ⟨(1),(X^2+X^3),(Polynomial.C (2:K)+X^2),(1+Polynomial.C (2:K)*X+X^2),(X),(X+X^2),(1+Polynomial.C (2:K)*X),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_8 : GeneralCantorRun (bezoutA 4 8)
    (bezoutB 4 8) (bezoutC 4 8) (point 4) (point 8)
    (point (expectedIndex 4 8)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+X^2+X^3),(Polynomial.C (2:K)+Polynomial.C (2:K)*X+X^2),(1),(X),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)+X),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_4_9 : GeneralCantorRun (bezoutA 4 9)
    (bezoutB 4 9) (bezoutC 4 9) (point 4) (point 9)
    (point (expectedIndex 4 9)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+X^2+X^3),(Polynomial.C (2:K)+X^2),(Polynomial.C (2:K)),(X),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_0 : GeneralCantorRun (bezoutA 5 0)
    (bezoutB 5 0) (bezoutC 5 0) (point 5) (point 0)
    (point (expectedIndex 5 0)) := by
  refine ⟨(1),(1+X),0,0,(1+X),(1),0,(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3+X^4),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1+X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1+X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_1 : GeneralCantorRun (bezoutA 5 1)
    (bezoutB 5 1) (bezoutC 5 1) (point 5) (point 1)
    (point (expectedIndex 5 1)) := by
  refine ⟨(1),(X+X^2),(1+X),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2+X^3),(1+X),(X),(1),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_2 : GeneralCantorRun (bezoutA 5 2)
    (bezoutB 5 2) (bezoutC 5 2) (point 5) (point 2)
    (point (expectedIndex 5 2)) := by
  refine ⟨(1),(X^2+X^3),(1+Polynomial.C (2:K)*X^2),(1+Polynomial.C (2:K)*X+X^2),(1+X),(X^2),(1),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_3 : GeneralCantorRun (bezoutA 5 3)
    (bezoutB 5 3) (bezoutC 5 3) (point 5) (point 3)
    (point (expectedIndex 5 3)) := by
  refine ⟨(1),(X^2+X^3),(Polynomial.C (2:K)+X^2),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2),(1+X),(X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_4 : GeneralCantorRun (bezoutA 5 4)
    (bezoutB 5 4) (bezoutC 5 4) (point 5) (point 4)
    (point (expectedIndex 5 4)) := by
  refine ⟨(1),(X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3),(1+X),(X),(Polynomial.C (2:K)),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_5 : GeneralCantorRun (bezoutA 5 5)
    (bezoutB 5 5) (bezoutC 5 5) (point 5) (point 5)
    (point (expectedIndex 5 5)) := by
  refine ⟨(1+X),(1),0,0,(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_6 : GeneralCantorRun (bezoutA 5 6)
    (bezoutB 5 6) (bezoutC 5 6) (point 5) (point 6)
    (point (expectedIndex 5 6)) := by
  refine ⟨(1+X),(X),(1),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2+X^3),(1),(X),(1),(X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_7 : GeneralCantorRun (bezoutA 5 7)
    (bezoutB 5 7) (bezoutC 5 7) (point 5) (point 7)
    (point (expectedIndex 5 7)) := by
  refine ⟨(1+X),(X),(Polynomial.C (2:K)),(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3),(1),(X),(Polynomial.C (2:K)),(X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_8 : GeneralCantorRun (bezoutA 5 8)
    (bezoutB 5 8) (bezoutC 5 8) (point 5) (point 8)
    (point (expectedIndex 5 8)) := by
  refine ⟨(1),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3),(1+Polynomial.C (2:K)*X^2),(1),(1+X),(Polynomial.C (2:K)+X+X^2),(X),(X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_5_9 : GeneralCantorRun (bezoutA 5 9)
    (bezoutB 5 9) (bezoutC 5 9) (point 5) (point 9)
    (point (expectedIndex 5 9)) := by
  refine ⟨(1),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3),(Polynomial.C (2:K)+X^2),(Polynomial.C (2:K)),(1+X),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)*X),(X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_0 : GeneralCantorRun (bezoutA 6 0)
    (bezoutB 6 0) (bezoutC 6 0) (point 6) (point 0)
    (point (expectedIndex 6 0)) := by
  refine ⟨(1),(X+X^2),(1+X),0,(X+X^2),(1),(1+X),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_1 : GeneralCantorRun (bezoutA 6 1)
    (bezoutB 6 1) (bezoutC 6 1) (point 6) (point 1)
    (point (expectedIndex 6 1)) := by
  refine ⟨(1),(X^2+X^3),(1+Polynomial.C (2:K)*X^2),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2),(X+X^2),(X),(Polynomial.C (2:K)+X),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_2 : GeneralCantorRun (bezoutA 6 2)
    (bezoutB 6 2) (bezoutC 6 2) (point 6) (point 2)
    (point (expectedIndex 6 2)) := by
  refine ⟨(1),(X^3+X^4),(1+X^3),(1+Polynomial.C (2:K)*X),(X+X^2),(X^2),(Polynomial.C (2:K)+X),(1+Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(1+Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_3 : GeneralCantorRun (bezoutA 6 3)
    (bezoutB 6 3) (bezoutC 6 3) (point 6) (point 3)
    (point (expectedIndex 6 3)) := by
  refine ⟨(X),(X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1+Polynomial.C (2:K)*X+X^2),(1+X),(X),(1),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_4 : GeneralCantorRun (bezoutA 6 4)
    (bezoutB 6 4) (bezoutC 6 4) (point 6) (point 4)
    (point (expectedIndex 6 4)) := by
  refine ⟨(X),(1+X),0,(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2+X^3),(1+X),(1),(1),(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3+X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1+X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1+X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_5 : GeneralCantorRun (bezoutA 6 5)
    (bezoutB 6 5) (bezoutC 6 5) (point 6) (point 5)
    (point (expectedIndex 6 5)) := by
  refine ⟨(1+X),(X),(1),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2+X^3),(X),(1),(1),(X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_6 : GeneralCantorRun (bezoutA 6 6)
    (bezoutB 6 6) (bezoutC 6 6) (point 6) (point 6)
    (point (expectedIndex 6 6)) := by
  refine ⟨(1+X),(X^2),(1),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2),(X),(X),(Polynomial.C (2:K)),(X^3),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_7 : GeneralCantorRun (bezoutA 6 7)
    (bezoutB 6 7) (bezoutC 6 7) (point 6) (point 7)
    (point (expectedIndex 6 7)) := by
  refine ⟨(X+X^2),(1),0,(1+X),(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_8 : GeneralCantorRun (bezoutA 6 8)
    (bezoutB 6 8) (bezoutC 6 8) (point 6) (point 8)
    (point (expectedIndex 6 8)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4),(1+Polynomial.C (2:K)*X^2),0,(X+X^2),(Polynomial.C (2:K)+X+X^2),(1+Polynomial.C (2:K)*X),(X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_6_9 : GeneralCantorRun (bezoutA 6 9)
    (bezoutB 6 9) (bezoutC 6 9) (point 6) (point 9)
    (point (expectedIndex 6 9)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4),(1+X^3),(1+X),(X+X^2),(Polynomial.C (2:K)+X+X^2),(1),(Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_0 : GeneralCantorRun (bezoutA 7 0)
    (bezoutB 7 0) (bezoutC 7 0) (point 7) (point 0)
    (point (expectedIndex 7 0)) := by
  refine ⟨(1),(X+X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),0,(X+X^2),(1),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_1 : GeneralCantorRun (bezoutA 7 1)
    (bezoutB 7 1) (bezoutC 7 1) (point 7) (point 1)
    (point (expectedIndex 7 1)) := by
  refine ⟨(X),(1+X),0,(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3),(1+X),(1),(Polynomial.C (2:K)),(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3+X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1+X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1+X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_2 : GeneralCantorRun (bezoutA 7 2)
    (bezoutB 7 2) (bezoutC 7 2) (point 7) (point 2)
    (point (expectedIndex 7 2)) := by
  refine ⟨(X),(X+X^2),(1+X),(Polynomial.C (2:K)+X+Polynomial.C (2:K)*X^2),(1+X),(X),(Polynomial.C (2:K)),(1+X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_3 : GeneralCantorRun (bezoutA 7 3)
    (bezoutB 7 3) (bezoutC 7 3) (point 7) (point 3)
    (point (expectedIndex 7 3)) := by
  refine ⟨(1),(X^3+X^4),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^3),(Polynomial.C (2:K)+X),(X+X^2),(X^2),(1+Polynomial.C (2:K)*X),(1+Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(Polynomial.C (2:K)+X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_4 : GeneralCantorRun (bezoutA 7 4)
    (bezoutB 7 4) (bezoutC 7 4) (point 7) (point 4)
    (point (expectedIndex 7 4)) := by
  refine ⟨(1),(X^2+X^3),(Polynomial.C (2:K)+X^2),(1+Polynomial.C (2:K)*X+X^2),(X+X^2),(X),(1+Polynomial.C (2:K)*X),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_5 : GeneralCantorRun (bezoutA 7 5)
    (bezoutB 7 5) (bezoutC 7 5) (point 7) (point 5)
    (point (expectedIndex 7 5)) := by
  refine ⟨(1+X),(X),(Polynomial.C (2:K)),(1+Polynomial.C (2:K)*X+X^2+Polynomial.C (2:K)*X^3),(X),(1),(Polynomial.C (2:K)),(X^4),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X):K[X]).degree=(1:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X):K[X]).natDegree=1 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (1:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_6 : GeneralCantorRun (bezoutA 7 6)
    (bezoutB 7 6) (bezoutC 7 6) (point 7) (point 6)
    (point (expectedIndex 7 6)) := by
  refine ⟨(X+X^2),(1),0,(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_7 : GeneralCantorRun (bezoutA 7 7)
    (bezoutB 7 7) (bezoutC 7 7) (point 7) (point 7)
    (point (expectedIndex 7 7)) := by
  refine ⟨(1+X),(X^2),(Polynomial.C (2:K)),(1+Polynomial.C (2:K)*X+X^2),(X),(X),(1),(X^3),
    (by
      apply Polynomial.monic_of_degree_le 1
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)):K[X]).degree≤(0:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_8 : GeneralCantorRun (bezoutA 7 8)
    (bezoutB 7 8) (bezoutC 7 8) (point 7) (point 8)
    (point (expectedIndex 7 8)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^3),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(X+X^2),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_7_9 : GeneralCantorRun (bezoutA 7 9)
    (bezoutB 7 9) (bezoutC 7 9) (point 7) (point 9)
    (point (expectedIndex 7 9)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4),(Polynomial.C (2:K)+X^2),0,(X+X^2),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)+X),(X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_0 : GeneralCantorRun (bezoutA 8 0)
    (bezoutB 8 0) (bezoutC 8 0) (point 8) (point 0)
    (point (expectedIndex 8 0)) := by
  refine ⟨(1),(Polynomial.C (2:K)+X+X^2),(X),0,(Polynomial.C (2:K)+X+X^2),(1),(X),(Polynomial.C (2:K)+Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)+X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((Polynomial.C (2:K)+X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_1 : GeneralCantorRun (bezoutA 8 1)
    (bezoutB 8 1) (bezoutC 8 1) (point 8) (point 1)
    (point (expectedIndex 8 1)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+X^2+X^3),(1+Polynomial.C (2:K)*X^2),(1),(Polynomial.C (2:K)+X+X^2),(X),(1+X),(X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_2 : GeneralCantorRun (bezoutA 8 2)
    (bezoutB 8 2) (bezoutC 8 2) (point 8) (point 2)
    (point (expectedIndex 8 2)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X^2+X^3+X^4),(1+Polynomial.C (2:K)*X^2),(1),(Polynomial.C (2:K)+X+X^2),(X^2),(1+X),(1+X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(Polynomial.C (2:K)+X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_3 : GeneralCantorRun (bezoutA 8 3)
    (bezoutB 8 3) (bezoutC 8 3) (point 8) (point 3)
    (point (expectedIndex 8 3)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X^2+X^3+X^4),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^3),(1),(Polynomial.C (2:K)+X+X^2),(X^2),(Polynomial.C (2:K)+X),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(Polynomial.C (2:K)+X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_4 : GeneralCantorRun (bezoutA 8 4)
    (bezoutB 8 4) (bezoutC 8 4) (point 8) (point 4)
    (point (expectedIndex 8 4)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+X^2+X^3),(Polynomial.C (2:K)+Polynomial.C (2:K)*X+X^2),(1),(Polynomial.C (2:K)+X+X^2),(X),(Polynomial.C (2:K)+X),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_5 : GeneralCantorRun (bezoutA 8 5)
    (bezoutB 8 5) (bezoutC 8 5) (point 8) (point 5)
    (point (expectedIndex 8 5)) := by
  refine ⟨(1),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3),(1+Polynomial.C (2:K)*X^2),(1),(Polynomial.C (2:K)+X+X^2),(1+X),(X),(X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_6 : GeneralCantorRun (bezoutA 8 6)
    (bezoutB 8 6) (bezoutC 8 6) (point 8) (point 6)
    (point (expectedIndex 8 6)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4),(1+Polynomial.C (2:K)*X^2),0,(Polynomial.C (2:K)+X+X^2),(X+X^2),(1+Polynomial.C (2:K)*X),(X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_7 : GeneralCantorRun (bezoutA 8 7)
    (bezoutB 8 7) (bezoutC 8 7) (point 8) (point 7)
    (point (expectedIndex 8 7)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^3),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(Polynomial.C (2:K)+X+X^2),(X+X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_8 : GeneralCantorRun (bezoutA 8 8)
    (bezoutB 8 8) (bezoutC 8 8) (point 8) (point 8)
    (point (expectedIndex 8 8)) := by
  refine ⟨(1),(1+X+Polynomial.C (2:K)*X^2+Polynomial.C (2:K)*X^3+X^4),(Polynomial.C (2:K)+Polynomial.C (2:K)*X+X^2),(X+Polynomial.C (2:K)*X^2),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)*X),(X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1+X+Polynomial.C (2:K)*X^2+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+Polynomial.C (2:K)*X+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((1+X+Polynomial.C (2:K)*X^2+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(1+Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_8_9 : GeneralCantorRun (bezoutA 8 9)
    (bezoutB 8 9) (bezoutC 8 9) (point 8) (point 9)
    (point (expectedIndex 8 9)) := by
  refine ⟨(Polynomial.C (2:K)+X+X^2),(1),0,(X),(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_0 : GeneralCantorRun (bezoutA 9 0)
    (bezoutB 9 0) (bezoutC 9 0) (point 9) (point 0)
    (point (expectedIndex 9 0)) := by
  refine ⟨(1),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)*X),0,(Polynomial.C (2:K)+X+X^2),(1),(Polynomial.C (2:K)*X),(Polynomial.C (2:K)+Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2+X^3),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)+X+X^2):K[X]).degree=(2:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)*X):K[X]).degree≤(1:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inl ⟨(by
      have hU : ((Polynomial.C (2:K)+X+X^2):K[X]).natDegree=2 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (2:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_1 : GeneralCantorRun (bezoutA 9 1)
    (bezoutB 9 1) (bezoutC 9 1) (point 9) (point 1)
    (point (expectedIndex 9 1)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+X^2+X^3),(1+X+Polynomial.C (2:K)*X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)+X+X^2),(X),(1+Polynomial.C (2:K)*X),(Polynomial.C (2:K)+X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(1),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_2 : GeneralCantorRun (bezoutA 9 2)
    (bezoutB 9 2) (bezoutC 9 2) (point 9) (point 2)
    (point (expectedIndex 9 2)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X^2+X^3+X^4),(1+X^3),(Polynomial.C (2:K)),(Polynomial.C (2:K)+X+X^2),(X^2),(1+Polynomial.C (2:K)*X),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(1+Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_3 : GeneralCantorRun (bezoutA 9 3)
    (bezoutB 9 3) (bezoutC 9 3) (point 9) (point 3)
    (point (expectedIndex 9 3)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X^2+X^3+X^4),(Polynomial.C (2:K)+X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)+X+X^2),(X^2),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(1+X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X^2+X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(1+Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_4 : GeneralCantorRun (bezoutA 9 4)
    (bezoutB 9 4) (bezoutC 9 4) (point 9) (point 4)
    (point (expectedIndex 9 4)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+X^2+X^3),(Polynomial.C (2:K)+X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)+X+X^2),(X),(Polynomial.C (2:K)+Polynomial.C (2:K)*X),(X+X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_5 : GeneralCantorRun (bezoutA 9 5)
    (bezoutB 9 5) (bezoutC 9 5) (point 9) (point 5)
    (point (expectedIndex 9 5)) := by
  refine ⟨(1),(Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3),(Polynomial.C (2:K)+X^2),(Polynomial.C (2:K)),(Polynomial.C (2:K)+X+X^2),(1+X),(Polynomial.C (2:K)*X),(X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 3
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3):K[X]).degree=(3:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)+Polynomial.C (2:K)*X^2+X^3):K[X]).natDegree=3 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(3:ℕ))),(1:K),(Polynomial.C (2:K)),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_6 : GeneralCantorRun (bezoutA 9 6)
    (bezoutB 9 6) (bezoutC 9 6) (point 9) (point 6)
    (point (expectedIndex 9 6)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4),(1+X^3),(1+X),(Polynomial.C (2:K)+X+X^2),(X+X^2),(1),(Polynomial.C (2:K)*X^2),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X^3):K[X]).degree≤(3:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(2:K),(Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_7 : GeneralCantorRun (bezoutA 9 7)
    (bezoutB 9 7) (bezoutC 9 7) (point 9) (point 7)
    (point (expectedIndex 9 7)) := by
  refine ⟨(1),(Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4),(Polynomial.C (2:K)+X^2),0,(Polynomial.C (2:K)+X+X^2),(X+X^2),(Polynomial.C (2:K)+X),(X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((Polynomial.C (2:K)+X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((Polynomial.C (2:K)*X+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(Polynomial.C (2:K)*X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_8 : GeneralCantorRun (bezoutA 9 8)
    (bezoutB 9 8) (bezoutC 9 8) (point 9) (point 8)
    (point (expectedIndex 9 8)) := by
  refine ⟨(Polynomial.C (2:K)+X+X^2),(1),0,(Polynomial.C (2:K)*X),(1),(1),0,(1+X^5),
    (by
      apply Polynomial.monic_of_degree_le 2
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1):K[X]).degree=(0:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      simp),?_,Or.inl ⟨(by
      have hU : ((1):K[X]).natDegree=0 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : (0:ℕ)≤2)),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem cert_9_9 : GeneralCantorRun (bezoutA 9 9)
    (bezoutB 9 9) (bezoutC 9 9) (point 9) (point 9)
    (point (expectedIndex 9 9)) := by
  refine ⟨(1),(1+X+Polynomial.C (2:K)*X^2+Polynomial.C (2:K)*X^3+X^4),(1+X+Polynomial.C (2:K)*X^2),(Polynomial.C (2:K)*X+X^2),(Polynomial.C (2:K)+X+X^2),(Polynomial.C (2:K)+X+X^2),(X),(X),
    (by
      apply Polynomial.monic_of_degree_le 0
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),(by
      apply Polynomial.monic_of_degree_le 4
      · compute_degree!
      · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]),?_,?_,?_,?_,?_,?_,(by
      have hU : ((1+X+Polynomial.C (2:K)*X^2+Polynomial.C (2:K)*X^3+X^4):K[X]).degree=(4:WithBot ℕ) := by
        apply Polynomial.degree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      rw [hU]
      apply lt_of_le_of_lt (show ((1+X+Polynomial.C (2:K)*X^2):K[X]).degree≤(2:WithBot ℕ) from by compute_degree!)
      norm_num),?_,Or.inr ⟨(by
      have hU : ((1+X+Polynomial.C (2:K)*X^2+Polynomial.C (2:K)*X^3+X^4):K[X]).natDegree=4 := by
        apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
        · compute_degree!
        · norm_num [Polynomial.coeff_one, Polynomial.coeff_X]
      simpa only [hU] using (by decide : 2<(4:ℕ))),(1:K),(Polynomial.C (2:K)+X),(by decide),?_,?_⟩⟩
  all_goals norm_num [bezoutA,bezoutB,bezoutC,expectedIndex,repPerm,
    point,candidatePair,representative,uPolynomial,vPolynomial,curvePolynomial]
  all_goals stoll_full_poly

theorem all_pairs (i j : Fin 10) :
    GeneralCantorRun (bezoutA i j) (bezoutB i j) (bezoutC i j)
      (point i) (point j) (point (expectedIndex i j)) := by
  fin_cases i <;> fin_cases j
  · exact cert_0_0
  · exact cert_0_1
  · exact cert_0_2
  · exact cert_0_3
  · exact cert_0_4
  · exact cert_0_5
  · exact cert_0_6
  · exact cert_0_7
  · exact cert_0_8
  · exact cert_0_9
  · exact cert_1_0
  · exact cert_1_1
  · exact cert_1_2
  · exact cert_1_3
  · exact cert_1_4
  · exact cert_1_5
  · exact cert_1_6
  · exact cert_1_7
  · exact cert_1_8
  · exact cert_1_9
  · exact cert_2_0
  · exact cert_2_1
  · exact cert_2_2
  · exact cert_2_3
  · exact cert_2_4
  · exact cert_2_5
  · exact cert_2_6
  · exact cert_2_7
  · exact cert_2_8
  · exact cert_2_9
  · exact cert_3_0
  · exact cert_3_1
  · exact cert_3_2
  · exact cert_3_3
  · exact cert_3_4
  · exact cert_3_5
  · exact cert_3_6
  · exact cert_3_7
  · exact cert_3_8
  · exact cert_3_9
  · exact cert_4_0
  · exact cert_4_1
  · exact cert_4_2
  · exact cert_4_3
  · exact cert_4_4
  · exact cert_4_5
  · exact cert_4_6
  · exact cert_4_7
  · exact cert_4_8
  · exact cert_4_9
  · exact cert_5_0
  · exact cert_5_1
  · exact cert_5_2
  · exact cert_5_3
  · exact cert_5_4
  · exact cert_5_5
  · exact cert_5_6
  · exact cert_5_7
  · exact cert_5_8
  · exact cert_5_9
  · exact cert_6_0
  · exact cert_6_1
  · exact cert_6_2
  · exact cert_6_3
  · exact cert_6_4
  · exact cert_6_5
  · exact cert_6_6
  · exact cert_6_7
  · exact cert_6_8
  · exact cert_6_9
  · exact cert_7_0
  · exact cert_7_1
  · exact cert_7_2
  · exact cert_7_3
  · exact cert_7_4
  · exact cert_7_5
  · exact cert_7_6
  · exact cert_7_7
  · exact cert_7_8
  · exact cert_7_9
  · exact cert_8_0
  · exact cert_8_1
  · exact cert_8_2
  · exact cert_8_3
  · exact cert_8_4
  · exact cert_8_5
  · exact cert_8_6
  · exact cert_8_7
  · exact cert_8_8
  · exact cert_8_9
  · exact cert_9_0
  · exact cert_9_1
  · exact cert_9_2
  · exact cert_9_3
  · exact cert_9_4
  · exact cert_9_5
  · exact cert_9_6
  · exact cert_9_7
  · exact cert_9_8
  · exact cert_9_9

#print axioms all_pairs
end
end MumfordStollFullCertificatesAudit
