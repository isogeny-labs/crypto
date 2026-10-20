# Koblitz modular and low-genus carrier investigation

The current round is [12](rounds/12/README.md), with its
[report](rounds/12/REPORT.md), [PDF](rounds/12/report.pdf),
[proof map](rounds/12/PROOF_MAP.md), [ledger](rounds/12/ledger.json)
and exact replay. All supersingular zeros of the eight coupled functions
now have complete closed supports and local multiplicities: 3,080 source
points in 308 degree-10 orbits. The antidiagonal contributes degree 740
there, leaving ordinary degree 500. All seven whole unfinished divisors
still have ordinary supports to construct. All supersingular target
values are killed by 8 and give zero 11-primary image under the retained
projections. Relation ranks remain unchanged.

Each of the four actual full homogenized quotient circuits has a certified
false boundary point. A native saturation step emits 213,120 generated
scalar linear relations, with h-membership depth at most 5; they need not
be independent. The four affine open models remain available. Complete
projective saturation, proper quotient boundaries and polarized embeddings
remain open. Twenty-four new named Lean results include eight actual
boundary-circuit certificates. Modular geometry and native semantics remain
partially formalized. No IC cost or efficiency verdict is supplied.

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
- [Open construction problem](../../problems/koblitz-modular-carrier-transfer.md):
  current obligations, exact scope, and next discriminating experiments.

Research arithmetic and record assembly are native Rust in `native/`.
The pinned Lean project is `lean/`. Each round has its own protocol,
failed-attempt records, proof map, immutable raw archive and replay script.
