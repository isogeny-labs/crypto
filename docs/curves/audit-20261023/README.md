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
| Experimental one-line `*curve_ids.json` harvest plus full rebuild | 0 | [candidate patch](curve-ids-candidate.patch), [build log](curve-ids-candidate-build.log) |
| Freshness check against that unreviewed scratch rebuild | 0 | [candidate check log](curve-ids-candidate-check.log) |
| Bind each new model and order to a unique row in original JSON | 0 | [checker](check_candidate_sources.py), [summary](candidate-source-summary.json), [bindings](candidate-source-bindings.jsonl.gz) |
| Independently check all new prime-curve orders | 0 | [checker](check_candidate_orders.py), [summary](candidate-order-summary.json), [certificates](candidate-order-certificates.jsonl.gz) |
| SEA formal gate with the added order-uniqueness theorem | 0 | [gate log](current-formal-gate.log), [Lean axiom log](registry-order-lean-axioms.log) |
| Full-checkout serialized Rust library suite at `7a74a1d938` | 0 | [complete log](full-checkout-cargo-test.log.gz): 4,502 passed, 119 ignored |

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

The [source-scope summary](source-scope-summary.json) classifies all 13,219
extra entries: all are prime models, with first source paths in the two
volcano-cost studies (7,760 and 870), volcano census (4,583), isogeny-volcano
rho (3), and weak-curve leaderboard (3). None has an EC1 representation.
This is an input-inventory classification, not validation of their group
orders. Harvesting the seven tracked `*curve_ids.json` documents as exact
representations resolves the n73 UID mismatch in scratch, and the freshness
check then passes against the scratch-generated 15,990-entry registry. That
single-line candidate is retained for review; neither it nor the generated
registry is promoted by this audit.

The [validation receipt](candidate-validation.json) binds the unpromoted
candidate registry, source rows, arithmetic certificates, and Lean theorem by
SHA-256. The checker also independently recomputes all 13,219 ICV1 model
hashes, identifiers, slugs, traces and j-invariants. Each new model matches
exactly one JSON row in one
of 152 tracked files; an in-memory one-field mutation removes that match
([negative control](candidate-source-negative.log)). For 13,216 models, the
checker confirms a point order dividing the claimed order and large enough
that Hasse's interval contains only one multiple. The remaining three models
over fields of size at most 103 were counted over every field element. The
small-field arithmetic self-test passed; a deliberately wrong order was
rejected with Python assertions disabled.

The Lean 4.19 theorem
[`point_order_forces_group_order`](../../../research/sea_point_count_20261016/lean/RegistryOrderUniqueness.lean)
formalizes the final interval-uniqueness step under explicit Hasse interval
and divisibility premises. Its axiom audit lists only `propext` and
`Quot.sound`. Lean does not check the 13,219 Python point calculations or
the provenance of their JSON input. The source and order certificates can be
read with `gzip -cd` and regenerated from the SHA-pinned candidate. The
historical SEA TLA configurations also passed their positive and negative
controls under the current stable tool pin.

The scratch rebuild additionally changes 527 of the 2,771 existing registry
entries, chiefly representation status, source lists, parameters, and trace
field formatting. It drops 278 prime-curve EC1 representations present in
the committed file because the four conductor-gap `*curve_ids.json` lists
carry an `r` subgroup field that the current generic JSON harvester does not
consume. Those changes and the policy for including all 152 source
files still need review before any generated registry or curve-linked graph
is replaced.
