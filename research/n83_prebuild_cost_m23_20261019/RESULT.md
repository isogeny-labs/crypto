# N83 prebuilt factor-base cost and bounded natural queries

**Evidence status, 2026-10-20:** D18 export and full-object replay pass; the paired D prebuilt/on-demand m=2 and m=3 finite-query panel is complete; the separate K2048/D12 m=2/m=3 panel completed with one preserved Spot interruption and a frozen retry. The [smaller-base m=4 gate](SMALL_M4_PROTOCOL.md) completed four conflict-capped natural searches with audited progress. The [full-size U/D panel](reports/run-full-m4-natural-20261020/RUN_REPORT.md) completed all eighteen scheduled attempts: thirteen conflict caps and five wall caps, with 329 files independently audited. The [XOR cache follow-up](XOR_CACHE_PROTOCOL.md) is a separate frozen component diagnostic. All timing here is shared-host L0. `candidate_id: null`; `selected_best_total_runtime: null`. No complete index-calculus or matched-rho runtime is established.

## Exact objects and boundaries

The primary curve is `icv1-f2m83-tm6151469093347-debefd74`, subgroup order `2417851639230796216685689`. The D18 native census enumerated 262,144 low-bit x values, found 32,643 full signed-Frobenius direct-subgroup classes, and selected the first 16,627 under the frozen rule. The selected CSV SHA-256 is `534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4`. The 81,844,623-byte D compressed object BLAKE3 is `e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682`; SHA-256 is `347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7`. Generic independent replay checked every representative, phase, sign, coefficient label, subgroup point, and duplicate rule for all 2,760,082 exported rows; see [the export result](../n83_d18_subgroup_orbit_census_20261018/export-v1/RESULT.md) and [the full-object same-host replay](runs/export-cost/export-cost-v3/generated/replay.json). The point-set BLAKE3 is `8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0`.

The retained U base has the same column and row counts but a different point set. Its first 36-arm native comparison to D used the same eight public points, with m=2 scanning every distinct first summand and m=3 making `2^20` deterministic pair draws per arm. All 36 arms completed, all fresh replays passed, and four constructed controls passed separate general-field replay. The finite panel returned zero natural relation rows and useful rank zero across 49,681,476 m=2 and 18,874,368 m=3 attempts. [Its run report](reports/run-20261019T0536Z/RUN_REPORT.md) preserves the precise policy and A/A controls. These counts apply only to those finite events and are not an estimate of unrestricted relation yield.

The m=4 paired U1–D1–D2–U2 SAT card on another Spot host ended after three `UNKNOWN_budget` arms and a Spot interruption before U2. Each completed arm preserved solver progress; none returned a natural relation receipt. [Its bounded result](../n83_d18_subgroup_orbit_census_20261018/natural-v1/NATURAL_RESULT.md) is an incomplete same-host comparison. It does not rank U and D.

## Matched D preparation modes, eight public points

The [frozen paired-mode protocol](PAIRED_MODE_PROTOCOL.md) and [host card](PAIRED_MODE_SPOT_ADDENDUM.md) ran prebuilt import and on-demand D generation under one native binary SHA-256 `80a53099ff4db99519de3d3700bbde3948dc0b435e6a3c7ac800164ac7450a24`, one exact D archive and one public corpus SHA-256 `2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5`. Both preparation paths produced the same point-set BLAKE3 above and labeled-row BLAKE3 `805d1cbc65cac0a74ebf49315b8c17b0d5229bdceba5725ba4349fb1cc6c9e9c`. Each worker used a fresh process, one pinned CPU, hard 4 GiB, zero swap, no network and read-only inputs. The m=2 policy examined 2,760,082 complement attempts per target; m=3 drew `2^20` pairs. Target 0 had two A/A repetitions per mode and m; the other seven targets had one. This is L0 because unrelated jobs shared the host.

| m | mode | public arms | complete attempts | mean worker wall, range (s) | mean base preparation / query (s) | largest worker peak RSS (MiB) | natural rows / rank |
| ---: | :--- | ---: | ---: | :--- | :--- | ---: | :--- |
| 2 | prebuilt | 9 | 24,840,738 | 9.848 [9.769, 10.212] | 3.600 / 5.985 | 620.9 | 0 / 0 |
| 2 | on-demand | 9 | 24,840,738 | 35.131 [35.033, 35.459] | 28.686 / 6.179 | 537.3 | 0 / 0 |
| 3 | prebuilt | 9 | 9,437,184 | 8.828 [8.659, 9.815] | 3.635 / 4.923 | 621.0 | 0 / 0 |
| 3 | on-demand | 9 | 9,437,184 | 34.601 [33.894, 39.585] | 29.242 / 5.088 | 537.4 | 0 / 0 |

All 76 Docker arms exited zero: two setup, two constructed controls, 36 public probes and 36 fresh replays. Every public probe says `COMPLETE` and every replay says `PASS`; there were no capped probes in this finite panel. The two constructed controls each had useful rank one and rejected changed targets/rows; they are excluded from natural counts. [The paired run report](reports/run-paired-20261019/RUN_REPORT.md), [sealed raw tree](runs/paired-mode-v1/), [its SHA-256 manifest](runs/paired-mode-v1.sha256), and transferred archive SHA-256 `236e64b40cc836ff91509f86006df886046d18df61e145d79e958d98b1ff24ed` bind the receipts. The mean worker-wall ratio on-demand/prebuilt is 3.567 at m=2 and 3.919 at m=3. These are repeated finite-query process costs, including preparation each time; they are not a full IC speedup. The card occupied 16 min 48 s from first setup start through last replay end, including container launches, controls and replays; the process means do not substitute for that observed card interval.

## Charged construction and reuse

The same SSH host independently reran the unchanged D18 census binary SHA-256 `2ad7e1fb0126081a247088ba1888b4656e60fb29379be9bc232d3605816c9653` under a 4 GiB, zero-swap, one-CPU systemd scope. It matched the 262,144 / 32,643 / 16,627 counts, selected CSV and selected-row BLAKE3. Its native process wall was 27.914 s (GNU outer wall 27.92 s, 6,100 KiB peak RSS); [raw census receipts](runs/census-cost-v1/) and [their manifest](runs/census-cost-v1.sha256) are retained. A same-host original-commit exporter produced the byte-identical D object in 39.373 s (97,528 KiB peak RSS), and a fresh generic full-object replay took 20.702 s (233,568 KiB peak RSS). The [export cost records](runs/export-cost/) preserve two earlier unsuccessful attempts: a cgroup-path wrapper error, then a complete export with a different provenance header and therefore different compressed bytes. Those attempts are not silently folded into the successful wall value.

For a derived eight-query budget with the same finite m policy, repeating fresh query processes, charge the one-time census + serialization/export + generic replay = 87.989 s before prebuilt queries. For m=2, one cold prebuilt query is 97.838 s versus 35.131 s on demand; eight prebuilt queries are 166.777 s versus 281.045 s on demand. For m=3, one cold prebuilt query is 96.818 s versus 34.601 s on demand; eight are 158.617 s versus 276.812 s. These sums use same-host L0 observations from different pinned native revisions for the one-time exporter and the paired worker, and repeated-arm means for the query term. They exclude remote upload, persistent storage, any capped or alternative-policy query, rank growth, linear algebra, individual logarithm and scalar verification. They are cost projections for the bounded finite-query event, not an admitted complete-cold crossover. The observed prebuilt worker peak RSS is about 84 MiB higher than on-demand despite lower preparation wall. [Figure data](reports/result-20261019/cost-data.json), [SVG](reports/result-20261019/cost.svg), [150 dpi PNG](reports/result-20261019/cost.png), and [results PDF](reports/result-20261019/RESULT.pdf) show the charged phase split.

## Reuse accounting addendum, 2026-10-20

The [native accounting addendum](reports/reuse-20261020/RUN_REPORT.md)
completes the frozen `q=1,8,64` calculation from all 833 pinned files
of the paired finite-query card. It also makes a previously omitted
interval explicit: fresh per-query receipt replay and the worker
container boundary outside its internal timer. The original
worker-interval projections above remain preserved.

Charging setup plus both worker/replay container intervals gives the
following **derived** budgets in seconds:

| m | q=1, prebuilt / on demand | q=8, prebuilt / on demand | q=64, prebuilt / on demand |
|---:|---:|---:|---:|
| 2 | 101.981 / 39.135 | 199.924 / 313.079 | 983.472 / 2,504.633 |
| 3 | 100.836 / 38.567 | 190.762 / 308.539 | 910.172 / 2,468.314 |

The 64-query panels were not executed. Docker launch/teardown outside
the containers, remote custody/storage, prior retry history and later
rank/linear-algebra/individual-logarithm phases remain excluded. This is
an accounting addition, with no algorithm change or qualified timing
promotion. The [source-linked PDF](reports/reuse-20261020/RUN_REPORT.pdf)
and [new graph](reports/reuse-20261020/reuse.svg) retain that boundary.

## Separate smaller-base finite policies

The [K2048/D12 protocol](SMALL_BASE_PROTOCOL.md) froze m=2 exhaustive first-point complement scans and m=3 `2^20` deterministic pairs on the same eight public targets under one native worker revision. K2048 has 2,048 columns and 339,968 points; D12 has 2,001 columns and 332,166 points. They are distinct retained objects with different point-set hashes and rank dimensions. The [run report](reports/run-small-20261019/RUN_REPORT.md) records nine complete public arms per base and m, 36 passing fresh replays, and five constructed controls that passed separate general-field replay. Every completed host exit was zero. The first D12 target-7 m=3 attempt was stopped by Spot capacity loss and has no worker or exit receipt; the same instance resumed, a new control passed, and a separately named retry and replay completed. Its original partial directory remains preserved.

| m | base | completed finite attempts over nine public arms | natural rows / useful rank | worker-wall mean, observed range (s) |
| ---: | :--- | ---: | :--- | :--- |
| 2 | K2048 | 3,059,712 | 0 / 0 | 1.283 [1.169, 1.593] |
| 2 | D12 | 2,989,494 | 0 / 0 | 1.258 [1.139, 1.621] |
| 3 | K2048 | 9,437,184 | 0 / 0 | 6.072 [5.156, 6.921] |
| 3 | D12 | 9,437,184 | 0 / 0 | 5.761 [4.954, 7.202] |

The [small-base raw tree](runs/small-base-v1/), [file manifest](runs/small-base-v1.sha256) and transferred package SHA-256 `d0de87f98451af8d2ebf4d84eaa98094fa74f86fd3ca20b575081aafc9d7d5f4` preserve the complete and interrupted attempts. [Figure data](reports/run-small-20261019/small-data.json), [SVG](reports/run-small-20261019/small.svg), [150 dpi PNG](reports/run-small-20261019/small.png), and [run PDF](reports/run-small-20261019/RUN_REPORT.pdf) show the bounded worker costs. These arms use different point sets, rank dimensions and m=2 attempt counts, and their L0 samples straddle a Spot stop. Their zero finite-event yields do not select a base or establish a cost improvement.

## Completed smaller-base m=4 public card

The [K1,D1,D2,K2 public card](reports/run-small-m4-natural-20261019/RUN_REPORT.md)
finished at 20:48:34 UTC on 2026-10-19. All four natural arms returned
`UNKNOWN_search_cap` at exactly 100,000 conflicts, normal exit and no OOM.
Worker walls were 1,702.777036 / 1,750.332048 / 1,676.022754 / 1,697.533479 s.
Each base's repeated search counters agree. The native
[receipt audit](small-m4-v1/final/audit.json) checks all 132 archived files,
container limits, controls, phases and progress, including final counters.
Natural yield/rank remain null. Recorded XOR elapsed accounts for
97.508–97.561% of worker wall; the timer measures elapsed intervals rather
than exclusive CPU time. These are L0 observations on one public point.
This baseline motivated caching fully assigned, satisfied XOR rows until
backjump or matrix rebuild. The separate correctness and fresh
cache-off/on measurements are recorded below.

## Open obligations

The opt-in XOR-row cache has passed directed backjump/matrix tests,
400 exhaustive mixed CNF/XOR comparisons, the full Linux release-library
suite (4,403 passed), and four actual N83 controls with independent replay
and identical off/on model/row digests. The
[control and launch-history report](reports/run-xor-cache-controls-20261020/RUN_REPORT.md)
retains a missing-image launch and a fixed shell-label failure. The
[corrected sixteen-attempt card](reports/run-xor-cache-natural-20261020/RUN_REPORT.md)
completed at **02:00:10 UTC on 2026-10-20**. All sixteen attempts returned
`UNKNOWN_search_cap` at 10,000 conflicts. Its native audit passes 294
files and all four ABBA cells: search work matches, while checked row
visits fall by **82.960–85.361%**, with exact checked-plus-skipped
conservation. Recorded worker off/on ratios are **1.321160–1.345360** on
the shared L0 host. These are component work/timing observations; natural
yield/rank remain censored and qualified timing remains open. The default
solver remains cache-off.

The [full-base common-public panel](full-m4-v1/STATUS.md) ran from
**02:06:17 to 06:48:53 UTC on 2026-10-20**. Four actual U/D
off/on controls, four independent replays and two native control-pair
checks passed for the original **16,627-class / 2,760,082-point bases**.
All eighteen natural attempts over all eight public points completed,
with U,D,D,U at target 0 and alternating base order thereafter.
The [329-file native audit](full-m4-v1/final/audit.json) passed:
D returned nine 10,000-conflict caps; U returned four conflict caps
and five 1,790-second wall caps. Worker/cap interval sums were
**2,636.872934 s for D** and **14,284.411736 s for U**. No accepted
natural row was returned. The retained empty matrices have rank zero;
natural yield and the complete useful-rank panel remain null. Different
stopping points prevent a U/D speed ratio. The
[final report and visual](reports/run-full-m4-natural-20261020/RUN_REPORT.md)
retain every attempt, durable counters, resource bounds and missing
wall-cap final cgroup peaks. The later watchdog resource fix is unmeasured
by this original card.

A concurrent [counter-only reconciliation report](reports/run-20261020T0116Z/RUN_REPORT.md),
its [graph](reports/run-20261020T0116Z/work-counters.svg), and its
[receipt copy](runs/xor-cache-v2-final.tar.xz) are also preserved. Both
archive and manifest hashes exactly match the native-audited copy; no
measurements were selected or discarded to combine the summaries. That
report does not compare elapsed times. The separate audit report above
records descriptive L0 intervals without a qualified timing claim and
adds browsing copies to `benchmarks/`. Canonical qualified-cost charts
and boundary rows remain unchanged. The reconciliation report's original
renderer limitation is preserved alongside a later local rendering note.

Accounting clarification: the m=4 `process_wall_ms` field is elapsed time
inside the worker before final receipt serialization. Container start/end
receipts preserve the surrounding process interval. A complete cold-cost
comparison must also charge that final output and custody boundary.

The [smaller-base m=4 controls](reports/run-small-m4-controls-20261019/RUN_REPORT.md)
passed exact import, pinned SAT/group/rank checking and separate native
general-field replay on x86_64 and arm64. K2048 and D12 each produced one
checked constructed rank-one row. Their worker walls were 672.997 and
651.126 ms (one L0 observation each); their SAT models had 84,966 and
84,590 variables. All six SSH preflight/control/replay containers exited
zero without OOM. [Sealed control evidence](small-m4-v1/controls/README.md)
is retained separately from the public query card, which started at
2026-10-19 18:54:41 UTC and entered unpinned K1 search after both controls
and their replays passed. Its four natural-arm outcomes remain pending at
that control boundary; constructed rows are excluded from natural counts.

The K2048 and D12 m=4 resource-feasible policies are frozen in [SMALL_M4_PROTOCOL.md](SMALL_M4_PROTOCOL.md); execution and independent replay determine their status. The completed m=2/m=3 finite events supplied no natural rank, and the earlier U/D m=4 paired SAT card was censored/incomplete. Consequently useful-rank rate beyond these exact events, linear algebra, target verification, complete cold and genuinely amortized IC runtime, same-point matched rho, and any base selection remain open. The [problem record](../../problems/n83-prebuilt-base-quality-m2-m4.md) tracks the exit criteria. No canonical leaderboard row moves on this L0 bounded evidence.

## Original five gates after the full-size card

| Requested gate | Status | Evidence and remaining gap |
|:--|:--|:--|
| Export D18 and independently check every point/label | Verified complete | Selected 16,627 classes, all 2,760,082 exported rows generically replayed; exact identities above. |
| Make capped m=4 search observable | Verified for this card; later resource patch implemented/tested | All 18 attempts have audited phases/progress/final or cap receipts. The original five wall caps lack final cgroup peak; the later watchdog patch adds explicit resource/error fields for future cards. |
| Common public U/D comparison and smaller feasible arms | Bounded executions complete; useful-rank requirement partial | Full-size 18-attempt card and smaller four-attempt card retained; all capped. No accepted natural rows, no usable natural-yield estimate or full rank panel. |
| Prebuilt versus on-demand complete cold/reuse cost | Partial | Same D set and native revision in 36 paired finite probes. Setup, worker, replay/container observations and derived q=1,8,64 budgets retained. Full IC phases, custody, all failed setup history, qualified timing, exact IC incumbent and same-point rho remain open. |
| Separate m=2/m=3 frozen policies | Verified finite policies; broader natural-rank requirement partial | Full-size and K2048/D12 finite public panels and replays complete under their own caps. Zero rows apply to those finite events. A broader or partial-relation policy remains a new frozen experiment. |

The next algorithmic obligation is independently checked natural rank
growth under a separately frozen oracle policy. Current primary-curve
stage baselines are named by their exact cards above; no complete verified
primary-curve IC runtime is available to promote as an incumbent. Results
from the different a=1 curve do not fill that gap. The
[catalog reconciliation](../cover_transfer_assessment_20261013/catalog-refresh-20261020.md)
and [validation logs](merge-release-lib-20261020-2.log) repair the earlier
local PR gates; current-head GitHub checks remain a separate merge gate.
