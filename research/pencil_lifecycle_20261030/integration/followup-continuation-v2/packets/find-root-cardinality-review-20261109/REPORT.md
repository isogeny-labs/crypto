# Find root cardinality source review

Six prospective declarations follow the existing GraphFind.Run interface and preserve root-map fibers and size equality at actual roots. No compiler or axiom readback was run by this reviewer.

- Run recursion matches existing constructor: recursive call updates t before final compress.
- Edge-constant map preservation uses induction and parent edge equality. Root preservation excludes self redirect via strict rank decrease.
- Returned root equality follows fixed-label-at-root and preserved labels; run_exists supplies actual endpoint existence, avoiding arbitrary-label interpretation.
- RootSizes restricted to actual roots; same map fibers and unchanged size array transfer equality. No all-node size claim.
- Native new/node/find source records initial singleton sizes and parent/potential-only find writes; operations counter changes are outside model.
- Word decoding/run_refines, insertion/union component disjointness and overflow remain explicit future composition.
- Main source1aea703... and all seven provided prior source bindings match inspected bytes.

The initial joint-premise control gap is resolved: composed_preservation provides rank=i.val, zero values/target, initial RootSizes and applies the main theorem to the actual two-step compression. Six main declarations and eight controls now have fourteen prospective readbacks. The stale child counterexample correctly avoids claiming size equality at nonroots.

The literal word Run bridge, initial/insertion invariants, union disjointness and full dispatcher/counter costs remain separate integrations. No main theorem statement changed.
