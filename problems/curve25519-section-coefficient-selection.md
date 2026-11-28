# Target-conditioned section coefficients within a cold search budget

- **Status:** investigating
- **Kind:** algorithm/proof gap; query implementation gap; scoped derived obstruction
- **Affected work:** [section-search derivation](../research/curve25519_torus_native_20261115/section-search-v1/DERIVATION.md), [parent decomposition problem](curve25519-torus-four-point-decomposition.md)
- **Next action owner:** Curve25519 section-search continuation
- **Last updated:** 2026-11-17

## Problem

The structured membership/restriction query can be specified for supplied
section coefficients, but no target-conditioned coefficient constructor has
an established complete attempt/yield bound. Its cost cannot be omitted from
the proposed exponent change. The growing family and original private-tag,
kernel/recovery and raw-row contracts remain unchanged. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** two fixed Rust residual-identity records and the one existing-law
  Lean unit-cancellation endpoint have narrow checks. PCC-20261117-e20e6b and
  the study's lean-check files; no quotient query or coefficient solver was run.
- **Derived:** norm-sextic support tests and a geometric interval-product
  recurrence; full written assumptions/costs in DERIVATION.md. Uniform c4=1
  seeds have hit probability at most binomial(M+5,6)/p^5, thus O(p^-4) at
  M=O(p^(1/6)). This rejects only that sampling model under the divisor-count
  assumptions; the actual geometric counting injection has no complete Lean
  proof or independent signed verdict. [pcc:unverified]
- **Unknown:** whether a biased algebraic constructor meets the required budget.
- **Missing:** native finite-free-algebra query/extraction implementation;
  admissible family and useful-yield bounds; original-object formal bindings.
- **Not established:** a universal exclusion, full-base root, changed exponent,
  favorable scaling sample or query timing.

## What would unblock it

- [ ] Give a target-conditioned public-input coefficient algorithm without
  planted roots, caller-only indices/logs or an uncharged triple table.
- [ ] Bound ALL trials C_seed, setup, failures, extraction, original replay,
  memory, private selection, kernel and recovery on the declared growing family.
  At k=B^theta and C_seed=B^chi, the proposed query interface requires
  chi+theta/2<3 and every other cold phase below the same target.
- [ ] Return more than B usable common-column privately tagged original rows,
  with the unchanged acceptance criteria; no new full-rank-row assumption.
- [ ] Independently validate the actual algebra/maps/implementation and bind
  the formal objects. A unit-cancellation endpoint is insufficient.

These conditions would close the coefficient/search bridge for this precise
route, while downstream complete-solve requirements still apply.

## Next discriminating step

Implement the explicit quotient-product and interval restriction in Rust under
a separate prospective qualification; compare exact direct products on fixed
controls, including nonreduced rings and zero divisors, then lift original raw
witnesses. This is development work, not a scheduled solver campaign. A fast
query with wrong support/signs is rejected; a correct query advances only that
component. Slow scalar work changes its cost bound; an infrastructure refusal
does not reject the mathematics. Recheck on that implementation or a concrete
biased-constructor lemma. Existing experiment/paid-run limits remain in force.

## Resolution and history

Unresolved. The uniform sampler is the narrowly rejected route instance;
recording this problem or merging its PR does not solve the constructor gap.
