# Koblitz modular and low-genus carrier investigation

The current round is [10](rounds/10/README.md), with its
[report](rounds/10/REPORT.md), [PDF](rounds/10/report.pdf),
[proof map](rounds/10/PROOF_MAP.md), [ledger](rounds/10/ledger.json)
and exact replay. All four determinant-19 quotients have explicit quadratic
F2 polynomial presentations and closed affine embeddings of specified
descended dense opens. Inverse witnesses give denominator-norm saturation.
The systems have 424,080 or 415,800 equations per graph in a lossless
coefficient-block format. Whole-quotient projective saturation, boundary
gluing and closed projective embeddings remain open.

The coupled divisor x0*xinf is complete on 22 closed cusp points, with
zero and pole degrees 2,640. All eight pole divisors and cuspidal zeros
are computed; seven noncuspidal supports remain open. The diagonal search
finds 400 supersingular moduli labels in 40 degree-10 orbits, but its
multiplicity lower sum leaves 460 unaccounted for. No rank is promoted:
evaluated rank stays 12/1308 and nonzero-image rank 11 on 1165/1160 columns.
Twenty-three new named Lean proofs pass; global geometric bridges remain
written. No IC cost is measured.

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
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
