# Koblitz modular and low-genus carrier investigation

The current round is [16](rounds/16/README.md), with its
[report](rounds/16/REPORT.md), [PDF](rounds/16/report.pdf),
[CM equations](rounds/16/CM.md), [cover sections](rounds/16/THETA.md),
[saturation](rounds/16/SATURATION.md), [proof map](rounds/16/PROOF_MAP.md),
[ledger](rounds/16/ledger.json) and exact replay.

The level77 genus61 carrier has explicit F2 affine incidence equations and
a derived diamond twist, with nonvacuous independent torsion checks through
F32. Proper normalization and Koblitz-factor maps are open; the simple
Frobenius sign-projector denominator704 fails integrality at11. Four
polarized diagonal F32 covers now have explicit factor section bases,
but243 target sections and proper equations still need kernel descent.

A depth6 quartic separator excludes all four old Q17 boundary points for
any ramification; four new checked Q22 survivors keep full saturation open.
Sixty-three new named Lean results cover identities and actual finite data.
Round15's new norm fibers/images and Round14's ranks remain unchanged.
No IC efficiency verdict is supplied.

Earlier evidence stays frozen:

- [Round 15](rounds/15/README.md): direct level7p CM carriers, rational
  theta descent, new coupled norms and first-jet obstructions.

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
- [Round 07](rounds/07/README.md): independent forward norms, corrected odd
  reverse divisor, nonzero-image rank6 and determinant-19 cofactor covers.
- [Round 08](rounds/08/README.md): complete line-norm family and four
  intrinsic Kummer quotients with six covering charts and exact transitions.
- [Round 09](rounds/09/README.md): explicit Frobenius circuits, trace-dual
  rational generators and bounded twist/reflection and coupled-function controls.
- [Round 10](rounds/10/README.md): explicit quadratic F2 open models,
  complete product cusp divisor and the then-unresolved diagonal multiplicity gap.
- [Round 11](rounds/11/README.md): closes the460 diagonal multiplicity gap
  using characteristic2 local equations;projective coupled-graph control.
- [Round 12](rounds/12/README.md): complete SS strata and actual false
  quotient boundary points with certified constant repairs.
- [Round 13](rounds/13/README.md): ordinary500/580 supports and zero
  deck images; inverse/shared/four-leaf/square saturation repairs.
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.

- [Round 14](rounds/14/README.md): all57 ordinary supports/images, eight closed
  signed divisors and principal/deck nonzero ranks15/15 and18/17.
