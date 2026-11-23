import Common22CoordinateRing

/-!
# Affine point allocation for the source-style common 2+2 norm identity

These are pointwise statements over the base field. They do not construct
divisors, account for intersection multiplicity, or prove a Picard-class law.
-/

set_option autoImplicit false

namespace Common22PointAllocation

open Polynomial

variable {K : Type*} [Field K]

/-- Evaluation of the polynomial norm identity on an affine curve point. -/
theorem eval_norm
    (f u1 u2 n support : K[X]) (b x y : K)
    (hcurve : y ^ 2 = f.eval x)
    (hpoly : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f) :
    (n.eval x - b * y) * (n.eval x + b * y) =
      u1.eval x * u2.eval x * support.eval x := by
  have he := congrArg (Polynomial.eval x) hpoly
  simp only [eval_sub, eval_pow, eval_mul, eval_C] at he
  calc
    (n.eval x - b * y) * (n.eval x + b * y) =
        (n.eval x) ^ 2 - b ^ 2 * y ^ 2 := by ring
    _ = (n.eval x) ^ 2 - b ^ 2 * f.eval x := by rw [hcurve]
    _ = u1.eval x * u2.eval x * support.eval x := by
      linear_combination he

/-- Simultaneous zeros of the conjugate factors lie over a branch point
when 2 and the interpolation scale are nonzero. -/
theorem overlap_implies_branch (n b y : K)
    (h2 : (2 : K) ≠ 0) (hb : b ≠ 0)
    (hminus : n - b * y = 0) (hplus : n + b * y = 0) :
    y = 0 := by
  have hmul : (2 : K) * b * y = 0 := by
    calc
      (2 : K) * b * y = (n + b * y) - (n - b * y) := by ring
      _ = 0 := by rw [hminus, hplus]; ring
  exact (mul_eq_zero.mp hmul).resolve_left (mul_ne_zero h2 hb)

/-- At an unramified rational point above a root of the norm product,
exactly one conjugate factor vanishes. -/
theorem exactly_one_nonbranch
    (f u1 u2 n support : K[X]) (b x y : K)
    (h2 : (2 : K) ≠ 0) (hb : b ≠ 0) (hy : y ≠ 0)
    (hcurve : y ^ 2 = f.eval x)
    (hpoly : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f)
    (hroot : u1.eval x * u2.eval x * support.eval x = 0) :
    ((n.eval x - b * y = 0) ∧ (n.eval x + b * y ≠ 0)) ∨
      ((n.eval x - b * y ≠ 0) ∧ (n.eval x + b * y = 0)) := by
  have hproduct := eval_norm f u1 u2 n support b x y hcurve hpoly
  rw [hroot] at hproduct
  have hdisjoint : ¬(n.eval x - b * y = 0 ∧ n.eval x + b * y = 0) := by
    rintro ⟨hminus, hplus⟩
    exact hy (overlap_implies_branch (n.eval x) b y h2 hb hminus hplus)
  rcases mul_eq_zero.mp hproduct with hminus | hplus
  · exact Or.inl ⟨hminus, fun hplus => hdisjoint ⟨hminus, hplus⟩⟩
  · exact Or.inr ⟨(fun hminus => hdisjoint ⟨hminus, hplus⟩), hplus⟩

/-- At a rational branch point with norm-product zero, both conjugate
factors vanish. This is the exceptional overlap that the pointwise allocation
theorem excludes. -/
theorem branch_overlap
    (f u1 u2 n support : K[X]) (b x y : K)
    (hcurve : y ^ 2 = f.eval x)
    (hpoly : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f)
    (hroot : u1.eval x * u2.eval x * support.eval x = 0)
    (hy : y = 0) :
    n.eval x - b * y = 0 ∧ n.eval x + b * y = 0 := by
  have hproduct := eval_norm f u1 u2 n support b x y hcurve hpoly
  rw [hroot] at hproduct
  have hn2 : n.eval x * n.eval x = 0 := by simpa [hy, pow_two] using hproduct
  rcases mul_eq_zero.mp hn2 with hn | hn <;> simp [hn, hy]

/-- Direct source-style evaluated norm identity on a rational affine point.
This uses the proved source-style polynomial theorem, not an assumed norm. -/
theorem source_style_eval_norm
    (q1 r1 s1 t1 q2 r2 s2 t2 : K) (f : K[X]) (x y : K)
    (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0)
    (valid1 : Common22Coprime.firstSupport q1 r1 ∣
      f - (C s1 * X + C t1) ^ 2)
    (valid2 : Common22Coprime.secondSupport q2 r2 ∣
      f - (C s2 * X + C t2) ^ 2)
    (f_degree : f.natDegree ≤ 5)
    (f_five : f.coeff 5 = 1)
    (f_four : f.coeff 4 = 0)
    (hcurve : y ^ 2 = f.eval x) :
    let (b, a, c, q, r) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
      C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1)
    let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
    (n.eval x - b * y) * (n.eval x + b * y) =
      u1.eval x * u2.eval x * support.eval x := by
  have hs := Common22SourceHead.source_curve_eq q1 r1 s1 t1 q2 r2 s2 t2 f
    hb valid1 valid2 f_degree f_five f_four
  unfold Common22SourceHead.sourceE at hs
  simpa only using (eval_norm f
    (X ^ 2 + C q1 * X + C r1)
    (X ^ 2 + C q2 * X + C r2)
    (let (b, a, c, _, _) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
     C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
      C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1))
    (let (_, _, c, q, r) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
     C (c ^ 2) * X ^ 2 + C q * X + C r)
    (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1
    x y hcurve
    (by simpa only [Common22Coprime.firstSupport,
      Common22Coprime.secondSupport] using hs))

/-- Vanishing source-style cubic coefficient lowers the interpolant's
polynomial degree to at most two. The pole order of `n/b-Y` therefore cannot
be asserted to be six solely from the common-branch determinant gate. -/
theorem cubic_degree_le_two_of_c_zero
    (c n2 n1 n0 : K) (hc : c = 0) :
    degree (C (-c) * X ^ 3 + C n2 * X ^ 2 + C n1 * X + C n0 : K[X]) ≤ 2 := by
  subst c
  simp only [neg_zero, map_zero, zero_mul, zero_add]
  apply degree_add_le_of_degree_le
  · apply degree_add_le_of_degree_le
    · exact degree_C_mul_X_pow_le 2 n2
    · exact (degree_C_mul_X_le n1).trans (by norm_num)
  · exact (degree_C_le).trans (by norm_num)

/-- The corresponding residual support polynomial drops to degree at most
one when the source-style cubic coefficient vanishes. -/
theorem support_degree_le_one_of_c_zero
    (c q r : K) (hc : c = 0) :
    degree (C (c ^ 2) * X ^ 2 + C q * X + C r : K[X]) ≤ 1 := by
  subst c
  simp only [zero_pow (by norm_num : 2 ≠ 0), map_zero, zero_mul, zero_add]
  apply degree_add_le_of_degree_le
  · exact degree_C_mul_X_le q
  · exact (degree_C_le).trans (by norm_num)

/-- Source coefficient values with nonzero determinant `B` but zero cubic
coefficient `C`: equal `q` and equal ordinates, with distinct constants in
the two quadratic supports. This is an algebraic gate witness; validity on a
particular smooth curve is a separate condition. -/
theorem b_ne_zero_c_zero_witness :
    let data := Common22SupportHead.coefficientData
      (0 : K) 1 0 1 0 2 0 1
    data.1 ≠ 0 ∧ data.2.2.1 = 0 := by
  have h : (1 : K) - 2 = -1 := by ring
  simp [Common22SupportHead.coefficientData, h]

end Common22PointAllocation

#print axioms Common22PointAllocation.eval_norm
#print axioms Common22PointAllocation.overlap_implies_branch
#print axioms Common22PointAllocation.exactly_one_nonbranch
#print axioms Common22PointAllocation.branch_overlap
#print axioms Common22PointAllocation.source_style_eval_norm
#print axioms Common22PointAllocation.cubic_degree_le_two_of_c_zero
#print axioms Common22PointAllocation.support_degree_le_one_of_c_zero
#print axioms Common22PointAllocation.b_ne_zero_c_zero_witness
