from pathlib import Path
import json,hashlib
D=Path(__file__).parent;S=Path('/var/tmp/pencil-add11-distinct-invariant-20261031-v1');h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();gate=Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs')
r={'schema':'research.independent-source-fidelity-review/v1','producer_manifest_sha256':h(S/'manifest.json'),'plan_sha256':h(D/'PLAN.md'),'source_sha256':{n:h(S/(n+'.lean')) for n in ['Add11','Controls','Readback']},'rust_source':{'path':str(gate),'sha256':h(gate),'lines':'511-532 add11;569-580 from_affine'},'compile_exit_codes':{n:json.loads((D/(n+'.receipt.json')).read_text())['exit_code'] for n in ['Add11','Controls','Readback','ReviewerControls']},'axioms':['propext','Classical.choice','Quot.sound'],'warnings':'four unnecessarySeqFocus linter warnings, no native failure','claims':[{'claim':'unit-weight distinct branch outputs q=-(x1+x2),r=x1*x2,s=y1-y2,t=x1*y2-x2*y1,z=1,w=x1-x2','finding':'matches field specialization of actual Rust branch'},{'claim':'nonzero weights and normalized support divisibility from degree-one input divisibility and x1!=x2','finding':'supported via explicit interpolation and constant Bezout identity'},{'claim':'distinct x excludes inverse and doubling guards','finding':'supported; source precedence inverse then doubling'}],'controls':['Producer full validity instance on X^5-X+1 with points(0,1),(1,1)','Producer inverse precedence at(0,0),(0,0), doubling at(0,1),(0,1), zero denominator equal-x','Reviewer full nonzero-slope instance on X^5-X+X^2 with points(0,0),(1,1)','Reviewer invalid equal-x points(0,1),(0,2) reach modeled distinct fallback although native assertion rejects; main theorem excludes them'],'scope_notes':['AMENDMENT.md explicitly replaces evaluation predicate with equivalent degree-one support-divisibility premise before proof implementation','Unit-weight field values only; no UInt64/canonical-arithmetic correspondence','Formal theorem does not establish Rust Jacobian::valid refinement','classify is faithful for stated distinct-x result, not total native classifier correspondence outside its premise','Normalized divisor validity is not a formal group-law/class-sum theorem','No generic weighted branch or whole-walk closure'],'blocking_findings':[],'scientific_execution':False,'cost_ratio':None}
(D/'review.json').write_text(json.dumps(r,indent=2)+'\n')
(D/'REVIEW.md').write_text('''# Independent unit-weight add11 distinct review

All 52 producer manifest entries match. Private Add11, Controls, separate Readback and ReviewerControls compilations exited 0. Four unnecessarySeqFocus warnings are retained; axiom readback lists only standard propext, Classical.choice and Quot.sound.

For unit weights, direct substitution in gate add11 gives q=-(x1+x2), r=x1*x2, s=y1-y2, t=x1*y2-x2*y1, z=1, w=x1-x2. Distinct x makes both source guards false (inverse first, doubling second) and satisfies the source assertion. The output support is (X-x1)(X-x2); v=(sX+t)/w interpolates y1,y2. The proof explicitly factors v-yi by X-xi, transfers input divisibility to f-v², and uses a constant Bezout identity for the two distinct supports to prove product divisibility. The normalized output theorem adds nonzero z=1 and w without assuming its conclusion.

The amendment changes curve-evaluation inputs to equivalent degree-one Mumford divisibility after a retained unavailable-API probe. It does not claim to prove the native Jacobian::valid bridge. Producer controls instantiate the full theorem at (0,1),(1,1) on X^5-X+1 and test inverse precedence, doubling, and equal-x denominator. Reviewer controls additionally instantiate nonzero interpolation slope using (0,0),(1,1) on X^5-X+X².

An explicit boundary control shows classify(0,1,0,2)=distinct, whereas the actual source asserts unequal x after both guards. These invalid equal-x inputs are outside the main theorem's distinct-x premise. This is not a blocking discrepancy for distinct_guard; it rules out interpreting classify as total native classifier semantics.

No blocking discrepancy found within unit-weight field initialization/branch validity scope. This packet does not establish word-arithmetic correspondence, generic nonunit weights, group-law class-sum correctness, or nonidentity whole-walk closure. The exact source, frozen plan, fresh receipts, controls and readback are bound in review.json. Sources consulted: producer PLAN and AMENDMENT, direct add11/from_affine source, and existing SameSupport polynomial environment. No root edit, scientific run or cost measurement occurred.
''')
(D/'overview.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="330"><rect width="1000" height="330" fill="white"/><g font-family="sans-serif" fill="#17334d"><text x="25" y="35" font-size="22">Unit-weight add11 distinct branch: independent review</text><text x="25" y="82" font-size="18">Distinct x + two degree-one validity premises</text><text x="25" y="128" font-size="18">→ v=(sX+t)/w interpolates both points; w=x1-x2 is nonzero</text><text x="25" y="174" font-size="18">→ separate linear divisibility + Bezout → quadratic support divides f-v²</text><text x="25" y="220" font-size="18">4 private compile/readback commands pass; standard axioms only</text><text x="25" y="270" font-size="16">Boundary: invalid equal-x fallback is not total native classifier correspondence.</text><text x="25" y="308" font-size="15">Field-value validity only; machine refinement and class-sum correctness are separate.</text></g></svg>''')
(D/'report.tex').write_text(r'''\documentclass[11pt]{article}
\usepackage[margin=0.8in]{geometry}
\usepackage[T1]{fontenc}
\begin{document}
\section*{Independent add11 distinct review}
31 October 2026 UTC. Scope: unit-weight field formula, normalized divisor validity and distinct-input branch selection.

\textbf{Checks.} All 52 producer manifest entries match. Private Add11, producer Controls, separate Readback and ReviewerControls compilations exited 0. Four non-fatal unnecessarySeqFocus warnings are retained. Axiom readback contains only propext, Classical.choice and Quot.sound.

\textbf{Source correspondence.} With unit weights, Rust add11 yields
\[q=-(x_1+x_2),\quad r=x_1x_2,\quad s=y_1-y_2,\quad t=x_1y_2-x_2y_1,\]
\[z=1,\qquad w=x_1-x_2.\]
Distinct $x$ excludes both guards, which test inverse before doubling, and satisfies the source assertion. The formal output retains this field specialization.

\textbf{Derivation.} The support polynomial is $(X-x_1)(X-x_2)$ and $v=(sX+t)/w$ interpolates the two input values. The proof factors $v-y_i$ by $X-x_i$, transfers input support divisibility to $f-v^2$, and explicitly constructs a constant Bezout identity to combine distinct factors. It establishes nonzero weights and normalized output validity without assuming coefficient identities or the desired product divisibility.

\textbf{Controls.} Producer tests instantiate full validity on $X^5-X+1$ with points $(0,1),(1,1)$ and check inverse precedence, doubling and equal-$x$ denominator. Reviewer tests add a full nonzero-slope instance on $X^5-X+X^2$ with points $(0,0),(1,1)$. All compile successfully.

\textbf{Classifier boundary.} A reviewer control shows that invalid equal-$x$ points $(0,1),(0,2)$ fall through the modeled classifier to distinct, while Rust then asserts unequal $x$. The theorem requires distinct $x$, so this is outside its premises; it prevents reading the model as a total native classifier correspondence.

\textbf{Scope.} No blocking discrepancy was found. An explicit prospective amendment uses degree-one support divisibility instead of an unavailable evaluation API. Native Jacobian validity refinement remains external. This is not a machine-word, generic weighted, class-sum or whole-walk theorem. No scientific run or cost measurement occurred.

Artifacts: PLAN.md, review.json, REVIEW.md, overview.svg, replay/ sources and separate native receipts/readback. Producer failures remain in the unchanged source packet.
\end{document}
''')
