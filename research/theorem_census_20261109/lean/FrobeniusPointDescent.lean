import TorsionBoundaries
import Mathlib.FieldTheory.Finite.Basic

/-! Actual finite-field Frobenius and rational curve points. This closes
the point-descent bridge, not the geometric torsion rank/determinant theorem. -/
namespace FrobeniusPointDescentAudit
noncomputable section
open Polynomial
variable {F L : Type*} [Field F] [Fintype F] [Field L] [Algebra F L]

theorem finite_field_split : (X^Fintype.card F-X : F[X]).Splits := by
  rw [Polynomial.splits_iff_card_roots,FiniteField.roots_X_pow_card_sub_X,
    ←Finset.card_def,Finset.card_univ,
    FiniteField.X_pow_card_sub_X_natDegree_eq F Fintype.one_lt_card]

theorem scalar_frobenius_fixed_iff (a : L) :
    FiniteField.frobeniusAlgHom F L a=a ↔ ∃ b : F,algebraMap F L b=a := by
  constructor
  · intro h
    change a∈(algebraMap F L).range
    apply finite_field_split.mem_range_of_isRoot
      (FiniteField.X_pow_card_sub_X_ne_zero F Fintype.one_lt_card)
    simpa [Polynomial.IsRoot,sub_eq_zero] using h
  · rintro ⟨b,rfl⟩
    exact (FiniteField.frobeniusAlgHom F L).commutes b

variable [DecidableEq F] [DecidableEq L]

def pointFrobenius (W : WeierstrassCurve.Affine F) :
    (W.baseChange L).Point→+(W.baseChange L).Point :=
  WeierstrassCurve.Affine.Point.map (W':=W) (FiniteField.frobeniusAlgHom F L)

set_option backward.isDefEq.respectTransparency false in
theorem point_frobenius_fixed_iff (W : WeierstrassCurve.Affine F)
    (P : (W.baseChange L).Point) :
    pointFrobenius W P=P ↔
      ∃ Q : W.Point,WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId F L) Q=P := by
  cases P with
  | zero =>
    constructor
    · intro _
      exact ⟨0,rfl⟩
    · intro _
      rfl
  | some x y h =>
    constructor
    · intro hfix
      have hx : FiniteField.frobeniusAlgHom F L x=x :=
        (WeierstrassCurve.Affine.Point.some.inj hfix).1
      have hy : FiniteField.frobeniusAlgHom F L y=y :=
        (WeierstrassCurve.Affine.Point.some.inj hfix).2
      obtain ⟨a,ha⟩ := (scalar_frobenius_fixed_iff x).mp hx
      obtain ⟨b,hb⟩ := (scalar_frobenius_fixed_iff y).mp hy
      have hab : W.Nonsingular a b := by
        apply (W.baseChange_nonsingular (Algebra.ofId F L).injective a b).mp
        change (W.baseChange L).Nonsingular (algebraMap F L a) (algebraMap F L b)
        rw [ha,hb]
        exact h
      refine ⟨.some a b hab,?_⟩
      change WeierstrassCurve.Affine.Point.some (algebraMap F L a) (algebraMap F L b) _=
        WeierstrassCurve.Affine.Point.some x y h
      rw [WeierstrassCurve.Affine.Point.some.injEq]
      exact ⟨ha,hb⟩
    · rintro ⟨Q,hQ⟩
      rw [←hQ]
      cases Q with
      | zero => rfl
      | some a b hab =>
        change WeierstrassCurve.Affine.Point.some _ _ _=
          WeierstrassCurve.Affine.Point.some _ _ _
        rw [WeierstrassCurve.Affine.Point.some.injEq]
        exact ⟨(FiniteField.frobeniusAlgHom F L).commutes a,
          (FiniteField.frobeniusAlgHom F L).commutes b⟩

omit [DecidableEq F] in
theorem point_frobenius_preserves_torsion (W : WeierstrassCurve.Affine F)
    (r : ℕ) (P : (W.baseChange L).Point) (h : r • P=0) :
    r • pointFrobenius W P=0 := by
  simpa only [map_nsmul,map_zero] using congrArg (pointFrobenius W) h

theorem torsion_rational_iff_frobenius_identity (W : WeierstrassCurve.Affine F) (r : ℕ) :
    (∀ P : (W.baseChange L).Point,r • P=0 →
      P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId F L))) ↔
    (∀ P : (W.baseChange L).Point,r • P=0 → pointFrobenius W P=P) := by
  constructor
  · intro h P hP
    exact (point_frobenius_fixed_iff W P).mpr (h P hP)
  · intro h P hP
    exact (point_frobenius_fixed_iff W P).mp (h P hP)

private instance primeSevenControl : Fact (Nat.Prime 7) := ⟨by decide⟩
local instance : DecidableEq TorsionBoundaryAudit.K := Classical.decEq _

theorem bk_nonfixed_control :
    Nat.Prime 3 ∧ Nat.Coprime 3 7 ∧ 3 ∣ 7-1 ∧
      3^2 ∣ Nat.card TorsionBoundaryAudit.bkCurve.Point ∧
      TorsionBoundaryAudit.bkCurve.Δ≠0 ∧
      ∃ P : (TorsionBoundaryAudit.bkCurve.baseChange TorsionBoundaryAudit.K).Point,
        (3:ℕ) • P=0 ∧ pointFrobenius TorsionBoundaryAudit.bkCurve P≠P := by
  obtain ⟨hp,hc,hd,hN,hΔ,P,hP,hn⟩ := TorsionBoundaryAudit.bk_excluded_boundary_counterexample
  refine ⟨hp,hc,hd,hN,hΔ,P,hP,?_⟩
  intro hfix
  obtain ⟨Q,hQ⟩ := (point_frobenius_fixed_iff TorsionBoundaryAudit.bkCurve P).mp hfix
  exact hn ⟨Q,hQ⟩

theorem bk_full_torsion_fixed_false :
    ¬ ∀ P : (TorsionBoundaryAudit.bkCurve.baseChange TorsionBoundaryAudit.K).Point,
      (3:ℕ) • P=0 → pointFrobenius TorsionBoundaryAudit.bkCurve P=P := by
  intro h
  obtain ⟨_,_,_,_,_,P,hP,hfix⟩ := bk_nonfixed_control
  exact hfix (h P hP)

theorem frobenius_descent_audit_certificate :
    (∀ (A B : Type*) [Field A] [Fintype A] [Field B] [Algebra A B]
      [DecidableEq A] [DecidableEq B] (W : WeierstrassCurve.Affine A)
      (P : (W.baseChange B).Point),pointFrobenius W P=P ↔
        ∃ Q : W.Point,WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId A B) Q=P) ∧
    (∀ (A B : Type*) [Field A] [Fintype A] [Field B] [Algebra A B]
      [DecidableEq A] [DecidableEq B] (W : WeierstrassCurve.Affine A)
      (r : ℕ) (P : (W.baseChange B).Point),r • P=0 → r • pointFrobenius W P=0) ∧
    (∀ (A B : Type*) [Field A] [Fintype A] [Field B] [Algebra A B]
      [DecidableEq A] [DecidableEq B] (W : WeierstrassCurve.Affine A) (r : ℕ),
      (∀ P : (W.baseChange B).Point,r • P=0 →
        P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId A B))) ↔
      (∀ P : (W.baseChange B).Point,r • P=0 → pointFrobenius W P=P)) ∧
    (Nat.Prime 3 ∧ Nat.Coprime 3 7 ∧ 3 ∣ 7-1 ∧
      3^2 ∣ Nat.card TorsionBoundaryAudit.bkCurve.Point ∧
      TorsionBoundaryAudit.bkCurve.Δ≠0 ∧
      ∃ P : (TorsionBoundaryAudit.bkCurve.baseChange TorsionBoundaryAudit.K).Point,
        (3:ℕ) • P=0 ∧ pointFrobenius TorsionBoundaryAudit.bkCurve P≠P) ∧
    (¬ ∀ P : (TorsionBoundaryAudit.bkCurve.baseChange TorsionBoundaryAudit.K).Point,
      (3:ℕ) • P=0 → pointFrobenius TorsionBoundaryAudit.bkCurve P=P) := by
  refine ⟨?_,?_,?_,bk_nonfixed_control,bk_full_torsion_fixed_false⟩
  · intro A B _ _ _ _ _ _ W P
    exact point_frobenius_fixed_iff W P
  · intro A B _ _ _ _ _ _ W r P h
    exact point_frobenius_preserves_torsion W r P h
  · intro A B _ _ _ _ _ _ W r
    exact torsion_rational_iff_frobenius_identity W r

#print axioms scalar_frobenius_fixed_iff
#print axioms point_frobenius_fixed_iff
#print axioms torsion_rational_iff_frobenius_identity
#print axioms bk_nonfixed_control
#print axioms bk_full_torsion_fixed_false
#print axioms frobenius_descent_audit_certificate
end
end FrobeniusPointDescentAudit
