# Independent root-set acceptance contract

Design recorded before implementation, 2026-10-28. Work is on
`codex/ml-consumer-root-certificates-20261028`, based on consumer runtime and
evidence head `03dfdeca1a5a2d376eefdc7bfc222c292fda19ba`; PR2776 is unchanged.
Conductor T-745 reserves only this consumer research subtree. No scientific
run, cloud worker, IAM change or budget increase is authorized by this work.
The campaign's USD200 total ceiling remains unchanged.

## Sources and algebraic criterion

Read the frozen prime producer's `src/cryptanalysis/prime_root_trace.rs`,
`src/ml_dataset/solvers.rs`, `docs/ml-dataset/PRIME-ROOT-RECORDS.md`, and the
binary producer's `src/cryptanalysis/binary_root_trace.rs` and typed adapter
`src/ml_dataset/binary_roots.rs`. The original design required a committed,
validated schema before interoperability. Its owner subsequently froze source
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`; the retained fixture driver pins its
source-bound binary SHA256 and records zero-generation provenance preflights.

The mathematical reference is [Shoup, second edition](https://shoup.net/ntb/ntb-v2.pdf),
Theorem 17.2 (polynomial Euclid), Theorems 19.5–19.6 (the product of all field
linear factors), and section 20.4.1 (distinct-degree factorization). Over a
validated field of order q, independently compute the monic
`gcd(f, X^q-X)` using bounded polynomial arithmetic. Require it to equal the
product of distinct returned `(X-r)` factors. This certifies exactly the
distinct field roots of a nonzero f, including repeated-root inputs, without
invoking either producer root finder or enumerating the field. Prime inputs
are at most 2^61-1; binary inputs have independently validated irreducible
polynomial basis of degree 3 through 61. Dataset S3 calls have degree at most
two. The generic prime report checker may accept degree through eight.

SEC1 v2 was already the independent consumer's curve-arithmetic source. A
fresh attempt to retrieve its PDF timed out; the existing native field/curve
checks are reused rather than changing their formulas in this task.

## Acceptance and compatibility

- Require a unique recognized JSON root certificate on an eligible root/m=2
  cell with public input and supported factor base. Reject orphan artifacts,
  mixed family roles and outcome root-claim fields without a certificate.
- Validate public/config/curve/field/twist and factor-base policy bindings,
  explicit ordered signed points, target, work caps, sorted unique x queries,
  reconstructed S3 coefficients, report input and terminal state, per-call
  seed when prime, lift-prefix records and aggregate outcome consistency.
  Binary certificates additionally bind hashes of the full public/base/config
  values. Prime v1 has no full-object binding hash: its certificate establishes
  the mathematical input bindings actually present, not uncommitted annotations.
- Root-set algebra and finite-domain lift arithmetic run when the existing
  independent-arithmetic option is requested. Structural admission and outcome
  binding still run without that option. Output distinguishes not requested,
  absent legacy evidence, censored reports, and verified completed root sets.
- The zero polynomial means all field elements; verify its explicit terminal
  semantics and check only the supplied finite factor-base x domain for lifts.
  A zero polynomial with an ordinary empty-root completion is rejected.
  Censored reports never establish root absence or completion. Completed calls
  preceding censorship retain per-call verification without promoting the cell.
- Older root-polynomial artifacts and explicitly unsupported cells remain
  readable and unverified. A known certificate role with a malformed or unknown
  schema fails closed. Missing old certificates never become verified by default.

## Claim limits and validation plan

New labels report completed distinct-root sets and finite-domain positive
equations only. Keep execution-trace completeness, all supplied event identities,
total operation counts, subgroup order, factor-base construction, UNSAT,
first/last fall degree and producer-training-export equivalence unverified.
Accounting consistency does not establish the actual instruction trajectory.
Root summaries belong in labels/provenance and never in public static features.

Native regressions cover omitted/extra/duplicate roots, invalid fields,
noncanonical coefficients, wrong target/config/domain/outcomes, reordered or
omitted queries, false finish/censor/zero claims, and rehashed substitutions.
Check known root sets against exhaustive small-field test oracles, including
even/odd binary degrees and repeated roots. Replay the existing sealed prime
fixtures; obtain pinned sealed binary fixtures from the producer owner. Preserve
the v1 and policy compatibility tests and record exact source hashes.
No Cargo invocation or PR push occurs until the parent coordinates the test
slot and native gates. Design and finished checkpoints persist in canonical S3.
