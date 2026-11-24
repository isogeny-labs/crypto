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

The selected Gröbner-basis backend is explicitly `sage/libsingular:std`; root extraction uses Sage’s `variety` dispatch. This explicit selection may differ from earlier runs that used Sage’s default backend selection, so timings across that change are not directly comparable. The system is the third summation polynomial evaluated at x(R), plus one product-of-roots polynomial for each block. The record includes the input system, the computed basis and roots. `input_terms` encodes each polynomial as ordered exponent vectors and coefficients modulo `coefficient_modulus`; `input_term_counts` records its actual sparsity. Each root is mapped to retained full points; all signs are tried and the actual group sum is checked. For `s1 P1 + s2 P2 = R`, it records `A=s1*a1+s2*a2-aR` and `B=s1*b1+s2*b2-bR` modulo subgroup order. When B is nonzero, it computes `-A/B` and checks the resulting multiple of P against Q. `relation_counts` separately records zero-B witnesses, nonzero-B witnesses, verified nonzero-B witnesses, and failed scalar checks. Any failed nonzero-B scalar check marks the whole attempt `verification_failed`. The subgroup check requires a prime-order P and the curve order to contain that prime only once. No target scalar is used to create R or the systems.

The generation-phase `generation_operations` records target draw attempts and explicit scalar-multiplication and point-addition API calls. It includes target generation, repeated draws, walk setup, walk advances and two-group precomputation. These are calls, including calls with zero scalars or identity operands; they are not counts of internal curve additions, field operations or operations in setup, solving or verification. The panel aggregates these counters with observed/unknown coverage.

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

### Raw Singular protocol

Add `--solver-protocol` to the driver or the panel plan's common arguments to request Singular's raw computation protocol. The panel saves stdout and stderr separately and includes their SHA-256 hashes in each receipt. With the option disabled, the explicitly selected backend is the same. Protocol collection adds overhead and may be buffered or incomplete after interruption. Its text is backend/version specific; the driver does not interpret its numbers as degree of regularity, solving degree, arithmetic-operation totals, or certificates. Sage and the root-extraction backend may print additional text in the same stream.

The driver emits `=== BEGIN GROEBNER BASIS PROTOCOL ===` and `=== END GROEBNER BASIS PROTOCOL ===` around the basis call, flushing C and Python stdout at the boundaries. The end marker means the call returned or raised; consult the process exit and result status to establish completion. A killed process can leave the trace without an end marker. Root extraction is outside these markers.

The [Singular option manual](https://www.singular.uni-kl.de/Manual/latest/sing_324.htm) describes the `std` protocol: bracketed `[m:n]` values concern an internal representation change, and a separate numeric token describes a current leading-term degree. The [Sage libSingular documentation](https://sagemath.gitlab.io/documentation/html/en/reference/libs/sage/libs/singular/option.html) notes that degree interpretation depends on the command and ordering. The stream may be incomplete or contain other output; do not extract a regularity or solving-degree claim from a maximum number in the stream.

### Compare saved filter panels

After producing two panels with the same seed sequence and common non-filter arguments, compare their saved artifacts without running another solve:

```sh
python3 research/e2e_relation_measurement/compare_panels.py /path/to/panel-a /path/to/panel-b --output /tmp/filter-comparison.json
python3 research/e2e_relation_measurement/test_compare_panels.py
```

The comparator checks plan and artifact hashes, each receipt against its result, each result or checkpoint configuration against its frozen plan, source identity, explicit non-filter arguments, seed order, timeout, platform string, and target values. It lists each paired outcome and whole-process wall time, including missing or failed attempts. Per-method summaries include attempts, completed and useful runs, nonzero-B outcomes, summed whole-process wall, and observed candidate/rejection counts with unknown coverage. Candidate counts are checked against the corresponding result or checkpoint before aggregation. A missing target, incomplete run, unverified independent audit or mismatch makes `matching_checks_pass` false. Explicit argument spellings must match even if omitted and explicit defaults would be mathematically equivalent; ambiguous arguments are rejected. The result file must not already exist.

These checks establish only the stated record-level matches. Equal timeout limits are **not** equal realized cost, and a matching platform string does not establish matching hardware, load, resource isolation or timing boundaries. The comparison does not include a single-target rho measurement, authenticate the files' origin, or infer a speedup. Frozen resource envelopes, verification evidence and complete single-target intervals are still required for a cost claim.

### Resource observations

The panel snapshots parent-process metadata at the start and end: operating-system release and architecture, CPU model when available, logical CPU count, affinity when supported, inherited resource limits, selected thread-count environment variables, and the native RSS unit. The child may change settings after launch; this snapshot does not observe that or prove isolation, exact hardware identity, equal load, physical-memory availability, or an external resource envelope. Comparison reports metadata as `unavailable`, `incomplete`, `different`, or `metadata_match_only`; even the last value is not an attestation that resources match.

Each completed driver reports CPU time for its measured internal interval and process-lifetime peak RSS. The panel summarizes observed CPU time by sum and peak RSS by maximum, together with observed/unknown attempt counts. RSS is bytes on macOS and KiB on Linux. A killed child generally cannot report these values, so they remain unknown. The internal CPU interval excludes process startup and some final output work; it is not the same interval as the panel's whole-process wall time. Whole-process wall totals sum child attempts and exclude panel-supervisor work between attempts, including resource-metadata collection.

### Independent replay of a saved record

```sh
python3 research/e2e_relation_measurement/audit_record.py /tmp/relation.json --output /tmp/relation-audit.json
```

This standard-library verifier independently checks prime field and subgroup orders below `2^32`, curve membership, each recorded target/candidate coefficient pair, identity rejection, x-filter and x-deduplication decisions, exact structured system coefficients, each reported root, every signed witness for those roots, and each supplied scalar. It never searches for a scalar. A capped acquisition can receive a `verified_partial` bookkeeping result; invalid records receive `invalid` and a nonzero exit status. The receipt binds the input and verifier source by SHA-256 and records the audit's internal wall/CPU interval. That interval excludes interpreter startup and receipt writing. The panel separately records the full audit-subprocess wall time, including startup and receipt writing.

The audit does not establish solver root completeness, regenerate the seeded stream, authenticate source files, or validate timings and resource conditions. Unless a provided scalar verifies, it does not independently establish that Q belongs to the subgroup generated by P. It rejects field and subgroup parameters outside the stated 32-bit limit; the main driver has a broader range.

The panel streams child stdout and stderr directly to per-attempt files and hashes them in bounded chunks. It does not include supervisor memory or log-storage costs in child peak RSS. Log storage is not size-limited; an external storage or job limit is needed for long-running protocols. Hashing and supervisor processing after a child exits are outside that child's whole-process wall interval.

For each driver result with status `complete`, the panel invokes the independent auditor when both field and subgroup order are below `2^32`. It saves a separate audit receipt and stdout/stderr, binds their hashes and the auditor source to the attempt, and distinguishes `verified`, `invalid_or_error`, `timeout`, `launch_error`, and `unsupported`. Incomplete driver runs do not launch an audit. `useful_runs` reflects the driver's internal verification; `independently_verified_runs` requires a successful independent audit of the supplied record and witnesses. Neither asserts that all roots were found.

The audit subprocess has the plan's per-attempt timeout **in addition to** the driver's timeout. `total_process_wall_ns` remains the sum of driver subprocess intervals; `independent_audit_wall_ns` gives separately observed audit cost and unknown coverage. `observed_child_and_audit_wall_ns` adds all executed driver and audit subprocess intervals, including failed attempts, but excludes supervisor processing, storage accounting, retries not included in the panel, and other target-dependent phases. It is not a complete single-target cost. The comparator requires verified audit receipts for all paired completed runs; older panels without these receipts are not silently upgraded.
