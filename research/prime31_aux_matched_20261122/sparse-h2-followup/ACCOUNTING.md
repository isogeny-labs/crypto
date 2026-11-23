# Accounting and interpretation of the staged Prime31 comparison

This document defines the comparison's cost boundary and the meaning of its two stages. It reports no timing result, relation yield, scalar recovery, or speedup. The historical v1 panel and its receipts retain their original meaning. The v2 input inventory is prospective; generated inputs and successful correctness checks are not experiment executions.

Implementation references: [choice driver](../native/src/choice_main.rs), [collector](../native/src/collector.rs), [frozen plain reference](../native/src/plain_reference.rs), and [independent auditor](../native/src/audit.rs). The retained matching evidence is the [exact kernel fixture](../isogeny-choice/fixtures/kernel_correctness_fixtures.json), with its [scope and construction notes](../isogeny-choice/fixtures/README.md).

## Plain controls: the corrected comparison

The historical v1 plain-control path first constructed an auxiliary kernel fiber and lifted its reference base to learn the number of usable columns. It then constructed a plain base of that size. Its reported cold interval therefore included work that a plain solver, given the preregistered column count, does not intrinsically need:

\[
C_{\mathrm{plain,v1}} =
C_{\mathrm{reference\ construction}} +
C_{\mathrm{plain\ setup}} +
C_{\mathrm{plain\ collection\ and\ recovery}}.
\]

Those historical records are retained, including that overhead. They must not be silently reinterpreted as a standalone plain-algorithm cost.

The v2 plain cell instead binds the exact retained fixture by SHA-256, kernel, source-coset scalar, coordinate factorization, target arm, and usable-column count. The retained fixture covers coset scalar one only. The sparse H2 factorization has the same source curve, kernel, and complete coset x-set as standard H2, so its matching count comes from the same mathematical fixture. Counts for unverified new cosets are rejected.

The timed v2 plain path performs public-target transport and validation, plain-base construction and indexing, optional pair-table construction, collection, matrix work, and scalar verification. It calls neither the auxiliary kernel builder nor the auxiliary target-base preparation routine. Its reference event identifies previously prepared evidence; it is not a receipt claiming fresh auxiliary construction.

The offline auditor does more than compare count labels. It independently enumerates the specified kernel and the full coset, compares every retained x coordinate, independently checks the target lifts and transported columns, and then verifies that the plain base has the resulting number of columns. Changing the count consistently in both the cell and the event does not manufacture a valid match.

## Keep algorithm intervals and study expenditure separate

| Quantity | Included work | Interpretation |
|---|---|---|
| Auxiliary candidate's algorithm interval | Auxiliary construction and certification; target transport/lifting; base indexing; optional pair-table preparation; collection, matrix work, and scalar replay | Setup needed by this implemented candidate is charged to that candidate. |
| Plain candidate's algorithm interval | Its own target transport/validation, base construction/indexing, optional pair table, collection, matrix work, and scalar replay | Auxiliary reference reconstruction is absent. |
| Rho candidate's interval | Target transport/validation, the selected rho implementation's setup and walk, and destination/source scalar replay | Compare a verified complete solve with a verified complete solve. |
| Study preparation | Fixture construction, independent matching checks, compilation, admission and measurement qualification, plus other common preparation | Retain separately; do not attribute the complete reference construction solely to the plain candidate. |
| Outer process observation | Input reading/validation, driver work, logging, final output, and process overhead within the launched command | Required for the full launched-process cost and resource envelope. |

The v2 `algorithm_cold_seconds_diagnostic` field is an alias of the driver's existing cold interval. That interval starts **after `read_cell` returns** and ends **before the final cell-summary serialization and write**. It includes setup and intermediate event output within the interval. Input parsing and the plain-reference fixture lookup/hash in `read_cell`, final summary output, and process startup/shutdown are outside that internal clock but inside the outer process observation. The internal field must not be described as the entire process cost.

Historical reference-preparation time and priced cost are unavailable in the retained fixture. The new plain receipts therefore report them as null with `historical_matching_work_unmeasured_not_zero`. Null means unknown or unavailable; it is not a zero contribution. No arithmetic subtraction from a historical clock is used to fabricate a new measurement.

The study ledger must retain preparation and failed execution expenditure even when the algorithm comparison excludes shared matching work. A later amortized analysis must state the reuse count and exactly which preparation it amortizes. It cannot replace the observed first-use process cost or turn an unmeasured preparation into free work.

## Screening and complete-recovery attempts

Both v2 stages explicitly use `stop_after_first=true`: at most one signed relation is accepted from each public query. Their unchanged resource envelope is 110 seconds of collection, limited further by remaining driver time; a 120-second outer process envelope; 4096 MiB; and the same registered per-query and polynomial limits. The launcher sends TERM at 115 seconds and permits a five-second kill grace. The outer limit remains necessary because internal deadlines are cooperative.

| Stage | Natural-query ceiling | Intended evidence |
|---|---:|---|
| `screening` | 64 | Setup feasibility, first-hit behavior under caps, errors/timeouts, replayed rows, and early rank growth or recovery if it occurs. |
| `complete_recovery` | 65,536 | A bounded attempt to reach a useful dependency and verified scalar recovery, or an explicitly censored outcome. |

The larger ceiling removes the artificial 64-row bottleneck. It does not guarantee that the deadline permits that many queries, that a query yields a relation, or that the collected rows determine the scalar. The per-query deadline also does not apply to pair-table construction: that preparation is included in algorithm setup and bounded by the outer process envelope. If the process is killed before a setup report is returned, missing partial table-work counters remain unavailable; retain the outer cost and incomplete transcript.

For these fixed bases, the canonical logarithm-column counts range from 506 to 533. The historical 64-query first-hit cap therefore permitted at most 64 rows, often fewer. Full factor-base rank is nevertheless not a prerequisite for recovery. The collector accumulates

\[
A\ell = \alpha + \beta d.
\]

Any left dependency satisfying \(\lambda^T A=0\) and \(\lambda^T\beta\ne0\) gives

\[
d=-\frac{\lambda^T\alpha}{\lambda^T\beta}\pmod n.
\]

An early useful dependency is a valid success even below full rank. Conversely, a high row count or a full-rank factor-base matrix is not itself a recovered discrete logarithm. The scalar must be determined and independently replayed against the original public target; transported runs also check the destination. Duplicate coefficient rows with different public-query coefficients remain in the recovery process because they can be useful.

The 77-input screening inventory uses one paired query seed, 4188. Its per-stream configuration inventory is 35 auxiliary polynomial cells, 12 auxiliary pair-table controls, 12 plain polynomial controls, 12 plain pair-table controls, and six rho controls. The auxiliary polynomial count includes seven sparse-H2 representations. The prospective 385-input recovery candidate pool applies the same inventory to five prespecified seeds: 4188, 104729, 130363, 169087, and 224737. The pool begins with **zero selected candidates**. It is not an instruction to execute every candidate; shortlist and budget decisions follow screening evidence and must be recorded before launching recovery work.

These five streams share the same public target, field, and source coset. They examine query-stream variability within that fixed instance. They are not five independent curves or targets. Repeated execution of an identical stream is a timing replicate and must be labeled separately. Timing qualification and A/A admission remain separate from both input inventories.

## Pair tables, counters, and the final comparison

The pair-table control uses the same selected signed base and the same collector, public-query stream, row replay, and recovery logic as the polynomial arm. Its v2 mode is the production batched, negation-folded implementation. Different backends can return different first witnesses for the same query, so compare their actual replayed row and rank progress instead of assuming the selected rows coincide. Pair-table construction belongs to setup and must be included when comparing full solve cost. An online-query speed ratio that omits that construction is only a component ratio.

The mathematical auditor binds setup metadata to the checked column count and backend, checks event order, and replays returned signed witnesses. It does not reconstruct every stored pair-table entry or independently certify negative search exhaustion, operation counts, or runtime. Those limits are explicit in the audit result. The per-query native pair ledger remains available separately from the polynomial counter aggregate. Zero polynomial counters in a pair query mean that no polynomial work was reported; they do not mean the pair query was free.

Some operation ledgers overlap. For example, a group/field delta can be reported within backend work and also merged into the collector's ledger. Inversion-call metadata can overlap with the field products used for powering. Do not add duplicated or unlike counters and label the result total cost. Follow the repository's [common-unit and native-work rules](../../../AGENTS.md), retaining unpriced or incomplete work and the outer process observation.

Complete-recovery cost remains unknown until admitted attempts produce retained, independently replayed outcomes. No result here establishes a speedup over plain IC, the same-base pair-table control, or the registered rho implementations. A component improvement, a successful arithmetic fixture, and a verified complete solve are different evidence, and the report must keep those distinctions visible.
