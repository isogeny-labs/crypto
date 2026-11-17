import Mathlib.Algebra.Field.Basic
import Mathlib.Tactic.Ring

/-!
The eight cross-scaled inputs to the common weighted 2+2 branch are literal
polynomials. The separate source checker compares these definitions with the
pinned Rust statements. This theorem connects them to the represented affine
coefficients when the weighted numerator equations hold; it does not prove
the final Jacobian addition law.
-/

set_option autoImplicit false

namespace CrossScale22

variable {R : Type*} [CommRing R]

structure WeightedInput (R : Type*) where
  q1 : R
  r1 : R
  s1 : R
  t1 : R
  z1 : R
  w1 : R
  q2 : R
  r2 : R
  s2 : R
  t2 : R
  z2 : R
  w2 : R

structure CrossInput (R : Type*) where
  q1c : R
  q2c : R
  r1c : R
  r2c : R
  s1c : R
  s2c : R
  t1c : R
  t2c : R

/-- Source-shaped straight-line calculation through the cross-scaled inputs. -/
def crossScale22 (i : WeightedInput R) : CrossInput R :=
  let z1_2 := i.z1 ^ 2
  let z2_2 := i.z2 ^ 2
  let z1_3 := z1_2 * i.z1
  let z2_3 := z2_2 * i.z2
  let z1_4 := z1_2 ^ 2
  let z2_4 := z2_2 ^ 2
  let z1_5 := z1_4 * i.z1
  let z2_5 := z2_4 * i.z2
  let q1c := i.q1 * z2_2
  let q2c := i.q2 * z1_2
  let r1c := i.r1 * z2_4
  let r2c := i.r2 * z1_4
  let s1c := (i.s1 * z2_3) * i.w2
  let s2c := (i.s2 * z1_3) * i.w1
  let t1c := (i.t1 * z2_5) * i.w2
  let t2c := (i.t2 * z1_5) * i.w1
  { q1c, q2c, r1c, r2c, s1c, s2c, t1c, t2c }

/-- If the weighted numerators encode affine Mumford coefficients, then all
eight cross-scaled inputs share the common weights `z1*z2` and `w1*w2`. -/
theorem crossScale22_represents_affine
    (i : WeightedInput R)
    (Q1 R1 S1 T1 Q2 R2 S2 T2 : R)
    (hq1 : i.q1 = Q1 * i.z1 ^ 2)
    (hr1 : i.r1 = R1 * i.z1 ^ 4)
    (hs1 : i.s1 = S1 * i.z1 ^ 3 * i.w1)
    (ht1 : i.t1 = T1 * i.z1 ^ 5 * i.w1)
    (hq2 : i.q2 = Q2 * i.z2 ^ 2)
    (hr2 : i.r2 = R2 * i.z2 ^ 4)
    (hs2 : i.s2 = S2 * i.z2 ^ 3 * i.w2)
    (ht2 : i.t2 = T2 * i.z2 ^ 5 * i.w2) :
    (crossScale22 i).q1c = Q1 * (i.z1 * i.z2) ^ 2 ∧
    (crossScale22 i).q2c = Q2 * (i.z1 * i.z2) ^ 2 ∧
    (crossScale22 i).r1c = R1 * (i.z1 * i.z2) ^ 4 ∧
    (crossScale22 i).r2c = R2 * (i.z1 * i.z2) ^ 4 ∧
    (crossScale22 i).s1c = S1 * (i.z1 * i.z2) ^ 3 * (i.w1 * i.w2) ∧
    (crossScale22 i).s2c = S2 * (i.z1 * i.z2) ^ 3 * (i.w1 * i.w2) ∧
    (crossScale22 i).t1c = T1 * (i.z1 * i.z2) ^ 5 * (i.w1 * i.w2) ∧
    (crossScale22 i).t2c = T2 * (i.z1 * i.z2) ^ 5 * (i.w1 * i.w2) := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · change i.q1 * i.z2 ^ 2 = _
    rw [hq1]
    ring
  · change i.q2 * i.z1 ^ 2 = _
    rw [hq2]
    ring
  · change i.r1 * (i.z2 ^ 2) ^ 2 = _
    rw [hr1]
    ring
  · change i.r2 * (i.z1 ^ 2) ^ 2 = _
    rw [hr2]
    ring
  · change i.s1 * (i.z2 ^ 2 * i.z2) * i.w2 = _
    rw [hs1]
    ring
  · change i.s2 * (i.z1 ^ 2 * i.z1) * i.w1 = _
    rw [hs2]
    ring
  · change i.t1 * ((i.z2 ^ 2) ^ 2 * i.z2) * i.w2 = _
    rw [ht1]
    ring
  · change i.t2 * ((i.z1 ^ 2) ^ 2 * i.z1) * i.w1 = _
    rw [ht2]
    ring

/-! The common weighted branch uses the same Z and W factors for all eight
normalized inputs. These identities compute the weights of its B, A, and C
coefficient expressions; the nonzero corollaries cover its B/C branch gates. -/

theorem scale_B (q1 r1 q2 r2 z : R) :
    ((q1 * z ^ 2 - q2 * z ^ 2) *
        ((q2 * z ^ 2) * (r1 * z ^ 4) - (q1 * z ^ 2) * (r2 * z ^ 4)) -
        (r1 * z ^ 4 - r2 * z ^ 4) ^ 2) =
      ((q1 - q2) * (q2 * r1 - q1 * r2) - (r1 - r2) ^ 2) * z ^ 8 := by
  ring

theorem scale_A (q1 r1 s1 t1 q2 r2 s2 t2 z w : R) :
    ((q1 * z ^ 2 - q2 * z ^ 2) *
        ((t1 * z ^ 5 * w - t2 * z ^ 5 * w) * (q2 * z ^ 2) -
         (s1 * z ^ 3 * w - s2 * z ^ 3 * w) * (r2 * z ^ 4)) -
      (r1 * z ^ 4 - r2 * z ^ 4) * (t1 * z ^ 5 * w - t2 * z ^ 5 * w)) =
      ((q1 - q2) * ((t1 - t2) * q2 - (s1 - s2) * r2) -
       (r1 - r2) * (t1 - t2)) * z ^ 9 * w := by
  ring

theorem scale_C (q1 r1 s1 t1 q2 r2 s2 t2 z w : R) :
    ((q1 * z ^ 2 - q2 * z ^ 2) * (t1 * z ^ 5 * w - t2 * z ^ 5 * w) -
      (r1 * z ^ 4 - r2 * z ^ 4) * (s1 * z ^ 3 * w - s2 * z ^ 3 * w)) =
      ((q1 - q2) * (t1 - t2) - (r1 - r2) * (s1 - s2)) * z ^ 7 * w := by
  ring

variable {K : Type*} [Field K]

theorem scale_B_ne_zero_iff (q1 r1 q2 r2 z : K) (hz : z ≠ 0) :
    ((q1 * z ^ 2 - q2 * z ^ 2) *
        ((q2 * z ^ 2) * (r1 * z ^ 4) - (q1 * z ^ 2) * (r2 * z ^ 4)) -
        (r1 * z ^ 4 - r2 * z ^ 4) ^ 2 ≠ 0) ↔
      ((q1 - q2) * (q2 * r1 - q1 * r2) - (r1 - r2) ^ 2 ≠ 0) := by
  rw [scale_B]
  simp [hz]

theorem scale_C_ne_zero_iff (q1 r1 s1 t1 q2 r2 s2 t2 z w : K)
    (hz : z ≠ 0) (hw : w ≠ 0) :
    ((q1 * z ^ 2 - q2 * z ^ 2) * (t1 * z ^ 5 * w - t2 * z ^ 5 * w) -
      (r1 * z ^ 4 - r2 * z ^ 4) * (s1 * z ^ 3 * w - s2 * z ^ 3 * w) ≠ 0) ↔
      ((q1 - q2) * (t1 - t2) - (r1 - r2) * (s1 - s2) ≠ 0) := by
  rw [scale_C]
  simp [hz, hw]

end CrossScale22

#print axioms CrossScale22.crossScale22_represents_affine

#print axioms CrossScale22.scale_B
#print axioms CrossScale22.scale_A
#print axioms CrossScale22.scale_C
#print axioms CrossScale22.scale_B_ne_zero_iff
#print axioms CrossScale22.scale_C_ne_zero_iff
