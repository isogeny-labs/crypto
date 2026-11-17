from pathlib import Path
import json,re,hashlib
p=Path(__file__).parent
(p/'REPORT.md').write_text('''# Reduced polynomial validity under affine normalization

2026-10-31, T-1140. Producer formal proof packet; independent source-fidelity review pending.

For a field K and a!=0, T_n(p)=a^(-n)p(aX+b) preserves polynomial degree, including the zero polynomial, and natural degree. If u is monic, T_deg(u)(u) is monic. Combining these facts with the preceding divisibility theorem preserves the explicit predicate: monic u, natDegree u<=2, degree v<degree u, and u|(f-v²).

The strict ordinate bound uses degree in WithBot(N), with degree(0)=bottom. This includes the identity (1,0), without a false natDegree(0)<0 requirement. The proof applies for every field with nonzero scale; no characteristic assumption is added here.

Seven main declarations and eleven fixed controls are checked, followed by separate named axiom readback. Controls include the exact rational quadratic substitution T_2((X+1)^2)=(X+2)^2 for a=2,b=3; the unscaled polynomial has leading coefficient4 and is rejected as monic. The identity and (X,1) on f=X^5+1 are valid and transport correctly. Equal support/ordinate degrees and nonmonic2X are rejected.

Source binding: three_leads.rs:945-980 slices up through d and vp strictly below d; valid checks d<=2, the leading slot1, and empty remainder of f-v² modulo u. The historical genus2_model_gate.rs:14-56 scales the support by a^d and ordinate by a^3 and retains stored d. The algebraic predicate matches these intended polynomial conditions. This packet does not prove the literal vector loop, trim/remainder implementation, machine-field arithmetic, global projective extension or divisor-class equality. Those remain separate bindings.

Attempt1 found unavailable imports/lemma names. Attempt2 exposed Lean module-root lookup after adding only the missing compiled modules. Attempt3 passed with a private union containing the three pinned cached modules and inherited symlinks. Three source modules and their copied cache files have custody records; the final launcher hashes the full dependency union before and after each check. No cache build or download was performed. The first two control attempts failed on mapped rational constants and a lemma name; controls3 passed. All failed receipts are retained. Existing compiled dependencies remain in the trust boundary; no independent binary rechecker or independent review is claimed.

Primary sources: Mathlib Polynomial Degree/Lemmas and Degree/Operations official documentation, checked against pinned source. https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Degree/Lemmas.html . No novelty claim. No scientific execution, timing, enumeration or cost ratio. Campaign limits remain unchanged. The diagram represents logical dependencies, not measured performance.
''')
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=.75in]{geometry}\usepackage{amsmath,amssymb,tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
\textbf{Reduced polynomial validity under normalization}\hfill2026-10-31\\T-1140. Producer proof packet; independent source-fidelity review pending.
\section*{Checked construction}
For a field $K$ and $a\ne0$, set $T_n(p)=a^{-n}p(aX+b)$. Degree and natural degree are preserved. For monic $u$, $T_{\deg u}(u)$ is monic. Together with the preceding congruence result, this proves preservation of
\[u\text{ monic},\quad\operatorname{natDegree}(u)\le2,\quad\deg v<\deg u,\quad u\mid f-v^2.\]
The ordinate bound uses $\deg0=\bot$, so the identity $(1,0)$ is included. No characteristic restriction is added beyond the field and nonzero-scale hypotheses.
\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node (a) at (0,0) {Degree and monicity\\this packet};
\node (b) at (7,0) {Congruence $u\mid f-v^2$\\preceding packet};
\node (c) at (3.5,-1.3) {Normalized reduced polynomial validity};
\draw[->](a)--(c);\draw[->](b)--(c);
\end{tikzpicture}\end{center}
\section*{Controls and readback}
Seven main declarations and eleven controls receive separate named axiom readback. With $a=2,b=3$, the exact quadratic control is $T_2((X+1)^2)=(X+2)^2$. Omitting support scaling gives leading coefficient4 and fails monicity. The identity and $(X,1)$ on $f=X^5+1$ satisfy validity and transport correctly. Equal support/ordinate degrees and nonmonic $2X$ are rejected.
\section*{Source correspondence}
\texttt{three\_leads.rs}:945--980 slices support through stored degree$d$ and ordinate strictly below$d$; its validator checks $d\le2$, leading slot1, and empty remainder. \texttt{genus2\_model\_gate.rs}:14--56 retains$d$ and uses scales$a^d,a^3$. This theorem preserves the intended polynomial conditions. Literal vector loops, trim/remainder implementation, machine arithmetic and geometric divisor-class transport remain separate bindings.
\section*{Receipts and scope}
The initial compile failed on missing imports/lemma names. A partial cache overlay then failed module lookup. The third main attempt passed after building a private union from existing cached files. Three added modules and inherited dependencies are hashed; no cache build or download. Two control attempts failed on mapped rational constants/lemma naming; the third passed. Raw failures remain available.

Standardized status, proof sources, full dependency bindings and custody records accompany the packet. Existing compiled Lean dependencies remain in the trust boundary. No independent binary rechecker or semantic review is claimed.

Primary reference: \href{https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Degree/Lemmas.html}{Mathlib polynomial degree lemmas}, compared with pinned sources. No scientific campaign, enumeration, timing or cost ratio; expired limits unchanged. Diagram shows proof dependencies, not performance.
\end{document}
''')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="260"><rect width="900" height="260" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#19384b"><text x="450" y="30" font-size="21">T-1140 · Reduced polynomial validity</text><g stroke="#19384b" fill="#edf6f0"><rect x="40" y="60" width="320" height="65"/><rect x="540" y="60" width="320" height="65"/><rect x="220" y="175" width="460" height="45"/></g><text x="200" y="88">Degree + monicity preserved</text><text x="200" y="110">this packet · a ≠ 0</text><text x="700" y="88">u divides f − v² preserved</text><text x="700" y="110">preceding packet</text><path d="M200 125L340 175M700 125L560 175" stroke="#19384b"/><text x="450" y="203">Valid reduced polynomial pair transports</text><text x="450" y="248">Identity included · independent fidelity review pending · no performance data</text></g></svg>''')
