# Amendment 2: exact equal-size interval control

Frozen before any held-out cold target arm. This supplements the original
`PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md` and Amendment 1. Their files,
source pins, workload manifest, preflight receipts, and stores are retained.
The v1 and v2 cold stores contain headers and busy-host checks only; neither
launched a cold target.

The deterministic selected-map search matches the number of carriers in
`x ∈ [0,32)`. The originally specified `prime-abscissa:below=32` instead
builds carriers in `[1,32)`. Its F4 oracle used `bits=5,lo=0`, so its stated
factor-base interval and oracle interval also differed. The native
`prime-interval:t=5,lo=0` builder and its F4 oracle both use exactly
`[0,32)`, with all 32 Legendre tests and successful square roots charged.
Use that control for both interleaved interval arms in v3; keep all candidate,
MITM, Kummer, target, seed, cap, and promotion definitions unchanged.

Independent computation from the frozen public curve equations gives:

| Rung | Curve | `[0,32)` carriers | `[1,32)` carriers |
|---:|---:|---:|---:|
| 11 | 0 | 17 | 16 |
| 11 | 1 | 16 | 15 |
| 11 | 2 | 17 | 17 |
| 14 | 0 | 14 | 14 |
| 14 | 1 | 20 | 19 |
| 14 | 2 | 17 | 16 |

The first development F4 diagnostic attempted six interval/candidate arms
with the old interval builder. Its raw receipts remain at
`/Volumes/SSD990-2/pkm-z2xz8-dev-f4-parity-v2-20261128/receipts.jsonl`
(SHA-256 `a9d9b67872f988158ac9d5eaaf4fbdf284240442e638f04a5f8c728ab2b12d2c`),
and its result is `RESULT_Z2XZ8_DEV_F4_MISMATCH_20261128.json`
(SHA-256 `44bab1ada0dd74d4831513513ad0989400b21be18368fbbae6fb4a9498b768f9`).
It is rejected as an equal-size comparison. An earlier attempted interval
process killed for host contention is preserved in
`/Volumes/SSD990-2/pkm-z2xz8-dev-f4-parity-20261128/receipts.jsonl`
(SHA-256 `a86409ff51166d57476e07f70c17987b7f81b637dcb2d17a4f994a03c8d3936e`).

The corrected 64-trial development diagnostic on three existing development
curves is in `RESULT_Z2XZ8_DEV_F4_EQUAL_SIZE_20261128.json`
(SHA-256 `4437722a94f0a4954175107e0eebef5027248dc19a09f9be72788fc913e6872c`).
All three pairs have equal abscissa counts (17, 17, 20). Candidate versus
interval relation counts were 8/6, 7/6, and 10/7; candidate F4 elimination
operations were about 254 million versus about 8.1 billion for each interval
arm. Every arm stopped at 64 trials with incomplete rank and positive
uncharged native work, on a contended host. These are development signals,
not complete cold costs or a scaling result.

Run the six held-out arms under `run_z2xz8_holdout_cold_v3.py` into a new
store `/Volumes/SSD990-2/pkm-z2xz8-holdout-cold-parity-interval-20261128`.
Pin both amendments and the original protocol in its header. Continue to
enforce source, binary, manifest, preflight, quiet-host, process-cap and
full-accounting checks. Keep failed and censored cases in the denominator.
