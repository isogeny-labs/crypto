# Cold-query work comparison and separate arithmetic validation

Question: does the optimized production solver preserve exact finite-base
answers while removing executed work from the previous implementation?
This is an implementation comparison on disclosed fixed controls, not a
population, scaling, native-time or external-best claim.

Inputs are the unchanged 65-case workload from the prior scale study
(SHA256 2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5).
Development validation compares all 65 old/new answers and exact witness order.
A separate Rust checker imports no production field/group/solver code: it uses
schoolbook binary polynomials and Euclid inversion, validates all coordinates
and known witnesses, proves small negatives with complete pair tables and
large negatives through independent inconsistency of every necessary residual
linear system. Producer-written checking is not independently signed review [pcc:unverified].

Primary comparison: six existing d12 controls (n83 and n131, each with
`distinct-planted-0`, `uniform`, `cancellation`), three arms (`baseline`,
`baseline-aa`, `candidate`), five repetitions: **90 fixed primary executions**.
The unchanged baseline source is retained byte-for-byte in baseline_snapshot.rs.
Both implementations are compiled into one release executable with identical
field/group arithmetic. Order rotates by field/kind/replicate. No new target
selection or dropped cells after data. These controls are already exposed
anchors; no heldout/generalization claim is licensed.

Each row has TWO fresh processes: (1) native exact field-operation tally;
(2) the same query under Callgrind with tally disabled. Both include workload
parsing, fixture/variety construction, base admission, solver, roots/sign lifts
and final replay as applicable; Callgrind counts the complete process, including
parser, allocator, Gaussian and integer work. Field counters begin before query
fixture construction and end after replay; they exclude parsing. Compare the
full returned decisions, indices, points, models and diagnostics between runs.
A count-only process and a simulated-instruction process are not two independent
query samples. Inversion is informational: its inner multiplications/squarings
are already counted, so do not add it again. Batching charges unused tail work.

Primary metric: whole-process simulated instructions (Ir); secondary separate
counts of field multiply, square and informational inversion. Never price
unlike operations as a single group-operation unit. Profile/instruction results
are not native throughput, cycles or an admitted wall-time ratio. No isolated
clock is required to count instructions; no native-time value is primary.

A/A compares identical baseline code in different interleaved slots. Acceptance
per cell requires identical answers/witness order in every row, all five rows
per arm, and candidate median Ir at least 5% below baseline while baseline A/A
median difference is <=1%. Otherwise retain the descriptive values and label
the cell inconclusive/invalid as applicable. Show min/median/max and both A/A
values; no p-value, population CI or asymptotic inference. Positive planted and
cancellation cells must Found/replay, uniform cells must agree with independent
negative proofs. Any invalid answer stops the run and retains its prefix.

Pilot: both n83/131 d12 classes, `distinct-planted-1` and `uniform`, all three
arms (12 engineering calls). Positive controls must Found/replay; every query
must be valid. Estimate each field/arm's maximum observed combined call cost
across the two pilot kinds, times its 15 primary calls; add actual fixed pilot
elapsed. Pilot outputs stay separate; they are not substituted for primary
rows. Estimate infeasibility requires a new pre-data protocol, never silent
sample reduction. Dry-run analysis is frozen before primary data.

Execution: existing explicit local exception, one CPU0,2GiB,600seconds combined
pilot/freeze/primary, zero paid resources and retries. The parent flushes rows;
outer timeout preserves any prefix. Default model/allocation caps remain; no
query wall budget affects deterministic comparisons. Independent audit and
regressions are separate capped validation, not free query work. No parallel
build or native timed benchmark runs during primary execution.

Sources: existing Semaev/Diem binary S3 derivation, native half-trace and
Itoh–Tsujii arithmetic; Montgomery simultaneous inversion as already used by
koblitz_fast_arith.rs and wide_gf2m.rs; conventional packed binary elimination.
KB unavailable (no loaded tool, wrapper127); source/repository fallback recorded.
No mathematical novelty claim. External M4RI/FLINT/Magma comparison is not
provided by this old/new full-query comparison; it does not identify the fastest
external implementation. Existing leaderboard/benchmark admission gates remain.
