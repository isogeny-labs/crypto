import GeneralS4
import RationalCounterexample
import Mathlib.Algebra.BigOperators.Fin
import Mathlib.Data.Fin.Tuple.Basic

/-! Actual finite point families and the two-point extension used by the
geometric summation-polynomial recursion. -/
namespace SemaevPointListsAudit
noncomputable section
open SemaevS3 GeneralS3Audit GeneralS4Audit
open scoped BigOperators
variable {K : Type*} [Field K] [DecidableEq K]

/-- Every prescribed input is the affine xcoordinate of an actual curve
point, and their actual group sum is zero. -/
def ManyPoints (W : WeierstrassCurve.Affine K) {k : ℕ} (xs : Fin k→K) : Prop :=
  ∃ P : Fin k→W.Point,(∀ i,pointX (P i)=some (xs i)) ∧ (∑ i,P i)=0

theorem sum_snoc {A : Type*} [AddCommMonoid A] {k : ℕ} (P : Fin k→A) (Q : A) :
    (∑ i : Fin (k+1),Fin.snoc P Q i)=(∑ i : Fin k,P i)+Q := by
  simpa only [Fin.snoc_castSucc,Fin.snoc_last] using
    (Fin.sum_univ_castSucc (Fin.snoc P Q))

theorem manyPoints_snoc_iff (W : WeierstrassCurve.Affine K) {k : ℕ}
    (xs : Fin k→K) (x : K) :
    ManyPoints W (Fin.snoc xs x) ↔
      ∃ (P : Fin k→W.Point) (Q : W.Point),
        (∀ i,pointX (P i)=some (xs i)) ∧ pointX Q=some x ∧ (∑ i,P i)+Q=0 := by
  constructor
  · rintro ⟨P,hP,hs⟩
    refine ⟨fun i=>P i.castSucc,P (Fin.last k),?_,?_,?_⟩
    · intro i
      simpa only [Fin.snoc_castSucc] using hP i.castSucc
    · simpa only [Fin.snoc_last] using hP (Fin.last k)
    · simpa only [Fin.sum_univ_castSucc] using hs
  · rintro ⟨P,Q,hP,hQ,hs⟩
    refine ⟨Fin.snoc P Q,?_,?_⟩
    · intro i
      refine Fin.lastCases ?_ (fun j=>?_) i
      · simpa only [Fin.snoc_last] using hQ
      · simpa only [Fin.snoc_castSucc] using hP j
    · simpa only [sum_snoc] using hs

theorem manyPoints_two_snoc_iff (W : WeierstrassCurve.Affine K) {k : ℕ}
    (xs : Fin k→K) (x y : K) :
    ManyPoints W (Fin.snoc (Fin.snoc xs x) y) ↔
      ∃ (P : Fin k→W.Point) (Q R : W.Point),
        (∀ i,pointX (P i)=some (xs i)) ∧ pointX Q=some x ∧ pointX R=some y ∧
          (∑ i,P i)+Q+R=0 := by
  constructor
  · rintro ⟨P,hP,hs⟩
    refine ⟨fun i=>P i.castSucc.castSucc,P (Fin.last k).castSucc,P (Fin.last (k+1)),?_,?_,?_,?_⟩
    · intro i
      simpa only [Fin.snoc_castSucc] using hP i.castSucc.castSucc
    · simpa only [Fin.snoc_castSucc,Fin.snoc_last] using hP (Fin.last k).castSucc
    · simpa only [Fin.snoc_last] using hP (Fin.last (k+1))
    · simpa only [Fin.sum_univ_castSucc] using hs
  · rintro ⟨P,Q,R,hP,hQ,hR,hs⟩
    refine ⟨Fin.snoc (Fin.snoc P Q) R,?_,?_⟩
    · intro i
      refine Fin.lastCases ?_ (fun j=>?_) i
      · simpa only [Fin.snoc_last] using hR
      · refine Fin.lastCases ?_ (fun l=>?_) j
        · simpa only [Fin.snoc_castSucc,Fin.snoc_last] using hQ
        · simpa only [Fin.snoc_castSucc] using hP l
    · simpa only [sum_snoc] using hs

omit [DecidableEq K] in
theorem pointX_exists_of_ne_zero (W : WeierstrassCurve.Affine K)
    (P : W.Point) (hP : P≠0) : ∃ x : K,pointX P=some x := by
  cases P with
  | zero => exact (hP rfl).elim
  | some x y h => exact ⟨x,rfl⟩

omit [DecidableEq K] in
theorem exists_point_at_x [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (x : K) : ∃ P : W.Point,pointX P=some x := by
  obtain ⟨y,hy⟩ := exists_curve_y W x
  have hp := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hy
  exact ⟨.some x y hp,rfl⟩

theorem two_point_recurrence [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) {k : ℕ} (xs : Fin k→K) (x y : K) :
    ManyPoints W (Fin.snoc (Fin.snoc xs x) y) ↔
      (∃ u : K,ManyPoints W (Fin.snoc xs u) ∧ generalS3 W x y u=0) ∨
      (ManyPoints W xs ∧ x=y) := by
  constructor
  · intro h
    obtain ⟨P,Q,R,hP,hQ,hR,hs⟩ := (manyPoints_two_snoc_iff W xs x y).mp h
    let A : W.Point := ∑ i,P i
    change A+Q+R=0 at hs
    by_cases hA : A=0
    · have hQR : Q+R=0 := by simpa only [hA,zero_add] using hs
      have hneg : Q= -R := eq_neg_iff_add_eq_zero.mpr hQR
      have hxy : x=y := Option.some.inj (hQ.symm.trans
        ((congrArg (fun T : W.Point=>pointX T) hneg).trans
          ((TheoremAudit.pointX_neg W R).trans hR)))
      exact Or.inr ⟨⟨P,hP,hA⟩,hxy⟩
    · obtain ⟨u,hu⟩ := pointX_exists_of_ne_zero W (-A) (neg_ne_zero.mpr hA)
      have hpref : ManyPoints W (Fin.snoc xs u) :=
        (manyPoints_snoc_iff W xs u).mpr ⟨P,-A,hP,hu,add_neg_cancel A⟩
      have hlast : generalS3 W x y u=0 := by
        apply (generalS3_zero_iff_exists_points W hΔ x y u).mpr
        refine ⟨Q,R,A,hQ,hR,(TheoremAudit.pointX_neg W A).symm.trans hu,?_⟩
        calc
          Q+R+A=A+Q+R := by abel
          _=0 := hs
      exact Or.inl ⟨u,hpref,hlast⟩
  · rintro (⟨u,hpref,hlast⟩ | ⟨hpref,hxy⟩)
    · obtain ⟨P,U,hP,hU,hs⟩ := (manyPoints_snoc_iff W xs u).mp hpref
      obtain ⟨R,S,V,hR,hS,hV,hRS⟩ :=
        (generalS3_zero_iff_exists_points W hΔ x y u).mp hlast
      rcases same_pointX_eq_or_neg W (hU.trans hV.symm) with heq | heq
      · apply (manyPoints_two_snoc_iff W xs x y).mpr
        refine ⟨P,-R,-S,hP,(TheoremAudit.pointX_neg W R).trans hR,
          (TheoremAudit.pointX_neg W S).trans hS,?_⟩
        calc
          (∑ i,P i)+ -R+ -S=((∑ i,P i)+U)-(R+S+V) := by rw [heq]; abel
          _=0 := by rw [hs,hRS]; simp
      · apply (manyPoints_two_snoc_iff W xs x y).mpr
        refine ⟨P,R,S,hP,hR,hS,?_⟩
        calc
          (∑ i,P i)+R+S=((∑ i,P i)+U)+(R+S+V) := by rw [heq]; abel
          _=0 := by rw [hs,hRS]; simp
    · obtain ⟨P,hP,hs⟩ := hpref
      obtain ⟨R,hR⟩ := exists_point_at_x W hΔ x
      apply (manyPoints_two_snoc_iff W xs x y).mpr
      refine ⟨P,R,-R,hP,hR,(TheoremAudit.pointX_neg W R).trans
        (hR.trans (congrArg some hxy)),?_⟩
      rw [hs,zero_add,add_neg_cancel]

theorem actual_point_list_recurrence_audit_certificate :
    ∀ (F : Type*) [Field F] [DecidableEq F] [IsAlgClosed F]
      (W : WeierstrassCurve.Affine F),W.Δ≠0 →
      ∀ (k : ℕ) (xs : Fin k→F) (x y : F),
        ManyPoints W (Fin.snoc (Fin.snoc xs x) y) ↔
          (∃ u : F,ManyPoints W (Fin.snoc xs u) ∧ generalS3 W x y u=0) ∨
          (ManyPoints W xs ∧ x=y) := by
  intro F _ _ _ W hΔ k xs x y
  exact two_point_recurrence W hΔ xs x y

theorem manyPoints_empty (W : WeierstrassCurve.Affine K) :
    ManyPoints W (Fin.elim0 : Fin 0→K) := by
  refine ⟨(fun i=>Fin.elim0 i),(fun i=>Fin.elim0 i),?_⟩
  simp

theorem append_opposite_pair (W : WeierstrassCurve.Affine K) {k : ℕ}
    (xs : Fin k→K) (h : ManyPoints W xs) (P : W.Point) (x : K)
    (hP : pointX P=some x) : ManyPoints W (Fin.snoc (Fin.snoc xs x) x) := by
  obtain ⟨Q,hQ,hs⟩ := h
  apply (manyPoints_two_snoc_iff W xs x x).mpr
  refine ⟨Q,P,-P,hQ,hP,(TheoremAudit.pointX_neg W P).trans hP,?_⟩
  rw [hs,zero_add,add_neg_cancel]

private instance primeFiveControl : Fact (Nat.Prime 5) := ⟨by decide⟩

private def finiteBasePoint : SkepticalAudit.rationalCurve.Point := .some 4 0
  ((WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
    SkepticalAudit.rationalCurve_nonsingular).mp
    (by rw [WeierstrassCurve.Affine.equation_iff]; decide +kernel))

def finitePrefix : Fin 4→ZMod 5 :=
  Fin.snoc (Fin.snoc (Fin.snoc (Fin.snoc (Fin.elim0 : Fin 0→ZMod 5) 4) 4) 4) 4

theorem finite_prefix_exists : ManyPoints SkepticalAudit.rationalCurve finitePrefix :=
  append_opposite_pair SkepticalAudit.rationalCurve _
    (append_opposite_pair SkepticalAudit.rationalCurve _
      (manyPoints_empty SkepticalAudit.rationalCurve) finiteBasePoint 4 rfl)
    finiteBasePoint 4 rfl

theorem finite_no_pair :
    ¬ ManyPoints SkepticalAudit.rationalCurve (Fin.snoc (Fin.snoc finitePrefix 3) 3) := by
  intro h
  obtain ⟨_,Q,_,_,hQ,_,_⟩ :=
    (manyPoints_two_snoc_iff SkepticalAudit.rationalCurve finitePrefix 3 3).mp h
  exact SkepticalAudit.no_rational_point_above_three ⟨Q,hQ⟩

theorem closure_free_recurrence_false :
    ¬ ∀ (F : Type) [Field F] [DecidableEq F] (W : WeierstrassCurve.Affine F),
      W.Δ≠0 → ∀ (k : ℕ) (xs : Fin k→F) (x y : F),
      (ManyPoints W (Fin.snoc (Fin.snoc xs x) y) ↔
        (∃ u : F,ManyPoints W (Fin.snoc xs u) ∧ generalS3 W x y u=0) ∨
        (ManyPoints W xs ∧ x=y)) := by
  intro h
  have he := h (ZMod 5) SkepticalAudit.rationalCurve
    SkepticalAudit.rationalCurve_nonsingular 4 finitePrefix 3 3
  exact finite_no_pair (he.mpr (Or.inr ⟨finite_prefix_exists,rfl⟩))

theorem point_list_domain_audit_certificate :
    (∀ (F : Type*) [Field F] [DecidableEq F] [IsAlgClosed F]
      (W : WeierstrassCurve.Affine F),W.Δ≠0 →
      ∀ (k : ℕ) (xs : Fin k→F) (x y : F),
        ManyPoints W (Fin.snoc (Fin.snoc xs x) y) ↔
          (∃ u : F,ManyPoints W (Fin.snoc xs u) ∧ generalS3 W x y u=0) ∨
          (ManyPoints W xs ∧ x=y)) ∧
    (¬ ∀ (F : Type) [Field F] [DecidableEq F] (W : WeierstrassCurve.Affine F),
      W.Δ≠0 → ∀ (k : ℕ) (xs : Fin k→F) (x y : F),
      (ManyPoints W (Fin.snoc (Fin.snoc xs x) y) ↔
        (∃ u : F,ManyPoints W (Fin.snoc xs u) ∧ generalS3 W x y u=0) ∨
        (ManyPoints W xs ∧ x=y))) := by
  exact ⟨actual_point_list_recurrence_audit_certificate,closure_free_recurrence_false⟩

#print axioms point_list_domain_audit_certificate
#print axioms closure_free_recurrence_false
#print axioms finite_prefix_exists
#print axioms finite_no_pair
#print axioms actual_point_list_recurrence_audit_certificate
#print axioms two_point_recurrence
#print axioms manyPoints_snoc_iff
#print axioms manyPoints_two_snoc_iff
#print axioms exists_point_at_x
end
end SemaevPointListsAudit
