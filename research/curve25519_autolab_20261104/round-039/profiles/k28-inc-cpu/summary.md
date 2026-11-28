# cpu profile: round-039 inc, 160 jobs x 10, ecbench exec

- status: **ok**
- command: `/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/jobs/run_arm.sh inc 10`
- binary: `/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/jobs/run_arm.sh` sha256 `7688250cdf0577ba…`
- commit: `f187cad6a260` (0 dirty tracked files)
- host: Intel(R) Xeon(R) Processor @ 2.10GHz, 4 logical CPUs, kernel 6.18.44-fc-v653, PMU absent, perf_event_paranoid 2
- perf: perf version 6.8.12
- isolation: none; wall 12.141 s (profiled run: not a timing measurement)
- started: 2026-11-19T15:36:00Z

## Hotspots (perf, self samples)

Event `cpu-clock` (software timer), 999 Hz, call graph `dwarf`, 9480 samples, 0 lost events. s.e. is the binomial standard error of the share; shares within ~2 s.e. of each other are not ordered by this profile.

| # | self % | ±s.e. | samples | symbol | object |
|---|---:|---:|---:|---|---|
| 1 | 19.49 | 0.41 | 1848 | `_RNvMso_NtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundaryNtB5_10PrimeCurve7add_raw.llvm.5384899875590169803` | ecbench |
| 2 | 6.37 | 0.25 | 604 | `exc_page_fault` | [kernel.kallsyms] |
| 3 | 5.76 | 0.24 | 546 | `_RNCINvMsF_NtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundaryINtB8_9WalkTableNtB8_10PrimePointE11build_innerNtB8_10PrimeCurveEs4_0Bc_` | ecbench |
| 4 | 3.43 | 0.19 | 325 | `next_uptodate_folio` | [kernel.kallsyms] |
| 5 | 3.04 | 0.18 | 288 | `intel_check_word.constprop.0` | ld-linux-x86-64.so.2 |
| 6 | 2.16 | 0.15 | 205 | `__vdso_clock_gettime` | [vdso] |
| 7 | 1.99 | 0.14 | 189 | `_raw_spin_lock` | [kernel.kallsyms] |
| 8 | 1.66 | 0.13 | 157 | `get_common_cache_info.constprop.0` | ld-linux-x86-64.so.2 |
| 9 | 1.6 | 0.13 | 152 | `__memmove_avx512_unaligned_erms` | libc.so.6 |
| 10 | 1.42 | 0.12 | 135 | `_dl_relocate_object` | ld-linux-x86-64.so.2 |
| 11 | 1.41 | 0.12 | 134 | `_RNvMs1_NtCsfKiFC1ztrmh_9hashbrown3mapINtB5_7HashMapyjINtNtCscI6d9CVNmLh_4core4hash18BuildHasherDefaultNtNtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundary9MixHasherEE6insertB1I_` | ecbench |
| 12 | 1.4 | 0.12 | 133 | `_raw_spin_unlock_irqrestore` | [kernel.kallsyms] |
| 13 | 1.34 | 0.12 | 127 | `gather_pte_stats` | [kernel.kallsyms] |
| 14 | 1.33 | 0.12 | 126 | `memcg1_commit_charge` | [kernel.kallsyms] |
| 15 | 1.32 | 0.12 | 125 | `_int_malloc` | libc.so.6 |
| 16 | 1.26 | 0.11 | 119 | `_RNvNtNtCsi9YoAGUbCUq_10std_detect6detect5cache21detect_and_initialize` | ecbench |
| 17 | 0.94 | 0.1 | 89 | `_RNvNvNtCsiT8PH1w6pqq_11rand_chacha4guts11refill_wide9impl_avx2` | ecbench |
| 18 | 0.92 | 0.1 | 87 | `__cpu_indicator_init` | libgcc_s.so.1 |
| 19 | 0.89 | 0.1 | 84 | `__list_del_entry_valid_or_report` | [kernel.kallsyms] |
| 20 | 0.88 | 0.1 | 83 | `clear_page_erms` | [kernel.kallsyms] |
| 21 | 0.81 | 0.09 | 77 | `do_lookup_x` | ld-linux-x86-64.so.2 |
| 22 | 0.66 | 0.08 | 63 | `filemap_map_pages` | [kernel.kallsyms] |
| 23 | 0.62 | 0.08 | 59 | `_RINvNtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundary43collect_and_solve_relations_scaled_touchingNtB2_10PrimeCurveDNtB2_14RelationSolverEL_INtNtNtNtCsgczF5crJ4sT_3std11collections4hash3map7HashMapyuINtNtCscI6d9CVNmLh_4core4hash18BuildHasherDefaultNtB2_9MixHasherEENCINvNtB4_12ic_framework18run_pipeline_innerB1H_Es0_0NCB4q_s1_0EB6_` | ecbench |
| 24 | 0.58 | 0.08 | 55 | `copy_mc_enhanced_fast_string` | [kernel.kallsyms] |
| 25 | 0.57 | 0.08 | 54 | `_raw_spin_unlock_irq` | [kernel.kallsyms] |

Flame graph: [flame.svg](flame.svg) (folded stacks in `folded.txt`).

## Files

- `command.stderr`
- `command.stdout`
- `flame.svg`
- `folded.txt`
- `hotspots.json`
- `hotspots.svg`
- `hotspots.tsv`
- `manifest.json`
- `perf.data`
- `report.txt`

Raw profiler output (perf.data, *.ncu-rep, *.nsys-rep, callgrind.out) is large: push it with `crypto store push` (AGENTS.md section 13) rather than committing it.
