# Errata: research/quartic_diem_la_20261123

Source: independent validator report `validation/validator-report.md` (validated commit 23384701f18; disposition **validated with errata**; gate checker `valid`). The pinned files (PROTOCOL.md, analyze.py, run_*.sh, raw/, analysis.json) stay byte-equal. These corrections are recorded here instead. No number in `analysis.json` and no verdict changes. Every number is [pcc:unverified].

- **E1, `PROTOCOL.md:1`.** The title still reads "(DRAFT, NOT FROZEN)". Line 3 is correct: the protocol was frozen in 612f8dee20e before any grid-prime run.
- **E2, `analysis.json` decision text (from `analyze.py:242`).**
  - The text says "IMPROVEMENT (ranked below 2lp-sfb across the band)" [pcc:unverified]. That is not a §5a ranking.
  - At w = 0 the pooled ratio is 1.0028 [1.0010, 1.0048], entirely above 1. The order therefore reverses inside the band; the pooled ratio falls below 1 only for w > about 1.6e-7.
  - The decision is IMPROVEMENT [pcc:unverified] only because the frozen rule allows a w = 0 ratio up to 1.01. Any row must cite it as "IMPROVEMENT under the preregistered w = 0 tolerance of 1.01", not as a ranking that holds across the band.
- **E3, `RESULTS.md:32`.** "about one failed `[t]G = T` check of ~log₂ r operations" should read "~1.5 log₂ r". The 61–77 extra group operations are 1.37–1.55 × log₂ r.
- **E4, `RESULTS.md:59`.** "Sparse products are still 85–96% of what remains" holds for `la_add` only. For `la_add + la_mul` the share is 81.6–95.1%.
- **E5, `RESULTS.md:8` and `reports/run-20261124T0719Z/report.typ:8`.** "Built from 4faca82a6f0" overstates the evidence.
  - The binary was built at 06:47:21, before that commit was made at 06:50:13, from the uncommitted working tree that commit then recorded.
  - The bench record says `commit_dirty: true`, and `source.sha256` hashes only three source files.
  - The binary hash in `binary.sha256` is the binding.
  - The validator replayed all 148 rows byte-identically with that binary, but did not rebuild it from source.
- **E6, `PROTOCOL.md:56` (section 4 amendment).** "The first attempt's system has no solution for either arm" is wrong.
  - On all 8 grid fallback pairs and all 4 development pairs, attempt 1 has rank = rank without t, which is less than the number of columns. So t is undetermined, not inconsistent.
  - The solver returns a wrong t, which the certificate check rejects.
  - The validator found this with the binary's uncounted `--quartic-diag`, which leaves every committed field unchanged (40 of 40 runs).
  - The conclusion still holds: the full-core fallback repeats a solve that cannot determine t.
  - It sharpens the next lead. A rank test on the square subsystem would detect an undetermined t before the fallback.

## Process findings (no effect on the result)

- **P1.** 13 data files over 64 KiB are in Git (`raw/run.jsonl` and 12 `dev/*.jsonl`). S3 upload is unavailable in this environment: the bench record says `uploaded: false`.
- **P2.** No `w_cal` (measured GAE per native operation on this host) is recorded. The band w ∈ {0, 0.1, 1} is reported without it.
- **P3.** RESULTS.md does not write out the §3a cost chain. The validator checked its premise: the relation phase is identical in both arms on all 40 pairs.
- **P4.** The frozen `analyze.py` reads keys with `.get` and has a weak completeness check. Neither affects this study, because every key exists and the grid is complete.
- **P5.** No `cargo test` or clean rebuild was run by the validator, because builds were not allowed during validation.
