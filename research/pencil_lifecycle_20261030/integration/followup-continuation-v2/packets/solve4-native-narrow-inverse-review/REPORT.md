# Independent native narrow inverse value review

Accepted the scoped value bridge at published revision 378d3fcfd5b9dfa22e421322fec7391907fa17d8. Ten theorem declarations, seven producer controls and four independent controls passed; all 21 named axiom readbacks used only the permitted axioms.

Observation: one 300-second software wrapper completed in 17.024424seconds with six Lean invocations. Narrow, Controls, Readback, OwnControls and OwnReadback passed. The deliberately false inverse 3 modulo 7 equals 4 was rejected by decide. Three malformed axiom-audit inputs were rejected. All 28,785 recursive dependency/toolchain hashes matched before and after.

Source comparison: native raw_mul uses 64-bit multiplication then remainder. The literal power loop tests lowbit equality to one, conditionally updates its accumulator, squares unconditionally including the final iteration, then shifts right. The Lean model matches those value transitions; 64 fuel covers every UInt64 exponent. Canonical operands with p at most 2^32 prevent product overflow.

Inference: prime p, canonical nonzero input and p at most 2^32 yield a canonical result whose field decoding is the inverse. The inverse is derived rather than supplied as a premise. The power theorem itself does not require primality. Independent checked controls show failure without nonzero, primality or enough fuel, and validate the inclusive 2^32 multiplication boundary.

Limitations: Ops.i effects and counter overflow are excluded. This supplies an inverse-value witness but does not add native pivot, swap, prefix propagation or full Gaussian-array-loop composition. Source correspondence is a manual fidelity review, not a compiler-correctness proof. Shared pinned mathlib/toolchain caches remain trusted; no independent binary/kernel rechecker was available. Pre/post hashes do not exclude transient edits.

Independence and custody: review plan was frozen before reading producer proofs and replay began after root confirmed publication. Graph/comparator inputs were copied privately. Exact source, commands, inventories, logs and audit results are archived in replay-evidence.tar.gz. No runtime qualification or scientific trajectories were executed. Next work is composition into the reviewed normalization call under its remaining hypotheses.
