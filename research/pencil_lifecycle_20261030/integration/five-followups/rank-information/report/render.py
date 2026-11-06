from pathlib import Path
import json,hashlib,subprocess
base=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/five-followups/rank-information')
out=base/'report';out.mkdir(exist_ok=True)
rows=[json.loads(x) for x in (base/'validation/fixtures.stdout').read_text().splitlines()]
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="300" viewBox="0 0 1000 300"><rect width="1000" height="300" fill="white"/><g font-family="sans-serif" fill="#152b3c"><text x="25" y="35" font-size="24">Equal total rank does not imply equal target information</text><text x="25" y="65" font-size="17">Derived linear systems over F7; synthetic correctness fixtures, not cohort measurements</text><rect x="25" y="95" width="450" height="140" rx="8" fill="#eef2f5"/><rect x="515" y="95" width="450" height="140" rx="8" fill="#e7f3ee"/><text x="45" y="130" font-size="22">x = 0</text><text x="45" y="165" font-size="19">rank(A) = rank([A b]) = 1</text><text x="45" y="205" font-size="19">k remains arbitrary</text><text x="535" y="130" font-size="22">k + 3 = 0</text><text x="535" y="165" font-size="19">rank(A) = 0; rank([A b]) = 1</text><text x="535" y="205" font-size="19">k = 4 is determined</text><text x="25" y="275" font-size="17">Both augmented row spaces have rank 1. The existing collector already checks a target pivot.</text></g></svg>'''
(out/'information.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(out/'information.pdf'),str(out/'information.svg')],check=True)
status={'schema':'pencil.target_information_diagnostic/v1','claim_status':'derived linear algebra with native synthetic correctness checks; independent source review pending','equation':'A*x + b*k + c = 0 over prime scalar field','consistent_iff':'rank([A b c]) = rank([A b])','target_unique_iff_consistent':'rank([A b]) = rank(A)+1','existing_source_observation':'GraphSolver::relation already checks the target pivot after Independent admission; total rank is not its stopping criterion','implementation':'offline diagnostic reuses residual::Basis','counter_caveat':'Basis calls change thread-local counters; diagnostic must stay outside measured producer','fixtures':rows,'new_scientific_targets':0,'cost_ratios':None,'historical_per_row_target_information':'not established by this packet','next_step':'independent review; identify retained full relation stream or preregister instrumentation before policy experiment'}
(out/'result.json').write_text(json.dumps(status,indent=2)+'\n')
tex=r'''\documentclass[11pt]{article}
\usepackage[margin=0.8in]{geometry}\usepackage{amsmath,amssymb,graphicx,booktabs,tabularx,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\begin{document}
{\LARGE Target information in retained relations}\par
2026-10-30. Derived diagnostic and synthetic correctness controls.

For a consistent system $Ax+bk+c=0$ over a prime field, the target $k$ is uniquely determined exactly when $b$ is outside the column space of $A$. Equivalently,
\[\operatorname{rank}[A\ b]-\operatorname{rank}A=1.\]
Consistency is tested separately by $\operatorname{rank}[A\ b\ c]=\operatorname{rank}[A\ b]$. An inconsistent system is never reported as determining a valid target.

\textbf{Derivation.} Subtract any two solutions. If $b\notin\operatorname{col}A$, a nonzero difference in $k$ would express $b$ as a linear combination of columns of $A$, a contradiction. Conversely, if $b=Av$, every solution extends to $(x-tv,k+t)$ for every field element $t$, so $k$ is not unique. Existence is why consistency is checked first.

\includegraphics[width=\textwidth]{information.pdf}

\textbf{Source observation.} The existing GraphSolver in \texttt{native\_solver.rs} already checks a target-column pivot after an independent admission (lines 500--516 at the recorded source). This packet does not introduce a new stopping rule. It adds an offline diagnostic to separate nuisance rank from target information.

\newpage
{\Large Implementation and review boundary}\par
The diagnostic reuses the existing prime-field residual basis three times, projecting to nuisance columns, nuisance plus target, and all columns. Rows have a fixed width and composite moduli are rejected by the existing basis constructor. It does not alter collection, elimination order, enrollment, or original results.

\textbf{Controls.} Six native unit tests passed: equal total rank with different information; nuisance-rank growth; inconsistency; row reordering/scaling and empty systems; wrong-domain/shape rejection; and direct enumeration of solution sets for fixed small algebraic fixtures. These are software correctness checks, not scientific samples or a cohort experiment. Native fixture output retains all five example matrices.

\textbf{Cost boundary.} Three basis computations incur additional work and mutate the kernel's thread-local operation counters. The diagnostic must run outside a measured producer. No time, memory advantage or end-to-end ratio is claimed. Replaying a fixed relation stream cannot establish a changed collector policy's future collection cost.

\textbf{Remaining evidence.} Independent source review is pending. This packet does not reconstruct per-relation information histories from aggregate historical rank counters. A subsequent policy comparison needs an actual retained relation stream, or prospectively reviewed instrumentation and a separate frozen evaluation. Existing 1,260-target obligations remain unchanged.

\textbf{Artifacts.} \texttt{result.json} contains formulas, scope and exact fixture results; \texttt{information.svg} is editable; source, validation logs and the source manifest accompany the report. No scalar search or new curve experiment was performed.
\end{document}'''
(out/'report.tex').write_text(tex)
with (out/'render.log').open('w') as f: subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=out,stdout=f,stderr=subprocess.STDOUT,check=True)
print(out)
