#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Native N83 SAT boundary continuation
Boundary source 5e9ee5d4 · export source cbe8e44da · public primary a=0 N83 · L0 observations

The original fixed-coordinate/free-selector witness now passes independent native model/domain/group replay: 787 conflicts, 35,307 decisions, 167.341433194 s worker wall under its unchanged 5,000-conflict/180-second cap. The separate post-solve exporter reads actual selector bits and adds no assumptions. This resolves this exact known-witness control, while natural yield, useful rank and full-cold base selection remain open.

Three observations: pinned selectors SAT 0 conflicts; original free SAT 787 conflicts but frozen audit stops because selectors were not exported; separate free export SAT 787 conflicts and independent audit PASS. Original failures remain unchanged. Each observation is n=1, unreserved shared host, no confidence interval or speed comparison.

#image("phases.svg", width: 100%)

Native checkpoints are elapsed milliseconds from each solve entry, not exclusive phase durations. Initial propagation and CDCL entry are separate. Counters/timers are cumulative per solver instance; a killed prefix supplies no terminal count. No speedup is inferred from the earlier 180-second censored run. Full source IDs, specs and every recorded value are linked in REPORT.md and figure-data.json.

#pagebreak()
= Control outcomes and verification
#image("outer-wall.svg", width: 100%)

Outer walls: pinned 1001.912152 ms; original free 168213.674931 ms; exported free 167835.628938 ms. Worker walls and solve-entry clocks are separate intervals. Added allocation360+180=540 s; observed outer walls sum 337051.216021 ms. Compilation, tests, audit, storage and rendering are separate overhead; prior ledgers are not combined into a global spent total.

Verified export: initial propagation complete 192.850022 ms, CDCL entry 193.136020 ms, SAT return 167259.951266 ms. Existing timers record 167102070169 ns XOR propagation,70147211 ns clause propagation,16224291 ns analysis and 3634380 ns minimization, with 36757 XOR passes. This localizes this control's cost to XOR work during search. These counters are not a complete IC phase-cost model.

The free controls use identical 101636 variables,714736 clauses,4980 XORs and eight fixed x blocks, exact 1182/866 domains and original first-pair order. The fully pinned control has different selector assumptions. All use the fixed public fixture fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538, polynomial x^83+x^45+x^2+x+1 and subgroup order 2417851639230796216685689. No unknown scalar is recovered.

Docker: immutable image 541096ae9b05…, one CPU,4 GiB RAM,zero swap,no network,64 PIDs,task UID,read-only root/source/input/worker,capabilities dropped,no-new-privileges. All cleanup verified, no OOM. Independent native arithmetic replays actual representative indices, seven-bit phases, domain x values and original five-point group sum. A coherently rehashed forged phase is rejected.

Preserved failures: original unsupported-positive audit stop; pinned relay check changes a float spelling on reserialization. The saved relay equals its original receipt byte-for-byte. A native byte-preserving correction and regression test pass, with original failed check and impeded assessment retained. No experiment was rerun for that transport defect. Director assessments verify exact relays and output hashes before HEAD changes.

Validation: full permitted serial release library 4397 pass/118 ignored/0 fail, exporter 19 and pinned-selector 1. The final library sources are byte-identical; new decoder1 and native runner 15 tests pass. Original full validation logs, raw workers, source delta, cards, relays and failures are preserved. Python only plots/renders/inspects this report. Next proposed work is a frozen native XOR-engine profile/candidate comparison and complete natural positive-relation replay; neither is running or scheduled here.

#image("execution.svg", width: 100%)

#pagebreak()
= Publication and separate peer paired receipt
The explicitly approved 49,012,484-byte archive is in setec-astronomy-artifacts/store. Fresh GET matches SHA256 bef8a03c355b2238282638e326dd571bd91dd6873f5ed1df5fe12db2f0dcd5c3 and length; all 308 payload checks and manifest 30b56a3960e48b164dc354e779d32a5c6fc251a2eafb938d17af380bf2084004 pass after fresh extraction. S3 reports AES256. The earlier rejected plan remains immutable and is superseded by a publication addendum. This archive precedes the new boundary/export controls; their compact successor archive is recorded separately.

Separate upstream observations, not this harness's recertification: job 20261019-023945-n83-k2048-k4096-abba-5fcee8, c7i.xlarge, source 2b6bbb94998e562dd9bc0b623e5a9c803892cf71. All four natural-query arms report UNKNOWN_solver_cap at 1000 conflicts with 0 models/relations/rank. Native Rust worker; original Bash supervisor and Python audits/tests remain their provenance. No research Python replay produced this report.

#image("peer-paired.svg", width: 100%)

The peer RESULT reports mean container walls 61.103 s forK2048 and 132.084 s forK4096, ratio 2.162 for this one query/cap. This is not total IC cost or a relation-yield ranking. It differs from our witness controls and earlier local natural arms in source/host/workload; do not pool timings. Its original raw S3 location remains separate and was not migrated by this publication.

Sources: REPORT.md links native-selector-export-20261019 records/protocols/validation/forgery controls, native-phase-publication-20261019 HEAD/GET/checksums, and the original native-paired-k2048-k4096-20261019 RESULT/audit. No useful rank, full cold winner, matched-rho ratio, fastest-tool win, asymptotic advantage or nonexistence is established. External comparison was not run: these are bounded observability controls, not an admitted performance ranking. Full-cold leaderboard, scoreboard and progress timeline remain unchanged.
