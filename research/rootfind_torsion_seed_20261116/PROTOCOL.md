# Koblitz torsion-row pair-table shortcut — frozen mechanism check

Frozen before implementation and before any new execution on 2026-11-16.
This is an opt-in correctness and operation-accounting check for a new
`pdp3-binary` engine. It does not replace the frozen ten-arm K13 EKS L2
F4/F5/F6 panel, and it does not assert a wall-time advantage.

## Mechanism and prediction

On the binary model `y^2 + xy = x^3 + ax^2 + 1`, the nonidentity point
`T=(0,1)` has order two. For `x(P) != 0`, the group law gives
`x(T+P)=1/x(P)`. Thus the first row of a negation-folded pair table can
compute one key per nonzero base abscissa with a batch inversion, retaining
the first signed point as the witness. Its negative twin has the same key
and supplies only `2T=O`. Since the fixed cells have odd prime subgroup
order `r` and cofactor four, the corresponding log column is zero.

The prediction is that an opt-in torsion engine yields the **same** table
keys and first witnesses as `rootfind-negfold-batch-harvest`, and the same
seed-row span, while avoiding the first-row group additions and redundant
full-group checks. Its native field inversion, table insertion, row and
rank work must all be recorded. A complete-call or cold-process advantage
is unknown until a separately frozen timing panel measures it.

## Fixed inputs and controls

The three cells are Koblitz E0 over GF(2^13) with standard-subspace d=5,
E0/GF(2^19) d=7, and E0/GF(2^23) d=8, each using the same
`SignedFrobeniusOrbit` columns as the earlier collision census. Their raw
records are respectively `k13.json` SHA-256
`b90950bf46394dd82bdc87d6da6ea597825b63b5e17a2146046cc37e7d050aa1`,
`k19.json` SHA-256
`55e434b0dcba675f951b98a78adbf690d92e1ecfdd876f3166bf8b069d1f5dda`,
and `k23.json` SHA-256
`950460e945919b65bcbdeb0d48f9fe42e6355639ee5d07a56a08578c96823b35`
under `../rootfind_pair_collision_census_20261116/reports/run-20261116T0711Z/`.
The previous source is commit `4be5f2b0fee5b3cb2c5260b56ed639ef3408d41a`.
All source pointers and records in this paragraph are internal provenance.

Before the candidate is accepted, require:

1. The existing scalar and batch tables, and the new torsion table, have
   identical key-to-first-witness maps and identical probe answers on every
   table key in each fixed cell. The new builder reports actual attempts,
   skipped twin witnesses, batch-inversion lanes and group operations.
2. Every retained collision row still passes the full group-equation check.
   The skipped first-row twins must have `i=fi=T` and `j=-fj`, with a direct
   negative-index and point-negation check. The seeded torsion row is admitted
   only when `T=-T`, `T != O`, `r` is odd and the cofactor is even. A corrupted
   negative-index control and an unsupported-curve/shape control must refuse.
3. Modular rank of the retained rows plus torsion seed equals the original
   5, 11 and 23 on K13/K19/K23. The original collision candidates remain
   27, 99 and 201; the predicted skipped-twin counts are 16, 69 and 137.
4. The same public K13 generator, target, seed and known scalar 641 used in
   the current binary integration test recover an answer satisfying
   `[d]G=Q` under both engines. Keep target trial and relation counts visible,
   even if they differ.

A table mismatch, wrong/unchecked witness, wrong answer or control failure
invalidates this implementation. A lower operation count without lower
whole-call wall time is not a performance success. A timeout or build failure
is infrastructure/implementation evidence only.

## Execution boundary

First compile and run the release `ecbench` binary tests on the local host;
this is a correctness gate, not a benchmark. Stop after these three fixed
base cells and the one fixed same-target pipeline fixture. No adaptive cell
choice or performance claim follows from the tests. Retain exact command,
source commit and dirty state, compiler and host, stdout/stderr/exit, binary
hash, operation counters, graph and PDF in a write-once report directory.
A later wall-time study needs its own pre-data spec with complete-process
timing, matched arms, A/A, isolation and independent replay. The existing
EKS combined panel remains unchanged and gated by live budget headroom.
