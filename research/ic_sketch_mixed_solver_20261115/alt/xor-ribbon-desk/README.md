# Lane xor-ribbon-desk: static XOR / ribbon retrieval structure for the N83 root index (desk model)

All figures [pcc:unverified]. Desk model only: no Rust, no timing run, no ecbench. Every number
is recomputed by `model.py` from committed files (`research/n83_*/measurements.json.gz`, 100 rows
with table/state/Bloom allocations, peak RSS, probes, setup, rank, online; plus two RESULT.md
files for matched rho). Raw output: `raw_output.txt`, `results.json`.

```
python3 research/ic_sketch_mixed_solver_20261115/alt/xor-ribbon-desk/model.py --json results.json > raw_output.txt
```

## Question
If the N83 root table (root4 or root11) were a static XOR filter (about 1.23 n f bits) or a ribbon
filter (1.005 to 1.10 n f bits) over f-bit values/fingerprints plus exact verification, how much
larger K fits in the same RSS guard, and what do the repository's K-versus-cost rows predict for
online and cold one-target cost and for the gap to matched rho?

## Does the repo already sit near this bound? (section 1 of raw_output.txt)
- root11 (5-byte slots, dense load): 41.06 bits/entry at K3550 against a 40-bit slot payload, i.e.
  1.027x of payload. A ribbon at 1.005-1.10x of the same 40 bits is 40.2-44.0 bits, an XOR filter
  49.2 bits. So root11 at K>=3450 (1.087x and below) is already at or better than XOR and inside the
  ribbon band. No table gain there. At K3300 it is 1.188x (load is lower), still inside XOR's 1.23x.
- root4 (4-byte slots, 2^31 slots): 64.03 bits/entry at K3596 for a 32-bit payload = 2.00x. A ribbon
  at 32.2-35.2 bits or XOR at 39.4 bits would free 3.7 to 4.1 GB (ribbon 1.05: 8.59 GB -> 4.51 GB;
  XOR 1.23: 5.28 GB). The headroom is the power-of-two slot count at 50% load, not the fingerprint.
- The state array (11 B per state, 5.90 GB at K3596) is the other big block and no filter shrinks it.

## Memory model (sections 2 and 4)
peak_rss = 11 S(K) + table + Bloom + residual, S(K)=83K(K-1)/2+42K, entries = 1.99995 S. The residual
is the max over 32 measured root4 rows, 0.499 GB (min 0.286). Check: the model gives 14.963 GiB for
measured K3596 root4 (14.791 GiB measured, +1.2%, conservative) and its largest K inside the 15.25 GiB
guard is exactly 3596, the measured frontier. Modelled layouts use f = ceil(log2(2S+2)) value bits
(31 here), optionally +8 fingerprint bits (no Bloom). Largest K in the same guard (modelled):

| scenario | Kmax @15.25 GiB | Kmax @14 GiB |
|---|---|---|
| base root4 (measured) | 3596 | 3266 |
| XOR 1.23, id only, Bloom kept | 4167 | 3974 |
| XOR 1.23, id+8 fp, no Bloom | 4078 | 3902 |
| ribbon 1.10, id only, Bloom kept | 4274 | 4075 |
| ribbon 1.05, id only, Bloom kept | 4317 | 4116 |
| ribbon 1.05, id+8 fp, no Bloom | 4244 | 4060 |
| ribbon 1.005, id only, Bloom kept | 4356 | 4154 |
| ribbon 1.005, id+8 fp, no Bloom | 4288 | 4103 |

So +480 to +760 columns (13% to 21% larger K) than the measured root4 frontier. Against root11
(K3550 at 12.7 GiB, table already dense) the gain is smaller and was not modelled separately.

## What the repo's own K-law predicts (sections 3 and 5)
- probes per target ~ K^-1.985 (100 rows, R^2 0.965, residual sd 0.025 ln). Only measured to K3596;
  beyond is extrapolation.
- Anchor: same-host six-arm frontier run, K3596 root4: online 25.951 ms, setup 615.9 s, rank 47.4 s.
  Online is scaled by the probe law; setup linearly in S (K1107 row is sub-linear, so this is
  pessimistic for setup); rank by K^1.56 (3 points).
- Matched rho (wall clock, same public points): 2,334.964 ms geometric mean over 16 points
  (`n83_cold_base_confirm_20261021`), 1,908.419 ms mean over 5 points (`n83_tier1_combined_results_20261015`).

| row | K | online ms (kappa 1 / 1.5 / 2) | cold s | cold / rho | targets to beat rho |
|---|---|---|---|---|---|
| measured K3596 root4 | 3596 | 25.95 | 663 | 284x | 287 |
| XOR 1.23, id, Bloom kept | 4167 | 19.37 / 29.05 / 38.74 | 887 | 380x | 383 |
| ribbon 1.05, id, Bloom kept | 4317 | 18.06 / 27.08 / 36.11 | 951 | 407x | 410 |
| ribbon 1.005, id+8 fp | 4288 | 18.30 / 27.45 / 36.60 | 938 | 402x | 405 |

kappa is the per-probe cost multiplier of the new structure against the Bloom+table probe; kappa=1 is
optimistic and unmeasured. The best case lowers the amortised online time by about 25-30% (25.95 -> 18.1 ms)
only if kappa<=1.34 (XOR) to 1.44 (ribbon 1.05): at kappa 1.5 the online time is worse than the measured baseline for every row.
The cold one-target cost goes UP by 28% to 43% because setup grows with S(K). The gap to matched rho
(cold, one target) widens from 284x to 364-407x. The online-only gap (rho / IC online) at best
moves from 90x to about 129x, with kappa=1. A setup-exponent 0.8 sensitivity gives cold 879 s (376x) for
the ribbon 1.005 + fp row, still worse than the measured 663 s.

## False-positive verification cost (section 6)
- Baseline Bloom (k=3, 8.59e9 bits, K3596): FPR 3.06% if it holds all 1.07e9 roots, 0.50% if one key per
  state. That is 240-1,471 false positives per target; even charged at a full probe (0.555 us) that is
  0.13-0.82 ms, 0.5-3.1% of the 25.95 ms online time. XOR8 or an 8-bit ribbon fingerprint (2^-8) gives 188
  per target (0.10 ms), 12 at 12 bits. A better filter is worth at most about 3% of online time at constant K.
- Retrieval value with no fingerprint and no prefilter: every probe needs a dependent state read
  (48,160 per target). At an assumed 0.1 us each that is 4.8 ms (18.6% of the anchor online time); the
  anchor's whole per-probe cost is 0.555 us, so this is the kappa that decides the lever. Unmeasured.

## Caveats that can kill the lever
- Construction memory is outside the RSS model: an unsharded ribbon build holds about 12 B per row
  (13.5 GB at K3596, 17.9 GB at K4317) and an XOR peel 9 B per key x 1.23 (11.9 / 15.7 GB), above the guard.
  Sharding or streaming is required and is unmeasured (the sibling lane xor-ribbon-build covers builds).
- A retrieval structure needs distinct keys; the current table keeps the first witness for duplicates,
  so a dedupe pass (more memory/time) is needed to preserve the first-witness rule.
- Setup (base + index) would not shrink; a static build adds to it.
- probes ~ K^-1.985 and linear setup are extrapolated past K3596; no N83 row above K3596 has a measured
  online time in this repository (K4300/K4405/K5500 have bases and capacity receipts only).

## Verdict
- Reaches counted GAE at w=0: no. A larger K needs 166K usable base points (more group operations),
  so counted setup rises. It is a memory lever that moves only native lookups (w>0) and RSS.
- Beats the baseline: only in amortised online time and only under kappa<=~1.3-1.4; it does not beat the
  baseline on cold one-target cost (worse by 28-43%) and it widens the cold gap to matched rho.
- Honest net: root11 already sits at 1.03x of its payload at dense K; root4 is 2.0x over the bound, so the
  only real saving is replacing root4's 50%-load pow2 table (about 4 GB). Prior evidence that filter
  layout changes give no gain: research/ic_tool_program/explorations/blocked-filter-20261112 (a different
  IC path, same conclusion). No existing N83 file mentions a XOR/ribbon structure [pcc:unverified].
