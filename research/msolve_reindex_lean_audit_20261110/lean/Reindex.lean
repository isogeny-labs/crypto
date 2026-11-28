import Mathlib.Tactic

/-! Arithmetic and table semantics of the observed coefficient-index snapshot.
The snapshot is native evidence, not a Lean proof of C execution. The copied
table is defined by the three operational updates before its invariant is proved.
Live basis entries are distinguished from the possibly larger C allocation. -/
namespace MsolveReindexAudit

def sourceIndex (row : Fin 3) : Fin 3 := ![0,2,0] row
def destinationIndex (row : Fin 3) : ℕ := 2+row.val

def basisLookup {α : Type*} (basis : Fin 3→α) (index : ℕ) : Option α :=
  if h : index<3 then some (basis ⟨index,h⟩) else none

/-- The three actual-copy operations, starting with an empty matrix table. -/
def relocated {α : Type*} (basis : Fin 3→α) : ℕ→Option α :=
  ([0,1,2] : List (Fin 3)).foldl
    (fun table row=>Function.update table (destinationIndex row)
      (some (basis (sourceIndex row)))) (fun _=>none)

theorem source_and_destination_bounds (row : Fin 3) :
    (sourceIndex row).val<3 ∧ 2≤destinationIndex row ∧ destinationIndex row<5 := by
  have h:=row.isLt
  exact ⟨(sourceIndex row).isLt,by unfold destinationIndex;omega,
    by unfold destinationIndex;omega⟩

theorem copied_lookup_correct {α : Type*} (basis : Fin 3→α) (row : Fin 3) :
    relocated basis (destinationIndex row)=some (basis (sourceIndex row)) := by
  fin_cases row <;> simp [relocated,destinationIndex,sourceIndex,Function.update]

/-- Both failed live-basis lookups are absent for every possible coefficient
type and basis contents; no guessed coefficient values are required. -/
theorem observed_live_index_failures {α : Type*} (basis : Fin 3→α) :
    sourceIndex 1=2 ∧ destinationIndex 1=3 ∧
    basisLookup basis (destinationIndex 1)=none ∧
    relocated basis (destinationIndex 1)=some (basis 2) ∧
    basisLookup basis (destinationIndex 2)=none ∧
    relocated basis (destinationIndex 2)=some (basis 0) := by
  simp [sourceIndex,destinationIndex,basisLookup,relocated,Function.update]

theorem observed_lookup_disagrees {α : Type*} (basis : Fin 3→α) :
    basisLookup basis (destinationIndex 1)≠relocated basis (destinationIndex 1) := by
  simp [destinationIndex,basisLookup,relocated,sourceIndex,Function.update]

/-- Even an in-range rewritten index need not denote the original row. -/
theorem in_range_lookup_can_change_coefficients :
    destinationIndex 0<3 ∧
    basisLookup (fun i : Fin 3=>i.val) (destinationIndex 0)=some 2 ∧
    relocated (fun i : Fin 3=>i.val) (destinationIndex 0)=some 0 := by
  decide

theorem universal_basis_reindex_invariant_false :
    ¬∀ (basis : Fin 3→ℕ) (row : Fin 3),
      basisLookup basis (destinationIndex row)=relocated basis (destinationIndex row) := by
  intro h
  exact observed_lookup_disagrees (fun i : Fin 3=>i.val) (h _ 1)

def signed32Representable (z : ℤ) : Prop := -(2^31)≤z ∧ z<2^31

/-- Integer representability only; no C expression parser/semantics is assumed. -/
theorem signed_shift_boundary : ¬signed32Representable (2^31) := by
  simp [signed32Representable]

theorem unsigned_bit_fits (ctr : ℕ) (hctr : ctr<32) : (2:ℕ)^ctr<2^32 := by
  exact Nat.pow_lt_pow_right (by decide) hctr

theorem reindex_audit_certificate :
    (∀ (basis : Fin 3→ℕ) (row : Fin 3),
      relocated basis (destinationIndex row)=some (basis (sourceIndex row))) ∧
    (∀ basis : Fin 3→ℕ,
      basisLookup basis (destinationIndex 1)≠relocated basis (destinationIndex 1)) ∧
    (¬∀ (basis : Fin 3→ℕ) (row : Fin 3),
      basisLookup basis (destinationIndex row)=relocated basis (destinationIndex row)) ∧
    ¬signed32Representable (2^31) ∧
    (∀ ctr : ℕ,ctr<32 → (2:ℕ)^ctr<2^32) := by
  exact ⟨copied_lookup_correct,observed_lookup_disagrees,
    universal_basis_reindex_invariant_false,signed_shift_boundary,unsigned_bit_fits⟩

#print axioms copied_lookup_correct
#print axioms reindex_audit_certificate
end MsolveReindexAudit
