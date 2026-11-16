# Selector A: bounded kernels and their symbolic union

This assessment follows candidate A in [PR4338](https://github.com/isogeny-labs/crypto/pull/4338): colored rational interpolation on prime-field elliptic points. It supplies a shared formal component and an exact symbolic reformulation. The source builds and canonical audit pass locally; signed PCC standing is separate. [pcc:unverified]

## What is established at the source level

For a field F, injective abscissas x, arbitrary ordinates y, and a selected support D, use the actual bounded polynomial state

    K_D = {(u,v): natDegree u <= dU, natDegree v <= dV,
                    u(x_j) + y_j*v(x_j) = 0 for j in D}.

Mathlib's existing nodal polynomial h_D separates supports: for |D| <= dU, (h_D,0) is nonzero and belongs to K_E exactly when E is a subset of D. Thus equal kernels recover equal supports when both support sizes are within the cap. With t colors and b records per color, the range of the colored-prefix kernel map has cardinality exactly b^t for t <= dU. These are laws of actual polynomial subsets over a field; the assembly also applies them unchanged to clean nonsingular Weierstrass coordinates over ZMod p. [pcc:unverified]

The positive counterpart keeps the union implicit:

    P_i(u,v) = product_(j in Fin b) (u(x_ij) + y_ij*v(x_ij)).
    [both degree caps and all P_i=0]
        iff [there exists c with (u,v) in K_(D_c)].

The nonzero-existence endpoint retains (u,v) != 0 on both sides. The union equivalence needs no abscissa-injectivity, curve or sampling hypothesis. It is an exact representation of existential choice, rather than vanishing at every point in a color. [pcc:unverified]

The [owner report](owner/docs/interpolation-kernels-20261115.md) contains the full statement/inheritance table, scoped cost derivation and failure history. Its [three-page PDF](owner/docs/interpolation-kernels-20261115/report.pdf), [editable diagram](owner/docs/interpolation-kernels-20261115/flow.dot) and [SVG](owner/docs/interpolation-kernels-20261115/flow.svg) are byte-identical to the inspected owner artifacts. All three final PDF pages were rendered and inspected before this snapshot. [pcc:unverified]

## Consequence for one explicit architecture

Set dU=floor(m/2), dV=floor((m-3)/2), m>=3. An architecture that materializes the entire level t=dU, stores one entry per exact kernel, merges only equal kernels and prunes only nonzero-infeasible current constraints needs b^t entries: every prefix at that level has a nonzero nodal witness. With b=ceil(r^(1/m)), this is at least r^(1/2-1/(2m)). At m=ceil(sqrt(n)), n=ceil(log2 r), its base-two logarithm is at least n/2-O(sqrt(n)). This is a hand storage consequence for whole-level materialization, under input lengths polynomial in n. [pcc:unverified]

It is not a runtime or visited-state lower bound for lazy search, early stopping, selective search, future-color pruning, coarser states with a justified lifting interface, or symbolic algorithms. The finite endpoint does not eliminate candidate A. The symbolic products exhibit one representation that avoids explicitly listing the union. [pcc:unverified]

For a supplied bounded pair, Horner evaluation and factor scans give a hand membership-check budget O(t*b*(dU+dV+1)) field operations. This is neither a measured program nor the cost of finding the pair. The homogeneous zero pair is inadmissible. [pcc:unverified]

## Request and acceptance criteria

| Requested component | Current evidence | Status |
|---|---|---|
| New selector ideas beyond the tensor route | Three hypotheses in PR4338; this assessment addresses A | Proposed; unimplemented |
| Check the actual interpolation state | 18 shared laws, generic and actual-prime-field probes | Locally checked; PCC unverified |
| Avoid enumeration by exact-kernel equality alone | b^t distinct kernels for the specified full level | Scoped finite count; hand cost consequence |
| Keep the colored union implicit | Exact factored-product equivalence, including nonzero existence | Locally checked; extractor missing |
| Demonstrate a complete end-to-end algorithm | No capped nonzero extractor or useful-yield bound supplied | Unmet |

The next acceptance test is constructive: on the actual clean random elliptic input distribution, return a nonzero bounded pair satisfying every color product, with known per-attempt cost

    T <= 2^(C*m*log(m+1)) * (m*b)^c * N^c
    theta >= 2^(-C*m*log(m+1)),

for fixed constants C,c, where N is the supplied input bit length and theta is useful-output probability after actual relation/label checks. These remain targets from the proposal. Charge input construction, all failed attempts, extraction, decoding, label recovery and final checks. The existing divisor/group interpretation and scalar-label obligations are separate from the polynomial membership component. [pcc:unverified]

This exact witness-search gap is linked from the existing [complete-cost problem](../../problems/aps-single-relation-complete-cost.md). No native numerical case, paid job, qualification run, fitted exponent, completed selector, novelty claim or speedup is reported. The frozen multiprecision panel and its checkpoint-access requirement remain unchanged. [pcc:unverified]

## Source and evidence boundary

Owner: [aburan28/crypto-lean](https://github.com/aburan28/crypto-lean), snapshot commit `99a965a92d301b1349051d2b89ae9e9cdcd1b995`; mathematical source `b94180fe11689d0af8b086d2dbe28b460bf58078`; gate dependency `f319f7affaa8144bb28035d5f907a6dbbc904a18`. Toolchain Lean 4.35.0, Mathlib `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`. [pcc:unverified]

- Prospective six-field plans and inherited-law floor probes precede both new source families; the generic laws live in their shared owner. This consumer carries byte snapshots with `.lean.txt` suffixes, not duplicate modules.
- Focused builds, floor probes and final generic/actual-field assembly pass. Explicit axiom output for all 18 named laws lists only propext, Classical.choice and Quot.sound. [pcc:unverified]
- Canonical gate003 audits 167 theorems, 0 failures, 0 trusted-native exceptions. All 11 existing gate controls pass. No lean4checker executable is present; no fresh-checker pass is asserted. [pcc:unverified]
- Failed PATH/build floor001 and canonical gates001/002 remain intact. The gate dependency only recognizes Lake's additional unregistered-source diagnostic in its existing direct-compile fallback. All tracked sources remain compiled and audited; source enumeration, allowlist and axiom policy are unchanged. Three diagnostic sources were renamed byte-for-byte to valid Lean module paths after an explicit String.toName reproduction. [pcc:unverified]
- [artifact-index.json](artifact-index.json) maps every original owner path to its exact consumer bytes/hash. Original receipt-relative paths retain their owner context. Sources, plans, raw failures, controls, report and final artifacts are preserved.

This is an independent main-based assessment branch. It does not extend PR4311 or PR4338 through their frozen PCC-history conflicts, select either conflicting history, edit their claims or rechain their ledgers. Source checks are separate from hosted checks and signed PCC standing.

## Prior work used

KB unavailable in the inherited runtime: no loaded research-kb connector and no usable fallback executable. The review instead used the frozen candidate-A plan and earlier coordinate-join assessments, exact Mathlib nodal/cardinality/product-zero declarations, and primary sources recorded in the owner report. No novelty claim is made.

- [Jeannerod, Neiger, Schost and Villard, Computing minimal interpolation bases](https://cs.uwaterloo.ca/~eschost/publications/jsc_final.pdf), sections 1.1-1.2/Definition 1.1: linear interpolation constraints are supplied; this does not give a colored existential root selector.
- [Cheng, Hard Problems of Algebraic Geometry Codes](https://arxiv.org/html/cs/0507026), sections 2-5: the elliptic divisor/coding connection supplies context, not a distribution-specific extractor or lower bound for this search.
- [Zhang and Liu, Solving ECDLP via List Decoding](https://doi.org/10.1007/978-3-030-31919-9_13): publisher metadata/abstract only; full ePrint access was denied. No full algorithm or complexity was imported.

The proposal's original diagram and canonical measured charts remain unchanged: this component has no measured data or new completed-algorithm edge.
