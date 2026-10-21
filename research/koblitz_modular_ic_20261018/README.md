# Koblitz modular and low-genus carrier investigation

The current round is [14](rounds/14/README.md), with its
[report](rounds/14/REPORT.md), [PDF](rounds/14/report.pdf),
[proof map](rounds/14/PROOF_MAP.md), [ledger](rounds/14/ledger.json)
and exact replay. The five remaining ordinary components have57 simple
closed points and complete even/odd images under the written global bridges.
Forty fresh split columns contain35 even and30 odd nonzero images.
The joined coupled principal rows have rank16/1348 and nonzero-image
rank15/15. Adding eight separate deck-fiber rows gives rank20/1348 and
nonzero-image ranks18/17 on1200/1190 columns. Deck rows are not principal.

All eight F2 closed signed divisors are complete under written bridges:
O28 enumerates44 closed cusp orbits and reproduces the old product divisor. Q16/Q17
add98 linear products and288 coefficient quadrics per circuit without a
new separating scalar witness. Full h-saturation remains uncomputed.
The canonical6lambda polarized target is P^7775, with actual pullback
and corrected Cartier character data; global sections and proper equations
remain open. Sixty-five new named Lean results retain explicit geometric
and implementation limits. No IC cost or efficiency verdict is supplied.

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
