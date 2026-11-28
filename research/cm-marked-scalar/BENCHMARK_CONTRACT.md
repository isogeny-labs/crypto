# Benchmark contract: proposed, no measurements yet

## Claim and workload

Claim class: comparative, absolute end-to-end evaluation of the exact public scalar [mu]P. This is public-scalar variable-base arithmetic; it is not a secret-scalar constant-time claim or an ECDLP recovery measurement. A single target point is one workload. Keep per-point rows; batch throughput is secondary.

Targets: P-384 and P-521 first, with the supplied order-seven mu and oriented maps. Preserve P-192 as the primary research continuation. Freeze the full point manifest before timing. Include O, G, -G, 17G as correctness controls and a deterministic independently generated panel of subgroup points for measurement. No point-specific tuning or selecting winning points after results.

## Arms

- Reference A: optimized native signed-window public-scalar multiplication, including point-dependent table setup. Freeze window selection before the held-out panel.
- External reference B: OpenSSL EC_POINT_mul; also assess the fastest exposed appropriate public-scalar OpenSSL configuration so a constant-time/private-scalar baseline cannot be the only comparison.
- Candidate C: phi, eta, sigma, centered scalar correction k, all on the same field/backend. Retain the existing implementation as the unmodified candidate before optimizing.
- A/A: two independent invocations of the selected reference with the same workload.
- Positive correctness control: known public scalar and point, checked by a separate implementation.
- Negative correctness control: a deliberately altered scalar label must fail equality and never enter speedup aggregation.

Correctness comes before interpretation. Every point result must match the reference, remain on the exact original curve and subgroup, and retain a compressed-point digest. Measure public-scalar methods together; report constant-time APIs as separately typed references.

## Accounting boundary

Primary cold per-point time starts before construction of the required runtime curve/map objects and point-dependent precomputation. It includes decomposition/correction preparation, forward and return maps, closing isomorphism, scalar multiplication, coordinate conversion and symmetric output verification. End after the verified original-model point exists. Report those phases and their sum.

Supplied curve and map coefficient constants are algorithm inputs, like the published curve parameters and scalar. Their historical derivation and construction attempts are separately preserved; do not hide them inside an amortized speed claim. Also report a full first-use accounting that explicitly states whether generating map constants is required for the proposed deployment. Compilation, process startup and known-answer point generation are excluded symmetrically. Deserialization or setup excluded from the clock must be reported separately, never called end-to-end cold.

Repeated-point or repeated-curve amortization is a separate secondary question with actual reuse counts. No precomputed point orbit, known input scalar, cached result or input-point transporter may serve as the candidate.

## Freeze, execution and decision

Before timing, add a native experiment.protocol/v1 record, exact source/input/compiler hashes, analysis, seeds, order, samples, warmups and commands. Run at least five paired rounds, alternating randomized A/B order, with A/A noise measured on the same host. Keep a held-out panel independent of window/chain tuning. Keep raw per-call results and failures.

Use the repository isolation contract: exclusive core reservation, affinity and environment/PSI readback; taskset alone is insufficient. Native replacement of the legacy Python isolation wrapper must preserve its contract before it is used. Wall time is inadmissible when isolation or contention gates fail; do not silently fall back to an ordinary shared-host timing claim. Deterministic operation counts remain admissible with their stated implementation scope.

Initial local budget: one process, one build at a time, at most 2 CPU threads and 2 GiB memory, 10 minutes per build, 2 minutes per correctness invocation, and 2 minutes per bounded measurement invocation; no automatic retries. No cloud build or EKS job is authorized by this contract. Failed attempts stay in versioned records. A new source/protocol version precedes a corrected attempt.

Success requires correctness on every included workload and an end-to-end improvement beyond the preregistered A/A uncertainty against the fastest reproduced matched reference. Report each curve and input separately, with setup included. The absence of a gain directs the next implementation; it does not close the user goal.
