# L4-A timing pilot (not a result)

Binary `semaev_lattice_reach-pilot` (same lattice code as v1), 40-bit curve,
seed 7, one planted and one unplanted trial per configuration, run while
other benchmark jobs shared the host.

| m | ℓ | dim | outcome |
|--:|--:|--:|:--|
| 3 | 1 | 35 | 6–31 ms per reduction (smoke runs and the measured cells) |
| 3 | 2 | 165 | 116 s per reduction ([`pilot-m3-l2.jsonl`](pilot-m3-l2.jsonl)); 45–280 s in the measured cells under load |
| 3 | 3 | 455 | no reduction finished in 5 h 10 min (21:42 to 02:52); stopped by hand |
| 4 | 1 | 495 | not started (queued behind ℓ = 3) |

These timings are why the protocol measures only m = 3 at ℓ = 1 and 2. Main's
`lattice::lll_reduce` is a textbook LLL with BigInt Gram–Schmidt; a
floating-point or blocked reduction (fplll-class) would move the cap, but that
is a library change outside this study.
