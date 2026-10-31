from pathlib import Path
import hashlib,json,subprocess
p=Path(__file__).parent;sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
paths=[Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs'),Path('/var/tmp/pencil-add11-distinct-independent-review-20261031/replay/Add11.lean'),Path('/var/tmp/pencil-add11-distinct-independent-review-20261031/replay/Add11.olean')]
(p/'sources.json').write_text(json.dumps({str(f):sha(f) for f in paths},indent=2)+'\n')
(p/'status.json').write_text(json.dumps({'scope':'weighted degree-one doubling field polynomial validity','theorems':7,'final_compile_exit_code':0,'controls_exit_code':0,'readback_exit_code':0,'initial_attempt_exit_code':1,'independent_review':None,'requires_nonzero_derivative':False,'requires_global_nonsingularity_for_this_claim':False,'self_add_guard_only':True,'word_execution_bridge':False,'scientific_execution':False},indent=2)+'\n')
(p/'diagram.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="950" height="260"><rect width="950" height="260" fill="white"/><g font-family="sans-serif" fill="#182433"><text x="20" y="32" font-size="22">Weighted doubling: local field preservation</text><rect x="20" y="55" width="910" height="45" fill="#e8eff5"/><text x="35" y="85" font-size="18">z,w,y,2 nonzero + normalized point equation</text><text x="35" y="135" font-size="18">Normalize literal output → u=(X−xx)², v=sX+(yy−s·xx)</text><text x="35" y="180" font-size="18">s=f′(xx)/(2yy); explicit Taylor quotient → u divides f−v²</text><text x="35" y="230" font-size="17">y=0 uses earlier inverse branch. Zero derivative is permitted.</text></g></svg>''')
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'figure.pdf'),str(p/'diagram.svg')],check=True)
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}\usepackage[margin=.7in]{geometry}\usepackage{graphicx}\begin{document}\section*{Weighted degree-one doubling}\includegraphics[width=\linewidth]{figure.pdf}
Seven Lean declarations compile. Separate controls and axiom readback return exit zero. The weighted field formula preserves normalized Mumford divisibility on $f=X^5+a_3X^3+a_2X^2+a_1X+a_0$ under the displayed assumptions.

\paragraph{Construction} The output has $z'=z$, $w'=2y$. Its normalized quadratic is $(X-xx)^2$ and its linear polynomial is the tangent interpolant. An explicit quotient identity proves divisibility; input point validity and the tangent equation cancel the residual terms.

\paragraph{Branches and controls} Self-add with nonzero ordinate reaches doubling; zero ordinate reaches the earlier inverse branch. A nonunit-weight fixture instantiates the full theorem. A separate zero-derivative fixture also instantiates it. Derivative nonvanishing is not a required denominator condition.

\paragraph{Scope} This is field polynomial validity. Global nonsingularity and class-sum interpretation are separate; UInt64 execution, counters, compiler and walk closure are not proved. The curve preprocessing is not included. The guard lemma covers self-add, not every differently weighted equal-point input. The first algebra fixture uses $f=X^5$ and is not presented as a smooth curve.

\paragraph{Evidence} Initial failed attempt retained; corrected modules, controls and readback pass. Only standard Lean axioms occur. REPORT.md records exact formula/source boundaries and literature equation(3); sources.json and the manifest bind artifacts. Independent review pending. No scientific trials or repository edits.
\end{document}''')
r=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,capture_output=True);(p/'render.stdout').write_bytes(r.stdout);(p/'render.stderr').write_bytes(r.stderr);assert r.returncode==0
subprocess.run(['pdftoppm','-f','1','-singlefile','-scale-to','1000','-png',str(p/'report.pdf'),str(p/'preview')],check=True)
(p/'manifest.json').write_text(json.dumps({'files':{str(q.relative_to(p)):sha(q) for q in sorted(p.rglob('*')) if q.is_file() and q!=p/'manifest.json'}},indent=2)+'\n');print('source',sha(p/'Doubling.lean'));print('manifest',sha(p/'manifest.json'))
