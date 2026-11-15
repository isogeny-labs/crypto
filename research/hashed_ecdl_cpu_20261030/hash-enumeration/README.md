# Remove repeated CPU hash-base enumeration work

## What was done

Inverse-hash listing now starts with z(0)=alpha^-1*(-beta) mod s and advances
with z(t+1)=z(t)+alpha^-1 mod s. It retains identical decode attempts and sorted
points while replacing tau modular products with one initial product and tau
bounded additions. Hash inverses themselves remain computed independently.

The default CPU base-list builder and each independent certificate replay now
validate a repeated consecutive hash modulus once per invocation. Every hash
still checks its full parameter bounds. A changed modulus gets a fresh prime
test; no state crosses calls, collection or certificate replays. The public
standalone HashBase::enumerate API still performs its own complete validation.
A private enumerate_validated helper is reachable only after these checks.

This changes the verifier's base-reconstruction call; the verifier is no longer
byte-identical to the prior candidate. It still reconstructs bases itself,
checks every relation, dependency and recovered scalar, and both full replays
remain charged. No oracle search, target, random draw or certificate layout
changes. Root crypto/CUDA was not compiled locally under the disk constraint.

37 standalone release tests, strict all-target Clippy, formatting and whitespace checks pass. New tests exhaust all multipliers/offsets at
three small hash primes with thresholds 1,half and full field, comparing with
direct u128 modular products and Fermat inverses. A certified 40-bit field
exercises wide hash arithmetic. Grouped listing is compared with standalone
listing for repeated/changing moduli and invalid moduli, multipliers, offsets
and thresholds before or after valid hashes. All archived certificates and
existing independent curve/group/dependency checks still pass.

## Decisions and references

Reviewed the supplied Akshima paper, Section 3.5/Lemma 13 factor-base listing:
z=alpha^-1*(t-beta) mod s, followed by compressed point decoding. The recurrence
is its exact first difference, not an alternative factor base or approximation.
The existing shared source implements the same defining membership predicate;
the complete-group-scan reference remains in the tests. This is standard
arithmetic reuse, with no novelty or asymptotic improvement claim.

All residues in the recurrence are canonical and validated s<2^63, so the sum
is below 2^64 and addmod's single correction is exact. The primality reuse
stores only Option<u64> on the stack; there is no new persistent or heap cache.
All setup and both independent replay invocations remain inside complete cost.
Source-level counts are not measured instruction counts or latency results.

## Open threads

No timing exists for this candidate or the previous Curve-context candidate.
Round 17 failed its scheduling deadline and is preserved without a retry.
The next study must retain original baseline `2bd4588fba60`, all three fixtures,
setup, discarded trials, both certificate checks and serialization, while
separately identifying the two currently unmeasured implementation steps.
The current EKS worker is occupied; do not duplicate the failed job or alter
its frozen sample counts. Further 10x, strict isolation, independent scientific
review, external/rho comparisons and largest feasible CPU scale are unproven.

## Errors and ownership

T-946 reserved the source and this handoff directory before editing. No
compilation or release-test failure occurred. Prior evidence remains immutable.
