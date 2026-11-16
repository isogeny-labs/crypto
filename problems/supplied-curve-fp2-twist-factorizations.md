# Supplied-curve extension-field twist factorizations

- **Status:** `open`
- **Kind:** incomplete integer factorization within the recorded computational budgets
- **Affected work:** [supplied-curve assessment](../research/curve_candidates_dissect_20261021/README.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-21

## Problem

The assessment cannot yet supply complete prime factorizations for two
quadratic-twist cardinalities over `Fp^2`. The exact models, supplied
generators and ICV1 identities are in the frozen study records; the affected
records are selected by the input labels below. No other field or curve is
substituted.

The precise cardinalities, proven prime factors and residual integers are in
the study's [continuation summary](../research/curve_candidates_dissect_20261021/evidence/continuation-summary.json):

| Frozen input label | Trait | Proven-composite residual size |
| --- | --- | --- |
| `P224-candidate-5736` | `twist_order`, `deg = 2` | 327 bits |
| `P256-candidate-876` | `twist_order`, `deg = 2` | 444 bits |

## Evidence and uncertainty

- **Observed:** upstream default factorization timed out. Follow-up used
  120 seconds of PARI and 120 seconds of GMP-ECM per distinct integer target,
  followed by a 120-second FLINT attempt on each exposed residual. The
  exact per-method outputs and deterministic ECM curve parameters are in
  the [frozen archive](../research/curve_candidates_dissect_20261021/evidence/curve-assessment-evidence.zip).
- **Derived or proved:** recovered factors are proven prime and their
  product with the retained residual equals the exact target cardinality.
  Both residuals are proven composite. The base-field curve and twist
  orders have complete proof-enabled certificates.
- **Suspected explanation:** no claim about the undiscovered prime sizes
  or the difficulty of a different factoring method is established.
- **Missing result:** complete prime decompositions of the two explicit
  residual integers.
- **Limit:** a timeout and an unsuccessful bounded search do not establish
  a mathematical obstruction, security, or a factorization lower bound.

## What would unblock it

- [ ] Factor both retained residuals into primes, keeping any repeated factors.
- [ ] Prove primality of every resulting factor and check exact products
      against the recorded residuals and the full twist cardinalities.
- [ ] Add a versioned supplement with source, input hashes, commands,
      resource limits and actual outputs; keep the existing snapshot intact.

These conditions complete the two auxiliary trait records. They do not
establish an ECDLP attack, a runtime advantage, or a statistical outlier result.

## Next discriminating step

Use a repository-native factoring path on the two fixed residuals, selecting
between an explicitly bounded larger ECM campaign and a size-appropriate
quadratic- or number-field-sieve implementation. Freeze the execution
protocol and limits before a new run. Reuse a completely factored target in
the archive as a product/primality control; do not replace either residual
with an easier integer. Record successful factors, bounded failure or an
infrastructure failure separately. This step is planned, not executed by
the report-import PR; no new campaign cap or runtime estimate is asserted.

## Resolution and history

2026-10-21: imported the frozen external assessment and its independently
checked continuation. Three original missing parameter results were
recovered; these two remain partial. Unresolved.
