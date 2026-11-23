# Offline pencil target-stream replay

This package validates one target-local captured pencil stream and replays its exact `Collector` inputs. It uses the producer's shared reporting types, the existing augmented-row adapter, and the existing offline rank diagnostic. Signed-graph and Kummer events are not implemented; unknown conventions are rejected.

```
cargo run --offline -- --expected-header EXPECTED.json STREAM.jsonl
```

`EXPECTED.json` is a separately pinned complete `Header` from the intended workload/run/source/curve/target configuration. Do not copy an untrusted stream's header and describe that as independent identity verification. The stream may be the producer's JSONL or a JSON array of `Record` values. The CLI bounds each input to16MiB; library callers must similarly bound bytes before `decode`. Replay additionally limits records, columns, terms and accumulated diagnostic prefix cells. This is an offline diagnostic and changes the reused kernel's thread-local counters; it must not run in the measured producer or be included as a zero-cost observer.

The loader checks every expected header field, prime/domain/order encoding, fixed identity-order x map and small partition, sequence numbers, candidate order and fixed walk labels, candidate/relation adjacency, exact point-sign/label/RHS row assembly, canonical coefficients and terminal states/counts. It preserves raw term order, duplicate columns and zero terms when replaying `Collector`. Each Admission, pending constraint in small-column coordinates, retained residual rank and scalar candidate is recomputed independently in fresh collector state.

For each relation the output names two distinct prefix spaces:

- `generated_reported_verified_inputs`: all input rows whose geometric verification the producer reports, including subsequently rejected or capped rows.
- `incorporated_inputs`: only Joined, Anchored, Independent and Dependent admissions. RejectedThreeLarge and cap-pending rows never enter this rank calculation.

Both spaces report rank(A), rank([A b]), rank([A b c]), consistency and target identifiability for A*x+b*k+c=0. Only the final augmented RHS column is negated. Retained residual rank and the unincorporated pending constraint are separate fields. A cap may already have mutated graph state; excluding its input does not assert rollback.

A `candidate_scalar` is a coefficient read from the replayed target pivot. `terminal.source_verified` carries the producer's source-group check claim. `scalar_independently_verified` and `geometric_relations_independently_verified` are always false here. Actual subgroup/curve validity, geometric relation proofs, and scalar replay remain separate evidence requirements; checking canonical encodings and equation consistency does not establish them.

`syntactically_complete` means the supplied ordered packet has a coherent terminal record. It does not authenticate successful producer publication: a failed flush may leave terminal bytes, and copied/forged packets cannot be authenticated by their own contents. Use the producer's successful atomic publication and retained execution receipts as independent capture custody. A sequence gap, incomplete JSON record, missing terminal or inconsistent terminal summary is rejected. Interior edits accompanied by forged headers/counters and resequencing require external artifact hashes/custody to detect.

Tests are deterministic algebraic/serialization fixtures. Their coordinates are not presented as verified curve points, and their outputs are not scientific cohort measurements. The original1260-target scope and campaign limits remain unchanged.
