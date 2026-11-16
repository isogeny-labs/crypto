import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.FieldTheory.IsAlgClosed.Basic
import Mathlib.Tactic

namespace TwoTorsionRankAudit
open Polynomial WeierstrassCurve.Affine
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual subgroup of Mathlib points killed by multiplication by2. -/
def torsion (W : WeierstrassCurve.Affine K) : AddSubgroup W.Point where
  carrier := {P | 2 • P=0}
  zero_mem' := by simp
  add_mem' := by intro P Q hP hQ; simp only [Set.mem_ofPred_eq] at *; rw [nsmul_add,hP,hQ,zero_add]
  neg_mem' := by intro P hP; simp only [Set.mem_ofPred_eq] at *; rw [smul_neg,hP,neg_zero]

abbrev TwoTorsion (W : WeierstrassCurve.Affine K) := ↥(torsion W)
def Root (W : WeierstrassCurve.Affine K) := {x : K // W.twoTorsionPolynomial.toPoly.IsRoot x}

def AffineTwo (W : WeierstrassCurve.Affine K) :=
  {xy : K×K // ∃ h : W.Nonsingular xy.1 xy.2,
    2 • (Point.some _ _ h)=0}

def affineToRoot (W : WeierstrassCurve.Affine K) (A : AffineTwo W) : Root W :=
  ⟨A.val.1,Point.isRoot_twoTorsionPolynomial_of_add_self A.property.choose
    (by simpa only [two_nsmul] using A.property.choose_spec)⟩

theorem affineToRoot_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (affineToRoot W) := by
  intro A B he
  have hx : A.val.1=B.val.1 := congrArg Subtype.val he
  have hA := A.property.choose
  have hB := B.property.choose
  have hkill : Point.some _ _ hB= -(Point.some _ _ hB) :=
    add_eq_zero_iff_eq_neg.mp (by simpa only [two_nsmul] using B.property.choose_spec)
  have hxy : Point.some _ _ hA=Point.some _ _ hB ∨
      Point.some _ _ hA= -(Point.some _ _ hB) :=
    (Point.X_eq_iff (h₁:=hA) (h₂:=hB)).mp hx
  apply Subtype.ext
  apply Prod.ext hx
  rcases hxy with hh | hh
  · have hpair : A.val.1=B.val.1 ∧ A.val.2=B.val.2 := by
      simpa only [Point.some.injEq] using hh
    exact hpair.2
  · rw [← hkill] at hh
    have hpair : A.val.1=B.val.1 ∧ A.val.2=B.val.2 := by
      simpa only [Point.some.injEq] using hh
    exact hpair.2

theorem affineToRoot_surjective [NeZero (2 : K)] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) : Function.Surjective (affineToRoot W) := by
  intro x
  obtain ⟨y,hy,hP⟩ :=
    (Point.isRoot_twoTorsionPolynomial_iff (W:=W) inferInstance hΔ x.val).mp x.property
  refine ⟨⟨(x.val,y),hy,by simpa only [two_nsmul] using hP⟩,?_⟩
  apply Subtype.ext
  rfl

def affineRootEquiv [NeZero (2 : K)] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    AffineTwo W ≃ Root W :=
  Equiv.ofBijective (affineToRoot W) ⟨affineToRoot_injective W,affineToRoot_surjective W hΔ⟩

/-- Actual group points, identity and distinct cubic roots; no rank premise. -/
def pointRootEquiv [NeZero (2 : K)] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    TwoTorsion W ≃ Option (Root W) := by
  have hz : 2 • (Point.zero : W.Point)=0 := by
    change 2 • (0 : W.Point)=0
    exact smul_zero _
  let e : TwoTorsion W ≃ Option (AffineTwo W) :=
    W.nonsingularPointEquivSubtype (p:=fun P => 2 • P=0) hz
  exact e.trans ((affineRootEquiv W hΔ).optionCongr)

#print axioms pointRootEquiv
end
end TwoTorsionRankAudit
