# Koblitz modular and low-genus carrier investigation

The current round is [03](rounds/03/README.md), with its
[report](rounds/03/REPORT.md), [PDF](rounds/03/report.pdf),
[proof map](rounds/03/PROOF_MAP.md), [ledger](rounds/03/ledger.json)
and exact replay. It covers the remaining del Pezzo twists, finite polarized
kernel gluing and n=5 same-field maps and geometric relations. The actual
F2 modular-factor evaluator remains open; the user assesses IC implications.

Earlier evidence stays frozen:

- [Round 01](REPORT.md): modular signed-character carrier and cyclic-cover bound.
- [Round 02](rounds/02/README.md): explicit Klein transfer, geometric relations,
  bounded low-genus searches and the first thirteen structural Lean lemmas.
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
