# How to assess and measure an elliptic-curve transfer

This is the operational decision guide for covers, descents, isogenies,
Jacobian correspondences, abelian varieties, and lattice representations. It
prevents a valid map from being mistaken for a faster discrete-log algorithm.

## Choose the workstream by the question

| Question | Use | Required output |
|---|---|---|
| Does the proposed curve, cover, surface, or lattice map exist? | algebraic screen | dimensions, fields, equations, degree/different or Riemann--Hurwitz, and Jacobian/polarization status |
| Does it transport the intended prime-order subgroup? | transfer assessment | executable forward/recovery maps, image order, kernel intersection, exceptional fibers, and scalar-recovery certificate |
| Can a destination algorithm actually consume the image? | solver applicability audit | named solver, image representation, relation/search compatibility, individual log, and source replay |
| Is the destination intrinsically easier? | destination-weakness analysis | solver exponent and constants on the image, smoothness/factor-base model, relation yield, matrix/descent cost, and scaling controls |
| Is the whole route faster? | three-arm measurement | source reference, destination generic control, destination candidate, common costs, cold route overhead, A/A, and replay receipts |

Do not skip a row. A supported row permits the next question; it does not
answer it.

## Destination weakness diagnostic

Measure the destination at three levels.

1. **Reachability.** Prove that the transported generator and targets lie in
   the domain actually handled by the solver. For a Jacobian this means
   canonical divisor classes, not source points carried beside them.
2. **Algorithmic advantage.** Compare the candidate to a generic solver in the
   *same destination group*. A useful candidate changes an exponent, reduces a
   demonstrated finite-size cost, or unlocks a structural quotient. Generic
   BSGS beating generic rho with memory is a control result, not a weak-group
   result.
3. **Net advantage.** Charge route construction, pullback/descent, solver
   setup, relations/walk, linear algebra, individual log, source recovery, and
   verification. Compare the complete candidate to the best applicable source
   reference on identical public targets.

The minimum three arms are therefore:

```text
source_reference
destination_generic_control = route + generic solve on the image
destination_candidate       = route + proposed faster solve on the image
```

Add a source A/A arm for wall-noise diagnosis. If precomputation is reused,
publish both cold one-target and explicitly amortized results, including the
break-even target count.

## Units and promotion gates

Keep unlike units separate until a conversion is certified. Source elliptic
additions, genus-three Cantor additions, field multiplications, table probes,
matrix operations, bytes, and wall time are not interchangeable. A zero in an
unpriced source-GAE column is not free destination work.

A net speedup is promotable only when:

- every answer independently replays on the source;
- every material phase is priced in one versioned unit;
- at least four faithful sizes support the claimed scaling range;
- deterministic records replay and failures remain in the panel;
- wall claims earn the stated isolation tier and pass A/A controls; and
- the source/candidate complete-cost ratio and interval support the claim.

Otherwise report a stage result: valid transfer, solver reachability,
destination-native comparison, or descriptive wall observation.

## What this n=83 study establishes

The exact transfer and generic solver reachability pass. The four successful
toy sizes are `n=9,13,19,23`; `n=7` is retained because both source reference
copies exhausted while both destination solvers verified. Destination BSGS is
the better of the two generic destination implementations in native operations
and descriptive wall time. It does not establish a weaker destination DLP:
both methods retain generic square-root behavior, no index calculus is attached,
native work has no source-GAE conversion, and the wall evidence is L0.

The clean six-round panel strengthens the practical decision without changing
that evidence tier. All 896 records verified and all 768 measured records
replayed. Paired median BSGS/source wall ratios rise from 5.36 at `n=9` to
131.32 at `n=23`; rho/source rises from 9.82 to 364.98, while A/A remains within
2.4% of one. These L0 ratios reject an exact generic `n=83` attempt but do not
constitute an admitted cross-host performance result.

The standard ambient genus-three index calculus is also not the next useful
method. For this order-`r ~= q` image, its `q^(4/3)` work is about 70.17 exponent
bits above source rho at `n=83`, and ambient factor-base relations have no
proved projection into the image. A future destination calculus should be
implemented only after it defines an image-closed factor base or projector,
measures relation yield for image samples, completes matrix and individual-log
stages, and predicts cost in both `q` and `r`.

Use the methods and artifacts as follows:

| Need | Use | Do not infer |
|---|---|---|
| Certify the exact cover and subgroup | exact certificate and typed pull/push checks | destination weakness |
| Check that a solver reaches the image | `jacobian_image.rho3` or `jacobian_image.bsgs` | asymptotic improvement |
| Count complete-route destination field work | the separately named `_field_tally` adapters | wall time from those instrumented runs |
| Compare end-to-end wall behavior | untallied three-arm spec plus A/A at L2 or better | a speed claim from this local L0 session |
| Admit a non-generic destination calculus | the six gates in `DESTINATION_CALCULUS_SCREEN.md` | ambient-Jacobian heuristics applied to the strict image |

The next measurements are the already frozen untallied spec, once on each
registered `c8a.2xlarge` and `m8a.2xlarge` EKS Spot class. The next algorithmic
work is a paper design for an image-compatible relation calculus; it should not
receive implementation or exact-family compute until its exponent and
reachability gates pass.
