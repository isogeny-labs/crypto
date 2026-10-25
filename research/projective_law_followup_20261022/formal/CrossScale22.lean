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

end CrossScale22

#print axioms CrossScale22.crossScale22_represents_affine
