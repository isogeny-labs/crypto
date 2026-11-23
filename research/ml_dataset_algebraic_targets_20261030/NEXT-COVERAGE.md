# Next dataset coverage: causal SAT supervision and remaining requirements

Planning audit, 2026-10-30 UTC. This is **not a frozen execution contract** and
does not authorize a new experiment, generation wave or IAM change. The audited
runtime source is `db9bd045e38f2e55039d02372b92b5bee3f29ba0` (abbreviated `db9`
in the handoff). File/line pointers below refer to that source. No source edits,
builds or tests were performed for the read-only audit.

At document creation, the retained attempt 02 pipeline receipt records this
source, `last_phase=algebraic-cli-02`, `exit_code=1`; its CLI phase receipt says
`stage=command_failed`. The parent is correcting the fixture and arranging
continuation. This document does not reinterpret that failure as a validation
pass or independently diagnose it. Existing receipts remain authoritative.

The original objective remains Koblitz, general binary and prime curves, both
curve/twist positions, n=23..31 and 47..61 including p=2^61-1, m=2..15, multiple
generators/keys, all named solver families, intermediate artifacts, precise
costs and durable replayable publication. Implemented adapters and local
software checks do not establish a solved cloud corpus. **Verified solved cloud
dataset cells remain 0.** No new generation or worker-access evidence is supplied
by this audit.

## Recommended next implementation increment

The highest-impact next software increment is a causal SAT training channel:
retain the encoder's algebraic gate map, independently replay bounded solver
prefixes, and export verified pre-decision contexts. This serves the requested
novel-SAT-solver objective and adds supervision for binary/Koblitz instances
beyond the current prime-chain learning channels. It does not itself extend
mathematical solver coverage or establish a better solver.

The reviewed design intent is in [DESIGN.md](../../docs/ml-dataset/DESIGN.md)
and [OPERATIONS.md](../../docs/ml-dataset/OPERATIONS.md). The earlier
[next-target roadmap](../../docs/ml-dataset/NEXT-STRUCTURE-TARGETS.md) already
identifies this channel; the present audit supplies concrete reuse and
admission boundaries.

| Concern | Code pointer at audited source | Current behavior and missing work |
| --- | --- | --- |
| SAT adapter | [solvers.rs](../../src/ml_dataset/solvers.rs), `sat_solve`, line 943 | Builds finite-domain chained-S3 CNF/XOR, saves original formulas, records native events, checks SAT assignments and lifts decoded coordinates in the group. |
| Missing gate semantics | Same file, line 1111 | The recorded scope explicitly says only summand-block meanings are exported; remaining gate provenance is absent. |
| Gate-map donor | [binary_semaev_chain_sat.rs](../../src/cryptanalysis/binary_semaev_chain_sat.rs), `Circuit`, line 175; `finish`, line 263 | The encoder already retains AND triples, XOR rows, units, field reduction and squaring maps before consuming them. Preserve those semantics together with summand/intermediate/target block roles. |
| Finite-domain restriction | Same file, `constrain_summands`, line 401 | Domain constraints must join the retained map; the S3 circuit alone does not identify the actual constrained PDP domain. |
| Typed search events | [sat.rs](../../src/cryptanalysis/sat.rs), `SolverEvent`, line 97; `set_event_observer`, line 918 | Existing snapshot, clause, assignment, decision, conflict, learning, deletion, backjump, restart and XOR-operation events provide replay input. |
| Replay donor and limitation | Same file, test-only `replay_observer`, line 3227 | It checks some trail/reason/parity behavior but accepts learned clauses without entailment proof, accepts snapshot/XOR-matrix provenance and does not fully enforce level transitions. Do not promote it unchanged as an admission checker. |
| Export integration | [export.rs](../../src/ml_dataset/export.rs), sorters near line 789 | Reuse bounded sorting and strict public/restricted separation. There is currently no SAT-prefix admission path. |

### Part 1: retained and independently checked encoding

Add a versioned gate-map artifact containing exact wire roles, AND constraints,
XOR rows, constants, field basis/reduction rules, finite-domain constraints and
identity-pattern semantics. Reconstruct its constraints and bind them to the
exported CNF/XOR and attachment snapshot. Account explicitly for encoding-time
normalization; do not assume the installed formula is byte-identical to the raw
gate expansion. Independently evaluate small gate maps on known assignments
and check decoded point/group lifts.

The bounded implementation scope is the existing binary chained-S3 adapter.
Keep unsupported prime SAT and excluded/capped parameter cells explicit. Orbit
selector or ordering encodings require their own complete gate semantics before
they can be admitted under the same schema.

### Part 2: bounded causal replay and export

Add `src/ml_dataset/sat_prefix.rs` and integrate it into native dataset export.
Reconstruct clause IDs, parity rows, assignments, levels and trail boundaries;
check each transition before admitting a decision context. Require independent
entailment evidence before accepting learned clauses into a certified state.
A bounded propagation check can establish some additions; an exhausted or
inconclusive check ends that certified suffix without invalidating earlier rows.

Public features contain only the verified current formula/gate graph, trail
and levels **before** the decision. Selected literals, later conflicts, learned
clauses, outcomes, costs and event provenance remain restricted. In particular,
`SolverEvent::Decision.activity_bits` describes the already selected variable.
The stream does not supply a complete pre-decision activity vector, so the
selected-variable value must not be exported as generic public heuristic-state
input. Observed decisions are imitation labels, not optimal actions or
counterfactual branch costs.

Before implementation, freeze separate cumulative limits. A candidate envelope
is 4,096 examined events, 64 MiB derived payload and 10 million explicitly counted
literal/parity-word verifier operations. These are **proposed software limits**,
not measured feasibility claims or an approved experiment. Specify all count
units and retained-state limits in the subsequent contract. Preserve earlier
admitted rows and emit one restricted censoring record when a channel reaches
its bound; keep native solver work and verifier work separate.

Acceptance checks should cover wrong clause IDs, forged learned clauses,
changed XOR rows, malformed masks/literals, invalid levels/backjumps,
reordered/truncated streams, source/formula/gate-map substitution, and actual
immutable CLI export/retry. Future events must not change earlier feature
bytes. Solver-reported UNSAT remains unverified unless the complete proof
obligations are independently checked. Tiny exhaustive SAT/UNSAT controls and
model-to-group checks are software validation, not evidence of practical
full-size solves.

### Primary-source check

Reviewed Cruz-Filipe, Heule, Hunt, Kaufmann and Schneider-Kamp,
[Efficient Certified RAT Verification](https://arxiv.org/html/1612.02353v2),
Sections 2–3. The paper defines checking additions via unit propagation and
explains explicit proof hints. Reuse established RUP/LRAT ideas rather than
calling recorded analysis events proofs. Native XOR transitions need separately
checked parity reasoning; they are not automatically covered by a clausal proof
format. No novel proof-checking or solver-performance claim is proposed here.

## Other remaining requirements

| Requirement | Current source boundary | Remaining obligation |
| --- | --- | --- |
| Families/arity | `solvers.rs:326` declares Boolean F4/F5 and Weil primarily at m=2, geometric F6 at m=4, standalone roots at m=2, and PKM at m=2/3, p<2^31. Recursive prime F4 covers m=3..15 below 2^32; supervised wide-prime F4 is m=2. Prime SAT is unsupported. | Extend legitimate native coverage and independently verify actual outcomes. A capability row or structured cap is not a solve. |
| SAT admission at requested sizes | `sat_solve` passes a 100,000-variable construction cap. `chain_s3_max_variables` at `binary_semaev_chain_sat.rs:25` gives n(2m-1)+4n(m-1)+3n²(m-1). At n=61, m=10 this is 103,822. | Preserve structured capped outcomes even within the advertised m=2..15 range. A higher cap requires measured memory/work admission; no broad practical coverage follows from the formula. |
| Factor-base populations | `curves.rs:820` constructs one small-x/projected policy; `verify_factor_base` at line 1309 accepts that rule explicitly. `CaseConfig` at `types.rs:6` has a size but no policy enum. | Introduce a versioned policy enum and verified orbit-union construction as a separate increment. Existing orbit witnesses do not mean the selected point set is an orbit-union population. Bind policy/domain through verification, solver inputs, relation transcripts and public feature identity; retain construction/rejection costs. |
| Instruction measurements | [telemetry.rs](../../src/ml_dataset/telemetry.rs), line 34, already instruments phases; checked telemetry retains producer-consistency evidence. Counts exclude startup, publication and preexisting worker threads. | Admit actual worker PMU configuration, interval/thread coverage and readback. Do not relabel phase counts or native abstract operations as whole-solve instructions. |
| Trace completeness | Current adapters retain bounded semantic events and selected algebraic states. `solvers.rs:326` still lists private matrix/critical-pair, SAT minimization/watch/heap, filter and interrupted-call gaps. | Preserve missing/partial status; add specific native instrumentation and replay obligations. Root adapters already retain bounded factorization/GCD/split states in current source, so do not describe those artifacts as wholly absent merely because an older coverage table is stale. |
| Durable corpus and learning efficacy | Native CLI/publication machinery and software artifacts exist; actual verified solved cloud cells remain 0. | Resolve existing authorized worker-access/deployment gates, demonstrate finite generation/restore without duplicates, then freeze held-out model comparisons including inference, rejection and fallback cost. No trained-model advantage or novel SAT solver is established. |

This planning increment preserves the full campaign objective. It does not
replace the wider curve/twist/solver coverage, exact fall-degree proof
obligations, whole-run cost accounting or actual durable cloud corpus.
