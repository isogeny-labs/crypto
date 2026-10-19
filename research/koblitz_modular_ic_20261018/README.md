# Koblitz modular and low-genus carrier investigation

The current round is [07](rounds/07/README.md), with its
[report](rounds/07/REPORT.md), [PDF](rounds/07/report.pdf),
[proof map](rounds/07/PROOF_MAP.md), [ledger](rounds/07/ledger.json)
and exact replay. Two complete full-map norm relations raise nonzero-image
rank to 6 for both signs, on 675/678 columns. The full evaluated matrix
has rank 7/768. Independent pointwise forward norms return the source for
both even and odd reverse generators. The corrected odd divisor has all
20 fibers in explicit coordinates, degree 6,600 on each side, with formal
F2 descent. Four mixed determinant-19 translation atlases and expanded
supersingular [19] cover equations are checked. The intrinsic mixed
quotient image ideal, Jacobian reduction and individual descent remain
open. Seventeen new named Lean results pass. The user assesses IC
implications; no timing is measured.

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
- [Round 06](rounds/06/README.md): 44 evaluated zero-image columns,
  all ten determinant-5 quotient graphs and explicit even reverse fibers.
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
