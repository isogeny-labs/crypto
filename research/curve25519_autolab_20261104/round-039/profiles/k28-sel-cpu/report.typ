#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)
#show table: set text(size: 7.5pt)

= cpu profile: round-039 sel, 160 jobs x 10, ecbench exec

- status: *ok*
- command: #raw("/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/jobs/run_arm.sh sel 10")
- binary: #raw("/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/jobs/run_arm.sh") sha256 #raw("7688250cdf0577ba…")
- commit: #raw("f187cad6a260") (0 dirty tracked files)
- host: Intel(R) Xeon(R) Processor \@ 2.10GHz, 4 logical CPUs, kernel 6.18.44-fc-v653, PMU absent, perf\_event\_paranoid 2
- perf: perf version 6.8.12
- isolation: none; wall 14.167 s (profiled run: not a timing measurement)
- started: 2026-11-19T15:35:39Z

== Hotspots (perf, self samples)

Event #raw("cpu-clock") (software timer), 999 Hz, call graph #raw("dwarf"), 11004 samples, 0 lost events. s.e. is the binomial standard error of the share; shares within \~2 s.e. of each other are not ordered by this profile.

#table(columns: 6,
  [*\#*],  [*self %*],  [*±s.e.*],  [*samples*],  [*symbol*],  [*object*],
  [1],  [16.24],  [0.35],  [1787],  [#raw("_RNvMso_NtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundaryNtB5_10PrimeCurve7add_raw.llvm.5384899875590169803")],  [ecbench],
  [2],  [8.42],  [0.26],  [926],  [#raw("exc_page_fault")],  [\[kernel.kallsyms\]],
  [3],  [4.58],  [0.2],  [504],  [#raw("_RNvMs6_NtCsfKiFC1ztrmh_9hashbrown3rawINtB5_8RawTableTyToyyEEE14insert_no_growCs6TdBej9u76G_10crypto_lib")],  [ecbench],
  [4],  [3.26],  [0.17],  [359],  [#raw("_raw_spin_lock")],  [\[kernel.kallsyms\]],
  [5],  [3.1],  [0.17],  [341],  [#raw("intel_check_word.constprop.0")],  [ld-linux-x86-64.so.2],
  [6],  [2.78],  [0.16],  [306],  [#raw("next_uptodate_folio")],  [\[kernel.kallsyms\]],
  [7],  [2.53],  [0.15],  [278],  [#raw("memcg1_commit_charge")],  [\[kernel.kallsyms\]],
  [8],  [2.04],  [0.13],  [225],  [#raw("_RINvMsH_NtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundaryINtB6_10TargetWalkNtB6_10PrimePointE12build_lcheckNtB6_10PrimeCurveEBa_")],  [ecbench],
  [9],  [1.65],  [0.12],  [182],  [#raw("_raw_spin_unlock_irqrestore")],  [\[kernel.kallsyms\]],
  [10],  [1.51],  [0.12],  [166],  [#raw("_RNvMNtCsfKiFC1ztrmh_9hashbrown11rustc_entryINtNtB4_3map7HashMapyToyyEINtNtCscI6d9CVNmLh_4core4hash18BuildHasherDefaultNtNtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundary9MixHasherEE11rustc_entryB1Y_")],  [ecbench],
  [11],  [1.45],  [0.11],  [160],  [#raw("clear_page_erms")],  [\[kernel.kallsyms\]],
  [12],  [1.39],  [0.11],  [153],  [#raw("_dl_relocate_object")],  [ld-linux-x86-64.so.2],
  [13],  [1.3],  [0.11],  [143],  [#raw("_RNvNtNtCsi9YoAGUbCUq_10std_detect6detect5cache21detect_and_initialize")],  [ecbench],
  [14],  [1.28],  [0.11],  [141],  [#raw("__memmove_avx512_unaligned_erms")],  [libc.so.6],
  [15],  [1.26],  [0.11],  [139],  [#raw("_int_malloc")],  [libc.so.6],
  [16],  [1.25],  [0.11],  [137],  [#raw("__list_del_entry_valid_or_report")],  [\[kernel.kallsyms\]],
  [17],  [1.23],  [0.1],  [135],  [#raw("gather_pte_stats")],  [\[kernel.kallsyms\]],
  [18],  [1.12],  [0.1],  [123],  [#raw("get_common_cache_info.constprop.0")],  [ld-linux-x86-64.so.2],
  [19],  [1.06],  [0.1],  [117],  [#raw("_RNvNvNtCsiT8PH1w6pqq_11rand_chacha4guts11refill_wide9impl_avx2")],  [ecbench],
  [20],  [1.04],  [0.1],  [114],  [#raw("_RNvMsH_NtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundaryINtB5_10TargetWalkNtB5_10PrimePointE9canon_keyB9_")],  [ecbench],
  [21],  [0.95],  [0.09],  [105],  [#raw("unmap_page_range")],  [\[kernel.kallsyms\]],
  [22],  [0.85],  [0.09],  [93],  [#raw("__cpu_indicator_init")],  [libgcc\_s.so.1],
  [23],  [0.78],  [0.08],  [86],  [#raw("_RINvYNtNtNtCs5xYxNcWIgJX_4rand4rngs3std6StdRngNtNtB9_3rng3Rng9gen_rangejINtNtNtCscI6d9CVNmLh_4core3ops5range5RangejEECs6TdBej9u76G_10crypto_lib")],  [ecbench],
  [24],  [0.68],  [0.08],  [75],  [#raw("get_mem_cgroup_from_mm")],  [\[kernel.kallsyms\]],
  [25],  [0.67],  [0.08],  [74],  [#raw("do_lookup_x")],  [ld-linux-x86-64.so.2],
)

Flame graph: #link("flame.svg")[flame.svg] (folded stacks in #raw("folded.txt")).

== Files

- #raw("command.stderr")
- #raw("command.stdout")
- #raw("flame.svg")
- #raw("folded.txt")
- #raw("hotspots.json")
- #raw("hotspots.svg")
- #raw("hotspots.tsv")
- #raw("manifest.json")
- #raw("perf.data")
- #raw("report.txt")

Raw profiler output (perf.data, \*.ncu-rep, \*.nsys-rep, callgrind.out) is large: push it with #raw("crypto store push") (AGENTS.md section 13) rather than committing it.

== Graphs

#figure(image("hotspots.svg", width: 100%))

#figure(image("flame.svg", width: 100%))

