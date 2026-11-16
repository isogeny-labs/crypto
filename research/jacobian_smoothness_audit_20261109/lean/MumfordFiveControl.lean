import Mathlib.Algebra.Polynomial.Div
import Mathlib.Algebra.Polynomial.Splits
import Mathlib.FieldTheory.Separable
import Mathlib.Tactic
import Mathlib.Tactic.DeriveFintype

/-! Adapted from the frozen MumfordSmoothness.lean coefficient-shape proof.
All hypotheses and actual polynomial divisibility are retained at characteristic five.

Actual bounded Mumford polynomial pairs for y^2=x^5-x over F5.
The geometric identification with Pic^0 is a separate obligation; no
arbitrary finite group or selected list is assumed to be that Jacobian. -/
namespace MumfordFiveControlAudit
noncomputable section
open Polynomial
set_option maxRecDepth 4096
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev K := ZMod 5

def curvePolynomial : K[X] := X^5-X

/-- Every monic u of degree at most two and every v of degree less than u
has this canonical coefficient shape. Completeness is proved separately. -/
inductive Candidate
  | identity
  | linear (a b : K)
  | quadratic (a b c d : K)
  deriving DecidableEq, Fintype

def uPolynomial : Candidate→K[X]
  | .identity => 1
  | .linear a _ => X-C a
  | .quadratic a b _ _ => X^2+C a*X+C b

def vPolynomial : Candidate→K[X]
  | .identity => 0
  | .linear _ b => C b
  | .quadratic _ _ c d => C c*X+C d

/-- Monic division tests the actual polynomial divisibility condition. -/
def Valid (D : Candidate) : Prop :=
  (vPolynomial D^2-curvePolynomial) %ₘ uPolynomial D=0

instance (D : Candidate) : Decidable (Valid D) := inferInstanceAs
  (Decidable ((vPolynomial D^2-curvePolynomial) %ₘ uPolynomial D=0))

theorem curve_factorization : curvePolynomial=
    X*(X-1)*(X+1)*(X^2+1) := by unfold curvePolynomial; ring

theorem candidate_cardinality : Fintype.card Candidate=651 := by decide

def quadraticU (a b : K) : K[X] := X^2+C a*X+C b
def quadraticV (c d : K) : K[X] := C c*X+C d
def residueX (a b c d : K) : K := -a*c^2+2*c*d-a^4+3*a^2*b-b^2+1
def residueC (a b c d : K) : K := d^2-b*c^2-a^3*b+2*a*b^2

theorem quadraticU_monic (a b : K) : (quadraticU a b).Monic := by
  apply Polynomial.monic_of_degree_le 2
  · unfold quadraticU; compute_degree!
  · simp [quadraticU]

theorem quadraticU_degree (a b : K) : (quadraticU a b).degree=2 := by
  unfold quadraticU
  compute_degree!

theorem candidates_monic : ∀ D : Candidate, (uPolynomial D).Monic := by
  intro D
  cases D with
  | identity => exact Polynomial.monic_one
  | linear a b => exact Polynomial.monic_X_sub_C a
  | quadratic a b c d => exact quadraticU_monic a b

theorem candidates_degree_bound :
    ∀ D : Candidate, (uPolynomial D).natDegree≤2 := by
  intro D
  cases D <;> simp only [uPolynomial] <;> compute_degree!

theorem candidates_v_degree :
    ∀ D : Candidate, (vPolynomial D).degree<(uPolynomial D).degree := by
  intro D
  cases D with
  | identity => simp [uPolynomial,vPolynomial]
  | linear a b => simp only [uPolynomial,vPolynomial,Polynomial.degree_X_sub_C]; compute_degree!
  | quadratic a b c d =>
    change (quadraticV c d).degree<(quadraticU a b).degree
    rw [quadraticU_degree]
    unfold quadraticV
    compute_degree!

theorem quadratic_remainder (a b c d : K) :
    (quadraticV c d^2-curvePolynomial) %ₘ quadraticU a b=
      C (residueX a b c d)*X+C (residueC a b c d) := by
  let q : K[X] := C (c^2)-(X^3-C a*X^2+C (a^2-b)*X+C (-a^3+2*a*b))
  let r : K[X] := C (residueX a b c d)*X+C (residueC a b c d)
  have he : r+quadraticU a b*q=quadraticV c d^2-curvePolynomial := by
    simp only [q,r,quadraticU,quadraticV,curvePolynomial,residueX,residueC,
      map_add,map_sub,map_neg,map_mul,map_pow,map_one,Polynomial.C_ofNat]
    ring
  have hd : r.degree<(quadraticU a b).degree := by
    rw [quadraticU_degree]
    dsimp [r]
    compute_degree!
  exact (Polynomial.div_modByMonic_unique q r (quadraticU_monic a b) ⟨he,hd⟩).2

/-- Computable scalar test, proved equivalent to actual division below. -/
def CoefficientValid : Candidate→Prop
  | .identity => True
  | .linear a b => b^2=a^5-a
  | .quadratic a b c d => residueX a b c d=0 ∧ residueC a b c d=0

instance (D : Candidate) : Decidable (CoefficientValid D) := by
  cases D <;> unfold CoefficientValid <;> infer_instance

theorem valid_iff_coefficientValid (D : Candidate) : Valid D↔CoefficientValid D := by
  cases D with
  | identity => simp [Valid,CoefficientValid,uPolynomial,vPolynomial]
  | linear a b =>
    simp [Valid,CoefficientValid,uPolynomial,vPolynomial,
      Polynomial.modByMonic_X_sub_C_eq_C_eval,curvePolynomial]
  | quadratic a b c d =>
    change (quadraticV c d^2-curvePolynomial) %ₘ quadraticU a b=0 ↔ _
    rw [quadratic_remainder]
    change C (residueX a b c d)*X+C (residueC a b c d)=0 ↔
      residueX a b c d=0 ∧ residueC a b c d=0
    constructor
    · intro h
      exact ⟨by simpa using congrArg (fun p : K[X]=>p.coeff 1) h,
        by simpa using congrArg (fun p : K[X]=>p.coeff 0) h⟩
    · rintro ⟨hx,hc⟩
      simp [hx,hc]

theorem coefficient_valid_cardinality :
    Fintype.card {D : Candidate // CoefficientValid D}=16 := by decide

theorem valid_cardinality : Fintype.card {D : Candidate // Valid D}=16 := by
  rw [Fintype.card_congr (Equiv.subtypeEquivRight valid_iff_coefficientValid)]
  exact coefficient_valid_cardinality

theorem expansion_one (p : K[X]) (h : p.natDegree≤1) :
    p=C (p.coeff 1)*X+C (p.coeff 0) := by
  calc
    p=∑ i∈Finset.range 2, C (p.coeff i)*X^i :=
      p.as_sum_range_C_mul_X_pow' (by omega)
    _=_ := by simp [Finset.sum_range_succ]; ring

theorem expansion_two (p : K[X]) (h : p.natDegree≤2) :
    p=C (p.coeff 2)*X^2+C (p.coeff 1)*X+C (p.coeff 0) := by
  calc
    p=∑ i∈Finset.range 3, C (p.coeff i)*X^i :=
      p.as_sum_range_C_mul_X_pow' (by omega)
    _=_ := by simp [Finset.sum_range_succ]; ring

theorem natDegree_lt_of_positive_bound (p : K[X]) (n : ℕ) (hn : 0<n)
    (h : p.degree<(n:WithBot ℕ)) : p.natDegree<n := by
  by_cases hz : p=0
  · simpa [hz] using hn
  · exact (Polynomial.natDegree_lt_iff_degree_lt hz).mpr h

/-- Exhaustiveness for arbitrary actual polynomials, rather than a
cardinality assertion about a chosen list of representatives. -/
theorem canonical_shape (u v : K[X]) (hm : u.Monic) (hu : u.natDegree≤2)
    (hv : v.degree<u.degree) :
    ∃ D : Candidate, uPolynomial D=u ∧ vPolynomial D=v := by
  have htop : u.coeff u.natDegree=1 := (Polynomial.coeff_natDegree (p:=u)).trans hm
  have hcases : u.natDegree=0 ∨ u.natDegree=1 ∨ u.natDegree=2 := by omega
  rcases hcases with hd | hd | hd
  · have hc : u.coeff 0=1 := by simpa only [hd] using htop
    have hu0 : u=1 := by rw [Polynomial.eq_C_of_natDegree_eq_zero hd,hc,map_one]
    have hv0 : v=0 := by
      by_contra hn
      rw [Polynomial.degree_eq_natDegree hn,hu0] at hv
      norm_num at hv
    exact ⟨.identity,hu0.symm,hv0.symm⟩
  · have hc : u.coeff 1=1 := by simpa only [hd] using htop
    have huv : u=X+C (u.coeff 0) := by
      simpa only [hc,map_one,one_mul] using expansion_one u (by omega)
    have hdegree : v.degree<((1:ℕ):WithBot ℕ) := by
      simpa only [Polynomial.degree_eq_natDegree hm.ne_zero,hd] using hv
    have hn : v.natDegree=0 := by
      have hh := natDegree_lt_of_positive_bound v 1 (by decide) hdegree
      omega
    refine ⟨.linear (-u.coeff 0) (v.coeff 0),?_,?_⟩
    · simpa only [uPolynomial,map_neg,sub_neg_eq_add] using huv.symm
    · exact (Polynomial.eq_C_of_natDegree_eq_zero hn).symm
  · have hc : u.coeff 2=1 := by simpa only [hd] using htop
    have huu : u=X^2+C (u.coeff 1)*X+C (u.coeff 0) := by
      simpa only [hc,map_one,one_mul] using expansion_two u hu
    have hdegree : v.degree<((2:ℕ):WithBot ℕ) := by
      simpa only [Polynomial.degree_eq_natDegree hm.ne_zero,hd] using hv
    have hn : v.natDegree≤1 := by
      have hh := natDegree_lt_of_positive_bound v 2 (by decide) hdegree
      omega
    exact ⟨.quadratic (u.coeff 1) (u.coeff 0) (v.coeff 1) (v.coeff 0),
      huu.symm,(expansion_one v hn).symm⟩

theorem canonical_valid_shape (u v : K[X]) (hm : u.Monic) (hu : u.natDegree≤2)
    (hv : v.degree<u.degree) (he : u ∣ v^2-curvePolynomial) :
    ∃ D : Candidate, Valid D ∧ uPolynomial D=u ∧ vPolynomial D=v := by
  obtain ⟨D,hU,hV⟩ := canonical_shape u v hm hu hv
  refine ⟨D,?_,hU,hV⟩
  unfold Valid
  apply (Polynomial.modByMonic_eq_zero_iff_dvd (candidates_monic D)).mpr
  simpa only [hU,hV] using he

def signature : Candidate→K×K×K×K×K
  | .identity => (1,0,0,0,0)
  | .linear a b => (-a,1,0,b,0)
  | .quadratic a b c d => (b,a,1,d,c)

theorem signature_injective : Function.Injective signature := by
  intro D E h
  cases D <;> cases E <;> simp_all [signature]

theorem signature_coefficients (D : Candidate) :
    signature D=((uPolynomial D).coeff 0,(uPolynomial D).coeff 1,
      (uPolynomial D).coeff 2,(vPolynomial D).coeff 0,(vPolynomial D).coeff 1) := by
  cases D <;> norm_num [signature,uPolynomial,vPolynomial,
    Polynomial.coeff_one,Polynomial.coeff_X]

theorem candidate_polynomials_injective (D E : Candidate)
    (hu : uPolynomial D=uPolynomial E) (hv : vPolynomial D=vPolynomial E) : D=E := by
  apply signature_injective
  rw [signature_coefficients,signature_coefficients,hu,hv]

/-- All actual polynomial pairs satisfying the standard reduced Mumford
conditions for the fixed one-infinity degree-five model. -/
@[ext] structure ReducedPair where
  u : K[X]
  v : K[X]
  monic_u : u.Monic
  degree_u : u.natDegree≤2
  degree_v : v.degree<u.degree
  equation : u ∣ v^2-curvePolynomial

def candidatePair (D : {D : Candidate // Valid D}) : ReducedPair where
  u := uPolynomial D.val
  v := vPolynomial D.val
  monic_u := candidates_monic D.val
  degree_u := candidates_degree_bound D.val
  degree_v := candidates_v_degree D.val
  equation := (Polynomial.modByMonic_eq_zero_iff_dvd (candidates_monic D.val)).mp D.property

theorem candidatePair_injective : Function.Injective candidatePair := by
  intro D E he
  apply Subtype.ext
  exact candidate_polynomials_injective D.val E.val
    (congrArg ReducedPair.u he) (congrArg ReducedPair.v he)

theorem candidatePair_surjective : Function.Surjective candidatePair := by
  intro P
  obtain ⟨D,hd,hu,hv⟩ := canonical_valid_shape P.u P.v P.monic_u P.degree_u P.degree_v P.equation
  exact ⟨⟨D,hd⟩,ReducedPair.ext hu hv⟩

def representationEquiv : {D : Candidate // Valid D}≃ReducedPair :=
  Equiv.ofBijective candidatePair ⟨candidatePair_injective,candidatePair_surjective⟩

instance : Fintype ReducedPair := Fintype.ofEquiv _ representationEquiv

theorem reducedPair_cardinality : Fintype.card ReducedPair=16 :=
  (Fintype.card_congr representationEquiv).symm.trans valid_cardinality

theorem valid_quadratic_has_root : ∀ a b c d : K,
    residueX a b c d=0 ∧ residueC a b c d=0 →
      ∃ r : K, r^2+a*r+b=0 := by decide

theorem quadratic_splits_of_root (a b r : K) (hr : r^2+a*r+b=0) :
    (quadraticU a b).Splits := by
  have hb : b= -r^2-a*r := by linear_combination hr
  have he : quadraticU a b=(X-C r)*(X-C (-a-r)) := by
    rw [hb]
    simp only [quadraticU,map_sub,map_neg,map_mul,map_pow]
    ring
  rw [he]
  exact (Polynomial.Splits.X_sub_C r).mul (Polynomial.Splits.X_sub_C (-a-r))

theorem valid_candidates_split (D : Candidate) (h : Valid D) :
    (uPolynomial D).Splits := by
  have hs := (valid_iff_coefficientValid D).mp h
  cases D with
  | identity => exact Polynomial.Splits.one
  | linear a b => exact Polynomial.Splits.X_sub_C a
  | quadratic a b c d =>
    obtain ⟨r,hr⟩ := valid_quadratic_has_root a b c d hs
    exact quadratic_splits_of_root a b r hr

theorem every_pair_smooth (P : ReducedPair) : P.u.Splits := by
  obtain ⟨D,rfl⟩ := candidatePair_surjective P
  exact valid_candidates_split D.val D.property

instance (P : ReducedPair) : Decidable P.u.Splits := Classical.propDecidable _

theorem smooth_reducedPair_cardinality :
    Fintype.card {P : ReducedPair // P.u.Splits}=16 := by
  rw [Fintype.card_congr (Equiv.subtypeUnivEquiv every_pair_smooth)]
  exact reducedPair_cardinality

def CoefficientVZero : Candidate→Prop
  | .identity => True
  | .linear _ b => b=0
  | .quadratic _ _ c d => c=0 ∧ d=0

instance (D : Candidate) : Decidable (CoefficientVZero D) := by
  cases D <;> unfold CoefficientVZero <;> infer_instance

theorem every_valid_v_zero : ∀ D : Candidate, CoefficientValid D →
    CoefficientVZero D := by decide

theorem every_pair_v_zero (P : ReducedPair) : P.v=0 := by
  obtain ⟨D,rfl⟩ := candidatePair_surjective P
  have hv := every_valid_v_zero D.val ((valid_iff_coefficientValid _).mp D.property)
  rcases D with ⟨D,hD⟩
  cases D with
  | identity => rfl
  | linear a b => simpa [candidatePair,vPolynomial,hv]
  | quadratic a b c d =>
    rcases hv with ⟨hc,hd⟩
    simp [candidatePair,vPolynomial,hc,hd]

theorem curve_monic : curvePolynomial.Monic := by
  apply Polynomial.monic_of_degree_le 5
  · unfold curvePolynomial; compute_degree!
  · norm_num [curvePolynomial,Polynomial.coeff_X]

theorem curve_degree : curvePolynomial.natDegree=5 := by
  unfold curvePolynomial
  compute_degree!

theorem curve_derivative : derivative curvePolynomial=(-1:K[X]) := by
  norm_num [curvePolynomial,Polynomial.derivative_X_pow]
  rw [show (5:K)=0 by decide]

theorem curve_separable : curvePolynomial.Separable := by
  apply (Polynomial.separable_def' _).mpr
  refine ⟨0,-1,?_⟩
  rw [curve_derivative]
  simp

/-- Exact positive control before native calibration: complete actual
polynomial-pair counts; no geometric Jacobian identification is assumed. -/
theorem mumford_five_control_certificate :
    curvePolynomial.Monic ∧ curvePolynomial.natDegree=5 ∧ curvePolynomial.Separable ∧
    Fintype.card ReducedPair=16 ∧
    Fintype.card {P : ReducedPair // P.u.Splits}=16 ∧
    (∀ P : ReducedPair, P.v=0) := by
  exact ⟨curve_monic,curve_degree,curve_separable,reducedPair_cardinality,
    smooth_reducedPair_cardinality,every_pair_v_zero⟩

#print axioms mumford_five_control_certificate

end
end MumfordFiveControlAudit
