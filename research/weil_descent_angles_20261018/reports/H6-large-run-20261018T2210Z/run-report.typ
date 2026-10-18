#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: H6-large-run-20261018T2210Z

Descriptive record of one recorded run of examples/n2\_routes.rs, mode run at 44 and 48 bits (Addendum H6), on a RunPod CPU pod under
`research/weil_descent_angles_20261018/PROTOCOL.md` (Addendum A and
Amendments B–G and Addendum H, each committed before the run it governs). It lists what
ran, on what host, and what the tool recorded, file by file. Interpretation
is in the study's `REPORT.md`. Costs are counted operations
(F#sub[p] multiplications and inversions, group operations, native
operations), never wall-clock; the only times are the tool's own seconds.
Isolation: none earned (a shared cloud container, or a RunPod CPU pod whose lscpu is in host.txt); this
affects only the seconds, since every recorded value is an exact count or
a counted operation total.

== env.txt
```
commit 6083c58aa45e04e4d3194689b3b3e6ef5c7be115 (fetched by the pod from raw.githubusercontent.com, sha256-checked)
cmd: rustc --edition 2021 -O examples/n2_routes.rs -o n2 && N2_THREADS=16 N2_SIZES=44,48 N2_CS=2 ./n2 "$OUT" run
host: RunPod CPU pod vv1r308vkkoewx (h6-large-sizes), 16 vCPU, flavors cpu5c,cpu3c, image rust:1-bookworm, launched 2026-10-18T22:09:01Z
cpu: AMD EPYC 4564P 16-Core Processor
rustc 1.99.1 (58cf8f9c3 2026-10-13)
runner: research/weil_descent_angles_20261018/runpod/podrun.py
```
== tool\_log.txt
```
jobs 6 total 514.8s
```
== host.txt
```
Architecture:                         x86_64
CPU op-mode(s):                       32-bit, 64-bit
Address sizes:                        48 bits physical, 48 bits virtual
Byte Order:                           Little Endian
CPU(s):                               32
On-line CPU(s) list:                  0-31
Vendor ID:                            AuthenticAMD
Model name:                           AMD EPYC 4564P 16-Core Processor
CPU family:                           25
Model:                                97
Thread(s) per core:                   2
Core(s) per socket:                   16
Socket(s):                            1
Stepping:                             2
CPU(s) scaling MHz:                   33%
CPU max MHz:                          5881.0000
CPU min MHz:                          545.0000
BogoMIPS:                             8983.84
Flags:                                fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good amd_lbr_v2 nopl nonstop_tsc cpuid extd_apicid aperfmperf rapl pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate ssbd mba perfmon_v2 ibrs ibpb stibp ibrs_enhanced vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid cqm rdt_a avx512f avx512dq rdseed adx smap avx512ifma clflushopt clwb avx512cd sha_ni avx512bw avx512vl xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local user_shstk avx512_bf16 clzero irperf xsaveerptr rdpru wbnoinvd cppc arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif x2avic v_spec_ctrl vnmi avx512vbmi umip pku ospke avx512_vbmi2 gfni vaes vpclmulqdq avx512_vnni avx512_bitalg avx512_vpopcntdq rdpid overflow_recov succor smca fsrm flush_l1d
Virtualization:                       AMD-V
L1d cache:                            512 KiB (16 instances)
L1i cache:                            512 KiB (16 instances)
L2 cache:                             16 MiB (16 instances)
L3 cache:                             64 MiB (2 instances)
NUMA node(s):                         1
NUMA node0 CPU(s):                    0-31
Vulnerability Gather data sampling:   Not affected
Vulnerability Itlb multihit:          Not affected
Vulnerability L1tf:                   Not affected
Vulnerability Mds:                    Not affected
Vulnerability Meltdown:               Not affected
Vulnerability Mmio stale data:        Not affected
Vulnerability Reg file data sampling: Not affected
Vulnerability Retbleed:               Not affected
Vulnerability Spec rstack overflow:   Mitigation; Safe RET
Vulnerability Spec store bypass:      Mitigation; Speculative Store Bypass disabled via prctl
Vulnerability Spectre v1:             Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:             Mitigation; Enhanced / Automatic IBRS; IBPB conditional; STIBP always-on; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
Vulnerability Srbds:                  Not affected
Vulnerability Tsx async abort:        Not affected
               total        used        free      shared  buff/cache   available
Mem:             124          14          69           0          42         110
Swap:              0           0           0
16
Linux 526a2cc5ccb1 6.8.0-51-generic #52~22.04.1-Ubuntu SMP PREEMPT_DYNAMIC Mon Dec  9 15:00:52 UTC 2 x86_64 GNU/Linux
rustc 1.99.1 (58cf8f9c3 2026-10-13)
```
== rusage.txt
```
Command being timed: "bash -c mkdir -p examples && curl -sSfL https://raw.githubusercontent.com/aburan28/crypto/6083c58aa45e04e4d3194689b3b3e6ef5c7be115/examples/n2_routes.rs -o examples/n2_routes.rs && echo 'fb19b56a3ed276e163c2cbc84f496014983e6f2b4376e2ea090a8423009afc7a  examples/n2_routes.rs' | sha256sum -c - && rustc --edition 2021 -O examples/n2_routes.rs -o n2 && N2_THREADS=16 N2_SIZES=44,48 N2_CS=2 ./n2 "$OUT" run"
	User time (seconds): 1610.12
	System time (seconds): 22.75
	Percent of CPU this job got: 315%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 8:37.15
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 3342800
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 44
	Minor (reclaiming a frame) page faults: 15953532
	Voluntary context switches: 1251
	Involuntary context switches: 4870
	Swaps: 0
	File system inputs: 0
	File system outputs: 12456
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
```
== summary.txt
```
O/J bits 44 arm J c 2: IC/rho ratio in [0.988, 2.961] over 12 cells -> unresolved; steps/p 0.295 (rho 0.328)
O/J bits 44 arm D c 2: IC/rho ratio in [28.403, 80.874] over 6 cells -> rho cheaper in every cell; steps/p 0.342 (rho 0.328)
O/J bits 48 arm J c 2: IC/rho ratio in [0.768, 2.316] over 12 cells -> unresolved; steps/p 0.311 (rho 0.448)
O/J bits 48 arm D c 2: IC/rho ratio in [28.134, 86.838] over 6 cells -> rho cheaper in every cell; steps/p 0.433 (rho 0.448)
O/J scaling arm R c 0: cost ~ p^1.213
O/J scaling arm J c 2: cost ~ p^1.038
O/J scaling arm D c 2: cost ~ p^1.263
K6 bits 44: rho steps/p mean 0.3284 (s.e. 0.0262, 30 runs) vs 0.886*sqrt(ell)/p mean 0.3132
K6 bits 48: rho steps/p mean 0.4478 (s.e. 0.0418, 30 runs) vs 0.886*sqrt(ell)/p mean 0.4430
O/J all answers verified: true
```
== ranking.csv (first 4 of 4 rows)
#table(columns: 9, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*arm*], [*c*], [*cells*], [*ic\_cheaper\_cells*], [*rho\_cheaper\_cells*], [*min\_ratio*], [*max\_ratio*], [*verdict*],
[44], [J], [2], [12], [2], [10], [0.9879], [2.9613], [unresolved],
[44], [D], [2], [6], [0], [6], [28.4034], [80.8740], [rho cheaper in every cell],
[48], [J], [2], [12], [5], [7], [0.7681], [2.3163], [unresolved],
[48], [D], [2], [6], [0], [6], [28.1337], [86.8385], [rho cheaper in every cell]
)
== routes.csv (first 12 of 180 rows)
#table(columns: 26, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*instance*], [*p*], [*N*], [*h*], [*ell*], [*target*], [*arm*], [*c*], [*factor\_base*], [*steps*], [*relations*], [*b\_relations*], [*jac\_ops*], [*walk\_M*], [*walk\_I*], [*walk\_native*], [*jac\_M*], [*jac\_I*], [*la\_M*], [*la\_I*], [*la\_native*], [*eadd\_M*], [*eadd\_I*], [*verified*], [*seconds*],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [0], [R], [0], [0], [2070002], [0], [0], [0], [25218491], [2089538], [4111831], [0], [0], [0], [0], [0], [12], [1], [1], [0.267],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [0], [J], [2], [4162], [1717014], [858442], [12], [1771393], [79756715], [1770564], [5159128], [72838803], [1770513], [0], [0], [6869824], [12], [1], [1], [0.925],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [0], [D], [2], [4162], [1682105], [662172], [10], [0], [1876790343], [16714828], [8535144], [0], [0], [0], [0], [4742902], [12], [1], [1], [7.456],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [1], [R], [0], [0], [476301], [0], [0], [0], [5896036], [486858], [945786], [0], [0], [0], [0], [0], [12], [1], [1], [0.063],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [1], [J], [2], [4162], [1914199], [959517], [25], [1974158], [88892022], [1973241], [5755835], [81174187], [1973181], [0], [0], [8180299], [12], [1], [1], [1.073],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [1], [D], [2], [4162], [1454316], [571581], [5], [0], [1623598570], [14451772], [7375552], [0], [0], [0], [0], [3894459], [12], [1], [1], [6.497],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [2], [R], [0], [0], [704078], [0], [0], [0], [8655340], [715770], [1398488], [0], [0], [0], [0], [0], [12], [1], [1], [0.094],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [2], [J], [2], [4162], [1600517], [800253], [6], [1651352], [74356015], [1650590], [4809754], [67903816], [1650535], [0], [0], [6190481], [12], [1], [1], [0.832],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [2], [D], [2], [4162], [401367], [157967], [0], [0], [448230106], [3995327], [2047208], [0], [0], [0], [0], [860130], [12], [1], [1], [1.782],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [3], [R], [0], [0], [2214124], [0], [0], [0], [26967547], [2234461], [4397464], [0], [0], [0], [0], [0], [12], [1], [1], [0.293],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [3], [J], [2], [4162], [678118], [338643], [1], [702016], [31637978], [701690], [2041865], [28876297], [701638], [0], [0], [2033360], [12], [1], [1], [0.347],
[44], [0], [4330457], [18752853744904], [8], [2344106718113], [3], [D], [2], [4162], [1418200], [557040], [0], [0], [1582554912], [14090291], [7186926], [0], [0], [0], [0], [3769093], [12], [1], [1], [6.276]
)
== routes\_summary.csv (first 12 of 36 rows)
#table(columns: 14, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*arm*], [*c*], [*ipr*], [*jac\_pricing*], [*native\_w*], [*runs*], [*verified*], [*mean\_p*], [*mean\_cost*], [*mean\_rho\_cost*], [*ratio\_to\_rho*], [*mean\_steps\_over\_p*], [*rho\_steps\_over\_p*],
[44], [J], [2], [3M], [measured], [0], [30], [1], [4423334], [64895366.0], [22031611.5], [2.9456], [0.2952], [0.3284],
[44], [J], [2], [3M], [measured], [0.1], [30], [1], [4423334], [65780251.7], [22318725.2], [2.9473], [0.2952], [0.3284],
[44], [J], [2], [3M], [measured], [1], [30], [1], [4423334], [73744223.3], [24902748.1], [2.9613], [0.2952], [0.3284],
[44], [J], [2], [3M], [eadd], [0], [30], [1], [4423334], [25551720.3], [22031611.5], [1.1598], [0.2952], [0.3284],
[44], [J], [2], [3M], [eadd], [0.1], [30], [1], [4423334], [26436606.0], [22318725.2], [1.1845], [0.2952], [0.3284],
[44], [J], [2], [3M], [eadd], [1], [30], [1], [4423334], [34400577.6], [24902748.1], [1.3814], [0.2952], [0.3284],
[44], [J], [2], [2log2p\_M], [measured], [0], [30], [1], [4423334], [120482153.7], [82160142.4], [1.4664], [0.2952], [0.3284],
[44], [J], [2], [2log2p\_M], [measured], [0.1], [30], [1], [4423334], [121367039.5], [82447256.1], [1.4721], [0.2952], [0.3284],
[44], [J], [2], [2log2p\_M], [measured], [1], [30], [1], [4423334], [129331011.0], [85031279.0], [1.5210], [0.2952], [0.3284],
[44], [J], [2], [2log2p\_M], [eadd], [0], [30], [1], [4423334], [81166587.3], [82160142.4], [0.9879], [0.2952], [0.3284],
[44], [J], [2], [2log2p\_M], [eadd], [0.1], [30], [1], [4423334], [82051473.0], [82447256.1], [0.9952], [0.2952], [0.3284],
[44], [J], [2], [2log2p\_M], [eadd], [1], [30], [1], [4423334], [90015444.6], [85031279.0], [1.0586], [0.2952], [0.3284]
)

== Graph
#figure(image("../../figures/h1_cofactor_law.svg", width: 100%), caption: [H6 values are in this report's summary; graph shown: H1 of the same round.])
