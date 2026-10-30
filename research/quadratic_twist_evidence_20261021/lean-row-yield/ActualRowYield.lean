import CM8Dual
import Mathlib

/-! Actual F13 relation rows. Complete matched-row equality does not imply
that different solver degree budgets admit the same subset. Degree profiles
below are supplied abstractions, not a proved native solver/regularity model. -/
namespace ActualRowYield
open CM8Transfer

abbrev Coeff (b : ℕ) := Fin b → ℤ
abbrev Vec7 (b : ℕ) := Fin b → ZMod 7

def reduce7 {b : ℕ} (v : Coeff b) : Vec7 b := fun i => v i

theorem seven_sum {b : ℕ} (B : Fin b → E.Point) (v : Coeff b)
    (hB : ∀ i, (7:ℕ) • B i=0) :
    (7:ℕ) • (∑ i, v i • B i)=0 := by
  rw [Finset.smul_sum]
  apply Finset.sum_eq_zero
  intro i hi
  rw [smul_comm]
  simp [hB i]

theorem forward_sum {b : ℕ} (B : Fin b → E.Point) (v : Coeff b) :
    phi (∑ i, v i • B i)=∑ i, v i • phi (B i) := by
  change CM8Dual.forwardHom (∑ i, v i • B i)=_
  simp only [map_sum,map_zsmul]

theorem forward_injective_on_seven (Q R : E.Point)
    (hQ : (7:ℕ) • Q=0) (hR : (7:ℕ) • R=0)
    (h : phi Q=phi R) : Q=R := by
  have hi := congrArg CM8Dual.inverse7 h
  rw [CM8Dual.inverse7_left Q hQ,CM8Dual.inverse7_left R hR] at hi
  exact hi

theorem actual_relation_iff {b : ℕ} (B : Fin b → E.Point) (Q : E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hQ : (7:ℕ) • Q=0) (v : Coeff b) :
    (∑ i, v i • phi (B i)=phi Q) ↔ (∑ i, v i • B i=Q) := by
  rw [← forward_sum]
  constructor
  · exact forward_injective_on_seven _ Q (seven_sum B v hB) hQ
  · intro h
    rw [h]

noncomputable def mainRows {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (Q : E.Point) : Finset (Coeff b) := by
  classical
  exact V.filter (fun v => ∑ i, v i • B i=Q)
noncomputable def twistRows {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (Q : E.Point) : Finset (Coeff b) := by
  classical
  exact V.filter (fun v => ∑ i, v i • phi (B i)=phi Q)

theorem actual_rows_equal {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (Q : E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hQ : (7:ℕ) • Q=0) :
    twistRows V B Q=mainRows V B Q := by
  classical
  unfold twistRows mainRows
  apply Finset.filter_congr
  intro v hv
  exact actual_relation_iff B Q hB hQ v

theorem actual_row_count_equal {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (Q : E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hQ : (7:ℕ) • Q=0) :
    (twistRows V B Q).card=(mainRows V B Q).card := by
  rw [actual_rows_equal V B Q hB hQ]

noncomputable def rowSpan {b : ℕ} (R : Finset (Coeff b)) :
    Submodule (ZMod 7) (Vec7 b) := by
  classical
  exact Submodule.span (ZMod 7) (↑(R.image reduce7) : Set (Vec7 b))
noncomputable def usefulRank {b : ℕ} (L : Submodule (ZMod 7) (Vec7 b))
    (R : Finset (Coeff b)) : ℕ :=
  Module.finrank (ZMod 7) (L ⊔ rowSpan R)-Module.finrank (ZMod 7) L

theorem actual_coefficient_spans_equal {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (Q : E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hQ : (7:ℕ) • Q=0) :
    rowSpan (twistRows V B Q)=rowSpan (mainRows V B Q) := by
  rw [actual_rows_equal V B Q hB hQ]

theorem actual_useful_ranks_equal {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (Q : E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hQ : (7:ℕ) • Q=0)
    (L : Submodule (ZMod 7) (Vec7 b)) :
    usefulRank L (twistRows V B Q)=usefulRank L (mainRows V B Q) := by
  unfold usefulRank
  rw [actual_coefficient_spans_equal V B Q hB hQ]

/-- Any admitted target subset gives valid original-source relations.
The subset's construction/degree/correctness must be justified separately. -/
theorem admitted_rows_are_main_valid {b : ℕ} (V A : Finset (Coeff b))
    (B : Fin b → E.Point) (Q : E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hQ : (7:ℕ) • Q=0)
    (hA : A ⊆ twistRows V B Q) : A ⊆ mainRows V B Q := by
  rwa [actual_rows_equal V B Q hB hQ] at hA

/-- Actual inverse is additive; no global injectivity of Phi is assumed. -/
def inverseHom : T.Point →+ E.Point where
  toFun := CM8Dual.inverse7
  map_zero' := by simp [CM8Dual.inverse7,CM8Dual.dual]
  map_add' := by
    intro A B
    simp only [CM8Dual.inverse7,CM8Dual.dual_add,nsmul_add]

theorem actual_target_aggregate_recovers {b : ℕ} (A : Fin b → T.Point)
    (Q : E.Point) (hQ : (7:ℕ) • Q=0) (v : Coeff b)
    (hrel : ∑ i, v i • A i=phi Q) :
    (∑ i, v i • CM8Dual.inverse7 (A i))=Q := by
  have h := congrArg inverseHom hrel
  simp only [map_sum,map_zsmul] at h
  change (∑ i, v i • CM8Dual.inverse7 (A i))=CM8Dual.inverse7 (phi Q) at h
  rw [CM8Dual.inverse7_left Q hQ] at h
  exact h

noncomputable def admitted {b : ℕ} (R : Finset (Coeff b))
    (degreeProfile : Coeff b → ℕ) (cap : ℕ) : Finset (Coeff b) := by
  classical
  exact R.filter (fun v => degreeProfile v ≤ cap)

theorem rowSpan_mono {b : ℕ} {A B : Finset (Coeff b)} (h : A ⊆ B) :
    rowSpan A ≤ rowSpan B := by
  classical
  apply Submodule.span_mono
  exact_mod_cast Finset.image_subset_image h

/-- Abstract admission compression hypothesis, not a theorem about regularity
or the native capped solver. Even a lower target cap still needs this bridge. -/
theorem qualified_span_inclusion {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (Q : E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hQ : (7:ℕ) • Q=0)
    (dE dT : Coeff b → ℕ) (capE capT : ℕ)
    (hcompress : ∀ v ∈ mainRows V B Q, dE v ≤ capE → dT v ≤ capT) :
    rowSpan (admitted (mainRows V B Q) dE capE) ≤
      rowSpan (admitted (twistRows V B Q) dT capT) := by
  classical
  apply rowSpan_mono
  rw [actual_rows_equal V B Q hB hQ]
  intro v hv
  simp only [admitted,Finset.mem_filter] at hv ⊢
  exact ⟨hv.1,hcompress v hv.1 hv.2⟩

#print axioms seven_sum
#print axioms forward_sum
#print axioms forward_injective_on_seven
#print axioms actual_relation_iff
#print axioms actual_rows_equal
#print axioms actual_row_count_equal
#print axioms actual_coefficient_spans_equal
#print axioms actual_useful_ranks_equal
#print axioms admitted_rows_are_main_valid
#print axioms actual_target_aggregate_recovers
#print axioms rowSpan_mono
#print axioms qualified_span_inclusion
end ActualRowYield
