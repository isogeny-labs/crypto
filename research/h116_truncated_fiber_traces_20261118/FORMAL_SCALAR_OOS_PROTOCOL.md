# H116 formal-scalar out-of-sample protocol

Protocol id: `H116-FORMAL-SCALAR-OOS-001`.

Status: frozen before selecting alternative curve models, constructing any new
full map, or evaluating the candidate on those models. This is an excluded
control panel; it does not read the blinded H116 target census or scalar labels.

## Question and prediction

Does the unchanged formal-scalar constructor reproduce the first eleven
normalized x-map coefficients of `V-1` on five curve models that were not used
by the constructor or the prior full-map control? [pcc:unverified]

The frozen prediction is exact equality of coefficients 0 through 10 on all
five selected models. The first nine coefficients must also equal the
independently implemented normalized-isogeny recurrence. Any mismatch is
retained and rejects this prediction for the panel. [pcc:unverified]

## Deterministic selection

The five public `(p,r,d)` triples are read from the already frozen
`COHORT_V2.json`, whose SHA-256 is
`29e2fd5596ce3ed8e93933161e05b92229cd3065a892bc8ee59d6265f6fa2d99`.
For each triple, scan short-Weierstrass models `(A,B)` lexicographically by
`A`, then `B`, and select the first model satisfying all of:

1. `4A^3+27B^2 != 0 mod p`;
2. `#E(F_p)=r` by direct base-field point count;
3. its j-invariant differs from the admitted cohort model's j-invariant;
4. it is not any exact H115 excluded fixture;
5. no model is replaced after any downstream result is observed.

Different j-invariants prevent the selected model from being an Fp-coordinate
rescaling of the admitted model. The panel intentionally remains in the same
five trace/isogeny classes, so it tests curve-coefficient generalization but not
new `(p,r,t)` classes. [pcc:unverified]

If no qualifying model exists for one triple, that cell is retained as
`selection_failure`; no new prime, order, model ordering or sample-size rule is
substituted in this run.

## Bound sources and separation

- Candidate source SHA-256:
  `914c1909ffdd5e6b87a01f4c2da9de8faca0187da490cb56af323e105e64b6b4`.
  Its formal-series formulas and precision are frozen unchanged.
- The prior single-fixture oracle source SHA-256 is
  `492d6ee838009672ad81059f59a03cacdd84577079785683f7ff8db93cec8901`.
  A new native multi-curve oracle may reuse its field and Vélu formulas, but
  its final source hash must be committed before the first map run.
- For every selected model, all five candidate prefixes are computed and
  serialized before the new full-map oracle output is parsed by the comparison
  step.
- The independent verifier consumes only retained JSON and does not call the
  candidate or native oracle.

## Frozen controls

For each completed cell:

1. candidate prefix equals the full Vélu quotient through coefficient 10;
2. candidate prefix equals the Bostan recurrence through coefficient 8;
3. the candidate prefix has zero transformed first-order residual through
   degree 9;
4. the Vélu codomain equals `(a^4 A,a^6 B)`, `a=t-1`;
5. the full-map cleared identity passes and a one-coefficient numerator
   mutation fails;
6. replacing `a` by `a+1` and omitting target scaling each disagree with the
   oracle prefix;
7. the independent verifier repeats quotient, recurrence, identity and
   mutation checks.

The existing synthetic p=23/p=19 boundary checks remain mechanism controls;
they are not counted as additional panel cells.

## Measurement and stopping rule

Record exact base-field additions, multiplications and inversions for candidate
construction separately per curve. Record native full-map field operations
separately. Wall time, if emitted for diagnostics, is not a comparison because
the runtimes and workloads differ. [pcc:unverified]

Exactly five selection attempts and at most one full-map construction per
selected cell are permitted. A compile error, missing kernel, non-base-field
map coefficient, identity failure, timeout or mismatch is a retained outcome.
No curve is dropped or replaced. The result is descriptive; no generic or
asymptotic claim follows from five passes.

The canonical `experiment.protocol/v1` package in the sibling
`crypto-autoresearcher` checkout was fetched at `origin/main=d54c01d3` but is
stored as unreadable `VLT1` encrypted data in this runtime. This native H116
protocol therefore records the available pre-data contract without claiming a
canonical-schema validation receipt.
