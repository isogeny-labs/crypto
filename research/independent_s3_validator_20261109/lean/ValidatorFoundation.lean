import S3Completion
import Mathlib.FieldTheory.Finite.Basic
import Mathlib.FieldTheory.Finiteness
import Mathlib.RingTheory.AdjoinRoot
import Mathlib.Algebra.Polynomial.SpecificDegree

/-! Mathematical foundation for the independent finite S3 validator.
Actual finite-field extension/cardinality and actual Mathlib elliptic points.
This does not prove Rust compilation, execution or host independence. -/
namespace IndependentS3Audit
noncomputable section
open Polynomial SemaevS3 TheoremAudit

theorem odd_square_half (q : ℕ) (h : Odd q) :
    q^2/2 = (q-1)*((q+1)/2) := by
  obtain ⟨k,rfl⟩ := h
  have hr : (2*k+1+1)/2 = k+1 := by omega
  rw [hr]
  simp only [Nat.add_sub_cancel]
  rw [show (2*k+1)^2 = 2*(2*k*(k+1))+1 by ring]
  simp [Nat.add_div]

/-- Every base-field element acquires a square root in a quadratic finite
field extension. The square criterion is applied to the actual field. -/
theorem base_element_isSquare (F K : Type*) [Field F] [Field K]
    [Fintype F] [Fintype K] [Algebra F K]
    (hcard : Fintype.card K = (Fintype.card F)^2)
    (hodd : Odd (Fintype.card F)) (a : F) : IsSquare (algebraMap F K a) := by
  by_cases ha : a=0
  · simp [ha]
  have hchar : ringChar K ≠ 2 := by
    intro hc
    have hk := FiniteField.even_card_of_char_two hc
    rw [hcard, Nat.pow_mod, Nat.odd_iff.mp hodd] at hk
    norm_num at hk
  have ha' : algebraMap F K a ≠ 0 := by
    intro h
    exact ha ((algebraMap F K).injective (by simpa using h))
  apply (FiniteField.isSquare_iff hchar ha').mpr
  rw [hcard, odd_square_half _ hodd, pow_mul, ← map_pow,
    FiniteField.pow_card_sub_one_eq_one a ha, map_one, one_pow]

def extensionPolynomial (F : Type*) [Field F] (d : F) : Polynomial F := X^2-C d

theorem extensionPolynomial_monic (F : Type*) [Field F] (d : F) :
    (extensionPolynomial F d).Monic := monic_X_pow_sub_C _ (by decide)

theorem extensionPolynomial_irreducible (F : Type*) [Field F] (d : F)
    (hd : ∀ x : F, x^2 ≠ d) : Irreducible (extensionPolynomial F d) := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · simp [extensionPolynomial]
  · intro x hx
    exact hd x (by simpa [Polynomial.IsRoot,extensionPolynomial,sub_eq_zero] using hx)

theorem extension_finrank (F : Type*) [Field F] (d : F) :
    Module.finrank F (AdjoinRoot (extensionPolynomial F d)) = 2 := by
  rw [(AdjoinRoot.powerBasis (extensionPolynomial_monic F d).ne_zero).finrank]
  change (extensionPolynomial F d).natDegree=2
  simp [extensionPolynomial]

theorem extension_cardinality (F : Type*) [Field F] [Fintype F] (d : F) :
    Nat.card (AdjoinRoot (extensionPolynomial F d)) = (Fintype.card F)^2 := by
  let : Module.Finite F (AdjoinRoot (extensionPolynomial F d)) :=
    (AdjoinRoot.powerBasis (extensionPolynomial_monic F d).ne_zero).finite
  rw [Module.natCard_eq_pow_finrank (K:=F),extension_finrank,Nat.card_eq_fintype_card]

theorem actual_quadratic_square_lifts (F : Type*) [Field F] [Fintype F]
    (d : F) (hd : ∀ x : F, x^2 ≠ d) (hodd : Odd (Fintype.card F)) (a : F) :
    IsSquare (algebraMap F (AdjoinRoot (extensionPolynomial F d)) a) := by
  let : Fact (Irreducible (extensionPolynomial F d)) :=
    ⟨extensionPolynomial_irreducible F d hd⟩
  let b := (AdjoinRoot.powerBasis (extensionPolynomial_monic F d).ne_zero).basis
  let : Finite (AdjoinRoot (extensionPolynomial F d)) :=
    Finite.of_injective b.equivFun b.equivFun.injective
  let : Fintype (AdjoinRoot (extensionPolynomial F d)) := Fintype.ofFinite _
  apply base_element_isSquare F (AdjoinRoot (extensionPolynomial F d)) _ hodd a
  simpa only [Nat.card_eq_fintype_card] using extension_cardinality F d

/-- The exact pair-sum oracle used by the finite check, including repeated
coordinates and two-torsion. The points belong to Mathlib's actual group. -/
theorem actual_pair_oracle_equivalence {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0)
    (h2 : (2:K) ≠ 0) {xR yR xF yF : K}
    (hR : W.Nonsingular xR yR) (hF : W.Nonsingular xF yF) (z : K) :
    S3 W.a₄ W.a₆ xR xF z=0 ↔
      pointX (WeierstrassCurve.Affine.Point.some xR yR hR +
        WeierstrassCurve.Affine.Point.some xF yF hF)=some z ∨
      pointX (WeierstrassCurve.Affine.Point.some xR yR hR +
        -WeierstrassCurve.Affine.Point.some xF yF hF)=some z :=
  S3_pointX_iff W ha1 ha2 ha3 h2 hR hF z

theorem chosen_moduli_prime :
    ∀ p ∈ ([3,5,7,11,13,17,19,23] : List ℕ), Nat.Prime p := by decide

def baseZeroCount (p : ℕ) [Fact (Nat.Prime p)] [NeZero p] : ℕ :=
  ∑ a : ZMod p, ∑ b : ZMod p,
    if 4*a^3+27*b^2=0 then 0 else
      (Finset.univ.filter (fun xyz : ZMod p × ZMod p × ZMod p =>
        S3 a b xyz.1 xyz.2.1 xyz.2.2=0)).card

/-- Exact small calibration on the actual polynomial and discriminant. -/
theorem preflight_zero_count : baseZeroCount 3 = 54 := by
  unfold baseZeroCount SemaevS3.S3
  decide +kernel

#print axioms base_element_isSquare
#print axioms extensionPolynomial_irreducible
#print axioms extension_cardinality
#print axioms actual_quadratic_square_lifts
#print axioms actual_pair_oracle_equivalence
#print axioms chosen_moduli_prime
#print axioms preflight_zero_count
end
end IndependentS3Audit
