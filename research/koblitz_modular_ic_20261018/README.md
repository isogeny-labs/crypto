# Koblitz modular and low-genus carrier investigation

The current round is [06](rounds/06/README.md), with its
[report](rounds/06/REPORT.md), [PDF](rounds/06/report.pdf),
[proof map](rounds/06/PROOF_MAP.md), [ledger](rounds/06/ledger.json)
and exact replay. All 44 degree-210 images are zero for both signs.
The evaluated matrix has rank 5/588; its nonzero-image submatrices remain
rank 4 on 510/520 columns. All ten frozen determinant-5 graphs now have
split quotient equations and effective F2 descent. The even reverse map
on one order-11 generator is evaluated as an unreduced divisor with all
20 fibers in split/etale coordinates, degree 6,600 on each side.
Independent pointwise forward norms, Jacobian reduction, the odd reverse
divisor and determinant-19 quotients remain open. Fourteen new named Lean
theorems pass. The user assesses IC implications; no timing is measured.

Earlier evidence stays frozen:

- [Round 01](REPORT.md): modular signed-character carrier and cyclic-cover bound.
- [Round 02](rounds/02/README.md): explicit Klein transfer, geometric relations,
  bounded low-genus searches and the first thirteen structural Lean lemmas.
- [Round 03](rounds/03/README.md): remaining del Pezzo twists, conditional
  census exclusion through genus 6, polarized kernels and n=5 F32 controls.
- [Round 04](rounds/04/README.md): natural quotient equations, global
  minimum certificates and evaluated modular rows of rank 4/544.
- [Round 05](rounds/05/README.md): integral reverse correspondence and
  11-primary splitting, canonical quotient constructions and symbolic rank 5/588.
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
