from pathlib import Path
p=Path(__file__).parent
(p/'REPORT.md').write_text('''# Computed scale, trimming and divisor storage

T-1174, 2026-10-31. Producer proof packet; independent source-fidelity review pending.

The source-model small power now supplies the nonzero canonical scale and the equality required by the T_m normalization theorem. The complete word transform followed by terminal-zero trimming and zero padding represents T_m(f), without a separately supplied scale-equality premise.

The scale theorem retains prime1<p<2^31, canonical nonzero alpha, canonical beta and coefficients, length<=6 and exponent m in0..5. The list polynomial laws need only a commutative ring. The trim model tests getLast?=some0 and removes dropLast; it preserves the represented polynomial for any fuel and finishes with no terminal zero whenever fuel is at least the original length. Padding appends zeros, preserves the polynomial, and reaches the requested capacity when the original length fits. Over-capacity inputs are not truncated and do not satisfy that capacity theorem.

A coefficient theorem connects the list polynomial to the preceding array-decoding representation. The composed theorem then identifies the trimmed/padded output with T_m(f). For source divisor storage, the intended capacities are3 for u and2 for v; source degree/domain validity remains to be composed with the earlier Mumford validity theorem. This is a logical model of scale, last/pop and zero-initialized prefix copying, not compiled-Rust equivalence or a geometric class transport proof.

Seventeen main declarations and fifteen controls passed, with32 named standard-axiom readbacks. Controls include empty/all-zero lists, internal zeros, multiple trailing zeros, insufficient fuel, capacities2/3, forbidden over-capacity truncation, and power exponents0/2/3. Modulo17, the transformed u coefficient example[1,2,1] at alpha2,beta3,scalealpha^2 yields[4,4,1]; the separate v example[5,0] at scalealpha^3 trims and pads to[7,0]. These examples test representations, not validity of a paired divisor on a named curve.

Scale, controls and readback passed first attempt. Trim attempt1 required an explicit dropLast-length rewrite and the available empty-list lemma; trim2 passed. Bridge attempt1 contained compact <=i notation parsed as an initial-segment token; bridge2 passed after spacing correction. All failures and nonfatal unused-simp warnings remain recorded; final named readback contains only standard axioms.

Sources: projective_law_gate.rs transform/map_divisor, prior smallPow and polynomial-transform proofs, pinned Mathlib Data/List/Basic.lean dropLast_append_getLast?, and official Lean take/drop documentation https://lean-lang.org/doc/api/Init/Data/List/TakeDrop.html. Standard polynomial/list identities are reused without novelty claims. Existing compiled dependencies remain in the trust boundary.

Inspected PDF, editable SVG, status JSON, native receipts and custody accompany this packet. No scientific campaign, timing or cost ratio; cohort and authorization limits unchanged.
''')
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=.65in]{geometry}\usepackage{amsmath,tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{6pt}
\begin{document}
\textbf{Computed scale, trimming and divisor storage}\hfill2026-10-31\\T-1174. Producer proof; independent source-fidelity review pending.

The checked small-power computation supplies a nonzero canonical scale and its decoded equality $s=a^m$. The complete word transform followed by terminal-zero trimming and zero padding represents
\[
 T_m(f)=a^{-m}f(aX+b).
\]
The source-model theorem no longer requires a separately supplied scale-equality premise.
\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node(a)at(0,0){Compute $a^m$\\canonical and nonzero};
\node(b)at(4,0){Complete transform\\represents $T_m(f)$};
\node(c)at(8,0){Trim last zeros\\pad to capacity};
\node(d)at(4,-1.1){Same polynomial; exact capacity when input fits};
\draw[->](a)--(b);\draw[->](b)--(c);\draw[->](c)--(d);
\end{tikzpicture}\end{center}
The word theorem assumes prime $1<p<2^{31}$, canonical coefficients and parameters, nonzero $a$, length $\le6$ and $m\in\{0,\ldots,5\}$. The list polynomial laws require only a commutative ring.

The trim model tests whether the last word is zero, then removes it. Polynomial preservation holds for any fuel. Fuel at least the original length suffices to finish with no terminal zero. Padding appends zeros and preserves the polynomial. Exact capacity requires the original length to fit; oversized inputs are not silently truncated.

The array/list coefficient bridge connects these storage laws to the previously decoded transform. Source divisor capacities are3 for $u$ and2 for $v$. Composition with the divisor degree/domain and Mumford-validity theorem remains next. This is a logical model of scale, last/pop and zero-initialized prefix copying, not compiled-Rust equivalence or geometric class transport.

Seventeen main declarations and fifteen controls passed, with32 named standard-axiom readbacks. Controls include empty/all-zero lists, internal/trailing zeros, insufficient fuel, capacities2/3, over-capacity exclusion, and powers0/2/3. Modulo17, separate representation examples give
\[
 [1,2,1]\xrightarrow{a=2,b=3,m=2}[4,4,1],\qquad
 [5,0]\xrightarrow{a=2,b=3,m=3}[7,0]
\]
after trimming and padding. These are not a claim that the two examples form a valid divisor on a named curve.

Scale, controls and readback passed first attempt. Trim attempt1 needed the dropLast-length rewrite and an available empty-list lemma; attempt2 passed. Bridge attempt1 contained notation parsed as an initial-segment token; attempt2 passed after spacing correction. All failures and nonfatal warnings remain recorded. Final readback contains only standard axioms.

Sources: \texttt{projective\_law\_gate.rs} transform/map\_divisor; prior small-power and polynomial proofs; pinned Mathlib last-element identity; \href{https://lean-lang.org/doc/api/Init/Data/List/TakeDrop.html}{Lean take/drop semantics}. Existing compiled dependencies remain in the trust boundary. No scientific campaign, timing or cost ratio.
\end{document}
''')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="270"><rect width="1040" height="270" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#17364a"><text x="520" y="28" font-size="20">Computed scale → transform → trim and pad · T-1174</text><g stroke="#17364a" fill="#edf5f7"><rect x="20" y="65" width="280" height="65"/><rect x="380" y="65" width="280" height="65"/><rect x="740" y="65" width="280" height="65"/><rect x="220" y="170" width="600" height="55"/></g><text x="160" y="92">Compute canonical a^m</text><text x="160" y="115">nonzero scale proved</text><text x="520" y="92">Complete word transform</text><text x="520" y="115">polynomial T_m(f)</text><text x="880" y="92">Remove terminal zeros; pad</text><text x="880" y="115">same represented polynomial</text><path d="M300 97H380M660 97H740M880 130L720 170" stroke="#17364a"/><text x="520" y="194">Exact output capacity when original input fits</text><text x="520" y="215">No silent truncation; separate divisor-validity composition next</text><text x="520" y="254">Independent fidelity review pending · no performance measurements</text></g></svg>''')
for name in ['REPORT.md','report.tex']:
 f=p/name;s=f.read_text()
 for a,b in {'prime1':'prime 1','in0..5':'in 0..5','some0':'some 0','are3':'are 3','and2':'and 2','with32':'with 32','capacities2':'capacities 2','exponents0':'exponents 0','powers0':'powers 0','Modulo17':'Modulo 17','example[':'example [','yields[':'yields [','to[':'to [','alpha2':'alpha 2','beta3':'beta 3','scalealpha':'scale alpha','attempt1':'attempt 1','attempt2':'attempt 2'}.items():s=s.replace(a,b)
 f.write_text(s)
