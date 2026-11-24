# Seed-to-relation measurement: two-summand development driver

This bounded SageMath driver captures the path from seeded coefficient pairs to a real two-summand polynomial system, roots, signed point relations, and checked scalar candidates. It is an initial instrument, not a general solver adapter or a completed comparison campaign.

Example correctness control (the scalar behind Q is fixed at 17):

```sh
sage -python research/e2e_relation_measurement/measure.py \
  --p 101 --curve-a 1 --curve-b 32 --px 4 --py 10 --qx 82 --qy 27 \
  --mode independent --seed 42 --block-size 10 --output /tmp/relation.json
python3 research/e2e_relation_measurement/test_measure.py
```

`--mode` selects `independent`, `walk`, or `two-group`. Each produces candidates `aP+bQ`, rejects the identity, filters on `x mod modulus == residue` and the inclusive x interval, and deduplicates globally by x. A walk has a seeded starting pair and fixed seeded increment, restarted after `--walk-length` candidates; a two-group generator precomputes independently seeded lists of multiples of P and Q and samples their Cartesian product. Rejections, coefficient pairs, coordinates, acceptance and order are recorded, including the group lists or walk parameters. Generation stops at two blocks of the requested size or the explicit candidate limit. The target pair is generated first and is the same across arms for identical seed and curve. Fixed increments and finite group lists can exhaust the available distinct points; exhaustion is retained as a distinct outcome.

The system is the third summation polynomial evaluated at x(R), plus one product-of-roots polynomial for each block. The record includes the input system, the computed basis and roots. `input_terms` encodes each polynomial as ordered exponent vectors and coefficients modulo `coefficient_modulus`; `input_term_counts` records its actual sparsity. Each root is mapped to retained full points; all signs are tried and the actual group sum is checked. For `s1 P1 + s2 P2 = R`, it records `A=s1*a1+s2*a2-aR` and `B=s1*b1+s2*b2-bR` modulo subgroup order. When B is nonzero, it computes `-A/B` and checks the resulting multiple of P against Q. `relation_counts` separately records zero-B witnesses, nonzero-B witnesses, verified nonzero-B witnesses, and failed scalar checks. Any failed nonzero-B scalar check marks the whole attempt `verification_failed`. The subgroup check requires a prime-order P and the curve order to contain that prime only once. No target scalar is used to create R or the systems.

The JSON includes candidate counts, source digest, parameters, Sage/Python and host information, separate generation, polynomial construction, Gröbner-basis, root extraction and verification wall/CPU times, total process-interval wall/CPU time, and process-lifetime peak RSS. Total includes validation/setup and reporting computations outside those phases; it is not the sum of the named phases. On macOS RSS is in bytes and on Linux in KiB. Root extraction may itself perform additional basis work. Timings are local observations and include instrumentation overhead. Two-group precomputation is included in generation. The empirical filter fraction and duplicate cost can be calculated from the candidate trace; do not replace them with an assumed uniform distribution.

**Coverage limits:** Sage's selected backend does not expose a degree of regularity, solving degree or field-operation count through this driver; these fields are null, with reasons. Maximum degree of the returned basis is a different quantity and is labelled as such. This driver only supports two summands, a single system per invocation, short Weierstrass fields of prime characteristic greater than 3, and interval/residue x filters. It does not provide an optimized rho arm, matched resource controls, a full campaign, independent polynomial certificates, or a total across retries to first success. A direct driver process interruption can leave no final JSON receipt; `--checkpoint PATH` writes an atomic snapshot at phase boundaries. The panel supervisor enables that option and records the snapshot hash alongside the outer timeout and exit status. An OOM kill is retained as an exit or signal, but is not diagnosed as OOM. Do not interpret this as an end-to-end IC-versus-rho comparison. A useful comparison must include every failed attempt and setup, use the same target and machine, repeat across frozen seeds, record resource limits, and include a verified single-target rho run.

Prior work: Semaev's [summation-polynomial construction](https://eprint.iacr.org/2004/031.pdf) supplies the signed-coordinate equation; `docs/ic/FACTOR_BASE_FACTORY.md` and `docs/ml-dataset/DESIGN.md` describe cold-cost accounting and replayable coverage. The third polynomial used here is also spelled out in the cryptanalysis `docs/known-label-pair-index-20261029/README.md` source. This implementation makes no novelty or speed claim.

## Matched panel supervisor

Write a plan JSON with a shared argument vector, explicit seed list, and per-process wall limit, then give it a new output directory:

```json
{"common_args":["--p","101","--curve-a","1","--curve-b","32","--px","4","--py","10","--qx","82","--qy","27","--block-size","10"],"seeds":[42],"timeout_seconds":30}
```

```sh
python3 research/e2e_relation_measurement/panel.py --plan plan.json --out /tmp/relation-panel
```

The supervisor runs the three modes for each seed and retains the copied plan, command lines, stdout, stderr, individual exit/timeout receipts, result hashes, whole-process wall times, and a summary. It checks that all three successful child records contain the same generated target per seed; unavailable comparisons are null. It counts completed, nonzero-B, useful, and verification-failed runs separately, records runs for which the nonzero-B outcome is unknown, and includes failed or incomplete attempts in accumulated whole-process time. Individual receipts retain candidate rejection counts. It does not enforce equal total work across arms, perform retries to first success, measure child CPU/RSS for killed jobs, or establish that a killed process ran out of memory. Each invocation needs a fresh output directory. The output directory and per-attempt files are durable as they are produced; a supervisor killed mid-attempt may leave that final attempt without a receipt or summary.

The per-mode summary also totals observed phase wall times and candidate, identity, filter-rejection and duplicate counts. Each total carries `observed_attempts` and `unknown_attempts`; `observed_sum` is null if no attempt supplied that value. Successful results supply these observations directly. If an attempt fails or times out, the supervisor may instead use its last valid phase checkpoint, which records only completed phases. An observed sum with unknown attempts is a partial sum, not a complete cost or an estimated value for the missing attempts. The separately reported whole-process wall total includes all launched attempts and their recorded timeouts.

For a practical comparison, predefine the filter and budget, run the same frozen seed/target panel, and report candidate acceptance and all rejected work along with useful-run frequency. Do not compare only successful solve times. The counted negation-aware `rho_full_ec` in `src/cryptanalysis/chained_transfer/negation_remeasure.rs` is a potential reference, but this panel has not yet connected it or checked its wall-clock and resource boundaries against the Sage arm.

### Phase checkpoints

Each snapshot records the source digest at startup, current phase and completed phase timings. Once generation finishes it includes all candidates, the curve and target; after construction it also includes the input polynomials. Later snapshots add the computed basis and roots. A snapshot naming a phase does not imply that phase completed; only entries in `timings` correspond to completed calls. A terminated process may leave the last snapshot at an earlier boundary, including during setup or generation. The JSON replacement is atomic but is not an fsync-backed crash durability guarantee. Snapshot writing is included in the driver's total wall and CPU intervals, but not in individual solver-phase timings; it adds instrumentation and I/O overhead.

The panel records the driver and relation-accounting module digests before and after the panel and reports whether the file remained unchanged. This is a source-file integrity check, not an isolated execution environment or a guarantee that libraries and hardware remained unchanged.
