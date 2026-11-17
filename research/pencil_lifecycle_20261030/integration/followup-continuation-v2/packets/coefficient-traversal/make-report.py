from pathlib import Path
p=Path(__file__).parent
(p/'REPORT.md').write_text('''# Cached-inverse coefficient traversal

T-1167, 2026-10-31. Producer source-model proof; independent source-fidelity review pending.

The complete coefficient traversal now composes the preceding single-slot word update. A size-indexed array retains its length throughout. A single inverse word is computed before traversal and passed unchanged through every step, matching the source placement of Field::inv.

For any valid event sequence, each intermediate array stays canonical and each decoded final slot is its original value plus the sum of contributions addressed to that slot. The transform theorem instantiates the generated inverse, initializes a zero array and executes the complete nested schedule. Its domain is 0<=n<=6, prime1<p<2^31, canonical coefficients and parameters, and nonzero scale.

A finite theorem for every length0 through6 proves that the schedule is exactly range(n).flatMap(k => range(k+1).map(j => (k,j))), in that order. Additional theorems give n(n+1)/2 updates and no repeated index pair. Thus the n=6 traversal has21 coefficient updates; this is a derived schedule count, not a timing measurement.

Seven main declarations and eleven controls passed, followed by18 named standard-axiom readbacks. All four native module checks passed on their first attempts. Controls cover empty, constant, quadratic, scaled quadratic, zero, identity and shifted quintic inputs. Modulo17 the coefficients[1,2,3,4,5,6] under x->x+1 become[4,2,3,16,1,6]. Omitting the highest input row changes this output. All controls are fixed formal examples, not a campaign cohort.

Source bindings: projective_law_gate.rs transform, prior Storage.lean, WordCoefficient.lean, Inversion.lean and Loop.lean. Primary documentation: https://lean-lang.org/doc/api/Init/Data/List/Control.html and https://lean-lang.org/doc/api/Init/Data/Array/Set.html. Standard traversal and field identities are reused without novelty claims.

The final expression is a sum over the exact nested schedule. Its identification with the previously proved polynomial composition, trailing-zero trimming and fixed-size divisor copying remain the next bindings. This source logical model is not a Rust compiler proof. Native receipts bind sources and existing compiled dependencies; no independent binary rechecker or independent semantic verdict is claimed.

Includes inspected PDF, editable SVG, structured result domains, custody and receipts. No scientific campaign, timing or cost ratio; full cohort and authorization limits unchanged.
''')
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=.75in]{geometry}\usepackage{amsmath,tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\begin{document}
\textbf{Cached-inverse coefficient traversal}\hfill2026-10-31\\T-1167. Producer proof; independent source-fidelity review pending.

The full coefficient traversal composes the preceding word update through a size-indexed array. The inverse is computed once before traversal and passed unchanged through every step.

For any valid event sequence $E$, the checked invariant is
\[
 \operatorname{decode}(\mathrm{out}'_i)=\operatorname{decode}(\mathrm{out}_i)
 +\sum_{(k,j)\in E}\mathbf{1}_{i=j}\,c_k\binom{k}{j}a^jb^{k-j}s^{-1}.
\]
Every intermediate array remains canonical. The transform theorem starts with a zero array and supplies the generated inverse. Assumptions: $0\le n\le6$, prime $1<p<2^{31}$, canonical coefficients and parameters, and nonzero scale.
\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node(a)at(0,0){Generate inverse once\\initialize zero array};
\node(b)at(5,0){Ordered nested schedule\\all $k<n$, then $0\le j\le k$};
\node(c)at(2.5,-1.4){Canonical state after every step\\decoded slot equals sum of contributions};
\draw[->](a)--(b);\draw[->](b)--(c);
\end{tikzpicture}\end{center}
For every supported length $0,\ldots,6$, a separate theorem identifies the schedule with the source's nested ranges in their exact order. It has no repeated index pairs and has $n(n+1)/2$ events. The six-coefficient case has21 updates (derived schedule count, not measured time).

Seven main declarations and eleven fixed controls passed; all18 named axiom readbacks contain only standard Lean axioms. The four native module checks passed on their first attempts. Controls include empty, constant, quadratic, scaled quadratic, zero, identity and shifted quintic inputs. Modulo17,
\[
 [1,2,3,4,5,6]\quad\xrightarrow{x\mapsto x+1}\quad[4,2,3,16,1,6].
\]
Omitting the highest input row changes that output. These are formal controls, not cohort measurements.

Source: \texttt{projective\_law\_gate.rs} transform and the preceding storage, arithmetic, inverse and symbolic-loop proofs. Primary references: \href{https://lean-lang.org/doc/api/Init/Data/List/Control.html}{Lean list traversal} and \href{https://lean-lang.org/doc/api/Init/Data/Array/Set.html}{Array.set semantics}.

The expression above sums over the exact nested schedule. Identifying it with the existing polynomial-composition theorem, then binding trailing-zero trimming and fixed-size divisor copying, are the next steps. This is a logical source model, not compiled-Rust equivalence. Existing compiled dependencies remain in the trust boundary.

Native exits and unchanged source/dependency hashes accompany the PDF, editable diagram, status JSON and custody. No scientific campaign, timing or cost ratio; cohort and authorization limits unchanged.
\end{document}
''')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="270"><rect width="1000" height="270" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#17364a"><text x="500" y="28" font-size="20">Cached-inverse coefficient traversal · T-1167</text><g stroke="#17364a" fill="#edf5f7"><rect x="30" y="65" width="360" height="65"/><rect x="610" y="65" width="360" height="65"/><rect x="230" y="170" width="540" height="55"/></g><text x="210" y="92">Generate one inverse; initialize zeros</text><text x="210" y="115">same inverse throughout traversal</text><text x="790" y="92">Exact ordered nested schedule</text><text x="790" y="115">n ≤ 6; n(n+1)/2 unique pairs</text><path d="M390 97H610M790 130L650 170" stroke="#17364a"/><text x="500" y="194">Canonical state + decoded sum of contributions</text><text x="500" y="215">every valid prefix; full transform from zero</text><text x="500" y="254">Independent fidelity review pending · no performance measurements</text></g></svg>''')
for name in ['REPORT.md','report.tex']:
 f=p/name;s=f.read_text()
 for a,b in {'prime1':'prime 1','length0':'length 0','through6':'through 6','has21':'has 21','by18':'by 18','all18':'all 18','Modulo17':'Modulo 17','coefficients[':'coefficients [','become[':'become ['}.items():s=s.replace(a,b)
 f.write_text(s)
