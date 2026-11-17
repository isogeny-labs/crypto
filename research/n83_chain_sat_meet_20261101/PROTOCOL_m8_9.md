# Amendment: m = 8, 9 (preregistration, frozen before its run)

Added 2026-11-01 at the user's request ("add m=8 and m=9 after this run"),
after the m = 5…7 run started and before any m = 8, 9 data. Everything in
`PROTOCOL.md` holds except what is stated here.

## Why the scope changed

`PROTOCOL.md` excluded m = 8, 9 on a declared cost of ≈ 5.4·10^8 additions per
query. That count assumed 334 signed base points; the 167-point union base is
already closed under negation, so the lookup enumerates multisets of 167
points (warmup lines of the m = 5…7 run: 347,688 depth-3 fingerprints in
0.68 s, 12,976,303 depth-4 in 26.1 s). Corrected counts: C(170, 4) ≈ 3.4·10^7
additions per query for a back depth of 4; C(171, 5) ≈ 1.15·10^9 for the
depth-5 front set, built once.

## Arms

| id | arm | m = 8 | m = 9 |
|---|---|---|---|
| A2 | `all-summands+t2.2` (control) | ✓ | ✓ |
| A4 | `all-summands+t2.2+meet<d>` | d = 4 | d = 5 |

## Run

- Same seed, targets (3 public, 2 planted per m), caps (300,000 conflicts,
  120 s) and bench as `PROTOCOL.md`; binary rebuilt at the amendment commit
  (the only code change since the m = 5…7 binary reserves the depth-d
  fingerprint vector at its exact multiset count).
- `run_shard.sh <bin> raw/m8 8 4` and `run_shard.sh <bin> raw/m9 9 5` in
  parallel after the m = 5…7 run finished, one core each.
- Decision rule unchanged (per m, no pooling; a refuted planted target voids
  the arm at that m).
