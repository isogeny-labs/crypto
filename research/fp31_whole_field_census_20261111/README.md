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

## Proposed next step (not run; needs a decision)

The measured counterpart of this census asks whether rho cost depends on anything
except r. Hold r fixed by taking prime-order classes only (r ≈ 2^31, 8,646 of them).
Stratify by class size H, conductor f and |D_K|. In each sampled class, solve on
curves from the crater and from the floor with 10 generators each, and test
whether steps / √(πr/2) varies with any class invariant. The expected outcome is a null.

- Positive controls: a composite-order class (Pohlig–Hellman) and the j = 0 class
  with the order-6 automorphism fold.
- Negative control: an A/A arm.

This is a benchmark under the repository rules. It needs a frozen protocol,
an ecbench spec, a `bench.record/v1` record and the isolab-eks route, so it
was not launched from this session.
