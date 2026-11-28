# Conditional affine compression proof

`TwistAffineCompression.lean` proves exact target-value preservation under a
surjective parameterization of the known-constraint kernel and one-edge
substitution over a commutative ring. This is standard affine elimination,
not a novel elliptic-curve or index-calculus theorem.

The frozen replay at source6573036097a0 uses Lean4.34.1 and mathlib commit
`d13f23b723b8a846827a245b89c10fc7d3f11612`. Both lemmas compile and a fresh native
transitive axiom auditor passes: zero failing theorems and zero trusted
`native_decide` dependencies. `substitute_one_edge` has no axioms;
`target_values_parameterization` uses `propext`, `Classical.choice`, `Quot.sound`.
Exact complete receipts are under `receipts/frozen-v1/`; the prior development
receipt is retained independently. The large native auditor is managed by the
Setec artifact manifest, with exact binary/source hashes.

The formal theorem is conditional on the supplied linear maps, particular
solution and kernel parameterization. It does not prove the concrete native
finite-field arithmetic, RREF implementation, kernel spanning, curve map or
raw-data decoding. The cold experiments check those separately through an
independent GP rank/kernel/group/map replay. No full concrete formal bridge is
claimed.

Reproduce into a fresh absolute directory:

```sh
bash research/twist_routes_1_3_20261021/lean-v10/replay.sh /workspace/fresh-compression-proof-receipt
```
