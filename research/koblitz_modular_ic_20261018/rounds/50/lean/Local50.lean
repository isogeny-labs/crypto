import Mathlib.Tactic.Ring
namespace Local50
/-- Scaling the integral quadratic coordinate keeps a monic quadratic witness. -/
theorem scaled_quadratic_identity {R : Type*} [CommRing R] (b v e : R) :
    (b*v)^2 + b*(b*v) + e*b^2 = b^2*(v^2+v+e) := by ring

/-- Constructive inclusion of the intersection in (a,b*c), given a full
Bezout certificate for (a,b,c). -/
theorem bezout_intersection_reconstruction {R : Type*} [CommRing R]
    (a b c x y z f l0 l1 r0 r1 : R)
    (hu : a*x+b*y+c*z=1) (hl : f=a*l0+b*l1) (hr : f=a*r0+c*r1) :
    f=a*(f*x+b*y*r0+c*z*l0)+b*c*(y*r1+z*l1) := by
  have hzU : 1-(a*x+b*y+c*z)=0 := sub_eq_zero.mpr hu.symm
  have hzL : f-(a*l0+b*l1)=0 := sub_eq_zero.mpr hl
  have hzR : f-(a*r0+c*r1)=0 := sub_eq_zero.mpr hr
  apply sub_eq_zero.mp
  calc
    _ = f*(1-(a*x+b*y+c*z)) + b*y*(f-(a*r0+c*r1)) +
        c*z*(f-(a*l0+b*l1)) := by ring
    _ = 0 := by rw [hzU, hzR, hzL]; ring

/-- The seventh power separates every smaller exponent from the eighth. -/
theorem exact_eighth_power_nilpotency {R : Type*} [MonoidWithZero R]
    (h : R) (h8 : h^8=0) (h7 : h^7≠0) :
    h^8=0 ∧ ∀ k : ℕ, k<8 → h^k≠0 := by
  refine ⟨h8, ?_⟩
  intro k hk hz
  have hk7 : k≤7 := Nat.le_of_lt_succ hk
  apply h7
  calc
    h^7 = h^(7-k+k) := by rw [Nat.sub_add_cancel hk7]
    _ = h^(7-k)*h^k := pow_add h (7-k) k
    _ = 0 := by rw [hz, mul_zero]
end Local50
