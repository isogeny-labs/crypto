import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.GroupTheory.SpecificGroups.Cyclic
import Mathlib.Algebra.Field.ZMod
import Mathlib.Algebra.CharP.Algebra
import Mathlib.Tactic.NormNum

/-!
# Worked example: the order-three subgroup of y² = x³ + 1 over F₅ and F₇

The two generators are P = (0, 1) over F₅ and R = (0, 1) over F₇.
The transfer table is O ↦ O, P ↦ R, (0, 4) ↦ (0, 6).
The subgroup order 3 differs from both field characteristics 5 and 7.
-/

namespace CrossCharacteristicExample

open WeierstrassCurve.Affine

instance : Fact (Nat.Prime 3) := ⟨by decide⟩
instance : Fact (Nat.Prime 5) := ⟨by decide⟩
instance : Fact (Nat.Prime 7) := ⟨by decide⟩

def curve5 : WeierstrassCurve (ZMod 5) := ⟨0, 0, 0, 0, 1⟩

def curve7 : WeierstrassCurve (ZMod 7) := ⟨0, 0, 0, 0, 1⟩

instance : curve5.IsElliptic where
  isUnit := by
    apply isUnit_iff_ne_zero.mpr
    norm_num [curve5, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
    decide

instance : curve7.IsElliptic where
  isUnit := by
    apply isUnit_iff_ne_zero.mpr
    norm_num [curve7, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
    decide

/-- The source generator (0, 1) over F₅. -/
def P : curve5.toAffine.Point :=
  .some (x := 0) (y := 1) (by
    rw [nonsingular_iff', equation_iff]
    norm_num [curve5]
    decide)

/-- The target generator (0, 1) over F₇. -/
def R : curve7.toAffine.Point :=
  .some (x := 0) (y := 1) (by
    rw [nonsingular_iff', equation_iff]
    norm_num [curve7]
    decide)

/-- The source challenge (0, 4) over F₅. -/
def Q : curve5.toAffine.Point :=
  .some (x := 0) (y := 4) (by
    rw [nonsingular_iff', equation_iff]
    norm_num [curve5]
    decide)

/-- The transferred challenge (0, 6) over F₇. -/
def S : curve7.toAffine.Point :=
  .some (x := 0) (y := 6) (by
    rw [nonsingular_iff', equation_iff]
    norm_num [curve7]
    decide)

theorem P_double : P + P = Q := by
  norm_num [P, Q, Point.add_def, Point.add, negY, slope,
    addX, addY, negAddY, curve5,
    show (1 : ZMod 5) ≠ -1 from by decide,
    show (-1 : ZMod 5) = 4 from by decide,
    show (1 : ZMod 5) ≠ 4 from by decide]

theorem R_double : R + R = S := by
  norm_num [R, S, Point.add_def, Point.add, negY, slope,
    addX, addY, negAddY, curve7,
    show (1 : ZMod 7) ≠ -1 from by decide,
    show (-1 : ZMod 7) = 6 from by decide,
    show (1 : ZMod 7) ≠ 6 from by decide]

theorem Q_add_P : Q + P = 0 := by
  apply Point.add_of_Y_eq rfl
  change (4 : ZMod 5) = -1
  decide

theorem S_add_R : S + R = 0 := by
  apply Point.add_of_Y_eq rfl
  change (6 : ZMod 7) = -1
  decide

theorem P_order : addOrderOf P = 3 := by
  apply addOrderOf_eq_prime
  · simpa only [show (3 : ℕ) = 2 + 1 from rfl, add_nsmul, two_nsmul,
      one_nsmul, P_double] using Q_add_P
  · exact Point.some_ne_zero _

theorem R_order : addOrderOf R = 3 := by
  apply addOrderOf_eq_prime
  · simpa only [show (3 : ℕ) = 2 + 1 from rfl, add_nsmul, two_nsmul,
      one_nsmul, R_double] using S_add_R
  · exact Point.some_ne_zero _

def sourceGenerator : AddSubgroup.zmultiples P := ⟨P, by simp⟩

def targetGenerator : AddSubgroup.zmultiples R := ⟨R, by simp⟩

/-- Apply Mathlib's existing equal-order constructor to the two concrete subgroups. -/
noncomputable def phi : AddSubgroup.zmultiples P ≃+ AddSubgroup.zmultiples R :=
  addEquivOfAddOrderOfEq (g := sourceGenerator) (g' := targetGenerator)
    (AddSubgroup.forall_zmultiples.mpr (fun n => ⟨n, rfl⟩))
    (AddSubgroup.forall_zmultiples.mpr (fun n => ⟨n, rfl⟩))
    (by simpa only [sourceGenerator, targetGenerator, AddSubgroup.addOrderOf_mk]
      using P_order.trans R_order.symm)

def challenge : AddSubgroup.zmultiples P :=
  ⟨Q, AddSubgroup.mem_zmultiples_iff.mpr
    ⟨2, by simpa only [two_zsmul] using P_double⟩⟩

theorem challenge_as_multiple : challenge = (2 : ℤ) • sourceGenerator := by
  apply Subtype.ext
  simpa only [two_zsmul] using P_double.symm

theorem phi_zero : phi 0 = 0 := map_zero phi

theorem phi_generator : phi sourceGenerator = targetGenerator :=
  addEquivOfAddOrderOfEq_apply_gen _ _ _

theorem phi_challenge : (phi challenge : curve7.toAffine.Point) = S := by
  rw [challenge_as_multiple, map_zsmul, phi_generator]
  change (2 : ℤ) • R = S
  simpa only [two_zsmul] using R_double

/-- The transfer table covers the entire source subgroup. -/
theorem subgroup_cases (T : AddSubgroup.zmultiples P) :
    T = 0 ∨ T = sourceGenerator ∨ T = challenge := by
  obtain ⟨n, hn⟩ := AddSubgroup.mem_zmultiples_iff.mp T.property
  have hmod := mod_addOrderOf_zsmul P n
  rw [P_order] at hmod
  have hrepr : (T : curve5.toAffine.Point) = (n % 3) • P := hn.symm.trans hmod.symm
  have hlo : 0 ≤ n % 3 := Int.emod_nonneg n (by decide)
  have hhi : n % 3 < 3 := Int.emod_lt_of_pos n (by decide)
  have hcases : n % 3 = 0 ∨ n % 3 = 1 ∨ n % 3 = 2 := by omega
  rcases hcases with hz | ho | ht
  · left
    apply Subtype.ext
    simpa only [hz, zero_zsmul] using hrepr
  · right; left
    apply Subtype.ext
    simpa only [ho, one_zsmul] using hrepr
  · right; right
    apply Subtype.ext
    change (T : curve5.toAffine.Point) = Q
    rw [hrepr, ht, two_zsmul, P_double]

/-- Scalar preservation and reflection, read off from the assembled isomorphism. -/
theorem transfer_dlog (T : AddSubgroup.zmultiples P) (k : ℤ) :
    (phi T : curve7.toAffine.Point) = k • R ↔ (T : curve5.toAffine.Point) = k • P := by
  constructor
  · intro h
    have hsub : phi T = phi (k • sourceGenerator) := by
      rw [map_zsmul, phi_generator]
      exact Subtype.ext h
    exact congrArg Subtype.val (phi.injective hsub)
  · intro h
    have hsub : T = k • sourceGenerator := Subtype.ext h
    rw [hsub, map_zsmul, phi_generator]
    rfl

theorem example_dlog (k : ℤ) : S = k • R ↔ Q = k • P := by
  have h := transfer_dlog challenge k
  simpa only [phi_challenge] using h

theorem source_scalar_mod (k : ℤ) : Q = k • P ↔ (2 : ℤ) ≡ k [ZMOD 3] := by
  have hQ : Q = (2 : ℤ) • P := by simpa only [two_zsmul] using P_double.symm
  rw [hQ, zsmul_eq_zsmul_iff_modEq, P_order]
  norm_num

theorem target_scalar_mod (k : ℤ) : S = k • R ↔ (2 : ℤ) ≡ k [ZMOD 3] := by
  rw [example_dlog k, source_scalar_mod k]

/-- The subgroup isomorphism does not arise from a unital map of the base fields. -/
theorem no_field_map : IsEmpty (ZMod 5 →+* ZMod 7) := by
  refine ⟨fun f => ?_⟩
  have h : (5 : ℕ) = 7 :=
    CharP.eq (ZMod 7) (charP_of_injective_ringHom f.injective 5) inferInstance
  exact (by decide : (5 : ℕ) ≠ 7) h

end CrossCharacteristicExample
