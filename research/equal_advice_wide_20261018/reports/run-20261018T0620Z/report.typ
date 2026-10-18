#set page(margin: 2cm)
#set text(size: 10pt)
= Equal-advice wide instrument: n = 61 check and n = 41 harness smoke run

Run 2026-10-18 (UTC 06:20 to 06:30), commit of this PR, host: shared 4-vCPU container (load average about 14), 15 GB RAM, no isolation. Nothing was run at n = 73 or n = 83. Observations only.

== What ran
+ `ic_next10_09_equal_advice bl 61 0 65536` (narrow instrument) and `ic_equal_advice_wide bl 61 0 65536` (wide instrument, narrow field backend) on the first 8 targets of the frozen item-9 fixture `targets_n61.txt`, seed `v1`, 3 prefix levels, 1 thread. Records: `instrument_check_n61/{narrow,wide}_multi.jsonl`.
+ `BL_PLAN_ONLY` for `bl 61 0 5711008` against the frozen `bl_n61_64th.jsonl` record: `instrument_check_n61/plan_wide_bytes5711008.jsonl`.
+ `cargo test --release --example ic_equal_advice_wide -- --include-ignored`: 5 passed (n = 61 equivalence, planted collisions at n = 61, 73, 83, frozen rung constants, fingerprint).
+ `selftest` at n = 73 and n = 83 on the wide `Gf2_128` backend: group law, Frobenius, `[r]G = O`, batch inverse, planted collisions: pass. Subgroup orders found: 86,020,738,150,056,119 (n = 73) and 8,569,786,107,849,059 (n = 83), as frozen.
+ `run.sh n41smoke` (44 MB table, 16 targets, ABBA, one process group): harness smoke test, `smoke_n41/`. Not a result.

== Measured: wide against narrow at n = 61 (24 target rows, 3 levels)
#table(columns: 3,
  [quantity], [narrow], [wide],
  [dp_bits, entries], [14, 4096], [14, 4096],
  [precompute steps / walks / abandoned / duplicate DPs], [92,374,272 / 5,652 / 0 / 1,556], [92,374,272 / 5,652 / 0 / 1,556],
  [online steps, walks, DP misses, abandoned, recovered scalar (24 rows)], [-], [identical on 24/24 rows],
  [recovered scalar = frozen `scalars_n61.txt`], [8/8 (level 0)], [8/8 (all levels)],
  [ns per precompute step (contended host)], [151.5], [282.4],
)
Plan check: for `bytes = 5,711,008` the wide plan gives `classes_N = 1335147819528.0083`, `dp_bits = 11`, `entries = 356,938`, equal to the frozen `bl_precompute` record. The wide instrument at n = 61 reproduces the narrow instrument's counters exactly; the extra cost per step comes from the widened words and the generic wrapper (wall time, contended host, indicative only).

#figure(image("fig_n61_check.svg", width: 100%), caption: [Left: online steps per target, narrow against wide (24 points on the line). Right: ns per precompute step. Source: `plot.py` over `instrument_check_n61/*.jsonl`.])

== Harness smoke (n = 41, K = 100, 16 targets, tier A 43,689,632 B, tier B 60,000,000 B)
All 64 claim reports were produced and checked by `ecbench::claim::check_vs_rho`. With a frozen candidate manifest (n = 73 and n = 83 manifests re-hashed and accepted in a separate invocation) the only remaining problems are the honest ones: `independent_validation`, `independent_replay_pointer`, isolation level L0 below L2, record class exploratory. In the smoke run the IC A/A control failed (median IC#1/IC#2 = 0.74 on the loaded host), so the rung verdict is `UNRESOLVED_CONTROL_FAILED` by the frozen rule; the unconditional tier A reading was `BL_FASTER_AT_TIER_A` (median BL/IC 0.05 to 0.07). The smoke run says nothing about n = 73 or n = 83.

== Missing
Wide step cost on an idle host, BL precompute at n = 73/83, every IC arm at n = 73/83, independent replay, isolation receipt.
