module

public import ObjectiveBoundary
public import BucketStateReplay

@[expose] public section

namespace Curve25519PackingLabelSourceEmbedding

open Lean Elab Term Meta

syntax (name := includeStrChunks) "include_str_chunks " str : term

meta partial def chunkCharacters : List Char → List String
  | [] => []
  | characters =>
      let (chunk, rest) := characters.splitAt 56
      String.ofList chunk :: chunkCharacters rest

@[term_elab includeStrChunks] meta def elabIncludeStrChunks : TermElab
  | `(include_str_chunks $path:str), _ => do
      let context ← readThe Core.Context
      let sourcePath := System.FilePath.mk context.fileName
      let sourceDirectory := sourcePath.parent.getD "."
      let contents ← IO.FS.readFile (sourceDirectory / path.getString)
      Meta.mkListLit (Lean.mkConst ``String)
        ((chunkCharacters contents.toList).map Lean.mkStrLit)
  | _, _ => throwUnsupportedSyntax

end Curve25519PackingLabelSourceEmbedding

/-!
# Executable packing-label artifact

This module imports the compact projection of the native schema-3 packing
provenance fixture.  It checks the projection's shape and bounds, then replays
the exact row-then-column strict-first-minimum schedule from `LabelSequence`.
The exact-boundary parse, row, column, and source-guard receipts live in
separate kernel-reduction modules so their independent certificates can build
in parallel before the final certificate joins them.
-/

namespace Curve25519PackingLabelArtifact

open Curve25519LabelSequence
open Curve25519ConditionalObjective
open Curve25519ObjectiveBoundary
open Curve25519BucketStateReplay

/-- Shared native projection: schema/radix/context, endpoint quadruples, then
row and column labels. -/
def packingLabelWitnessArtifact : String :=
  include_str ".." / "packing_label_witness.csv"

/-- Dependency anchor for the native exact-boundary projection. -/
private def packingLabelSourceGuardWitnessRawArtifact : String :=
  include_str ".." / "packing_label_source_guard_witness.csv"

/-- Bounded elaboration-time chunks of the exact-boundary projection. -/
def packingLabelSourceGuardWitnessArtifactChunks : List String :=
  include_str_chunks "../packing_label_source_guard_witness.csv"

/-- Character representation used by kernel replay. -/
def packingLabelSourceGuardWitnessArtifactCharacters : List Char :=
  packingLabelSourceGuardWitnessArtifactChunks.flatMap String.toList

/-- Shared native projection at the exact strongest source-class boundary. -/
def packingLabelSourceGuardWitnessArtifact : String :=
  String.ofList packingLabelSourceGuardWitnessArtifactCharacters

/-- The public string view and chunked kernel view are exactly the same source. -/
theorem packingLabelSourceGuardWitnessArtifact_toList :
    packingLabelSourceGuardWitnessArtifact.toList =
      packingLabelSourceGuardWitnessArtifactCharacters := by
  exact String.toList_ofList

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
    packingLabelSourceGuardWitnessArtifactCharacters 0 []).getD []

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

/-- Source-shaped endpoint record matching one native `EdgeEndpoints` value. -/
structure PackingLabelEdge where
  left : ℕ
  leftPort : ℕ
  right : ℕ
  rightPort : ℕ
  deriving DecidableEq, Inhabited, Repr

/-- The four native endpoint fields in their projection order. -/
def PackingLabelEdge.fields (edge : PackingLabelEdge) : List ℕ :=
  [edge.left, edge.leftPort, edge.right, edge.rightPort]

/-- Named source view matching the successful native `LabelProjectionView`.
Machine-word conversions and allocation failure remain on the Rust side; this
record models the natural-number result once those checks succeed. -/
structure PackingLabelSource where
  schema : ℕ
  radix : ℕ
  packetExponent : ℕ
  coordinate : ℕ
  conditionalLabels : Bool
  sourceDegreeCap : Option ℕ
  edges : List PackingLabelEdge
  rowLabels : List ℕ
  columnLabels : List ℕ
  deriving DecidableEq, Repr

/-- Convert the named source view into the exact formal artifact fields. -/
def PackingLabelSource.artifact (source : PackingLabelSource) :
    PackingLabelArtifact := {
  schema := source.schema
  radix := source.radix
  packetExponent := source.packetExponent
  coordinate := source.coordinate
  conditionalLabels := if source.conditionalLabels then 1 else 0
  sourceDegreeCap := source.sourceDegreeCap.getD 0
  edgeCount := source.edges.length
  rowCount := source.rowLabels.length
  columnCount := source.columnLabels.length
  edgeFields := source.edges.flatMap PackingLabelEdge.fields
  rowLabels := source.rowLabels
  columnLabels := source.columnLabels
}

/-- Structured endpoint quadruples always flatten to exactly four fields per
native edge. -/
@[simp] theorem PackingLabelSource.edgeFields_length
    (source : PackingLabelSource) :
    (source.edges.flatMap PackingLabelEdge.fields).length =
      4 * source.edges.length := by
  induction source.edges with
  | nil => simp
  | cons edge edges ih =>
      simp [PackingLabelEdge.fields, ih]
      omega

private theorem edgeFields_left_getD
    (edges : List PackingLabelEdge) (index : ℕ)
    (hindex : index < edges.length) :
    (edges.flatMap PackingLabelEdge.fields).getD (4 * index) 0 =
      (edges.getD index default).left := by
  induction edges generalizing index with
  | nil => simp at hindex
  | cons edge edges ih =>
      cases index with
      | zero => simp [PackingLabelEdge.fields]
      | succ index =>
          have htail : index < edges.length := by simpa using hindex
          simpa [PackingLabelEdge.fields, Nat.mul_succ] using ih index htail

private theorem edgeFields_right_getD
    (edges : List PackingLabelEdge) (index : ℕ)
    (hindex : index < edges.length) :
    (edges.flatMap PackingLabelEdge.fields).getD (4 * index + 2) 0 =
      (edges.getD index default).right := by
  induction edges generalizing index with
  | nil => simp at hindex
  | cons edge edges ih =>
      cases index with
      | zero => simp [PackingLabelEdge.fields]
      | succ index =>
          have htail : index < edges.length := by simpa using hindex
          simpa [PackingLabelEdge.fields, Nat.mul_succ] using ih index htail

/-- Direct shape and context proposition over the named native source fields. -/
def PackingLabelSource.WellFormed (source : PackingLabelSource) : Prop :=
  source.schema = 3 ∧ source.conditionalLabels = true ∧
    0 < source.radix ∧ source.coordinate < source.packetExponent

/-- Direct endpoint and label bounds over the named native source fields. -/
def PackingLabelSource.Bounded (source : PackingLabelSource) : Prop :=
  ((∀ index < source.edges.length,
      let edge := source.edges.getD index default
      edge.left < source.rowLabels.length ∧
        edge.right < source.columnLabels.length) ∧
    (∀ label ∈ source.rowLabels,
      label < source.radix ^ source.packetExponent)) ∧
    (∀ label ∈ source.columnLabels,
      label < source.radix ^ source.packetExponent)

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

/-- Formal artifact shape checks are exactly the direct named-source proposition. -/
theorem PackingLabelSource.artifact_wellFormed_iff
    (source : PackingLabelSource) :
    source.artifact.wellFormed = true ↔ source.WellFormed := by
  simp only [PackingLabelArtifact.wellFormed, PackingLabelSource.artifact,
    Bool.and_eq_true, beq_iff_eq]
  rw [PackingLabelSource.edgeFields_length]
  cases hconditional : source.conditionalLabels <;>
    simp [PackingLabelSource.WellFormed, hconditional]
  intro _
  exact decide_eq_true_iff

/-- Formal artifact bounds are exactly the direct named-source proposition. -/
theorem PackingLabelSource.artifact_bounded_iff
    (source : PackingLabelSource) :
    source.artifact.bounded = true ↔ source.Bounded := by
  simp only [PackingLabelArtifact.bounded,
    PackingLabelSource.artifact,
    PackingLabelArtifact.leftIndex, PackingLabelArtifact.rightIndex,
    PackingLabelArtifact.labelCount, Bool.and_eq_true, List.all_eq_true,
    List.mem_range]
  constructor
  · rintro ⟨⟨hedges, hrows⟩, hcolumns⟩
    refine ⟨⟨?_, ?_⟩, ?_⟩
    · intro index hindex
      have hedge := hedges index hindex
      constructor
      · rw [← edgeFields_left_getD source.edges index hindex]
        exact of_decide_eq_true hedge.1
      · rw [← edgeFields_right_getD source.edges index hindex]
        exact of_decide_eq_true hedge.2
    · intro label hlabel
      exact of_decide_eq_true (hrows label hlabel)
    · intro label hlabel
      exact of_decide_eq_true (hcolumns label hlabel)
  · rintro ⟨⟨hedges, hrows⟩, hcolumns⟩
    refine ⟨⟨?_, ?_⟩, ?_⟩
    · intro index hindex
      have hedge := hedges index hindex
      constructor
      · apply decide_eq_true
        rw [edgeFields_left_getD source.edges index hindex]
        exact hedge.1
      · apply decide_eq_true
        rw [edgeFields_right_getD source.edges index hindex]
        exact hedge.2
    · intro label hlabel
      exact decide_eq_true (hrows label hlabel)
    · intro label hlabel
      exact decide_eq_true (hcolumns label hlabel)

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

/-- Canonical flat projection emitted by the native schema-3 exporter: the
nine-field header, endpoint quadruples, row labels, then column labels. -/
def PackingLabelArtifact.projectionValues
    (artifact : PackingLabelArtifact) : List ℕ :=
  [artifact.schema, artifact.radix, artifact.packetExponent,
    artifact.coordinate, artifact.conditionalLabels,
    artifact.sourceDegreeCap, artifact.edgeCount, artifact.rowCount,
    artifact.columnCount] ++ artifact.edgeFields ++ artifact.rowLabels ++
      artifact.columnLabels

/-- Direct source-shaped flattening in the order used by the native view. -/
def PackingLabelSource.projectionValues (source : PackingLabelSource) : List ℕ :=
  [source.schema, source.radix, source.packetExponent, source.coordinate,
    if source.conditionalLabels then 1 else 0,
    source.sourceDegreeCap.getD 0, source.edges.length,
    source.rowLabels.length, source.columnLabels.length] ++
      source.edges.flatMap PackingLabelEdge.fields ++ source.rowLabels ++
        source.columnLabels

/-- The named source view and the formal artifact have definitionally identical
canonical projections. -/
theorem PackingLabelSource.artifact_projectionValues
    (source : PackingLabelSource) :
    source.artifact.projectionValues = source.projectionValues := by
  rfl

/-- The unchecked parser is a left inverse of the canonical projection when
the two split positions have their declared lengths. -/
theorem parsePackingLabelArtifactUnchecked_projectionValues
    (artifact : PackingLabelArtifact)
    (hedges : artifact.edgeFields.length = 4 * artifact.edgeCount)
    (hrows : artifact.rowLabels.length = artifact.rowCount) :
    parsePackingLabelArtifactUnchecked artifact.projectionValues =
      some artifact := by
  simp [PackingLabelArtifact.projectionValues,
    parsePackingLabelArtifactUnchecked, hedges, hrows]

/-- Every well-formed, bounded artifact round-trips through the exact flat
projection interface consumed by the checked formal importer. -/
theorem parsePackingLabelArtifact_projectionValues
    (artifact : PackingLabelArtifact)
    (hwell : artifact.wellFormed = true)
    (hbounded : artifact.bounded = true) :
    parsePackingLabelArtifact artifact.projectionValues = some artifact := by
  have hshape := hwell
  simp only [PackingLabelArtifact.wellFormed, Bool.and_eq_true,
    beq_iff_eq] at hshape
  unfold parsePackingLabelArtifact
  rw [parsePackingLabelArtifactUnchecked_projectionValues artifact
    hshape.1.1.1.2 hshape.1.1.2]
  simp [hwell, hbounded]

/-- The unchecked parser retains every input field: projecting any successful
parse reconstructs the original flat value list exactly. -/
theorem PackingLabelArtifact.projectionValues_of_parseUnchecked
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifactUnchecked values = some artifact) :
    artifact.projectionValues = values := by
  rcases values with _ | ⟨schema, values⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  rcases values with _ | ⟨radix, values⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  rcases values with _ | ⟨packetExponent, values⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  rcases values with _ | ⟨coordinate, values⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  rcases values with _ | ⟨conditionalLabels, values⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  rcases values with _ | ⟨sourceDegreeCap, values⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  rcases values with _ | ⟨edgeCount, values⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  rcases values with _ | ⟨rowCount, values⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  rcases values with _ | ⟨columnCount, fields⟩
  · simp [parsePackingLabelArtifactUnchecked] at hparse
  simp [parsePackingLabelArtifactUnchecked] at hparse
  subst artifact
  simp [PackingLabelArtifact.projectionValues, List.take_append_drop]

/-- Checked parsing and canonical projection are inverse on every accepted
flat value list. -/
theorem PackingLabelArtifact.projectionValues_of_parse
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact) :
    artifact.projectionValues = values := by
  unfold parsePackingLabelArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with
    ⟨candidate, hparsed, hchecked⟩
  split at hchecked
  next =>
    have hartifact : candidate = artifact := Option.some.inj hchecked
    subst candidate
    exact artifact.projectionValues_of_parseUnchecked hparsed
  next => simp at hchecked

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

/-- Exact specification of the checked flat importer: accepted values are
precisely the canonical projection of a well-formed bounded artifact. -/
theorem parsePackingLabelArtifact_iff
    {values : List ℕ} {artifact : PackingLabelArtifact} :
    parsePackingLabelArtifact values = some artifact ↔
      (artifact.wellFormed = true ∧ artifact.bounded = true ∧
        artifact.projectionValues = values) := by
  constructor
  · intro hparse
    have hcomponents := parsePackingLabelArtifact_components hparse
    exact ⟨hcomponents.1, hcomponents.2,
      artifact.projectionValues_of_parse hparse⟩
  · rintro ⟨hwell, hbounded, hprojection⟩
    rw [← hprojection]
    exact parsePackingLabelArtifact_projectionValues artifact hwell hbounded

/-- The successful native-shaped view is checked exactly by the artifact's
well-formedness and bounds; its projection equality is definitional. -/
theorem PackingLabelSource.parseProjection_iff
    (source : PackingLabelSource) :
    parsePackingLabelArtifact source.projectionValues = some source.artifact ↔
      (source.WellFormed ∧ source.Bounded) := by
  simpa only [PackingLabelSource.artifact_projectionValues,
    PackingLabelSource.artifact_wellFormed_iff,
    PackingLabelSource.artifact_bounded_iff, and_true] using
    (parsePackingLabelArtifact_iff
      (values := source.projectionValues) (artifact := source.artifact))

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

/-- Replay the exact row-then-column first-minimum schedule in single passes
through the proved auxiliary bucket states. Positivity is checked before
dependent finite types are formed. -/
def PackingLabelArtifact.traceMatches (artifact : PackingLabelArtifact) : Bool :=
  if hlabels : 0 < artifact.labelCount then
    if hrows : 0 < artifact.rowCount then
      if hcolumns : 0 < artifact.columnCount then
        replayRowState hlabels
            (artifact.leftEndpoint hrows) (artifact.rightEndpoint hcolumns)
            (artifact.rowLabel hlabels) &&
          replayColumnState hlabels
            (artifact.leftEndpoint hrows) (artifact.rightEndpoint hcolumns)
            (artifact.rowLabel hlabels) (artifact.columnLabel hlabels)
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

/-- Successful shape/bounds parsing and trace replay construct a successful
replayed import. -/
theorem parseReplayedPackingLabelArtifact_of_components
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parsePackingLabelArtifact values = some artifact)
    (htrace : artifact.traceMatches = true) :
    parseReplayedPackingLabelArtifact values = some artifact := by
  simp [parseReplayedPackingLabelArtifact, hparse, htrace]

/-- Successful replay and source-guard checking construct a successful strict
import. -/
theorem parseSourceGuardPackingLabelArtifact_of_components
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hreplayed : parseReplayedPackingLabelArtifact values = some artifact)
    (hguard : artifact.sourceClassGuardMatches = true) :
    parseSourceGuardPackingLabelArtifact values = some artifact := by
  simp [parseSourceGuardPackingLabelArtifact, hreplayed, hguard]

/-- A formally replayable artifact is accepted after canonical projection. -/
theorem parseReplayedPackingLabelArtifact_projectionValues
    (artifact : PackingLabelArtifact)
    (hwell : artifact.wellFormed = true)
    (hbounded : artifact.bounded = true)
    (htrace : artifact.traceMatches = true) :
    parseReplayedPackingLabelArtifact artifact.projectionValues =
      some artifact := by
  exact parseReplayedPackingLabelArtifact_of_components
    (parsePackingLabelArtifact_projectionValues artifact hwell hbounded)
    htrace

/-- A formally replayable artifact satisfying the source-degree guard is
accepted by the strict importer after canonical projection. -/
theorem parseSourceGuardPackingLabelArtifact_projectionValues
    (artifact : PackingLabelArtifact)
    (hwell : artifact.wellFormed = true)
    (hbounded : artifact.bounded = true)
    (htrace : artifact.traceMatches = true)
    (hguard : artifact.sourceClassGuardMatches = true) :
    parseSourceGuardPackingLabelArtifact artifact.projectionValues =
      some artifact := by
  exact parseSourceGuardPackingLabelArtifact_of_components
    (parseReplayedPackingLabelArtifact_projectionValues
      artifact hwell hbounded htrace)
    hguard

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

/-- Exact specification of the replay importer: checked canonical values plus
the formal trace predicate are necessary and sufficient. -/
theorem parseReplayedPackingLabelArtifact_iff
    {values : List ℕ} {artifact : PackingLabelArtifact} :
    parseReplayedPackingLabelArtifact values = some artifact ↔
      ((artifact.wellFormed = true ∧ artifact.bounded = true ∧
          artifact.projectionValues = values) ∧
        artifact.traceMatches = true) := by
  constructor
  · intro hparse
    have hcomponents := parseReplayedPackingLabelArtifact_components hparse
    exact ⟨parsePackingLabelArtifact_iff.mp hcomponents.1, hcomponents.2⟩
  · rintro ⟨hbase, htrace⟩
    exact parseReplayedPackingLabelArtifact_of_components
      (parsePackingLabelArtifact_iff.mpr hbase) htrace

/-- Exact specification of the strict importer: checked canonical values, the
formal replay trace, and the source-class guard are jointly equivalent to
acceptance. -/
theorem parseSourceGuardPackingLabelArtifact_iff
    {values : List ℕ} {artifact : PackingLabelArtifact} :
    parseSourceGuardPackingLabelArtifact values = some artifact ↔
      (((artifact.wellFormed = true ∧ artifact.bounded = true ∧
          artifact.projectionValues = values) ∧
        artifact.traceMatches = true) ∧
        artifact.sourceClassGuardMatches = true) := by
  constructor
  · intro hparse
    have hcomponents :=
      parseSourceGuardPackingLabelArtifact_components hparse
    exact ⟨parseReplayedPackingLabelArtifact_iff.mp hcomponents.1,
      hcomponents.2⟩
  · rintro ⟨hreplayed, hguard⟩
    exact parseSourceGuardPackingLabelArtifact_of_components
      (parseReplayedPackingLabelArtifact_iff.mpr hreplayed) hguard

/-- The replay importer for a successful native-shaped view adds exactly the
formal trace predicate to the checked artifact conditions. -/
theorem PackingLabelSource.parseReplayedProjection_iff
    (source : PackingLabelSource) :
    parseReplayedPackingLabelArtifact source.projectionValues =
        some source.artifact ↔
      ((source.WellFormed ∧ source.Bounded) ∧
        source.artifact.traceMatches = true) := by
  simpa only [PackingLabelSource.artifact_projectionValues,
    PackingLabelSource.artifact_wellFormed_iff,
    PackingLabelSource.artifact_bounded_iff, and_true] using
    (parseReplayedPackingLabelArtifact_iff
      (values := source.projectionValues) (artifact := source.artifact))

/-- The strict importer for a successful native-shaped view adds exactly the
source-class guard after checked parsing and trace replay. -/
theorem PackingLabelSource.parseSourceGuardProjection_iff
    (source : PackingLabelSource) :
    parseSourceGuardPackingLabelArtifact source.projectionValues =
        some source.artifact ↔
      (((source.WellFormed ∧ source.Bounded) ∧
        source.artifact.traceMatches = true) ∧
        source.artifact.sourceClassGuardMatches = true) := by
  simpa only [PackingLabelSource.artifact_projectionValues,
    PackingLabelSource.artifact_wellFormed_iff,
    PackingLabelSource.artifact_bounded_iff, and_true] using
    (parseSourceGuardPackingLabelArtifact_iff
      (values := source.projectionValues) (artifact := source.artifact))

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
  have htrace := (parseReplayedPackingLabelArtifact_components hparse).2
  have hlabels := parseReplayedPackingLabelArtifact_labelCount_pos hparse
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hparse
  have hcolumns := parseReplayedPackingLabelArtifact_columnCount_pos hparse
  simp only [PackingLabelArtifact.traceMatches, hlabels, hrows, hcolumns,
    ↓reduceDIte, Bool.and_eq_true] at htrace
  exact congrFun (replayRowState_eq_executable hlabels
    (artifact.leftEndpoint hrows) (artifact.rightEndpoint hcolumns)
    (artifact.rowLabel hlabels) htrace.1) row

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
  have htrace := (parseReplayedPackingLabelArtifact_components hparse).2
  have hlabels := parseReplayedPackingLabelArtifact_labelCount_pos hparse
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hparse
  have hcolumns := parseReplayedPackingLabelArtifact_columnCount_pos hparse
  simp only [PackingLabelArtifact.traceMatches, hlabels, hrows, hcolumns,
    ↓reduceDIte, Bool.and_eq_true] at htrace
  have hrowLabels :
      artifact.rowLabel hlabels =
        executableRowLabels hlabels (artifact.leftEndpoint hrows)
          (artifact.rightEndpoint hcolumns) := by
    funext row
    exact parseReplayedPackingLabelArtifact_rows hparse row
  have hcolumns := replayColumnState_eq_executable hlabels
    (artifact.leftEndpoint hrows) (artifact.rightEndpoint hcolumns)
    (artifact.rowLabel hlabels) (artifact.columnLabel hlabels) htrace.2
  rw [hrowLabels] at hcolumns
  exact congrFun hcolumns column

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

/-- Largest recomputed endpoint-label class. Malformed zero-dimensional
artifacts map to zero; strict parsing establishes all three positive branches. -/
def PackingLabelArtifact.maximumClassSize
    (artifact : PackingLabelArtifact) : ℕ :=
  if hlabels : 0 < artifact.labelCount then
    if hrows : 0 < artifact.rowCount then
      if hcolumns : 0 < artifact.columnCount then
        (Finset.univ :
            Finset (Fin (artifact.labelCount * artifact.labelCount))).sup
          (fun gamma => fiberSize
            (classMap
              (artifact.leftEndpoint hrows)
              (artifact.rightEndpoint hcolumns)
              (artifact.rowLabel hlabels)
              (artifact.columnLabel hlabels)) gamma)
      else 0
    else 0
  else 0

/-- Strict source acceptance bounds the actual maximum recomputed class, not
only a caller-selected class. -/
theorem parseSourceGuardPackingLabelArtifact_maximumClassCap
    {values : List ℕ} {artifact : PackingLabelArtifact}
    (hparse : parseSourceGuardPackingLabelArtifact values = some artifact) :
    (artifact.maximumClassSize : ℝ) *
        ((artifact.labelCount * artifact.labelCount : ℕ) : ℝ) ≤
      2 * (artifact.edgeCount : ℝ) := by
  have hreplayed := parseSourceGuardPackingLabelArtifact_replayed hparse
  have hlabels := parseReplayedPackingLabelArtifact_labelCount_pos hreplayed
  have hrows := parseReplayedPackingLabelArtifact_rowCount_pos hreplayed
  have hcolumns := parseReplayedPackingLabelArtifact_columnCount_pos hreplayed
  have hclasses : 0 < artifact.labelCount * artifact.labelCount :=
    Nat.mul_pos hlabels hlabels
  have huniv :
      (Finset.univ :
        Finset (Fin (artifact.labelCount * artifact.labelCount))).Nonempty :=
    ⟨⟨0, hclasses⟩, Finset.mem_univ _⟩
  obtain ⟨gamma, _, hmaximum⟩ :=
    Finset.exists_mem_eq_sup
      (Finset.univ :
        Finset (Fin (artifact.labelCount * artifact.labelCount)))
      huniv
      (fun index => fiberSize
        (classMap
          (artifact.leftEndpoint hrows)
          (artifact.rightEndpoint hcolumns)
          (artifact.rowLabel hlabels)
          (artifact.columnLabel hlabels)) index)
  have hclass := parseSourceGuardPackingLabelArtifact_classCap hparse gamma
  simpa [PackingLabelArtifact.maximumClassSize, hlabels, hrows, hcolumns,
    hmaximum] using hclass

/-- A strictly accepted named source view feeds its recomputed class cap into
the retained-history eightfold shrink. The only remaining premises describe
the source length mass, padded group height, and two-sided child group. -/
theorem PackingLabelSource.historyChildStrictShrink
    (source : PackingLabelSource)
    (hparse :
      parseSourceGuardPackingLabelArtifact source.projectionValues =
        some source.artifact)
    (γ : Fin (source.artifact.labelCount * source.artifact.labelCount))
    (L height child : ℝ)
    (hM : 0 ≤ (source.artifact.edgeCount : ℝ))
    (hlength :
      L * ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ) ≤
        (source.artifact.edgeCount : ℝ))
    (hheight :
      height ≤
        (fiberSize
          (classMap
            (source.artifact.leftEndpoint
              (parseReplayedPackingLabelArtifact_rowCount_pos
                (parseSourceGuardPackingLabelArtifact_replayed hparse)))
            (source.artifact.rightEndpoint
              (parseReplayedPackingLabelArtifact_columnCount_pos
                (parseSourceGuardPackingLabelArtifact_replayed hparse)))
            (source.artifact.rowLabel
              (parseReplayedPackingLabelArtifact_labelCount_pos
                (parseSourceGuardPackingLabelArtifact_replayed hparse)))
            (source.artifact.columnLabel
              (parseReplayedPackingLabelArtifact_labelCount_pos
                (parseSourceGuardPackingLabelArtifact_replayed hparse)))) γ : ℝ) + L)
    (hchild :
      child * 64 ≤ 2 *
        (height *
          ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ))) :
    8 * child ≤ (source.artifact.edgeCount : ℝ) := by
  have hclass := parseSourceGuardPackingLabelArtifact_classCap hparse γ
  exact Curve25519ParametricRefinement.history_child_strict_shrink_of_class_capacity
    (source.artifact.edgeCount : ℝ)
    ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ)
    L
    (fiberSize
      (classMap
        (source.artifact.leftEndpoint
          (parseReplayedPackingLabelArtifact_rowCount_pos
            (parseSourceGuardPackingLabelArtifact_replayed hparse)))
        (source.artifact.rightEndpoint
          (parseReplayedPackingLabelArtifact_columnCount_pos
            (parseSourceGuardPackingLabelArtifact_replayed hparse)))
        (source.artifact.rowLabel
          (parseReplayedPackingLabelArtifact_labelCount_pos
            (parseSourceGuardPackingLabelArtifact_replayed hparse)))
        (source.artifact.columnLabel
          (parseReplayedPackingLabelArtifact_labelCount_pos
            (parseSourceGuardPackingLabelArtifact_replayed hparse)))) γ : ℝ)
    height child hM (by positivity) hclass hlength hheight hchild

/-- The exact maximum-class ceil-and-multiply height computed by the native
packer removes both a selected-class witness and the padded-height premise
from `historyChildStrictShrink`. Source length mass and two-sided child load
remain explicit. -/
theorem PackingLabelSource.historyChildStrictShrinkOfRoundedMaximum
    (source : PackingLabelSource)
    (hparse :
      parseSourceGuardPackingLabelArtifact source.projectionValues =
        some source.artifact)
    (packetLength : ℕ)
    (child : ℝ)
    (hM : 0 ≤ (source.artifact.edgeCount : ℝ))
    (hlength :
      (packetLength : ℝ) *
          ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ) ≤
        (source.artifact.edgeCount : ℝ))
    (hchild :
      child * 64 ≤ 2 *
        (((source.artifact.maximumClassSize ⌈/⌉ packetLength) *
            packetLength : ℕ) : ℝ) *
          ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ)) :
    8 * child ≤ (source.artifact.edgeCount : ℝ) := by
  let classSize := source.artifact.maximumClassSize
  have hclass :=
    parseSourceGuardPackingLabelArtifact_maximumClassCap hparse
  have hheightNat :=
    Curve25519ParametricRefinement.rounded_class_height_le_add
      classSize packetLength
  have hheight :
      (((classSize ⌈/⌉ packetLength) * packetLength : ℕ) : ℝ) ≤
        (classSize : ℝ) + (packetLength : ℝ) := by
    exact_mod_cast hheightNat
  have hchild' :
      child * 64 ≤ 2 *
        ((((classSize ⌈/⌉ packetLength) * packetLength : ℕ) : ℝ) *
          ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ)) := by
    simpa [classSize, mul_assoc] using hchild
  exact Curve25519ParametricRefinement.history_child_strict_shrink_of_class_capacity
    (source.artifact.edgeCount : ℝ)
    ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ)
    (packetLength : ℝ)
    (classSize : ℝ)
    (((classSize ⌈/⌉ packetLength) * packetLength : ℕ) : ℝ)
    child hM (by positivity) hclass hlength hheight hchild'

/-- Natural-number boundary matching the independently checked native node
summary. Its two inequalities are exactly source length mass and replayed
two-sided child incidence; all casts are discharged internally. -/
theorem PackingLabelSource.historyChildStrictShrinkOfRoundedMaximumNat
    (source : PackingLabelSource)
    (hparse :
      parseSourceGuardPackingLabelArtifact source.projectionValues =
        some source.artifact)
    (packetLength child : ℕ)
    (hlength :
      packetLength *
          (source.artifact.labelCount * source.artifact.labelCount) ≤
        source.artifact.edgeCount)
    (hchild :
      child * 64 ≤ 2 *
        ((source.artifact.maximumClassSize ⌈/⌉ packetLength) *
          packetLength) *
        (source.artifact.labelCount * source.artifact.labelCount)) :
    8 * child ≤ source.artifact.edgeCount := by
  have hlengthReal :
      (packetLength : ℝ) *
          ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ) ≤
        (source.artifact.edgeCount : ℝ) := by
    exact_mod_cast hlength
  have hchildReal :
      (child : ℝ) * 64 ≤ 2 *
        (((source.artifact.maximumClassSize ⌈/⌉ packetLength) *
            packetLength : ℕ) : ℝ) *
          ((source.artifact.labelCount * source.artifact.labelCount : ℕ) : ℝ) := by
    exact_mod_cast hchild
  have hshrink := source.historyChildStrictShrinkOfRoundedMaximum
    hparse packetLength (child : ℝ) (by positivity) hlengthReal hchildReal
  exact_mod_cast hshrink

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

/-- Structured named-source form of the shared exact-boundary native witness. -/
def packingLabelSourceGuardWitnessSource : PackingLabelSource := {
  schema := 3
  radix := 2
  packetExponent := 1
  coordinate := 0
  conditionalLabels := true
  sourceDegreeCap := some 1
  edges := (List.range 32).map fun edge => {
    left := edge
    leftPort := 0
    right := edge
    rightPort := 0
  }
  rowLabels := (List.range 32).map fun row => row % 2
  columnLabels := (List.range 32).map fun column => column / 2 % 2
}

/-- The structured source view converts to the existing exact-boundary
artifact without changing any header, endpoint, or label field. -/
theorem packingLabelSourceGuardWitnessSource_artifact :
    packingLabelSourceGuardWitnessSource.artifact =
      packingLabelSourceGuardWitness := by
  simp [packingLabelSourceGuardWitnessSource,
    packingLabelSourceGuardWitness, PackingLabelSource.artifact]
  rw [List.flatMap_map]
  rfl

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
