# Classical F3 discriminant and explicit lift

Formalize the standard integer-polynomial discriminant identity and the
explicit field-coordinate square root it supplies when the target point's
y-coordinate is nonzero. The exported existence theorem concerns points
over the coefficient field. It is not a new summation polynomial, proof of
an IC speedup or a completed elliptic-curve group-addition bridge.

Use the already cached Lean 4.34.1/mathlib revision from the earlier study.
Freeze the source and transport before the first check; preserve every failed
attempt separately. One Lean thread/CPU, 3,072 MiB managed memory, sampled
4 GiB resident guard and 600-second deadline, as in the earlier resource
amendment. No hard virtual-address cap: Lean maps cached proof objects.
Record source/tool/dependency hashes and run the existing native axiom auditor;
no sorry, trusted declarations or native_decide acceptance.

The formula suggests two separate experiments: remove a redundant curve-mask
condition for arity-two finite boxes, and replace exhaustive sign search by
backward propagation through actual affine prefix roots. Each still needs
its own frozen implementation, finite controls, exact-output and cost checks.
