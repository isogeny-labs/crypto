from pathlib import Path
p=Path(__file__).parent
(p/'REPORT.md').write_text('''# Word traversal equals affine polynomial composition

T-1171, 2026-10-31. Producer proof packet; independent source-fidelity review pending.

The decoded polynomial reconstructed from the complete word-array transform equals C(s^-1) times f composed with aX+b. The proof identifies the ordered list of binomial terms with the earlier symbolic outer-loop polynomial, identifies every stored output coefficient, and proves all higher coefficients vanish. Under the additional equality decode(s)=decode(a)^m, it equals the earlier normalization map T_m.

The generic list/binomial identity is proved for arbitrary coefficient count over a field. The machine-array theorem retains length<=6, prime1<p<2^31, canonical input coefficients and a,b,s, and nonzero scale. The supplied scale equality in the T_m corollary is explicit; computing the scale and subsequent trimming/copying remain separate source bindings.

Twelve main declarations and ten controls passed, with22 named standard-axiom readbacks. Controls include empty and constant polynomials, rational quadratic coefficients17/2,10,3 for f=1+2X+3X^2 under (a,b,s)=(2,3,4), negative-alpha linear coefficient-10, omitted-scale mismatch, quintic leading coefficient, high-coefficient zero, and identity transformation for arbitrary coefficient count. These are fixed formal controls, not cohort measurements.

The first identification attempt exposed broad list rewrites, function-composition simplification and coefficient simplification. Subsequent attempts corrected a closed-goal tactic, compact notation parsed as a principal-segment token, and equality orientation in a high-coefficient conditional. Attempt4 passed with deprecation warnings. The first reconstruction lacked the explicit prime-field instance required by sourcePoly; native exit1 rejected that build and its elaborator-generated sorry warnings. Reconstruct2 passed, and final named axiom readback contains no sorryAx. Controls and readback passed first attempt. All failures remain recorded.

Sources: projective_law_gate.rs transform; pinned Loop.lean, Schedule.lean, Traversal.lean, Storage.lean, WordCoefficient.lean and Inversion.lean. Primary reference: https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Coeff.html, checked through existing pinned modules. Standard binomial/coefficient identities are reused without novelty claims.

The remaining source bindings are trailing-zero trimming, fixed-size divisor copying, and the scale computation for the T_m corollary. This is a logical source model, not compiled-Rust equivalence or a geometric class transport proof. Existing compiled dependencies remain within the trust boundary. No scientific execution, timing or cost ratio; cohort and campaign limits unchanged.
''')
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=.7in]{geometry}\usepackage{amsmath,tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\begin{document}
\textbf{Word traversal equals affine polynomial composition}\hfill2026-10-31\\T-1171. Producer proof; independent source-fidelity review pending.

The complete decoded output polynomial now satisfies
\[
 \operatorname{decodedPoly}(\operatorname{transform}(c,a,b,s))
   = C(s^{-1})\,f(aX+b).
\]
The proof identifies the exact ordered list of binomial contributions with the earlier symbolic outer-loop polynomial, matches every stored coefficient, and proves higher coefficients vanish. If the decoded scale satisfies $s=a^m$, the result equals the existing normalization map $T_m$.
\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node(a)at(0,0){Exact scheduled sum\\symbolic outer-loop identity};
\node(b)at(5,0){Decoded stored coefficients\\higher coefficients zero};
\node(c)at(2.5,-1.2){Whole polynomial equality\\$s^{-1}f(aX+b)$};
\draw[->](a)--(c);\draw[->](b)--(c);
\end{tikzpicture}\end{center}
The generic list/binomial identity allows arbitrary coefficient count over a field. The word-array theorem retains length $\le6$, prime $1<p<2^{31}$, canonical coefficients and parameters, and nonzero scale. The scale equality in the $T_m$ corollary is an explicit hypothesis.

Twelve main declarations and ten controls passed, with22 named standard-axiom readbacks. Controls include empty/constant polynomials, identity at arbitrary length, a shifted/scaled quadratic, negative alpha, omitted scale, quintic leading coefficient and vanishing higher coefficients. For $f=1+2X+3X^2$ and $(a,b,s)=(2,3,4)$, the rational output coefficients are
\[
 [17/2,\;10,\;3].
\]
Changing $a$ to $-2$ changes the linear coefficient to $-10$; omitting the scale changes the constant coefficient. These are formal controls, not cohort measurements.

Identification attempt1 required explicit list/function/coefficient rewrites. Attempts2--3 corrected a closed-goal tactic, notation parsed as a principal-segment token, and a conditional equality orientation. Attempt4 passed. Reconstruction attempt1 lacked an explicit prime-field instance; native exit1 rejected it, including elaborator-generated sorry warnings. Attempt2 passed; final named axiom readback contains no \texttt{sorryAx}. Controls and readback passed first attempt. All failures and nonfatal warnings are retained.

Sources: \texttt{projective\_law\_gate.rs} transform and the preceding loop, schedule, traversal, storage and arithmetic proofs. Primary reference: \href{https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Coeff.html}{Mathlib polynomial coefficient identities}, through pinned imports.

Next source bindings: scale computation for the corollary, trailing-zero trimming and fixed-size divisor copying. This logical source model is not compiled-Rust equivalence or a geometric class transport proof. Existing compiled dependencies remain in the trust boundary. No scientific campaign, timing or cost ratio.
\end{document}
''')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="270"><rect width="1000" height="270" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#17364a"><text x="500" y="28" font-size="20">Whole polynomial identification · T-1171</text><g stroke="#17364a" fill="#edf5f7"><rect x="30" y="65" width="390" height="65"/><rect x="580" y="65" width="390" height="65"/><rect x="230" y="170" width="540" height="55"/></g><text x="225" y="92">Scheduled binomial sum</text><text x="225" y="115">equals symbolic outer-loop polynomial</text><text x="775" y="92">Decoded array coefficients match</text><text x="775" y="115">all higher coefficients vanish</text><path d="M225 130L360 170M775 130L640 170" stroke="#17364a"/><text x="500" y="194">Decoded output = s⁻¹ f(aX+b)</text><text x="500" y="215">equals T_m when decoded s = decoded a^m</text><text x="500" y="254">Producer proof · independent fidelity review pending · no performance measurement</text></g></svg>''')
for name in ['REPORT.md','report.tex']:
 f=p/name;s=f.read_text()
 for a,b in {'prime1':'prime 1','with22':'with 22','coefficients17':'coefficients 17','coefficient-10':'coefficient -10','exit1':'exit 1','Attempt4':'Attempt 4','attempt1':'attempt 1','attempt2':'attempt 2','Attempt2':'Attempt 2','Reconstruct2':'Reconstruct2'}.items():s=s.replace(a,b)
 f.write_text(s)
