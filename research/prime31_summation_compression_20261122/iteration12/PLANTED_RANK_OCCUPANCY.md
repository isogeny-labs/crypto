# Planted four-leaf relation coverage and a balanced-sign rank obstruction

2026-11-23. Source checkpoint: \`isogeny-labs/crypto\` main \`bd20b95dce44a91d7f74a1367853e724f280a3b6\`; tracking PR #5048. Mathematical/validation-preparation only; **PCC unverified**.

## Scope

This is a **classical coupon-collector and linear-algebra application**, not a new Semaev elimination algorithm. It supplies a necessary rank-admission gate for *planted-positive* four-leaf relation fixtures. It does not describe uniform natural targets, guarantee useful rank, or imply a faster Prime31 ECDLP solver. It is disjoint from active PR #5044's Frobenius-sector Semaev decomposition, from original Rust arithmetic, and from the immutable iteration6/iteration10 evidence.

Use the inherited ordinary Prime31 fixture \`p=2147483647\`, \`n=2147446831\`, \`B=31\` distinct abscissae, and signed lifts \`±P_i\`. For each of \`R\` independent **planted** four-leaf trials, choose index labels \`I_{t,1},…,I_{t,4}\` uniformly in \`[B]\` and arbitrary signs \`e_{t,j}∈{±1}\`. The coefficient row has entries

\[
c_{t,i}=\sum_{j=1}^4 e_{t,j}{\bf1}_{I_{t,j}=i}.
\]

Two sampling variants matter: WR, independent **with replacement** within each row; NR, four **distinct** labels uniformly without replacement within each row. Trials are independent in both variants. A seed-generated target \(\sum e_{t,j}P_{I_{t,j}}\) is *not* a uniform random group target. All-output recovery can produce additional non-planted rows; the following bound only applies when at most the selected planted row per trial enters the matrix.

## Exact full-column-coverage upper bounds on rank success

Write \(\mathcal{C}\) for the event that each of the \(B\) base columns is touched by at least one of the \(R\) selected planted rows. The nonzero coefficient support is contained in the touched set, because repeated opposite signs can cancel. Therefore

\[
\Pr(\mathrm{rank}_{\mathbb F_n}(C)=B)\le\Pr(\mathcal C).
\]

Inclusion–exclusion, applied to the \(B\) events “column \(i\) is never selected,” gives the exact formulas

\[
P_{\rm WR}(B,R)=\sum_{j=0}^B(-1)^j {B\choose j}
  \left(1-\frac jB\right)^{4R},
\]

\[
P_{\rm NR}(B,R)=\sum_{j=0}^B(-1)^j {B\choose j}
  \left(\frac{{B-j\choose4}}{{B\choose4}}\right)^R,
\]

where a binomial coefficient with upper argument below four is zero for positive \(R\). The first formula allows repeated leaves and their cancellations; the second makes every planted row touch exactly four columns. These are necessary **coverage** probabilities, not actual rank probabilities and not ECDLP relation yields.

| Selected planted rows R | WR: P(all B touched) | NR: P(all B touched) |
|---:|---:|---:|
| 20 | 0.072352147204 | 0.104672283634 |
| 31 | 0.573804207996 | 0.641077520355 |
| 40 | 0.847159084759 | 0.882459223354 |
| 42 | 0.880572570337 | 0.909744182991 |
| 44 | 0.907025274047 | 0.930904267817 |
| 45 | 0.918056342218 | 0.939597851991 |

At B=31, **at least 44 WR rows** or **42 NR rows** are needed merely to make 90% complete coverage possible in these models. More rows can be required for coefficient cancellations, duplicate rows, dependencies, RHS/target-column consistency and actual useful rank. The fact that a uniform-target first-moment bound suggests a certain expected number of returned relations cannot bypass this admission gate when the workload instead uses one iid planted row per trial.

## Growing-family obstruction for this sampling model only

Let \(Z\) count never-touched columns. For WR, \(\mathbb E Z=B(1-1/B)^{4R}\). For NR, \(\mathbb E Z=B(1-4/B)^R\). In either case the indicators of two columns being never touched are nonpositively correlated: for WR \((1-2/B)^{4R}\le(1-1/B)^{8R}\), and for NR \((\binom{B-2}{4}/\binom B4)^R\le(1-4/B)^{2R}\). Hence \(\mathrm{Var}(Z)\le\mathbb EZ\), so

\[
\Pr(\mathcal C)=\Pr(Z=0)\le\frac1{\mathbb EZ}.
\]

For any fixed \(\epsilon>0\), choosing \(R\le(1-\epsilon)\frac B4\log B\) makes \(\mathbb EZ=B^{\epsilon+o(1)}\) for growing B, and therefore \(P(\mathcal C)\le B^{-\epsilon+o(1)}\). Thus **high-probability full rank in this iid planted-row architecture requires \(\Omega(B\log B)\) selected rows**, even before matrix algebra. This is a logarithmic admission correction, not a lower bound on adaptive relation finding, natural-target distribution or a different dependency architecture.

## Balanced-sign algebraic kernel

If every retained relation is built with **two plus and two minus signs**, then exactly

\[
\sum_{i=1}^B c_{t,i}=\sum_{j=1}^4e_{t,j}=0.
\]

Consequently \(C{\bf1}=0\), so \(\mathrm{rank}(C)\le B-1\) over any field. Arbitrarily many such rows **cannot** determine all B *absolute base logarithms* without an independent anchor or an unbalanced row. This remains true with repeated leaves, even if their signed coefficients cancel. A single-target mixed-RHS system might nevertheless determine the target log while retaining a common base-log shift; this lemma does **not** rule that out. Any test must compute projected target rank separately rather than equating base-column rank with target recoverability.

## Independent arithmetic checks and explicit acceptance criteria

\`coverage_oracle.py\` uses exact integer/rational arithmetic, not elliptic-curve calculations. It compares WR inclusion–exclusion with the independently evaluated Stirling-number surjection count \(B!S(4R,B)/B^{4R}\), and NR inclusion–exclusion with a separate Markov DP on the number of previously touched columns. All selected cells, and the exact 90% threshold inequalities for B=31, agree. This is one coordinator performing two algorithms, **not** independent worker verification or a signed proof.

For a future source-frozen original-Rust extractor, retain for every returned row: the target-creation mode (natural versus planted), exact four index/sign labels, signed coefficient vector after cancellation, nonzero support bitset, duplicate canonical key, group-sum replay, RHS/target column, and exact projected useful rank. Check identity subtotals, repeated leaves, \(P,P,P,-P\), rationality of all S3 auxiliaries and zero/nonzero total separately. The pass condition is equality of replayed rows and an independently recomputed rank, not merely high coverage. Compute time, hash/proof traffic, failed targets and memory are still charged. Do not run new timed measurements without the shared reservation and EKS admission.

## Prior art, checks, and boundaries

The mathematical model inherits the signed-group-semiring exact control and uniform-target warning in \`iteration8/EXACT_CONTRACTION_AND_YIELD.md\`, and the stronger pair-spectrum support review in \`iteration10/parallel_review/REVIEW.md\`. Inclusion–exclusion, occupancy and \(C{\bf1}=0\) are elementary prior mathematics. Semaev auxiliary-chain prior art: https://eprint.iacr.org/2015/310 ; generic indexing tradeoffs are not elliptic reduction algorithms: https://drops.dagstuhl.de/storage/00lipics/lipics-vol374-icalp2026/html/LIPIcs.ICALP.2026.78/LIPIcs.ICALP.2026.78.html . The shared research-KB MCP and its repository-wrapper execution were not available in this tool environment; these source reads and primary references are the documented fallback. No novelty claim for the underlying combinatorics is made.

Open gate: prove and implement an *actual* exact sublinear empty-inclusive query for the Dickson-derived Prime31 pair spectrum with near-linear charged setup and full signed output; this note improves the **validation** admission criterion only. [pcc:unverified]
