#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9pt)
= Native N83 indexed XOR propagation and factor-base continuation
Public synthetic a=0 curve: icv1-f2m83-tm6151469093347-debefd74. Field polynomial x^83+x^45+x^2+x+1; subgroup order 2417851639230796216685689. Source and records: REPORT.md and figure-data.json beside this PDF.

The indexed implementation retains exact sorted nonzero-word supports of dense XOR rows. It preserves pivot, row and reason order. Seven native dense and seven indexed observations independently verify the actual selectors and original group sum, with identical 787 conflicts, 35,307 decisions and five restarts. Median solve walls: dense 182074.425421 ms; indexed 17767.144183 ms. Five matched ratios are 12.1636332262, 9.99446883302, 9.95866828830, 10.1695267031 and 10.1932494096. These are descriptive component observations under recorded conditions, not admitted full-cold IC timings.

#image("solver-times.svg",width:100%)

Dense walls range 177459.286074–216393.804510 ms; indexed 17731.849428–17862.255509 ms. CryptoMiniSat 5.17.0 is faster on the same exported original CNF plus native XOR constraints, using one thread on this host. Seven independently verified models have reported solve/simplify CPU times 2810–2840 ms. Candidate wall / CMS algorithm CPU ratios in five rounds: 6.28629953357, 6.27411646961, 6.34929032028, 6.27814282085 and 6.28952658768. Wall and CPU timers differ; process parsing/startup is recorded separately. No fastest-implementation claim.

Host: managed Linux x86-64, 32 vCPU, 128 GiB; pinned CPU15, sibling31 reserved. Root native guard holds an exclusive benchmark lock, evicts movable user threads and requires quiet readiness; Docker immutable image541096ae9b05…, 1CPU quota, 4GiB RAM, no extra swap, network none, read-only root/source/inputs/worker, capabilities dropped, no-new-privileges, 64 PIDs. Fixed kernel threads remain. Missing worker context-switch/fault accounting and A/A drift prevent an automatic L2 claim. No builds/tests ran during timed arms.

#pagebreak()
= Deterministic work and bounded natural trials
#image("word-updates.svg",width:100%)

Every native arm performs 3,493,683 row operations. Dense updates are derived natively as row operations ×1589 words per row; indexed updates are counted:14,064,862. The ratio is394.704355222. Indexed support visits:15,997,241. Search and selected summands remain identical. The initial original-row census,108026 nonzero of7913220 allocated words, is not a bound on later fill-in.

#image("natural-trials.svg",width:100%)

Natural trials use the same retained single public query, seed2026100801, m5, identity mask0. K2048 and K4096 each completed the separately frozen 1000-conflict trials with zero models, relations and rank. Version5 completed10000 conflicts per K under a separate300-second limit, again with zero models, relations and rank. Native solver intervals were42413.639201 ms and50433.062636 ms respectively; both query and custody audits passed. A bounded unknown is not UNSAT or a proof of absent decompositions. Independent native query, original field/group and custody replay remains required. Positive natural relation/rank/scalar replay and useful natural yield remain open; no factor base winner, full cold cost or IC/rho ratio is established.

#pagebreak()
= Preserved failures, validation and next obligation
#image("diagnostics.svg",width:100%)

The original5000-conflict/180-second profiling arm was wall censored; its final counters remain null. The separately frozen one-conflict profile recorded971224218 ns in XOR propagation and977.691845 ms solver wall. Original external maxconfl5000 returned INDETERMINATE at5001 reported conflicts; its original failed EOF audit is preserved. Corrected external1M/360s v3 refused execution before any solver because reserved sibling busy time exceeded the unchanged threshold. Version4 adds bounded quiet readiness waiting before the worker, with all samples preserved and no timed solver retry. Unattempted superseded arms remain unattempted.

Validation: full serial release library4398 passed,118 ignored,0 failed after a preserved managed-runtime interruption and fresh restart. The library blob did not change afterward. Differential indexed/dense cases cover widths1/63/64/65/127/130,12 planted systems each, blocking and growth; exact models/search counters/supports agree. Export example19 tests, external model checker2, native runner and card execution tests pass; raw logs give precise invocations. Python only renders recorded native values and inspects the PDF.

Director compiled v4 cards; an account usage limit prevented its later assessments. Primary ran the identical read-only native card checks at the frozen HEAD and retained assessment-primary files. Mechanical execution remained exact, with direct stdout relay, frozen hashes and write-once outputs. This staffing deviation is disclosed; no unavailable director assessment is claimed.

Next performance target: reduce repeated scans/re-pivots of already decided rows, with invalidation on backjump, differential model/search checks, and a fresh matched external comparison. This is a proposed experiment, not an implemented optimization. Natural positive verification and higher-yield search remain independent obligations.

Canonical leaderboard HTML/JSON/Markdown gains this engineering component outside cold tables A–C. No cold pipeline or matched-rho measurement changed, so the index-calculus scoreboard and progress timeline stay unchanged. Exact immutable protocols, cards, complete raw stdout/stderr, formula/models, worker/driver binaries, sources, tests and S3 transport/hash receipts are linked from REPORT.md. Historical censored runs are not timing denominators.

#pagebreak()
= Every planned or preserved observation
Solver column: native solve wall, CMS solve/simplify CPU, or natural solver phase difference, milliseconds. Missing values remain missing. Failed and unattempted observations are included. All definitions and source identities are in figure-data.json and REPORT.md.
#set text(size:8pt)
#include "runs.typ"
