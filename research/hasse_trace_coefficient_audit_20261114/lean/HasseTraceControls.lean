import ActualHasseTrace
import Mathlib.FieldTheory.Finite.GaloisField

namespace HasseTraceCoefficientAudit
noncomputable section
open scoped BigOperators
open Polynomial QuadraticTwistCountAudit SmallPrimeTraceGeometryAudit

instance thirteenPrime : Fact (Nat.Prime 13) := ⟨by decide +kernel⟩

def curve13a : WeierstrassCurve.Affine (ZMod 13) := cubicModel 0 (-1) 0
def curve13b : WeierstrassCurve.Affine (ZMod 13) := cubicModel 0 0 4

theorem fixture13a : curve13a.Δ ≠ 0 ∧ equationCount curve13a = 8 := by decide +kernel
theorem fixture13b : curve13b.Δ ≠ 0 ∧ equationCount curve13b = 21 := by decide +kernel

theorem actual_counts13 : Nat.card curve13a.Point = 8 ∧ Nat.card curve13b.Point = 21 := by
  rw [actual_point_count curve13a fixture13a.1, actual_point_count curve13b fixture13b.1]
  exact ⟨fixture13a.2, fixture13b.2⟩

theorem actual_traces13 : SupersingularTraceClassesAudit.fieldTrace curve13a = 6 ∧
    SupersingularTraceClassesAudit.fieldTrace curve13b = -7 := by
  unfold SupersingularTraceClassesAudit.fieldTrace
  rw [actual_counts13.1, actual_counts13.2]
  norm_num

theorem coefficients13 : hasseCoefficient (0 : ZMod 13) (-1) 0 = 6 ∧
    hasseCoefficient (0 : ZMod 13) 0 4 = 6 := by
  have ha := actual_trace_residue (K := ZMod 13) (by norm_num [ZMod.ringChar_zmod_n])
    0 (-1) 0 fixture13a.1
  have hb := actual_trace_residue (K := ZMod 13) (by norm_num [ZMod.ringChar_zmod_n])
    0 0 4 fixture13b.1
  change (SupersingularTraceClassesAudit.fieldTrace curve13a : ZMod 13) = _ at ha
  change (SupersingularTraceClassesAudit.fieldTrace curve13b : ZMod 13) = _ at hb
  rw [actual_traces13.1] at ha
  rw [actual_traces13.2] at hb
  constructor
  · simpa using ha.symm
  · calc
      _ = (-7 : ZMod 13) := by simpa using hb.symm
      _ = 6 := by decide +kernel

theorem prime13_uniqueness_false :
    ¬∀ a b c a' b' c' : ZMod 13,
      (cubicModel a b c).Δ ≠ 0 → (cubicModel a' b' c').Δ ≠ 0 →
      hasseCoefficient a b c = hasseCoefficient a' b' c' →
        Nat.card (cubicModel a b c).Point = Nat.card (cubicModel a' b' c').Point := by
  intro h
  have he := h 0 (-1) 0 0 0 4 fixture13a.1 fixture13b.1
    (coefficients13.1.trans coefficients13.2.symm)
  change Nat.card curve13a.Point = Nat.card curve13b.Point at he
  rw [actual_counts13.1, actual_counts13.2] at he
  omega

theorem char_two_root_card : rootCard (1 : ZMod 2) = 1 := by
  apply Nat.card_eq_one_iff_exists.mpr
  refine ⟨⟨1, by norm_num⟩, ?_⟩
  intro y
  apply Subtype.ext
  have h := sq_eq_one_iff.mp y.property
  rcases h with h | h
  · exact h
  · simpa using h

theorem char_two_euler_false :
    (rootCard (1 : ZMod 2) : ZMod 2) ≠ 1 + (1 : ZMod 2) ^ ((Fintype.card (ZMod 2) - 1) / 2) := by
  rw [char_two_root_card]
  simpa only [Nat.cast_one, one_pow] using
    (show (1 : ZMod 2) ≠ 1 + 1 by decide +kernel)

abbrev F9 := GaloisField 3 2
noncomputable instance field9Fintype : Fintype F9 := Fintype.ofFinite F9
noncomputable instance field9DecidableEq : DecidableEq F9 := Classical.decEq F9

theorem field9_card : Fintype.card F9 = 9 := by
  rw [← Nat.card_eq_fintype_card, GaloisField.card 3 2 (by decide)]
  norm_num

theorem field9_char : ringChar F9 = 3 := ringChar.eq F9 3

def characteristicCoefficient {K : Type*} [Field K] (a b c : K) : K :=
  ((cubicPolynomial a b c) ^ ((ringChar K - 1) / 2)).coeff (ringChar K - 1)

theorem cubic_delta_special {K : Type*} [Field K] (a : K) :
    (cubicModel a 0 1).Δ = -64 * a ^ 3 - 432 := by
  simp only [cubicModel, WeierstrassCurve.Δ, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈]
  ring

theorem field9_coefficients (a : F9) (ha : a ^ 2 = -1) :
    characteristicCoefficient a 0 1 = a ∧ hasseCoefficient a 0 1 = 1 := by
  constructor
  · norm_num [characteristicCoefficient, field9_char, cubicPolynomial, Polynomial.coeff_one]
  · have hp : (cubicPolynomial a 0 1) ^ 3 =
        (X ^ 9 + C (a ^ 3) * X ^ 6 + 1 : F9[X]) := by
      simp only [cubicPolynomial, map_zero, zero_mul, add_zero, map_one]
      rw [add_pow_char (x := (X ^ 3 + C a * X ^ 2 : F9[X])) (y := 1) (p := 3)]
      rw [add_pow_char (x := (X ^ 3 : F9[X])) (y := C a * X ^ 2) (p := 3)]
      simp only [mul_pow, ← pow_mul, map_pow, one_pow]
    have hfour : (cubicPolynomial a 0 1) ^ 4 =
        (X ^ 9 + C (a ^ 3) * X ^ 6 + 1) * cubicPolynomial a 0 1 := by
      rw [pow_succ, hp]
    have ha4 : a ^ 4 = 1 := by
      calc
        a ^ 4 = (a ^ 2) ^ 2 := by ring
        _ = 1 := by rw [ha]; norm_num
    have hid : (X ^ 9 + C (a ^ 3) * X ^ 6 + 1) * cubicPolynomial a 0 1 =
        (X ^ 12 + C a * X ^ 11 + C (1 + a ^ 3) * X ^ 9 + C (a ^ 4) * X ^ 8 +
          C (a ^ 3) * X ^ 6 + X ^ 3 + C a * X ^ 2 + 1 : F9[X]) := by
      simp only [cubicPolynomial, map_add, map_pow, map_zero, map_one, zero_mul, add_zero]
      ring
    simp only [hasseCoefficient, field9_card]
    change ((cubicPolynomial a 0 1) ^ 4).coeff 8 = 1
    rw [hfour, hid]
    norm_num only [Polynomial.coeff_add, Polynomial.coeff_C_mul_X_pow,
      Polynomial.coeff_X_pow, Polynomial.coeff_one, ha4]
    simp

theorem field9_control : ∃ a : F9,
    (cubicModel a 0 1).Δ ≠ 0 ∧ characteristicCoefficient a 0 1 = a ∧
      hasseCoefficient a 0 1 = 1 ∧
      (SupersingularTraceClassesAudit.fieldTrace (cubicModel a 0 1) : F9) = 1 ∧
      characteristicCoefficient a 0 1 ≠
        (SupersingularTraceClassesAudit.fieldTrace (cubicModel a 0 1) : F9) := by
  have hF : ringChar F9 ≠ 2 := by rw [field9_char]; decide
  have hs : IsSquare (-1 : F9) :=
    (FiniteField.isSquare_iff hF (neg_ne_zero.mpr one_ne_zero)).mpr (by
      rw [field9_card]
      norm_num)
  obtain ⟨a, ha⟩ := hs
  have ha2 : a ^ 2 = -1 := by simpa only [pow_two] using ha.symm
  have ha0 : a ≠ 0 := by intro hz; rw [hz] at ha2; norm_num at ha2
  have h63 : (63 : F9) = 0 := (CharP.cast_eq_zero_iff F9 3 63).mpr (by decide)
  have h64 : (64 : F9) = 1 := by linear_combination h63
  have h432 : (432 : F9) = 0 := (CharP.cast_eq_zero_iff F9 3 432).mpr (by decide)
  have hd : (cubicModel a 0 1).Δ ≠ 0 := by
    rw [cubic_delta_special, h64, h432]
    simpa using neg_ne_zero.mpr (pow_ne_zero 3 ha0)
  have hc := field9_coefficients a ha2
  have ht : (SupersingularTraceClassesAudit.fieldTrace (cubicModel a 0 1) : F9) = 1 := by
    rw [actual_trace_residue hF a 0 1 hd, hc.2]
  have hne : a ≠ 1 := by
    intro hz
    rw [hz] at ha2
    have htwo : (2 : F9) = 0 := by linear_combination ha2
    have hdiv := (CharP.cast_eq_zero_iff F9 3 2).mp htwo
    norm_num at hdiv
  exact ⟨a, hd, hc.1, hc.2, ht, by rw [hc.1, ht]; exact hne⟩

theorem degree_boundary_fixture :
    (∑ x : ZMod 3, x ^ 4) ≠ -(X ^ 4 : (ZMod 3)[X]).coeff 2 := by
  rw [sum_pow_nonzero 4 (by decide), ZMod.card, Polynomial.coeff_X_pow]
  decide +kernel

theorem nonstrict_degree_bound_false :
    ¬∀ f : (ZMod 3)[X], f.natDegree ≤ 2 * (Fintype.card (ZMod 3) - 1) →
      (∑ x : ZMod 3, f.eval x) = -f.coeff (Fintype.card (ZMod 3) - 1) := by
  intro h
  have he := h (X ^ 4) (by norm_num [ZMod.card])
  apply degree_boundary_fixture
  simpa only [Polynomial.eval_pow, Polynomial.eval_X, ZMod.card] using he

#print axioms actual_counts13
#print axioms prime13_uniqueness_false
#print axioms char_two_euler_false
#print axioms field9_control
#print axioms nonstrict_degree_bound_false
end
end HasseTraceCoefficientAudit
