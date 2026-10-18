# Koblitz modular and low-genus carrier investigation

The current round is [04](rounds/04/README.md), with its
[report](rounds/04/REPORT.md), [PDF](rounds/04/report.pdf),
[proof map](rounds/04/PROOF_MAP.md), [ledger](rounds/04/ledger.json)
and exact replay. It adds an explicit natural polarized quotient, global
short-vector certificates, actual n=5 modular transport and closed-divisor
factor projections, and 102 direct modular principal relations of rank 4
on 544 columns. Reverse subgroup transfer, richer relations and individual
descent remain open; the user assesses IC implications.

Earlier evidence stays frozen:

- [Round 01](REPORT.md): modular signed-character carrier and cyclic-cover bound.
- [Round 02](rounds/02/README.md): explicit Klein transfer, geometric relations,
  bounded low-genus searches and the first thirteen structural Lean lemmas.
- [Round 03](rounds/03/README.md): remaining del Pezzo twists, conditional
  census exclusion through genus 6, polarized kernels and n=5 F32 controls.
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
