# Koblitz modular and low-genus carrier investigation

The current round is [11](rounds/11/README.md), with its
[report](rounds/11/REPORT.md), [PDF](rounds/11/report.pdf),
[proof map](rounds/11/PROOF_MAP.md), [ledger](rounds/11/ledger.json)
and exact replay. The 460 diagonal multiplicity gap is closed: 400
supersingular labels have contacts 1, 2, 6, 14 with counts 260, 100, 20, 20,
weighted degree 860. Forty degree-10 closed points form the complete
noncuspidal diagonal under the written characteristic-2 moduli bridge.
Actual 11-torsion and quaternion/Velu maps are bound; explicit 49-torsion
point coordinates remain open.

A coupled multiplication/two-inverse graph has an exact five-quadric
projective saturation and six charts. The four determinant-19 quotient
affine open models remain available; whole proper quotient saturation,
boundary gluing and projective embeddings remain open. Translated T-fibers,
a genus-3 ambient x-sum correspondence and an equal-y residual curve add
structure to the seven still incomplete noncuspidal divisors. The
antidiagonal has unresolved degree 1,240. Ranks stay 12/1308 and nonzero
11 on 1165/1160 columns. Twenty-two new Lean results pass, with 168 project
theorems audited. Global geometric bridges remain written. No IC cost is
measured or assessed.

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
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
