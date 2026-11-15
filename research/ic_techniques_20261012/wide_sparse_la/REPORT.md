# Wide-modulus sparse LA: run report (RunPod, 2026-11-12/13)

**Status: measured. The protocol's frozen verdict is "extrapolation unsupported":
the width-law falsification test failed.** Protocol: `PROTOCOL.md` (frozen
2026-11-12) and `PROTOCOL_AMENDMENT_1.md` (RunPod host, committed before any
data). All numbers below are read from `results.json`, which the native
analysis crate (`analysis/`, `wide-sla-analysis --raw raw --out results.json`)
produced from `raw/`. They are labelled *counted* (host-independent operation
counts), *measured* (wall time on the RunPod pods) or *derived*
(ns per counted operation = 10⁹ · `solve_s` / `row_ops`). Every cited figure
is `[pcc:unverified]`, as no independent validator has re-run this study.

## Run

| item | value |
|:--|:--|
| instrument | `examples/la_engine_bench.rs ours --modulus-bits B` at commit `837a142e63` (bundle sha256 `12e53223…`, `launch.json`) |
| host | RunPod Secure CPU `cpu5g`, 4 vCPU, 16 GiB, on-demand, image `rust:1.97.0-bookworm`; seed 1 and 3 pods on AMD EPYC 4564P, seed 2 pod on AMD EPYC 9575F (`raw/seed*/host.json`); no L2 isolation (amendment §1) |
| grid | 7 width arms × 2 solvers (`full-bw`, `full-bl`, block 4) × D = 2¹²…2²⁰ × seeds 1–3, m = 3, ±1 coefficients; secondary `--coeffs wide` at D = 2¹⁶; A/A per pod before, at the D = 2¹⁶ midpoint, and after |
| attempts | 438 (146 per pod): 414 valid, 24 timeouts at the 3,600 s limit, 0 invalid, 0 censored, 0 repeats needed |
| timeouts | all at D = 2²⁰, both solvers, arms U128@127, U192@129, U192@190 and U256@255 on every pod (8 per pod) |
| cost | about 52.7 pod-hours at USD 0.184, about USD 9.7, plus USD 0.25 for the failed first launch (`launch.attempt1.json`); ceiling USD 25 |
| raw data | `raw/seed{1,2,3}/` and `s3://setec-astronomy-artifacts/lab-results/wide-sparse-la/20261112T2224Z-837a142e63/` |

## Results

**Counted work: size law (question 1).** `row_ops / D` grows as D^(β−1) with
β = 1.96–1.97 for every arm and both solvers, inside the expected Krylov range
[1.9, 2.1]. `core_dimension / D` and `core_nonzeros / D` are flat
(β = 0.998–0.999 and 0.992–0.993). The count does not depend on the width:
the worst max/min across arms at fixed (solver, D, seed) is 1.01. A/A counted
units are identical across all nine U192@129 runs and all nine u64@61 runs
from the three pods. `--coeffs wide` changes `row_ops` at D = 2¹⁶ by a factor
of 0.992–1.029.

**Wall time per counted operation: width law (question 2).** Derived
ns per counted operation, median over the three pods:

| arm | limbs | `full-bw` D = 2¹⁶ | `full-bw` D = 2¹⁹ | `full-bl` D = 2¹⁶ | `full-bl` D = 2¹⁹ |
|:--|--:|--:|--:|--:|--:|
| u64@61 | 1 | 1.09 | 1.39 | 2.78 | 2.63 |
| U128@61 | 2 | 3.61 | 4.30 | 7.16 | 7.59 |
| U128@89 | 2 | 4.58 | 5.48 | 8.85 | 9.06 |
| U128@127 | 2 | 7.03 | 8.19 | 11.25 | 11.57 |
| U192@129 | 3 | 10.75 | 13.53 | 16.53 | 16.84 |
| U192@190 | 3 | 9.38 | 11.85 | 14.37 | 14.73 |
| U256@255 | 4 | 20.31 | 20.93 | 24.22 | 24.39 |

Inside one word, the time per operation depends on the modulus. U128 runs at
3.6, 4.6 and 7.0 ns (`full-bw`, D = 2¹⁶) for the 61, 89 and 127-bit moduli.
U192 runs the 129-bit ECC2K-130 order slower than the 190-bit prime
(10.8 against 9.4 ns). The fitted t(L) = c₀ + c₂L² gives t(3)/t(1) ≈ 9.1–9.8
(`full-bw`) and 5.9–6.2 (`full-bl`).

**Falsification tests (frozen).**

| test | outcome |
|:--|:--|
| hold-out at 2²⁰ for the U192@129 target | not evaluable: the target arm timed out at 2²⁰; the same test passes on the six arms that reached 2²⁰ (u64@61, U128@61, U128@89, both solvers) |
| curvature, (log₂ D)² term over D ≥ 2¹⁴ | pass: coefficient 0.0032 (`full-bw`) and 0.0031 (`full-bl`), 95% bootstrap intervals include 0 |
| width independence of the count | pass: worst max/min 1.01 (threshold 1.10) |
| width law, same word, beyond the A/A band | **FAIL**: 43 of 54 same-word comparisons exceed their pod's A/A band |
| planted check | pass: 0 unverified cells |

**Extrapolation (question 3).** Under the frozen rule, a failed test rejects
the law, so the D = 10⁹ extrapolation for the ECC2K-130 order is reported as
**unsupported**. The arithmetic of the frozen formula, given here only for the
record and not as a supported prediction, is `row_ops(10⁹)` = 3.33·10¹⁷
(`full-bw`) times the fitted t(3) = 11.8 ns, which is about 1.1·10⁶
single-host CPU-hours.

## What failed and what did not

The size law of the counted work held up on every test that could be run. The
part of the model that failed is the claim that the time per operation depends
on the limb count alone. The cost of a multiply-subtract also depends on where
the modulus sits inside its word.

One candidate mechanism is in the code. `LazyLanes` folds an accumulator back
to a residue after `lazy_terms = ⌊(R − 1)/m⌋` products, so a modulus near the
top of its word folds far more often (`lazy_terms` = 2 for 2¹²⁷ − 1 in U128).
That is a hypothesis, not a measured cause, and it does not explain the U192
order (129-bit slower than 190-bit).

## Caveats

- Wall times come from three different RunPod hosts with no L2 admission. The
  seed-2 pod ran on a different CPU model and has the widest A/A band (1.57,
  from the u64@61 runs; 1.03–1.09 elsewhere). Ratios are taken within a pod,
  as frozen.
- The binaries built on the three pods have different SHA-256 values, although
  they came from the same verified source bundle and the same rustc. Their
  counted outputs are identical wherever compared: the A/A runs, and the
  width-independence test.
- The analysis was first prototyped in Python and then ported to the native
  crate under the repository's no-Python rule. The two agree on every
  non-bootstrap value to within 5.1·10⁻¹³ relative (`analysis/CROSSCHECK.md`).

## Next step

To make the extrapolation supportable, re-specify the width law per modulus,
not per limb count. Then test the `lazy_terms` hypothesis directly, by timing
U128 at moduli just below 2⁶⁴, 2⁹⁶ and 2¹²⁷ at fixed D. That is a new protocol,
not an amendment of this one.
