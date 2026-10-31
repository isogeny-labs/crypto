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

The next useful method is not a full-order n=83 BSGS run. It is a complete
small-field destination index-calculus adapter with relation, matrix, and
individual-log stages, tested against the existing destination generic control
before any exact-family resource gate.
