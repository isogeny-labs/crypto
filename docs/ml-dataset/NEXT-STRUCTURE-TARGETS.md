# Next learning targets beyond branch imitation

This is an implementation roadmap from the October30 read-only coverage audit,
not generated training data or a claim of model efficacy. The current prefix
export certifies bounded pre-decision algebra and observed ascending branch
attempts. Learning that ordering alone is not evidence of hidden structure.

Preserve the original capability/outcome matrix: Koblitz, general binary and
prime curves; both curve/twist positions; n23..31 and47..61; p=2^61-1; m2..15;
multiple generators and keys. Unsupported cells and capped checks remain
explicit. This roadmap does not authorize a new cloud wave or IAM change.

## 1. Checked polynomial-consequence supervision

Add a dedicated algebraic-target projection, reusing explicit generator
combinations and the independent identity checker in `prefix_algebra.rs`.
Public input is the exact pre-step polynomial graph and declared ring. Restricted
supervision contains the proposed consequence, multiplier combination, degrees
before cancellation, result degree and actual verification work. Distinguish
native-operation witnesses from separately searched certificates in every row.
A searched identity is not evidence that the native solver performed that step.

The first useful target is a verified observed degree reduction. Keep exact
first/last-fall labels null. A model may propose consequences or combinations;
only independently checked identities enter a certified target set. Compare
against simple cancellation and deterministic certificate-search baselines in a
later frozen experiment, counting inference, rejection and fallback work.

Likely files: new `src/ml_dataset/algebraic_targets.rs`, with scoped integration
in `prefix_algebra.rs`, `prime_chain_prefix.rs`, `export.rs` and central admission.
Coordinate any shared `mod.rs` reservation before editing. Inventory closure
machinery in `inherited_f4.rs`, `koblitz_bench.rs` and `closure_checkpoint.rs` before
adding a second engine.

Acceptance checks:

- Replay each identity and reject altered coefficients, generators, rings and
  quotient rules. Keep public features independent of future events and labels.
- Retain verified earlier examples when a later certificate exceeds its budget.
- Separate certificate-search/verifier work from native solver instructions.
- Before introducing bounded closure labels, retain a derivation DAG and check
  required generators, every retained row's derivation and every required
  variable-product closure obligation. State the exact invariant and degree
  convention. A fixed point at one degree does not establish exact last fall.

The older `pc_degree_harness.rs` describes a one-pass Macaulay row-space degree
as exact PC degree and rank deficiency as an operational first fall. Do not
import those names as certified labels. The selected last-fall definition uses
closure that can multiply derived polynomials within the degree bound, beyond a
single span of original-generator multiples. See [Huang, Kosters, Yang and Yeo,
Definition2.1 and Section2](https://arxiv.org/html/1505.02532v2), checked in the
coverage audit, and the bounded-search caveat in
[PREFIX-REPLAY-CERTIFICATES.md](PREFIX-REPLAY-CERTIFICATES.md). An exact exclusion
or universal claim needs additional checked evidence; successful identities do
not supply it.

## 2. Replayed SAT state and admissible-action supervision

Reuse the event machinery in `src/cryptanalysis/sat.rs`: formula snapshots,
decisions, reasoned assignments, conflicts, analysis, learned/deleted clauses,
backjumps, restarts and XOR operations. Complete the encoder gate map in
`binary_semaev_chain_sat.rs` and the dataset adapter. The current provenance
records summand bits but does not fully map internal gates to algebraic meaning.
Retain wire identities, AND constraints, XOR equations, finite-domain selectors
and field-basis semantics before consuming the circuit.

Add a bounded independent `src/ml_dataset/sat_prefix.rs` replayer. Public input
contains the reconstructed current CNF/XOR, gate graph and trail before a
verified decision. Restricted targets contain observed actions, later conflicts,
learned clauses and measured future costs. An observed trace provides imitation
labels; it cannot supply unexecuted counterfactual branch costs.

Acceptance checks:

- Reconstruct the formula from the gate map; verify propagation reasons and XOR
  transformations. Reject nonexistent clause IDs, contradictory assignments,
  invalid backjumps and rehashed tampering.
- Admit a learned clause only after an independent entailment check; otherwise
  end the certified prefix. Solver-reported UNSAT is not a proof-checked label.
- Use tiny exhaustively enumerable SAT/UNSAT controls, model-to-group lifts,
  truncation and sticky sink failures for software validation.
- Keep prime SAT unsupported until its own exact encoding exists; preserve that
  outcome in the complete campaign matrix.

## 3. Explicit factor-base populations

`curves.rs` currently constructs one small-x policy, and its verifier accepts
that policy's rule string. Introduce a versioned policy enum in `types.rs` and
plan/CLI configuration, retaining the existing policy as the compatible default.
First add a bounded Frobenius-orbit-union policy where the recorded curve model
has that action. Separate public candidate-domain semantics from restricted
sampling provenance and membership logs.

Bind the exact policy/domain through construction, verification, solver inputs,
relation transcripts and strict public projection. A subgroup projection must
not be relabeled an x-coordinate subspace. Polynomial-subspace and learned
proposal policies require separately declared constructions and checkers.

Acceptance checks:

- Reconstruct selected sets; verify subgroup membership, distinctness, sign
  actions, orbit closure and stabilizers. Reject cross-policy/domain transcript
  reuse and retain empty or capped constructions.
- Replay declared draws in a restricted sampling audit. Charge selection,
  rejection, projection, orbit expansion and membership verification separately.
- Use toy curve/twist fixtures for correctness before an authorized finite
  campaign. Model ranking targets require actual verified relation yield and
  rank, with construction costs and unsuccessful populations retained.

## Delivery boundary

Implement each target as a separately reviewable increment with exact checkers
and adversarial controls. Freeze an experiment before comparing models or
policies. Keep the whole-field-size partition until stronger transfer/isomorphism
components and frozen holdouts are actually constructed. Preserve prior-source
validation, and collect new-source receipts after runtime changes. These tasks
can advance locally as software work while worker publication remains gated;
none substitutes for actual cloud generation, independent corpus restore or
held-out model evaluation.
