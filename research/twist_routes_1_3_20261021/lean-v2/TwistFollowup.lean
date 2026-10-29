import TwistRoutes
import Mathlib.FieldTheory.Finite.Basic
import Mathlib.Algebra.Polynomial.SpecificDegree
import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.NormNum

/-!
Concrete p=59 quadratic field and actual Frobenius point endomorphism.
The point-subgroup/row-module bridge is still separate. No custom axioms,
sorry, native_decide, known target logarithms or NIST DLP are used.
-/
namespace TwistFollowup
open Polynomial
instance : Fact (Nat.Prime 59) := ⟨by decide⟩
abbrev F := ZMod 59
noncomputable def quadratic : F[X] := X^2-C 2

theorem two_nonsquare : ∀ x : F, x^2 ≠ 2 := by decide
theorem two_euler : (2:F)^29 = -1 := by decide

theorem quadratic_irreducible : Irreducible quadratic := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · simp [quadratic]
  · intro x hx
    apply two_nonsquare x
    simpa [Polynomial.IsRoot, quadratic, sub_eq_zero] using hx

instance : Fact (Irreducible quadratic) := ⟨quadratic_irreducible⟩
abbrev K := AdjoinRoot quadratic
noncomputable def w : K := AdjoinRoot.root quadratic
noncomputable def i : F →+* K := algebraMap F K
noncomputable def fieldFrob : K →ₐ[F] K := FiniteField.frobeniusAlgHom F K

theorem w_square : w^2 = 2 := by
  have h := AdjoinRoot.eval₂_root quadratic
  rw [quadratic, Polynomial.eval₂_sub, Polynomial.eval₂_pow,
    Polynomial.eval₂_X, Polynomial.eval₂_C, map_ofNat] at h
  exact sub_eq_zero.mp h

theorem fieldFrob_i (a:F) : fieldFrob (i a) = i a := fieldFrob.commutes a

theorem fieldFrob_w : fieldFrob w = -w := by
  have he : (2:K)^29 = -1 := by
    have h := congrArg i two_euler
    simpa only [map_pow, map_ofNat, map_neg, map_one] using h
  change w^59 = -w
  calc
    w^59 = (w^2)^29*w := by ring
    _ = (2:K)^29*w := by rw [w_square]
    _ = -w := by rw [he]; ring

noncomputable def pair (a b:F) : K := i a + i b*w
theorem pair_add (a b c d:F) : pair (a+c) (b+d) = pair a b + pair c d := by
  simp only [pair, map_add]; ring
theorem pair_mul (a b c d:F) :
    pair (a*c+2*b*d) (a*d+b*c) = pair a b * pair c d := by
  simp only [pair, map_add, map_mul, map_ofNat]
  calc
    i a*i c+2*i b*i d+(i a*i d+i b*i c)*w =
      i a*i c+i b*i d*w^2+(i a*i d+i b*i c)*w := by rw [w_square]; ring
    _ = (i a+i b*w)*(i c+i d*w) := by ring
theorem pair_frobenius (a b:F) : fieldFrob (pair a b) = pair a (-b) := by
  simp only [pair, map_add, map_mul, fieldFrob_i, fieldFrob_w, map_neg]; ring

noncomputable def curve : WeierstrassCurve.Affine F :=
  { a₁:=0, a₂:=0, a₃:=0, a₄:=1, a₆:=13 }
noncomputable def pointFrob : (curve.baseChange K).Point →+ (curve.baseChange K).Point :=
  WeierstrassCurve.Affine.Point.map (W':=curve) (S:=F) fieldFrob

theorem actual_frobenius_preserves_addition (P Q:(curve.baseChange K).Point) :
    pointFrob (P+Q) = pointFrob P + pointFrob Q := pointFrob.map_add P Q

theorem odd_annihilator_removes_two_torsion
    {L:Type*} [Field L] [DecidableEq L] (E:WeierstrassCurve.Affine L)
    (B:E.Point) (k:ℕ) (h2:B+B=0) (ho:(2*k+1) • B=0) : B=0 := by
  have ht : (2:ℕ) • B = 0 := by simpa only [two_nsmul] using h2
  have hm : (2*k) • B = 0 := by rw [mul_nsmul, ht, smul_zero]
  simpa only [add_nsmul, one_nsmul, hm, zero_add] using ho

theorem mixed_odd_twist_cancels
    {L:Type*} [Field L] [DecidableEq L] (E:WeierstrassCurve.Affine L)
    (pi:E.Point →+ E.Point) (R A B:E.Point) (k:ℕ)
    (h:R=A+B) (hR:pi R=R) (hA:pi A=A) (hB:pi B= -B)
    (ho:(2*k+1) • B=0) : B=0 := by
  exact odd_annihilator_removes_two_torsion E B k
    (TwistRoutes.mixed_relation_twist_double_zero E pi R A B h hR hA hB) ho

theorem integer_helper_trace_zero
    {L:Type*} [Field L] [DecidableEq L] (E:WeierstrassCurve.Affine L)
    (pi:E.Point →+ E.Point) {I:Type*} [Fintype I]
    (c:I → ℤ) (U:I → E.Point) (hU:∀ j,pi (U j)= -U j) :
    (∑ j,c j • U j) + pi (∑ j,c j • U j) = 0 := by
  simp only [map_sum, map_zsmul, hU, smul_neg]
  rw [← Finset.sum_add_distrib]
  apply Finset.sum_eq_zero
  intro j _
  exact add_neg_cancel _

end TwistFollowup
