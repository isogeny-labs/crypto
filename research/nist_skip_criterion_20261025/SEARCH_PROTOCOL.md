# Search for the unexplained prime-curve skip rule

2026-10-25. This is an exploratory search. The 21 eligibility labels and
earlier four- and six-curve analyses are already visible. This file freezes
the next search families before their new feature values are calculated;
it is not a retrospective claim of preregistration or blinded inference.

## Objective and population

Find an explicit, low-complexity criterion that accepts the ten published
prime curves and rejects the eleven earlier eligible but unpublished curves
in the six recorded seed streams. Test the same rule on every cohort. The
four streams with skips contain 19 curves; the P-384 and P-521 streams add
two published controls. Preserve the known eligibility filters and root
conventions. Do not substitute rejection of composite-order candidates for
an explanation of the eleven eligible skips.

An exact empirical separator is a candidate criterion. It is not, by itself,
evidence of historical use, a security advantage or a recovered attack.
Do not declare success by listing the eleven indices, fitting arbitrary
intervals to seeds, or fitting a large decision tree. Distinguish a natural
mechanistic explanation from an overfit separator.

## Inputs and validation

- Retain the public 19-curve parameter file from John Mattsson's October 18
  TLS-list message and Valsorda's independent enumeration source.
- Add the standardized P-384 and P-521 records from the local pinned standard
  parameter catalogue and check all ten published seeds against their source.
- Derive each stream seed natively from `(run, index)`, check the SHA-1/X9.62
  coefficient relation, prove primality, and certify cardinality using a
  nonidentity point killed by the prime order and the unique multiple in the
  Hasse interval. Preserve exact fields, coefficients and orders.
- Preserve the supplied/source model and separately distinguish root-sign
  normalization from the published model. Representation-dependent leads
  need a historically consistent root convention.

## Search families

1. Arithmetic: trace sign and magnitude, bit lengths, Hamming weights,
   valuations and residues of trace, order, coefficients and seed-derived
   values; quadratic characters of these values; small-prime factors and
   smooth portions of nearby orders and twist orders. A partial factor is
   not a prime. Unavailable complete factorization stays unavailable.
2. Frobenius/SEA: discriminant characters at primes through 251, Elkies and
   Atkin counts and coverage thresholds, repeated-eigenvalue tests, and
   factorization patterns of small division polynomials. Distinguish the
   Frobenius order from the actual endomorphism order.
3. Generation artifacts: fixed small-x point-lifting gates; powers-of-two
   x coordinates; points derived from the curve seed, SHA-1 expansion or
   adjacent RNG blocks; FreeLIP first-x gates with the documented generator;
   and explicitly parameterized RNG-consumption models. Label broad bit
   scans as exploratory controls rather than historical evidence.
4. Selection logic: single natural predicates and thresholds; short AND/OR
   combinations or decision lists. Report every mismatch and the complete
   family searched. Candidate-index or curve-name lookup is excluded.

## Protection against fitting the labels

- Deduplicate equal feature masks and retain their definitions.
- Measure the best fit and complexity on all recorded rows, and show
  within-stream errors; a pooled score alone is insufficient.
- Enumerate the permitted reassignments with the published counts fixed in
  each stream: 3 of 8, 3 of 5, 1 of 2, 1 of 4, and the two forced controls.
  Reapply the same search to these assignments where feasible. Calibration
  refers only to this declared search family and exchangeability model.
- Any proposed criterion must state an independent prediction or source
  evidence that could distinguish its historical use from coincidence.
  Fresh randomly generated streams have no historical chosen/skipped labels;
  they can measure a rule's frequency, not validate historical selection.

## Execution and reporting

Use native C++ with PARI/OpenSSL; shell only orchestrates, Python only lays
out recorded results. Keep code, exact inputs, outputs, failed attempts,
commands and dependencies. Separate bounded factoring failures and missing
historical records from mathematical impossibility. No performance or DLP
claim is authorized by a feature fit.

Continue from discriminating results rather than ending at this protocol.
If no credible rule emerges, retain the exact explored families, best
separators and the specific remaining evidence needed to identify the rule.
The original goal remains unresolved. A local protocol or PR alone does not
complete it.
