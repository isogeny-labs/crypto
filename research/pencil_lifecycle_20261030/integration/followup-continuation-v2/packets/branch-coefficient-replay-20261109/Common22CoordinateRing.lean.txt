import Mathlib.RingTheory.AdjoinRoot
import Common22SourceHead

open Polynomial

namespace Common22CoordinateRing

variable {K : Type*} [Field K]

noncomputable def equation (f : K[X]) : (K[X])[X] := X ^ 2 - C f
abbrev Coordinate (f : K[X]) := AdjoinRoot (equation f)

noncomputable def ordinate (f : K[X]) : Coordinate f := AdjoinRoot.root (equation f)
noncomputable def abscissa (f : K[X]) (g : K[X]) : Coordinate f :=
  AdjoinRoot.of (equation f) g

theorem ordinate_sq (f : K[X]) : ordinate f ^ 2 = abscissa f f := by
  have h : AdjoinRoot.mk (equation f) ((X : (K[X])[X]) ^ 2 - C f) = 0 := by
    simpa only [equation] using (AdjoinRoot.mk_self (f := equation f))
  have h' : ordinate f ^ 2 - abscissa f f = 0 := by
    simpa only [ordinate, abscissa, map_sub, map_pow,
      AdjoinRoot.mk_X, AdjoinRoot.mk_C] using h
  exact sub_eq_zero.mp h'

theorem norm_factorization (f u1 u2 n support : K[X]) (b : K)
    (h : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f) :
    (abscissa f n - abscissa f (C b) * ordinate f) *
      (abscissa f n + abscissa f (C b) * ordinate f) =
      abscissa f (u1 * u2 * support) := by
  have hn : n ^ 2 - C (b ^ 2) * f = u1 * u2 * support := by
    calc
      n ^ 2 - C (b ^ 2) * f = (n ^ 2 - u1 * u2 * support) +
          u1 * u2 * support - C (b ^ 2) * f := by ring
      _ = u1 * u2 * support := by rw [h]; ring
  have hc := congrArg (abscissa f) hn
  simp only [abscissa, map_sub, map_mul, map_pow] at hc
  change (abscissa f n) ^ 2 - (abscissa f (C b)) ^ 2 * abscissa f f =
    abscissa f u1 * abscissa f u2 * abscissa f support at hc
  calc
    (abscissa f n - abscissa f (C b) * ordinate f) *
        (abscissa f n + abscissa f (C b) * ordinate f) =
      (abscissa f n) ^ 2 - (abscissa f (C b)) ^ 2 * ordinate f ^ 2 := by ring
    _ = (abscissa f n) ^ 2 - (abscissa f (C b)) ^ 2 * abscissa f f := by
      rw [ordinate_sq]
    _ = abscissa f (u1 * u2 * support) := by
      rw [hc]
      simp only [abscissa, map_mul]

/-- The proof-side source-style common branch has the norm factorization in
the concrete affine coordinate ring, conditional on the same valid-input and
quintic-head hypotheses as `Common22SourceHead.source_curve_eq`. -/
theorem source_style_norm_factorization
    (q1 r1 s1 t1 q2 r2 s2 t2 : K) (f : K[X])
    (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0)
    (valid1 : Common22Coprime.firstSupport q1 r1 ∣
      f - (C s1 * X + C t1) ^ 2)
    (valid2 : Common22Coprime.secondSupport q2 r2 ∣
      f - (C s2 * X + C t2) ^ 2)
    (f_degree : f.natDegree ≤ 5)
    (f_five : f.coeff 5 = 1)
    (f_four : f.coeff 4 = 0) :
    let (b, a, c, q, r) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + a)) * X ^ 2 +
      C (b * s1 - c * r1 - a * q1) * X + C (b * t1 - a * r1)
    let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
    (abscissa f n - abscissa f (C b) * ordinate f) *
      (abscissa f n + abscissa f (C b) * ordinate f) =
        abscissa f (u1 * u2 * support) := by
  have hs := Common22SourceHead.source_curve_eq q1 r1 s1 t1 q2 r2 s2 t2 f
    hb valid1 valid2 f_degree f_five f_four
  unfold Common22SourceHead.sourceE at hs
  simpa only using (norm_factorization f
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
    (by simpa only [Common22Coprime.firstSupport,
      Common22Coprime.secondSupport] using hs))

end Common22CoordinateRing

#print axioms Common22CoordinateRing.ordinate_sq
#print axioms Common22CoordinateRing.norm_factorization
#print axioms Common22CoordinateRing.source_style_norm_factorization
