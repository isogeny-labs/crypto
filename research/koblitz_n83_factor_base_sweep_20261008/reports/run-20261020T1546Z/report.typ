#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:9.5pt)
= Native N83 sweep through 1M conflicts
Campaign2026-10-20UTC; completed continuation2026-10-21UTC. K2048-v20 and
K4096-v20 each reach1000000conflicts, models0, relations0, rank0. Native replay,
memory profile, exact relay and cleanup pass. Bounded unknowns; not UNSAT,
scalar recovery or base selection. n=1 per base; no uncertainty estimate.

#table(columns:(1fr,1.4fr,1.4fr),inset:5pt,
 [Arm],[Native solver wall (ms)],[Supervised outer wall (ms)],
 [K2048-v20],[3535009.185143],[3540895.977546],
 [K4096-v20],[3627836.176575],[3638645.773661])
#image("completed-clocks.svg",width:100%)

Rust summarize-xor calculates solver_return minus solver_enter. Commands:
3552.809621051s and3648.855038327s, both exit0/noOOM/no wall censor. Outer
includes startup policy setup. Clocks describe recorded conditions; no timing
ranking or end-to-end gain. Missing worker context-switch/fault counters and
fixed kernel threads prevent automatic L2. No task build/test/compression ran
inside either timed arm. Full core14/30 reserved, workerCPU14,1CPUquota,
8GiB/no extra swap, strict quiet/thread/affinity restoration, networknone,
read-only root/source/inputs/worker, capdropALL, no-new-privileges,64PIDs.

== Evidence map
Study: research/koblitz_n83_factor_base_sweep_20261008.
Proof: verification/native-natural-1m-20261020; protocols PROTOCOL.md,
READINESS-V16.md, RESTORE-V18.md, LOCK-V19.md, SCOPE-V20.md.
Completed native summaries/audits/assessments: completed-v20/.
This report's data/ contains the native solver and memory JSON; render.py
only lays out those records. REPORT.md has the complete requirement table,
source identities, per-attempt costs, replay instructions and linked custody.

#pagebreak()
= Memory profiles and corrected startup policy
#image("memory-profiles.svg",width:100%)
K2048-v20:59samples; maximum sampled charge and sampled kernel peak5304934400bytes.
K4096-v20:61samples; both5876035584bytes. Every sample memory.max8589934592,
swap.max0; counters/custody validate. Last-sample-to-outer-end gaps:
57001.402148ms and34479.901983ms. Sampled peaks are lower bounds on full-run
peaks;60-second sampling cannot exclude between-sample changes. Cgroup charges
include the container, not just SAT allocations. Init RSS belongs to a startup/
timeout supervisor. No worker-allocation attribution or minimum-memory claim.

Owned idle controls reproduce kernel0/systemdMemorySwapMax=infinity changing
to kernel"max" on resource reapply. Synchronizing the owned unit to0 restores
kernel0 and preserves0 on reapply:0,max,0,0. This reproduces a mechanism, not
the historical caller. A narrow filtered audit did not identify that caller.

Guard-v20 holds the unchanged worker at a shell barrier. Native code verifies
the owned Docker/PID/cgroup, writes only its runtime MemorySwapMax0, checks
property/kernel/drop-in,8GiB/quota1CPU/cpuset14, then writes a digest-bound
RESOURCE_READY. Both startup records are hashed and checked by native replay.
No correction is permitted after release: sampled drift stops the run. Both
new runs verify owned container/runtime-file cleanup. Peer units, daemon
policy and kernel settings were untouched.

The benchmark-lock fix opens an existing file without O_CREAT and creates
race-safely only if absent. Root foreign-owner/sticky-directory and exclusion
regressions pass. Same inode1088 retained; no lock mode/owner/sysctl edits by
this task. A peer changed its owner before timing; both new native arms
successfully acquired/released that same file.

#pagebreak()
= All actual attempts remain evidence
#image("attempt-custody.svg",width:100%)
K2048-v15: preflight refusal, no worker,39.349220755commandseconds,exit1;
last CPU PSIavg10=34.59 exceeds5. K4096-v16: terminal1M/zero, historical replay
PASS,3684.731943389s,exit0; profile rejected because swap becomes"max" from
120568.421691ms. Historical PASS cannot override the failed resource envelope.

K2048-v16: old-host shutdown interrupts execution; no completed receipt,
terminal, final inspection or verified cleanup. Last observed53000conflicts
is nonterminal; five retained memory samples form a prefix only. Missing
cost/custody stays missing. New-host absence does not prove old cleanup.

K2048-v18: existing-lock open refusal, no worker,15.120572840s,exit1.
K2048-v19: sampled swap drift stops the run;28samples, last at1622199.424337ms
records"max". Last SAT snapshot420000conflicts nonterminal; command
1643.653593095s,exit1; failure inspection137/noOOM,cleanupverified. Terminal
models/relations/rank remain null. Native trace prefix validates; profile
rejects. Full supplemental Docker stream equals final failure logs.

K4096-v18/v19 were never compiled or executed; no actual charge or observation.
K4096-v20 dispatch transcription typo was withdrawn before any command; actual
sole command matches frozen prompt. All seven actual attempts retain their
original cards/ELFs/sources/inputs/raw outputs/available receipts and assessments.
Conservative registered envelope42000worker-seconds, including full6000-second
allocations for refusals. It is not a complete historical cold/campaign ledger.

#pagebreak()
= Workload, validation and publication
Primary a=0 icv1-f2m83-tm6151469093347-debefd74; field x^83+x^45+x^2+x+1;
subgroup2417851639230796216685689. Retained public_x_hash seed2026100801,
m5,identitymask0,K2048/K4096panels/fixtures. Distinct positive-trace a=1
icv1-f2m83-t6151469093347-9eba6cd5 holdout; no EC1 equivalence promotion.
Each new arm1Mconflicts/6000worker-seconds/6600command-seconds; exact isolated
K2048->assessment->K4096->assessment at clean source freeze:
#text(size:7pt)[66ab009b1d91792c46d078e15adfeda45eacf29f].
RunnerSHA256:
#text(size:7pt)[ecc418f20f276b44cf8ec61019cee09c36228d1c78e5c62f4512afb665451f67].
WorkerSHA256:
#text(size:7pt)[3a31bd2bedee5519d30b517f39ec27f72561ce898213657688c55b51bbcdf7b6].
Worker unchanged; nine watched sources match660f0f4b10. Replacement host
ip-172-31-26-135.us-east-2.compute.internal, Xeon6975P-C,32logicalCPU;
old ip-172-31-36-154.us-east-2.compute.internal interrupted. No timing pooling.

Fresh release library4409pass/118ignored; exporter19pass; model-checker2pass.
Guard30unit+7card pass; root lock regression and final native startup/barrier/
owned-scope-reapply/cleanup smoke pass. Both native audits and profile checks
pass, all output digests verify, and both actual mechanical relays are
byte-identical to receipts. Supplemental live streams equal final logs.

== Storage and reproducibility
S3 access restored on the replacement host. Original800K raw/sidecar and
initial1M checkpoint have fresh verified downloads. Pre-worker v20 checkpoint
54769744bytes,SHA256:
#text(size:7pt)[dc52c9c6bd7e18b16fe0c0365efb79b21e9b06c008aa688eb95fcc72866b3026].
Fresh GET,payload/historical replay/negative-prefix checks/repack pass.
Final raw name: store:evidence/n83/natural-1m-20261020/runs.tar.xz.
Actual digest, native store index, PUT/HEAD/GET, payload, native replay and
byte-identical repack receipts are external to that archive:
verification/native-natural-1m-20261020/PUBLICATION.md. The proof sidecar
is published separately and fresh downloaded bytes checked. Report storage
status is established by those receipts, not self-referential archive content.

Graphs have native JSON provenance, SVG and same-source150dpiPNG; benchmark
copies include all graphs/PDF. PDF visually inspected. Canonical leaderboard,
scoreboard and timeline checked unchanged: no eligible new cold/rho/identity/
component speed promotion. Existing CMS5.17.0 known-witness results remain
faster than native; no fastest-tool or matched-natural-solve claim. Full cold
accounting, useful natural yield, rank/recovery and allocation cause remain open.
Next: separately freeze a SAT-worker allocation profile and targeted change;
a higher cap alone would remain bounded search. PR2036 stays subject to its
broader CI/base reconciliation hold.
