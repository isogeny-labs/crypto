import ProtocolCountermodels

/-!
# Transcript-level separation of zero knowledge, soundness, and extraction

Goldwasser--Micali--Rackoff define zero knowledge by simulation of the
verifier's view, separately from completeness and soundness.  This file gives
that distinction a finite semantic model: honest transcripts are reproduced
exactly by a deterministic simulator, while an always-accept verifier remains
unsound.

The `GlobalAcceptedTranscriptWitness` property below is stronger than
Bellare--Goldreich proof of knowledge only in its per-accepted-transcript and
false-instance quantification. Overall the notions are incomparable here: this
property omits their prover-oracle, success-probability, knowledge-error, and
efficiency obligations. Accordingly, failure of the global property is not
labelled failure of proof of knowledge.
-/

namespace ZKSeparation

/-- Minimal transcript semantics needed to keep relation membership,
simulation, and verifier acceptance distinct. -/
structure TranscriptProtocol
    (Statement Witness Transcript : Type*) where
  relation : Statement → Witness → Prop
  honestTranscript : Statement → Witness → Transcript
  simulator : Statement → Transcript
  accepts : Statement → Transcript → Prop

/-- The language induced by the witness relation. -/
def Language
    {Statement Witness Transcript : Type*}
    (P : TranscriptProtocol Statement Witness Transcript)
    (x : Statement) : Prop :=
  ∃ w, P.relation x w

/-- Honest related witnesses lead to accepting transcripts. -/
def Complete
    {Statement Witness Transcript : Type*}
    (P : TranscriptProtocol Statement Witness Transcript) : Prop :=
  ∀ x w, P.relation x w → P.accepts x (P.honestTranscript x w)

/-- In this deterministic finite model, perfect simulation is literal equality
of the honest transcript and the simulator output on related inputs. -/
def DeterministicPerfectTranscriptSimulation
    {Statement Witness Transcript : Type*}
    (P : TranscriptProtocol Statement Witness Transcript) : Prop :=
  ∀ x w, P.relation x w → P.honestTranscript x w = P.simulator x

/-- Every accepted transcript must belong to a statement having a witness. -/
def PerfectSound
    {Statement Witness Transcript : Type*}
    (P : TranscriptProtocol Statement Witness Transcript) : Prop :=
  ∀ x t, P.accepts x t → Language P x

/-- A deliberately strong, global transcript extractor: every accepted
`(statement, transcript)` pair is assigned a related witness.  The function is
defined only on accepted pairs, so no inhabitant is demanded when nothing is
accepted. -/
def GlobalAcceptedTranscriptWitness
    {Statement Witness Transcript : Type*}
    (P : TranscriptProtocol Statement Witness Transcript) : Prop :=
  ∃ E : ∀ x t, P.accepts x t → Witness,
    ∀ x t (hAccept : P.accepts x t), P.relation x (E x t hAccept)

/-- The global extraction property entails perfect soundness. -/
theorem globalAcceptedTranscriptWitness_implies_sound
    {Statement Witness Transcript : Type*}
    (P : TranscriptProtocol Statement Witness Transcript)
    (h : GlobalAcceptedTranscriptWitness P) :
    PerfectSound P := by
  obtain ⟨E, hE⟩ := h
  intro x t hAccept
  exact ⟨E x t hAccept, hE x t hAccept⟩

/-! ## A two-statement countermodel -/

/-- `true` is the only statement with a witness.  Transcripts reveal only the
statement, and the verifier accepts every transcript. -/
def alwaysAcceptTranscript : TranscriptProtocol Bool Unit Bool where
  relation := fun x _w ↦ x = true
  honestTranscript := fun x _w ↦ x
  simulator := fun x ↦ x
  accepts := fun _x _t ↦ True

theorem alwaysAcceptTranscript_complete :
    Complete alwaysAcceptTranscript := by
  intro _x _w _hRelation
  trivial

theorem alwaysAcceptTranscript_perfectly_simulated :
    DeterministicPerfectTranscriptSimulation alwaysAcceptTranscript := by
  intro _x _w _hRelation
  rfl

theorem alwaysAcceptTranscript_not_sound :
    ¬ PerfectSound alwaysAcceptTranscript := by
  intro hSound
  obtain ⟨w, hw⟩ := hSound false false trivial
  simp [alwaysAcceptTranscript] at hw

theorem alwaysAcceptTranscript_no_global_witness :
    ¬ GlobalAcceptedTranscriptWitness alwaysAcceptTranscript := by
  intro hExtractor
  exact alwaysAcceptTranscript_not_sound
    (globalAcceptedTranscriptWitness_implies_sound
      alwaysAcceptTranscript hExtractor)

/-- The transcript model simultaneously has completeness and exact simulation,
but lacks soundness and the stronger global extraction property. -/
theorem completeness_and_simulation_do_not_imply_soundness_or_global_extraction :
    Complete alwaysAcceptTranscript ∧
      DeterministicPerfectTranscriptSimulation alwaysAcceptTranscript ∧
      ¬ PerfectSound alwaysAcceptTranscript ∧
      ¬ GlobalAcceptedTranscriptWitness alwaysAcceptTranscript :=
  ⟨alwaysAcceptTranscript_complete,
    alwaysAcceptTranscript_perfectly_simulated,
    alwaysAcceptTranscript_not_sound,
    alwaysAcceptTranscript_no_global_witness⟩

/-- The pre-existing verdict-only countermodel is inherited rather than
re-proved.  The transcript model above refines the same `Bool` language. -/
theorem verdict_level_predecessor :
    (∀ x : Bool, x = true →
        (ProtocolCountermodels.alwaysAccept Bool).accepts x) ∧
      ∃ x : Bool, ¬ x = true ∧
        (ProtocolCountermodels.alwaysAccept Bool).accepts x :=
  ProtocolCountermodels.always_accept_complete_unsound
    (fun x : Bool ↦ x = true) false (by decide)

end ZKSeparation
