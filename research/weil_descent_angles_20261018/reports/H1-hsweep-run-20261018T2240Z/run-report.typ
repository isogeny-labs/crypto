#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: H1-hsweep-run-20261018T2240Z

Descriptive record of one recorded run of examples/n2\_routes.rs, mode hsweep (Addendum H1), on a RunPod CPU pod under
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
cmd: rustc --edition 2021 -O examples/n2_routes.rs -o n2 && N2_THREADS=16 ./n2 "$OUT" hsweep
host: RunPod CPU pod rdlhphjtyovtl4 (h1-cofactor-law-b), 16 vCPU, flavors cpu5c,cpu3c, image rust:1-bookworm, launched 2026-10-18T22:24:36Z
cpu: AMD EPYC 4564P 16-Core Processor
rustc 1.99.1 (58cf8f9c3 2026-10-13)
runner: research/weil_descent_angles_20261018/runpod/podrun.py
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
CPU(s) scaling MHz:                   36%
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
Mem:             124          13          72           0          39         111
Swap:              0           0           0
16
Linux 3eac50930d26 6.8.0-51-generic #52~22.04.1-Ubuntu SMP PREEMPT_DYNAMIC Mon Dec  9 15:00:52 UTC 2 x86_64 GNU/Linux
rustc 1.99.1 (58cf8f9c3 2026-10-13)
```
== hsweep\_summary.txt
```
H1 bits 32 h 4 J: trials/p 0.3497 (s.e. over 4 instances 0.0406) | model 0.3639 | rho theory 0.886/sqrt(h) 0.4430
H1 bits 32 h 4 Jind: trials/p 0.3704 (s.e. over 4 instances 0.0146) | rho theory 0.886/sqrt(h) 0.4430
H1 bits 32 h 4 R: trials/p 0.5019 (s.e. over 4 instances 0.0406) | rho theory 0.886/sqrt(h) 0.4430
H1 bits 32 h 8 J: trials/p 0.2881 (s.e. over 4 instances 0.0131) | model 0.3033 | rho theory 0.886/sqrt(h) 0.3132
H1 bits 32 h 8 Jind: trials/p 0.3243 (s.e. over 4 instances 0.0310) | rho theory 0.886/sqrt(h) 0.3132
H1 bits 32 h 8 R: trials/p 0.3075 (s.e. over 4 instances 0.0234) | rho theory 0.886/sqrt(h) 0.3132
H1 bits 32 h 12 J: trials/p 0.2253 (s.e. over 4 instances 0.0141) | model 0.2993 | rho theory 0.886/sqrt(h) 0.2558
H1 bits 32 h 12 Jind: trials/p 0.2344 (s.e. over 4 instances 0.0173) | rho theory 0.886/sqrt(h) 0.2558
H1 bits 32 h 12 R: trials/p 0.2851 (s.e. over 4 instances 0.0074) | rho theory 0.886/sqrt(h) 0.2558
H1 bits 32 h 16 J: trials/p 0.2323 (s.e. over 4 instances 0.0218) | model 0.2551 | rho theory 0.886/sqrt(h) 0.2215
H1 bits 32 h 16 Jind: trials/p 0.2182 (s.e. over 4 instances 0.0056) | rho theory 0.886/sqrt(h) 0.2215
H1 bits 32 h 16 R: trials/p 0.2319 (s.e. over 4 instances 0.0120) | rho theory 0.886/sqrt(h) 0.2215
H1 bits 32 h 24 J: trials/p 0.1872 (s.e. over 4 instances 0.0151) | model 0.2518 | rho theory 0.886/sqrt(h) 0.1809
H1 bits 32 h 24 Jind: trials/p 0.2183 (s.e. over 4 instances 0.0031) | rho theory 0.886/sqrt(h) 0.1809
H1 bits 32 h 24 R: trials/p 0.2008 (s.e. over 4 instances 0.0165) | rho theory 0.886/sqrt(h) 0.1809
H1 bits 32 h 32 J: trials/p 0.1658 (s.e. over 4 instances 0.0129) | model 0.2179 | rho theory 0.886/sqrt(h) 0.1566
H1 bits 32 h 32 Jind: trials/p 0.1693 (s.e. over 4 instances 0.0160) | rho theory 0.886/sqrt(h) 0.1566
H1 bits 32 h 32 R: trials/p 0.1737 (s.e. over 4 instances 0.0075) | rho theory 0.886/sqrt(h) 0.1566
H1 bits 32 h 48 J: trials/p 0.1473 (s.e. over 4 instances 0.0057) | model 0.1883 | rho theory 0.886/sqrt(h) 0.1279
H1 bits 32 h 48 Jind: trials/p 0.1578 (s.e. over 4 instances 0.0100) | rho theory 0.886/sqrt(h) 0.1279
H1 bits 32 h 48 R: trials/p 0.1440 (s.e. over 4 instances 0.0125) | rho theory 0.886/sqrt(h) 0.1279
H1 bits 32 h 64 J: trials/p 0.1360 (s.e. over 4 instances 0.0099) | model 0.1656 | rho theory 0.886/sqrt(h) 0.1108
H1 bits 32 h 64 Jind: trials/p 0.1291 (s.e. over 4 instances 0.0112) | rho theory 0.886/sqrt(h) 0.1108
H1 bits 32 h 64 R: trials/p 0.1157 (s.e. over 4 instances 0.0030) | rho theory 0.886/sqrt(h) 0.1108
H1 bits 32 h 96 J: trials/p 0.1026 (s.e. over 4 instances 0.0058) | model 0.1498 | rho theory 0.886/sqrt(h) 0.0904
H1 bits 32 h 96 Jind: trials/p 0.1265 (s.e. over 4 instances 0.0077) | rho theory 0.886/sqrt(h) 0.0904
H1 bits 32 h 96 R: trials/p 0.0970 (s.e. over 4 instances 0.0046) | rho theory 0.886/sqrt(h) 0.0904
H1 bits 32 h 128 J: trials/p 0.0861 (s.e. over 4 instances 0.0082) | model 0.1332 | rho theory 0.886/sqrt(h) 0.0783
H1 bits 32 h 128 Jind: trials/p 0.1022 (s.e. over 4 instances 0.0066) | rho theory 0.886/sqrt(h) 0.0783
H1 bits 32 h 128 R: trials/p 0.0977 (s.e. over 4 instances 0.0089) | rho theory 0.886/sqrt(h) 0.0783
H1 bits 36 h 4 J: trials/p 0.3622 (s.e. over 4 instances 0.0136) | model 0.3440 | rho theory 0.886/sqrt(h) 0.4430
H1 bits 36 h 4 R: trials/p 0.4670 (s.e. over 4 instances 0.0553) | rho theory 0.886/sqrt(h) 0.4430
H1 bits 36 h 8 J: trials/p 0.2620 (s.e. over 4 instances 0.0144) | model 0.3077 | rho theory 0.886/sqrt(h) 0.3132
H1 bits 36 h 8 R: trials/p 0.3041 (s.e. over 4 instances 0.0109) | rho theory 0.886/sqrt(h) 0.3132
H1 bits 36 h 12 J: trials/p 0.2274 (s.e. over 4 instances 0.0267) | model 0.3050 | rho theory 0.886/sqrt(h) 0.2558
H1 bits 36 h 12 R: trials/p 0.2913 (s.e. over 4 instances 0.0334) | rho theory 0.886/sqrt(h) 0.2558
H1 bits 36 h 16 J: trials/p 0.2353 (s.e. over 4 instances 0.0189) | model 0.2501 | rho theory 0.886/sqrt(h) 0.2215
H1 bits 36 h 16 R: trials/p 0.2328 (s.e. over 4 instances 0.0230) | rho theory 0.886/sqrt(h) 0.2215
H1 bits 36 h 24 J: trials/p 0.1821 (s.e. over 4 instances 0.0076) | model 0.2439 | rho theory 0.886/sqrt(h) 0.1809
H1 bits 36 h 24 R: trials/p 0.2112 (s.e. over 4 instances 0.0164) | rho theory 0.886/sqrt(h) 0.1809
H1 bits 36 h 32 J: trials/p 0.1581 (s.e. over 4 instances 0.0106) | model 0.2332 | rho theory 0.886/sqrt(h) 0.1566
H1 bits 36 h 32 R: trials/p 0.1865 (s.e. over 4 instances 0.0090) | rho theory 0.886/sqrt(h) 0.1566
H1 bits 36 h 48 J: trials/p 0.1517 (s.e. over 4 instances 0.0157) | model 0.1844 | rho theory 0.886/sqrt(h) 0.1279
H1 bits 36 h 48 R: trials/p 0.1319 (s.e. over 4 instances 0.0090) | rho theory 0.886/sqrt(h) 0.1279
H1 bits 36 h 64 J: trials/p 0.1271 (s.e. over 4 instances 0.0064) | model 0.1721 | rho theory 0.886/sqrt(h) 0.1108
H1 bits 36 h 64 R: trials/p 0.1332 (s.e. over 4 instances 0.0074) | rho theory 0.886/sqrt(h) 0.1108
H1 bits 36 h 96 J: trials/p 0.0909 (s.e. over 4 instances 0.0083) | model 0.1489 | rho theory 0.886/sqrt(h) 0.0904
H1 bits 36 h 96 R: trials/p 0.0999 (s.e. over 4 instances 0.0070) | rho theory 0.886/sqrt(h) 0.0904
H1 bits 36 h 128 J: trials/p 0.0895 (s.e. over 4 instances 0.0079) | model 0.1345 | rho theory 0.886/sqrt(h) 0.0783
H1 bits 36 h 128 R: trials/p 0.0790 (s.e. over 4 instances 0.0049) | rho theory 0.886/sqrt(h) 0.0783
H1 bits 40 h 4 J: trials/p 0.3396 (s.e. over 4 instances 0.0163) | model 0.3475 | rho theory 0.886/sqrt(h) 0.4430
H1 bits 40 h 4 R: trials/p 0.4147 (s.e. over 4 instances 0.0344) | rho theory 0.886/sqrt(h) 0.4430
H1 bits 40 h 8 J: trials/p 0.2740 (s.e. over 4 instances 0.0172) | model 0.3048 | rho theory 0.886/sqrt(h) 0.3132
H1 bits 40 h 8 R: trials/p 0.3116 (s.e. over 4 instances 0.0095) | rho theory 0.886/sqrt(h) 0.3132
H1 bits 40 h 12 J: trials/p 0.2499 (s.e. over 4 instances 0.0044) | model 0.2979 | rho theory 0.886/sqrt(h) 0.2558
H1 bits 40 h 12 R: trials/p 0.2570 (s.e. over 4 instances 0.0190) | rho theory 0.886/sqrt(h) 0.2558
H1 bits 40 h 16 J: trials/p 0.2448 (s.e. over 4 instances 0.0222) | model 0.2534 | rho theory 0.886/sqrt(h) 0.2215
H1 bits 40 h 16 R: trials/p 0.2054 (s.e. over 4 instances 0.0122) | rho theory 0.886/sqrt(h) 0.2215
H1 bits 40 h 24 J: trials/p 0.1568 (s.e. over 4 instances 0.0101) | model 0.2413 | rho theory 0.886/sqrt(h) 0.1809
H1 bits 40 h 24 R: trials/p 0.1706 (s.e. over 4 instances 0.0162) | rho theory 0.886/sqrt(h) 0.1809
... (17 more lines in hsweep_summary.txt)
```
== rusage.txt
```
Command being timed: "bash -c mkdir -p examples && curl -sSfL https://raw.githubusercontent.com/aburan28/crypto/0bd2a0f5c0b0f0005d57850c12466fa82b414c59/examples/n2_routes.rs -o examples/n2_routes.rs && echo 'fd8faeb6fb42db70933592f2dd2ad7d4ceecb66900b738e45e42f76ede88d66e  examples/n2_routes.rs' | sha256sum -c - && rustc --edition 2021 -O examples/n2_routes.rs -o n2 && N2_THREADS=16 ./n2 "$OUT" hsweep"
	User time (seconds): 803.56
	System time (seconds): 2.64
	Percent of CPU this job got: 1457%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:55.31
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 846272
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 36
	Minor (reclaiming a frame) page faults: 1354216
	Voluntary context switches: 1275
	Involuntary context switches: 1837
	Swaps: 0
	File system inputs: 0
	File system outputs: 13544
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
```
== hsweep.csv (first 12 of 2800 rows)
#table(columns: 17, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*bits*], [*h*], [*instance*], [*p*], [*ell*], [*cofactor\_exponent*], [*target*], [*arm*], [*theta*], [*steps*], [*relations*], [*stored\_rows*], [*verified*], [*model\_vertices*], [*model\_first\_cycle\_edges*], [*M\_total*], [*seconds*],
[32], [12], [0], [71329], [423989581], [6], [0], [J], [1], [12210], [6158], [6157], [1], [35664], [10938.860], [688418], [0.005466],
[32], [12], [0], [71329], [423989581], [6], [0], [Jind], [1], [18680], [9335], [9334], [1], [35664], [10938.860], [69087870], [0.334714],
[32], [12], [0], [71329], [423989581], [6], [0], [R], [1], [21854], [0], [326], [1], [35664], [10938.860], [354091], [0.005026],
[32], [12], [0], [71329], [423989581], [6], [1], [J], [1], [19515], [9715], [9714], [1], [35664], [10938.860], [1020696], [0.008217],
[32], [12], [0], [71329], [423989581], [6], [1], [Jind], [1], [28782], [14423], [14422], [1], [35664], [10938.860], [106413327], [0.509993],
[32], [12], [0], [71329], [423989581], [6], [1], [R], [1], [29485], [0], [430], [1], [35664], [10938.860], [450460], [0.006606],
[32], [12], [0], [71329], [423989581], [6], [2], [J], [1], [8193], [4189], [4188], [1], [35664], [10938.860], [504712], [0.003951],
[32], [12], [0], [71329], [423989581], [6], [2], [Jind], [1], [28260], [14012], [14011], [1], [35664], [10938.860], [104436752], [0.501917],
[32], [12], [0], [71329], [423989581], [6], [2], [R], [1], [19241], [0], [344], [1], [35664], [10938.860], [321485], [0.004623],
[32], [12], [0], [71329], [423989581], [6], [3], [J], [1], [17398], [8796], [8795], [1], [35664], [10938.860], [928748], [0.007530],
[32], [12], [0], [71329], [423989581], [6], [3], [Jind], [1], [6475], [3301], [3300], [1], [35664], [10938.860], [24030024], [0.115172],
[32], [12], [0], [71329], [423989581], [6], [3], [R], [1], [31196], [0], [501], [1], [35664], [10938.860], [470325], [0.006929]
)

== Graph
#figure(image("../../figures/h1_cofactor_law.svg", width: 100%), caption: [H1: Arm J trials per p against the cofactor h, with rho and the M-H1 model; 30 cells x 4 instances x 10 targets.])
