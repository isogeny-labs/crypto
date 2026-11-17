# Rebuild the complementary-projector theorem

Hosted proof job 122579090743 could not resolve `import SubgroupTransfer`
when this file was compiled as a standalone module. This project now declares
the tracked sibling `ghs_subgroup_transfer` Lake library as a local dependency.
Both projects keep their existing Lean 4.34.1 pins and unchanged proof sources.

From the repository root:

```sh
tools/lean/proof-gate.sh research/ghs_generalized_20261024/lean research/ghs_cover_decomposition_20261025
```

Both complete module builds and native axiom audits pass: 13 subgroup-transfer
theorems and the complementary-projector theorem, with no trusted exceptions.
Original source hashes, the failed hosted import and fresh receipts are kept
in `reports/ghs-project-wiring-20261028/`. Discovery retains all original targets;
only these two modules become registered project targets. The newly merged
triangle certificate is also independently built and audited.

The theorem still assumes the pull/push composition. Concrete geometric map
and Jacobian-decomposition proofs remain outside its stated scope. No Sage
experiment, scientific threshold, proof hypothesis or archived result changes.
