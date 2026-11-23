from pathlib import Path
p=Path(__file__).parent
text="""Coefficient array storage correspondence
Task T-1163. Producer proof packet; independent source-fidelity review pending.
The checked storage model uses bounds-checked Array.set. Its size is unchanged, the chosen slot receives the new word, and all other slots retain their original values. Canonical residues remain canonical when the replacement is canonical.
The composed update uses the generated inverse from the preceding packet. For prime p with 1<p<2^31, canonical input coefficients and parameters, nonzero scale, and 0<=j<=k<=5, each decoded output slot equals its prior value plus the binomial-transform term exactly when its index is j. All other decoded slots are unchanged. The source-loop index condition j<=k<length proves the update is in bounds.
This connects the checked arithmetic to one finite-array mutation. Full nested-loop accumulation, trimming and fixed-size divisor copying still require composition. Lean Array.set is a logical storage model, not a Rust compiler equivalence claim.
Source: projective_law_gate.rs transform; prior Loop.lean, WordCoefficient.lean and Inversion.lean. Standard Array.set semantics: https://lean-lang.org/doc/api/Init/Data/Array/Set.html. Existing polynomial and field lemmas are reused.
Seven main declarations and eight controls are checked, with separate axiom readback. Controls cover first/middle/last replacements, preserved neighbours, an empty array, excluded out-of-bounds index, canonical zeros, and a generated coefficient update. The first attempt failed because untouched-slot reads required the explicit Array.getElem_set rewrite; the corrected attempt passed. Native logs preserve warnings and failures.
The first controls attempt used an unavailable interval tactic; an explicit three-index split replaced it. The second controls attempt required changing the array size to its numeral and reverting proof-dependent hypotheses before decide. No scientific campaign or cost measurement. Existing compiled dependencies remain part of the trust boundary. The graph shows proof dependencies, not performance data.
"""
(p/'REPORT.md').write_text('# '+text)
(p/'report.tex').write_text(r"""\documentclass[10pt]{article}
\usepackage[margin=.8in]{geometry}\usepackage{amsmath,tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\begin{document}
\textbf{Coefficient array storage correspondence}\hfill2026-10-31\\T-1163. Producer proof; independent source-fidelity review pending.

Bounds-checked array replacement preserves length, writes the selected slot and leaves every other slot unchanged. Canonical residues remain canonical when the replacement is canonical.

For prime $p$ with $1<p<2^{31}$, canonical coefficients and parameters, nonzero scale $s$, and $0\le j\le k\le5$, the composed generated-inverse update gives
\[
 \operatorname{decode}(\mathrm{out}'_i)=\operatorname{decode}(\mathrm{out}_i)+
 \begin{cases}c\binom{k}{j}a^jb^{k-j}s^{-1}&i=j,\\0&i\ne j.\end{cases}
\]
The source index condition $j\le k<\mathrm{length}$ proves the array access is in bounds.
\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node(a)at(0,0){Generated inverse\\and word update};
\node(b)at(5,0){Bounds-checked Array.set\\length and other slots preserved};
\node(c)at(2.5,-1.3){Canonical output array\\per-index decoded formula};
\draw[->](a)--(c);\draw[->](b)--(c);
\end{tikzpicture}\end{center}
Seven main declarations and eight controls have separate named axiom readbacks. Controls cover first/middle/last replacements, preserved neighbours, empty-array size, an excluded out-of-bounds index, canonical zeros and an actual generated coefficient update. The first attempt needed explicit \texttt{Array.getElem\_set} rewriting for untouched slots; the corrected attempt passed. The first controls attempt needed an explicit three-index split in place of an unavailable tactic. A second controls attempt needed the literal size and proof-dependent hypotheses handled explicitly. All native receipts and warnings remain recorded.

This is one finite-array mutation composed with checked arithmetic. Full nested-loop accumulation, trailing-zero trimming and fixed-size divisor copying remain the next bindings. Lean array semantics are a source storage model, not compiled-Rust equivalence.

Source bindings: \texttt{projective\_law\_gate.rs} transform; preceding \texttt{Loop.lean}, \texttt{WordCoefficient.lean}, \texttt{Inversion.lean}. Primary reference: \href{https://lean-lang.org/doc/api/Init/Data/Array/Set.html}{Lean Array.set semantics}. No novelty claim for standard array or field identities.

Native exit codes, unchanged source/dependency bindings and axiom readback accompany this PDF, editable SVG and structured status. Existing compiled dependencies remain in the trust boundary. No scientific run, timing or cost ratio.
\end{document}
""")
(p/'flow.svg').write_text("""<svg xmlns="http://www.w3.org/2000/svg" width="960" height="260"><rect width="960" height="260" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#17364a"><text x="480" y="28" font-size="20">Coefficient array storage · T-1163</text><g stroke="#17364a" fill="#edf5f7"><rect x="30" y="60" width="380" height="65"/><rect x="550" y="60" width="380" height="65"/><rect x="220" y="160" width="520" height="55"/></g><text x="220" y="87">Generated inverse + coefficient arithmetic</text><text x="220" y="110">prime p; canonical inputs; p &lt; 2³¹</text><text x="740" y="87">Bounds-checked array update</text><text x="740" y="110">length and other slots preserved</text><path d="M220 125L360 160M740 125L600 160" stroke="#17364a"/><text x="480" y="184">Canonical output array + per-index decoded formula</text><text x="480" y="204">one source-model mutation</text><text x="480" y="246">Full nested-loop composition next · independent fidelity review pending</text></g></svg>""")
