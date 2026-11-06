#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

= IC collection: the aimed subset gathered once per unit (2026-10-23)

*Study:* `research/ic_aimed_gather_20261023/` · *Protocol:* `PROTOCOL.md`, committed (`8660e8ab4`) before any measurement · *Baseline:* `d34b9b390` (`origin/main`) · *Workload:* `ic price` on rule v3's `k0n53/T01.params.json` (and `k0n61` for identity and Callgrind) · *Host:* one x86-64 cloud VM (Intel Xeon @ 2.10 GHz, 4 vCPU, AVX-512), no hardware PMU.

Aimed collection scans, each trial, a window of a fixed sorted subset of the factor base at a rotating offset. Every trial gathered that window's coordinates again. Now `aimed()` gathers them once per unit into doubled arrays in the same order, and a trial's scan (`Scan::Gathered`) hands two slices to the folding kernel. The same summands go in, in the same order.

== Identity (measured)

- *Outputs:* identical at k0n53 and k0n61 once timing fields are removed: every count, the status, `all_verified`, and the recovered logarithms `518509155375` and `153752017445515`.
- *Unit tests:* the new `a_pre_gathered_scan_hands_the_sink_what_the_index_list_does` compares sink order exactly on a folded table at $n$ = 19 and 31. It and the other 23 scan tests pass (one is ignored by design).
- *Timing runs:* all 70 recovered the same logarithm.

== Three measurements

#figure(image("three-measures.svg", width: 100%), caption: [Left: Callgrind instructions per `ic price` process. Middle: share of native `perf` samples in `witnesses_fast_scan` and its closures, median of 3 runs a side. Right: paired native ratios with 95% bootstrap intervals.])

+ *Callgrind instructions (measured).* 32.30 → 31.57 $times 10^9$ at k0n53 (1.023×) and 140.0 → 136.0 $times 10^9$ at k0n61 (1.029×). Valgrind runs no AVX-512, so this counts the scalar fallback of the key kernels; the fused SoA path changed here does run under it.
+ *perf sampling (measured, descriptive).* The gather closure (2.53–2.68% of samples on main) is gone. The scan family drops from 20.6–21.6% to 18.3–19.1%. The hash closure is now inlined into `witnesses_fast_scan`, so only the family total is comparable.
+ *Native timing (measured).* 10 A/A runs (5 pairs), then 30 paired A/B rounds with the order alternating, all under `isolated_bench --cpus 3`. 70 processes, none contended, none failed; 38 launches were refused for CPU pressure and retried.

#table(columns: 5, align: (left, right, left, right, left),
  [*metric*], [*median ratio*], [*95% interval*], [*rounds > 1*], [*A/A interval*],
  [collect phase], [1.111], [1.065 – 1.160], [28 / 30], [0.944 – 1.087],
  [whole set-up], [1.037], [1.018 – 1.068], [26 / 30], [0.953 – 1.121],
)
#figure(image("paired-collect-ratio.svg", width: 95%), caption: [Per-round collect-phase ratio (main / candidate), the median and its interval, and the A/A pairs (`runs/time/`).])

*Verdict under the declared rule: not established.* Both intervals exclude 1, and 28 of 30 rounds favour the change. The rule committed before measuring also asks the interval's lower end to clear the A/A interval's upper end. It does not: 1.065 against 1.087 for collect, 1.018 against 1.121 for the set-up. With 5 A/A pairs that interval is wide. The rule is not changed after the fact. A larger A/A is the way to settle it.

The collect phase gains more in time (11%) than the whole process does in instructions (2.3%). The removed work is a scattered load from the base's coordinate arrays per summand. Its cost is cache misses, which Callgrind does not count.

== Scope

- *Counted units.* Every count is identical, so GAE records and every IC row's operation count stand. The change is native cost per set-up only.
- *Where it applies.* Aimed collection on a folded table with stored coordinates (the fused SoA path). Cyclic windows and the point-list path are unchanged.
- *Not measured.* Other sizes' native time; other hosts; IPC (no PMU).
