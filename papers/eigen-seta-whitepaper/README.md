# Eigen-Séta: performance and security white paper

`whitepaper.tex` is a standalone white paper for Eigen-Séta v2 as
implemented in the sibling repository `eigenx`. It covers:

* the cost of key generation, encapsulation, decapsulation and the 1 KiB
  authenticated PKE round trip at all five parameter sets (L1, L3, L5, Q3,
  Q5), measured in one campaign on one host;
* exact key, ciphertext and file sizes;
* the security level of each set under NIST's two readings (Tier A, gate
  count under MAXDEPTH; Tier B, at least 2^λ quantum queries), with the one
  cost-model input that can be measured (the attacker's chain step)
  measured;
* the catalogue of known attacks, ranked risks, big unknowns and open
  questions.

The specification, proofs and full security analysis live in the companion
paper [`../eigen-seta/`](../eigen-seta/) (revision 2).

**Status.** Research document, anonymized, not peer reviewed, not submitted
or posted. Security levels are conjectures derived from a cost model, not
measurements of attacks. Timings come from a shared 4-vCPU cloud VM at
isolation level L0 and are not reference-platform numbers.

## What it does not say

* Nothing about attacks we have not thought of. The hidden-orientation
  question (risk R1) is open and would break every set if answered the
  wrong way.
* No comparison with another implementation of Eigen-Séta (none exists).
  The comparison with other KEMs (§5) uses liboqs on the same host; HQC
  there is the portable implementation.
* No constant-time claim for the current theta build.

## Building

```
python figures/make_numbers.py ../../research/eigen_seta_whitepaper_20261017 .
make           # pdflatex, bibtex, two more passes
make check     # references, citations, LaTeX errors, overfull boxes
```

`make_numbers.py` only lays out recorded values (medians and ranges) from the
benchmark records into `numbers_data.tex` and `figures/perf.*`.

## Reproduction

| What | Where |
|---|---|
| benchmark harness | `eigenx:examples/bench_kem.rs` (commit `d89d790`) |
| records, host, run report, graphs | `crypto:research/eigen_seta_whitepaper_20261017/` |
| benchmark copies | `crypto:benchmarks/20261017-eigen-seta-whitepaper/` |
| security levels | `eigenx: cargo run --release --example security_estimate` |
| same-host comparison (liboqs 0.13.0) | `crypto:research/eigen_seta_whitepaper_20261017/tools/pqkem_bench/`, records `records/pqkem_same_host/` |

## Before any submission

1. External cryptanalysis of risks R1–R4.
2. Timings on a dedicated, pinned host and on the NIST reference platform.
3. Constant-time evidence on the theta build.
