# Matched factor-base policies and native root records

Design recorded on 2026-10-28 before new policy data or validation. This is
implementation work on top of source `12600db8c0dd2359ec3dc626b96542df3a685d06`.
It does not launch a new dataset campaign or amend the frozen 720-cell pilot.

## Purpose and public problem identity

The existing native generator constructs one sequential small-x projected
factor base. Add explicit, replayable alternative populations so future
separately registered datasets can compare bases on the same curve, twist,
generator, target, arity and backend. Selection must not use the known scalar,
target success or held-out solver outcomes. Preserve current whole-field-size
splits and actual solver-domain identities, including PKM and large-prime
adapters that transform the originating base.

Plan/v1 and absent-policy CaseConfig retain their exact serialized bytes,
mixed-radix mapping and legacy construction. Plan/v2 requires a nonempty,
bounded axis of typed recipes. Solver remains the fastest-varying axis, then
policy, then the original axes. Removing the selected policy from a v2 cell
must yield the corresponding v1 cell. A CLI JSON recipe input freezes explicit
parameters; a mere policy label is insufficient.

## Actual populations and limits

Initial recipes are legacy projection, target-independent sampled-x projected
control, binary polynomial-subspace intersection with the selected cyclic
subgroup, and binary model-preserving Frobenius union intersected with that
subgroup. Explicit independent basis masks define subspaces. Prime curves
retain explicit unsupported binary-policy records. General binary models use
their coefficient-field Frobenius power, including a recorded trivial action
when that power is n. No cofactor-projected image is described as a literal
subspace of final x coordinates. Complete sign/orbit units are admitted or
rejected together under a cap.

Save exact ordered points, mathematical recipe, actions, candidate history,
rejections, counters and complete/capped/unsupported status. Verify actual
cyclic membership rather than only r-torsion. Replaying construction checks
the saved point set and stop reason. Counts are observations; deterministic
scan frequencies are not rejection probabilities. Requested size, actual
points, distinct coordinates and signed/orbit columns remain separate axes.

Static feature projection is an explicit allowlist. Public bases, points and
actions are allowed; sampling seeds, membership logs, construction history,
known scalars and future solver outcomes remain restricted. Construction cost
and outcomes belong in label/provenance streams. The exact solved domain,
rather than policy name alone, binds a PDP label.

## Prime root records

Add an opt-in native, bounded root engine for odd prime fields through the
existing u64/u128 range, preserving legacy root APIs and counter meanings.
Expose modular powers, Euclidean divisions, proposed/accepted/rejected splits
and root extraction while computation runs. Bound degree, field work, split
attempts and emitted events; a stopped sink stops the computation. The zero
polynomial explicitly denotes all field elements. Keep incomplete prefixes
censored. Version new operation counts and state inverse accounting; these
are not retired CPU instruction counts.

Independently verify distinct-root completeness using the monic
`gcd(f, X^p-X)` and equality with the product of distinct linear factors. Check
saved polynomial identities without calling the producer root finder. Bind
certificates to the public modulus and actual root problem. Root completeness
is distinct from a full finite-domain PDP search, a scalar recovery, and a
first/last-fall-degree theorem.

The checker verifies each supplied event identity and recomputes the recorded
event tallies. It does not prove that every operation from a native execution
was recorded: valid summary identities can remain after field events are
omitted. The artifact and outcome therefore explicitly mark execution-trace
completeness and total-operation-count verification false. Native producer
counts remain useful recorded measurements; only the distinct-root-set claim
has the independent algebraic certificate in this increment.

## Validation and publication

Use native Rust correctness tests: byte-stable legacy plan/cell mapping;
matched policies on both twists; subspace, membership, closure and cap checks;
explicit prime unsupported cases; independent tamper rejection; small-prime
exhaustive polynomial roots, repeats/irreducibles/zero/constants, constructed
roots at `2^61-1`; exact cap/sink failure behavior; and public-only adapter
replay. Run focused tests, scoped lint and the required full release library
suite after integration. Local validation is software evidence, not a new
scientific corpus. Preserve failed outputs and final logs with source hashes
in canonical S3 and Git receipts.

The USD100–200 total campaign budget through tomorrow, maximum USD200, remains
unchanged. No IAM, quota-mount, cloud-instance, experiment-retry or deadline
change is part of this implementation. Production worker publication and
independent cloud replay remain separate unresolved deployment requirements.

## Sources checked

- Native `src/ml_dataset/{curves,types,mod,export,solvers}.rs` and existing
  Koblitz subspace/Frobenius builders; tracked
  `research/notes/index-calculus/RESEARCH_FACTOR_BASE_SOLVE_COST.md`.
- Galbraith, Granger, Merz and Petit, *On index calculus algorithms for
  subfield curves*, §§2–5,
  [accepted primary manuscript](https://pure-oai.bham.ac.uk/ws/portalfiles/portal/104974339/On_Index_Calculus_Algorithms_for_Subfield_Curves.pdf).
  Its subspace and Frobenius constructions motivate explicit coordinate
  domains. Its subgroup hypotheses and matched experiments preclude inferring
  a general benefit from invariance or dimension alone.
- Shoup, *A Computational Introduction to Number Theory and Algebra*,
  [version 2, §§20.4.1–20.4.2](https://www.shoup.net/ntb/ntb-v2_0.pdf).
  The distinct-linear-factor GCD characterization supports the root certificate;
  randomized splitting requires bounded retries and honest incomplete outcomes.

No novel algebraic method, speed advantage, degree bound or conjecture result
is claimed by this software work.
