# C78: guarded sparse-prime powers and changed batch cohort

The exact kernel forms a sparse signed exponent using one squaring ladder, multiplies positive/negative terms separately, and uses a checked polynomial inverse only for unit bases. Nonunits use the frozen positive-exponent routine. No exponent is reduced modulo p-1 in a repeated-root quotient. P521 keeps the inverse-free Mersenne branch. The field-entry router sends ineligible primes directly to the reference solver.

Independent checks passed1,320 Taylor/Hasse remainder cases in actual NIST and small fields,15 complete constructed large-field root cases including zero/repetitions,764 checked inverse calls and232 nonunit fallbacks. The direct-reference route was verified for17 and65537. Powering counters exclude Euclidean inverse work; measured process lifetimes include that work.

The changed cohort was frozen before evaluation: five distinct exposed translated orbit inputs per process in all five NIST fields plus F65537. Three fresh-process repetitions per method produced36 batch runs and180 complete conditional solves. All root vectors, shift candidates and degree12 polynomials matched. Startup, parsing, every solve, original residual/auxiliary verification and final stdout are included; parent bookkeeping is inside experiment lifetime but outside child times. These are batch times, not single-instance cold latency.

| Field | Generic batch median s | Adaptive batch median s | Generic/adaptive | Generic span | Adaptive span |
| --- | ---: | ---: | ---: | ---: | ---: |
| P192 | 0.300912 | 0.174091 | 1.7285 | 8.39% | 36.79% |
| P224 | 0.327824 | 0.227207 | 1.4428 | 4.79% | 6.95% |
| P256 | 0.425965 | 0.306449 | 1.3900 | 27.12% | 25.49% |
| P384 | 0.861362 | 0.498506 | 1.7279 | 3.85% | 14.82% |
| P521 | 1.960362 | 1.026377 | 1.9100 | 5.54% | 5.03% |
| F65537 | 0.039374 | 0.045938 | 0.8571 | 18.53% | 57.82% |

The five-NIST diagnostic geometric gain is 1.628128304253775x; the all-six-field gain including control is 1.4630083406427619x. The preregistered>=1.3x NIST throughput prediction is met by development medians. P192/P256/P384 groups remain noisy; F65537 has a retained slowdown despite direct-reference arithmetic. It cannot be dismissed as a successful regression check. The candidate remains experimental. This does not establish repeated isolated performance across arbitrary fields.

Every run records wait4 peak, post-exec samples, child peak and operation counters. All raw outputs are archived. Earlier C77 noisy single-instance results are preserved; P521 is not compared across incompatible single/batch timing definitions. These exposed translations are not independent hard holdouts, and a batch exceeding one second does not supply the gate's per-instance hardness floor.

Next: correct the supervisor's coarse completion polling using event-driven process completion while retaining deadlines and memory evidence. Freeze an A/A calibration before further performance promotion; startup variability remains charged and must not be erased. Binary semilinear trace jumps and specialized exact modular squaring are separate algorithmic hypotheses requiring independent oracle tests and full cost accounting.

Two frozen90-second,4-GiB,one-core experiments passed through resume/run-ready. Classical signed addition chains do not establish a novel generalized algorithm. Full independent-source search, genuine matched F4/F5/F6,20 hard completed instances and50x remain open. Scientific goal active. Budget receipts record coordinator/publication duration separately; worker limits were preserved.
