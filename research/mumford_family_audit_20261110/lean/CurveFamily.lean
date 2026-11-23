import ReducedPairs
import Mathlib.Algebra.Ring.GeomSum

namespace MumfordCurveFamilyAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit

def genus (t : ℕ) := 12*t+2
def exponent (t : ℕ) := 24*t+5
def curvePolynomial (t : ℕ) : K[X] := X^(exponent t)+1

theorem exponent_genus (t : ℕ) : exponent t=2*genus t+1 := by
  unfold exponent genus
  omega

theorem exponent_positive (t : ℕ) : 0<exponent t := by unfold exponent;omega

theorem curve_monic (t : ℕ) : (curvePolynomial t).Monic := by
  simpa only [curvePolynomial,map_one] using
    (Polynomial.monic_X_pow_add_C (R:=K) (a:=1) (n:=exponent t)
      (Nat.ne_of_gt (exponent_positive t)))

theorem curve_degree (t : ℕ) : (curvePolynomial t).natDegree=exponent t := by
  have h : (1:K[X]).natDegree<((X:K[X])^(exponent t)).natDegree := by
    simpa only [Polynomial.natDegree_one,Polynomial.natDegree_X_pow] using exponent_positive t
  exact (Polynomial.natDegree_add_eq_left_of_natDegree_lt h).trans (by simp)

theorem exponent_cast (t : ℕ) : (exponent t:K)=2 := by
  have h24 : (24:K)=0 := by decide +kernel
  have h5 : (5:K)=2 := by decide +kernel
  simp only [exponent,Nat.cast_add,Nat.cast_mul,Nat.cast_ofNat,h24,h5,zero_mul,zero_add]

theorem curve_derivative (t : ℕ) :
    derivative (curvePolynomial t)=2*X^(exponent t-1) := by
  simp only [curvePolynomial,Polynomial.derivative_add,Polynomial.derivative_X_pow,
    Polynomial.derivative_one,add_zero]
  rw [exponent_cast,Polynomial.C_ofNat]

theorem polynomial_three_zero : (3:K[X])=0 := by
  simpa only [Polynomial.C_ofNat,map_zero] using
    congrArg (C : K→K[X]) (by decide +kernel : (3:K)=0)

theorem curve_separable (t : ℕ) : (curvePolynomial t).Separable := by
  apply (Polynomial.separable_def' _).mpr
  refine ⟨1,X,?_⟩
  rw [curve_derivative]
  have he : (X:K[X])*X^(exponent t-1)=X^(exponent t) := by
    rw [←pow_succ']
    congr 1
  calc
    1*curvePolynomial t+X*(2*X^(exponent t-1))=
        (X^(exponent t)+1)+2*(X*X^(exponent t-1)) := by
      unfold curvePolynomial
      ring
    _=3*X^(exponent t)+1 := by rw [he];ring
    _=1 := by rw [polynomial_three_zero];simp

def badU : K[X] := X^2+X+C 2

theorem badU_monic : badU.Monic := by
  apply Polynomial.monic_of_degree_le 2
  · unfold badU
    compute_degree!
  · simp [badU,Polynomial.coeff_X]

theorem badU_natDegree : badU.natDegree=2 := by
  unfold badU
  compute_degree!

theorem badU_nonsplit : ¬badU.Splits := by
  have hd : badU.degree≠0 := by
    rw [Polynomial.degree_eq_natDegree badU_monic.ne_zero,badU_natDegree]
    norm_num
  have hn : ∀ a : K,a^2+a+2≠0 := by decide +kernel
  intro hs
  obtain ⟨a,ha⟩ := hs.exists_eval_eq_zero hd
  exact hn a (by simpa [badU] using ha)

theorem bad_period : badU∣(X:K[X])^8-1 := by
  have h4 : badU∣(X:K[X])^4+1 := by
    refine ⟨X^2-X+2,?_⟩
    simp only [badU,Polynomial.C_ofNat]
    linear_combination -(X^2+1)*polynomial_three_zero
  apply dvd_trans h4
  exact ⟨X^4-1,by ring⟩

theorem bad_base : badU∣((X:K[X])^5+1-X^2) := by
  refine ⟨X^3-X^2-X-1,?_⟩
  simp only [badU,Polynomial.C_ofNat]
  linear_combination (X^2+X+1)*polynomial_three_zero

theorem bad_divisibility (t : ℕ) : badU∣curvePolynomial t-X^2 := by
  have hp : badU∣((X:K[X])^8)^(3*t)-1 := by
    simpa only [one_pow] using
      dvd_trans bad_period (sub_dvd_pow_sub_pow ((X:K[X])^8) 1 (3*t))
  obtain ⟨q,hq⟩ := hp
  obtain ⟨r,hr⟩ := bad_base
  refine ⟨X^5*q+r,?_⟩
  have he : (X:K[X])^(exponent t)=X^5*(X^8)^(3*t) := by
    rw [show exponent t=5+8*(3*t) by unfold exponent;omega,pow_add,pow_mul]
  unfold curvePolynomial
  rw [he]
  calc
    X^5*(X^8)^(3*t)+1-X^2=X^5*((X^8)^(3*t)-1)+(X^5+1-X^2) := by ring
    _=badU*(X^5*q+r) := by rw [hq,hr];ring

def badPair (t : ℕ) : ReducedPair (genus t) (curvePolynomial t) :=
  ⟨badU,X,badU_monic,by rw [badU_natDegree];unfold genus;omega,
    by rw [Polynomial.degree_eq_natDegree badU_monic.ne_zero,badU_natDegree];norm_num,
    bad_divisibility t⟩

theorem badPair_nonsmooth (t : ℕ) : ¬Smooth (badPair t) := badU_nonsplit

def powerPair (t d : ℕ) (hd : 0<d) (hb : d≤genus t) (s : K) (hs : s^2=1) :
    ReducedPair (genus t) (curvePolynomial t) where
  u := X^d
  v := C s
  monic_u := Polynomial.monic_X_pow d
  degree_u := by simpa using hb
  degree_v := by
    apply lt_of_le_of_lt Polynomial.degree_C_le
    rw [Polynomial.degree_X_pow]
    exact_mod_cast hd
  divides := by
    rw [←map_pow,hs,map_one]
    change (X:K[X])^d∣(X^(exponent t)+1)-1
    rw [add_sub_cancel_right]
    refine ⟨X^(exponent t-d),?_⟩
    rw [←pow_add]
    congr 1
    have hn:=exponent_genus t
    omega

theorem powerPair_smooth (t d : ℕ) (hd : 0<d) (hb : d≤genus t) (s : K) (hs : s^2=1) :
    Smooth (powerPair t d hd hb s hs) := Polynomial.Splits.X.pow d

#print axioms curve_separable
#print axioms badPair_nonsmooth
#print axioms powerPair_smooth
end
end MumfordCurveFamilyAudit
