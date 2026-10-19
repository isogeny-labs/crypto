# Koblitz modular and low-genus carrier investigation

The current round is [08](rounds/08/README.md), with its
[report](rounds/08/REPORT.md), [PDF](rounds/08/report.pdf),
[proof map](rounds/08/PROOF_MAP.md), [ledger](rounds/08/ledger.json)
and exact replay. The complete line-norm family raises nonzero-image rank
to 11 for both signs, on 1165/1160 columns; evaluated rank is 12/1308.
Exact geometric joins and separate small-field checks pass. Four mixed
determinant-19 quotients have intrinsic Kummer equations and six explicit
covering charts each over F2^360, with 200 exact transition identities.
Sixteen new named Lean results pass, including chart coverage and cocycles.
Effective F2 coordinate descent, a closed projective embedding, Jacobian
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
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
