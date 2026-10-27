# Raw degree versus degree-fall closure: a fixed diagnostic

Prospective diagnostic, 2026-10-25, prompted by the completed first phi
case's zero certificates at raw degree <=8. Both larger registered studies
continue unchanged under their managed owners. This probe neither replaces
those workloads nor repeats an old F4, point census or DLP execution.

## Question and fixed inputs

Is the bounded failure explained by the raw degree limit, or can a closure
with smaller intermediate polynomials construct a valid certificate?
These are two possible explanations to test, not established explanations.

Use p37 seed 0, both phi and doubling, and the first three canonical system
indices 0,1,2 in each original input pack. These six systems are chosen by
stored index, without selecting new outcomes. Index 0 is the system whose
raw degree-8 polynomial matrix was observed to have 140 rows, 165 columns
and rank 134. That is a polynomial matrix, not the factor-log relation
matrix. This is an exploratory diagnostic of a bound after observing a
failure, not an independent secret sample or a population study.

Freeze the exact six equation-only packets and references before production.
The producer receives p, peeled a, original four polynomials and context;
no prior root, hit, original cost or degree-8 output is included. The
post-run auditor receives the immutable references and prior degree-8
records separately. Preserve the original public/pair/polynomial bindings.

## Registered modes and controls

For each of the six systems execute both modes, first plus one deterministic
repeat: 12 configurations, 24 producer executions, 12 independent first
audits and 12 parity checks. Keep powers k=1,2,3 in their original order.

- `raw16`: the original monomial-multiple method at total degrees 4..16.
  Through degree 8 its per-degree rows, columns, rank and arithmetic counters
  must equal the prior completed output on that exact system. A difference
  is an implementation failure, not a new improvement.
- `closure8`: intermediate polynomial degree 4..8. Reduce initial monomial
  multiples, then multiply each newly found lower-degree basis row by each
  variable when the intermediate degree stays within the bound. Track
  every representation over the original four polynomials. Allow original
  product degree <=16. Refuse a generated representation that exceeds that
  product bound, count the refusal, and keep the search bounded. Complete
  the bounded closure before testing powers in order.

Report the working degree and the original product degree separately.
A closure at working degree 8 is not called an original-product-degree-8
certificate when its multipliers require a larger bound. A failure at the
two caps establishes no nonexistence result.

Both modes use exact sparse coefficient expansion to verify every returned
identity q_a^k=sum(h_i f_i). Reuse the already checked conditional rejection
lemma and universal cubic IDC with their precise remaining native/field/
group bridges. The auditor checks the retained complete field roots and
requires no accepted physical relation in a certified system. The first
three systems are not assumed to be representative or all unproductive.

Synthetic tests cover a certificate requiring a power, a distinct-root
system where rejection must fail, tampered coefficients and a closure that
raises original product degree while keeping intermediate degree lower.
Exact first/repeat parity excludes only raw diagnostic clocks.

## Accounting and decisions

One worker, common busy reservation, 8 GiB per execution, 1,200-second outer
watchdog and 1,180-second cooperative producer deadline. Compile separately
with one worker. Retain actual launch receipts after the queue wait,
commands, sources, hashes, every attempted degree, rank, original-product
degree, cap refusals, successful/failed work and partial/pending events.
An interrupted phase is not automatically repeated with a fresh budget.

Count all construction and sparse-check multiplications/additions/inversion
calls. Inversion exponentiation products are already in multiplication
counts. Retain RSS separately; memory and I/O are unpriced. Wall clocks are
shared-host diagnostics. No full fallback, target recovery or admitted
pipeline measurement occurs here.

If raw16 succeeds above 8, the earlier bounded search missed a certificate
at the larger bound on that system. If closure8 succeeds with original
product degree above 8, record that distinction and its actual cost.
Neither result establishes a useful full-pipeline gain. If neither succeeds,
the power cap, finite-field restrictions or other algebraic components
remain possibilities; do not infer impossibility. Continue the unchanged
full panels and price any later integrated candidate completely.

## Prior-art and scope

The atlas match names `LEAD-ICE-descent-closure-degree-at-scale` and
`KR-IC-fbdb61`; the former already distinguishes raw Macaulay bounds from
MutantXL/F4 closure in binary descent. Its numerical degree/memory results
are not imported into this three-variable prime-field restriction of the
existing quartic-extension study. `LEAD-ICE-prime-field-semaev-solving-degree`
concerns a different prime-field factor-base setting and does not settle
this extension-field diagnostic. No novelty or atlas saturation is claimed.

This copy of the algebraic core comes from the frozen collision producer;
the two live native sources and their seals are not edited. Reports must
show all twelve configurations, failures included, with native reductions,
a graph/diagram and inspected PDF. Publish a separate versioned raw archive.
The original unique-relation, relation-rank, complete-cost, strongest IC/rho,
external-tool and proof obligations remain open.
