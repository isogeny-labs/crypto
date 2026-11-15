# Slice S4 results (track T2, lead 1): the full-base line sieve at genus 3 and 4

**Protocol:** PLAN.md §10, frozen at `44e2e36c57` before any S4 code. Its rows and bars were chosen
using the line-sieve probe (`SIEVE-PROBE-20261114.md`), as §10 says.

**Code:** `af2f5c83a6` (feature `count-ops`, phase accounting, `examples/s4_sieve.rs`).
**Scripts:** `f9517dfb1e` (`run_s4.sh`, `summarise_s4.py`).

**Binaries:** built from a `git archive` of `af2f5c83a6`, not from the worktree, into
`hec-s4-count` (`--features count-ops`) and `hec-s4-time`; their sha256 are in
`s4-20261115/binaries.sha256`. The record's `commit` field shows a later HEAD, because unrelated
probe commits landed while the run was going; the binaries are the archive's.

**Record:** `benchmarks/records/20261115T0626Z-hec-s4-line-sieve.json`.
**Raw output:** `s4-20261115/` (`count.txt`, `time.txt`, `host.txt` with load, `summary.md`,
`summary.json`, and the crate tests at `af2f5c83a6` in `tests-at-af2f5c83a6.txt`).
**Run report:** `reports/run-20261115T0626Z/`.

## Verdicts

All the pre-registered expectations hold.

- **E12 (exactness): holds.**
  - G10: every sieve relation, in all 55 count-build sieve runs, was checked in the Jacobian and held.
    All 110 `k` (both arms) are the planted one and verify.
  - G11: the count and timing builds give identical lines for all 110 (arm, row, seed), apart from
    the fields §10 excludes.
  - The 14 crate tests pass in release, release with `count-ops`, and debug. [pcc:unverified]
- **E13 (P1 ≥ 5 on the 9 bar rows): holds.** P1 = 5.8–16.9 at genus 3 and 8.2–16.1 at genus 4
  (p ≥ 127). The tightest row is (3, 61) at 5.8.
- **E14 (P2 ≥ 10): holds.** P2 = 14–1,168.
- **E15 (P3 ≥ 2 on the 8 bar rows with p ≤ 1021): holds.** P3 = 5.5–10.9.
- **E16 (at (3, 4001) the sieve's solve does more mulmods than the walk's): holds.** 8.48M against
  2.02M; P3 there is 0.79, so the full-base sieve loses end to end at m ≈ 2000 in counted products,
  as the probe found on the clock.

| g | p | m | walk / sieve products per relation | **P1** | **P2** | **P3** | solve mulmods walk / sieve |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | 61 | 33 | 1,511 / 259 | **5.8** | 14 | 6.17 | 7.9k / 0.7k |
| 3 | 211 | 104 | 1,735 / 205 | **8.5** | 50 | 9.77 | 83k / 5.4k |
| 3 | 401 | 196 | 1,928 / 175 | **11.0** | 107 | 9.73 | 128k / 16k |
| 3 | 1021 | 506 | 2,476 / 159 | **15.5** | 259 | 5.50 | 269k / 194k |
| 3 | 4001 | 1997 | 2,452 / 145 | **16.9** | 1,168 | 0.79 | 2.02M / 8.48M |
| 4 | 31 | 16 | 6,892 / 5,091 | 1.4 | 2 | 1.37 | (no bar) |
| 4 | 61 | 36 | 6,881 / 1,352 | 5.1 | 13 | 5.19 | (no bar) |
| 4 | 127 | 66 | 10,619 / 1,292 | **8.2** | 27 | 8.56 | 71k / 4.3k |
| 4 | 251 | 133 | 9,828 / 903 | **10.9** | 67 | 10.07 | 124k / 19k |
| 4 | 509 | 234 | 15,992 / 1,199 | **13.3** | 125 | 10.87 | 169k / 74k |
| 4 | 1009 | 488 | 16,432 / 1,020 | **16.1** | 208 | 8.37 | 480k / 504k |

Medians over 5 seeds (`20261115…19`); P1–P3 are ratios of per-arm medians. `summary.md` has the
min–max, relations per line against `p/(g+1)!`, line additions per relation, walk trials per
relation and products per trial, the excluded verification products, and timing-build wall clock.

## Reading

- **The relation phase.** Counted in one unit across both arms, the full-base line sieve needs
  5.8–16.9× fewer field products per relation than the walk at genus 3, and 8.2–16.1× fewer at
  genus 4 (p ≥ 127). The gap widens with p, because the walk's oracle cost grows with `log p` and the
  sieve's cost per relation falls slowly as lines yield more.
- **The per-line addition** is part of the sieve's count. At (4, 127) it is about one genus-4 Cantor
  addition per relation, and P1 is 8.2, inside §10's 6–14 estimate. At (4, 31) it dominates and P1 is
  1.4; that row has no bar.
- **The solve.** At m ≲ 500 the sieve's rows have no dense step block, and its solve is cheaper
  than the walk's. At m ≈ 2000 its weight-(g+1) rows fill in under `Solver::Fast`, and the solve
  outweighs the relation phase: P3 = 0.79 at (3, 4001). A solver for sieve rows (follow-up B) is what
  would change that.
- **The clock** (timing build, load average 92–174) agrees in direction: total ms walk / sieve is
  2.6–4.0× on the genus-3 rows up to p = 1021, 3.0–5.4× at genus 4 for p ≥ 127, and 0.77 at
  (3, 4001). Descriptive only.

## Class and limits

- Engineering on the relation phase of the full-base native engine. Not an advance, no `S` claim,
  not validated.
- The counted unit is the field product. Additions, memory traffic, bucket resets and allocation
  are not counted.
- The walk is the current native walk, without the Tier-2 levers of `LEADS-20261114.md`.
- The genus-3/4 exponent regime (reduced base, large primes) is not this slice. The probe
  `LP-SIEVE-PROBE-20261115.md` addresses it.
- §15 external comparison: not run. No open hyperelliptic-IC implementation is known.
