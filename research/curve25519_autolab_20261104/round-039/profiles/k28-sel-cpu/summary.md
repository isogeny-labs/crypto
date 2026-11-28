# cpu profile: round-039 sel, 160 jobs x 10, ecbench exec

- status: **ok**
- command: `/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/jobs/run_arm.sh sel 10`
- binary: `/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/jobs/run_arm.sh` sha256 `7688250cdf0577ba…`
- commit: `f187cad6a260` (0 dirty tracked files)
- host: Intel(R) Xeon(R) Processor @ 2.10GHz, 4 logical CPUs, kernel 6.18.44-fc-v653, PMU absent, perf_event_paranoid 2
- perf: perf version 6.8.12
- isolation: none; wall 14.167 s (profiled run: not a timing measurement)
- started: 2026-11-19T15:35:39Z

## Hotspots (perf, self samples)

Event `cpu-clock` (software timer), 999 Hz, call graph `dwarf`, 11004 samples, 0 lost events. s.e. is the binomial standard error of the share; shares within ~2 s.e. of each other are not ordered by this profile.

| # | self % | ±s.e. | samples | symbol | object |
|---|---:|---:|---:|---|---|
| 1 | 16.24 | 0.35 | 1787 | `_RNvMso_NtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundaryNtB5_10PrimeCurve7add_raw.llvm.5384899875590169803` | ecbench |
| 2 | 8.42 | 0.26 | 926 | `exc_page_fault` | [kernel.kallsyms] |
| 3 | 4.58 | 0.2 | 504 | `_RNvMs6_NtCsfKiFC1ztrmh_9hashbrown3rawINtB5_8RawTableTyToyyEEE14insert_no_growCs6TdBej9u76G_10crypto_lib` | ecbench |
| 4 | 3.26 | 0.17 | 359 | `_raw_spin_lock` | [kernel.kallsyms] |
| 5 | 3.1 | 0.17 | 341 | `intel_check_word.constprop.0` | ld-linux-x86-64.so.2 |
| 6 | 2.78 | 0.16 | 306 | `next_uptodate_folio` | [kernel.kallsyms] |
| 7 | 2.53 | 0.15 | 278 | `memcg1_commit_charge` | [kernel.kallsyms] |
| 8 | 2.04 | 0.13 | 225 | `_RINvMsH_NtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundaryINtB6_10TargetWalkNtB6_10PrimePointE12build_lcheckNtB6_10PrimeCurveEBa_` | ecbench |
| 9 | 1.65 | 0.12 | 182 | `_raw_spin_unlock_irqrestore` | [kernel.kallsyms] |
| 10 | 1.51 | 0.12 | 166 | `_RNvMNtCsfKiFC1ztrmh_9hashbrown11rustc_entryINtNtB4_3map7HashMapyToyyEINtNtCscI6d9CVNmLh_4core4hash18BuildHasherDefaultNtNtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundary9MixHasherEE11rustc_entryB1Y_` | ecbench |
| 11 | 1.45 | 0.11 | 160 | `clear_page_erms` | [kernel.kallsyms] |
| 12 | 1.39 | 0.11 | 153 | `_dl_relocate_object` | ld-linux-x86-64.so.2 |
| 13 | 1.3 | 0.11 | 143 | `_RNvNtNtCsi9YoAGUbCUq_10std_detect6detect5cache21detect_and_initialize` | ecbench |
| 14 | 1.28 | 0.11 | 141 | `__memmove_avx512_unaligned_erms` | libc.so.6 |
| 15 | 1.26 | 0.11 | 139 | `_int_malloc` | libc.so.6 |
| 16 | 1.25 | 0.11 | 137 | `__list_del_entry_valid_or_report` | [kernel.kallsyms] |
| 17 | 1.23 | 0.1 | 135 | `gather_pte_stats` | [kernel.kallsyms] |
| 18 | 1.12 | 0.1 | 123 | `get_common_cache_info.constprop.0` | ld-linux-x86-64.so.2 |
| 19 | 1.06 | 0.1 | 117 | `_RNvNvNtCsiT8PH1w6pqq_11rand_chacha4guts11refill_wide9impl_avx2` | ecbench |
| 20 | 1.04 | 0.1 | 114 | `_RNvMsH_NtNtCs6TdBej9u76G_10crypto_lib13cryptanalysis11ic_boundaryINtB5_10TargetWalkNtB5_10PrimePointE9canon_keyB9_` | ecbench |
| 21 | 0.95 | 0.09 | 105 | `unmap_page_range` | [kernel.kallsyms] |
| 22 | 0.85 | 0.09 | 93 | `__cpu_indicator_init` | libgcc_s.so.1 |
| 23 | 0.78 | 0.08 | 86 | `_RINvYNtNtNtCs5xYxNcWIgJX_4rand4rngs3std6StdRngNtNtB9_3rng3Rng9gen_rangejINtNtNtCscI6d9CVNmLh_4core3ops5range5RangejEECs6TdBej9u76G_10crypto_lib` | ecbench |
| 24 | 0.68 | 0.08 | 75 | `get_mem_cgroup_from_mm` | [kernel.kallsyms] |
| 25 | 0.67 | 0.08 | 74 | `do_lookup_x` | ld-linux-x86-64.so.2 |

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
