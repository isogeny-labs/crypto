import CrossCharacteristicTwo
import Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots
import Mathlib.Tactic.NormNum.Prime
import Mathlib.Tactic.FinCases

/-!
# A larger binary field and a new order-seven subgroup

The source field has 65537 elements; the target field has 2^256 elements.
The subgroup order is 7, not a cryptographic-size subgroup order. A primitive
cube root ω supplies coefficients outside the binary prime subfield.
-/

noncomputable section

namespace CrossCharacteristicLarger

open WeierstrassCurve.Affine

instance : Fact (Nat.Prime 65537) := ⟨by norm_num⟩

abbrev Fbig := GaloisField 2 256

instance : DecidableEq Fbig := Classical.decEq _

theorem source_card : Nat.card (ZMod 65537) = 65537 := by
  rw [Nat.card_eq_fintype_card, ZMod.card]

theorem target_card : Nat.card Fbig = 2 ^ 256 := GaloisField.card 2 256 (by decide)

theorem exists_omega : ∃ a : Fbig, IsPrimitiveRoot a 3 := by
  classical
  letI : Fintype Fbig := Fintype.ofFinite _
  have hcard : Fintype.card Fbig = 2 ^ 256 := by
    rw [← Nat.card_eq_fintype_card, target_card]
  obtain ⟨u, hu⟩ := exists_prime_orderOf_dvd_card (G := Fbigˣ) 3
    (by rw [Fintype.card_units, hcard]; norm_num)
  refine ⟨(u : Fbig), ?_⟩
  rw [IsPrimitiveRoot.coe_units_iff]
  simpa only [hu] using IsPrimitiveRoot.orderOf u

def omega : Fbig := Classical.choose exists_omega

theorem omega_primitive : IsPrimitiveRoot omega 3 := Classical.choose_spec exists_omega

theorem omega_ne_zero : omega ≠ 0 := (omega_primitive.isUnit (by decide)).ne_zero

theorem omega_ne_one : omega ≠ 1 := omega_primitive.ne_one (by decide)

theorem omega_not_from_F2 (x : ZMod 2) : algebraMap (ZMod 2) Fbig x ≠ omega := by
  fin_cases x <;> simp [Ne.symm omega_ne_zero, Ne.symm omega_ne_one]

theorem omega_relation : omega ^ 2 + omega + 1 = 0 := by
  have h := omega_primitive.geom_sum_eq_zero (by decide : 1 < 3)
  simpa only [Finset.sum_range_succ, Finset.sum_range_zero, pow_zero,
    pow_one, zero_add, add_comm, add_left_comm, add_assoc] using h

theorem omega_sq : omega ^ 2 = omega + 1 := by
  apply (CharTwo.add_eq_zero).mp
  rw [← add_assoc]
  exact omega_relation

theorem omega_inv : omega⁻¹ = omega + 1 := by
  apply inv_eq_of_mul_eq_one_right
  rw [mul_add, mul_one, ← pow_two, omega_sq]
  rw [add_comm omega 1, add_assoc, CharTwo.add_self_eq_zero, add_zero]

def sourceCurve : WeierstrassCurve (ZMod 65537) := ⟨-1, -4, -4, 0, 0⟩

def targetCurve : WeierstrassCurve Fbig := ⟨0, omega, omega, 0, 0⟩

instance : sourceCurve.IsElliptic where
  isUnit := by
    apply isUnit_iff_ne_zero.mpr
    norm_num [sourceCurve, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
    decide

instance : targetCurve.IsElliptic where
  isUnit := by
    apply isUnit_iff_ne_zero.mpr
    have hfour : (4 : Fbig) = 0 := by simpa using CharP.cast_eq_mod Fbig 2 4
    norm_num [targetCurve, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈,
      CharTwo.two_eq_zero]
    rw [hfour]
    norm_num
    exact ⟨fun h => (by decide : ¬ 2 ∣ 27) ((CharP.cast_eq_zero_iff Fbig 2 27).mp h),
      omega_ne_zero⟩

def P : sourceCurve.toAffine.Point :=
  .some (x := 0) (y := 0) (by
    rw [← equation_iff_nonsingular, equation_iff]
    norm_num [sourceCurve])

def P2 : sourceCurve.toAffine.Point :=
  .some (x := 4) (y := 8) (by
    rw [← equation_iff_nonsingular, equation_iff]
    norm_num [sourceCurve])

def Q : sourceCurve.toAffine.Point :=
  .some (x := 2) (y := 2) (by
    rw [← equation_iff_nonsingular, equation_iff]
    norm_num [sourceCurve])

def P6 : sourceCurve.toAffine.Point :=
  .some (x := 0) (y := 4) (by
    rw [← equation_iff_nonsingular, equation_iff]
    norm_num [sourceCurve])

def R : targetCurve.toAffine.Point :=
  .some (x := 0) (y := 0) (by
    rw [← equation_iff_nonsingular, equation_iff]
    norm_num [targetCurve])

def R2 : targetCurve.toAffine.Point :=
  .some (x := omega) (y := omega) (by
    rw [← equation_iff_nonsingular, equation_iff]
    simp [targetCurve, pow_succ', CharTwo.add_self_eq_zero])

def S : targetCurve.toAffine.Point :=
  .some (x := 1) (y := omega + 1) (by
    rw [← equation_iff_nonsingular, equation_iff]
    simp only [targetCurve, zero_mul, add_zero,
      one_pow, mul_one, CharTwo.add_sq, mul_add, ← pow_two, omega_sq]
    simp only [CharTwo.add_cancel_left, add_comm])

def R6 : targetCurve.toAffine.Point :=
  .some (x := 0) (y := omega) (by
    rw [← equation_iff_nonsingular, equation_iff]
    simp [targetCurve, pow_two, CharTwo.add_self_eq_zero])

theorem P_double : P + P = P2 := by
  norm_num [P, P2, Point.add_def, Point.add, negY, slope,
    addX, addY, negAddY, sourceCurve,
    show (0 : ZMod 65537) ≠ 4 from by decide]

theorem P2_add_P : P2 + P = Q := by
  have hdiv : (8 : ZMod 65537) / 4 = 2 := by
    apply (div_eq_iff (by decide : (4 : ZMod 65537) ≠ 0)).mpr
    norm_num
  norm_num [P, P2, Q, Point.add_def, Point.add, negY, slope,
    addX, addY, negAddY, sourceCurve,
    show (4 : ZMod 65537) ≠ 0 from by decide]
  simp only [hdiv]
  norm_num

theorem Q_double : Q + Q = P6 := by
  norm_num [Q, P6, Point.add_def, Point.add, negY, slope,
    addX, addY, negAddY, sourceCurve,
    show (2 : ZMod 65537) ≠ 4 from by decide]
  simp only [div_self (by decide : (2 : ZMod 65537) ≠ 0)]
  norm_num

theorem P6_add_P : P6 + P = 0 := by
  apply Point.add_of_Y_eq rfl
  norm_num [negY, sourceCurve]

private theorem three_eq_one : (3 : Fbig) = 1 := by
  simpa using CharP.cast_eq_mod Fbig 2 3

theorem R_double : R + R = R2 := by
  simp [R, R2, Point.add_def, Point.add, negY, slope, addX, addY, negAddY,
    targetCurve, CharTwo.neg_eq, CharTwo.sub_eq_add, Ne.symm omega_ne_zero]

theorem R2_add_R : R2 + R = S := by
  simp [R, R2, S, Point.add_def, Point.add, negY, slope, addX, addY, negAddY,
    targetCurve, CharTwo.neg_eq, CharTwo.sub_eq_add, omega_ne_zero,
    CharTwo.add_cancel_left, add_comm]

theorem S_double : S + S = R6 := by
  simp [S, R6, Point.add_def, Point.add, negY, slope, addX, addY, negAddY,
    targetCurve, CharTwo.neg_eq, CharTwo.sub_eq_add, omega_ne_zero,
    CharTwo.two_eq_zero, three_eq_one, omega_inv, CharTwo.add_sq, omega_sq,
    CharTwo.add_self_eq_zero, CharTwo.add_cancel_left, CharTwo.add_cancel_right]

theorem R6_add_R : R6 + R = 0 := by
  apply Point.add_of_Y_eq rfl
  simp [negY, targetCurve, CharTwo.neg_eq]

theorem Q_as_multiple : Q = (3 : ℕ) • P := by
  simpa only [show (3 : ℕ) = 2 + 1 from rfl, add_nsmul, two_nsmul,
    one_nsmul, P_double] using P2_add_P.symm

theorem S_as_multiple : S = (3 : ℕ) • R := by
  simpa only [show (3 : ℕ) = 2 + 1 from rfl, add_nsmul, two_nsmul,
    one_nsmul, R_double] using R2_add_R.symm

theorem P_order : addOrderOf P = 7 := by
  apply addOrderOf_eq_prime
  · simpa only [show (7 : ℕ) = 3 + 3 + 1 from rfl, add_nsmul, one_nsmul,
      ← Q_as_multiple, Q_double] using P6_add_P
  · exact Point.some_ne_zero _

theorem R_order : addOrderOf R = 7 := by
  apply addOrderOf_eq_prime
  · simpa only [show (7 : ℕ) = 3 + 3 + 1 from rfl, add_nsmul, one_nsmul,
      ← S_as_multiple, S_double] using R6_add_R
  · exact Point.some_ne_zero _

def sourceGenerator : AddSubgroup.zmultiples P := ⟨P, by simp⟩

def targetGenerator : AddSubgroup.zmultiples R := ⟨R, by simp⟩

def phi : AddSubgroup.zmultiples P ≃+ AddSubgroup.zmultiples R :=
  addEquivOfAddOrderOfEq (g := sourceGenerator) (g' := targetGenerator)
    (AddSubgroup.forall_zmultiples.mpr (fun n => ⟨n, rfl⟩))
    (AddSubgroup.forall_zmultiples.mpr (fun n => ⟨n, rfl⟩))
    (by simpa only [sourceGenerator, targetGenerator, AddSubgroup.addOrderOf_mk]
      using P_order.trans R_order.symm)

def challenge : AddSubgroup.zmultiples P :=
  ⟨Q, AddSubgroup.mem_zmultiples_iff.mpr
    ⟨3, by simpa only [natCast_zsmul] using Q_as_multiple.symm⟩⟩

theorem challenge_as_multiple : challenge = (3 : ℤ) • sourceGenerator := by
  apply Subtype.ext
  simpa only [natCast_zsmul] using Q_as_multiple

theorem phi_generator : phi sourceGenerator = targetGenerator :=
  addEquivOfAddOrderOfEq_apply_gen _ _ _

theorem phi_challenge : (phi challenge : targetCurve.toAffine.Point) = S := by
  rw [challenge_as_multiple, map_zsmul, phi_generator]
  change (3 : ℤ) • R = S
  simpa only [natCast_zsmul] using S_as_multiple.symm

theorem transfer_dlog (T : AddSubgroup.zmultiples P) (k : ℤ) :
    (phi T : targetCurve.toAffine.Point) = k • R ↔
      (T : sourceCurve.toAffine.Point) = k • P := by
  have h : phi T = phi (k • sourceGenerator) ↔ T = k • sourceGenerator :=
    phi.injective.eq_iff
  simpa only [map_zsmul, phi_generator, Subtype.ext_iff] using h

theorem example_dlog (k : ℤ) : S = k • R ↔ Q = k • P := by
  have h := transfer_dlog challenge k
  simpa only [phi_challenge] using h

theorem source_scalar_mod (k : ℤ) : Q = k • P ↔ (3 : ℤ) ≡ k [ZMOD 7] := by
  have hQ : Q = (3 : ℤ) • P := by simpa only [natCast_zsmul] using Q_as_multiple
  rw [hQ, zsmul_eq_zsmul_iff_modEq, P_order]
  norm_num

theorem target_scalar_mod (k : ℤ) : S = k • R ↔ (3 : ℤ) ≡ k [ZMOD 7] := by
  rw [example_dlog k, source_scalar_mod k]

end CrossCharacteristicLarger
