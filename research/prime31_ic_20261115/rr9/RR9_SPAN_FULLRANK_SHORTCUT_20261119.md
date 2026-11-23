# Prime31 RR9: exact full-rank exterior-span shortcut (post-hoc analytic result)

**2026-11-19 UTC.** Companion to [PR #4779](https://github.com/isogeny-labs/crypto/pull/4779) and its pre-registered [Pluecker-span protocol](RR9_PLUCKER_SPAN_PROTOCOL_20261119.md). This is a *new exact algebraic implication* and independently coded Python finite-field preflight; **not** a claim that the pending Rust producer/checker gate or signed PCC validation has executed. The shortcut supersedes the need to enumerate 65,536 Pluecker vectors *for the purpose of establishing the ranks in the four frozen cases*. It does not supersede the registered replay/mutation requirements.

## Theorem: one full-rank support per slot forces rank 70

Let K be a field and V=K^8. For j=1,2,3,4 let S_j be finite sets of vectors that span V. By multilinearity of the exterior product,

span_K {s_1 ∧ s_2 ∧ s_3 ∧ s_4 : s_j in S_j} = Λ^4(V).

**Proof.** Multilinearity permits replacing each s_j by an arbitrary linear combination, independently in each slot. Every pure wedge v_1∧v_2∧v_3∧v_4 is therefore in the left span; the pure wedges of basis vectors form the standard basis of Λ^4(V). Thus the span dimension is C(8,4)=70. QED.

In the parent RR9 construction the left four slots and the right four slots each independently meet this condition when each *individual slot* has row rank 8. The complement/sign pairing J in the Laplace expansion permutes and signs the standard 70 basis vectors, hence is nonsingular. Consequently rank(B_L J B_R^T)=70 as soon as the left/right wedge spans are both Λ^4(V).

This conclusion does **not** need testing any left-right candidate pair or constructing the 70-minor vector tables.

## Exact one-slot determinant certificate

For target R=(r,s) and signed leaf (x,y), the parent row is

T_R(x,y) = ((x-r)(1,x,x²,x³), (y+s)(1,x,x²,x³)).

Take four underlying leaves (x_i,y_i), 0<=i<4, with distinct abscissae; from a given slot choose its four positive rows followed by their four negative rows. If \(D\) is their resulting 8x8 row determinant, then row subtraction gives the exact identity

\[
D = (-2)^4 \Big(\prod_{i=0}^{3}(x_i-r)y_i\Big)
          \Big(\prod_{0\le i<j<4}(x_j-x_i)\Big)^2
  =16\prod_i(x_i-r)y_i\prod_{i<j}(x_j-x_i)^2.
\]

The target ordinate s cancels. Over a field of characteristic !=2, if every y_i !=0, all x_i are distinct and no x_i=r, then D!=0: eight signed rows of that single slot form a basis of K^8. Applying the exterior lemma to each half gives **left rank = right rank = pairing rank = 70**. This proof is independent of the hidden target logarithm and does not appeal to EC sums during extraction.

The conditions are *sufficient*, not necessary. If a chosen four points fail them, another 4-tuple might still establish full rank. **Stronger all-target corollary:** For fixed eight weights and a fixed four-point support, let F be the union of the 32 weighted abscissae \(x([w_j]P_i)\), \(0\le j<8\), \(0\le i<4\). As long as those four points in every slot have nonzero ordinates and distinct abscissae, the 70/70/70 conclusion holds for **every finite public target** \(R\) with \(x(R)\notin F\), independently of its scalar and ordinate. On a prime-order elliptic-curve group each excluded abscissa has at most two finite points, so the conclusion fails to be certified by this simple witness on at most \(2|F|\) target points; those points may still have full rank by a different witness.

## Original ordinary Prime31: exact finite-field preflight

Reuse the pinned original curve and arithmetic: p=2147483647, E: y²=x³+847514846x+2087444724, n=2147446831 (prime), G=(625037715,2100627251); slot weights [1,3,5,7,11,13,17,19]. Four canonical base points with x=0,1,2,3 have y = [309380359,749117756,51658786,166645833]. The eight x-sign support rows of each weighted slot use +/-[w]P_i for those four base points. Both frozen residuals are tested: N=G and P=sum_j[w_j]P_0=(1991076510,718644930). The B8 case contains the same first four base points as B4.

A separately implemented Python modular EC group calculation and two **independent determinant evaluators** (8x8 Gaussian elimination and the above closed formula) agree on all 32 slot/case combinations. All determinants are nonzero; no tested slot has zero y, repeated x among these four points, or x=r.

| Case | Eight exact nonzero determinant residues, slot weights in order |
|---|---|
| N4 (also N8) | 1468457909, 1667486837, 936410919, 886543102, 2137754017, 1290111997, 1235269058, 1541396348 |
| P4 (also P8) | 1418252579, 1982109171, 1921836591, 1045673972, 1683787312, 2146239327, 1923764810, 754634012 |

Result: for **N4, P4, N8 and P8**, the exact restricted exterior pairing is full rank **70** by theorem, conditional only on the already checked one-slot determinant values. More strongly, the 32 checked weighted support abscissae are distinct, so for **every finite \(R\) on this fixed Prime31 curve except at most 64 points**, the same explicit four-pair certificate proves both half ranks and restricted pairing rank exactly 70, for any B>=4 whose slot supports include the first four base points. Here \(n=2147446831\), so the exceptional set is at most 64/n of the full group (approximately 2.9815e-8); no independence assumption or probabilistic heuristic is used. At the exceptional targets the result is simply unproved by this particular certificate, not shown false. This is a narrower mathematical result than executing the frozen entire-span protocol; no producer/checker/PCC result is inferred. `[pcc:unverified]`

Receipt digest: SHA-256 of compact lexicographically sorted JSON object containing the four determinant arrays is `a899004552f53ba897d1a3145ea420dcd112686c1b35f5bfa7f4cebb5612e2a3`. The B8 entries reuse the B4 first-four subset and are intentionally identical.

A minimal **Rust check using the repository's existing Prime31 `Arithmetic`** is checked in alongside this note as [`prime31_rr9_analytic_span.rs`](../native/examples/prime31_rr9_analytic_span.rs). It recomputes the 32 determinants by Gaussian elimination and the formula and asserts the eight constants for N and P. Owner-waived local attempt 04 passed its focused test and produced byte-identical repeated outputs; attempts 01--03 preserve the formatting failure and two incorrect frozen P-case constants that validation exposed. The complete local transcript and hashes are in [`analytic_span_local_20261119/`](analytic_span_local_20261119/). It has **not** passed source-bound CI in this turn. `[pcc:unverified]`

## Scientific decision and what to work on instead

The pre-registered decision rule in #4779 says `KILL_FIXED_LINEAR_COMPRESSION` if both B8 residuals have 70/70/70 rank and independent replay passes. The mathematical rank condition is analytically established by explicit full-rank 8x8 minors, and owner-waived local protocol attempt 03 passed its independent checker and mutation gates. The source-bound CI and signed PCC gates remain absent. This kills only fixed *linear-message dimension reduction* of the 70-term wedge pairing on these supports. It does **not** give an Omega(N²) lower bound for nonlinear matching, SAT/F4/XL, orthogonal-vector indexing, or a complete ECDLP algorithm. `[pcc:unverified]`

The productive experiment is not another chart-preprocessing constant. Reuse the existing Rust four-B8-window rank31/1,024-target pipeline from #4728, and replace the 4+4 generic sum-table oracle with a genuinely nonenumerative **public-only RR9/F4/remainder-tower** extractor under #4692. Gate on the frozen 46-positive/18-negative B8 panel (full witness replay, no secret witness/index or EC half-table use, count misses/peak memory) before any same-runner cold/64/1024 comparison with signed shared BSGS and optimized rho. On the existing measured same-runner original-affine controls in #4774, 1,024 logs cost 7.835s by W8 IC vs 3.343s by signed shared BSGS; those controls are *not* an algebraic solver advantage.

## Prior scope, honesty, and follow-up

Checked active #4692 (remainder-tower unknown-index owner), #4779 (exterior-span lane), #4776 (exterior-4), #4780/#4783 (Grassmann and cached chart preprocessing), #4728 (four-window rank), #4774 (matched BSGS), and published Petit–Kosters–Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1), Amadori–Pintore–Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009), Kudo–Yokota–Takahashi–Yasuda (2018). These are prior index-calculus methods, not sources of this elementary multilinear argument. Shared research-KB server and CLI are not accessible in the connector-only environment, so novelty beyond the bounded audit is not claimed. No source-exact Rust execution, signed PCC certificate or cheaper complete prime-field non-generic solver is claimed.
