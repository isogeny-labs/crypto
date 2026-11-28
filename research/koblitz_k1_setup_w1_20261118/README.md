# K1 set-up levers: lazy negatives + Z[τ] same-orbit plan (n23 / n29 / n37), 2026-11-18

Confirmatory engineering-lever study, frozen in `PROTOCOL.md` before data (registration commit
`1f8b5df0e0`, `registered-at.txt`). Every number is `[pcc:unverified]` until a validator signs it.

**Result: the candidate w1 (current best arm + `lazy_neg=1` + `orbit_ids=1`) passes the frozen rule on all three
curves: upper 95% bound < 1 at w = 0, 0.1 and 1, pooled and in each of the 3 replicates (12/12 cells per curve).**
The gain is small at w = 0 (0.4-1.4%, group operations of the table build) and grows with the weight because most
of it is removed point negations, which bitop/v1 prices as one field addition each.

## Primary: w1 / inc, pooled (paired by target; 95% = wider of linearised-t and cluster bootstrap)

| curve | w = 0 | w = 0.1 | w = 1 | replicates (upper ends, w = 1) | decision |
|:--|:--|:--|:--|:--|:--|
| n23 (k = 4, sibling walk) | 0.991 [0.990, 0.992] | 0.976 [0.974, 0.979] | 0.953 [0.949, 0.958] | 0.962 / 0.962 / 0.959 | pass |
| n29 (k = 1) | 0.986 [0.985, 0.987] | 0.952 [0.948, 0.955] | 0.873 [0.865, 0.880] | 0.888 / 0.888 / 0.883 | pass |
| n37 (k = 8, sibling walk) | 0.996 [0.996, 0.996] | 0.986 [0.985, 0.987] | 0.971 [0.969, 0.973] | 0.972 / 0.977 / 0.973 | pass |

Single levers (secondary, pooled): `w1-neg` / inc = 1.000 / 0.987 / 0.967 (n23), 1.000 / 0.971 / 0.904 (n29),
1.000 / 0.991 / 0.977 (n37) - an exact tie at w = 0, as declared; `w1-ids` / inc = 0.991 / 0.989 / 0.986,
0.986 / 0.981 / 0.969, 0.996 / 0.995 / 0.994.

## Controls (all hold)

- `ecbench verify --replay-all`: 9 sessions, 1,536 records each, all verified, 1,152 replays reproduced each, 0 problems.
- Every solve of every arm `verified` with `matches_target` (576 per arm per curve).
- inc-off and inc-aa identical to inc on all 432 measured records per curve (cost, phases and native maps without
  wall time, counters, outcome, `points_sha256`).
- w1, w1-neg, w1-ids try the same `targets_tried` and recover the same log as inc on 432 / 432 pairs per curve.
- Count model: 0 mismatches on every inc, w1, w1-neg and w1-ids record (1,296 each); model ratio vs measured
  w1 / inc at w = 1: 0.952 / 0.954 (n23), 0.871 / 0.873 (n29), 0.970 / 0.971 (n37).

## Deviation from the protocol's description (found in the data, disclosed)

PROTOCOL.md section 1 expected 0..2 negatives formed after the base. The records show **k + (0 or 1)**: n23 4 + 0.53,
n29 1 + 0.54, n37 8 + 0.51 per solve. The extra k come from the table build (`ic_boundary.rs`, layout table loop):
in each orbit the rotation-0 pair (P, -P) is not a sibling pair (same abscissa), so the non-sibling fallback reads the
odd slot and forms -P. Those negations are charged in `oracle_setup`, and the model check that reads them from the
record matches exactly, so the decision is unaffected; the model expectation with one hit negation was slightly
optimistic (by 0.001-0.002 at w = 1). The rotation-0 pair is P + (-P), which yields no key, so the k negations (and
that fallback step) can be skipped: an open lever, small (k field additions per solve).

## Reading

- **The w = 1 margin is mostly field additions priced as group operations.** `lazy_neg` removes k·n - k negations
  (88 / 28 / 288 per solve); bitop/v1 prices `point_negations_uncharged` as one `FieldAdd`, and the §5a band prices it
  at 1 GAE at w = 1. `orbit_ids` removes heavier work (per orbit one addition, two sibling second points, three
  canonicalisations, three inserts) and is the only lever that moves w = 0.
- **Wall time (L0, descriptive) does not move:** mean solve 298.0 -> 304.2 us (n23), 190.3 -> 181.6 us (n29),
  875.6 -> 879.7 us (n37); set-up phase 105.7 -> 105.5, 74.4 -> 73.1, 441.1 -> 436.2 us. The counted saving is real
  but below the host's noise at this size.
- **Gap to rho-v5 (not a gate):** w1 / rho-v5 pooled = 1.030 / 1.355 / 2.763 (n23; band: rho-v5 < w1 at all three
  weights), 0.981 / 1.125 / 1.823 (n29, unresolved inside the band), 0.901 / 1.215 / 2.614 (n37, unresolved; w = 0
  upper 0.984 < 1). Band S (pooled) at w = 0.1: inc -> w1 0.331 -> 0.323 (n23), 0.458 -> 0.436 (n29),
  0.203 -> 0.200 (n37); rho-v5 0.239 / 0.387 / 0.165.

## Files

`PROTOCOL.md`, `REGISTRATION.sha256`, `registered-at.txt`; `make_specs.py`, `specs/`, `run.sh`; `analyze.py`,
`summarize.py`, `model.py`, `model_check.py`; `results.json.gz`, `summary.json`; `tables/` (band per session and
pooled, `model_check.json`, per-record CSV); `audits/` (verify receipts); `logs/` (binary hash, run times);
`figures/`, `REPORT.pdf` (`make_report.py`, written after data). Binary: ecbench from f54326cee1 (dd2fae0bab plus the
one-line compile fix), sha256 a6a38904...a2563a34. Raw sessions: scratchpad `w1-sessions/` (not in git; SHA-256 of
each `records.jsonl` in `summary.json`).
