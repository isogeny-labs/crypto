# Ordered-domain yield corollaries

Pinned Lean: `leanprover/lean4:v4.34.1`; no Mathlib dependency. The local path dependency is the existing `research/factor_base_designs_20261015/lean` project.

`IdeasExecution/OrderedYield.lean` enumerates the ordered Cartesian power of a base list, proves its length is B^m, and instantiates the existing finite-fibre `sum_count` theorem. It also compares equal-cardinality ordered patterns. Inputs must cover the pattern's entire image with a duplicate-free target list. This counts multiplicity, including repeated ordered entries; it is not a multiset count, coverage theorem, fixed-target estimate, rank claim or oracle-speed bound.

Fresh checks completed on 2026-10-18:

```sh
/workspace/toolchains/lean-4.34.1-linux/bin/lake build
/workspace/toolchains/lean-4.34.1-linux/bin/lake env lean AxiomAudit.lean
```

All six build jobs pass. The three declarations audited use only `[propext, Quot.sound]`; there is no `sorryAx`. Logs are at `../runs/ordered-yield-build.log` and `../runs/ordered-yield-axioms.txt`.

The campaign also freshly rebuilt/audited the original nine-job factor-base project and four-job handle-only generic-invariance project. The latter explicitly excludes representation-hashing walks. Neither a fresh build nor this new ordered-domain bridge formalizes the compact pipeline or its preprocessing lower bound; those remain open.
