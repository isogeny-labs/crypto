# Koblitz modular and low-genus carrier investigation

The current round is [15](rounds/15/README.md), with its
[report](rounds/15/REPORT.md), [PDF](rounds/15/report.pdf),
[CM construction](rounds/15/CM.md), [theta descent](rounds/15/THETA.md),
[proof map](rounds/15/PROOF_MAP.md), [ledger](rounds/15/ledger.json)
and exact replay. Direct odd CM characters give level7p carriers of genus61
for n=5 and45589 for n=131, with exact Euler/coset checks. A diamond
quadratic twist supplies the required n=131 sign. Actual maps and subgroup
injection are open; the global construction is written, not fully Lean.

Actual theta corrections give all four quotients a3lambda target P242,
with243 sections; section bases and proper equations remain open. New
coupled norms have ordinary pushed degrees460/367, with exact factors but
source fibers/images uncomputed. All four actual Q17 models have no h=t
first jet; ramified arcs and full saturation remain open. Fifty-eight new
named Lean results cover relevant identities and finite certificates.
Round14's complete eight divisors and ranks remain unchanged. No IC cost
or efficiency assessment is supplied.

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

- [Round 14](rounds/14/README.md): all57 ordinary supports/images, eight closed
  signed divisors and principal/deck nonzero ranks15/15 and18/17.
