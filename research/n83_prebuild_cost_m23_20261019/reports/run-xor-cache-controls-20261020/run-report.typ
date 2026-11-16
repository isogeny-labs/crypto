#set page(paper:"a4",margin:14mm)
#set text(font:"New Computer Modern",size:9pt)
= N83 XOR cache: launch recovery and controls
_2026-10-20 · xor-cache-v2 · constructed correctness gate_

All four actual controls—K2048/D12 with cache off/on—passed, each with a separate native general-field replay and rank one. Off/on returned identical complete SAT-model and relation-row digests within each base. The corrected card started at 01:16:34 UTC and entered public search after all controls passed.

== Retained launch history
#table(columns:(1fr,2.7fr),inset:4pt,
 [*Start UTC*],[*Observed state*],
 [01:08:07],[Pinned image absent; exit 1 before worker execution. No natural query.],
 [01:10:50],[K2048 off control/replay passed; a global shell label caused a doubled replay path. Exit 2 before natural queries.],
 [01:16:34],[Exact image restored; driver function isolated in a subshell; four controls and four replays passed. Public card running at this boundary.])

Both failed launch trees remain archived. The actual shell card passed two native fixture tests covering all 24 calls in order, a retained wall cap, remaining-cell execution and refusal to reuse outputs. Fixtures test driver sequencing; actual group correctness is checked independently.

#align(center,image("controls.svg",width:90%))

One observation per cell, all with zero decisions/conflicts and pinned x blocks/selectors/phases. Cache-on skipped 3,735 satisfied-row visits in each control. This records activation; natural-query effectiveness remains pending. Different bases use different constructed targets. These shared-host L0 observations admit no speedup or base ranking.

#pagebreak()
== Identity, limits and validation

Primary curve `icv1-f2m83-tm6151469093347-debefd74`, a=0, subgroup 2417851639230796216685689, cofactor 4, modulus z^83+z^45+z^2+z+1. K2048: 2,048 columns / 339,968 points; D12: 2,001 / 332,166. Native worker revision `1de47a9d4186bff08abc84c224a111183e820106`, identical binary across settings. Independent general-field replay checks subgroup/phase/sign/coefficients and rejects tampering.

SSH Spot host, Intel Xeon 6975P-C, CPU 31, one CPU quota, hard 4 GiB, zero swap, no worker network and read-only inputs/root. Worker intervals include import, model/root/search, group checking and rank. Construction, Docker launch, independent replay and failed-launch overhead are outside the worker interval and separately retained. The elapsed field ends before final receipt serialization; container times preserve the process boundary. Other host workloads and evidence copying make these L0 observations.

Linux release library: 4,403 passed, zero failed, 118 pre-existing ignored. Worker 3, receipt audit 2, replayer 2 and native card fixture 2 release tests passed. Cache invalidation and 400 mixed CNF/XOR truth-table cases passed. The cache stays off by default.

== Sources and open gates

Study `research/n83_prebuild_cost_m23_20261019/`: `XOR_CACHE_PROTOCOL.md`, host card, both continuation notes, `XOR_CACHE_VALIDATION.md`, and this directory's `RUN_REPORT.md`. Control archive `xor-cache-v2/controls/controls.tar.xz`: SHA-256 `fcaa18926f019f0fcc9fdd739827e58beaa8ce675bac355aa276656468c24f84`, 104 checked files. Failure archives: 19 and 39 checked files under `xor-cache-v1/`. Plot reads the four unmodified worker receipts in `data/`.

The sixteen-search public comparison uses targets 0 and 1, both bases, A1/B1/B2/A2, 10,000 conflicts and 1,790/1,800 s caps. Returned relations require independent replay. Natural rank, complete cold/reused IC cost, matched rho and best-base selection remain open. Candidate and best-runtime fields stay null; canonical leaderboard/boundary rows are unchanged.
