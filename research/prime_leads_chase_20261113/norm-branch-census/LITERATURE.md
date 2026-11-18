# Literature and prior-work receipt

**Checked:** 2026-11-13. **Scope:** primary sources and repository-local
evidence only; not exhaustive. No novelty claim.

## Primary sources

1. Christophe Petit, Michiel Kosters, and Ange Messeng, “Algebraic approaches
   for the Elliptic Curve Discrete Logarithm Problem over prime fields,” PKC
   2016, DOI `10.1007/978-3-662-49387-8_1`, author-hosted PDF:
   <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>.
   Read pp. 1–8 (contributions, factor-base map, Algorithm 1, partial cost
   model, Kummer and auxiliary-isogeny constructions). The paper explicitly
   leaves the relation-system complexity open and suggests dedicated solvers;
   it does not supply a sub-rho bound.
2. Alessandro Amadori, Federico Pintore, and Massimiliano Sala, “On the
   discrete logarithm problem for prime-field elliptic curves,” IACR ePrint
   2017/609: <https://eprint.iacr.org/2017/609.pdf>. Read the algorithm and
   relation-collection framing. It reduces Gröbner-basis invocations; it does
   not establish the sparse per-call exponent required by the continuation
   protocol.
3. Dan Singer, “A Bijective Proof of Borchardt’s Identity,” *Electronic
   Journal of Combinatorics* 11 (2004), R48:
   <https://www.kurims.kyoto-u.ac.jp/EMIS/journals/EJC/Volume_11/PDF/v11i1r48.pdf>.
   Theorem 1.1 gives
   `det(1/(x_i-y_j)^2)=det(1/(x_i-y_j))*per(1/(x_i-y_j))`.
   Therefore distinct disjoint nodes do not force the inverse-square Cauchy
   matrix nonsingular: they force only the ordinary Cauchy determinant factor.

## Repository evidence reconciled

- `../CONTINUATION_PROTOCOL.md`: corrected PKM whole-call gates and conditional
  norm-branch B1/B2 sequence.
- `../NORM_BRANCH_COUNTEREXAMPLE.md`: exact fixed-size rank-one block.
- `../../prime_norm_rank_20261112/DERIVATION.md`: norm/branch certificate and
  complete-cost boundary.
- `../../prime_norm_rank_20261112/public-jet-report/README.md`: supplied-branch
  reconstruction is not a public branch finder.
- `../L1-pkm/THEORY.md` and `PROTOCOL.md`: measured degree evidence and the
  unresolved two-free-block mechanism; degree alone does not price positive
  relation finding.

## Unavailable canonical search

- Research-KB MCP tools were not installed in this runtime.
- `scripts/research-kb status` could not run because the executable/catalog was
  absent from this checkout.
- The sibling `crypto-autoresearcher` checkout exposes the canonical protocol,
  review, schema, and lead-atlas paths only as `VLT1` vault ciphertext; no key
  was available. Ciphertext was not treated as evidence.

Consequences: no statement here is a novelty, priority, exhaustive-review, or
“best known” claim. Before any positive mechanism is promoted, rerun the
canonical lead-atlas/technique/decision and theorem searches.
