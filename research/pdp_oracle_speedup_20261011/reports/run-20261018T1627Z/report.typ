#set page(margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[PS-3 run report, host B: bit-sliced group-law kernel, threads, counts] \
  #text(size: 10pt)[PDP oracle speedup program, `research/pdp_oracle_speedup_20261011/ps3_bitsliced/raw_hostB/` --- 2026-10-18, 16:27--17:55 UTC]
]

= What ran

`ps3_bitsliced/run_ps3.sh` with `PS3_RAW=raw_hostB PS3_REUSE_BINARY=1` (the same binary, SHA-256 `c40434d6...a6c1`), then `run_counts.sh`. Host B: Intel Xeon 2.10 GHz KVM guest, 4 vCPU, AVX-512 including IFMA, VBMI, BF16, FP16, VPCLMULQDQ, GFNI, kernel `6.18.44-fc-v139` (`manifest_hostB.json`); the container came back on this host after a stop, so these rows are never pooled with host A. Timed runs through `tools/isolated_bench.py run`. Arms as on host A.

*Runs requested and completed.* 180 interleaved rows requested, 180 completed, 0 contended; 30 thread rows (`bs512` on 1, 2, 3 reserved cores, `l = 11, 12`, five rounds) completed; 5 + 5 non-isolated 4-thread rows completed. *Not completed:* the planned 4-thread _isolated_ run: `isolated_bench` refused it 200 times with "reserving every CPU leaves nowhere to move other work; leave at least one free" (`raw_hostB/refused_starts.log`), and the script stopped on the 200th refusal; the stage was re-run with the widest admissible reservation (3 cores) plus the labelled non-isolated 4-thread diagnostic. Callgrind: 15 runs requested at $l = 9, 10, 11$, 12 ran; the 3 `bs512` runs exit 252 (valgrind 3.22 has no AVX-512).

= Observations

#figure(
  image("ns_per_target.svg", width: 92%),
  caption: [Wall nanoseconds per refuted target by arm and rung (source `raw_hostB/times.jsonl`; black bars are per-arm medians; log scale; no run was marked contended).],
)

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto),
  align: (right, right, right, right, right, right, right),
  table.header[*l*][*baseline ms*][*group-scalar*][*bs64*][*bs256*][*bs512*][*rounds*],
  [8], [8.96], [0.249], [0.705], [0.688], [0.690], [5],
  [9], [37.19], [0.828], [2.584], [1.478], [1.584], [5],
  [10], [159.53], [2.615], [9.395], [4.170], [3.514], [5],
  [11], [687.65], [12.156], [39.036], [17.242], [13.712], [5],
  [12], [2979.92], [46.021], [175.385], [72.145], [47.463], [5],
  [13], [12426.10], [178.175], [784.152], [304.325], [208.012], [5],
)

Medians over five rounds, milliseconds per refuted target. A/A run-to-run spread: 6.15%, 9.76%, 6.34%, 4.48%, 6.14%, 4.80% at $l = 8..13$.

#figure(
  image("ns_per_pair.svg", width: 92%),
  caption: [Wall nanoseconds per own pair on host B.],
)

At $l = 11$, baseline against each arm, per target (B/A medians, reciprocals derived): group-scalar 0.0177 (56.5x), bs64 0.0570 (17.5x), bs256 0.0241 (41.5x), bs512 0.0199 (50.3x). Against the scalar group kernel, per own pair, $l = 11$: bs512 1.117 (min..max 0.957..1.393), bs256 1.438, bs64 3.308 (the scalar arm's A/A spread is 15.34%).

#figure(
  image("threads.svg", width: 88%),
  caption: [`bs512` ms per refuted target on 1, 2, 3 reserved cores (isolated) and on 4 threads without isolation (hollow, diagnostic), five rounds, source `raw_hostB/times.jsonl` and `times_4thread_not_isolated.jsonl`.],
)

Paired-round speed-up against one core (medians of `1c/Nc`): $l = 11$: 2.21 (2 cores), 3.22 (3 cores); $l = 12$: 2.02, 2.90. Single-core spread 14.33% and 16.14%. The non-isolated 4-thread medians: 3.29 ms ($l = 11$), 12.22 ms ($l = 12$), i.e. 3.92x and 3.64x the 1-core median.

Instructions retired per own pair (`callgrind`, $l = 11$): baseline 3,007; group-scalar 278; bs64 814; bs256 440; `bs512` did not run. These counts include every iteration of `rep` string instructions as instructions; the bit-sliced columns are not a time proxy.

Verdict digests equal PS-0's for all six configurations on all five rungs (30 records) and for the six $l = 13$ sample records, 0 mismatches. Recorded PS-2 program hashes reproduce.

= Missing or not done

The 4-thread isolated measurement (blocked, above); `bs512` instruction counts; the executed kernel's own counted bit operations (only PS-2's derived $G$). The `l = 13` sample has three targets. Wall figures are exploratory in the repository's isolation vocabulary. Nothing end to end; nothing for ECC2K-130.
