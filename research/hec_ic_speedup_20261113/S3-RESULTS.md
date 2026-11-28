# Slice S3 results (track T2, lever L1): exact inversion hygiene

**Protocol:** PLAN.md §9, frozen at `10e4dde42f` before any S3 code. That commit is also the `old`
build.

**Code:** `451cb16235`. **Scripts:** `91e5a13ef1` (`run_s3.sh`, `summarise_s3.py`).

**Record:** `benchmarks/records/20261115T0426Z-hec-s3-inversion-hygiene.json`. It holds two runs of
`run_s3.sh` on 2026-11-14 (local time): the first set and the pre-registered rerun.

**Raw output:**

- `s3-20261114/`: the first set;
- `s3-20261114-rerun/`: the rerun.

Each holds the stream lines (`stream<i>-{A,B,A2}.txt`), the end-to-end lines
(`e2e<i>-{A,B,A2}.txt`), the `count-inv` lines (`count-{old,new}.txt`), `binaries.sha256`,
`host.txt` (load before every process) and `summary.jsonl`.

**Run report:** `reports/run-20261115T0426Z/`.

## Change

1. `Fp::inv` runs the extended Euclid on `u64` remainders with `i64` cofactors instead of `i128`.
2. `Poly::divrem` skips the inversion, and the multiplication by 1, when the divisor is monic. That
   covers every `mulmod` in the oracle's `x^p mod u` and most divisions in the walk.
3. The degree-2 oracle and the EDF take `1/2 = (p + 1)/2`.

Nothing else changes: not the walk, the decision procedure, the trial sequence or the solve.

## Verdicts

**E8 (exactness): holds.**

- G7: `fp::tests::inv_matches_reference` and `poly::tests::divrem_matches_reference` compare the new
  routines with the pre-S3 ones, kept in the tests:
  - the inverse on every row prime and on primes near 2^31, 2^32, 2^61 and 2^62, 20,000 random
    arguments each plus edge cases;
  - division on 24,000 random pairs over six primes, more than 5,000 of them with monic divisors.
- G8: every `t2_stream` process, old and new, timed builds and `count-inv` builds, gives the same
  `walk_hash`, `oracle_hash`, `smooth`, `frobenius` and `parity_rejects` on all 13 rows as the old
  `count-inv` build. That is 22 processes per set.
- G9: every `hec_native_bench` process (`HEC_SOLVER=dense`, 9 per set) gives the same 55 JSON lines
  apart from the `*_ms` fields, and every `k` verifies.
- All 13 crate tests pass in release and in debug. [pcc:unverified]

**E9 (counts): falsified as registered**, on two genus-4 rows:

| g | p | `Fp::inv` per walk step, old → new | per oracle call, old → new |
|--:|--:|--:|--:|
| 2 | 101–4001 | 6.47–6.52 → 2.98–3.01 | 0.99–1.00 → 0 |
| 3 | 61 | 7.90 → 3.41 | 10.37 → 3.61 |
| 3 | 211 | 7.99 → 3.49 | 11.65 → 3.78 |
| 3 | 401 | 7.97 → 3.48 | 11.48 → 3.79 |
| 3 | 4001 | 8.01 → 3.51 | 16.22 → 3.80 |
| 4 | 31 | 8.83 → 4.26 | 9.38 → 4.92 |
| 4 | 61 | 8.87 → 4.34 | 10.67 → **5.07** |
| 4 | 1009 | 8.99 → 4.49 | 13.44 → **5.17** |

The bar was ≤ 5 per oracle call at genus 3 and 4. Genus 4 at p = 61 and p = 1009 miss it. The other
parts hold: 0 at genus 2, and walk inversions fall on every row, by 2.0–2.2×. Where the remaining
oracle inversions occur was not counted per call site. By reading the code, the likely sources are
the discriminant's generic resultant, whose Euclidean remainders are not monic, and the
equal-degree splitting step (EDF) on smooth candidates. Both are covered by lever (c) and by the
closed-form discriminant lead.

**E10 (per-trial cost, genus 3 and 4, ≥ 1.25×): not determined.** **E11 (genus 2, ≥ 1.10×): not
determined.** The A/A floor failed in both sets, so under §9 neither timing verdict can be given.

- The validity rule was a per-row A/A median in [0.95, 1.05]. 11 of 13 rows failed it in the first
  set, and 10 of 13 in the pre-registered rerun.
- Load average across the processes was 52–401 in the first set and 50–153 in the rerun
  (`host.txt`).

The medians are reported as measured, but they carry no verdict. The A/A medians themselves range
from 0.65 to 1.49, which is as wide as the effect the bars asked about.

| g | p | m | first set: ratio (min–max) | first set: A/A | rerun: ratio (min–max) | rerun: A/A | end to end, first / rerun |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 101 | 59 | 1.29 (0.32–2.15) | **1.13** | 0.88 (0.45–1.73) | **1.22** | 1.22 / 1.11 |
| 2 | 251 | 112 | 1.04 (0.28–1.80) | **0.78** | 1.33 (0.62–8.22) | **0.65** | 1.07 / 1.28 |
| 2 | 503 | 261 | 1.26 (0.72–1.72) | **0.93** | 1.07 (0.53–1.67) | **1.14** | 0.42 / 1.10 |
| 2 | 1009 | 507 | 1.25 (0.56–1.81) | **1.49** | 0.82 (0.75–1.76) | **1.46** | 1.47 / 1.04 |
| 2 | 2003 | 1003 | 1.34 (0.79–1.86) | **0.86** | 0.79 (0.57–1.58) | **0.82** | 4.23 / 0.49 |
| 2 | 4001 | 2066 | 1.32 (0.64–2.60) | **1.15** | 1.04 (0.63–1.86) | **0.86** | 3.94 / 1.29 |
| 3 | 61 | 35 | 1.51 (1.01–2.48) | **0.70** | 1.13 (0.77–1.31) | 1.03 | 1.18 / 1.20 |
| 3 | 211 | 106 | 1.46 (0.91–4.24) | 0.99 | 0.91 (0.76–1.40) | 0.97 | 0.89 / 1.18 |
| 3 | 401 | 193 | 1.34 (1.11–3.38) | **0.81** | 1.12 (0.60–4.16) | **1.18** | 1.18 / 1.39 |
| 4 | 31 | 18 | 1.74 (0.76–2.60) | **0.80** | 1.32 (0.71–2.45) | 0.95 | 1.20 / 1.19 |
| 4 | 61 | 35 | 1.58 (0.82–1.91) | **0.86** | 1.45 (1.15–2.91) | **1.09** | 1.21 / 1.22 |
| 3 | 4001 | 2029 | 1.34 (0.89–3.15) | 1.05 | 1.34 (1.02–1.97) | **1.45** | – / – |
| 4 | 1009 | 463 | 1.17 (0.81–2.66) | **0.89** | 1.35 (1.01–1.85) | **1.10** | – / – |

How to read the table:

- "ratio" is the primary statistic: the median over 7 repetitions of per-trial time
  old(mean of A, A′) / new.
- A bold A/A value is outside [0.95, 1.05].
- `m` is the stream curve's base size. `t2_stream` takes the smallest valid `c`, not round eight's
  instance, so `m` differs slightly from S1–S2b.
- "End to end" is the median over 15 (repetition, seed) pairs of `hec_native_bench` `total_ms`,
  old/new. It is descriptive only, and it has no A/A floor of its own.

**Class.** Engineering on the relation phase:

- Trials, relations, group operations and the solve are unchanged, as G9 shows.
- Field inversions fall, as counted under E9.
- No wall-clock gain is established.
- The native engine computes no `S`. No `S` claim, no advance.

## What this does and does not show

- **Shown, deterministically:** S3 does the same arithmetic with roughly half the field inversions
  in the walk, and a third or less in the genus-3/4 oracle. Inversions were measured at 35–90 ns,
  against about 1.2 ns for a multiplication. The code hunt's per-call-site count after S3 puts the
  remaining inversions at 3.0, 7.0–7.3 and 9.4–9.7 per trial at genus 2, 3 and 4. The cost now sits
  in multiplications: the oracle's exponentiation and the `+D` Cantor additions.
- **Not shown:** that this is faster on this host. The A/A spread of 0.65–1.49 is larger than the
  effect. The first-set medians lean above 1 on all 13 rows, but the rerun's are below 1 on
  4 rows, three of them at genus 2. That is noise this design cannot separate.
- **Stop rule (§5.5).** E10 is not falsified, so the stop rule is not triggered. It is not
  confirmed either. Lever L1 stays in the code, because it is exact and removes work, not because a
  timing verdict supports it. [pcc:unverified]

## Protocol deviations

1. **The first rerun attempt was stopped after about 1 minute** and restarted under
   `tools/benchrec.py run`, so that the record lists both runs' commands. The stopped attempt's
   partial output was deleted unread. The restarted rerun is the one reported.
2. **The working tree was dirty during both runs** (the record says `commit_dirty: true`). The only
   change was the tracked, locally modified `.resume/RESUME.md`. No code or script differed from
   `91e5a13ef1`.

## Not done

- **A quiet-host timing.** This host has not been quiet this session. A labelled re-measurement on a
  reserved CPU (the `isolated_bench` hosts used by earlier rounds) would settle E10 and E11. It
  would be a new run under the same §9 protocol.
- **External comparison (§15):** not run. No open hyperelliptic-IC implementation is known.
- **Validation:** no `VALIDATION.json`.
- **Next steps:** `LEADS-20261114.md` ranks the next levers. The line-sieve feasibility probe comes
  first.
