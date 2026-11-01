#!/usr/bin/env bash
set -euo pipefail
cd /var/tmp/independent-czero-weighted-review-20261101
jq -Rn '[inputs | capture("^\u0027(?<name>[^\u0027]+)\u0027 depends on axioms: \\[(?<axioms>[^]]*)\\]$") | .axioms=(.axioms|split(", "))] | {declarations:.,count:length,allowed_only:all(.[]|.axioms[]; .=="propext" or .=="Quot.sound" or .=="Classical.choice")}' < ReviewerReadback.stdout > axioms.json
jq -e '.count==27 and .allowed_only' axioms.json > axiom-check.txt
sha256sum snapshot/*.lean snapshot/native-source.rs.txt > reviewed-source.sha256
jq -n --slurpfile axioms axioms.json '{schema:"pencil.formal-review/v1",scope:"C0 weighted field formulas and conditional curve membership",machine_status:"passed",fidelity:"faithful_scoped",technical_outcome:"holds_with_stated_assumptions",workflow_complete:false,workflow_deviations:["Review dispatched on sealed private inputs before producer publication; PLAN is local source-first, not precommitted or fully blind."],main_declarations:14,producer_controls:7,reviewer_controls:6,axiom_audit:$axioms[0],fresh_modules:8,scientific_execution:false,cost_ratio:null,dependency_boundary:"Retained compiled imports; no independent kernel binary rechecker or complete Mathlib rebuild.",retained_assumptions:["Field K","Four input weights nonzero","B nonzero and C zero","Monic degree-five f with coefficient four zero","Both input Mumford residual divisibilities"],unproved_bridges:["Divisor class sum","Native caller assumptions","Machine scheduling, counters and word semantics","Compiler correctness"],reviewer_failed_attempt:{preserved:"reviewer-attempt1",cause:"Expected output z factor 10; literal B=-2 and W=-10 give factor 20. Corrected reviewer control only."},publication_task:"T-1225",source_bindings:"reviewed-source.sha256",receipts:"*.receipt.json",source_unchanged_check:"after.txt"}' > status.json
cat > REVIEW.md <<'TEXT'
# Independent C0 weighted field review

Fresh Lean replay passed all eight named modules. The axiom readback covers 14 main declarations, seven producer controls and six independent controls (27 total), using only propext, Classical.choice and Quot.sound. Scoped field-formula correspondence holds with the assumptions below. This packet records technical evidence; publication/workflow completion remains separate.

## Source correspondence

The bound native snapshot comes from crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs. normalize, lines 533–566, decodes DegreeOne by x/z² and y/(z⁵w). add22_weighted, lines 899–1023, crosses the other input's q/r/s/t weights with powers 2/4/3/5 and includes the opposite w on s/t. The C=0 branch's literal B, A, x, y, z and w=1 associations agree with WeightedCZero.payload. This is field-value correspondence, not a word-level or execution-order theorem.

Write Z=z1*z2 and W=w1*w2. Source-first scaling gives Bcross=Z⁸B, Across=Z⁹WA and Ccross=Z⁷WC. The output is xnum=Z¹⁸W²*xn, ynum=Z⁴⁵W⁵*yn, z=Z⁹WB, w=1. Division by z² and z⁵w cancels the weights. The proof modules establish those identities and the correct ordinate sign.

WeightedPoint.reconstruct covers arbitrary stored D2 coordinates with nonzero z and w, by reconstructing them from their normalized coordinates. Thus composition is not restricted to inputs originally created by weight. WeightedGuards proves the normalized/crossed B-nonzero and C-zero equivalences using their stated nonzero factors. These algebraic guards do not establish caller control flow.

WeightedPoint.weighted_on_curve composes decoding with CZeroOutput.czero_output. It requires a field, four nonzero weights, B nonzero, C zero, monic f of degree five with coefficient four zero, and both input Mumford residual divisibilities. Its conclusion is nonzero output weights and y²=f(x). Divisor-class addition is a separate unproved bridge here.

## Replay and adversarial controls

Exact copied producer modules were built in snapshot with the pinned Lean toolchain by run.sh. before.sha256/after.txt bind inputs and selected retained dependencies before/after. Dependency binaries remain trusted; neither all Mathlib dependencies nor the kernel implementation were independently rebuilt/rechecked. Per-module native exits and complete output are retained.

Independent rational controls cover nonunit mixed-sign weights yielding decoded (10,291), the literal output z=20*21⁹ and w=1, omitted-w failure, zero-w singular output, reconstruction of arbitrary raw coordinates, and a nonzero-C witness. Controls use kernel tactics, not native_decide.

The first reviewer output-z control expected 10*21⁹ and failed. Inspection found B=-2 and W=-10, giving 20*21⁹. The first source and failure receipt remain in reviewer-attempt1; only the reviewer expectation was corrected. Producer inputs were unchanged. Final six reviewer controls pass.

## Review independence and boundaries

The reviewer inspected native source and producer status/plan, wrote a local source-first plan, and then inspected proof bodies and replayed them. This was not a fully blind derivation and the plan was not precommitted. Dispatch preceded producer publication; root is recording that sequencing deviation and publishing through T-1225. No claim of a completed prescribed review workflow is made. Exact reviewed bytes are in reviewed-source.sha256; root must bind them to the published archive.

Unproved transitions are native caller validity, divisor-class sum, word arithmetic/counters/scheduling and compiler correctness. No scientific experiment or new measurement was performed. No cost ratio is inferred. The source-linked status JSON, receipts, controls and diagram support independent assessment of these scoped results.
TEXT
cat > report.tex <<'TEXT'
\documentclass[10pt]{article}
\usepackage[margin=0.8in]{geometry}
\usepackage[T1]{fontenc}
\usepackage{lmodern,amsmath,hyperref}
\title{Independent review: weighted C0 branch}
\date{1 November 2026}
\begin{document}\maketitle
Fresh isolated Lean replay passed eight modules: fourteen main declarations, seven producer controls and six independent controls. All 27 axiom readbacks use only propext, Classical.choice and Quot.sound. The scoped field formulas agree with the inspected native source.
\section*{Formula correspondence}
Native \texttt{projective\_law\_gate.rs}, normalize lines 533--566, decodes degree-one coordinates as $x/z^2$ and $y/(z^5w)$. The weighted addition branch, lines 899--1023, crosses the other input's weights. With $Z=z_1z_2$ and $W=w_1w_2$, the checked scaling is
\[B_c=Z^8B,\quad A_c=Z^9WA,\quad C_c=Z^7WC.\]
The literal output obeys
\[x_o=Z^{18}W^2x_n,\quad y_o=Z^{45}W^5y_n,\quad z_o=Z^9WB,\quad w_o=1.\]
Decoding cancels the weights. Reconstruction covers arbitrary stored inputs with nonzero weights, and the B/C guard equivalences retain their explicit nonzero-factor assumptions.
\section*{Conditional curve membership}
Composition requires a field, four nonzero input weights, $B\ne0$, $C=0$, a monic degree-five polynomial with coefficient four zero, and both input Mumford residual divisibilities. It concludes nonzero output weights and $y^2=f(x)$. Divisor-class addition, native caller validity, machine words, counters, scheduling and compiler correctness remain separate bridges.
\section*{Independent boundary controls}
The six independent controls cover mixed-sign nonunit weights with decoded $(10,291)$; output $z=20\cdot21^9$; missing-$w$ normalization failure; zero-$w$ singular output; arbitrary raw reconstruction; and nonzero $C$. They use kernel tactics rather than native evaluation. A reviewer initially expected factor 10 instead of 20 in the output-$z$ control. That failed attempt is preserved; only the reviewer's expectation changed. No producer repair occurred.
\section*{Evidence and process}
\texttt{run.sh}, module receipts, \texttt{before.sha256}, \texttt{after.txt} and \texttt{reviewed-source.sha256} bind the local replay. Retained dependency binaries remain a trust boundary; there was no complete dependency rebuild or independent kernel binary check.

The reviewer read native source and producer status/plan before writing a local source-first plan, then inspected proof bodies. This was not fully blind or precommitted. Review dispatch preceded producer publication; T-1225 records that sequencing deviation and publication binding. Workflow completion is not asserted.

\texttt{REVIEW.md} and \texttt{status.json} provide source references, assumptions, failures and scope. No scientific run or new measurement was performed; no cost ratio is inferred.
\end{document}
TEXT
cat > flow.svg <<'TEXT'
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="540" viewBox="0 0 1000 540"><rect width="1000" height="540" fill="white"/><g font-family="sans-serif" fill="#182c3b"><text x="40" y="42" font-size="23">Weighted C0 field review — checked dependencies</text><g fill="#eaf2f7" stroke="#476b81"><rect x="40" y="70" width="420" height="84" rx="8"/><rect x="540" y="70" width="420" height="84" rx="8"/><rect x="40" y="208" width="420" height="92" rx="8"/><rect x="540" y="208" width="420" height="92" rx="8"/><rect x="250" y="358" width="500" height="88" rx="8"/></g><g font-size="17"><text x="60" y="99">Native crossed weights and C0 payload</text><text x="60" y="128">D1 decode: x/z², y/(z⁵w)</text><text x="560" y="99">Nonzero weights; normalized B ≠ 0</text><text x="560" y="128">C = 0; input residual divisibilities</text><text x="60" y="238">Scaling + arbitrary-input reconstruction</text><text x="60" y="270">B/C guards; decoded payload identity</text><text x="560" y="238">Monic quintic; coefficient four = 0</text><text x="560" y="270">Retained CZeroOutput theorem</text><text x="270" y="389">Conditional output curve membership</text><text x="270" y="420">Fresh replay: 14 main + 13 controls</text></g><g stroke="#476b81" stroke-width="2" fill="none"><path d="M250 154V208M750 154V208M250 300L400 358M750 300L600 358"/></g><text x="40" y="485" font-size="16">Outside scope: class sum, native caller validity, word scheduling, counters, compiler.</text><text x="40" y="514" font-size="16">Source-first private review; publication sequencing deviation recorded. No measurements.</text></g></svg>
TEXT
pdflatex -interaction=nonstopmode -halt-on-error -output-directory . report.tex > render.log
pdftoppm -scale-to 1100 -png report.pdf page
rsvg-convert flow.svg -o flow.png
