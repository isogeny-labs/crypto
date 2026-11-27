module

public import ObjectiveBoundary
public import SparseLabelScore

@[expose] public section

/-!
# Executable packing-label artifact

This module imports the compact projection of the native schema-3 packing
provenance fixture.  It checks the projection's shape and bounds, then replays
the exact row-then-column strict-first-minimum schedule from `LabelSequence`.
-/

namespace Curve25519PackingLabelArtifact

open Curve25519LabelSequence
open Curve25519ConditionalObjective
open Curve25519ObjectiveBoundary
open Curve25519SparseLabelScore

/-- Shared native projection: schema/radix/context, endpoint quadruples, then
row and column labels. -/
def packingLabelWitnessArtifact : String :=
  include_str ".." / "packing_label_witness.csv"

/-- Shared native projection at the exact strongest source-class boundary. -/
def packingLabelSourceGuardWitnessArtifact : String :=
  include_str ".." / "packing_label_source_guard_witness.csv"

/-- Transparent decimal digit parser for the shared projection. -/
def packingLabelWitnessDigit : Char → Option ℕ
  | '0' => some 0
  | '1' => some 1
  | '2' => some 2
  | '3' => some 3
  | '4' => some 4
  | '5' => some 5
  | '6' => some 6
  | '7' => some 7
  | '8' => some 8
  | '9' => some 9
  | _ => none

/-- Transparent comma-separated decimal parser; the final newline is required. -/
def parsePackingLabelWitness : List Char → ℕ → List ℕ → Option (List ℕ)
  | [], _, _ => none
  | '\n' :: [], current, values => some ((current :: values).reverse)
  | ',' :: rest, current, values =>
      parsePackingLabelWitness rest 0 (current :: values)
  | character :: rest, current, values => do
      let digit ← packingLabelWitnessDigit character
      parsePackingLabelWitness rest (10 * current + digit) values

/-- Parse the shared projection without an opaque runtime parser. -/
def packingLabelWitnessNumbers : List ℕ :=
  (parsePackingLabelWitness packingLabelWitnessArtifact.toList 0 []).getD []

/-- Parse the exact source-class-boundary projection transparently. -/
def packingLabelSourceGuardWitnessNumbers : List ℕ :=
  (parsePackingLabelWitness
    packingLabelSourceGuardWitnessArtifact.toList 0 []).getD []

/-- Named representation of the compact native projection. -/
structure PackingLabelArtifact where
  schema : ℕ
  radix : ℕ
  packetExponent : ℕ
  coordinate : ℕ
  conditionalLabels : ℕ
  sourceDegreeCap : ℕ
  edgeCount : ℕ
  rowCount : ℕ
  columnCount : ℕ
  edgeFields : List ℕ
  rowLabels : List ℕ
  columnLabels : List ℕ
  deriving DecidableEq, Inhabited, Repr

/-- Number of native labels, `radix^packetExponent`. -/
def PackingLabelArtifact.labelCount (artifact : PackingLabelArtifact) : ℕ :=
  artifact.radix ^ artifact.packetExponent

/-- Left endpoint index of an edge in the interleaved endpoint quadruples. -/
def PackingLabelArtifact.leftIndex
    (artifact : PackingLabelArtifact) (edge : ℕ) : ℕ :=
  artifact.edgeFields.getD (4 * edge) 0

/-- Right endpoint index of an edge in the interleaved endpoint quadruples. -/
def PackingLabelArtifact.rightIndex
    (artifact : PackingLabelArtifact) (edge : ℕ) : ℕ :=
  artifact.edgeFields.getD (4 * edge + 2) 0

/-- Exact field lengths and the schema-3 conditional-label context. -/
def PackingLabelArtifact.wellFormed (artifact : PackingLabelArtifact) : Bool :=
  artifact.schema == 3 &&
    artifact.conditionalLabels == 1 &&
    artifact.edgeFields.length == 4 * artifact.edgeCount &&
    artifact.rowLabels.length == artifact.rowCount &&
    artifact.columnLabels.length == artifact.columnCount &&
    decide (0 < artifact.radix ∧ artifact.coordinate < artifact.packetExponent)

/-- Endpoint and label fields lie inside their serialized array bounds. -/
def PackingLabelArtifact.bounded (artifact : PackingLabelArtifact) : Bool :=
  (List.range artifact.edgeCount).all (fun edge =>
      decide (artifact.leftIndex edge < artifact.rowCount) &&
        decide (artifact.rightIndex edge < artifact.columnCount)) &&
    artifact.rowLabels.all (fun label => decide (label < artifact.labelCount)) &&
    artifact.columnLabels.all (fun label => decide (label < artifact.labelCount))

/-- Split the flat native projection into its named arrays. -/
def parsePackingLabelArtifactUnchecked : List ℕ → Option PackingLabelArtifact
  | schema :: radix :: packetExponent :: coordinate :: conditionalLabels ::
      sourceDegreeCap :: edgeCount :: rowCount :: columnCount :: fields => do
      let (edgeFields, afterEdges) := fields.splitAt (4 * edgeCount)
      let (rowLabels, columnLabels) := afterEdges.splitAt rowCount
      some {
        schema, radix, packetExponent, coordinate, conditionalLabels,
        sourceDegreeCap, edgeCount, rowCount, columnCount, edgeFields,
        rowLabels, columnLabels
      }
  | _ => none

/-- Shape- and bounds-checked parser for the native projection. -/
def parsePackingLabelArtifact (values : List ℕ) : Option PackingLabelArtifact := do
  let artifact ← parsePackingLabelArtifactUnchecked values
  if artifact.wellFormed && artifact.bounded then some artifact else none

private theorem parsePackingLabelArtifact_components
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact) :
    artifact.wellFormed = true ∧ artifact.bounded = true := by
  unfold parsePackingLabelArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with ⟨candidate, _, hchecked⟩
  split at hchecked
  next hconditions =>
    have hartifact : candidate = artifact := Option.some.inj hchecked
    subst candidate
    simpa only [Bool.and_eq_true] using hconditions
  next => simp at hchecked

/-- A shape-checked import retains the exact schema and array-length checks. -/
theorem parsePackingLabelArtifact_wellFormed
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact) :
    artifact.wellFormed = true :=
  (parsePackingLabelArtifact_components hparse).1

/-- A checked projection has the same nonzero-radix and active-coordinate
context required by the native provenance verifier. -/
theorem parsePackingLabelArtifact_context
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact) :
    0 < artifact.radix ∧ artifact.coordinate < artifact.packetExponent := by
  have hwell := parsePackingLabelArtifact_wellFormed hparse
  simp only [PackingLabelArtifact.wellFormed, Bool.and_eq_true,
    beq_iff_eq, decide_eq_true_eq] at hwell
  exact hwell.2

/-- A shape-checked import retains every endpoint and label bound. -/
theorem parsePackingLabelArtifact_bounded
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact) :
    artifact.bounded = true :=
  (parsePackingLabelArtifact_components hparse).2

/-- Typed endpoint map used by the formal objective. -/
def PackingLabelArtifact.leftEndpoint
    (artifact : PackingLabelArtifact) (hrows : 0 < artifact.rowCount) :
    Fin artifact.edgeCount → Fin artifact.rowCount :=
  fun edge =>
    ⟨artifact.leftIndex edge.val % artifact.rowCount,
      Nat.mod_lt _ hrows⟩

/-- Typed right-endpoint map used by the formal objective. -/
def PackingLabelArtifact.rightEndpoint
    (artifact : PackingLabelArtifact) (hcolumns : 0 < artifact.columnCount) :
    Fin artifact.edgeCount → Fin artifact.columnCount :=
  fun edge =>
    ⟨artifact.rightIndex edge.val % artifact.columnCount,
      Nat.mod_lt _ hcolumns⟩

/-- Typed serialized row label. -/
def PackingLabelArtifact.rowLabel
    (artifact : PackingLabelArtifact) (hlabels : 0 < artifact.labelCount) :
    Fin artifact.rowCount → Fin artifact.labelCount :=
  fun row =>
    ⟨artifact.rowLabels.getD row.val 0 % artifact.labelCount,
      Nat.mod_lt _ hlabels⟩

/-- Typed serialized column label. -/
def PackingLabelArtifact.columnLabel
    (artifact : PackingLabelArtifact) (hlabels : 0 < artifact.labelCount) :
    Fin artifact.columnCount → Fin artifact.labelCount :=
  fun column =>
    ⟨artifact.columnLabels.getD column.val 0 % artifact.labelCount,
      Nat.mod_lt _ hlabels⟩

/-- A parsed typed left endpoint is the exact serialized index, not merely its
reduction modulo the row count. -/
theorem parsePackingLabelArtifact_leftEndpoint_val
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact)
    (hrows : 0 < artifact.rowCount) (edge : Fin artifact.edgeCount) :
    (artifact.leftEndpoint hrows edge).val = artifact.leftIndex edge.val := by
  have hbounded := parsePackingLabelArtifact_bounded hparse
  simp only [PackingLabelArtifact.bounded, Bool.and_eq_true,
    List.all_eq_true, decide_eq_true_eq] at hbounded
  have hleft := (hbounded.1.1 edge.val (by simp)).1
  simp [PackingLabelArtifact.leftEndpoint, Nat.mod_eq_of_lt hleft]

/-- A parsed typed right endpoint is the exact serialized index, not merely
its reduction modulo the column count. -/
theorem parsePackingLabelArtifact_rightEndpoint_val
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact)
    (hcolumns : 0 < artifact.columnCount) (edge : Fin artifact.edgeCount) :
    (artifact.rightEndpoint hcolumns edge).val = artifact.rightIndex edge.val := by
  have hbounded := parsePackingLabelArtifact_bounded hparse
  simp only [PackingLabelArtifact.bounded, Bool.and_eq_true,
    List.all_eq_true, decide_eq_true_eq] at hbounded
  have hright := (hbounded.1.1 edge.val (by simp)).2
  simp [PackingLabelArtifact.rightEndpoint, Nat.mod_eq_of_lt hright]

/-- A parsed typed row label is the exact serialized natural value, not merely
its reduction modulo the label count. -/
theorem parsePackingLabelArtifact_rowLabel_val
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact)
    (hlabels : 0 < artifact.labelCount) (row : Fin artifact.rowCount) :
    (artifact.rowLabel hlabels row).val =
      artifact.rowLabels.getD row.val 0 := by
  have hwell := parsePackingLabelArtifact_wellFormed hparse
  simp only [PackingLabelArtifact.wellFormed, Bool.and_eq_true,
    beq_iff_eq] at hwell
  have hrowLength : artifact.rowLabels.length = artifact.rowCount :=
    hwell.1.1.2
  have hindex : row.val < artifact.rowLabels.length := by
    simpa only [hrowLength] using row.isLt
  have hbounded := parsePackingLabelArtifact_bounded hparse
  simp only [PackingLabelArtifact.bounded, Bool.and_eq_true,
    List.all_eq_true, decide_eq_true_eq] at hbounded
  have hlabel := hbounded.1.2
    artifact.rowLabels[row.val]
    (List.mem_of_getElem rfl)
  rw [List.getD_eq_getElem artifact.rowLabels 0 hindex]
  simp [PackingLabelArtifact.rowLabel, hindex, Nat.mod_eq_of_lt hlabel]

/-- A parsed typed column label is the exact serialized natural value, not
merely its reduction modulo the label count. -/
theorem parsePackingLabelArtifact_columnLabel_val
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact)
    (hlabels : 0 < artifact.labelCount) (column : Fin artifact.columnCount) :
    (artifact.columnLabel hlabels column).val =
      artifact.columnLabels.getD column.val 0 := by
  have hwell := parsePackingLabelArtifact_wellFormed hparse
  simp only [PackingLabelArtifact.wellFormed, Bool.and_eq_true,
    beq_iff_eq] at hwell
  have hcolumnLength :
      artifact.columnLabels.length = artifact.columnCount :=
    hwell.1.2
  have hindex : column.val < artifact.columnLabels.length := by
    simpa only [hcolumnLength] using column.isLt
  have hbounded := parsePackingLabelArtifact_bounded hparse
  simp only [PackingLabelArtifact.bounded, Bool.and_eq_true,
    List.all_eq_true, decide_eq_true_eq] at hbounded
  have hlabel := hbounded.2
    artifact.columnLabels[column.val]
    (List.mem_of_getElem rfl)
  rw [List.getD_eq_getElem artifact.columnLabels 0 hindex]
  simp [PackingLabelArtifact.columnLabel, hindex,
    Nat.mod_eq_of_lt hlabel]

/-- Replay each exact row-then-column first-minimum step from the serialized
prefix. Positivity is checked before dependent finite types are formed. -/
def PackingLabelArtifact.traceMatches (artifact : PackingLabelArtifact) : Bool :=
  if hlabels : 0 < artifact.labelCount then
    if hrows : 0 < artifact.rowCount then
      if hcolumns : 0 < artifact.columnCount then
        (List.finRange artifact.rowCount).all (fun row =>
          decide (artifact.rowLabel hlabels row =
            firstMinimumLabel hlabels
              (rowCandidateScore artifact.labelCount
                (artifact.leftEndpoint hrows) (artifact.rightEndpoint hcolumns)
                (prefixAssignment (artifact.rowLabel hlabels) row.val)
                (fun _ => none) row))) &&
        (List.finRange artifact.columnCount).all (fun column =>
          decide (artifact.columnLabel hlabels column =
            firstMinimumLabel hlabels
              (columnCandidateScore artifact.labelCount
                (artifact.leftEndpoint hrows) (artifact.rightEndpoint hcolumns)
                (fun row => some (artifact.rowLabel hlabels row))
                (prefixAssignment (artifact.columnLabel hlabels) column.val)
                column)))
      else false
    else false
  else false

/-- Fail closed unless the checked native projection equals the executable
formal first-minimum trace. -/
def parseReplayedPackingLabelArtifact
    (values : List ℕ) : Option PackingLabelArtifact := do
  let artifact ← parsePackingLabelArtifact values
  if artifact.traceMatches then some artifact else none

/-- Recompute the two serialized endpoint degree caps and the strongest
source-class guard needed by the formal class-cap theorem. -/
def PackingLabelArtifact.sourceClassGuardMatches
    (artifact : PackingLabelArtifact) : Bool :=
  if hrows : 0 < artifact.rowCount then
    if hcolumns : 0 < artifact.columnCount then
      (List.finRange artifact.rowCount).all (fun row =>
        decide (fiberSize (artifact.leftEndpoint hrows) row ≤
          artifact.sourceDegreeCap)) &&
      (List.finRange artifact.columnCount).all (fun column =>
        decide (fiberSize (artifact.rightEndpoint hcolumns) column ≤
          artifact.sourceDegreeCap)) &&
      decide (2 * (artifact.labelCount * artifact.labelCount) ^ 2 *
        artifact.sourceDegreeCap ≤ artifact.edgeCount)
    else false
  else false

/-- Fail closed unless a replayed projection also discharges its serialized
endpoint degree caps and strongest source-class guard. -/
def parseSourceGuardPackingLabelArtifact
    (values : List ℕ) : Option PackingLabelArtifact := do
  let artifact ← parseReplayedPackingLabelArtifact values
  if artifact.sourceClassGuardMatches then some artifact else none

private theorem parseReplayedPackingLabelArtifact_components
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseReplayedPackingLabelArtifact values = some artifact) :
    parsePackingLabelArtifact values = some artifact ∧
      artifact.traceMatches = true := by
  unfold parseReplayedPackingLabelArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with ⟨candidate, hchecked, htrace⟩
  split at htrace
  next hmatches =>
    have hartifact : candidate = artifact := Option.some.inj htrace
    subst candidate
    exact ⟨hchecked, hmatches⟩
  next => simp at htrace

private theorem parseSourceGuardPackingLabelArtifact_components
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseSourceGuardPackingLabelArtifact values = some artifact) :
    parseReplayedPackingLabelArtifact values = some artifact ∧
      artifact.sourceClassGuardMatches = true := by
  unfold parseSourceGuardPackingLabelArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with ⟨candidate, hreplayed, hguard⟩
  split at hguard
  next hmatches =>
    have hartifact : candidate = artifact := Option.some.inj hguard
    subst candidate
    exact ⟨hreplayed, hmatches⟩
  next => simp at hguard

/-- A source-guard import retains the complete executable label replay. -/
theorem parseSourceGuardPackingLabelArtifact_replayed
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseSourceGuardPackingLabelArtifact values = some artifact) :
    parseReplayedPackingLabelArtifact values = some artifact :=
  (parseSourceGuardPackingLabelArtifact_components hparse).1

/-- Every replayed import has a nonempty label alphabet. -/
theorem parseReplayedPackingLabelArtifact_labelCount_pos
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseReplayedPackingLabelArtifact values = some artifact) :
    0 < artifact.labelCount := by
  have htrace := (parseReplayedPackingLabelArtifact_components hparse).2
  by_contra hlabels
  simp [PackingLabelArtifact.traceMatches, hlabels] at htrace

/-- Every replayed import has a nonempty row array. -/
theorem parseReplayedPackingLabelArtifact_rowCount_pos
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseReplayedPackingLabelArtifact values = some artifact) :
    0 < artifact.rowCount := by
  have htrace := (parseReplayedPackingLabelArtifact_components hparse).2
  have hlabels := parseReplayedPackingLabelArtifact_labelCount_pos hparse
  by_contra hrows
  simp [PackingLabelArtifact.traceMatches, hlabels, hrows] at htrace

/-- Every replayed import has a nonempty column array. -/
theorem parseReplayedPackingLabelArtifact_columnCount_pos
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseReplayedPackingLabelArtifact values = some artifact) :
    0 < artifact.columnCount := by
  have htrace := (parseReplayedPackingLabelArtifact_components hparse).2
  have hlabels := parseReplayedPackingLabelArtifact_labelCount_pos hparse
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hparse
  by_contra hcolumns
  simp [PackingLabelArtifact.traceMatches, hlabels, hrows, hcolumns] at htrace

/-- A replayed import's row array is exactly the executable native-order row
trace. -/
theorem parseReplayedPackingLabelArtifact_rows
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseReplayedPackingLabelArtifact values = some artifact)
    (row : Fin artifact.rowCount) :
    artifact.rowLabel (parseReplayedPackingLabelArtifact_labelCount_pos hparse) row =
      executableRowLabels
        (parseReplayedPackingLabelArtifact_labelCount_pos hparse)
        (artifact.leftEndpoint
          (parseReplayedPackingLabelArtifact_rowCount_pos hparse))
        (artifact.rightEndpoint
          (parseReplayedPackingLabelArtifact_columnCount_pos hparse)) row := by
  rw [executableRowLabels_eq, firstMinimumRowLabel_eq_sparseScore]
  have htrace := (parseReplayedPackingLabelArtifact_components hparse).2
  have hlabels := parseReplayedPackingLabelArtifact_labelCount_pos hparse
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hparse
  have hcolumns := parseReplayedPackingLabelArtifact_columnCount_pos hparse
  simp only [PackingLabelArtifact.traceMatches, hlabels, hrows, hcolumns,
    ↓reduceDIte, Bool.and_eq_true, List.all_eq_true] at htrace
  have hlocal := of_decide_eq_true (htrace.1 row (by simp))
  have hprefix :
      prefixAssignment (artifact.rowLabel hlabels) row.val =
        prefixAssignment
          (executableRowLabels hlabels (artifact.leftEndpoint hrows)
            (artifact.rightEndpoint hcolumns)) row.val := by
    funext vertex
    simp only [prefixAssignment]
    split_ifs with hbefore
    · congr 1
      exact parseReplayedPackingLabelArtifact_rows hparse vertex
    · rfl
  rw [hprefix] at hlocal
  exact hlocal
termination_by row.val

/-- A replayed import's column array is exactly the executable native-order
column trace after fixing the replayed row trace. -/
theorem parseReplayedPackingLabelArtifact_columns
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseReplayedPackingLabelArtifact values = some artifact)
    (column : Fin artifact.columnCount) :
    artifact.columnLabel
        (parseReplayedPackingLabelArtifact_labelCount_pos hparse) column =
      executableColumnLabels
        (parseReplayedPackingLabelArtifact_labelCount_pos hparse)
        (artifact.leftEndpoint
          (parseReplayedPackingLabelArtifact_rowCount_pos hparse))
        (artifact.rightEndpoint
          (parseReplayedPackingLabelArtifact_columnCount_pos hparse))
        (executableRowLabels
          (parseReplayedPackingLabelArtifact_labelCount_pos hparse)
          (artifact.leftEndpoint
            (parseReplayedPackingLabelArtifact_rowCount_pos hparse))
          (artifact.rightEndpoint
            (parseReplayedPackingLabelArtifact_columnCount_pos hparse))) column := by
  rw [executableColumnLabels_eq, firstMinimumColumnLabel_eq_sparseScore]
  have htrace := (parseReplayedPackingLabelArtifact_components hparse).2
  have hlabels := parseReplayedPackingLabelArtifact_labelCount_pos hparse
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hparse
  have hcolumns := parseReplayedPackingLabelArtifact_columnCount_pos hparse
  simp only [PackingLabelArtifact.traceMatches, hlabels, hrows, hcolumns,
    ↓reduceDIte, Bool.and_eq_true, List.all_eq_true] at htrace
  have hlocal := of_decide_eq_true (htrace.2 column (by simp))
  have hrowLabels :
      artifact.rowLabel hlabels =
        executableRowLabels hlabels (artifact.leftEndpoint hrows)
          (artifact.rightEndpoint hcolumns) := by
    funext row
    exact parseReplayedPackingLabelArtifact_rows hparse row
  have hprefix :
      prefixAssignment (artifact.columnLabel hlabels) column.val =
        prefixAssignment
          (executableColumnLabels hlabels (artifact.leftEndpoint hrows)
            (artifact.rightEndpoint hcolumns)
            (executableRowLabels hlabels (artifact.leftEndpoint hrows)
              (artifact.rightEndpoint hcolumns))) column.val := by
    funext vertex
    simp only [prefixAssignment]
    split_ifs with hbefore
    · congr 1
      exact parseReplayedPackingLabelArtifact_columns hparse vertex
    · rfl
  rw [hrowLabels, hprefix] at hlocal
  exact hlocal
termination_by column.val

/-- A source-guard import proves every serialized left-endpoint fiber is at
most its serialized degree cap. -/
theorem parseSourceGuardPackingLabelArtifact_leftDegreeCap
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseSourceGuardPackingLabelArtifact values = some artifact) :
    ∀ row, fiberSize
        (artifact.leftEndpoint
          (parseReplayedPackingLabelArtifact_rowCount_pos
            (parseSourceGuardPackingLabelArtifact_replayed hparse))) row ≤
      artifact.sourceDegreeCap := by
  have hreplayed := parseSourceGuardPackingLabelArtifact_replayed hparse
  have hmatches := (parseSourceGuardPackingLabelArtifact_components hparse).2
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hreplayed
  have hcolumns := parseReplayedPackingLabelArtifact_columnCount_pos hreplayed
  simp only [PackingLabelArtifact.sourceClassGuardMatches, hrows, hcolumns,
    ↓reduceDIte, Bool.and_eq_true, List.all_eq_true] at hmatches
  intro row
  exact of_decide_eq_true (hmatches.1.1 row (by simp))

/-- A source-guard import proves every serialized right-endpoint fiber is at
most its serialized degree cap. -/
theorem parseSourceGuardPackingLabelArtifact_rightDegreeCap
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseSourceGuardPackingLabelArtifact values = some artifact) :
    ∀ column, fiberSize
        (artifact.rightEndpoint
          (parseReplayedPackingLabelArtifact_columnCount_pos
            (parseSourceGuardPackingLabelArtifact_replayed hparse))) column ≤
      artifact.sourceDegreeCap := by
  have hreplayed := parseSourceGuardPackingLabelArtifact_replayed hparse
  have hmatches := (parseSourceGuardPackingLabelArtifact_components hparse).2
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hreplayed
  have hcolumns := parseReplayedPackingLabelArtifact_columnCount_pos hreplayed
  simp only [PackingLabelArtifact.sourceClassGuardMatches, hrows, hcolumns,
    ↓reduceDIte, Bool.and_eq_true, List.all_eq_true] at hmatches
  intro column
  exact of_decide_eq_true (hmatches.1.2 column (by simp))

/-- A source-guard import proves the strongest real-valued class guard from
the serialized natural-number inequality. -/
theorem parseSourceGuardPackingLabelArtifact_guard
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseSourceGuardPackingLabelArtifact values = some artifact) :
    2 * (((artifact.labelCount * artifact.labelCount : ℕ) : ℝ) ^ 2) *
        (artifact.sourceDegreeCap : ℝ) ≤
      (artifact.edgeCount : ℝ) := by
  have hreplayed := parseSourceGuardPackingLabelArtifact_replayed hparse
  have hmatches := (parseSourceGuardPackingLabelArtifact_components hparse).2
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hreplayed
  have hcolumns := parseReplayedPackingLabelArtifact_columnCount_pos hreplayed
  simp only [PackingLabelArtifact.sourceClassGuardMatches, hrows, hcolumns,
    ↓reduceDIte, Bool.and_eq_true] at hmatches
  exact_mod_cast of_decide_eq_true hmatches.2

/-- A replayed artifact feeds its serialized labels directly into the strongest
source-guard class cap. The parser supplies the executable-label equalities;
callers provide only the graph's degree caps and source guard. -/
theorem parseReplayedPackingLabelArtifact_classCap
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseReplayedPackingLabelArtifact values = some artifact)
    (δ : ℕ)
    (hleftCap : ∀ row,
      fiberSize
        (artifact.leftEndpoint
          (parseReplayedPackingLabelArtifact_rowCount_pos hparse)) row ≤ δ)
    (hrightCap : ∀ column,
      fiberSize
        (artifact.rightEndpoint
          (parseReplayedPackingLabelArtifact_columnCount_pos hparse)) column ≤ δ)
    (hguard :
      2 * (((artifact.labelCount * artifact.labelCount : ℕ) : ℝ) ^ 2) *
          (δ : ℝ) ≤
        (artifact.edgeCount : ℝ)) :
    ∀ γ : Fin (artifact.labelCount * artifact.labelCount),
      (fiberSize
          (classMap
            (artifact.leftEndpoint
              (parseReplayedPackingLabelArtifact_rowCount_pos hparse))
            (artifact.rightEndpoint
              (parseReplayedPackingLabelArtifact_columnCount_pos hparse))
            (artifact.rowLabel
              (parseReplayedPackingLabelArtifact_labelCount_pos hparse))
            (artifact.columnLabel
              (parseReplayedPackingLabelArtifact_labelCount_pos hparse))) γ : ℝ) *
          ((artifact.labelCount * artifact.labelCount : ℕ) : ℝ) ≤
        2 * (artifact.edgeCount : ℝ) := by
  have hrows :
      artifact.rowLabel
          (parseReplayedPackingLabelArtifact_labelCount_pos hparse) =
        executableRowLabels
          (parseReplayedPackingLabelArtifact_labelCount_pos hparse)
          (artifact.leftEndpoint
            (parseReplayedPackingLabelArtifact_rowCount_pos hparse))
          (artifact.rightEndpoint
            (parseReplayedPackingLabelArtifact_columnCount_pos hparse)) := by
    funext row
    exact parseReplayedPackingLabelArtifact_rows hparse row
  have hcolumns :
      artifact.columnLabel
          (parseReplayedPackingLabelArtifact_labelCount_pos hparse) =
        executableColumnLabels
          (parseReplayedPackingLabelArtifact_labelCount_pos hparse)
          (artifact.leftEndpoint
            (parseReplayedPackingLabelArtifact_rowCount_pos hparse))
          (artifact.rightEndpoint
            (parseReplayedPackingLabelArtifact_columnCount_pos hparse))
          (executableRowLabels
            (parseReplayedPackingLabelArtifact_labelCount_pos hparse)
            (artifact.leftEndpoint
              (parseReplayedPackingLabelArtifact_rowCount_pos hparse))
            (artifact.rightEndpoint
              (parseReplayedPackingLabelArtifact_columnCount_pos hparse))) := by
    funext column
    exact parseReplayedPackingLabelArtifact_columns hparse column
  rw [hrows, hcolumns]
  exact class_cap_for_executable_labels
    (parseReplayedPackingLabelArtifact_labelCount_pos hparse)
    (artifact.leftEndpoint
      (parseReplayedPackingLabelArtifact_rowCount_pos hparse))
    (artifact.rightEndpoint
      (parseReplayedPackingLabelArtifact_columnCount_pos hparse))
    δ hleftCap hrightCap hguard

/-- A source-guard import reaches the class cap with no remaining degree or
guard premise: all three are recomputed from the flat projection. -/
theorem parseSourceGuardPackingLabelArtifact_classCap
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseSourceGuardPackingLabelArtifact values = some artifact) :
    ∀ γ : Fin (artifact.labelCount * artifact.labelCount),
      (fiberSize
          (classMap
            (artifact.leftEndpoint
              (parseReplayedPackingLabelArtifact_rowCount_pos
                (parseSourceGuardPackingLabelArtifact_replayed hparse)))
            (artifact.rightEndpoint
              (parseReplayedPackingLabelArtifact_columnCount_pos
                (parseSourceGuardPackingLabelArtifact_replayed hparse)))
            (artifact.rowLabel
              (parseReplayedPackingLabelArtifact_labelCount_pos
                (parseSourceGuardPackingLabelArtifact_replayed hparse)))
            (artifact.columnLabel
              (parseReplayedPackingLabelArtifact_labelCount_pos
                (parseSourceGuardPackingLabelArtifact_replayed hparse)))) γ : ℝ) *
          ((artifact.labelCount * artifact.labelCount : ℕ) : ℝ) ≤
        2 * (artifact.edgeCount : ℝ) := by
  have hreplayed := parseSourceGuardPackingLabelArtifact_replayed hparse
  exact parseReplayedPackingLabelArtifact_classCap hreplayed
    artifact.sourceDegreeCap
    (parseSourceGuardPackingLabelArtifact_leftDegreeCap hparse)
    (parseSourceGuardPackingLabelArtifact_rightDegreeCap hparse)
    (parseSourceGuardPackingLabelArtifact_guard hparse)

/-- The shared native projection contains these exact fields. -/
theorem packingLabelWitnessArtifactValues :
    packingLabelWitnessNumbers =
      [3, 2, 1, 0, 1, 2, 3, 3, 3,
        0, 0, 0, 0, 1, 0, 1, 0, 2, 0, 2, 0,
        0, 1, 0, 0, 0, 1] := by
  decide

/-- Named value encoded by the shared native projection. -/
def packingLabelWitness : PackingLabelArtifact := {
  schema := 3
  radix := 2
  packetExponent := 1
  coordinate := 0
  conditionalLabels := 1
  sourceDegreeCap := 2
  edgeCount := 3
  rowCount := 3
  columnCount := 3
  edgeFields := [0, 0, 0, 0, 1, 0, 1, 0, 2, 0, 2, 0]
  rowLabels := [0, 1, 0]
  columnLabels := [0, 0, 1]
}

/-- Native degree-one matching with `M=32=2*(2^2)^2*1`, so the strongest
source-class guard holds exactly. -/
def packingLabelSourceGuardWitness : PackingLabelArtifact := {
  schema := 3
  radix := 2
  packetExponent := 1
  coordinate := 0
  conditionalLabels := 1
  sourceDegreeCap := 1
  edgeCount := 32
  rowCount := 32
  columnCount := 32
  edgeFields := (List.range 32).flatMap fun edge => [edge, 0, edge, 0]
  rowLabels := (List.range 32).map fun row => row % 2
  columnLabels := (List.range 32).map fun column => column / 2 % 2
}

set_option maxRecDepth 100000 in
/-- The positive native source-guard projection parses to its named value. -/
theorem packingLabelSourceGuardWitness_parsed :
    parsePackingLabelArtifact packingLabelSourceGuardWitnessNumbers =
      some packingLabelSourceGuardWitness := by
  decide +revert

private theorem packingLabelSourceGuardWitness_leftEndpoint :
    packingLabelSourceGuardWitness.leftEndpoint (by decide) =
      fun edge : Fin 32 => edge := by
  funext edge
  fin_cases edge <;> apply Fin.ext <;> decide

private theorem packingLabelSourceGuardWitness_rightEndpoint :
    packingLabelSourceGuardWitness.rightEndpoint (by decide) =
      fun edge : Fin 32 => edge := by
  funext edge
  fin_cases edge <;> apply Fin.ext <;> decide

set_option maxHeartbeats 2000000 in
set_option maxRecDepth 100000 in
/-- The exact-boundary native projection passes executable trace replay. -/
theorem packingLabelSourceGuardWitness_traceMatches :
    packingLabelSourceGuardWitness.traceMatches = true := by
  decide +revert

/-- The exact-boundary projection parses and replays to its named artifact. -/
theorem packingLabelSourceGuardWitness_replayed :
    parseReplayedPackingLabelArtifact packingLabelSourceGuardWitnessNumbers =
      some packingLabelSourceGuardWitness := by
  simp [parseReplayedPackingLabelArtifact, packingLabelSourceGuardWitness_parsed,
    packingLabelSourceGuardWitness_traceMatches]

/-- The degree-one matching meets the source-class guard at equality. -/
theorem packingLabelSourceGuardWitness_sourceClassGuardMatches :
    packingLabelSourceGuardWitness.sourceClassGuardMatches = true := by
  decide

/-- The strict parser accepts the positive exact-boundary native projection. -/
theorem packingLabelSourceGuardWitness_sourceGuardParsed :
    parseSourceGuardPackingLabelArtifact packingLabelSourceGuardWitnessNumbers =
      some packingLabelSourceGuardWitness := by
  simp [parseSourceGuardPackingLabelArtifact,
    packingLabelSourceGuardWitness_replayed,
    packingLabelSourceGuardWitness_sourceClassGuardMatches]

/-- The shape- and bounds-checked parser returns the named projection. -/
theorem packingLabelWitness_parsed :
    parsePackingLabelArtifact packingLabelWitnessNumbers =
      some packingLabelWitness := by
  rw [packingLabelWitnessArtifactValues]
  decide

/-- First executable row choice for the identity three-edge fixture. -/
private theorem packingLabelWitness_row_zero :
    executableRowLabels (q := 2) (by decide)
        (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge) ⟨0, by decide⟩ =
      ⟨0, by decide⟩ := by
  rw [executableRowLabels_eq]
  rw [prefixAssignment_zero]
  unfold firstMinimumRowLabel firstMinimumLabel
  decide +revert

/-- Second executable row choice for the identity three-edge fixture. -/
private theorem packingLabelWitness_row_one :
    executableRowLabels (q := 2) (by decide)
        (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge) ⟨1, by decide⟩ =
      ⟨1, by decide⟩ := by
  rw [executableRowLabels_eq]
  have hprefix :
      prefixAssignment
          (executableRowLabels (q := 2) (by decide)
            (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)) 1 =
        fun vertex => if vertex.val = 0 then some ⟨0, by decide⟩ else none := by
    funext vertex
    fin_cases vertex <;> simp [prefixAssignment]
    exact packingLabelWitness_row_zero
  rw [hprefix]
  unfold firstMinimumRowLabel firstMinimumLabel
  decide +revert

/-- Third executable row choice for the identity three-edge fixture. -/
private theorem packingLabelWitness_row_two :
    executableRowLabels (q := 2) (by decide)
        (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge) ⟨2, by decide⟩ =
      ⟨0, by decide⟩ := by
  rw [executableRowLabels_eq]
  have hprefix :
      prefixAssignment
          (executableRowLabels (q := 2) (by decide)
            (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)) 2 =
        fun vertex =>
          if vertex.val = 0 then some ⟨0, by decide⟩
          else if vertex.val = 1 then some ⟨1, by decide⟩ else none := by
    funext vertex
    fin_cases vertex <;> simp [prefixAssignment]
    · exact packingLabelWitness_row_zero
    · exact packingLabelWitness_row_one
  rw [hprefix]
  unfold firstMinimumRowLabel firstMinimumLabel
  decide +revert

/-- Complete executable row array of the identity three-edge fixture. -/
private theorem packingLabelWitness_rows :
    executableRowLabels (q := 2) (by decide)
        (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge) =
      fun row => if row.val = 1 then ⟨1, by decide⟩ else ⟨0, by decide⟩ := by
  funext row
  fin_cases row
  · exact packingLabelWitness_row_zero
  · exact packingLabelWitness_row_one
  · exact packingLabelWitness_row_two

/-- First executable column choice after the fixed row trace. -/
private theorem packingLabelWitness_column_zero :
    executableColumnLabels (q := 2) (by decide)
        (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)
        (fun row => if row.val = 1 then ⟨1, by decide⟩ else ⟨0, by decide⟩)
        ⟨0, by decide⟩ = ⟨0, by decide⟩ := by
  rw [executableColumnLabels_eq]
  rw [prefixAssignment_zero]
  unfold firstMinimumColumnLabel firstMinimumLabel
  decide +revert

/-- Second executable column choice after the fixed row trace. -/
private theorem packingLabelWitness_column_one :
    executableColumnLabels (q := 2) (by decide)
        (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)
        (fun row => if row.val = 1 then ⟨1, by decide⟩ else ⟨0, by decide⟩)
        ⟨1, by decide⟩ = ⟨0, by decide⟩ := by
  rw [executableColumnLabels_eq]
  have hprefix :
      prefixAssignment
          (executableColumnLabels (q := 2) (by decide)
            (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)
            (fun row => if row.val = 1 then ⟨1, by decide⟩ else ⟨0, by decide⟩)) 1 =
        fun vertex => if vertex.val = 0 then some ⟨0, by decide⟩ else none := by
    funext vertex
    fin_cases vertex <;> simp [prefixAssignment]
    exact packingLabelWitness_column_zero
  rw [hprefix]
  unfold firstMinimumColumnLabel firstMinimumLabel
  decide +revert

/-- Third executable column choice after the fixed row trace. -/
private theorem packingLabelWitness_column_two :
    executableColumnLabels (q := 2) (by decide)
        (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)
        (fun row => if row.val = 1 then ⟨1, by decide⟩ else ⟨0, by decide⟩)
        ⟨2, by decide⟩ = ⟨1, by decide⟩ := by
  rw [executableColumnLabels_eq]
  have hprefix :
      prefixAssignment
          (executableColumnLabels (q := 2) (by decide)
            (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)
            (fun row => if row.val = 1 then ⟨1, by decide⟩ else ⟨0, by decide⟩)) 2 =
        fun vertex =>
          if vertex.val = 0 then some ⟨0, by decide⟩
          else if vertex.val = 1 then some ⟨0, by decide⟩ else none := by
    funext vertex
    fin_cases vertex <;> simp [prefixAssignment]
    · exact packingLabelWitness_column_zero
    · exact packingLabelWitness_column_one
  rw [hprefix]
  unfold firstMinimumColumnLabel firstMinimumLabel
  decide +revert

/-- Complete executable column array of the identity three-edge fixture. -/
private theorem packingLabelWitness_columns :
    executableColumnLabels (q := 2) (by decide)
        (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)
        (executableRowLabels (by decide)
          (fun edge : Fin 3 => edge) (fun edge : Fin 3 => edge)) =
      fun column => if column.val = 2 then ⟨1, by decide⟩ else ⟨0, by decide⟩ := by
  rw [packingLabelWitness_rows]
  funext column
  fin_cases column
  · exact packingLabelWitness_column_zero
  · exact packingLabelWitness_column_one
  · exact packingLabelWitness_column_two

/-- The projection's typed left endpoints are the identity edge map. -/
private theorem packingLabelWitness_leftEndpoint :
    packingLabelWitness.leftEndpoint (by decide) = fun edge : Fin 3 => edge := by
  funext edge
  fin_cases edge <;> rfl

/-- The projection's typed right endpoints are the identity edge map. -/
private theorem packingLabelWitness_rightEndpoint :
    packingLabelWitness.rightEndpoint (by decide) = fun edge : Fin 3 => edge := by
  funext edge
  fin_cases edge <;> rfl

/-- The projection's typed row labels are `[0,1,0]`. -/
private theorem packingLabelWitness_rowLabel :
    packingLabelWitness.rowLabel (by decide) =
      fun row => if row.val = 1 then ⟨1, by decide⟩ else ⟨0, by decide⟩ := by
  funext row
  fin_cases row <;> rfl

/-- The projection's typed column labels are `[0,0,1]`. -/
private theorem packingLabelWitness_columnLabel :
    packingLabelWitness.columnLabel (by decide) =
      fun column => if column.val = 2 then ⟨1, by decide⟩ else ⟨0, by decide⟩ := by
  funext column
  fin_cases column <;> rfl

/-- The shared native projection passes exact executable trace replay. -/
theorem packingLabelWitness_traceMatches :
    packingLabelWitness.traceMatches = true := by
  decide +revert

/-- The shared native projection parses and replays to the named artifact. -/
theorem packingLabelWitness_replayed :
    parseReplayedPackingLabelArtifact packingLabelWitnessNumbers =
      some packingLabelWitness := by
  simp [parseReplayedPackingLabelArtifact, packingLabelWitness_parsed,
    packingLabelWitness_traceMatches]

/-- The three-edge fixture deliberately fails the asymptotic source-class
guard; finite trace replay is not promoted to strict admission. -/
theorem packingLabelWitness_sourceClassGuardRejected :
    packingLabelWitness.sourceClassGuardMatches = false := by
  decide

/-- The strongest parser therefore rejects the finite fixture even though its
shape, bounds, and executable label trace all pass. -/
theorem packingLabelWitness_sourceGuardParserRejects :
    parseSourceGuardPackingLabelArtifact packingLabelWitnessNumbers = none := by
  simp [parseSourceGuardPackingLabelArtifact, packingLabelWitness_replayed,
    packingLabelWitness_sourceClassGuardRejected]

end Curve25519PackingLabelArtifact
