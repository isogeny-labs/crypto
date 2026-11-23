import CM8Dual
import PhaseRecovery
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
  rw [smul_comm,hB i]
  exact zsmul_zero (v i)

theorem forward_sum {b : ℕ} (B : Fin b → E.Point) (v : Coeff b) :
    phi (∑ i, v i • B i)=∑ i, v i • phi (B i) := by
  change CM8Dual.forwardHom (∑ i, v i • B i)=_
  simp only [map_sum,map_zsmul]
  rfl

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
  Module.finrank (ZMod 7) ↥(L ⊔ rowSpan R)-Module.finrank (ZMod 7) ↥L

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

/-- Public setup logs and an actual accepted shifted relation recover the
original main scalar. The log values are hypotheses on public base points,
not a definition or lookup of the unknown target logarithm. -/
theorem actual_signed_known_log_recovery {b : ℕ} (A : Fin b → T.Point)
    (Q : E.Point) (hQ : (7:ℕ) • Q=0) (v logs : Coeff b) (shift : ℤ)
    (hlogs : ∀ i, A i=logs i • U)
    (hrel : ∑ i, v i • A i=phi Q+shift • U) :
    Q=((∑ i, v i*logs i)-shift) • P := by
  have hI : inverseHom U=P := by
    change CM8Dual.inverse7 U=P
    rw [← generator_image]
    exact CM8Dual.inverse7_left P source_order.1
  have hh := congrArg inverseHom hrel
  simp only [map_sum,map_add,map_zsmul] at hh
  change (∑ i, v i • inverseHom (A i))=
    CM8Dual.inverse7 (phi Q)+shift • inverseHom U at hh
  rw [CM8Dual.inverse7_left Q hQ] at hh
  have hs : (∑ i, v i • inverseHom (A i))=(∑ i, v i*logs i) • P := by
    calc
      _ = ∑ i, (v i*logs i) • P := by
        apply Finset.sum_congr rfl
        intro i hi
        rw [hlogs i,map_zsmul,hI,mul_zsmul]
      _ = _ := by
        have hsum : ∀ S : Finset (Fin b),
            S.sum (fun i => (v i*logs i) • P)=(S.sum (fun i => v i*logs i)) • P := by
          intro S
          induction S using Finset.induction_on with
          | empty => simp only [Finset.sum_empty,zero_zsmul]
          | @insert i S hi ih =>
            simp only [Finset.sum_insert hi,add_zsmul,ih]
        exact hsum Finset.univ
  rw [hs,hI] at hh
  rw [sub_zsmul,hh]
  abel

theorem actual_signed_known_log_recovery_mod7 {b : ℕ} (A : Fin b → T.Point)
    (Q : E.Point) (hQ : (7:ℕ) • Q=0) (v logs : Coeff b) (shift : ℤ)
    (hlogs : ∀ i, A i=logs i • U)
    (hrel : ∑ i, v i • A i=phi Q+shift • U) :
    Q=(((∑ i, v i*logs i)-shift)%7) • P := by
  have hP : (7:ℤ) • P=0 := by decide +kernel
  exact PhaseRecovery.annihilator_mod P Q ((∑ i, v i*logs i)-shift) 7 hP
    (actual_signed_known_log_recovery A Q hQ v logs shift hlogs hrel)

#print axioms actual_signed_known_log_recovery
#print axioms actual_signed_known_log_recovery_mod7

/-- One actual relation-collection step under a supplied seven-torsion query
rule. Different degree-admitted oracles are deliberately not identified here. -/
noncomputable def mainStep {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (query : Submodule (ZMod 7) (Vec7 b) → E.Point)
    (L : Submodule (ZMod 7) (Vec7 b)) : Submodule (ZMod 7) (Vec7 b) :=
  L ⊔ rowSpan (mainRows V B (query L))
noncomputable def twistStep {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (query : Submodule (ZMod 7) (Vec7 b) → E.Point)
    (L : Submodule (ZMod 7) (Vec7 b)) : Submodule (ZMod 7) (Vec7 b) :=
  L ⊔ rowSpan (twistRows V B (query L))

theorem actual_adaptive_steps_equal {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (query : Submodule (ZMod 7) (Vec7 b) → E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hquery : ∀ L, (7:ℕ) • query L=0) :
    twistStep V B query=mainStep V B query := by
  funext L
  unfold twistStep mainStep
  rw [actual_coefficient_spans_equal V B (query L) hB (hquery L)]

theorem actual_adaptive_states_equal {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (query : Submodule (ZMod 7) (Vec7 b) → E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hquery : ∀ L, (7:ℕ) • query L=0)
    (L : Submodule (ZMod 7) (Vec7 b)) (n : ℕ) :
    (twistStep V B query)^[n] L=(mainStep V B query)^[n] L := by
  rw [actual_adaptive_steps_equal V B query hB hquery]

theorem actual_adaptive_rank_progress_equal {b : ℕ} (V : Finset (Coeff b))
    (B : Fin b → E.Point) (query : Submodule (ZMod 7) (Vec7 b) → E.Point)
    (hB : ∀ i, (7:ℕ) • B i=0) (hquery : ∀ L, (7:ℕ) • query L=0)
    (L : Submodule (ZMod 7) (Vec7 b)) (n : ℕ) :
    Module.finrank (ZMod 7) ↥((twistStep V B query)^[n] L)=
      Module.finrank (ZMod 7) ↥((mainStep V B query)^[n] L) := by
  rw [actual_adaptive_states_equal V B query hB hquery L n]

#print axioms actual_adaptive_steps_equal
#print axioms actual_adaptive_states_equal
#print axioms actual_adaptive_rank_progress_equal

/-- Actual domain control: the forward map has nonzero rational 2-kernel. -/
theorem source_kernel_nonzero : K ≠ (0:E.Point) := by decide +kernel

theorem forward_not_globally_injective : ¬ Function.Injective phi := by
  intro h
  apply source_kernel_nonzero
  apply h
  simpa only [zero_image] using kernel_image

theorem source_kernel_not_seven : (7:ℕ) • K ≠ (0:E.Point) := by decide +kernel

#print axioms source_kernel_nonzero
#print axioms forward_not_globally_injective
#print axioms source_kernel_not_seven

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
