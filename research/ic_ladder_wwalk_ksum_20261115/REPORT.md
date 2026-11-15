# Ladder panel results: wwalk C = 2 and ksum k = 6, 7 at 36 and 40 bits (2026-11-15)

Frozen protocol: `PROTOCOL.md` (commit `35e9bf77`, amendment A1 at `dba8c5cb`, both before any data). Every number is
measured by ecbench on the frozen binary unless labelled otherwise, and all are unverified assertions by the producing
session [pcc:unverified]. No validation gate has been run (no second-host replay certificate, no validator report), so
no leaderboard or `benchmarks/README.md` row is made.

## Run

- **Host:** RunPod pod `23hrxqj6cngzmc` (cpu5g, 8 vCPU, AMD EPYC, rust 1.97.1), 08:42–09:55Z, about $0.45. [pcc:unverified]
- **Build:** ecbench was built on the pod from a hash-checked archive of the frozen commit (binary sha256 in
  `results/BINARY.sha256`).
- **Earlier attempt:** the first pod (`bzxcwivc23x3hv`) collected no data. Every lane failed taskset (CPU ids not
  starting at 0), which led to amendment A1.
- **Sessions:** 36 bits, 40 bits and the k = 32 positive control are complete, each `verify --replay-all` rc = 0.
- **Refused at plan time:** 44 and 48 bits. ecbench refused both test sessions because the plain wwalk table packs
  coefficients in 32 bits (`wwalk: coefficients up to 2.64e10 do not fit the 32-bit packing`). This is a limitation of
  the existing implementation. Their training sessions (T44, T48) completed and are kept.
- **Digests:** `results/SHA256SUMS` verifies every file except `runlog.tsv`, to which `run.sh` appends its final line
  after writing the digest list.

## Results (48 runs per arm, S = counted GAE / √r; paired ratios with 4,000 bootstrap resamples)

| ratio, w = 0 [95% CI] | 36 bits | 40 bits |
|---|---|---|
| wwalk-c2 / rho-neg | 1.660 [1.358, 2.082] | 1.685 [1.386, 2.093] |
| wwalk-c2 / ksum5-sel | **0.240** [0.214, 0.268] | **0.212** [0.194, 0.231] |
| ksum6 / ksum5 | **0.930** [0.884, 0.979] | **0.868** [0.829, 0.911] |
| ksum7 / ksum5 | **0.941** [0.899, 0.985] | **0.856** [0.820, 0.893] |
| ksum7 / ksum6 | 1.012 [0.967, 1.059] | 0.986 [0.941, 1.032] |
| ksum5 / rho-neg | 6.91 | 7.94 |

At w = 1 the ksum gains grow, to 0.843 (ksum6) and 0.787 (ksum7) at 40 bits. wwalk-c2/rho-neg rises to 1.82–1.86.

## Against the frozen predictions

- **Lead 1 (wwalk at the ladder sizes):** [pcc:unverified]
  - wwalk-c2/rho-neg at w = 0 and 0.1 lies below the registered band [1.75, 2.15], i.e. cheaper than predicted. The
    point estimate is outside the band, and the interval overlaps it. At w = 1 it is ≤ 2.45 (registered).
  - wwalk-c2/ksum5 ≤ 0.25 holds at both sizes.
  - Kill criteria (ratio > 2.5 at ≥ 40 bits; wwalk/ksum5 > 0.40) do not fire.
  - The 44 and 48-bit cells are unmeasured (refused, see above).
- **Lead 2 (ksum k = 6, 7):**
  - ksum6 and ksum7 are below ksum5 at every weight at both sizes; the interval upper ends are all < 1.
  - ksum7 is not separable from ksum6.
  - At 36 bits, ksum8 does not beat ksum7 by more than 3% (registered check).
  - The kill rule (upper end ≥ 1 at w = 0.1) does not fire.
- **Controls:**
  - rho-neg A/A and wwalk-c2 A/A ratios are exactly 1.
  - Floor audits pass (wwalk S ≥ 1.50, ksum S ≥ 4.0).
  - The positive control on the autolab k = 32 curve gives S(w = 0) = 1.861, inside the registered 1.842 ± 10%.
  - There is no C = 0 arm (deviation recorded at freeze).

## Reading (distance remaining to matched rho, not pass/fail)

- **wwalk C = 2 holds its plateau beyond 32 bits:** 1.66–1.69× rho-neg at w = 0, against ksum5's 6.9–7.9×. The gap to
  rho is about 4.2–4.7× smaller than the validated generic row's at these sizes. This is new: wwalk had never run above
  k = 32.
- **ksum6/ksum5 = 0.87 at 40 bits** is a measured constant improvement of the k-sum row, 13% at w = 0 [pcc:unverified].
  The 40-bit distance to rho moves from 7.94× to 6.9×.
- **Before any row:** a second-host replay certificate, a validator report and a `VALIDATION.json` with the matched rho
  reference and §5a band (docs/VALIDATION_GATE.md).
- **To reach 44 and 48 bits:** a wider (64-bit) wwalk coefficient table and an amendment, then a rerun of those two
  sizes only (T44 and T48 can be reused).

Graph: `ladder_s_w01.svg`. Tables: `results/logs/evaluate.txt`, `results/analysis/results.{md,json}`. Large
`records.jsonl` files are kept in S3 and referenced by SHA-256 in `*.records.jsonl.pointer`.
