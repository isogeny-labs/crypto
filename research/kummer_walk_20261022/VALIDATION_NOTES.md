# Validation notes

- The native producer read exactly 180 `rho_neg` source records per size class
  from the immutable archive, and emitted 1,260 target rows plus 21 model
  rows. No target row was omitted on a failed algebraic check; failures would
  appear as `all_correct: false`.
- Each target independently checked `planted * source_generator == target`
  before the quotient tests. Each of its seven labelled sums was compared
  against transfer of the corresponding source-curve linear combination.
- The native summarizer reread every JSONL row, checked row-level predicates,
  uniqueness of workload IDs and model primes, all case counts, and all
  output/lift cardinalities. It found 0 target failures and 0 exception
  failures.
- Reexecution of the same release binary into `/var/tmp/kummer-replay`
  gave byte-identical JSONL for all 14 files. This is a deterministic replay
  and does **not** constitute a second independent Kummer-coordinate
  implementation. Source scalar multiplication and Jacobian arithmetic are
  separate correctness controls inside the producer.
- The invariant is proved algebraically in `REPORT.md`; its reference
  implementation tests finite inputs. Explicit fast Kummer coordinates,
  exception-safe coordinate formulas, collector integration, one-target DLP
  solves, and matched rho pricing remain unverified.
- The local uniqueness lemma is a conditional statement about quotient
  classes in an abelian group, not an unconditional polynomial equality.
  `identity.certificate/v1` supports only the latter expression type, so no
  polynomial-identity certificate is used as evidence for this lemma.
