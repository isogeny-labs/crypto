# PS-3 bit-sliced group-law oracle

Visual copies from the source study's `run-20261018T1627Z` report. The raw host-separated measurements, verification records, benchmark programs, and interpretation live in [`research/pdp_oracle_speedup_20261011/ps3_bitsliced`](../../research/pdp_oracle_speedup_20261011/ps3_bitsliced/).

The reported baseline/arm per-target ratios are 23.8×/60.6×/62.5× on host A and 17.5×/41.5×/50.3× on host B for 64/256/512 lanes at `l = 11`. Both hosts have exact root-set and verdict-digest checks. The study records the 4-thread isolation requirement as blocked and makes no end-to-end or ECC2K-130 claim.
