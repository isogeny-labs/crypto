import RecodedTop
noncomputable section
open MvPolynomial Finsupp
namespace RecodedTop
variable {K : Type*} [Field K]
local notation "P" => MvPolynomial (Fin 7) K
set_option maxHeartbeats 300000
set_option maxRecDepth 10000

def e1 : Fin 7 →₀ ℕ := single 0 1+single 2 1+single 3 1+single 4 1+single 5 1
def e2 : Fin 7 →₀ ℕ := single 0 2+single 3 1+single 4 1+single 5 1
def e3 : Fin 7 →₀ ℕ := single 0 3+single 2 1+single 5 1
def e4 : Fin 7 →₀ ℕ := single 0 4+single 5 1
def dual (a : K) (p : P) : K :=
  coeff e1 p+a*coeff e2 p+coeff e3 p+2*a*coeff e4 p

theorem dual_qx (a : K) (p : P) :
    dual a (p*(X 0^2-C a*X 2*X 0-X 3*X 4)) = 0 := by
  have h01 : ¬ single 0 2 ≤ e1 := by
    intro hh
    have h := hh (0 : Fin 7)
    simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] at h
    norm_num at h
  have h02 : single 0 2 ≤ e2 := by
    intro i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have h03 : single 0 2 ≤ e3 := by
    intro i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have h04 : single 0 2 ≤ e4 := by
    intro i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have h11 : single 2 1+single 0 1 ≤ e1 := by
    intro i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have h12 : ¬ single 2 1+single 0 1 ≤ e2 := by
    intro hh
    have h := hh (2 : Fin 7)
    simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] at h
    norm_num at h
  have h13 : single 2 1+single 0 1 ≤ e3 := by
    intro i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have h14 : ¬ single 2 1+single 0 1 ≤ e4 := by
    intro hh
    have h := hh (2 : Fin 7)
    simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] at h
    norm_num at h
  have h21 : single 3 1+single 4 1 ≤ e1 := by
    intro i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have h22 : single 3 1+single 4 1 ≤ e2 := by
    intro i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have h23 : ¬ single 3 1+single 4 1 ≤ e3 := by
    intro hh
    have h := hh (3 : Fin 7)
    simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] at h
    norm_num at h
  have h24 : ¬ single 3 1+single 4 1 ≤ e4 := by
    intro hh
    have h := hh (3 : Fin 7)
    simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] at h
    norm_num at h
  have s02 : e2-single 0 2 = single 3 1+single 4 1+single 5 1 := by
    ext i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have s03 : e3-single 0 2 = single 0 1+single 2 1+single 5 1 := by
    ext i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have s04 : e4-single 0 2 = single 0 2+single 5 1 := by
    ext i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have s11 : e1-(single 2 1+single 0 1) = single 3 1+single 4 1+single 5 1 := by
    ext i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have s13 : e3-(single 2 1+single 0 1) = single 0 2+single 5 1 := by
    ext i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have s21 : e1-(single 3 1+single 4 1) = single 0 1+single 2 1+single 5 1 := by
    ext i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  have s22 : e2-(single 3 1+single 4 1) = single 0 2+single 5 1 := by
    ext i
    fin_cases i <;> simp only [e1, e2, e3, e4, Finsupp.single_apply, Finsupp.add_apply, Finsupp.tsub_apply, Fin.ext_iff, Fin.coe_ofNat_eq_mod] <;> norm_num
  simp only [X_pow_eq_monomial]
  simp only [X, C_apply, monomial_mul, zero_add]
  simp only [dual, mul_sub, coeff_sub, coeff_mul_monomial',
    h01, h02, h03, h04, h11, h12, h13, h14, h21, h22, h23, h24, s02, s03, s04, s11, s13, s21, s22, ↓reduceIte, mul_one]
  ring


theorem dual_add (a : K) (p q : P) : dual a (p+q) = dual a p+dual a q := by
  simp only [dual, coeff_add]
  ring

theorem dual_sub (a : K) (p q : P) : dual a (p-q) = dual a p-dual a q := by
  simp only [dual, coeff_sub]
  ring

theorem dual_C_mul (a c : K) (p : P) : dual a (C c*p) = c*dual a p := by
  simp only [dual, coeff_C_mul]
  ring

theorem dual_forbidden (a c : K) (p : P) (d : Fin 7 →₀ ℕ) (i : Fin 7)
    (h : e1 i < d i ∧ e2 i < d i ∧ e3 i < d i ∧ e4 i < d i) :
    dual a (p*monomial d c) = 0 := by
  have h1 : ¬d ≤ e1 := fun hh => (not_lt_of_ge (hh i)) h.1
  have h2 : ¬d ≤ e2 := fun hh => (not_lt_of_ge (hh i)) h.2.1
  have h3 : ¬d ≤ e3 := fun hh => (not_lt_of_ge (hh i)) h.2.2.1
  have h4 : ¬d ≤ e4 := fun hh => (not_lt_of_ge (hh i)) h.2.2.2
  simp only [dual, coeff_mul_monomial', h1,h2,h3,h4, ↓reduceIte,
    mul_zero, add_zero]

theorem dual_generator (a b : K) (p q : P)
    (hq : q ∈ ({X 2^2, X 0*X 1, X 0^2-C a*X 2*X 0-X 3*X 4,
      X 1^2-C b*X 2*X 1-X 5*X 6, X 3^2, X 4^2, X 5^2, X 6^2} : Set P)) :
    dual a (p*q) = 0 := by
  have sq (j : Fin 7) (h : e1 j < 2 ∧ e2 j < 2 ∧ e3 j < 2 ∧ e4 j < 2) :
      dual a (p*X j^2) = 0 := by
    rw [X_pow_eq_monomial]
    apply dual_forbidden a 1 p _ j
    simpa only [Finsupp.single_eq_same] using h
  have prod (j k i : Fin 7)
      (h : e1 i < (single j 1+single k 1 : Fin 7 →₀ ℕ) i ∧
        e2 i < (single j 1+single k 1 : Fin 7 →₀ ℕ) i ∧
        e3 i < (single j 1+single k 1 : Fin 7 →₀ ℕ) i ∧
        e4 i < (single j 1+single k 1 : Fin 7 →₀ ℕ) i) : dual a (p*(X j*X k)) = 0 := by
    simp only [X, monomial_mul, one_mul]
    exact dual_forbidden a 1 p _ i h
  have hz : dual a (p*X 1^2) = 0 := by
    apply sq 1
    simp only [e1,e2,e3,e4,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod]
    norm_num
  have hmz : dual a (p*(X 2*X 1)) = 0 := by
    apply prod 2 1 1
    simp only [e1,e2,e3,e4,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod]
    norm_num
  have hst : dual a (p*(X 5*X 6)) = 0 := by
    apply prod 5 6 6
    simp only [e1,e2,e3,e4,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod]
    norm_num
  rcases hq with rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl
  · apply sq 2
    simp only [e1,e2,e3,e4,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod]
    norm_num
  · apply prod 0 1 1
    simp only [e1,e2,e3,e4,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod]
    norm_num
  · exact dual_qx a p
  · have hC : p*(C b*X 2*X 1) = C b*(p*(X 2*X 1)) := by ring
    rw [mul_sub, mul_sub, dual_sub, dual_sub, hC, dual_C_mul, hz, hmz, hst]
    ring
  all_goals
    apply sq
    simp only [e1,e2,e3,e4,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod]
    norm_num

theorem dual_ideal (a b : K) {q : P} (hq : q ∈ ideal a b) : dual a q = 0 := by
  have hh : ∀ p : P, dual a (p*q) = 0 := by
    induction hq using Submodule.span_induction with
    | mem q hq => exact fun p => dual_generator a b p q hq
    | zero => intro p; simp only [mul_zero,dual,coeff_zero,mul_zero,add_zero]
    | add q r hq hr ihq ihr =>
      intro p
      rw [mul_add,dual_add,ihq,ihr,add_zero]
    | smul r q hq ih =>
      intro p
      simpa only [smul_eq_mul, mul_assoc] using ih (p*r)
  simpa only [one_mul] using hh 1

def witness : P := X 0^3*X 2*X 5

theorem witness_homogeneous : (witness : P).IsHomogeneous 5 :=
  ((isHomogeneous_X_pow 0 3).mul (isHomogeneous_X K 2)).mul (isHomogeneous_X K 5)

theorem witness_not_mem (a b : K) : witness ∉ ideal a b := by
  intro h
  have hh := dual_ideal a b h
  have hw : (witness : P) = monomial e3 1 := by
    unfold witness
    rw [X_pow_eq_monomial]
    simp only [X, monomial_mul, one_mul, e3]
  have hne1 : e1 ≠ e3 := by
    intro h
    have hh := congrArg (fun e : Fin 7 →₀ ℕ => e 0) h
    simp only [e1,e3,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod] at hh
    norm_num at hh
  have hne2 : e2 ≠ e3 := by
    intro h
    have hh := congrArg (fun e : Fin 7 →₀ ℕ => e 0) h
    simp only [e2,e3,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod] at hh
    norm_num at hh
  have hne4 : e4 ≠ e3 := by
    intro h
    have hh := congrArg (fun e : Fin 7 →₀ ℕ => e 0) h
    simp only [e4,e3,Finsupp.single_apply,Finsupp.add_apply,
      Fin.ext_iff, Fin.coe_ofNat_eq_mod] at hh
    norm_num at hh
  rw [hw] at hh
  simp only [dual, coeff_monomial, Ne.symm hne1,Ne.symm hne2,Ne.symm hne4, ↓reduceIte, mul_zero,
    zero_add, add_zero] at hh
  exact one_ne_zero hh

theorem first_vanishing (a b : K) : SecantIdeal.FirstVanishing (ideal a b) 6 := by
  refine ⟨vanishes a b, ?_⟩
  intro d hd hv
  have hh := PencilTop.vanishes_mono (ideal a b) hv (show d ≤ 5 by omega)
  exact witness_not_mem a b (hh _ witness_homogeneous)

end RecodedTop
