import DegreeWords
import F4ExpectedOutputs

namespace MsolveDegreeWordAudit
noncomputable section
open MvPolynomial SolvingDegreeBound

def highX (n : ℕ) : Fin 2 →₀ ℕ := Finsupp.single 0 (n+2)
def highXY (n : ℕ) : Fin 2 →₀ ℕ := Finsupp.single 0 (n+1) + Finsupp.single 1 1
def exponentX : Fin 2 →₀ ℕ := Finsupp.single 0 1
def exponentY : Fin 2 →₀ ℕ := Finsupp.single 1 1

theorem highX_ne_highXY (n : ℕ) : highX n ≠ highXY n := by
  intro h
  have hh := congrArg (fun e => e (1 : Fin 2)) h
  simp [highX, highXY] at hh

theorem highX_ne_zero (n : ℕ) : highX n ≠ 0 := by
  intro h
  have hh := congrArg (fun e => e (0 : Fin 2)) h
  simp [highX] at hh

theorem highXY_ne_zero (n : ℕ) : highXY n ≠ 0 := by
  intro h
  have hh := congrArg (fun e => e (1 : Fin 2)) h
  simp [highXY] at hh

theorem highX_ne_exponentY (n : ℕ) : highX n ≠ exponentY := by
  intro h
  have hh := congrArg (fun e => e (0 : Fin 2)) h
  simp [highX, exponentY] at hh

theorem highXY_ne_exponentY (n : ℕ) : highXY n ≠ exponentY := by
  intro h
  have hh := congrArg (fun e => e (0 : Fin 2)) h
  simp [highXY, exponentY] at hh

theorem exponentY_ne_zero : exponentY ≠ 0 := by
  intro h
  have hh := congrArg (fun e => e (1 : Fin 2)) h
  simp [exponentY] at hh

theorem highX_degree (n : ℕ) : (highX n).sum (fun _ e => e) = n+2 := by
  exact Finsupp.sum_single_index rfl

theorem highXY_degree (n : ℕ) : (highXY n).sum (fun _ e => e) = n+2 := by
  simp [highXY, Finsupp.sum_add_index]

theorem row_exponents_equal (n : ℕ) : exponentY + highX n = exponentX + highXY n := by
  ext i
  fin_cases i
  · simp [exponentY, exponentX, highX, highXY]
    omega
  · simp [exponentY, exponentX, highX, highXY]

variable {K : Type*} [Field K]
abbrev Poly (A : Type*) [Field A] := MvPolynomial (Fin 2) A
def firstPoly (n : ℕ) : Poly K := monomial (highX n) 1 + 1
def secondPoly (n : ℕ) : Poly K := monomial (highXY n) 1
def linearY : Poly K := monomial exponentY 1
def inputs (n : ℕ) : Set (Poly K) := {firstPoly n, secondPoly n}

theorem combination_highX (n : ℕ) (a b : K) :
    ((a • firstPoly n + b • secondPoly n) : Poly K).coeff (highX n) = a := by
  simp [firstPoly, secondPoly, MvPolynomial.coeff_one,
    Ne.symm (highX_ne_zero n), Ne.symm (highX_ne_highXY n)]

theorem combination_highXY (n : ℕ) (a b : K) :
    ((a • firstPoly n + b • secondPoly n) : Poly K).coeff (highXY n) = b := by
  simp [firstPoly, secondPoly, MvPolynomial.coeff_one,
    Ne.symm (highXY_ne_zero n), highX_ne_highXY]

theorem combination_linearY (n : ℕ) (a b : K) :
    ((a • firstPoly n + b • secondPoly n) : Poly K).coeff exponentY = 0 := by
  simp [firstPoly, secondPoly, MvPolynomial.coeff_one, highX_ne_exponentY,
    highXY_ne_exponentY, Ne.symm exponentY_ne_zero]

theorem span_representation (n : ℕ) {p : Poly K}
    (hp : p ∈ Submodule.span K (inputs n)) :
    ∃ a b : K, p = a • firstPoly n + b • secondPoly n := by
  induction hp using Submodule.span_induction with
  | mem p hp =>
    simp only [inputs, Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl
    · exact ⟨1, 0, by simp⟩
    · exact ⟨0, 1, by simp⟩
  | zero => exact ⟨0, 0, by simp⟩
  | add p q _ _ hp hq =>
    obtain ⟨a, b, hp⟩ := hp
    obtain ⟨c, d, hq⟩ := hq
    refine ⟨a+c, b+d, ?_⟩
    rw [hp, hq]
    module
  | smul c p _ hp =>
    obtain ⟨a, b, hp⟩ := hp
    refine ⟨c*a, c*b, ?_⟩
    rw [hp]
    simp only [smul_add, mul_smul]

theorem low_span_zero (n : ℕ) {p : Poly K}
    (hp : p ∈ Submodule.span K (inputs n)) (hd : p.totalDegree+1 ≤ n+2) : p=0 := by
  obtain ⟨a, b, hab⟩ := span_representation n hp
  have hX : p.coeff (highX n) = 0 := by
    apply coeff_eq_zero_of_totalDegree_lt
    change p.totalDegree < (highX n).sum (fun _ e => e)
    rw [highX_degree]
    omega
  have hXY : p.coeff (highXY n) = 0 := by
    apply coeff_eq_zero_of_totalDegree_lt
    change p.totalDegree < (highXY n).sum (fun _ e => e)
    rw [highXY_degree]
    omega
  rw [hab, combination_highX] at hX
  rw [hab, combination_highXY] at hXY
  rw [hab, hX, hXY]
  simp

theorem input_span_closed (n : ℕ) : DClosed (n+2) (Submodule.span K (inputs n)) := by
  intro p hp hd i
  rw [low_span_zero n hp hd, mul_zero]
  exact Submodule.zero_mem _

theorem linearY_not_span (n : ℕ) : (linearY : Poly K) ∉ Submodule.span K (inputs n) := by
  intro h
  obtain ⟨a, b, hab⟩ := span_representation n h
  have hh := congrArg (fun p : Poly K => p.coeff exponentY) hab
  have hz : (1 : K) = 0 := by
    simpa only [linearY, coeff_monomial, ite_true, combination_linearY] using hh
  exact one_ne_zero hz

theorem first_degree (n : ℕ) : (firstPoly n : Poly K).totalDegree = n+2 := by
  have h : (1 : Poly K).totalDegree < (monomial (highX n) (1 : K)).totalDegree := by
    simp only [totalDegree_one, totalDegree_monomial _ one_ne_zero, highX_degree]
    omega
  rw [firstPoly, totalDegree_add_eq_left_of_totalDegree_lt h,
    totalDegree_monomial _ one_ne_zero, highX_degree]

theorem second_degree (n : ℕ) : (secondPoly n : Poly K).totalDegree = n+2 := by
  rw [secondPoly, totalDegree_monomial _ one_ne_zero, highXY_degree]

theorem reduction_identity (n : ℕ) :
    monomial exponentY (1 : K) * firstPoly n -
      monomial exponentX (1 : K) * secondPoly n = linearY := by
  simp only [firstPoly, secondPoly, mul_add, mul_one, monomial_mul_monomial,
    row_exponents_equal, linearY]
  abel

#print axioms low_span_zero
#print axioms input_span_closed
#print axioms linearY_not_span
#print axioms reduction_identity
end
end MsolveDegreeWordAudit
