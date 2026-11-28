# Exact in-degree variance for every distinct finite step support

Lean proves the exact formula $1-\sum_i p_i^2$ for actual in-degree
variance in the ideal independent-label model on every finite abelian
group. Increment values must be distinct; probabilities may be any
nonnegative reals summing to 1. The mean is 1 at every target vertex.
The theorem constructs a genuine PMF and counts actual incoming graph
edges. [pcc:unverified]

The complete expanded walk project gate passes **70 declarations** with
zero failures and zero trusted-native exceptions. Only propext,
Classical.choice and Quot.sound are accepted. See the
[gate summary](evidence/general-variance-gate.log) and
[full audit](evidence/general-variance-gate/research_walk_audit_20261109_lean/audit.log).
Lean 4.35.0 and Mathlib 56a00585 are pinned. [pcc:unverified]

For a hash $h:G\to I$, the graph is $x\mapsto x+g_{h(x)}$ and its probability
is the product of the vertex-label probabilities. The proof uses a
finite product-of-sums identity to normalize that law and establish
independence of observations at distinct vertices. It proves that the
actual incoming-edge cardinality at $z$ is the sum of indicators
$h(z-g_i)=i$. The second moment separates equal and different labels;
step injectivity gives distinct predecessor vertices for different labels.
No fitted values or assumed graph table enters the proof. [pcc:unverified]

Lean also directly disproves the formula when distinctness is replaced
by the weaker condition that every increment is nonzero. On Z/3, two
labels both selecting +1 give a translation for every hash, so the
in-degree is always 1 and its variance is 0. The proposed unmerged-label
formula is 1/2 for probabilities 1/2 and 1/2. The negative control preserves
the group, actual PMF and nonzero increments. It does not refute the
properly qualified distinct-support formula. [pcc:unverified]

| Obligation | Declaration in `WalkWeightedGeneralAudit` |
|:--|:--|
| Genuine normalized probability law | `weight_sum`, `hashLaw`, `hashLaw_real_mass` |
| Independent vertex observations | `expectation_product`, `expectation_two_coordinates` |
| Actual incoming-edge count | `inDegree_eq_coordinates` |
| Exact first and second moments | `inDegree_mean`, `inDegree_second_moment` |
| Exact variance and actual PMF moments | `inDegree_variance`, `actual_graph_moments` |
| Duplicate nonzero increment control | `inDegree_constant`, `constant_increment_variance_zero` |
| Direct universal disproof without distinctness | `distinctness_cannot_be_dropped` |
| Fully quantified combined binding | `general_variance_audit_certificate` |

This verifies the variance component. The collision-time factor
$1/\sqrt{1-\sum_i p_i^2}$, limiting constants, Hildebrand's random-support
averaging and behavior of practical hash families remain separate.
There is no new native experiment, timing or paid compute. The earlier
finite weighted proof and native records remain frozen. [pcc:unverified]

See [source](lean/WalkWeightedGeneral.lean),
[source plan](general-variance-sources.json), and retained failed sources
and exact gzip diagnostics in evidence/. Primary Galbraith chapter 14,
version 2, §14.2.5, pages 296–297, was re-read at the recorded scope in
weighted-sources.json. No novelty claim is made. Scientific PCC standing
requires an independent signed verdict. [pcc:unverified]

Source commit `6f963444` precedes packet `PCC-20261110-4e62d0`. The full
local check passes at 01:29:41 UTC on producer host `762f5fc120c9`;
signed standing remains unverified. See [actual bindings](general-variance-verification.json),
[inspected two-page PDF](reports/proof-20261110-general-variance/report.pdf),
and [editable report](reports/proof-20261110-general-variance/report.md).
[pcc:unverified]
