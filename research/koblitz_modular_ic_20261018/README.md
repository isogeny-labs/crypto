# Koblitz modular and low-genus carrier investigation

The current round is [09](rounds/09/README.md), with its
[report](rounds/09/REPORT.md), [PDF](rounds/09/report.pdf),
[proof map](rounds/09/PROOF_MAP.md), [ledger](rounds/09/ledger.json)
and exact replay. All four determinant-19 quotients have explicit
Frobenius coordinate circuits over F2^360 and 4,320 invariant rational
generating circuits each. The dual base uses inverse-transpose descent;
80 exact slices, 576 chart squares and the written constant-deck argument
check the lift. Trace-dual recovery checks pass. Seventeen new named Lean
results pass their kernel build and transitive audit.

All 50 signed diagonal-twist/reflection controls fail to give a stable
quotient retaining the known trace-zero component. This is a bounded
component test. Eight coupled functions are nonzero on all 720 tested norm
columns; their complete divisors remain open. No relation rank is promoted:
evaluated rank stays 12/1308, with nonzero-image rank 11 on 1165/1160 columns.
F2 polynomial relations, saturation, a closed embedding, Jacobian
identification and individual descent remain open. No timing is measured.

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
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
