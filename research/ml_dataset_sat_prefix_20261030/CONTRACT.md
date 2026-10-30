# Causal binary SAT dataset contract

Frozen before source edits, 2026-10-30 UTC. Source base51e0c7abe735, isolated
worktree crypto-ml-sat-prefix-20261030. Conductor T907 owns the native encoder,
new sat_prefix module and this research directory; T828 delegates exporter/docs
integration in this worktree. Shared solvers/mod paths remain reserved by T887;
no edits there until coordinated permission/ownership. Parent consequence full
validation runs separately and its frozen source must not change.

This is software development with retained synthetic correctness fixtures, not
a scientific experiment, a cloud launch or a solver-performance comparison.
The full original Koblitz/binary/prime, curve/twist, n23..31/47..61,p2^61-1,m2..15,
all-method/cost/artifact corpus goal remains intact. Verified cloud cells0.

## Prior work and learning objective

Read existing NEXT-COVERAGE.md and native sat.rs/Circuit/event emitters. Primary
source: Cruz-Filipe et al., Efficient Certified RAT Verification,
https://arxiv.org/html/1612.02353v2, sections2-3. Unit propagation under the
negation of a candidate clause can certify entailment. RAT-only satisfiability
preservation does not establish logical consequence and is insufficient here.
Native mixed CNF/XOR transitions need separately checked parity reasoning.

Learn to choose legal branch literals from a verified causal formula/trail and
its algebraic gate graph. Restricted targets record the native choice as an
imitation label, not an optimal action or counterfactual cost. No solver or
cryptanalytic advantage follows from producing this channel.

## Opt-in retained encoding

Extend ChainedS3Encoding with build_with_gate_map using the existing builder,
not another field arithmetic engine. Ordinary build retains existing behavior
and no gate-retention copies. GateMap is versioned serde data containing exact
n/m, canonical irreducible low terms, decimal b/optional target_x, identity mask,
variable count, coordinate wire blocks, field reduction and squaring maps,
ordered raw AND triples [output,left,right], XOR wire lists/RHS, units,
impossible flag, and installed finite-domain sorted unique decimal codes with
raw clause count. Compact S3 node wire roles may be included to identify
algebraic operations; they must be deterministic and exactly regenerated.

GateMapLimits bounds serialized map bytes at16MiB; use a bounded counting writer
before creating the serialized buffer. A typed getter distinguishes disabled,
byte-capped, domain-not-installed and unsupported mutation. Capture may become
unavailable without changing the solver's mathematical state. The dataset
adapter must save a complete admitted map or stop before search with an explicit
coverage/cap record. Successful unsupported orbit-domain, ordering and pin
mutators invalidate retention; failed preflight calls do not. The public solver
field can still be mutated, so the getter alone is never a verification claim.

Initial integration covers the existing exact finite-coordinate domain and one
solve per identity pattern. It does not pretend to export complete semantics
for orbit selectors, ordering constraints, lazy theory or model enumeration.
Those remain explicit extensions, not silently accepted axioms.

## Source and formula admission

Bind each map to the already verified cell's field/basis, curve b, public target,
arity, identity pattern and exact factor-base x-domain. Regenerate it using the
existing native encoder and require exact equality. This is native regeneration,
not an independent encoder implementation or independent field engine.

In sat_prefix.rs, separately reconstruct constraint installation from the raw
map, without trusting the producer's solver state. Preserve insertion order:
three Tseitin clauses per AND, normalized XOR rows, units, impossible empty
clause, then finite-domain prefix clauses. Clauses sort/deduplicate, discard
tautologies, simplify against previously installed root units, immediately
assign accepted units, and retain sticky contradiction. XOR lists cancel repeated
variables; empty-false vanishes and empty-true sets contradiction. Construction
does not perform general CNF propagation. Compare ordered stored clauses/XORs,
explicit terminal empty clause, variable count and initial units to saved DIMACS
and the complete attachment snapshot. Track exact arena IDs/length increments.

Support the dataset's fresh pre-search attachment: empty trail limits and empty
initial XOR matrix, with exactly checked construction-time assignments. A valid
but unsupported pre-propagated snapshot censors the suffix. Malformed counts,
IDs, masks, coefficients, literals, fields or inconsistent bindings are errors.
No decisions are admitted before the whole snapshot matches. The normalized
formula and GateMap must come from the same pattern, target and source artifact.
Imported byte/vector/string/wire bounds are checked before expensive parsing,
BigUint conversion, traversal, allocation or reconstruction.

## Independent bounded semantic replay

Use a new private sat_prefix module from the exporter. Maintain ordered clauses
with stable IDs, active/deleted flags, original XORs, current matrix, trail,
assignment levels and decision boundaries. Validate each event against preceding
accepted state. Native counters and producer booleans are not admission proof.

- Require contiguous event sequence and bound pattern association. One snapshot
  and one solve per admitted pattern; a later snapshot cannot reset trust.
- Original constraints and initial assignments must exactly match admission.
- Clause reasons must name an installed or checked pending clause and prove the
  assignment unit at that point. Initial and later assignment flags, indexes,
  levels, duplication and literal ranges must be consistent.
- XOR reset restores admitted original rows. Row-XOR checks both indexes, width,
  padding bits and RHS and performs one exact operation. Implications/conflicts
  validate full parity support and explanation signs; subsequent XOR assignments
  must match the checked implication, not an arbitrary supplied reason.
- Independently check every Learned clause by bounded RUP using the admitted
  original CNF, earlier entailment-checked lemmas, and certified parity rows or
  explanations. Negate the candidate and propagate CNF units/parity single-free
  rows to contradiction. Do not include current decisions as proof assumptions.
  Fixed point without contradiction means ProofNotEstablished, not nonentailment.
  Missing native minimization steps are not silently accepted as proof.
- Learned is emitted before backjump and installation. Reserve the checked ID;
  allow learned-unit Assignment before its confirming Clause event. Exact repeated
  declarations confirm the same clause and never replace its contents.
- Validate backjump levels, retained trail and removed literals, and restart and
  deletion references. Deleted proven lemmas may remain in a bounded proof archive
  but are absent from active public state. No active reason may be invalidated.
- Unmatched dynamic clauses/XOR additions, lazy theory, additional solve calls or
  model blocking are unsupported transitions; they never become new axioms.
- Check conflict/reason and analysis-reference semantics where reported; do not
  claim the event stream captures minimization, watch lists, heap or whole execution.

At Decision, construct context from the preceding replay state, before the new
level or assignment. Hold its restricted action pending until the matching
Assignment is verified. A truncated decision/assignment pair is not admitted.
A later unsupported or bounded transition retains earlier admitted examples and
emits one restricted unknown-suffix record. No later event resumes admission in
that pattern. A new pattern may be considered only after its own complete source
and snapshot admission, within the same cumulative channel budget. Global budget
exhaustion stops the entire channel, with one global terminal record.

## Bounds and measured work

Use producer-aligned maximum100000 raw events and16MiB raw event bytes, separately
4096 admitted decision contexts and64MiB derived serialized feature/record payload.
A terminal censoring record is separately bounded at4096bytes. Never use4096 as
the raw-event cap: n23,m2 emits4761 initial AND-clause events before any decision.
Input variables max100000; source formula/map input limits and retained logical
state limits are explicit. Retained verifier state is bounded at64MiB by declared
logical element/byte accounting; this is not an RSS measurement.

One cumulative budget of10000000 literal visits/parity-word operations covers
formula normalization, snapshot checks, replay and learned-clause checks across
patterns. Specify exact charge units in code/records; count partial work at caps,
charge before expensive work, and do not reset per event/lemma/pattern. Encoding,
parsing, hashing, sorting, I/O and native solver work remain separately scoped;
no abstract verifier count is called CPU instructions. Public features exclude
all costs/caps/outcomes; restricted rows distinguish proof miss, cap, unsupported
profile, truncation and native unknown.

## Public/restricted export and compatibility

Add sat-features.jsonl and restricted/sat-records.jsonl under a new explicit
allowlist version, keeping existing static/prime-prefix/algebraic feature bytes
stable. Reuse existing sorters, hashing, immutable installation and replay gates.
Public inputs: task binary-sat-decision/v1, initial public problem, exact verified
gate graph, active CNF/XOR state and causal trail/levels. Stable feature identity
must not depend on chosen action, source event IDs, solver outcomes or future
suffix. Current entailed learned clauses may enter later contexts only.

Restricted rows: chosen literal/level and event provenance, checked proof/replay
work, censoring reasons and source joins. Selected-variable activity_bits and
priority are restricted: the event does not provide a complete pre-action vector.
Models, decoded coordinates, recovered scalars, group lifts and future costs
remain restricted. Keep full native instruction coverage, UNSAT proof checking
and native trace completeness false; local entailment certification is narrower.
Historical source cells missing the new map get an explicit unavailable channel,
not fabricated gate semantics. New SAT cells require saved map before solve.

## Meaningful acceptance and delivery

Unit controls must reject malformed IDs/literals/masks, formula/map substitution,
forged propagation/explanation, changed parity operation, invalid levels/backjump,
unproved learned clauses, unknown dynamic constraints, reordered/duplicate or
truncated streams. Prefix-preservation tests must keep earlier feature bytes
unchanged when a later suffix/cap changes. Test a learned unit, nonunit minimized
lemma, restart/deletion and explicit unsupported model-blocking transition.

The actual CLI fixture must produce nonzero verified decision rows for curve and
twist, verify full joins/hashes, deterministic retry, unchanged source inventory
and corruption rejection/censoring. Require a native PDP example with a decision
before and after a certified learned/backjump transition. If no bounded retained
example has that transition, report the gate missing; zero-row exports and
solver-only traces cannot satisfy this PDP integration obligation.

No competing build while the parent full-library run is active. Commit contract
before source edits, freeze runtime before tests, run scoped tests/strict lint/
actual CLI/full release library checks, preserve failed attempts, archive to S3
with fresh readback, then deliver Git/PR with exact source/check scope. No new
cloud wave, IAM change or scientific promotion is authorized by this contract.
