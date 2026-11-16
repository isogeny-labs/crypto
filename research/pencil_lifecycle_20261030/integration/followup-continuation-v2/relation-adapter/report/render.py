from pathlib import Path
import json,subprocess,hashlib
p=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/relation-adapter');o=p/'report';o.mkdir(exist_ok=True)
(o/'correspondence.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="280"><rect width="1000" height="280" fill="white"/><g font-family="sans-serif" fill="#183247"><text x="30" y="38" font-size="25">Collector row to diagnostic equation</text><rect x="30" y="65" width="420" height="135" rx="8" fill="#eaf0f6"/><rect x="550" y="65" width="420" height="135" rx="8" fill="#e7f3ed"/><text x="50" y="105" font-size="23">Native: [A, b, −a]</text><text x="50" y="155" font-size="23">Ax + bk = −a</text><text x="476" y="135" font-size="32">→</text><text x="570" y="105" font-size="23">Diagnostic: [A, b, +a]</text><text x="570" y="155" font-size="23">Ax + bk + a = 0</text><text x="30" y="245" font-size="18">Negate the final augmented column; retain nuisance and target coefficients.</text></g></svg>''')
subprocess.run(['rsvg-convert','-f','pdf','-o',str(o/'correspondence.pdf'),str(o/'correspondence.svg')],check=True)
(o/'report.tex').write_text(r'''\documentclass[11pt]{article}
\usepackage[margin=0.8in]{geometry}\usepackage{amsmath,graphicx,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{8pt}
\begin{document}
{\LARGE Signed relation-row correspondence}\par
2026-10-30. Offline adapter and fixed software checks.

The collector constructs $d=\tau(aG+bQ)$ and verifies that its complementary points sum with $d$ to zero. The resulting relation has form $Ax+bk+a=0$. Native augmented storage places $-a$ in its last column, interpreted as a right-hand side. The existing information diagnostic instead expects its last column as a constant on the left.

\includegraphics[width=\textwidth]{correspondence.pdf}

\textbf{Implementation.} A separate offline adapter validates the prime-field domain and row widths, reduces coefficients, and negates only the final augmented column. Rank and consistency predicates are unchanged by this nonzero column scaling, but concrete solution interpretation now agrees with the signed equation.

\textbf{Fixed controls.} Three native release tests passed. One fixture reconstructs the unique assignment $(x,k)=(2,3)$ over $\mathbf F_7$. Exhaustive checks cover all 343 single-row augmented systems over $\mathbf F_7$ and all 49 assignments for each, comparing the original right-hand-side equation to the converted zero equation. Wrong domains, widths, overflowing column counts and noncanonical inputs are checked. Strict Clippy passed.

\textbf{Scope.} This is an adapter for supplied rows, not production trace capture. It performs additional elimination and copies outside measured collection. It provides no historical per-relation costs, collection-policy timing or cohort result. Independent source review is pending in a separate packet. Future ingestion must bind actual producer source, scalar modulus, column identities, admission status and phase-cost records.

\textbf{Reproduction.} The source manifest binds the native producer, original information diagnostic and adapter. The frozen test plan, exact commands and native exit receipts accompany this report. Existing full 1,260-target evaluation requirements remain unchanged.
\end{document}''')
with (o/'render.stdout').open('w') as f:subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=o,stdout=f,stderr=subprocess.STDOUT,check=True)
(o/'result.json').write_text(json.dumps({'schema':'pencil.relation_adapter/v1','native_equation':'A*x+b*k=-a','diagnostic_equation':'A*x+b*k+a=0','native_release_tests_passed':3,'enumerated_single_row_systems':343,'assignments_per_system':49,'clippy_exit':0,'scientific_targets':0,'cost_ratio':None,'independent_review':'pending'},indent=2)+'\n')
print(hashlib.sha256((o/'report.pdf').read_bytes()).hexdigest())
