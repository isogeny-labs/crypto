# Koblitz modular and low-genus carrier investigation

The current round is [05](rounds/05/README.md), with its
[report](rounds/05/REPORT.md), [PDF](rounds/05/report.pdf),
[proof map](rounds/05/PROOF_MAP.md), [ledger](rounds/05/ledger.json)
and exact replay. It derives the integral n=5 reverse correspondence and
11-primary splitting, five canonical polarized quotient constructions,
and a level-dependent divisor with a symbolic projected row raising rank
to 5 on 588 columns. The 44 new column images remain unevaluated; the
evaluated matrix stays rank 4/544. Other graph equations, reverse-divisor
evaluation and individual descent remain open. The user assesses IC implications.

Earlier evidence stays frozen:

- [Round 01](REPORT.md): modular signed-character carrier and cyclic-cover bound.
- [Round 02](rounds/02/README.md): explicit Klein transfer, geometric relations,
  bounded low-genus searches and the first thirteen structural Lean lemmas.
- [Round 03](rounds/03/README.md): remaining del Pezzo twists, conditional
  census exclusion through genus 6, polarized kernels and n=5 F32 controls.
- [Round 04](rounds/04/README.md): natural quotient equations, global
  minimum certificates and evaluated modular rows of rank 4/544.
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
