import CrossScale22
import Common22SupportHead

/-!
The straight-line prefix and two early exits of the Rust
`add22_weighted_common` branch, expressed over a field.  The native source
checker pins every Rust statement represented here, including guard order.
This model stops before output construction and does not define Rust machine
semantics or a Jacobian class map.
-/

set_option autoImplicit false

namespace WeightedSource22

variable {K : Type*} [Field K] [DecidableEq K]

def coefficients (c : CrossScale22.CrossInput K) : K × K × K :=
  let dq := c.q1c - c.q2c
  let dr := c.r1c - c.r2c
  let ds := c.s1c - c.s2c
  let dt := c.t1c - c.t2c
  let bcoef := dq * (c.q2c * c.r1c - c.q1c * c.r2c) - dr ^ 2
  let acoef := dq * (dt * c.q2c - ds * c.r2c) - dr * dt
  let ccoef := dq * dt - dr * ds
  (bcoef, acoef, ccoef)

def classifyPrefix (i : CrossScale22.WeightedInput K) : Option (K × K × K) :=
  let (bcoef, acoef, ccoef) := coefficients (CrossScale22.crossScale22 i)
  if bcoef = 0 then none
  else if ccoef = 0 then none
  else some (bcoef, acoef, ccoef)

theorem classifyPrefix_some_iff (i : CrossScale22.WeightedInput K) :
    (classifyPrefix i).isSome ↔
      (coefficients (CrossScale22.crossScale22 i)).1 ≠ 0 ∧
      (coefficients (CrossScale22.crossScale22 i)).2.2 ≠ 0 := by
  unfold classifyPrefix
  rcases h : coefficients (CrossScale22.crossScale22 i) with ⟨b, a, c⟩
  by_cases hb : b = 0
  · simp [hb]
  · by_cases hc : c = 0
    · simp [hb, hc]
    · simp [hb, hc]

theorem coefficients_eq_affine_scaled
    (i : CrossScale22.WeightedInput K)
    (q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hq1 : i.q1 = q1 * i.z1 ^ 2)
    (hr1 : i.r1 = r1 * i.z1 ^ 4)
    (hs1 : i.s1 = s1 * i.z1 ^ 3 * i.w1)
    (ht1 : i.t1 = t1 * i.z1 ^ 5 * i.w1)
    (hq2 : i.q2 = q2 * i.z2 ^ 2)
    (hr2 : i.r2 = r2 * i.z2 ^ 4)
    (hs2 : i.s2 = s2 * i.z2 ^ 3 * i.w2)
    (ht2 : i.t2 = t2 * i.z2 ^ 5 * i.w2) :
    coefficients (CrossScale22.crossScale22 i) =
      ((Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 *
          (i.z1 * i.z2) ^ 8,
       (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).2.1 *
          (i.z1 * i.z2) ^ 9 * (i.w1 * i.w2),
       (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).2.2.1 *
          (i.z1 * i.z2) ^ 7 * (i.w1 * i.w2)) := by
  have h := CrossScale22.crossScale22_represents_affine i
    q1 r1 s1 t1 q2 r2 s2 t2 hq1 hr1 hs1 ht1 hq2 hr2 hs2 ht2
  rcases h with ⟨hq1c, hq2c, hr1c, hr2c, hs1c, hs2c, ht1c, ht2c⟩
  unfold coefficients
  rw [hq1c, hq2c, hr1c, hr2c, hs1c, hs2c, ht1c, ht2c]
  simp only [Common22SupportHead.coefficientData]
  apply Prod.ext
  · exact CrossScale22.scale_B q1 r1 q2 r2 (i.z1 * i.z2)
  · apply Prod.ext
    · exact CrossScale22.scale_A q1 r1 s1 t1 q2 r2 s2 t2
        (i.z1 * i.z2) (i.w1 * i.w2)
    · exact CrossScale22.scale_C q1 r1 s1 t1 q2 r2 s2 t2
        (i.z1 * i.z2) (i.w1 * i.w2)

theorem classifyPrefix_some_iff_affine
    (i : CrossScale22.WeightedInput K)
    (q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hq1 : i.q1 = q1 * i.z1 ^ 2)
    (hr1 : i.r1 = r1 * i.z1 ^ 4)
    (hs1 : i.s1 = s1 * i.z1 ^ 3 * i.w1)
    (ht1 : i.t1 = t1 * i.z1 ^ 5 * i.w1)
    (hq2 : i.q2 = q2 * i.z2 ^ 2)
    (hr2 : i.r2 = r2 * i.z2 ^ 4)
    (hs2 : i.s2 = s2 * i.z2 ^ 3 * i.w2)
    (ht2 : i.t2 = t2 * i.z2 ^ 5 * i.w2)
    (hz1 : i.z1 ≠ 0) (hz2 : i.z2 ≠ 0)
    (hw1 : i.w1 ≠ 0) (hw2 : i.w2 ≠ 0) :
    (classifyPrefix i).isSome ↔
      (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0 ∧
      (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).2.2.1 ≠ 0 := by
  rw [classifyPrefix_some_iff,
    coefficients_eq_affine_scaled i q1 r1 s1 t1 q2 r2 s2 t2
      hq1 hr1 hs1 ht1 hq2 hr2 hs2 ht2]
  have hz : i.z1 * i.z2 ≠ 0 := mul_ne_zero hz1 hz2
  have hw : i.w1 * i.w2 ≠ 0 := mul_ne_zero hw1 hw2
  simp [hz, hw]

end WeightedSource22

#print axioms WeightedSource22.classifyPrefix_some_iff
#print axioms WeightedSource22.coefficients_eq_affine_scaled
#print axioms WeightedSource22.classifyPrefix_some_iff_affine
