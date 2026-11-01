import Add11
set_option autoImplicit false
namespace WeightedAdd11
open Polynomial
variable {K : Type*} [Field K]
structure Input (K : Type*) where
  x : K
  y : K
  z : K
  w : K
structure Output (K : Type*) where
  q : K
  r : K
  s : K
  t : K
  z : K
  w : K

def nx (a : Input K) : K := a.x / a.z^2
def ny (a : Input K) : K := a.y / (a.z^5*a.w)
def crossA (a b : Input K) : K := a.x * b.z^2
def crossB (a b : Input K) : K := b.x * a.z^2
def crossY (a b : Input K) : K := a.y * (b.z^5*b.w)
def output (a b : Input K) : Output K :=
  let az2 := a.z*a.z
  let bz2 := b.z*b.z
  let axbz := a.x*bz2
  let bxaz := b.x*az2
  let ay := a.y*(b.z^5*b.w)
  let yb :=  b.y*(a.z^5*a.w)
  ⟨-(axbz+bxaz), axbz*bxaz, ay-yb, axbz*yb-bxaz*ay,
    a.z*b.z, (axbz-bxaz)*(a.w*b.w)⟩

theorem cross_difference (a b : Input K) (haz : a.z ≠ 0) (hbz : b.z ≠ 0) :
    crossA a b-crossB a b = (nx a-nx b)*(a.z^2*b.z^2) := by
  unfold crossA crossB nx
  field_simp
  <;> ring

theorem normalized_distinct_cross (a b : Input K) (haz : a.z ≠ 0) (hbz : b.z ≠ 0)
    (hx : nx a ≠ nx b) : crossA a b ≠ crossB a b := by
  apply sub_ne_zero.mp
  rw [cross_difference a b haz hbz]
  exact mul_ne_zero (sub_ne_zero.mpr hx) (mul_ne_zero (pow_ne_zero 2 haz) (pow_ne_zero 2 hbz))

theorem normalized_coefficients (a b : Input K)
    (haz : a.z ≠ 0) (haw : a.w ≠ 0) (hbz : b.z ≠ 0) (hbw : b.w ≠ 0)
    (hx : nx a ≠ nx b) :
    let c := output a b
    c.z ≠ 0 ∧ c.w ≠ 0 ∧
    c.q/c.z^2 = -(nx a+nx b) ∧
    c.r/c.z^4 = nx a*nx b ∧
    c.s/(c.z^3*c.w) = (ny a-ny b)/(nx a-nx b) ∧
    c.t/(c.z^5*c.w) = (nx a*ny b-nx b*ny a)/(nx a-nx b) := by
  have hd : a.x*b.z^2-b.x*a.z^2 ≠ 0 :=
    sub_ne_zero.mpr (normalized_distinct_cross a b haz hbz hx)
  have hz : (output a b).z ≠ 0 := mul_ne_zero haz hbz
  have hw : (output a b).w ≠ 0 := by
    simpa [output,pow_two] using mul_ne_zero hd (mul_ne_zero haw hbw)
  refine ⟨hz,hw,?_,?_,?_,?_⟩
  all_goals
    dsimp [output,nx,ny]
    field_simp
    <;> ring

theorem preserves_mumford (a b : Input K) (f : K[X])
    (haz : a.z ≠ 0) (haw : a.w ≠ 0) (hbz : b.z ≠ 0) (hbw : b.w ≠ 0)
    (hx : nx a ≠ nx b)
    (ha : (X-C (nx a)) ∣ f-(C (ny a))^2)
    (hb : (X-C (nx b)) ∣ f-(C (ny b))^2) :
    let c := output a b
    c.z ≠ 0 ∧ c.w ≠ 0 ∧
    (X^2+C (c.q/c.z^2)*X+C (c.r/c.z^4)) ∣
      f-(C (c.s/(c.z^3*c.w))*X+C (c.t/(c.z^5*c.w)))^2 := by
  obtain ⟨hz,hw,hq,hr,hs,ht⟩ := normalized_coefficients a b haz haw hbz hbw hx
  refine ⟨hz,hw,?_⟩
  rw [hq,hr,hs,ht]
  exact (Add11Distinct.distinct_preserves_mumford f (nx a) (ny a) (nx b) (ny b) hx ha hb).2

theorem distinct_guard [DecidableEq K] (a b : Input K)
    (haz : a.z ≠ 0) (hbz : b.z ≠ 0) (hx : nx a ≠ nx b) :
    Add11Distinct.classify (crossA a b) (crossY a b) (crossB a b) (crossY b a) = .distinct := by
  exact Add11Distinct.distinct_guard _ _ _ _ (normalized_distinct_cross a b haz hbz hx)

end WeightedAdd11
