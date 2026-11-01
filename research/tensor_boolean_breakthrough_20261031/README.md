# Tensor–Boolean breakthrough experiments

Date: 2026-10-31
Status: experiment design complete; E0 core implemented; no affected measurements taken
Repository baseline: `e3741fd5bbcc1f52a86be609587e7cf4ceeef566`

## Outcome

The next useful move is not an unrestricted SAT sweep. It is a staged attempt to find a **compressible invariant of the exact boundary tensors** produced by the hashed prime-field ECDLP encoding. The first decisive study is a boundary-tensor flattening-rank census against topology-preserving rewired nulls. It can reject the most direct version of the low-rank hypothesis cheaply, and a positive result tells us which contraction engine to build.

The portfolio below turns the proposed workflow into six gated experiments plus one SAT/BDD triangulation study. Experiments 0–1 are the immediate work. The others are not authorized to consume benchmark budget until their predecessor gate passes.

## Portfolio

| ID | Experiment | Question | Decisive observable | Advance gate |
|---|---|---|---|---|
| E0 | Canonical IR equivalence and mutation gate | Can every compared network be regenerated, hashed, and replayed without changing its semantics? | Exact count, every interface-prefix count, least witness, mutation rejection | 100% round-trip equality and all declared mutations detected |
| E1 | Boundary-tensor rank census | Do natural hashed networks have lower balanced flattening rank than graph- and predicate-preserving rewired nulls? | Exact integer rank on admitted cuts; normalized rank growth | Held-out target/null gap and slope gap pass the frozen thresholds |
| E1S | SAT/BDD mechanism triangulation | Do exact Boolean solvers exploit the E1 signature, rather than merely receive a differently named input? | Complete solver work, peak state, and rank/work correlation on the same pairs | Same-sign natural/null effect in two frozen solvers and held-out rank/work association |
| E2 | Local Fourier and partition-rank census | Is any E1 compression explained by a stable local basis or predicate family? | Certified local ranks and composed factor dimensions | A composition certificate predicts at least 20% lower held-out exact work |
| E3 | Certified rewrite/order search | Can semantics-preserving rewrites expose that invariant and lower exact contraction cost? | Exact operations, peak live states, equivalence certificates | At least 2x held-out improvement on the three largest sizes, plus a lower growth slope |
| E4 | Structure-specific exact contraction engine | Does a rank/Fourier-aware engine beat the strongest exact baseline on the same IR? | Exact semiring/bit operations and peak bytes; wall time secondary | At least 2x operations and memory on the two largest held-out sizes, all answers equal |
| E5 | End-to-end relation oracle | Does the mechanism survive relation extraction, failures, verification, and linear algebra? | Complete relation yield and fully priced work | Improvement on four increasing sizes and an unseen target; still no asymptotic claim |

The machine-readable cards are in [`experiment-matrix.json`](experiment-matrix.json). E1 has a narrative preregistration in [`PROTOCOL.md`](PROTOCOL.md) and a schema-valid draft record in [`protocol.draft.json`](protocol.draft.json).

## Why E1 comes before solver search

SAT, tensor contraction, BDDs, variable elimination, and algebraic elimination are different ways of exploiting repeated low-dimensional messages. A solver race can show that one implementation wins on a ladder; it usually does not identify the reusable mathematical reason. E1 asks directly whether the natural network's boundary tensors occupy a smaller space than a close null. E1S then gives the SAT hypothesis a fair test on exactly those paired instances: parsimonious CNF/CNF-XOR exports, two pinned solvers, full setup and verification work, and a preregistered rank/work association.

The null changes only the global wiring of interior ports while preserving the vertex predicates, vertex degrees, edge count, interface, local evaluation code, cut width, and each vertex's cut-degree profile. Conditioning on a preregistered solution-count stratum prevents a trivial “one family is mostly unsatisfiable” explanation. This does not make the null uniquely correct; the report must show both conditioned and unconditioned panels.

## Immediate implementation status

1. **Implemented:** a canonical, versioned network IR with stable edge/port ordering, interface order, pins, predicate semantics, and provenance.
2. **Implemented on tiny controls:** exact IR round trip and all five E0 mutations. See [`E0_IMPLEMENTATION.md`](E0_IMPLEMENTATION.md).
3. **Implemented 2026-11-01:** the exact `tensor-census` core materializes the boundary tensor for an explicit canonical vertex cut and computes fraction-free integer ranks. See [`E1_IMPLEMENTATION.md`](E1_IMPLEMENTATION.md).
4. **Next:** add the production semantic cut generator, matched rewired-null constructor and fresh-process A/A runner; then run only scratch preflight fixtures disjoint from the declared ladder and fill binary, analysis, runtime, memory, and power fields.
5. **Only after that:** freeze and commit `protocol.json`, derive seeds from its hash, and generate E1 data.

The current `Network` stores predicates behind `Arc<dyn Predicate>`, so closure identity cannot be serialized faithfully. E0 therefore uses a hybrid semantic representation: predicates at or below 16 ports carry their complete truth table; larger production predicates carry a strict typed operation that is also their live evaluator. Tag, parameters, truth table or typed program, port order, interface and pins are bound into one SHA-256 identity. Untagged predicates are admitted only through an explicit bounded-truth-table policy for tiny fixtures and controls. Canonicalizing graph shape alone would be insufficient.

This hybrid requirement is a pre-freeze clarification of the original tag-only draft. Tags make the IR interpretable but do not by themselves prove that an imported closure has the same semantics. An initial universal-table implementation was rejected by its own representative control: the tiny hash network contains a 132-port Fourier relation, so a 16-port cap cannot cover production. The typed program supplies exact, replayable semantics for those vertices without materializing a `2^132` table. No affected data preceded either correction.

## Interpretation discipline

- A low finite-size rank is a mechanism lead, not a subexponential ECDLP result.
- A full-rank E1 result rejects this registered boundary-compression route on these cuts and sizes; it does not rule out every Boolean or algebraic representation.
- An E1S win nominates a specific solver/encoding bundle. It does not establish that “SAT” in general is the mechanism, and an E1S loss does not negate a passing E1 rank result.
- Local low rank in E2 does not advance unless a proved composition rule lowers a global message bound.
- E3 search uses a frozen training panel and an untouched holdout panel. Search-time wins do not count.
- E4 compares complete target-dependent work on identical semantic instances. Setup, precision growth, failed candidates, extraction, and verification remain charged.
- E5 is the first stage allowed to use the phrase “relation-oracle improvement.” None of E0–E4 establishes an ECDLP speedup.

## Requirement-to-evidence map

| Requirement | Artifact | Status |
|---|---|---|
| Devise multiple experiments | This portfolio and `experiment-matrix.json` | Complete at design scope |
| Make the next action concrete | E0 implementation slice and E1 protocol | Complete at design scope |
| Include controls and falsifiers | `PROTOCOL.md` §§ Controls, decision rule, invalidation | Complete at design scope |
| Prevent post-hoc search claims | Frozen train/holdout policy in E3 and protocol freeze gate | Complete at design scope |
| Implement semantic comparison gate | `canonical.rs` and `E0_IMPLEMENTATION.md` | Core implemented and tiny controls verified; final-ladder preflight open |
| Produce measured evidence | No rows or exports exist | Not started; intentionally blocked by draft status |
| Establish a tensor/Boolean breakthrough | Requires E1–E5 evidence and, for asymptotics, a theorem | Open |

## Files

- [`PROTOCOL.md`](PROTOCOL.md) — E1 estimand, construction, controls, analysis, and gates.
- [`protocol.draft.json`](protocol.draft.json) — draft `experiment.protocol/v1` record.
- [`experiment-matrix.json`](experiment-matrix.json) — machine-readable experiment cards and dependencies.
- [`E0_IMPLEMENTATION.md`](E0_IMPLEMENTATION.md) — canonical-IR format, controls, evidence boundary, and open obligations.
- [`E1_IMPLEMENTATION.md`](E1_IMPLEMENTATION.md) — exact boundary contraction/rank core, local controls and remaining generator gates.
- [`SEARCH_20261101.md`](SEARCH_20261101.md) — reconciliation of the requested separator, small-root and special-prime leads.
- [`REPORT.md`](REPORT.md) — visual decision brief.
- `report.pdf` — rendered report after validation.
- [`VALIDATION.md`](VALIDATION.md) — generated-artifact and protocol checks.
