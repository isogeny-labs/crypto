#set page(paper:"a4",margin:14mm)
#set text(font:"New Computer Modern",size:9pt)
#let data = json("../../xor-cache-v2/final/audit.json")
= N83 m=4: matched XOR-cache public comparison
_Frozen bounded component diagnostic · 2026-10-20 UTC_

The native receipt audit covers all sixteen natural attempts, four constructed controls and their four separate general-field replays. Each cell uses cache off/on/on/off on one public point with one identical worker binary. The second public point is a fresh m=4 holdout. Search semantics and work conservation are checked separately from elapsed-time observations.

All sixteen natural attempts returned `UNKNOWN_search_cap` at exactly 10,000 conflicts, with normal exits and no OOM. Natural yield and useful rank remain censored.

#align(center,image("natural.svg",width:97%))

The implementation caches a fully assigned XOR row only after checking that its residual is false. It invalidates on unassignment, matrix changes and incremental changes. The optimization remains disabled by default. It reduces repeated checks of satisfied rows; the same XOR row operations, row scans and search decisions remain charged. This component record does not select a factor base or establish complete index-calculus cost.

#pagebreak()
== Inputs, execution and correctness

Curve `icv1-f2m83-tm6151469093347-debefd74`, a=0, subgroup 2417851639230796216685689, cofactor 4, polynomial z^83+z^45+z^2+z+1. K2048: 2,048 columns / 339,968 points; D12: 2,001 columns / 332,166 points. Both use full signed-Frobenius closures. The accepted sets differ, so this is not a same-set prebuilt/on-demand comparison.

Public corpus SHA-256 `2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5`; indices 0 and 1. Validation scalars are excluded from every container. Cell order K2048/0, D12/0, K2048/1, D12/1. Each has A1/B1/B2/A2 with exactly 10,000 conflicts, one model, 1,790 s internal / 1,800 s outer caps, 300,000 variables and 8,000,000 domain clauses.

Worker source `1de47a9d4186bff08abc84c224a111183e820106`; corrected driver source `f1667c313910a1021daed455c9e48cd03524f42a`. Source, binary and input hashes were frozen before execution. All four constructed controls pass separate general-field replay; returned model and row digests match off/on within each base. Library validation: 4,403 passed, zero failed, 118 ignored. Directed invalidation tests and 400 exhaustive mixed CNF/XOR cases preserve verdicts, models and search counters.

== Host and accounting

Authorized SSH Spot host, Intel Xeon 6975P-C, CPU 31, one CPU quota, hard 4 GiB, zero swap, no worker network and read-only inputs/root. Other research and migration jobs share the host: isolation tier L0. The timed card is not concurrent with our remote compilation. The card's construction, controls, replays, failed launch history, raw progress and container process boundaries are retained.

The worker interval includes import/index, corpus validation, model construction, root propagation and search; returned relations additionally include checking and rank. Final receipt serialization, Docker launch, independent replay, construction/export, whole-base verification and custody are outside this interval. A component ratio cannot replace full cold/reused cost, linear algebra, useful-rank collection or matched same-point rho.

== Audit and evidence boundary

Archive SHA-256: #text(size:8pt, data.archive_sha256). Manifest: #text(size:8pt, data.manifest_sha256). The archive contains #data.manifest_files checked files. Auditor: `examples/n83_xor_cache_audit.rs`. It checks full inventory, source/input/binary identities, resource policy, exact mounted inputs, serial order, all phase/progress streams, independent replay bindings, exact repeated endpoints and row-visit conservation. Wall caps are retained as unmatched endpoints. Bounded search caps do not determine natural yield.

Sources in `research/n83_prebuild_cost_m23_20261019/`: `XOR_CACHE_PROTOCOL.md`, host and continuation cards, `xor-cache-v2/final/`, and this report's `RUN_REPORT.md`. Earlier missing-image and shell-driver failures remain under `xor-cache-v1/`. Earlier 100,000-conflict evidence is a different event and is not combined with this 10,000-conflict comparison.

Candidate, best-runtime and useful-rank-panel fields remain null. The original D18 export/replay is complete in the parent evidence. Complete natural-rank comparison, all cold/reused IC phases, same-point rho and best-base selection remain open. The next measurement must retain this correctness gate, inspect remaining propagation costs and use a qualified host before a timing promotion.
