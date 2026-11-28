# Before-data amendment 7: peel pair-table diagonals

Status: specified before any scientific execution or outcome row in this study,
2026-11-15 PST. This amendment changes only the candidate
`rootfind-negfold-batch` binary pair-table constructor. The fixed six-arm
`SPEC-K13.json`, eight public targets, seeds, repeats, resource and failure
rules, and Amendment 3 complete cold wall decision remain unchanged. The
scalar `rootfind-negfold` and F4/F5/F6 reference arms keep their source.

Each abscissa row starts with its own group. Those same-abscissa additions
are exceptional and make the existing SIMD batch routine decline the entire
row. The candidate now completes that diagonal group first, in its original
order. It passes the remaining groups to the paired x-only SIMD routine when
they form adjacent finite signed pairs and the kernel accepts them. Otherwise
it takes the existing paired batch fallback. The resulting abscissae stay in
the scalar constructor's pair order, and the first witness for each key wins.
The abstract addition charge remains one per attempted pair.

Before publication, check the K13/K17 scalar-versus-batch table, first
witness, insertion counts, and m=2/m=3 decomposition parity in release mode;
cross-compile the x86 ecbench path. A local ARM check exercises the fallback,
not the AVX-512 path. The admitted x86 study must retain exact source and
binary hashes, output validation, failed rows, and an independent replay.
The complete cold wall comparison remains the decision metric. This source
revision alone establishes no quantitative advantage.

Earlier publication receipts retain their historical scope. Commit and push
this amendment, the source review, updated status and source, then publish a
separate `publication-amendment-7.json` receipt with `check-experiment-git`
before scientific execution on this revision. The monthly EKS cost gate and
independent replay requirement continue to apply.
