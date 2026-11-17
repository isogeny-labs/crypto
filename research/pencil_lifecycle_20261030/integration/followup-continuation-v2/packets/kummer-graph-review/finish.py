from pathlib import Path
import json,hashlib
D=Path(__file__).parent;S=Path('/var/tmp/pencil-kummer-graph-integration-20261031');h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert (D/'snapshot/run.stdout').read_bytes()==(S/'run.stdout').read_bytes()
review={'schema':'research.independent-source-review/v1','producer_manifest_sha256':h(S/'manifest.json'),'review_plan_sha256':h(D/'PLAN.md'),'source_hashes':{p:h(S/p) for p in ['src/graph.rs','src/pruned.rs','src/eager.rs','src/check.rs','native/src/three_leads.rs']},'custody_checked_files':30,'custody_mismatches':[],'native_replay_exit_code':json.loads((D/'snapshot/receipt.json').read_text())['exit_code'],'native_stdout_identical':True,'independent_python_prefix_checks':70,'control_prefix_checks':9,'observations':json.loads((D/'independent-counts.json').read_text()),'claims':[{'claim':'Recovered original-G states feed existing signed graph interface before reference construction','finding':'supported for fixed fixtures by source and replay'},{'claim':'Both arms insert 35 identical equations from 63 states','finding':'supported; independent raw recount and replay'},{'claim':'Degree-one recovery adds no graph edge','finding':'supported; explicit degree!=2 early return'},{'claim':'Every inserted prefix agrees with separately implemented dense elimination','finding':'supported; Rust matrix implementation separate from signed union-find; reviewer Python implementation confirms 70 prefixes'},{'claim':'Synthetic controls exercise stopping at prefixes 3,2,4','finding':'supported; controls are not geometric cycles'}],'scope_notes':['21 retained models, three original-G transitions each, separate graph per model and arm','Zero geometric cycles and zero geometric terminal returns','Dense elimination checks formal signed equations over checked prime moduli, not individual-node subgroup membership','Historical collector filters before signed recovery; this wrapper recovers first, so scheduling and total-cost equivalence are not established','Duplicate and repeated-root geometric paths unobserved; inconsistent trailing synthetic sentinel intentionally unconsumed','Degree-one, Delta-zero, generic/fallback, and other earlier packets retain their separate scope'],'blocking_findings':[],'scientific_execution':False,'new_cost_ratio':None,'official_scientific_decision':'not made'}
(D/'review.json').write_text(json.dumps(review,indent=2)+'\n')
(D/'REVIEW.md').write_text('''# Independent graph-adapter review

The private replay exited 0 and reproduced producer stdout byte for byte. All 30 manifest entries matched. A separate Python elimination implementation checked all 70 inserted prefixes and nine synthetic control prefixes, including prime-modulus validation.

On each arm's 63 original-G transitions, 35 recovered degree-two divisors produce the same signed equations and 26 are nonsplit. The eager arm retains two recovery failures; the candidate recovers those two degree-one divisors, which the explicit degree gate skips. Both graphs have zero cycles and zero terminal returns. Edge gain is zero in this fixture.

The source passes the recovered divisor directly into Adapter.consume before building aG+bQ. The adapter splits the monic quadratic, extracts v(x), assigns canonical signs, deduplicates up to negation, and inserts su*Xu+sv*Xv=a+b*k. The dense reference represents this as [node coefficients,-b | a]. Its independent matrix elimination removes node coefficients; a nonzero remaining target coefficient determines k. The production SignedGraph instead uses signed union-find potentials. Reviewer elimination reconstructed all emitted prefixes and obtained no target value, matching both implementations. All geometric rows have independent full row rank.

Synthetic controls over F101 independently return 7 at prefixes 3,2,4. The repeated-edge control has a target-free cycle before its terminal row; the negative-cycle control anchors a node. An inconsistent trailing sentinel is never consumed after a returned value. These controls remain separate from the geometric trajectory; they do not supply its absent cycle or terminal coverage.

No blocking discrepancy found within the stated fixture claims. This is emitted-equation integration: historical production scheduling filters before recovery whereas this wrapper recovers each state first. The checker does not prove individual graph nodes lie in the prime-order subgroup. No geometric return branch, repeated-root gate, or duplicate gate was exercised. Reports preserve those boundaries and do not assign a timing or total-cost ratio.

Review source bindings and native receipt are in review.json and snapshot/receipt.json; PLAN.md preceded source and raw inspection. No producer repair, root build, cloud action, or scientific run occurred.
''')
(D/'overview.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="950" height="360" viewBox="0 0 950 360"><rect width="950" height="360" fill="white"/><g font-family="sans-serif" fill="#17334d"><text x="25" y="35" font-size="22">Fixed original-G trajectory: graph-adapter review</text><text x="25" y="67" font-size="15">21 models x 3 transitions per arm; independent replay and formal equation checks</text><rect x="25" y="93" width="410" height="145" rx="8" fill="#edf4f8"/><rect x="465" y="93" width="455" height="145" rx="8" fill="#edf4f8"/><text x="45" y="123" font-size="19">Eager: 63 states</text><text x="45" y="154" font-size="17">35 inserted + 26 nonsplit + 2 failed</text><text x="45" y="185" font-size="17">0 cycles; 0 terminal returns</text><text x="485" y="123" font-size="19">Candidate: 63 states</text><text x="485" y="154" font-size="17">35 inserted + 26 nonsplit + 2 degree-1 skips</text><text x="485" y="185" font-size="17">0 cycles; 0 terminal returns</text><text x="45" y="217" font-size="15">Same 35 signed equations per arm</text><text x="25" y="277" font-size="17">Separate F101 synthetic controls: target 7 at prefixes 3, 2, 4</text><text x="25" y="311" font-size="15">70 geometric prefixes + 9 control prefixes checked by independent Python elimination</text><text x="25" y="339" font-size="14">Fixed software checks; no timing or total-cost ratio. Source: review.json / independent-counts.json</text></g></svg>''')
(D/'report.tex').write_text(r'''\documentclass[11pt]{article}
\usepackage[margin=0.8in]{geometry}
\usepackage[T1]{fontenc}
\usepackage{booktabs}
\usepackage{amssymb}
\usepackage{hyperref}
\begin{document}
\section*{Independent graph-adapter review}
Private component evidence, 31 October 2026 UTC. Producer packet:\\ \texttt{pencil-kummer-graph-integration-20261031}.

\textbf{Replay and custody.} All 30 producer manifest entries match. Private offline native replay exited 0 and reproduced stdout byte for byte. A separately written Python matrix eliminator checked 70 geometric prefixes and nine synthetic prefixes. All 21 subgroup moduli per arm were checked prime.

\begin{center}\begin{tabular}{lrr}\toprule
Observation & Eager & Candidate\\\midrule
Original-G transitions &63&63\\
Inserted equations &35&35\\
Nonsplit quadratics &26&26\\
Recovery failures &2&0\\
Degree-one gate skips &0&2\\
Geometric graph cycles &0&0\\
Geometric terminal returns &0&0\\\bottomrule
\end{tabular}\end{center}

The 35 equations are identical between arms. The two additional recovered degree-one divisors are rejected by the degree-two graph interface, so emitted-edge gain is zero in this fixed fixture. No trajectory was extended to seek a cycle.

\textbf{Source correspondence.} The adapter consumes the recovered signed divisor before constructing the reference $aG+bQ$. Quadratic roots and $v(x)$ supply signed point nodes. The equation is $s_u X_u+s_v X_v=a+bk$, represented by dense elimination as $[A,-b\mid a]$. The signed union-find implementation and the independently implemented dense matrix reference agree at every insertion. Reviewer Python elimination also agrees. Every geometric row increases row rank, with no target-only equation.

\textbf{Separate stopping controls.} Synthetic signed equations over $\mathbb F_{101}$ return the planted value 7 at prefixes 3, 2, and 4. They cover positive-cycle stopping, negative-cycle anchoring, and a target-free repeated-edge cycle followed by stopping. Each intentionally unconsumed trailing sentinel is inconsistent. These are equation controls, not geometric cycle observations.

\textbf{Scope.} No blocking discrepancy was found in these component claims. The historical collector filters before recovery; this wrapper recovers before filtering. Thus emitted-equation semantics are checked, not production scheduling or total costs. Individual graph nodes are not proved to lie in the prime-order subgroup. Geometric scalar-return, duplicate, and repeated-root branches were not exercised. No timing or cost ratio is assigned.

\textbf{Artifacts.} \texttt{review.json} contains exact source hashes and scoped findings; \texttt{PLAN.md} preceded source/raw inspection; \texttt{snapshot/receipt.json} retains the native command and exit; \texttt{independent-counts.json} and \texttt{overview.svg} provide structured counts and diagram. No producer repair, root build, cloud action, or scientific run occurred.
\end{document}
''')
