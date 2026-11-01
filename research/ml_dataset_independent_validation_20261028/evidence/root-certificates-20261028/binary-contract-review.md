# Independent consumer binary-root contract review

2026-10-28. Read-only review by the binary-root producer implementer of the
separately implemented consumer checker. This is an additional contract review,
not a new independent execution or replacement for the consumer's own gates.

Reviewed files in `/home/ec2-user/crypto-ml-consumer-root-certificates-20261028`:

- `research/ml_dataset_independent_validation_20261028/native/src/roots.rs`,
  Git blob `7b526075a04a755a3f657852738f1ba41f45f741`.
- `research/ml_dataset_independent_validation_20261028/native/src/audit.rs`,
  Git blob `84e914c0c7d4bd3afaf3752e1bf2a5e5497093ee`.

Both reviewed working-file blobs exactly match commit `25e863fe` (checked with
`git hash-object` and `git rev-parse` on that commit's paths).
Producer comparison source was clean runtime commit
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`. The reviewed Cargo manifest has no
producer dependency. Related portable field/curve and factor-base binding
helpers were inspected where called.

No actionable defect was found in the requested binary binding, zero-polynomial
or censoring scope. The review checked:

1. Recursive sorting of object keys before compact JSON hashing binds the full
   public instance, actual factor base and CaseConfig, while preserving arrays.
2. Canonical polynomial-basis field and irreducible modulus, exact target and
   binary S3 coefficients, deterministic signed-domain x order, and native call
   limits are checked against that public context.
3. The independent monic gcd/product identity checks distinct finite-field
   roots. Zero polynomials use explicit all-field semantics with no finite
   partial root list, followed by bounded actual-domain lift checks.
4. An unacknowledged Finish remains censored. Censored native calls have no
   final roots or lift checks. Missing later queries/lifts require justified
   cap exhaustion; byte-capped sink failures require the precise declared
   cause and an event that would exceed the aggregate envelope cap.
5. Query/lift work and acknowledged event/byte totals bind to both certificate
   and outcome. Completed per-call roots do not imply whole-cell completion.
6. Recognized per-call JSON artifacts must exactly equal the certificate query
   records. Unsupported/legacy absence is not promoted to verified roots, and
   recognized root outcome metadata cannot survive certificate omission.
7. Execution completeness, total cost, subgroup order, factor-base construction,
   UNSAT and fall-degree claims remain false or unverified.

The consumer intentionally does not verify every non-field intermediate event
identity. Its `all_event_identities_verified=false` and execution/cost flags
accurately preserve that limitation; supplied-event counter agreement is a
consistency check. Algebra-disabled checks do not promote root verification.

No source edits, builds, tests, fixture generation, cloud jobs or S3 writes were
performed for this review. Existing consumer test/fixture results were reported
by their owner and were not independently rerun here. One initial source search
used the sparse consumer checkout for producer `src/ml_dataset` files; it
reported missing files and was corrected to the producer checkout.
