from pathlib import Path
p=Path(__file__).parent
(p/'REPORT.md').write_text('''# Affine coefficient accumulation and bounded word arithmetic

T-1142, 2026-10-31. Producer formal/source-binding packet; independent source-fidelity review pending.

The symbolic nested loop computes C(scale^-1)*P(aX+b), where P is the polynomial represented by the first n input coefficients. This is proved for arbitrary n over a field. A row contributes C(c*scale^-1)*(aX+b)^k. Inner and outer prefix invariants establish accumulation order, slot updates change only the selected coefficient, and dropping a last zero coefficient preserves the represented polynomial. The final equation is connected directly to the preceding T_m normalization by taking scale=a^m.

The helper's binomial recurrence is separately modeled with natural numbers and UInt64 multiplication/division. For every0<=j<=k<=5 the word result equals Nat.choose k j. Every multiplication intermediate before division is at most 30; denominators are positive, and30<2^64. These are finite kernel-checked statements, not a generalized no-overflow claim. Degree 6 produces an intermediate60 and is explicitly outside that30 bound. Loop index inequalities establish j<n and j<=k, including nonnegative subtraction, under k<n and j<k+1.

Fixed controls cover constant/linear/quadratic rows with a2,b3,c4,scale2:2;6+4X;18+24X+8X². The middle coefficient 24 rejects 12 from omission of the binomial factor. Zero coefficients, empty input, slot isolation, final index, choose(5,2)=10 and the attained 30 product are included. Thirteen main declarations and fourteen controls receive separate named axiom readback. Source/dependency hashes and native exits are checked. Main attempts passed with one unused-simp warning; the first control attempt failed on mapped rational constants, the correction passed, and controls were rebuilt after the final UInt64 theorem was added.

Source excerpt and full-file hashes bind genus2_model_gate.rs:1-56. Its inverse call requires a nonzero scale; the algebraic identity remains meaningful at zero using the field's totalized inverse but is not an execution claim for that branch. The source uses choose%p; connecting encoded prime-field values, field-operation results and that reduction to abstract field coefficients remains a separate correspondence obligation. The UInt64 theorem models the bounded choose recurrence, not the compiled Rust binary or all field operations. Literal array/trim/compiler behavior and geometric divisor-class transport remain separate bindings.

Relevant primary sources reviewed: Mathlib Taylor and Eval/Coeff documentation, plus the pinned Data/Nat/Choose/Sum.lean add_pow theorem. https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Taylor.html ; https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Eval/Coeff.html . Established binomial composition is reused; no novelty claim.

No scientific campaign, curve enumeration, timing or cost ratio. Frozen cohort and campaign limits unchanged. Diagram describes proof dependencies; no performance data were generated. Independent semantic review and binary rechecking are not claimed.
''')
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=.7in]{geometry}\usepackage{amsmath,amssymb,tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
\textbf{Affine coefficient loop and bounded word arithmetic}\hfill2026-10-31\\T-1142. Producer proof packet; independent source-fidelity review pending.
\section*{Checked loop model}
For a field, input coefficients $c_k$ and scale $s$, define
\[R_k=\sum_{j=0}^{k}C\!\left(c_k\binom{k}{j}a^jb^{k-j}s^{-1}\right)X^j.\]
The row theorem proves $R_k=C(c_ks^{-1})(aX+b)^k$. Inner and outer prefix invariants then give, for arbitrary $n$,
\[\operatorname{outer}(n)=C(s^{-1})\left(\sum_{k<n}C(c_k)X^k\right)\!\circ(aX+b).\]
Slot updates affect only the selected coefficient. Removing a trailing zero preserves the represented polynomial. Taking $s=a^m$ connects this loop model to the preceding $T_m$ normalization theorem.
\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node (a) at (0,0) {Slot update};\node (b) at (3.5,0) {Binomial row};\node (c) at (7.5,0) {Prefix composition};
\draw[->](a)--(b);\draw[->](b)--(c);
\node (d) at (3.5,-1) {Bounded choose recurrence\\natural numbers and UInt64};\draw[->](d)--(b);
\end{tikzpicture}\end{center}
\section*{Word bound and controls}
For every $0\le j\le k\le5$, the UInt64 recurrence equals$\binom{k}{j}$. Its pre-division products are at most 30, divisors are positive, and $30<2^{64}$. Index bounds give$j<n$, $j\le k$ and$k-j+j=k$ under the loop conditions. Degree 6 attains a product 60, so the 30 bound is deliberately limited to degree 5.

With $a=2,b=3,c=4,s=2$, rows 0/1/2 are $2$, $6+4X$, and$18+24X+8X^2$. The middle coefficient 24 rejects 12 from a missing binomial factor. Controls also cover zero/empty input, slot isolation, the final index, $\binom52=10$ and the attained 30 bound.
\section*{Evidence and source boundary}
Thirteen main declarations and fourteen controls have separate named axiom readback. Native exits and input hashes are checked. The main attempts passed with an unused-simp warning; the first control attempt failed on mapped rational constants, then passed after correction. Controls were rebuilt against the final word theorem.

The retained source excerpt and hashes bind \texttt{genus2\_model\_gate.rs}:1--56. Its inverse call requires $s\ne0$. Abstract totalized inverse at zero is not a source-execution claim. Prime-field encoding, \texttt{choose\%p}, field-operation correspondence, literal array/trim/compiler behavior and geometric class transport remain separate bindings. The word theorem concerns the bounded choose recurrence.

Sources: \href{https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Taylor.html}{Mathlib Taylor}, \href{https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Eval/Coeff.html}{evaluation/coefficient documentation}, and pinned \texttt{Data/Nat/Choose/Sum.lean}. No novelty claim. No scientific campaign, curve enumeration, timing or cost ratio. Diagram shows proof dependencies; cohort and campaign limits unchanged.
\end{document}
''')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="950" height="250"><rect width="950" height="250" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#18374a"><text x="475" y="28" font-size="20">T-1142 · Affine coefficient accumulation</text><g fill="#edf5f8" stroke="#18374a"><rect x="20" y="65" width="240" height="55"/><rect x="355" y="65" width="240" height="55"/><rect x="690" y="65" width="240" height="55"/><rect x="300" y="165" width="350" height="45"/></g><text x="140" y="99">Slot-update identity</text><text x="475" y="99">Binomial row</text><text x="810" y="99">Prefix composition = T</text><path d="M260 92H350m245 0h90M475 165V125" stroke="#18374a"/><text x="475" y="194">UInt64 choose · k ≤ 5 · products ≤ 30</text><text x="475" y="238">Producer proofs · source-fidelity review pending · no performance measurements</text></g></svg>''')
