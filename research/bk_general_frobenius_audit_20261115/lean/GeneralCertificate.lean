import SourceField

namespace BKGeneralFrobeniusAudit
noncomputable section
open FrobeniusPointDescentAudit

/-- Explicit actual-object arguments avoid named-assignment prefixes in the
frozen written-header extractor. These are definitional aliases only. -/
abbrev actualFrobenius (F L : Type*) [Field F] [Fintype F] [Field L] [Algebra F L]
    [DecidableEq L] (W : WeierstrassCurve.Affine F) (r : ℕ) :=
  linearFrobenius (L := L) W r

abbrev actualFixedPower (F L : Type*) [Field F] [Fintype F] [Field L] [Algebra F L]
    [DecidableEq L] (W : WeierstrassCurve.Affine F) (r k : ℕ) :=
  fixedPower (L := L) W r k

abbrev actualPointMap (F K L : Type*) [Field F] [Field K] [Field L]
    [Algebra F K] [Algebra F L] [Algebra K L] [IsScalarTower F K L]
    [DecidableEq K] [DecidableEq L] (W : WeierstrassCurve.Affine F) :=
  WeierstrassCurve.Affine.Point.map (W' := W) (Algebra.ofId K L)

theorem general_frobenius_certificate :
    (∀ (F L : Type*) [Field F] [Fintype F] [Field L] [Algebra F L]
      [Algebra.IsAlgebraic F L] [DecidableEq L] (W : WeierstrassCurve.Affine F) (r : ℕ),
      Function.Bijective (actualFrobenius F L W r) ∧
      ∀ P : Torsion (W.baseChange L) r,
        (frobeniusEquiv W r P).val = pointFrobenius W P.val) ∧
    (∀ (F K L : Type*) [Field F] [Fintype F] [Field K] [Fintype K] [Field L]
      [Algebra F K] [Algebra F L] [Algebra K L] [IsScalarTower F K L]
      [DecidableEq K] [DecidableEq L] (W : WeierstrassCurve.Affine F) (r k : ℕ)
      (_hc : Fintype.card K = Fintype.card F^k),
      Nonempty (Torsion (W.baseChange K) r ≃ₗ[ZMod r] actualFixedPower F L W r k) ∧
      Finite (actualFixedPower F L W r k) ∧
      Nat.card (actualFixedPower F L W r k) = Nat.card (Torsion (W.baseChange K) r) ∧
      ((∀ P : (W.baseChange L).Point, r • P = 0 →
        P ∈ Set.range (actualPointMap F K L W)) ↔
          (actualFrobenius F L W r)^k = 1)) ∧
    (∀ (F L : Type*) [Field F] [Fintype F] [Field L] [Algebra F L]
      [DecidableEq F] [DecidableEq L] (W : WeierstrassCurve.Affine F) (r : ℕ),
      r.Prime → r ∣ Nat.card W.Point →
        ∃ P : Torsion (W.baseChange L) r, P ≠ 0 ∧ linearFrobenius W r P = P) ∧
    (∀ (K : Type*) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K),
      Subsingleton (Torsion W 1) ∧ Nonempty (Torsion W 0 ≃+ W.Point)) ∧
    (SourceNineteen.curve.Δ ≠ 0 ∧ Nat.card SourceNineteen.curve.Point = 18 ∧
      addOrderOf SourceNineteen.generator = 18 ∧ IsAddCyclic SourceNineteen.curve.Point ∧
      Nat.card (Torsion SourceNineteen.curve 3) = 3 ∧
      Nat.card (Torsion (SourceNineteen.curve.baseChange SourceNineteen.Closure) 3) = 9 ∧
      Nat.card (actualFixedPower (ZMod 19) SourceNineteen.Closure SourceNineteen.curve 3 1) = 3 ∧
      actualFrobenius (ZMod 19) SourceNineteen.Closure SourceNineteen.curve 3 ≠ 1 ∧
      orderOf (actualFrobenius (ZMod 19) SourceNineteen.Closure SourceNineteen.curve 3) = 3 ∧
      SourceNineteen.sourceMatrix.det = (19 : ZMod 3) ∧
      Fintype.card SourceNineteen.CubeField = 19^3 ∧
      Nat.card (Torsion (SourceNineteen.curve.baseChange SourceNineteen.CubeField) 3) = 9) ∧
    (∀ (K : Type*) [Field K] [Fintype K] [Algebra (ZMod 19) K]
      [Algebra K SourceNineteen.Closure] [IsScalarTower (ZMod 19) K SourceNineteen.Closure]
      [DecidableEq K] (k : ℕ), Fintype.card K = 19^k →
      ((∀ P : (SourceNineteen.curve.baseChange SourceNineteen.Closure).Point, (3 : ℕ) • P = 0 →
        P ∈ Set.range (actualPointMap (ZMod 19) K SourceNineteen.Closure SourceNineteen.curve)) ↔ 3 ∣ k)) ∧
    (∀ P : (SourceNineteen.curve.baseChange SourceNineteen.Closure).Point, (3 : ℕ) • P = 0 →
      P ∈ Set.range (actualPointMap (ZMod 19) SourceNineteen.CubeField SourceNineteen.Closure SourceNineteen.curve)) := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, SourceNineteen.source_extension_three⟩
  · intro F L _ _ _ _ _ _ W r
    exact ⟨linear_frobenius_bijective W r, frobeniusEquiv_apply W r⟩
  · intro F K L _ _ _ _ _ _ _ _ _ _ _ W r k hc
    exact ⟨⟨fixedFieldEquiv W r k hc⟩, fixedPower_finite W r k hc,
      fixedPower_card W r k hc, actual_extension_iff_power_identity W r k hc⟩
  · intro F L _ _ _ _ _ _ W r hr hN
    exact actual_prime_fixed_vector W r hr hN
  · intro K _ _ W
    exact ⟨torsion_one_subsingleton W, ⟨zeroTorsionEquiv W⟩⟩
  · exact ⟨SourceNineteen.curve_nonsingular, SourceNineteen.rational_cardinality,
      SourceNineteen.generator_order, SourceNineteen.rational_cyclic,
      SourceNineteen.rational_three_torsion_card, SourceNineteen.geometric_three_card,
      SourceNineteen.source_fixed_card, SourceNineteen.source_frobenius_not_identity,
      SourceNineteen.source_frobenius_order, SourceNineteen.sourceMatrix_det,
      SourceNineteen.cube_field_card, SourceNineteen.cubic_three_torsion_card⟩
  · intro K _ _ _ _ _ _ k hc
    exact SourceNineteen.source_all_extensions k hc

#print axioms general_frobenius_certificate
end
end BKGeneralFrobeniusAudit
