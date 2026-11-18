module
public import CubicCharacter
public import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

@[expose] public section
namespace PrimeTraceWeakBoundAudit
noncomputable section
open Polynomial
open scoped BigOperators
variable {K : Type*} [Field K]

def SquareFiber (a : K) := {y : K // y ^ 2 = a}
def squareFiberCard (a : K) : ℕ := Nat.card (SquareFiber a)

instance squareFiberFintype [Fintype K] [DecidableEq K] (a : K) : Fintype (SquareFiber a) :=
  inferInstanceAs (Fintype {y : K // y ^ 2 = a})

/-- Credited square-fiber argument from the earlier SquareFibers packet. -/
theorem square_fiber_zero : squareFiberCard (0 : K) = 1 := by
  apply Nat.card_eq_one_iff_exists.mpr
  refine ⟨⟨0, by simp⟩, fun y => Subtype.ext ?_⟩
  exact sq_eq_zero_iff.mp y.property

theorem square_fiber_nonsquare (a : K) (ha : ¬ IsSquare a) : squareFiberCard a = 0 := by
  let : IsEmpty (SquareFiber a) := ⟨fun y => ha ⟨y.val, by
    simpa only [pow_two] using y.property.symm⟩⟩
  exact Nat.card_of_isEmpty

theorem square_fiber_square (h2 : (2 : K) ≠ 0) (a : K)
    (ha : a ≠ 0) (hs : IsSquare a) : squareFiberCard a = 2 := by
  classical
  obtain ⟨y, hy⟩ := hs
  have hy0 : y ≠ 0 := by intro h; apply ha; rw [hy, h]; simp
  have hyn : y ≠ -y := by
    intro h
    have hz : 2 * y = 0 := by linear_combination h
    exact (mul_ne_zero h2 hy0) hz
  let s : Finset K := {y, -y}
  have he : {z : K | z ^ 2 = a} = (↑s : Set K) := by
    ext z
    simp only [Finset.mem_coe, Finset.mem_insert, Finset.mem_singleton, Set.mem_ofPred_eq, s]
    rw [hy, ← pow_two]
    exact sq_eq_sq_iff_eq_or_eq_neg
  calc
    squareFiberCard a = Nat.card ↥s := Nat.card_congr (Set.equivOfEq he)
    _ = 2 := by
      rw [Nat.card_eq_fintype_card, Fintype.card_coe]
      exact Finset.card_pair hyn

def completedCubic (W : WeierstrassCurve.Affine K) : K[X] :=
  X ^ 3 + C (W.a₂ + W.a₁ ^ 2 / 4) * X ^ 2 +
    C (W.a₄ + W.a₁ * W.a₃ / 2) * X + C (W.a₆ + W.a₃ ^ 2 / 4)

def squareShift (W : WeierstrassCurve.Affine K) (x : K) : K := (W.a₁ * x + W.a₃) / 2

theorem completed_cubic_degree (W : WeierstrassCurve.Affine K) :
    (completedCubic W).natDegree = 3 := by
  unfold completedCubic
  compute_degree!

theorem completed_square_identity (W : WeierstrassCurve.Affine K) (h2 : (2 : K) ≠ 0)
    (x y : K) :
    (y + squareShift W x) ^ 2 - (completedCubic W).eval x =
      y ^ 2 + W.a₁ * x * y + W.a₃ * y - (x ^ 3 + W.a₂ * x ^ 2 + W.a₄ * x + W.a₆) := by
  have h4 : (4 : K) ≠ 0 := by
    have h := pow_ne_zero 2 h2
    norm_num at h ⊢
    exact h
  simp only [completedCubic, squareShift, eval_add, eval_mul, eval_C, eval_X, eval_pow]
  field_simp [h2, h4]
  ring

theorem nonsingular_iff_completed (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (h2 : (2 : K) ≠ 0) (x y : K) :
    W.Nonsingular x y ↔ (y + squareShift W x) ^ 2 = (completedCubic W).eval x := by
  rw [← W.equation_iff_nonsingular_of_Δ_ne_zero hΔ, WeierstrassCurve.Affine.equation_iff]
  constructor
  · intro h
    apply sub_eq_zero.mp
    rw [completed_square_identity W h2]
    exact sub_eq_zero.mpr h
  · intro h
    apply sub_eq_zero.mp
    rw [← completed_square_identity W h2]
    exact sub_eq_zero.mpr h

def nonsingularFiberEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (h2 : (2 : K) ≠ 0) :
    {xy : K × K // W.Nonsingular xy.1 xy.2} ≃
      (Σ x : K, SquareFiber ((completedCubic W).eval x)) where
  toFun p := ⟨p.val.1, ⟨p.val.2 + squareShift W p.val.1,
    (nonsingular_iff_completed W hΔ h2 _ _).mp p.property⟩⟩
  invFun p := ⟨(p.1, p.2.val - squareShift W p.1),
    (nonsingular_iff_completed W hΔ h2 _ _).mpr (by simpa using p.2.property)⟩
  left_inv p := by
    apply Subtype.ext
    apply Prod.ext
    · rfl
    · simp
  right_inv p := by
    rcases p with ⟨x, ⟨y, hy⟩⟩
    dsimp only
    simp only [sub_add_cancel]

/-- Actual Mathlib points and the actual completed-square fibers. The earlier
EquationCount argument is credited; no point count is an input premise. -/
theorem actual_point_fiber_count [Fintype K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (h2 : (2 : K) ≠ 0) :
    Nat.card W.Point = (∑ x : K, squareFiberCard ((completedCubic W).eval x)) + 1 := by
  classical
  calc
    Nat.card W.Point = Nat.card (Option {xy : K × K // W.Nonsingular xy.1 xy.2}) :=
      Nat.card_congr W.nonsingularPointEquiv
    _ = Nat.card (Option (Σ x : K, SquareFiber ((completedCubic W).eval x))) :=
      Nat.card_congr (nonsingularFiberEquiv W hΔ h2).optionCongr
    _ = _ := by
      rw [Nat.card_eq_fintype_card, Fintype.card_option, Fintype.card_sigma]
      simp only [squareFiberCard, Nat.card_eq_fintype_card]

#print axioms completed_square_identity
#print axioms nonsingularFiberEquiv
#print axioms actual_point_fiber_count
end
end PrimeTraceWeakBoundAudit
