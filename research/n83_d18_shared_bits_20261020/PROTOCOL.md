# D18 shared root bits: frozen implementation and comparison protocol

Frozen before implementation or execution on 2026-10-20. The baseline
source is `e294f8c927b019f59e6552039d32e6b4ba237f84` from PR 2271.
The original U/D card and its frozen worker remain historical evidence.
This follow-up tests a change to the exact finite-domain encoding.

## Requested obligations

| Obligation | Acceptance |
|:--|:--|
| Finish PR 2271 | Approved index rows, repaired CI, applicable final-head checks and merge into its named parent. |
| Preserve the factor base | Identical representatives, points, column/phase/sign labels and subgroup; both encodings admit exactly the same x-domain. |
| Implement shared bits | Explicit opt-in policy; unchanged default; checked size arithmetic; all rejected input and cap cases leave the solver untouched. |
| Establish semantics | Core Lean proof of selector-implication factorization, exact hypotheses and proof map; exhaustive native accepted-domain/phase checks and adversarial caps. |
| Full D18 counts | Read the pinned 16,627 representatives, check their identity, and build both encodings for m=2,3,4; compare predicted and actual variables, clauses and XOR rows. |
| Common-public m=4 follow-up | One binary, same base, public points, deterministic settings, limits and host; independently replay every returned relation; preserve every cap. |
| Complete IC comparison | Remains open until natural useful rank, linear algebra, all cold costs, independently verified answers and matched same-point rho are measured. |

## Exact encoding change

The current representative selection is exactly-one. For each root bit,
every selector implies that representative's value. If **every**
representative has the same value at a bit, those implications are
equivalent, under the existing at-least-one clause, to one root-bit unit.
Varying bits retain every selector implication. This applies to shared
ones as well as shared zeros. The phase transforms, selector order,
variable allocation, bounded phase code and final x links are unchanged.

For K distinct representatives, n root bits, c shared bits and m equal
summand domains, the proposed clause reduction is `m*c*(K-1)`. Variables
and XOR-row counts are unchanged. This is a formula to check against the
actual construction, not an observed solver speedup. The direct-subgroup
D18 source coordinates are below `2^18`; the actual shared mask must be
computed from the retained selected coordinates and exported rows.

Use the curve `icv1-f2m83-tm6151469093347-debefd74`, a=0, field polynomial
`z^83+z^45+z^2+z+1`, cofactor 4 and subgroup order
2417851639230796216685689. The full D object is
`e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682`;
the full U control object is
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.
Both contain 16,627 classes and 2,760,082 labeled points.

## Validation before a natural run

1. Retain an unmodified baseline binary, source revision, compiler and
   host record before changing the encoding.
2. Implement native exhaustive small-field checks against an independently
   constructed Frobenius closure. Pin each valid/invalid phase and
   representative selector. Include shared-one, shared-zero, no-shared-bit,
   unsorted, duplicate, out-of-range and heterogeneous-domain cases.
3. Check exact cap acceptance and one-under rejection before mutation;
   compare actual allocation with preflight sizes for both policies.
4. Build the actual D18 m=2,3,4 domain models under bounded memory. These
   are construction/count checks and do not replace their natural-query
   policies. Retain failed attempts. Build counts are the primary output.
5. Check the Lean theorem with its pinned core toolchain and inspect its
   transitive axioms. Link its exact statement to the emitted clauses.
6. Freeze the final native binary, source, inputs and command card before
   any paired solver execution. Run constructed controls and independent
   general-field replay first; stop on an invalid witness or policy drift.

## Paired m=4 policy

Use the existing authorized SSH Spot host. A later host card binds the
actual host, binary, source, wrappers and image before launch. One CPU,
hard 4 GiB, zero swap, no worker network, read-only inputs, no scalar
sidecar. Preserve host activity; shared-host observations are L0.
Qualified timing requires a separate uncontended isolation receipt.

Both encodings use the same binary, XOR-decided cache on, identity mask
zero, free selectors/phases for natural queries, one returned model,
10,000 conflicts, 300,000 variables and 8,000,000 domain clauses.
Internal deadline 1,790 seconds and independent outer cap 1,800 seconds.
Retain durable ten-second progress plus CPU/RSS/cgroup receipts.

The public corpus SHA-256 is
`2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5`.
Use all eight public points. Define A as original per-representative
encoding and B as shared-bit encoding of **D**, with the same accepted
point set. Target 0 order A,B,B,A; targets 1 through 7 alternate B,A for
odd indices and A,B for even indices: eighteen natural attempts total.
Any returned row requires fresh general-field replay; useful rank is
computed from replayed rows only. Capped outcomes remain censored.

Before natural queries, run five constructed-control A/A pairs and five
D A/B pairs, plus one full-U A/B pair. Freshly replay all 22 control
receipts. Require the same checked point equation, sparse row and
complete SAT assignment within each matched control pair. Search
counters may change with earlier unit propagation; do not require or
claim equal search paths. A/A and A/B resource observations are reported
separately. A model mismatch stops the natural card for investigation.

The full card has at most 18 natural workers plus 22 controls and their
22 replays, each with an independent 1,800-second cap. A failure in a
control/replay stops before natural queries. All actual intervals,
serialization, setup, failures and verification are retained. Runtime
ratios require matched completed work and qualified host conditions;
different capped endpoints give no speed ratio. No fresh host or broader
budget is authorized by this document.

## Decision boundary

Exact accepted-set equivalence and fewer clauses establish an encoding
size reduction. Returned and independently replayed natural rows can
advance the useful-rank gate. Neither alone establishes lower complete
IC cost. `candidate_id` and `selected_best_total_runtime` stay null until
the original end-to-end acceptance fields are present. External solver
comparison and matched rho remain explicit obligations.

Every executed card receives its native audit, report, graph and inspected
PDF; timing records also receive matching benchmark browsing copies.
Prior evidence and baseline measurements are never rewritten.
