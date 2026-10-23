# Full-checkout curve registry audit, 2026-10-23

This is a source-freshness audit on an isolated **complete** checkout of
crypto PR #2304 head `cc4185f3da586e28a8e83982699910777817bcbc`.
The checkout held 130,239 tracked files. The registry builder used Python
3.11.17, and the standalone source checker used Python 3.9.25. It did not
run a new ECDLP experiment or change any frozen result or graph. The
[open problem](../../../problems/curve-registry-full-tree-rebuild-diverges.md)
states the claim boundary and exit criteria.

| Command | Exit | Raw output |
|---|---:|---|
| `python scripts/build_curve_registry.py --check` | 1 | [full-registry-check.log](full-registry-check.log) |
| `python scripts/build_curve_registry.py` | 1, after writing scratch output | [full-registry-build.log](full-registry-build.log) |
| `python -O docs/curves/audit-20261023/check_volcano_slugs.py` | 0 | [source-check.log](source-check.log) |
| Same checker with one `b` value changed in a copied source map | 1 | [negative-control.log](negative-control.log) |

The full-tree generator produced 15,990 curve entries in scratch, against
2,771 in the PR's committed registry; the scratch output was not promoted.
It contained all three `icv1-fp26-tm6998-*` model slugs missing from the
committed registry. The standalone checker verifies the two original source
hashes, recomputes model slugs without the registry builder, checks primality
of the recorded field and subgroup orders by trial division, checks that
the recorded generators lie on their curves, and independently computes
`[r]G=O`. It does not count the full curve groups.

The full-tree build's terminal error is the unchanged n73 EC1 source claim
named in the problem record. The output includes many skipped records whose
declared orders violate Hasse; those are retained in the raw logs and are
not promoted as curves. The difference between a sparse-derived registry
and the full-tree result requires source-class review before regeneration.
