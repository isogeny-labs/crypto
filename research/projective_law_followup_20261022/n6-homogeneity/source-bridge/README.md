# Compiled expression bridge for weighted 2+2 doubling

This source-bound finite packet checks exactly the `N1,N2,B,N5,N6,A,C`
intermediate expressions in the pinned Rust `double22_weighted_variant` against
the pinned author Magma `DBL2` formulas. The [PDF](REPORT.pdf), [graph](GRAPH.svg),
[25-stratum CSV](ROWS.csv), and [machine receipt](RESULT.json) expose all counts.
The prior [Lean packet](../source-correction/README.md) gives universal polynomial
identities for the abstract expressions. The present check compiles the exact
Rust expression block with a minimal `F_5` field, then compares 390,625 outputs
to independently evaluated author formulas. The squared-W path has zero
mismatches. A separately compiled synthetic W¹ variant gives 187,500 `N6`,
120,000 `A`, and 150,000 `C` mismatches. Every `w=0` or `w=1` stratum has
zero mutation mismatches; every `w=2,3,4` stratum shows them. These are counts
of unconstrained tuples, not curve-point coverage or cost measurements.

The checker verifies full source SHA-256 digests, source-statement ordering,
the production squared-W selector, compiler exit, row count, every tuple's
arithmetic, CSV stratum cardinality, and mutation sensitivity. Its limitation
is explicit: the generated harness compiles extracted production expressions
with a minimal field type. It does not execute the production binary's branch
dispatch, prove Rust field primitive semantics, or establish the Pic⁰ law.
The adjacent [SHA256SUMS](SHA256SUMS) binds the report, graph, source, and
standardized result files. Hydrate the PDF with
`python3 tools/artifacts/artifacts.py pull research/projective_law_followup_20261022/n6-homogeneity/source-bridge`
before opening it from a fresh checkout.

Replay from the repository root after retrieving the pinned author file:

```sh
curl --fail --location --silent --show-error \
  https://raw.githubusercontent.com/ozbayelif/jac-on-jac/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m \
  --output /tmp/jac-on-jac-ec98.m
sha256sum /tmp/jac-on-jac-ec98.m
python3 research/projective_law_followup_20261022/n6-homogeneity/source-bridge/run.py \
  --author-source /tmp/jac-on-jac-ec98.m
python3 research/projective_law_followup_20261022/n6-homogeneity/source-bridge/render.py
```

The expected author-file SHA-256 is
`7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a`.
