# Frobenius cross-bilinear sum-bucket test (2026-11-18)

**Result: FALSIFIED in the frozen toy experiment** (producer-local; independent PCC replication pending; no deployed-curve security claim).

## Distinct hypothesis

Test a **pair-dependent** Frobenius-invariant coordinate statistic on binary Koblitz curves:
\[
C_k(P,Q)=\operatorname{Tr}_{2^n/2}(x(P)x(Q)^{2^k}+x(Q)x(P)^{2^k}).
\]
Can it predict the nonlinear sum-coordinate bucket \(Z(P+Q)=\operatorname{Tr}(x(P+Q)^3)\) sufficiently to skip elliptic additions in a target-specific point-pair search?

The identity \(C_k(\pi P,\pi Q)=C_k(P,Q)\) is **PROVEN** because absolute trace is invariant under simultaneous Frobenius. Useful prediction and cheaper ECDLP are **CONJECTURES**; no novelty beyond the checked repository slice is asserted.

This is **not** the previously rejected single-operand trace-moment base, the 2026-11-10 isogeny-class x-interval coverage/variance (five-prime negative experiment), the rejected genus-2 torsion-order sieve, or the open D4/S5 lattice. It uses pairwise coordinate cross terms, not group-law or homomorphic trace features.

## Preregistration and test

Frozen pre-measurement protocol: [PROTOCOL.md](PROTOCOL.md), committed as 50716fbeb838ae4386260f9c439e018169773666 before data collection. Python stdlib implementation: [run.py](run.py), regression: [test_run.py](test_run.py), full data including the eight training models, confusion tables, controls, costs and excluded identities: [RESULTS.json](RESULTS.json).

Main cells: E_a: y^2+xy=x^3+a*x^2+1 over GF(2^n): (n,a)=(11,1), (13,0), using irreducible moduli 0x805 and 0x201b and exact odd-prime subgroup orders r=991 and r=2003. A third n=15,a=0,r=751 cell was sensitivity only. Seed 20261118, 8,000 unique unordered pair training samples and 8,000 disjoint holdout pairs per cell. Four k values and two polarities selected *on training only*. The harness verifies curve membership, full curve order, actual subgroup operations, sampled equality P+Q, Frobenius, associativity, inverse, subgroup order, and disjointness. 128 label permutation nulls and synthetic 90%-correct positive signal were evaluated.

Prerecorded gate: BOTH primary cells need heldout accuracy >=1.20 times class-majority accuracy **and** optimistic filter-first cost speedup >=1.20x. Failure of either falsifies this specific hypothesis.

## EMPIRICAL outcome

| binary field | r | k / polarity | test accuracy (95% Wilson descriptive) | majority accuracy | ratio | optimistic charged filter advantage |
|---|---:|---|---:|---:|---:|---:|
| 2^11, a=1 | 991 | 2 / inverted | 0.5071 [0.4962,0.5181] | 0.5020 | 1.010x | 0.930x |
| 2^13, a=0 | 2003 | 1 / direct | 0.5076 [0.4967,0.5186] | 0.5099 | 0.996x | 0.943x |
| 2^15, a=0 sensitivity | 751 | 4 / direct | 0.5071 [0.4962,0.5181] | 0.5959 | 0.851x | 0.951x |

Null permutation upper-tail empirical p: 0.124 and 0.085 primary. Synthetic positive controls: 0.894 and 0.900 accuracy. All 3 cells fail. Dependence among sampled pair endpoints makes the Wilson bands descriptive, not a formal universal bound.

## Predicted advantage, best baselines and correct limits

If Z has near-balanced labels and the target is uniformly distributed **inside** its bucket (an unproved favorable assumption), a classifier with accuracy A gives at most about 2A candidate-hit enrichment. Any filter-first pair search must pay the filter (at least two field multiplications) *plus* eventual EC addition. Even granting a cheap affine EC addition cost of 2n multiplication-equivalents and ignoring all setup/misses, the ideal ratio is
\[
  s_{\mathrm{optimistic}} = \frac{2A\,2n}{2n+2}.
\]
Observed A gives 0.930 and 0.943: filtering is worse even under this optimistic scenario.

For comparison, classical rho expects about sqrt(pi*r/2) group iterations: **39.45** and **56.09** on the two primary subgroups. An even more favorable **2n automorphism-folded** rho expectation sqrt(pi*r/(4n)) gives **8.41** and **11.00** iterations. A complete two-sum table at B=round(sqrt(r)) costs at least B(B+1)/2 = **496** and **1035** EC additions before any linear algebra, logarithms, construction, verification or descent, **59.0x and 94.1x** those optimistic rho references. These are counted operation *models*, not isolated wall-clock timing or a complete IC run. The degree-23 pt-l9 IC leaderboard arm is not a size-matched comparator; its already established gap to strong rho remains the more relevant production baseline.

The generic preprocessing barrier of Corrigan-Gibbs--Kogan remains relevant: S*T^2 = Omega(epsilon*r) in a plain cyclic prime group, adjusted for automorphism quotient in an orbit model. Pair-coordinate predicates may require a structured oracle model, but their existence alone does not evade any lower bound or imply an ECDLP break.

## Prior work, closure and next step

Repository sources checked: "research/ic_mega_pass_20261031/brief/BRIEF.md", "research/ic_mega_pass_20261031/report/REPORT.md", "research/ic_mega_pass_20261031/report/GAPS.md", "research/prime_field_lead_assessment_20261107/atlas_survey.md", "research/lead_sweep_20261110/LEAD-ICP-isogeny-transported-relation-yield/RESULT.md", and this repository's "AGENTS.md". Primary literature: [Semaev 2004](https://eprint.iacr.org/2004/031), [Gaudry 2009](https://doi.org/10.1016/j.jsc.2008.08.005), [Corrigan-Gibbs--Kogan 2018](https://people.eecs.berkeley.edu/~henrycg/pubs/eurocrypt18discrete/). The owner-private ledger and prior negative control records were inspected.

**Verdict:** exact C_k-to-Tr(x^3) filter rejected; no further scaling of this frozen feature. A genuinely distinct next hypothesis would require a mathematical reason for a low-cost conditional prediction of a target's *full* sum, not just a bit with no heldout correlation. The research-KB and Conductor conflict-check/work-claim tools were not exposed; the PR is therefore marked as uncoordinated pending external claim/independent replay. No historical negative result is reopened or renamed as an advance.

Reproduction: "python3 research/unexplored_frob_bilinear_20261118/run.py --output research/unexplored_frob_bilinear_20261118/RESULTS.json"; then "cd research/unexplored_frob_bilinear_20261118 && python3 test_run.py". All results are producer-local; no isolated CPU wall times.
