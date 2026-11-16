from pathlib import Path
p=Path(__file__).parent
(p/'REPORT.md').write_text('''# Normalization source attribution correction

T-1177, 2026-10-31. Producer provenance correction and fresh proof replay. Independent source-fidelity review remains pending.

Four packets copied the first61 lines of projective_law_gate.rs into source-transform.rs.txt. Those excerpts contain none of choose, transform or map_divisor. The original affine-loop packet actually bound research/five_leads_e2e_20261019/native/genus2_model_gate.rs. Its current full-file hash still equals the original recorded hash. The defect is incorrect source attribution and excerpt selection, not evidence that the canonical source changed. Earlier commentary suggesting a moved canonical function is superseded by this finding.

Affected packets, preserved unchanged: coefficient-array-storage, coefficient-traversal, traversal-polynomial-composition, scale-trim-storage. Their mathematical checks remain recorded as producer proof checks; the excerpts do not supply the advertised source review material. Use this correction's named canonical function bodies and source map for that purpose.

The extractor requires one matching function header, balances braces, and asserts no quoted strings or comments occur in the selected function bodies. It records full-file/function hashes and exact line ranges. Canonical choose, transform and map_divisor are included. In the related gate, choose and transform exist later in the file; map_divisor is absent. In projective_state_collector.rs all three functions exist. Collector choose/transform are byte-identical to canonical. The other present bodies match after the explicitly recorded local-variable identifier replacements: gate out->x in choose, gate scale_inv->inv in transform, collector out->result in map_divisor. These are text comparisons, not a compiler equivalence or independent semantic verdict.

The unchanged Scale, Trim, Bridge, Controls and Readback modules are recompiled in a fresh private directory. The verifier additionally binds canonical genus2_model_gate.rs, projective_state_collector.rs, extraction code and source-map.json, alongside the earlier source/dependency bindings. All five checks passed with unchanged inputs; the32 named readbacks contain only standard Lean axioms. No proof-source edits were needed. Existing compiled upstream dependencies remain in the trust boundary; this is a producer replay, not independent review.

The report, editable diagram, structured status, corrected function excerpts, full source snapshots, extraction script and receipts are additive. Old evidence is not overwritten. Divisor-validity composition remains the next mathematical step. No scientific campaign, timing or cost ratio.
''')
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=.7in]{geometry}\usepackage{tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\begin{document}
\textbf{Normalization source attribution correction}\hfill2026-10-31\\T-1177. Producer correction and replay; independent fidelity review pending.

Four recent packets copied the first61 lines of \texttt{projective\_law\_gate.rs}. Those excerpts contain none of \texttt{choose}, \texttt{transform} or \texttt{map\_divisor}. The original loop packet bound \texttt{genus2\_model\_gate.rs}; its current full-file hash still matches the original record. The defect is source attribution and excerpt selection, not a demonstrated canonical-source change.

Affected packets, preserved unchanged:\\
\texttt{coefficient-array-storage}; \texttt{coefficient-traversal}\\
\texttt{traversal-polynomial-composition}; \texttt{scale-trim-storage}.\\
Use this correction's named canonical excerpts and source map for source review.

\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node(a)at(0,0){Four old fixed excerpts\\relevant functions absent};
\node(b)at(5,0){Canonical named functions\\original full hash unchanged};
\node(c)at(2.5,-1.3){Corrected source map + fresh proof replay\\expanded source bindings; old evidence retained};
\draw[->](a)--(c);\draw[->](b)--(c);
\end{tikzpicture}\end{center}
The extractor records exact line ranges and full-file/function hashes. It requires one function-header match and balanced braces, and checks that the selected bodies contain no strings or comments that could confuse brace counting.

\begin{tabular}{lll}
Source & Function & Observed text correspondence\\\hline
Related gate & choose & matches after out $\to$ x\\
Related gate & transform & matches after scale\_inv $\to$ inv\\
Related gate & map\_divisor & absent\\
State collector & choose, transform & byte-identical\\
State collector & map\_divisor & matches after out $\to$ result
\end{tabular}

These are text comparisons, not compiler equivalence or independent semantic review. The mathematical statements and prior checks remain recorded, but the old excerpts do not provide the advertised source review material.

The unchanged Scale, Trim, Bridge, Controls and Readback modules were rebuilt in a fresh directory. The helper now also binds the intended canonical source, state collector, extraction script and source map. All five checks passed with unchanged input hashes. All32 named axiom readbacks contain only standard Lean axioms. Existing compiled upstream dependencies remain within the trust boundary; this is a producer replay.

The additive packet includes this inspected PDF, editable SVG, structured status, full source snapshots, named excerpts and receipts. Earlier commentary suggesting canonical function movement is superseded by the source-attribution finding. Divisor-validity composition remains the next mathematical step. No scientific campaign, timing or cost ratio.
\end{document}
''')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="270"><rect width="1000" height="270" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#17364a"><text x="500" y="28" font-size="20">Normalization source attribution correction · T-1177</text><g stroke="#17364a" fill="#edf5f7"><rect x="30" y="65" width="390" height="65"/><rect x="580" y="65" width="390" height="65"/><rect x="220" y="170" width="560" height="55"/></g><text x="225" y="92">Four fixed-line excerpts</text><text x="225" y="115">choose / transform / map_divisor absent</text><text x="775" y="92">Canonical source and named functions</text><text x="775" y="115">full hash matches original loop packet</text><path d="M225 130L360 170M775 130L640 170" stroke="#17364a"/><text x="500" y="194">Corrected bindings + five-module fresh replay</text><text x="500" y="215">unchanged proof sources; old packets retained</text><text x="500" y="254">Text correspondence only · independent semantic review pending</text></g></svg>''')
for name in ['REPORT.md','report.tex']:
 f=p/name;s=f.read_text().replace('first61','first 61').replace('the32','the 32').replace('All32','All 32');f.write_text(s)
