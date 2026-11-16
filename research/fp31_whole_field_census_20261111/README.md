# Every isogeny class over F_p, p = 2^31 − 1: exact census (2026-11-11)

Request (user, 2026-11-11): sweep a whole prime field, 2^31 − 1, for ECDLP:
every curve, every isogeny, every transfer up (covers, F_{p^2}), several
generators, and look for statistical differences across isogeny classes.

**Report:** [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)).
All results are exact computations and unregistered in pcc: **[pcc:unverified]**.
This is a characterization study. It is not an improvement over the
index-calculus baseline and adds no leaderboard or `benchmarks/` row.

## What was already settled, and not redone

| part of the request | status | where |
|---|---|---|
| rho across the curves of one class | measured null (38 curves × 100 seeds; KW p 0.38) | `research/isogeny-class-rho/`, `research/isogeny-class-rho-n45/` |
| IC relation yield on every curve of a full class, 24–32 bits | measured null (9/12 cells; 1 flag not replicated) | `research/isogeny_class_ic_yield_20261102/p2_prime_analogues/RESULTS.md` |
| why: generic attacks see only #E and Aut(E) | theorem | `research/isogeny_class_difficulty_20261008/` |
| lift E/F_p to F_{p^2} | theorem: #E(F_{p^2}) = #E · #E^tw, so r cannot shrink; census: 0 of 185,363 classes | `REPORT.pdf` §1 |
| covers over a prime field | theorem: no subfield to descend to; genus ≥ 3 IC over the same F_p costs more than rho | `REPORT.pdf` §1 |

## What this study adds

One row for each of the 185,363 traces (= isogeny classes) of F_{2^31−1}:
#E and its factorisation, the largest prime r, twist and F_{p^2} orders,
D = f²·D_K, h(D_K), the exact number of curves in the class H(4p − t²),
the endomorphism-ring strata (volcano depths), the embedding degree, and a
modelled Pohlig–Hellman + rho cost.

Headline numbers (exact, [pcc:unverified]):

- Σ_t H(4p − t²) = 2p = 4,294,967,294 curves exactly.
- 8,646 classes (4.66%) have prime order, but they hold only 2.64% of curves.
  Median log2 r is 18.65 over curves and 19.31 over classes.
- The only systematic cross-class effect: larger classes have smoother orders
  (Spearman log2 r vs log2 H = −0.141; vs number of endomorphism-ring strata
  −0.134). This is Lenstra's divisibility bias: P(ℓ | #E) for a random curve is
  ℓ/(ℓ²−1) or 1/(ℓ−1), against 1/ℓ for a random trace. The census matches it
  for every ℓ ≤ 47 to a relative error of at most 1.3e-4. This is a known theorem,
  not a new weakness.
- Weak classes are weak for textbook reasons only: small r (Pohlig–Hellman),
  t = 1 (anomalous, 17,184 curves), t = 0 (supersingular). No class with
  r ≥ 2^20 has embedding degree below 31.

## Checks

1. Σ 6H = 12p exactly (`data/census_p2147483647.jsonl.check.json`).
2. `fp31-class-census bruteforce --p P` counts the points of every (a, b) over
   F_P naively and matches (P−1)/2 · H on every trace, for P = 101, 103, 107,
   109, 113, 1009 and 1013 (all of P mod 12). There were 0 mismatches.
3. A PARI (cypari2) prototype gave the same values as the native tool on all
   17 integer fields of all 185,363 rows. The prototype was used only as an
   external cross-check, is kept out of this directory (AGENTS.md
   "Implementation language: no Python"), and its output had sha256
   `b228d140f9f82711cf147dfbe1c62a08c925d990214faed73f1c7cdc5ce26009`.

## Reproduce

```sh
cargo build --release --manifest-path tool/Cargo.toml
B=tool/target/release/fp31-class-census
for p in 101 103 107 109 113 1009 1013; do $B bruteforce --p $p; done
$B census --out /tmp/rows.jsonl --threads 4          # ~3.5 min on 4 vCPU
gunzip -c data/census_p2147483647.jsonl.gz | sha256sum   # compare with SHA256SUMS
$B summary --rows /tmp/rows.jsonl --out /tmp/summary.json
python3 figures/plot.py /tmp/rows.jsonl /tmp/summary.json figures   # layout only
```

`SHA256SUMS` lists every data file and source file. The uncompressed row file
is git-ignored; `data/census_p2147483647.jsonl.gz` is `gzip -n -9` of it.

## Phase 2: rho measured across 57 classes (run 2026-11-11)

Report: [`phase2/REPORT_PHASE2.pdf`](phase2/REPORT_PHASE2.pdf). Protocol, frozen before data:
[`phase2/PROTOCOL.md`](phase2/PROTOCOL.md) (sha256 `3d81d9fe…19c7ba`). The task card was compiled by the
director and run by the executor; its receipt and the director's assessment ("executed as specified") are in
`phase2/cards/`. Benchmark record: `benchmarks/records/20261111T0634Z-fp31-rho-across-classes.json`.

- **Classes.** 57 classes were drawn by a frozen rule from 400,000 seeded random curves:
  - 16 prime-order classes in each of three endomorphism-ring strata;
  - 8 cofactor-4–64 classes, to check √r scaling;
  - the j = 0 class.
- **Scale.** 394 curves reached by verified isogeny walks; 281,240 rho runs; 0 classes failed.
- **Result** ([pcc:unverified]): **null supported** under the frozen rule. After normalizing by √(πr/2),
  rho cost does not depend on the class (Kruskal–Wallis p = 0.076), the class size, the conductor, the
  number of endomorphism rings or |D_K| (Spearman p ≥ 0.45). Nor does it depend on the curve within a class
  or on the generator (per-class p-values uniform, KS p = 0.20 and 0.20). The resolution is about 1% per class.
- **Controls.**
  - Detector (planted negation fold): 0.741, inside its window.
  - A/A replica: 1.003. Isomorphic models: 0.998.
  - √r scaling: 1.009.
  - j = 0 automorphism fold: 0.449, against 1/√6 = 0.408.
- **Execution.** Local at isolation L0, a recorded deviation: there is no AWS identity here, and the EKS
  dispatch only accepts ecbench specs. The raw data are hashed in the record but not uploaded to S3.
- **Reproduce.**
  - Regenerate the root streams with `fp31-class-census roots` and check them against `phase2/ROOTS.sha256`.
  - Run `phase2/run_phase2.sh target/release/isoclass-rho phase2/manifest.json <dir> 200 40`.
  - Or unpack `phase2/runs_phase2.tar.gz` (sha256 `450d8ea9…a2291e`) and run `fp31-class-census analyze`.

## Phase 3: index-calculus relation yield across 48 classes (run 2026-11-12)

Report: [`phase3/REPORT_PHASE3.pdf`](phase3/REPORT_PHASE3.pdf). Protocol, frozen before data:
[`phase3/PROTOCOL.md`](phase3/PROTOCOL.md) (sha256 `572f7dfb…eeea`). Instrument: the P2 instrument of
`research/isogeny_class_ic_yield_20261102`, unchanged (`p2tool` + `ecbench`). Cross-class tests:
`fp31-class-census ic-analyze`. Benchmark record:
`benchmarks/records/20261112T0103Z-fp31-ic-yield-across-classes.json`.

- **Population.** The 48 prime-order classes of phase 2:
  - 332 curves reached by their isogeny walks;
  - 96 isomorphic models.
- **Measurement.** m = 3 and m = 2, with a prime-abscissa factor base of one size per m in every class
  (378 and 1448), and a planted +20% arm.
- **Scale.** 526,111 relations in 143.9 M attempts.
- **Execution.**
  - Attempt 1 was lost to a container restart after 37 of 96 cells. It is recorded in `benchmarks/losses/` and
    described in `phase3/RESTART_NOTE.md`.
  - Attempt 2 resumed with identical bound inputs. 96/96 cells finished; 0 failed.
- **Result** ([pcc:unverified]): **null supported for both m.**
  - All 14 tests have p > 0.05/14. The smallest is the m = 3 within-class over-dispersion test, p = 0.028.
  - Normalized yield does not depend on the class (Kruskal–Wallis p 0.58 and 0.64), nor on class size,
    conductor, number of endomorphism rings or |D_K| (Spearman p ≥ 0.33).
  - It does not depend on the curve within a class (KS p 0.87 and 0.38).
  - The planted +20% arm reads 1.198 and 1.200.
- **Reading.** Over F_{2^31−1}, neither rho (phase 2) nor this IC decomposition distinguishes isogenous curves or
  depends on the class beyond n.
