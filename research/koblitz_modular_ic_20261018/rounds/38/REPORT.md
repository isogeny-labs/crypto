# R38: larger closed-point algebra

ALG38 constructs the exact algebra F2[t]/(t⁵+t³+1)⁶¹⁵, of dimension 3075,
at the same certified degree-five point Q used in R35. All 3075 inverse columns
and every pivot are checked by recombination. The actual coefficient-field
embedding, ring identities and corrupt inverse/field controls also pass.

The 1024th power still extracts the constant coefficient: 615 is below 1024,
and the coefficient field has Frobenius period 32. One pinned Lean lemma proves
this identity under explicit characteristic-two, residue-period and nilpotence
hypotheses. The native arrays and proper geometry are not imported as formal
proofs. The terminal compile and axiom-audit receipts are indexed in results.json.

For the proposed base divisor D1 = 4D0, of degree 512, the evaluation divisor
615Q has degree 3075, exceeding the pole bound 3072 for L(6D1). Under the retained
proper-curve and pole-divisor arguments, this algebra can host larger
Representation B spaces. Their prospective dimensions 452, 1476 and 3012 have
**not** been constructed by this run.

The remaining work includes 615-term source jets, complete section spaces,
ordinary degree-367/460 divisor evaluations and their factor images. A
degree-367 support can be padded with 29Q to degree 512. The degree-460 gap is
52, so it requires an additional certified support or local trivialization;
copies of a degree-five point alone cannot fill it.

The native 4 GiB virtual-memory/1200 s and pinned Lean -j1 -M4096/1200 s limits
are unchanged. Both R38 attempts passed. This is an algebra construction,
not an IC relation rank, speed comparison, scalar extraction or a new larger-n
Koblitz transfer. Global metric integration is pending exclusive path ownership;
see R37/METRICS_PENDING.md and its scoped patch.
