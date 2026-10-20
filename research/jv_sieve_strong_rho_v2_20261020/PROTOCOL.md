# Protocol V2: independent replication, numeric thresholds fixed before data

Frozen 2026-10-20, **committed before any V2 run exists**. It replaces nothing: study V1
(`research/jv_sieve_strong_rho_20261019/`) stays as it is, with its qualifications. V1's frozen
controls had no numeric acceptance bounds and its producer left some hash evaluations uncounted;
this study repeats the measurement on the corrected producer with every acceptance rule and every
decision rule stated as a number below. If a threshold turns out unsuitable, the outcome is reported
as failed, not changed: a changed threshold is a new protocol version.

Question and class are V1's: how does the Joux–Vitse sieve on `E(F_{p⁶})` (weak class only, never
prime order, genus-3 Jacobian) compare with a strong Pollard rho on the same instances, and does its
cost grow as ℓ^{1/3}? Class: measurement of an existing algorithm against a stronger reference.

## Producer identity

The producer is the source at the commit recorded in `raw/PROVENANCE.json` by `run.sh` before the
first run (`git rev-parse HEAD` and the SHA-256 of the `jv_cover` binary). It is built from the
repaired engine of PR #2281 (DP table keeps `y`, every `hash_x` evaluation counted where it happens,
option validation). No data of V1 is reused.

## Instances and arms

* Instances `generate_spec(p, seed)`, **seeds 17–32** (disjoint from V1's 1–16), `p ∈ {251, 503, 1009,
  1511, 1823}` (`ℓ = 2^45.8 … 2^63.0`; 1823 is the last `p` with `ℓ < 2^63`).
* Sieve: `jv_cover --exp sieve --stop 64`, `JV_LIN=dense` and `sparse`, 16 seeds per size and arm; the
  sparse arm repeated at `p = 1009` (A/A).
* Rho (`jv_rho_strong`, `r = 256`, DP bits 10/12/14/15/15): R0 plain, R1 batched (k = 16), R2
  negation, R3 negation + batched. Sample: R3 16 per size; R0 16 at 251/503/1009, 8 at 1511, 4 at
  1823; R1 and R2 16 at 251/503/1009; A/A (run index 1, new walk randomness) R0 and R3, 16 at 1009.
* Prices, `S`, `S⁺`, GAE = 331 and the §5a band (w ∈ {0, 0.1, 1}, layers A and B, the sieve side with
  the labelled upper bound for unemitted F4 additions) are V1's. **No derived corrections**: the
  corrected producer counts its hash evaluations, and the check A2 below enforces it.
* Statistics: means, 95 % bootstrap intervals over instances (10⁴ resamples, fixed seeds in the
  analyzer), ratios paired by instance **seed identity**, never by position.

## Acceptance (all must pass, or every decision below is withheld)

| id | requirement |
|:--|:--|
| A1 | the seed inventory of every arm and size is exactly the scheduled one (no missing, no duplicate, no extra seed) and every run is correct and uncapped |
| A2 | hash-scope completeness, per run: `ops.hash / steps ∈ [1.98, 2.02]` for arms without the negation map and `[3.9, 4.1]` with it (partition + DP check, plus two cycle-window hashes) |
| A3 | positive control: at every size where R0 ran, the 95 % interval of R0's mean `S` (harness price) contains the ledger value 1.3 and its point estimate lies in `[1.0, 1.6]` |
| A4 | rho A/A at `p = 1009`, for R0 and for R3: the ratio of means (first run over repeat), paired by seed, lies in `[0.85, 1.15]` and its 95 % interval contains 1 |
| A5 | sieve A/A at `p = 1009`: identical total multiplications and descent digest on all 16 instances |
| A6 | dense against sparse FGLM: identical descent digest and outcome on every pair, every size |
| A7 | instrument check: the strong rho's fitted exponent of total multiplications in ℓ (all five sizes) has point estimate in `[0.45, 0.55]` and a 95 % interval containing 0.5 |

## Decisions (computed only if A1–A7 pass)

* **D1 (K1).** At each `p ≥ 1009`: the 95 % interval of sieve/R3 in `S⁺` (harness price) lies below 1
  → "sieve below strong rho"; its upper end ≥ 1 → K1 fires at that `p`.
* **D2 (K2).** At each `p ≥ 1009`: ranked below at every weight iff the upper end of every sieve/R3
  interval is below 1 in all four band columns (A and B, harness and tuned) at w = 0, 0.1, 1 **and**
  in `S⁺` (tuned price); otherwise narrowed. A pair whose order changes inside the band is
  "unresolved", never a win.
* **D3 (exponent), confirmatory, both fits fixed now.** F1: sieve, all five sizes, total multiplications
  against ℓ. F2: sieve, `p ≥ 1009` only. Classification of a fit by its 95 % interval: "ℓ^{1/3} supported"
  iff it contains 1/3; "flat" iff it lies within `[−0.1, 0.1]`; "falling" iff its upper end is below −0.1;
  "other" otherwise. The claim "cost grows as ℓ^{1/3}" is made only if F2 is "ℓ^{1/3} supported".
* **D4 (crossover).** The smallest tested `p` whose sieve/R3 `S` (harness) interval lies below 1, and
  the same for `S⁺`, reported separately.
* **D5 (accelerations).** Negation "buys" iff the interval of mean-`S` ratio R0/R2 at `p = 1009` lies
  above 1. Batching "buys nothing in the harness price" iff the intervals of R0/R1 and of R2/R3 at
  `p = 1009` both contain 1; in the tuned price batching "costs" iff the interval of R3/R2 lies above 1.
* **D6 (sparse FGLM).** The end-to-end `S` gain dense/sparse at a size is "a gain" iff its interval lies
  above 1.

## What is reported regardless

Every raw run, including any invalid one, with its status; per size, per arm means and intervals; the
cross-study comparison with V1 (descriptive only: same sizes, different seeds, different producer).
Wall time is an observation. The tuned price is a cost model (Karatsuba products, Chung–Hasan
squarings, adjugate inversion; 24/18/62), not a faster implementation, and credits rho only.
