import Mathlib.Tactic.Ring
import Mathlib.Tactic.Linarith
import Mathlib.Algebra.CharP.Two
import Mathlib.Algebra.CharP.Basic
import Mathlib.Data.ZMod.Basic
import Mathlib.LinearAlgebra.Dual.Basis
import Mathlib.LinearAlgebra.Basis.Basic

/-! Algebraic content of Proposition A (trace relation, m = 2 binary Semaev system),
abstract version: `Tr : R →+ ZMod 2` with `Tr (a*a) = Tr a` stands for the absolute trace. -/

namespace PropA

/-- Semaev's third summation polynomial in characteristic 2 (`β` is the curve constant `b`). -/
def S3 {R : Type*} [CommRing R] (β a b c : R) : R :=
  (a*b + a*c + b*c)^2 + a*b*c + β

/-- Kosters–Yeo trace relation. -/
theorem traceRelation {R : Type*} [CommRing R] [CharP R 2] (Tr : R →+ ZMod 2)
    (hTr : ∀ a, Tr (a * a) = Tr a) (β x3 w : R) (hw : x3 * w = 1) (X1 X2 : R) :
    Tr (S3 β X1 X2 x3 * w^2) = Tr X1 + Tr X2 + Tr (β * w^2) := by
  have h2 : (2:R) = 0 := CharTwo.two_eq_zero
  have key : S3 β X1 X2 x3 * w^2
      = (X1*X2*w * (X1*X2*w) + X1*X2*w) + (X1 + X2) * (X1 + X2) + β * w^2 := by
    unfold S3
    have e : (X1*X2 + X1*x3 + X2*x3)^2 * w^2 = (X1*X2*w + (X1+X2)*(x3*w))^2 := by ring
    have e2 : X1*X2*x3 * w^2 = X1*X2*w*(x3*w) := by ring
    rw [add_mul, add_mul, e, e2, hw]
    have e3 : (X1 * X2 * w + (X1 + X2) * 1) ^ 2 + X1 * X2 * w * 1 + β * w ^ 2
        = X1 * X2 * w * (X1 * X2 * w) + X1 * X2 * w + (X1 + X2) * (X1 + X2) + β * w ^ 2
          + 2 * (X1 * X2 * w * (X1 + X2)) := by ring
    rw [e3, h2]; ring
  have hs : Tr ((X1 + X2) * (X1 + X2)) = Tr X1 + Tr X2 := by rw [hTr, map_add]
  have hz : Tr (X1*X2*w * (X1*X2*w)) + Tr (X1*X2*w) = 0 := by
    rw [hTr]
    exact CharTwo.add_self_eq_zero _
  rw [key]
  simp only [map_add]
  rw [hs, hz, zero_add]

/-- Additive maps into `ZMod 2` are compatible with the `ZMod 2`-scalar action. -/
theorem map_smul2 {R : Type*} [AddCommGroup R] [Module (ZMod 2) R] (f : R →+ ZMod 2)
    (y : ZMod 2) (v : R) : f (y • v) = y * f v := by
  obtain ⟨n, rfl⟩ : ∃ n : ℕ, (n : ZMod 2) = y := ⟨y.val, ZMod.natCast_zmod_val y⟩
  rw [Nat.cast_smul_eq_nsmul, map_nsmul, nsmul_eq_mul]

/-- An additive map into `ZMod 2` is a combination of the basis values with the coordinate
functionals of the basis. -/
theorem coordinateCombination {R : Type*} [CommRing R] [Module (ZMod 2) R]
    {ι : Type*} [Fintype ι] (bR : Module.Basis ι (ZMod 2) R) (μ : R →+ ZMod 2) (r : R) :
    μ r = ∑ i, μ (bR i) * bR.repr r i := by
  conv_lhs => rw [← bR.sum_repr r]
  rw [map_sum]
  refine Finset.sum_congr rfl fun i _ => ?_
  rw [map_smul2, mul_comm]

/-- The functional `r ↦ Tr (r * w^2)` is an F2-combination of the coordinate functionals;
instantiated at `S3`, the "descended equations" are the coordinates `bR.repr (S3 …) i`. -/
theorem traceFunctional_in_span {R : Type*} [CommRing R] [Module (ZMod 2) R]
    {ι : Type*} [Fintype ι] (bR : Module.Basis ι (ZMod 2) R) (Tr : R →+ ZMod 2) (w s : R) :
    Tr (s * w^2) = ∑ i, Tr (bR i * w^2) * bR.repr s i :=
  coordinateCombination bR (Tr.comp (AddMonoidHom.mulRight (w^2))) s

/-- Combined statement: for `S3`, `Tr X1 + Tr X2 + Tr (β w²)` is an F2-combination of the
coordinates of `S3 β X1 X2 x3`. -/
theorem traceRelation_in_span {R : Type*} [CommRing R] [CharP R 2] [Module (ZMod 2) R]
    {ι : Type*} [Fintype ι] (bR : Module.Basis ι (ZMod 2) R) (Tr : R →+ ZMod 2)
    (hTr : ∀ a, Tr (a * a) = Tr a) (β x3 w : R) (hw : x3 * w = 1) (X1 X2 : R) :
    Tr X1 + Tr X2 + Tr (β * w^2)
      = ∑ i, Tr (bR i * w^2) * bR.repr (S3 β X1 X2 x3) i := by
  rw [← traceFunctional_in_span, traceRelation Tr hTr β x3 w hw]

/-- Additivity of `Tr` over F2-bit combinations. -/
theorem affineInBits {R : Type*} [CommRing R] [Module (ZMod 2) R] {κ : Type*} [Fintype κ]
    (Tr : R →+ ZMod 2) (v : κ → R) (y : κ → ZMod 2) :
    Tr (∑ j, y j • v j) = ∑ j, y j * Tr (v j) := by
  rw [map_sum]
  refine Finset.sum_congr rfl fun j _ => ?_
  exact map_smul2 Tr (y j) (v j)

/-- `L` in the bits of `X1 = ∑ y1 j • v j`, `X2 = ∑ y2 j • v j`. -/
theorem L_affine {R : Type*} [CommRing R] [Module (ZMod 2) R] {κ : Type*} [Fintype κ]
    (Tr : R →+ ZMod 2) (v : κ → R) (c : ZMod 2) (y1 y2 : κ → ZMod 2) :
    Tr (∑ j, y1 j • v j) + Tr (∑ j, y2 j • v j) + c
      = ∑ j, Tr (v j) * (y1 j + y2 j) + c := by
  rw [affineInBits, affineInBits, ← Finset.sum_add_distrib]
  congr 1
  refine Finset.sum_congr rfl fun j _ => ?_
  ring

/-- `L` is constant in the bits exactly when `Tr` vanishes on the spanning family. -/
theorem L_const_iff {R : Type*} [CommRing R] [Module (ZMod 2) R] {κ : Type*} [Fintype κ]
    [DecidableEq κ] (Tr : R →+ ZMod 2) (v : κ → R) (c : ZMod 2) :
    (∀ y1 y2 : κ → ZMod 2, Tr (∑ j, y1 j • v j) + Tr (∑ j, y2 j • v j) + c = c)
      ↔ ∀ j, Tr (v j) = 0 := by
  constructor
  · intro h j
    have := h (Pi.single j 1) 0
    rw [L_affine] at this
    simpa [Finset.sum_add_distrib, Pi.single_apply] using this
  · intro h y1 y2
    rw [L_affine]
    simp [h]

/-! Sanity instance: `R = ZMod 2`, `Tr = id`. -/
theorem hTr_zmod2 : ∀ a : ZMod 2, (AddMonoidHom.id (ZMod 2)) (a * a) = (AddMonoidHom.id (ZMod 2)) a := by
  decide

example (β x3 w : ZMod 2) (hw : x3 * w = 1) (X1 X2 : ZMod 2) :
    (AddMonoidHom.id (ZMod 2)) (S3 β X1 X2 x3 * w^2)
      = X1 + X2 + β * w^2 :=
  traceRelation (AddMonoidHom.id (ZMod 2)) hTr_zmod2 β x3 w hw X1 X2

noncomputable def bs : Module.Basis Unit (ZMod 2) (ZMod 2) := Module.Basis.singleton Unit (ZMod 2)

theorem sanity_inCoordinates (β x3 w : ZMod 2) (hw : x3 * w = 1) (X1 X2 : ZMod 2) :
    X1 + X2 + β * w^2
      = ∑ i : Unit, (bs i * w^2) * bs.repr (S3 β X1 X2 x3) i := by
  have := traceRelation_in_span bs (AddMonoidHom.id (ZMod 2)) hTr_zmod2 β x3 w hw X1 X2
  simpa using this

#print axioms traceRelation
#print axioms map_smul2
#print axioms coordinateCombination
#print axioms traceFunctional_in_span
#print axioms traceRelation_in_span
#print axioms affineInBits
#print axioms L_affine
#print axioms L_const_iff
#print axioms hTr_zmod2
#print axioms sanity_inCoordinates

end PropA
