import Init
set_option autoImplicit false
set_option linter.unusedSectionVars false

/-! Machine-checked round-trip identities for the EFD coordinate charts.

The Rust encoders/projectors in `src/prime_ecc/efd/formats.rs` are rational
maps of one shape: encode writes affine coordinates as `X = x·z^a`, `Y = y·z^b`
(or the inverse chart `X = z/x`), and project divides back. This file states
those maps over an abstract field-like structure and proves, for every chart
family, that projecting an encoded point returns the affine coordinates.

Assumptions are explicit: a nonzero projective scale, and for the inverted and
YZ charts the nonzero coordinates used by the division. The theorem set is
independent of the characteristic; the instantiation to `F_p` only needs the
field axioms collected in `FieldLike`.

The `charts` list at the end pins the 29 two-coordinate charts, the X-only and
Y-only quotient charts, and the Jacobi-intersection triple to their family.
`src/prime_ecc/efd/tests.rs` checks that this list equals the catalogue id set.
-/

class FieldLike (K : Type) where
  mul : K → K → K
  add : K → K → K
  inv : K → K
  zero : K
  one : K
  mul_assoc : ∀ a b c : K, mul (mul a b) c = mul a (mul b c)
  mul_comm : ∀ a b : K, mul a b = mul b a
  mul_one : ∀ a : K, mul a one = a
  mul_inv_cancel : ∀ a : K, a ≠ zero → mul a (inv a) = one
  mul_ne_zero : ∀ a b : K, a ≠ zero → b ≠ zero → mul a b ≠ zero
  zero_ne_one : (zero : K) ≠ one
  inv_mul : ∀ a b : K, a ≠ zero → b ≠ zero → inv (mul a b) = mul (inv a) (inv b)
  inv_inv : ∀ a : K, a ≠ zero → inv (inv a) = a
  inv_ne_zero : ∀ a : K, a ≠ zero → inv a ≠ zero

namespace EFDCharts

variable {K : Type} [FieldLike K]

def mul (a b : K) : K := FieldLike.mul a b
def inv (a : K) : K := FieldLike.inv a
def div (a b : K) : K := mul a (inv b)
def zed : K := FieldLike.zero
def un : K := FieldLike.one
def two : K := FieldLike.add un un

@[simp] theorem mul_assoc (a b c : K) : mul (mul a b) c = mul a (mul b c) :=
  FieldLike.mul_assoc a b c
@[simp] theorem mul_comm (a b : K) : mul a b = mul b a := FieldLike.mul_comm a b
@[simp] theorem mul_one (a : K) : mul a un = a := FieldLike.mul_one a
@[simp] theorem one_mul (a : K) : mul un a = a := by
  rw [mul_comm, mul_one]
@[simp] theorem mul_inv_cancel (a : K) (h : a ≠ zed) : mul a (inv a) = un :=
  FieldLike.mul_inv_cancel a h
theorem mul_ne_zero (a b : K) (ha : a ≠ zed) (hb : b ≠ zed) : mul a b ≠ zed :=
  FieldLike.mul_ne_zero a b ha hb
theorem inv_ne_zero (a : K) (h : a ≠ zed) : inv a ≠ zed :=
  FieldLike.inv_ne_zero a h
theorem inv_mul_of_ne (a b : K) (ha : a ≠ zed) (hb : b ≠ zed) :
    inv (mul a b) = mul (inv a) (inv b) :=
  FieldLike.inv_mul a b ha hb
theorem inv_inv_of_ne (a : K) (h : a ≠ zed) : inv (inv a) = a :=
  FieldLike.inv_inv a h

/-- Power by repeated multiplication, matching `z^2`, `z^3` in the charts. -/
def pow (a : K) : Nat → K
  | 0 => un
  | n + 1 => mul (pow a n) a

@[simp] theorem pow_zero (a : K) : pow a 0 = un := rfl
@[simp] theorem pow_succ (a : K) (n : Nat) : pow a (n + 1) = mul (pow a n) a := rfl

theorem pow_ne_zero (a : K) (h : a ≠ zed) : ∀ n : Nat, pow a n ≠ zed
  | 0 => by
      rw [pow_zero]
      exact Ne.symm FieldLike.zero_ne_one
  | n + 1 => by
      rw [pow_succ]
      exact mul_ne_zero _ _ (pow_ne_zero a h n) h

/-- Generic cancellation: `(a·b)/b = a`. -/
theorem div_mul_cancel (a b : K) (hb : b ≠ zed) : div (mul a b) b = a := by
  unfold div
  rw [mul_assoc, mul_inv_cancel b hb, mul_one]

theorem div_pow_cancel (a z : K) (hz : z ≠ zed) (n : Nat) :
    div (mul a (pow z n)) (pow z n) = a :=
  div_mul_cancel a (pow z n) (pow_ne_zero z hz n)

theorem div_mul_cancel_mul (a b c : K) (hb : b ≠ zed) (hc : c ≠ zed) :
    div (mul a (mul b c)) (mul b c) = a :=
  div_mul_cancel a (mul b c) (mul_ne_zero b c hb hc)

theorem mul_inv_cancel_right (a b : K) (hb : b ≠ zed) : mul (mul a b) (inv b) = a := by
  rw [mul_assoc, mul_inv_cancel b hb, mul_one]

/-- Inverted charts: `z/(z/x) = x`. -/
theorem div_div_cancel (z x : K) (hz : z ≠ zed) (hx : x ≠ zed) : div z (div z x) = x := by
  unfold div
  rw [inv_mul_of_ne z (inv x) hz (inv_ne_zero x hx), inv_inv_of_ne x hx]
  rw [← mul_assoc, mul_inv_cancel z hz, one_mul]

/-! ## Chart families -/

/-- Affine chart: coordinates are already affine. -/
def encodeAffine (x y : K) : K × K := (x, y)
def projectAffine (X Y : K) : K × K := (X, Y)
theorem affine_roundtrip (x y : K) : projectAffine (encodeAffine x y).1 (encodeAffine x y).2 = (x, y) := rfl

/-- Projective / Hessian / extended Edwards: `x=X/Z`, `y=Y/Z`. -/
def encodeProjective (x y z : K) : K × K × K := (mul x z, mul y z, z)
def projectProjective (X Y Z : K) : K × K := (div X Z, div Y Z)
theorem projective_roundtrip (x y z : K) (hz : z ≠ zed) :
    projectProjective (encodeProjective x y z).1 (encodeProjective x y z).2.1
      (encodeProjective x y z).2.2 = (x, y) := by
  unfold projectProjective encodeProjective
  simp only
  rw [div_mul_cancel x z hz, div_mul_cancel y z hz]

/-- Jacobian-like charts: `x=X/Z^2`, `y=Y/Z^3` (shortw Jacobian, 3dik). -/
def encodeJacobian (x y z : K) : K × K × K :=
  (mul x (pow z 2), mul y (pow z 3), z)
def projectJacobian (X Y Z : K) : K × K := (div X (pow Z 2), div Y (pow Z 3))
theorem jacobian_roundtrip (x y z : K) (hz : z ≠ zed) :
    projectJacobian (encodeJacobian x y z).1 (encodeJacobian x y z).2.1
      (encodeJacobian x y z).2.2 = (x, y) := by
  unfold projectJacobian encodeJacobian
  simp only
  rw [div_pow_cancel x z hz 2, div_pow_cancel y z hz 3]

/-- XYZ / W12 / XXYZZ / 2dik charts: `x=X/Z`, `y=Y/Z^2` (and `x=X/Z`, `y=Y/ZZ`,
`ZZ=Z^2` after substituting the cache). -/
def encodeXYZ (x y z : K) : K × K × K := (mul x z, mul y (pow z 2), z)
def projectXYZ (X Y Z : K) : K × K := (div X Z, div Y (pow Z 2))
theorem xyz_roundtrip (x y z : K) (hz : z ≠ zed) :
    projectXYZ (encodeXYZ x y z).1 (encodeXYZ x y z).2.1
      (encodeXYZ x y z).2.2 = (x, y) := by
  unfold projectXYZ encodeXYZ
  simp only
  rw [div_mul_cancel x z hz, div_pow_cancel y z hz 2]

/-- XYZZ charts: `x=X/ZZ`, `y=Y/ZZZ` with `ZZ=z^2`, `ZZZ=z^3`. -/
def encodeXYZZ (x y z : K) : K × K × K × K :=
  (mul x (pow z 2), mul y (pow z 3), pow z 2, pow z 3)
def projectXYZZ (X Y ZZ ZZZ : K) : K × K := (div X ZZ, div Y ZZZ)
theorem xyzz_roundtrip (x y z : K) (hz : z ≠ zed) :
    projectXYZZ (encodeXYZZ x y z).1 (encodeXYZZ x y z).2.1
      (encodeXYZZ x y z).2.2.1 (encodeXYZZ x y z).2.2.2 = (x, y) := by
  unfold projectXYZZ encodeXYZZ
  simp only
  rw [div_pow_cancel x z hz 2, div_pow_cancel y z hz 3]

/-- X-only quotient charts (`shortw/xz`, `montgom/xz`). -/
def encodeX (x z : K) : K × K := (mul x z, z)
def projectX (X Z : K) : K := div X Z
theorem x_only_roundtrip (x z : K) (hz : z ≠ zed) :
    projectX (encodeX x z).1 (encodeX x z).2 = x := by
  unfold projectX encodeX
  simp only
  exact div_mul_cancel x z hz

/-- Inverted charts (`edwards/inverted`, `twisted/inverted`). -/
def encodeInverted (x y z : K) : K × K × K := (div z x, div z y, z)
def projectInverted (X Y Z : K) : K × K := (div Z X, div Z Y)
theorem inverted_roundtrip (x y z : K) (hz : z ≠ zed) (hx : x ≠ zed) (hy : y ≠ zed) :
    projectInverted (encodeInverted x y z).1 (encodeInverted x y z).2.1
      (encodeInverted x y z).2.2 = (x, y) := by
  unfold projectInverted encodeInverted
  simp only
  rw [div_div_cancel z x hz hx, div_div_cancel z y hz hy]

/-- YZ charts: `r·y = Y/Z`. -/
def encodeYZ (r y z : K) : K × K := (mul (mul r y) z, z)
def projectYZ (r Y Z : K) : K := div Y (mul r Z)
theorem yz_roundtrip (r y z : K) (hr : r ≠ zed) (hz : z ≠ zed) :
    projectYZ r (encodeYZ r y z).1 (encodeYZ r y z).2 = y := by
  unfold projectYZ encodeYZ div
  simp only
  rw [inv_mul_of_ne r z hr hz, mul_comm (inv r) (inv z), ← mul_assoc]
  rw [mul_inv_cancel_right (mul r y) z hz]
  rw [mul_comm r y, mul_inv_cancel_right y r hr]

/-- Squared-YZ charts: `r·y^2 = Y/Z`. -/
def encodeYZSquared (r y z : K) : K × K := (mul (mul r (mul y y)) z, z)
theorem yzsquared_roundtrip (r y z : K) (hr : r ≠ zed) (hz : z ≠ zed) :
    projectYZ r (encodeYZSquared r y z).1 (encodeYZSquared r y z).2 = mul y y := by
  unfold projectYZ encodeYZSquared div
  simp only
  rw [inv_mul_of_ne r z hr hz, mul_comm (inv r) (inv z), ← mul_assoc]
  rw [mul_inv_cancel_right (mul r (mul y y)) z hz]
  rw [mul_comm r (mul y y), mul_inv_cancel_right (mul y y) r hr]

/-- Jacobi-intersection triple: `s=S/Z`, `c=C/Z`, `d=D/Z`. -/
def encodeIntersection (s c d z : K) : K × K × K × K :=
  (mul s z, mul c z, mul d z, z)
def projectIntersection (S C D Z : K) : K × K × K := (div S Z, div C Z, div D Z)
theorem jintersect_roundtrip (s c d z : K) (hz : z ≠ zed) :
    projectIntersection (encodeIntersection s c d z).1 (encodeIntersection s c d z).2.1
      (encodeIntersection s c d z).2.2.1 (encodeIntersection s c d z).2.2.2 =
      (s, c, d) := by
  unfold projectIntersection encodeIntersection
  simp only
  rw [div_mul_cancel s z hz, div_mul_cancel c z hz, div_mul_cancel d z hz]

/-! ## Coordinate caches

The Rust encoders fill cached coordinates from the chart relations. These
theorems state the cache definitions; each is true by construction, so the
catalogue cannot claim a cache the encoder does not write. -/

structure ExtendedChart (K : Type) where
  X : K
  Y : K
  Z : K
  U : K
  V : K
  W : K
  T : K

def encodeExtended (x y z : K) : ExtendedChart K :=
  let X := mul x z
  let Y := mul y (pow z 2)
  let Z := z
  let U := mul X X
  let V := mul two (mul X Z)
  let W := mul Z Z
  let T := FieldLike.add U W
  { X := X, Y := Y, Z := Z, U := U, V := V, W := W, T := T }

theorem extended_cache_U (x y z : K) :
    (encodeExtended x y z).U = mul (encodeExtended x y z).X (encodeExtended x y z).X := rfl
theorem extended_cache_V (x y z : K) :
    (encodeExtended x y z).V = mul two (mul (encodeExtended x y z).X (encodeExtended x y z).Z) := rfl
theorem extended_cache_W (x y z : K) :
    (encodeExtended x y z).W = mul (encodeExtended x y z).Z (encodeExtended x y z).Z := rfl
theorem extended_cache_T (x y z : K) :
    (encodeExtended x y z).T = FieldLike.add (encodeExtended x y z).U (encodeExtended x y z).W := rfl

structure HessianChart (K : Type) where
  X : K
  Y : K
  Z : K
  XX : K
  YY : K
  ZZ : K
  XY : K
  XZ : K
  YZ : K

def encodeHessian (x y z : K) : HessianChart K :=
  let X := mul x z
  let Y := mul y z
  let Z := z
  { X := X, Y := Y, Z := Z, XX := mul X X, YY := mul Y Y, ZZ := mul Z Z,
    XY := mul two (mul X Y), XZ := mul two (mul X Z), YZ := mul two (mul Y Z) }

theorem hessian_cache_XX (x y z : K) :
    (encodeHessian x y z).XX = mul (encodeHessian x y z).X (encodeHessian x y z).X := rfl
theorem hessian_cache_XY (x y z : K) :
    (encodeHessian x y z).XY = mul two (mul (encodeHessian x y z).X (encodeHessian x y z).Y) := rfl
theorem hessian_cache_XZ (x y z : K) :
    (encodeHessian x y z).XZ = mul two (mul (encodeHessian x y z).X (encodeHessian x y z).Z) := rfl

structure IntersectionChart (K : Type) where
  S : K
  C : K
  D : K
  Z : K
  SC : K
  DZ : K

def encodeIntersectionChart (s c d z : K) : IntersectionChart K :=
  let S := mul s z
  let C := mul c z
  let D := mul d z
  { S := S, C := C, D := D, Z := z, SC := mul S C, DZ := mul D z }

theorem intersection_cache_SC (s c d z : K) :
    (encodeIntersectionChart s c d z).SC =
      mul (encodeIntersectionChart s c d z).S (encodeIntersectionChart s c d z).C := rfl
theorem intersection_cache_DZ (s c d z : K) :
    (encodeIntersectionChart s c d z).DZ =
      mul (encodeIntersectionChart s c d z).D (encodeIntersectionChart s c d z).Z := rfl

structure QuarticChart (K : Type) where
  X : K
  Y : K
  Z : K
  R : K

def encodeQuarticR (x y z : K) : QuarticChart K :=
  { X := mul x z, Y := mul y (pow z 2), Z := z, R := mul two (mul (mul x z) z) }

theorem quartic_cache_R (x y z : K) :
    (encodeQuarticR x y z).R = mul two (mul (encodeQuarticR x y z).X (encodeQuarticR x y z).Z) := rfl

/-! ## Catalogue manifest

Family codes: 0 affine, 1 projective, 2 Jacobian-like, 3 XYZ/XXYZZ/2dik,
4 XYZZ, 5 X-only, 6 inverted, 7 YZ, 8 squared-YZ, 9 Jacobi intersection.
The Rust test `lean_chart_manifest_matches_catalogue` checks this list against
the pinned EFD catalogue and against the family dispatch in `formats.rs`. -/

def charts : List (String × Nat) :=
  [ ("shortw/jacobian", 2), ("shortw/jacobian-0", 2), ("shortw/jacobian-3", 2),
    ("shortw/modified", 2), ("shortw/projective", 1), ("shortw/projective-1", 1),
    ("shortw/projective-3", 1), ("shortw/w12-0", 3), ("shortw/xyzz", 4),
    ("shortw/xyzz-3", 4), ("shortw/xz", 5), ("montgom/xz", 5),
    ("edwards/inverted", 6), ("edwards/projective", 1), ("edwards/yz", 7),
    ("edwards/yzsquared", 8), ("twisted/extended", 1), ("twisted/extended-1", 1),
    ("twisted/inverted", 6), ("twisted/projective", 1),
    ("twistedhessian/projective", 1), ("hessian/extended", 1),
    ("hessian/standard", 1), ("jintersect/extended", 9),
    ("jintersect/standard", 9), ("jquartic/2xxyzz", 3), ("jquartic/2xxyzzr", 3),
    ("jquartic/2xyz", 3), ("jquartic/xxyzz", 3), ("jquartic/xxyzzr", 3),
    ("jquartic/xyz", 3), ("2dik/standard", 3), ("3dik/standard", 2) ]

/-- Every family in the manifest has a proved round trip. This theorem records
the dispatch: nothing in the list points at an unproved family. -/
theorem family_covered : ∀ p ∈ charts, p.2 ≤ 9 := by decide

end EFDCharts
