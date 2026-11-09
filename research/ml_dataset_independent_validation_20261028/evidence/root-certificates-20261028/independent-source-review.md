# Independent source review: consumer root certificates

Pinned consumer commit: `25e863fe6075d7bac795ee62680b72c123ba0f33`.

**No actionable correctness finding was identified in the reviewed source.** This is a read-only, uncompiled source assessment. It is not a statement that tests, saved-corpus replay, CI, merge gates, an EKS run or recovery acceptance have passed. The implementation owner retains the compilation and test lane.

The reviewer used `git show <exact-commit>:<path>` for implementation inspection so concurrent worktree changes could not change the review target. `assessment.json` records the Git blob identities, lengths and SHA-256 digests of reviewed consumer files. No repository files, shared Cargo targets, fixtures, cloud state or other agents' work were modified. Only this external review packet was written. The Conductor advisory identified the implementation owner's reservation; it was respected.

## Arithmetic assessment

`native/src/roots.rs:45-152` implements bounded dense polynomial arithmetic over the consumer's existing field implementation. Canonical coefficient vectors have no trailing zero; the empty vector represents the zero polynomial. Polynomial remainder eliminates the leading term using its field inverse; Euclid normalizes the final gcd to monic form. Modular binary exponentiation obtains `X^q mod f` without enumerating the field.

For each nonzero polynomial, `root_identity` requires sorted, distinct canonical field roots and compares the product of their monic linear factors with `monic_gcd(f, X^q-X)`. This certifies exactly the distinct roots over the declared finite field, including repeated-root and constant-polynomial cases. It does not certify multiplicities, a producer execution trace or arithmetic costs. The zero polynomial is handled separately: all-field semantics require no enumerated partial root list, and the dataset adapter uses only the finite actual factor-base x-domain for lifts.

The unchanged arithmetic core was also inspected where newly reused: deterministic prime checks and u128 modular arithmetic, binary multiplication and Rabin irreducibility, canonical public coordinates, curve non-singularity, binary and prime addition, negation and twist validation. Word limits are enforced before field arithmetic. The standalone prime report permits p=3 explicitly; dataset prime curves retain their existing p>3 constraints. Binary reports support degree 3 through 61; standalone prime polynomials are bounded by degree eight, while dataset calls are reconstructed quadratics.

The root-set argument matches the standard finite-field identity in Victor Shoup, *A Computational Introduction to Number Theory and Algebra*, Theorem 19.6: `X^q-X` factors into the distinct linear factors corresponding to all field elements. Theorem 19.10 gives the broader factorization and square-freeness statement; Theorem 17.2 supplies polynomial Euclid. These were checked in the [primary text](https://shoup.net/ntb/ntb-v2.pdf), not treated as a novel result.

## Binding and termination assessment

| Requirement | Source assessment |
| --- | --- |
| Field, curve, target and config | `public_curve`, `policy::bind_base` and `roots.rs:704-817` validate the field and selected curve, eligible m=2 root solver, requested/effective work limits, target, family and policy context. Binary records additionally bind recursively sorted public/base/config JSON hashes. |
| Actual finite domain | `roots.rs:566-597` validates finite on-curve points, rejects duplicate original points, reproduces signed-domain ordering, sorts unique x-values and reconstructs the appropriate binary/prime S3 coefficient recipe. |
| Every supplied call | `roots.rs:833-882` binds index, fixed x, coefficient vector, work prefix, limits, prime seed and native report polynomial. `audit.rs:741-787` requires exactly one recognized aggregate certificate and a matching fixed-name native-call artifact for each query, with identical parsed JSON. |
| Lift prefix and positive witness | `roots.rs:600-639,923-965` replays finite-domain group additions when arithmetic is requested and reproduces the producer's bounded lift order and count. The enclosing auditor independently requires a positive PDP certificate whenever PDP success is claimed. |
| Omitted work and censorship | `roots.rs:966-1005` rejects unjustified omitted roots/queries and binds aggregate found/censored/completed flags and counters to the recorded outcome. A native incomplete report cannot supply final roots or lift checks. Unacknowledged finish events are still censored. |
| All-field result | `roots.rs:345-388,913-921` permits all-field semantics only for a zero polynomial, then restricts dataset lift enumeration to the actual finite x-domain. It does not enumerate the field or assert a decomposition. |
| Legacy records | `audit.rs:722-738` retains legacy/unsupported evidence as explicitly unverified, while rejecting new root outcome claims or native-call artifacts that lack a recognized certificate. |
| Labels and features | Root verification results enter labels. The existing static feature allowlist and versioned split/view handling remain separate; this patch does not feed root results into static routing features. |

The prime adapter was compared against producer `d71ec2ef5ab9b7f2a038f61c941af10297a775ad`, especially `src/ml_dataset/solvers.rs` root accounting, limits, seed and recursive lift. The binary adapter and signed-domain/lift behavior were compared against producer `d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`, especially `src/ml_dataset/binary_roots.rs` and `src/ml_dataset/solvers.rs`. No coefficient, limit, query-order or stopping mismatch was identified by source inspection. This comparison does not execute either producer.

## Hostile-input and claim boundaries

Recognized aggregate, query, report, limit, begin, finish and field-operation records have exact-key checks. Unsigned numeric types, canonical decimal strings, bounded field words, polynomial degree, sorted unique roots, event count and work bounds are validated. The JSON loader rejects duplicate keys recursively and bounds JSON bytes; the auditor checks artifact paths, sizes and hashes before trusting parsed content. Dataset event-envelope bytes are recomputed against the native 16 MiB cap, and byte-cap failure must include the precise unacknowledged envelope that would exceed that cap.

A rehashed wrong root list cannot pass the independent gcd/product identity when arithmetic is enabled. Changing fixed x, coefficient recipes, target or bounded query/lift prefixes encounters independent bindings rather than merely an outer checksum. Binary full-object bindings use recursive map sorting explicitly because the consumer JSON build preserves insertion order.

The following are deliberate limitations, not defects against the stated contract:

- Most non-field intermediate event payloads are recognized and tallied but their equations, dimensions or execution order are not independently reconstructed. Separately persisted intermediate artifacts are hash-checked, not semantically replayed against every native event. A self-consistent incomplete event narrative therefore does not prove actual execution. `all_event_identities_verified`, `execution_trace_complete_verified` and `total_operation_count_verified` stay false.
- Reported tag counts, work sums and adapter prefixes are consistency checks. They do not prove that unreported producer work was absent, that every operation was recorded, or that these mixed admission units are measured costs. `reported_accounting_consistent` must retain that narrow meaning.
- Prime v1 does not carry binary-style full public/base/config hashes. It binds the mathematical inputs and frozen adapter inputs that its schema exposes, plus the enclosing dataset provenance. `full_object_hash_binding_verified` is correctly false for prime records.
- `all_supplied_root_sets_verified` is scoped to supplied completed native calls. It can be true even when later lifts or remaining domain queries are censored. Consumers must also inspect `cell_censored` and `all_domain_queries_completed`; the field is not an UNSAT or whole-cell completion label.
- Arithmetic-disabled audits retain structural bindings but grant no independent root-set verification. Positive root/PDP evidence does not verify subgroup order, factor-base construction, general solver execution, scalar recovery, last-fall degree or an ECDLP pipeline. These flags remain false.

No source change is requested by this review. Before treating this commit as validated delivery, use the implementation owner's exact-commit compilation/unit tests and saved-artifact interoperability results, including completed and censored prime/binary/Koblitz cases, both twists/parities where applicable, rehashed mutations and arithmetic-disabled labels. Those executions were explicitly outside this review and have not been claimed here.

## Review execution record

- Read exact consumer source, root design and test source; reviewed relevant producer source pins.
- Read repository instructions/session-log tail and the local `report-evidence` skill; no session-log write or PR mutation because this delegated lane is read-only.
- Checked primary finite-field literature as cited above.
- No Cargo, test process, producer invocation, fixture generation, cloud query, launch, IAM operation or repository edit.
- Saved this review, machine-readable assessment and checksums outside the source worktree.
