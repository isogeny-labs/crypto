#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: H2-glue-run-20261018T2240Z

Descriptive record of one recorded run of examples/n2\_routes.rs, modes gluecheck and glue (Addendum H2/H3), on a RunPod CPU pod under
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
commit 0bd2a0f5c0b0f0005d57850c12466fa82b414c59 (fetched by the pod from raw.githubusercontent.com, sha256-checked)
cmd: rustc --edition 2021 -O examples/n2_routes.rs -o n2 && ./n2 "$OUT" gluecheck && N2_THREADS=16 ./n2 "$OUT" glue
host: RunPod CPU pod p22sy8golm3xb0 (h2-glue), 16 vCPU, flavors cpu5c,cpu3c, image rust:1-bookworm, launched 2026-10-18T22:24:37Z
cpu: AMD EPYC 4564P 16-Core Processor
rustc 1.99.1 (58cf8f9c3 2026-10-13)
runner: research/weil_descent_angles_20261018/runpod/podrun.py
post-run (ffabb7cd8): glue_summary.txt re-derived from glue.csv by `n2_routes <dir> gluesummarize` (adds the pricing band with the Legendre split test at one I, Addendum H3); glue.csv unchanged.
```
== glue\_checks.txt
```
G p=131 N=17209 h=1 ell=17209 t=-47: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist true | G1 #C(F_p)=132 (p+1=132) #C(F_p2)=17256 (p^2+1-2t=17256) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=137 N=18911 h=1 ell=18911 t=-141: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist true | G1 #C(F_p)=138 (p+1=138) #C(F_p2)=19052 (p^2+1-2t=19052) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=131 N=17132 h=4 ell=4283 t=30: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=132 (p+1=132) #C(F_p2)=17102 (p^2+1-2t=17102) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=139 N=19204 h=4 ell=4801 t=118: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=140 (p+1=140) #C(F_p2)=19086 (p^2+1-2t=19086) | G2 assoc true inverse true N*A=0 degenerate | G3 hom true nonzero true ell*tau=0 true
G p=277 N=76603 h=1 ell=76603 t=127: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=278 (p+1=278) #C(F_p2)=76476 (p^2+1-2t=76476) | G2 assoc true inverse true N*A=0 degenerate | G3 hom true nonzero true ell*tau=0 true
G p=257 N=66301 h=1 ell=66301 t=-251: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=258 (p+1=258) #C(F_p2)=66552 (p^2+1-2t=66552) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 degenerate
G p=271 N=73252 h=4 ell=18313 t=190: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=272 (p+1=272) #C(F_p2)=73062 (p^2+1-2t=73062) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=257 N=66212 h=4 ell=16553 t=-162: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=258 (p+1=258) #C(F_p2)=66374 (p^2+1-2t=66374) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=1033 N=1065949 h=1 ell=1065949 t=1141: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=1034 (p+1=1034) #C(F_p2)=- (p^2+1-2t=1064808) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=1109 N=1231631 h=1 ell=1231631 t=-1749: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=1110 (p+1=1110) #C(F_p2)=- (p^2+1-2t=1233380) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=1097 N=1201292 h=4 ell=300323 t=2118: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=1098 (p+1=1098) #C(F_p2)=- (p^2+1-2t=1199174) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=1039 N=1079564 h=4 ell=269891 t=-42: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=1040 (p+1=1040) #C(F_p2)=- (p^2+1-2t=1079606) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=4507 N=20306837 h=1 ell=20306837 t=6213: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=4508 (p+1=4508) #C(F_p2)=- (p^2+1-2t=20300624) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=4457 N=19863127 h=1 ell=19863127 t=1723: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=4458 (p+1=4458) #C(F_p2)=- (p^2+1-2t=19861404) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=4549 N=20695252 h=4 ell=5173813 t=-1850: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=4550 (p+1=4550) #C(F_p2)=- (p^2+1-2t=20697102) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G p=4133 N=17083268 h=4 ell=4270817 t=-1578: t2=sigma(s2) true lambda^2 in F_p true lambda in F_p true | twist false | G1 #C(F_p)=4134 (p+1=4134) #C(F_p2)=- (p^2+1-2t=17084846) | G2 assoc true inverse true N*A=0 true | G3 hom true nonzero true ell*tau=0 true
G5 p=1063: fast = generic on 800/800 pairs (0 degenerate) | add fast 41.0M+1.00I generic 112.8M+4.00I | double fast 51.0M+1.00I generic 122.7M+4.00I
G5 p=17159: fast = generic on 800/800 pairs (0 degenerate) | add fast 41.0M+1.00I generic 113.0M+4.00I | double fast 51.0M+1.00I generic 123.0M+4.00I
G5 p=265871: fast = generic on 800/800 pairs (0 degenerate) | add fast 41.0M+1.00I generic 113.0M+4.00I | double fast 51.0M+1.00I generic 123.0M+4.00I
```
== glue\_summary.txt
```
H2 bits 28 h 1 Jglue: steps/p 0.4353 (s.e. 0.0168, 40 runs) | mean cost (I=3M, w=0) 5.0237e5 | rho theory 0.8860
H2 bits 28 h 1 R: steps/p 0.9556 (s.e. 0.0638, 40 runs) | mean cost (I=3M, w=0) 3.5065e5 | rho theory 0.8860
H2 bits 28 h 4 Jglue: steps/p 0.3561 (s.e. 0.0182, 40 runs) | mean cost (I=3M, w=0) 4.1819e5 | rho theory 0.4430
H2 bits 28 h 4 R: steps/p 0.4478 (s.e. 0.0353, 40 runs) | mean cost (I=3M, w=0) 2.0860e5 | rho theory 0.4430
H2 bits 32 h 1 Jglue: steps/p 0.4260 (s.e. 0.0184, 40 runs) | mean cost (I=3M, w=0) 1.5867e6 | rho theory 0.8860
H2 bits 32 h 1 R: steps/p 0.8440 (s.e. 0.0691, 40 runs) | mean cost (I=3M, w=0) 9.8975e5 | rho theory 0.8860
H2 bits 32 h 4 Jglue: steps/p 0.3163 (s.e. 0.0215, 40 runs) | mean cost (I=3M, w=0) 1.2304e6 | rho theory 0.4430
H2 bits 32 h 4 R: steps/p 0.4155 (s.e. 0.0308, 40 runs) | mean cost (I=3M, w=0) 5.5382e5 | rho theory 0.4430
H2 bits 36 h 1 Jglue: steps/p 0.4387 (s.e. 0.0144, 40 runs) | mean cost (I=3M, w=0) 5.9674e6 | rho theory 0.8860
H2 bits 36 h 1 R: steps/p 0.7782 (s.e. 0.0757, 40 runs) | mean cost (I=3M, w=0) 3.2959e6 | rho theory 0.8860
H2 bits 36 h 4 Jglue: steps/p 0.3372 (s.e. 0.0179, 40 runs) | mean cost (I=3M, w=0) 4.8011e6 | rho theory 0.4430
H2 bits 36 h 4 R: steps/p 0.4033 (s.e. 0.0411, 40 runs) | mean cost (I=3M, w=0) 1.8353e6 | rho theory 0.4430
H2 bits 40 h 1 Jglue: steps/p 0.4428 (s.e. 0.0158, 40 runs) | mean cost (I=3M, w=0) 2.4105e7 | rho theory 0.8860
H2 bits 40 h 1 R: steps/p 0.8600 (s.e. 0.0681, 40 runs) | mean cost (I=3M, w=0) 1.4378e7 | rho theory 0.8860
H2 bits 40 h 4 Jglue: steps/p 0.3496 (s.e. 0.0175, 40 runs) | mean cost (I=3M, w=0) 1.9498e7 | rho theory 0.4430
H2 bits 40 h 4 R: steps/p 0.4446 (s.e. 0.0314, 40 runs) | mean cost (I=3M, w=0) 7.7110e6 | rho theory 0.4430
H2 bits 28 h 1: Jglue/rho cost ratio (measured, I=3M, w=0) 1.433
H2 bits 28 h 4: Jglue/rho cost ratio (measured, I=3M, w=0) 2.005
H2 bits 32 h 1: Jglue/rho cost ratio (measured, I=3M, w=0) 1.603
H2 bits 32 h 4: Jglue/rho cost ratio (measured, I=3M, w=0) 2.222
H2 bits 36 h 1: Jglue/rho cost ratio (measured, I=3M, w=0) 1.811
H2 bits 36 h 4: Jglue/rho cost ratio (measured, I=3M, w=0) 2.616
H2 bits 40 h 1: Jglue/rho cost ratio (measured, I=3M, w=0) 1.677
H2 bits 40 h 4: Jglue/rho cost ratio (measured, I=3M, w=0) 2.529
H2 Jglue: measured per Jacobian op 41.3M + 1.00I
H2 band bits 28 h 1: Jglue/rho in [0.594, 1.514] over 12 cells (IC cheaper in 6) -> unresolved
H2 band bits 28 h 4: Jglue/rho in [0.823, 2.136] over 12 cells (IC cheaper in 3) -> unresolved
H2 band bits 32 h 1: Jglue/rho in [0.706, 1.712] over 12 cells (IC cheaper in 6) -> unresolved
H2 band bits 32 h 4: Jglue/rho in [0.972, 2.372] over 12 cells (IC cheaper in 2) -> unresolved
H2 band bits 36 h 1: Jglue/rho in [0.815, 1.938] over 12 cells (IC cheaper in 3) -> unresolved
H2 band bits 36 h 4: Jglue/rho in [1.176, 2.788] over 12 cells (IC cheaper in 0) -> rho cheaper in every cell
H2 band bits 40 h 1: Jglue/rho in [0.759, 1.795] over 12 cells (IC cheaper in 6) -> unresolved
H2 band bits 40 h 4: Jglue/rho in [1.144, 2.690] over 12 cells (IC cheaper in 0) -> rho cheaper in every cell
H2 all answers verified: true
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
CPU(s) scaling MHz:                   78%
CPU max MHz:                          5881.0000
CPU min MHz:                          400.0000
BogoMIPS:                             8983.20
Flags:                                fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good amd_lbr_v2 nopl nonstop_tsc cpuid extd_apicid aperfmperf rapl pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate ssbd mba perfmon_v2 ibrs ibpb stibp ibrs_enhanced vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid cqm rdt_a avx512f avx512dq rdseed adx smap avx512ifma clflushopt clwb avx512cd sha_ni avx512bw avx512vl xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local user_shstk avx512_bf16 clzero irperf xsaveerptr rdpru wbnoinvd cppc arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif x2avic v_spec_ctrl vnmi avx512vbmi umip pku ospke avx512_vbmi2 gfni vaes vpclmulqdq avx512_vnni avx512_bitalg avx512_vpopcntdq rdpid overflow_recov succor smca flush_l1d
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
Mem:             124          27          81           0          18          97
Swap:              0           0           0
16
Linux c7864259a52b 6.8.0-45-generic #45~22.04.1-Ubuntu SMP PREEMPT_DYNAMIC Wed Sep 11 15:25:05 UTC 2 x86_64 GNU/Linux
rustc 1.99.1 (58cf8f9c3 2026-10-13)
```
== rusage.txt
```
Command being timed: "bash -c mkdir -p examples && curl -sSfL https://raw.githubusercontent.com/aburan28/crypto/0bd2a0f5c0b0f0005d57850c12466fa82b414c59/examples/n2_routes.rs -o examples/n2_routes.rs && echo 'fd8faeb6fb42db70933592f2dd2ad7d4ceecb66900b738e45e42f76ede88d66e  examples/n2_routes.rs' | sha256sum -c - && rustc --edition 2021 -O examples/n2_routes.rs -o n2 && ./n2 "$OUT" gluecheck && N2_THREADS=16 ./n2 "$OUT" glue"
	User time (seconds): 78.58
	System time (seconds): 2.09
	Percent of CPU this job got: 563%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:14.31
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 627408
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 39
	Minor (reclaiming a frame) page faults: 674573
	Voluntary context switches: 1497
	Involuntary context switches: 1338
	Swaps: 0
	File system inputs: 0
	File system outputs: 13192
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
```
== glue.csv (first 12 of 640 rows)
#table(columns: 22, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*h*], [*instance*], [*p*], [*N*], [*ell*], [*twist*], [*target*], [*arm*], [*steps*], [*relations*], [*jac\_ops*], [*walk\_M*], [*walk\_I*], [*walk\_native*], [*jac\_M*], [*jac\_I*], [*la\_native*], [*eadd\_M*], [*eadd\_I*], [*verified*], [*seconds*],
[28], [1], [0], [17317], [299862469], [299862469], [0], [0], [Jglue], [4639], [2318], [7385], [337941], [7367], [13914], [318815], [7329], [15843], [12], [1], [1], [0.002],
[28], [1], [0], [17317], [299862469], [299862469], [0], [0], [R], [20634], [0], [0], [336477], [26381], [41200], [0], [0], [0], [12], [1], [1], [0.004],
[28], [1], [0], [17317], [299862469], [299862469], [0], [1], [Jglue], [6629], [3356], [9588], [437549], [9572], [19971], [410279], [9534], [27095], [12], [1], [1], [0.003],
[28], [1], [0], [17317], [299862469], [299862469], [0], [1], [R], [28472], [0], [0], [435680], [34550], [56983], [0], [0], [0], [12], [1], [1], [0.005],
[28], [1], [0], [17317], [299862469], [299862469], [0], [2], [Jglue], [9763], [4820], [12673], [576321], [12675], [29167], [536850], [12631], [48101], [12], [1], [1], [0.004],
[28], [1], [0], [17317], [299862469], [299862469], [0], [2], [R], [19398], [0], [0], [321035], [25130], [38815], [0], [0], [0], [12], [1], [1], [0.003],
[28], [1], [0], [17317], [299862469], [299862469], [0], [3], [Jglue], [3755], [1876], [6503], [298280], [6492], [11263], [282678], [6454], [12117], [12], [1], [1], [0.002],
[28], [1], [0], [17317], [299862469], [299862469], [0], [3], [R], [10060], [0], [0], [205155], [15550], [20108], [0], [0], [0], [12], [1], [1], [0.002],
[28], [1], [0], [17317], [299862469], [299862469], [0], [4], [Jglue], [9036], [4542], [11879], [540388], [11859], [27156], [503557], [11821], [42555], [12], [1], [1], [0.004],
[28], [1], [0], [17317], [299862469], [299862469], [0], [4], [R], [13843], [0], [0], [252276], [19433], [27687], [0], [0], [0], [12], [1], [1], [0.003],
[28], [1], [0], [17317], [299862469], [299862469], [0], [5], [Jglue], [9193], [4585], [12130], [551577], [12109], [27556], [514451], [12084], [43635], [12], [1], [1], [0.004],
[28], [1], [0], [17317], [299862469], [299862469], [0], [5], [R], [5287], [0], [0], [145253], [10614], [10573], [0], [0], [0], [12], [1], [1], [0.001]
)

== Graph
#figure(image("../../figures/h2_glue.svg", width: 100%), caption: [H2: general E~E^sigma gluing, h = 1 and 4: group operations per p and the pricing band.])
