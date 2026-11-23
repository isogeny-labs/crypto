# Prospective disjoint enrollment contract

Status: reviewable preparation design, not frozen and not scientific or resource admission. This contract was drafted with knowledge of the completed p739 observations. It preserves the original 1,260-workload objective and the failed clock-admission status of the first gate.

## Immutable original design and proposed partition

Retain the seven original cohort specifications and native plans unchanged. The proposed enrolled set has eight plans:

| Enrolled member | Workloads | Base executions | Measured candidate/reference pairs |
| --- | ---: | ---: | ---: |
| Original gate-p739 plan | 60 | 1,080 | 300 |
| New cohort16 remainder excluding p739 | 120 | 2,160 | 600 |
| Original cohort20/24/28/32/36/40 plans | 1,080 | 19,440 | 5,400 |
| Total | 1,260 | 22,680 | 6,300 |

The 22,680 slots include 18,900 measured arm executions and 3,780 warmups. Each workload retains candidate, rho and rho_AA arms, five measured streams and one warmup. No historical pencil method substitutes for this instrument.

The remainder specification is a new artifact derived from the original cohort16 specification by deleting only the exact p739 curve and changing declared label metadata if needed. Preserve every other field. Do not alter the original specification or manually rewrite raw records. Workload array indices, execution sequences and specification identity may differ; complete workload objects, resolved methods, parameters, arm/role, warmup flag, round, exact unsigned64-bit algorithm seed and per-workload arm ordering must match their original scientific slots.

The current instrument derives seeds and arm shuffle from workload identity and round, not workload-array position. That source observation motivates this design but does not replace checking the actual generated plan.

## Native preparation and certificate

Keep existing prepare/check commands and their original nine-specification inventory and 180-target cohort checks unchanged. Add a narrowly scoped disjoint preparation route that binds the original seven plans, original gate plan, original specifications and exact instrument binary. Its preparation operation may invoke only the pinned instrument's native plan-only interface, then compare the generated remainder plan against the original plan.

Plan-only generation is preparation, not a solver trial or new scientific measurement. It remains subject to root review of the implementation and input bindings before invocation. This note authorizes neither invocation nor a scientific execution.

The validator must prove an exact bijection between the original seven-plan scientific slot set and the eight-plan enrolled slot set, including all 22,680 slots. Reject duplicate/missing workloads or slots, changed exact seeds, changed methods/targets/rounds/warmups, altered per-workload arm ordering, and any undeclared specification edit. Cardinality alone is insufficient.

Preserve both original and enrolled plan hashes, specification identities, sequence numbers and workload indices in the mapping. Produce the ordinary prepared-plan validation receipt with its exact eight-file map and complete totals, plus a separate hash-bound disjoint-enrollment certificate recording the original seven inputs, enrolled eight outputs, instrument identity, permitted transformations and all mapping checks. Raw gate records retain their original plan, session, sequence and record IDs.

## Analysis and protocol lineage

The native full-cohort aggregation denominator remains 1,260 workloads and 6,300 measured pairs. Its existing per-plan record binding and duplicate-workload rejection remain active. The registered disjoint route must additionally require the enrollment certificate and exact original membership; it must not become an arbitrary smaller-denominator or arbitrary-plan admission path.

Reused gate data remain bound to their original frozen version5 protocol and actual instrument. New data would be bound to a prospective continuation protocol. Bind both lineages explicitly; do not relabel previous observations as collected after the continuation freeze. Coverage review, analysis and canonical export must check the lineage rather than accepting an unverified metadata assertion. The metadata-provenance erratum route must not be repurposed as scientific reuse approval.

A prospective scientific amendment and independent admission are still needed before reuse. Native preparation/equivalence does not override the old failed-gate stopping rule. Finite counted-cost completion and timing completion remain separate. Gate clocks stay invalid, and completion of the finite-cost track does not discharge the original timing obligation. Missing/capped/invalid outcomes remain explicit and do not reduce denominators.

## Upper gate and resources

The p186379 target0 gate remains a separately charged prerequisite with its independent replay. The other six cohort plans, including cohort32, are unchanged in this minimal design. Therefore the upper-gate record is not silently reused inside cohort32: gate work remains additional execution/resource work, not additional cohort coverage. Any further partition to reuse that target requires a separate reviewed extension.

No later-stage cumulative budget or deadline is supplied by this contract. Do not transfer or reset the first gate's 14,400-second allowance or its original absolute deadline. Preserve controls, warmups, gates, replay, failures, recovery, infrastructure and allocated-lifetime accounting. No solver, new empirical result, job, resource authorization, scientific admission or timing qualification follows from this note.

## Required software checks

Positive checks must verify exact eight-to-seven slot equivalence and all fixed totals. Negative checks must reject a changed high-bit u64 seed; missing/duplicated slot; overlapping gate/original cohort16 membership; method/parameter/target change; altered round/warmup/order; unauthorized specification edit; changed bound input hash; and relabelled gate record identity. Analysis/export integration must retain original gate clock failures and reject omitted enrollment or protocol-lineage evidence.
