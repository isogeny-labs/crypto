# N83 full-size U/D public-query panel

**Completed and audited:** 2026-10-20 02:06:17–06:48:53 UTC.
The frozen card completed all **18 natural attempts** over eight common
public points. The [native audit](../../full-m4-v1/final/audit.json)
passes **329 archived files**. Four constructed controls, four fresh
general-field replays and two cache off/on control-pair checks passed
before public search. Constructed rows are excluded from natural rank.

## Inputs and policy

Primary curve `icv1-f2m83-tm6151469093347-debefd74`, a=0, subgroup
`2417851639230796216685689`, cofactor 4, field polynomial
`z^83 + z^45 + z^2 + z + 1`. U is the retained public-x-hash orbit base;
D is the exported direct-subgroup dimension-18 base. Both contain
**16,627 signed-Frobenius columns and 2,760,082 points**, with different
accepted point sets.

The [protocol](../../FULL_M4_PROTOCOL.md) and
[host card](../../FULL_M4_HOST_CARD.md) fix the same public corpus,
worker, free selectors and phases, cache-on policy, 10,000 conflicts,
1,790 s internal / 1,800 s outer caps, 300,000-variable and
8,000,000-domain-clause limits. Each actual model has 201,598 variables,
5,915,238 clauses and 3,901 XOR rows. Target 0 uses U,D,D,U; later
indices alternate D,U and U,D. Validation scalars are excluded.

Worker revision: `d9ef42301cc3d22f437e1f89304e673836afb6a3`.
Worker SHA-256:
`d4324884fdd52ee05a82e2f053708d320d8255591ec61640b700a8a6684a0486`.
Corpus SHA-256:
`2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5`.
The archive retains the exact image, object, replayer and source hashes.

## Every natural outcome

| Attempt | Worker interval (s) | Endpoint | Completed conflicts | Completed decisions |
|:--|--:|:--|--:|--:|
| t0-u1 | 1790.013421 | wall cap | ≥2877 | ≥827319 |
| t0-d1 | 296.193176 | conflict cap | 10000 | 51772 |
| t0-d2 | 298.820652 | conflict cap | 10000 | 51772 |
| t0-u2 | 1790.016903 | wall cap | ≥2822 | ≥816126 |
| t1-D | 295.885548 | conflict cap | 10000 | 51770 |
| t1-U | 1262.609898 | conflict cap | 10000 | 512363 |
| t2-U | 1790.013302 | wall cap | ≥4196 | ≥773723 |
| t2-D | 288.384582 | conflict cap | 10000 | 51798 |
| t3-D | 291.750891 | conflict cap | 10000 | 51794 |
| t3-U | 1790.013781 | wall cap | ≥4195 | ≥765714 |
| t4-U | 1260.343154 | conflict cap | 10000 | 512363 |
| t4-D | 278.557023 | conflict cap | 10000 | 51787 |
| t5-D | 292.372367 | conflict cap | 10000 | 51795 |
| t5-U | 1482.674554 | conflict cap | 10000 | 585753 |
| t6-U | 1328.708755 | conflict cap | 10000 | 512363 |
| t6-D | 304.147710 | conflict cap | 10000 | 51804 |
| t7-D | 290.760986 | conflict cap | 10000 | 51789 |
| t7-U | 1790.017967 | wall cap | ≥4194 | ≥739312 |

Wall caps are `UNKNOWN_budget`; conflict caps are `UNKNOWN_search_cap`.
Every cap remains censored. Wall-cap counters come from the last durable
sample, with an explicit unobserved tail of 3.871–4.174 s. Conflict-cap
counters bind to the returned worker receipt.

| Base | Attempts | Conflict / wall caps | Sum of worker/cap intervals (s) | Accepted natural rows | Retained factor-row rank |
|:--|--:|:--|--:|--:|--:|
| U | 9 | 4 / 5 | 14284.411736 | 0 | 0 |
| D | 9 | 9 / 0 | 2636.872934 | 0 | 0 |

Rank is computed modulo the exact subgroup order, separately by base,
excluding controls and dependent rows. Zero is the rank of the retained
empty matrices. Natural yield, complete IC rank, the useful-rank panel,
`candidate_id` and `selected_best_total_runtime` remain null.
Different cap endpoints do not support a U/D speed ratio or base selection.

## Accounting and host

The worker intervals include import/index, corpus validation, model
construction, root propagation and search through the final/cap timestamp.
They end before final serialization. Container start/end records retain
the surrounding interval. Setup, full-object verification, controls and
fresh replay, Docker launch/teardown and custody remain separate charges.
This is not a complete cold IC cost or a cost per useful relation.

The Intel Xeon 6975P-C SSH Spot host used CPU 31, a one-CPU quota,
hard 4 GiB, zero swap, no worker network and read-only inputs/root.
Unrelated workloads shared the machine: **L0**. The closing snapshot
retains another workload on CPU 14. The five original wall-cap receipts
lack final cgroup peak readings; last durable process CPU/RSS observations
are lower bounds. The new watchdog resource implementation is a later
change and was not substituted into this card.

## Verification and remaining obligations

[The sealed archive and reproduction commands](../../full-m4-v1/final/README.md)
bind all 329 files. The native auditor checks exact inventory, source,
binary/corpus/object identity, Docker arguments/mounts/image/resources,
exits, serial schedule, controls, phases, progress, counter/resource
monotonicity and final/cap consistency. Returned natural rows would
require independent replay bindings before modular rank admission.
Fresh extraction and replay produce a byte-identical audit.

This completes the frozen full-size bounded comparison and preserves its
censored outcome. It leaves natural useful-rank collection, linear
algebra, individual-logarithm verification, qualified full cold/reuse
cost, the exact primary-curve IC incumbent, matched same-point rho and
external-fastest comparison open. No complete primary-curve IC baseline
is established by these stage cards; the different a=1 curve is not a
substitute. See the [problem record](../../../../problems/n83-prebuilt-base-quality-m2-m4.md).

The next discriminating stage is a separately frozen search or
partial-relation policy that can return independently checked natural
rank increments. The present capped attempts stay in the record.

## Visual record

[SVG](natural.svg), [150 dpi PNG](natural.png), [PDF](RUN_REPORT.pdf).
The plot reads the audited JSON directly. The PDF and identical graph
copies are also in `benchmarks/20261020-n83-full-m4-natural/`.
Canonical qualified-cost graphs and ledger rows are unchanged because
this bounded L0 panel supplies no qualified cost or rank improvement.
