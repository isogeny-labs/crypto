# Local scalar multiplication timings

Use the new scalar-timing subcommand of the original ecbench CLI against its
unchanged qualified release-v4 crypto library. Timing adapter only; no new
arithmetic or ECDLP method. Three focused native controls pass and the pilot
plan remains byte-identical. Build argv and rlib/source hashes are retained.
[pcc:unverified]

The full frozen source/image/u2 coordinate models on202/96bit fields provide
32 target points each; six measured rounds plus one warmup. Deduplicate the
three solver arms only for schedule events: no solver is invoked. Use public
scalar1+paired_algorithm_seed%(r-1), explicitly not claimed uniform. This
changes the multiplier across rounds while sharing it across endpoint models.
Each timed sample is64 counted multiplications of the varied workload target
base Q, with black_box input/output; no precomputation is shared or charged
away. Independent expected [k*planted modr]G and result checks occur outside
the interval, with their own recorded counters. [pcc:unverified]

Raw JSONL samples retain elapsed nanoseconds, both operation ledgers, warmups,
scalar, result and exact workload/plan identities. Shared benchmark lock;
300second outer watchdog,12GiB address limit, no concurrent builds/tests.
All clocks are local descriptive L0 and never admitted wall evidence.
Subsequent independent PARI verification checks every [k]Q result. No generic
ECDLP speed or conductor effect is inferred from scalar timing. [pcc:unverified]
