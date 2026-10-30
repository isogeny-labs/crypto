import Common22PolynomialHead
import Common22SupportHead
import Common22Coprime
import Common22Degree
import Common22Residues

/-!
Actual polynomial head for the proof-side unit-weight common 2+2 residual.
The source-derived scalar coefficient formulas are substituted into the
generic polynomial expansion. This does not parse Rust or prove the curve
and divisor-class bridge.
-/

open Polynomial
variable {K : Type*} [Field K]

namespace Common22SourceHead

theorem canonical_n_eq_interpolant (q1 r1 s1 t1 b a c : K) :
    (C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
      C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1) : K[X]) =
    C b * (C s1 * X + C t1) -
      (C c * X + C a) * (X ^ 2 + C q1 * X + C r1) := by
  simp only [map_add, map_sub, map_mul, map_neg]
  ring

/-- The source-style cubic also agrees with the second input ordinate
modulo the second quadratic support. -/
theorem canonical_n_interpolates_second
    (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    let (b, a, c, _, _) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
    let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
      C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1)
    n - C b * (C s2 * X + C t2) =
      (C (-c) * X + C (-c * (q1 - q2) - a)) *
        (X ^ 2 + C q2 * X + C r2) := by
  dsimp [Common22SupportHead.coefficientData]
  simp only [map_add, map_sub, map_mul, map_pow, map_neg]
  ring

noncomputable def sourceE (q1 r1 s1 t1 q2 r2 s2 t2 : K) : K[X] :=
  let (b, a, c, q, r) := Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
  let u1 : K[X] := X ^ 2 + C q1 * X + C r1
  let u2 : K[X] := X ^ 2 + C q2 * X + C r2
  let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
    C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1)
  let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
  n ^ 2 - u1 * u2 * support

theorem sourceE_coeff_six (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2).coeff 6 = 0 := by
  unfold sourceE
  simp only [Common22SupportHead.coefficientData]
  rw [Common22PolynomialHead.generic_coeff_six]
  ring


theorem sourceE_coeff_five (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2).coeff 5 =
      (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ^ 2 := by
  unfold sourceE
  simp only [Common22SupportHead.coefficientData]
  rw [Common22PolynomialHead.generic_coeff_five]
  ring_nf

theorem sourceE_coeff_four (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2).coeff 4 = 0 := by
  unfold sourceE
  simp only [Common22SupportHead.coefficientData]
  rw [Common22PolynomialHead.generic_coeff_four]
  ring_nf

/-- The proof-side source-style residual is at most sextic. -/
theorem sourceE_natDegree_le_six (q1 r1 s1 t1 q2 r2 s2 t2 : K) :
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2).natDegree ≤ 6 := by
  unfold sourceE
  simp only [Common22SupportHead.coefficientData]
  apply Common22PolynomialHead.generic_natDegree_le_six

/-- For a monic quintic with zero fourth coefficient, the source-style
residual difference has degree below the two-support product. -/
theorem sourceE_strict_degree
    (q1 r1 s1 t1 q2 r2 s2 t2 : K) (f : K[X])
    (f_degree : f.natDegree ≤ 5)
    (f_five : f.coeff 5 = 1)
    (f_four : f.coeff 4 = 0) :
    degree (sourceE q1 r1 s1 t1 q2 r2 s2 t2 -
      C ((Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ^ 2) * f) <
    degree (Common22Coprime.firstSupport q1 r1 *
      Common22Coprime.secondSupport q2 r2) := by
  exact Common22Degree.cleared_difference_degree_lt_supports_of_sextic_head
    (Common22Coprime.firstSupport q1 r1)
    (Common22Coprime.secondSupport q2 r2)
    (sourceE q1 r1 s1 t1 q2 r2 s2 t2) f
    (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1
    (Common22Coprime.firstSupport_nonzero q1 r1)
    (Common22Coprime.secondSupport_nonzero q2 r2)
    (Common22Coprime.firstSupport_natDegree q1 r1)
    (Common22Coprime.secondSupport_natDegree q2 r2)
    (sourceE_natDegree_le_six q1 r1 s1 t1 q2 r2 s2 t2)
    (sourceE_coeff_six q1 r1 s1 t1 q2 r2 s2 t2)
    f_degree
    (sourceE_coeff_five q1 r1 s1 t1 q2 r2 s2 t2)
    f_five
    (sourceE_coeff_four q1 r1 s1 t1 q2 r2 s2 t2)
    f_four

set_option maxHeartbeats 500000 in
/-- The source-style common 2+2 residual equals the cleared curve polynomial
under valid Mumford inputs, nonzero interpolation determinant, and the
normalised quintic head assumptions. This is a proof-side polynomial theorem. -/
theorem source_curve_eq
    (q1 r1 s1 t1 q2 r2 s2 t2 : K) (f : K[X])
    (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0)
    (valid1 : Common22Coprime.firstSupport q1 r1 ∣
      f - (C s1 * X + C t1) ^ 2)
    (valid2 : Common22Coprime.secondSupport q2 r2 ∣
      f - (C s2 * X + C t2) ^ 2)
    (f_degree : f.natDegree ≤ 5)
    (f_five : f.coeff 5 = 1)
    (f_four : f.coeff 4 = 0) :
    Common22SourceHead.sourceE q1 r1 s1 t1 q2 r2 s2 t2 =
      C ((Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ^ 2) * f := by
  let data := Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
  let b := data.1
  let a := data.2.1
  let c := data.2.2.1
  let q := data.2.2.2.1
  let r := data.2.2.2.2
  let u1 := Common22Coprime.firstSupport q1 r1
  let u2 := Common22Coprime.secondSupport q2 r2
  let v1 : K[X] := C s1 * X + C t1
  let v2 : K[X] := C s2 * X + C t2
  let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
    C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1)
  let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
  have hcop : IsCoprime u1 u2 := by
    have hdet : Common22Coprime.determinant q1 r1 q2 r2 ≠ 0 := by
      simpa [Common22SupportHead.coefficientData,
        Common22Coprime.determinant] using hb
    exact Common22Coprime.supports_coprime q1 r1 q2 r2 hdet
  have hfirst : u1 ∣ n - C b * v1 := by
    refine ⟨-(C c * X + C a), ?_⟩
    dsimp [n, u1, v1, Common22Coprime.firstSupport]
    rw [Common22SourceHead.canonical_n_eq_interpolant q1 r1 s1 t1 b a c]
    ring
  have hsecond : u2 ∣ n - C b * v2 := by
    refine ⟨C (-c) * X + C (-c * (q1 - q2) - a), ?_⟩
    have h2 := Common22SourceHead.canonical_n_interpolates_second
      q1 r1 s1 t1 q2 r2 s2 t2
    dsimp only [n, u2, v2, Common22Coprime.secondSupport] at h2 ⊢
    exact h2.trans (mul_comm _ _)
  have hsmall : degree (n ^ 2 - u1 * u2 * support - (C b) ^ 2 * f) <
      degree (u1 * u2) := by
    have hs := Common22SourceHead.sourceE_strict_degree
      q1 r1 s1 t1 q2 r2 s2 t2 f f_degree f_five f_four
    dsimp only [Common22SourceHead.sourceE] at hs
    simpa only [n, u1, u2, support,
      Common22Coprime.firstSupport, Common22Coprime.secondSupport,
      map_pow] using hs
  have hres := Common22Residues.residual_curve_eq u1 u2 n v1 v2 (C b)
    support f hcop hfirst hsecond valid1 valid2 hsmall
  change n ^ 2 - u1 * u2 * support = C (b ^ 2) * f
  simpa only [map_pow] using hres

end Common22SourceHead

#print axioms Common22SourceHead.sourceE_coeff_six
#print axioms Common22SourceHead.sourceE_coeff_five
#print axioms Common22SourceHead.sourceE_coeff_four
#print axioms Common22SourceHead.canonical_n_eq_interpolant
#print axioms Common22SourceHead.canonical_n_interpolates_second
#print axioms Common22SourceHead.sourceE_natDegree_le_six
#print axioms Common22SourceHead.sourceE_strict_degree
#print axioms Common22SourceHead.source_curve_eq
