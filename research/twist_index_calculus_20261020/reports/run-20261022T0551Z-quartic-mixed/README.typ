#set page(paper: "a4", margin: 19mm)
#set text(font: "Liberation Sans", size: 9pt)
= Quartic twist two-line relation yield
UTC start `2026-10-22 05:51:35.462`; native Rust producer; Intel Xeon 6975P-C, 32 logical CPUs; 16 Rayon threads; L0 shared host. Seven curves × three seeds × two bases, each with one public target $[271]G$.

The base $L_0$ uses abscissae in $F_p$; $L_0 union L_2$ adds abscissae in $t^2F_p$. Both are invariant under the same order-24 action. A dedicated RNG makes the residual stream identical across each pair. The run completed all 42 records and 126 IC solves. Native independent replay checked 69,045 attempted residuals, 7,275 decompositions, 5,298 selected relations, all matrix rows and all answers. Zero failures. Raw SHA-256: `d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c`.

#image("quartic_mixed_lines_v1.png", width: 100%)

Median same-target hit yield rises 2.04–17.55× across curves. The union also raises matrix columns and modular row operations. Its counted field-multiplication path improves at $p=229,241$, worsens at $p=37,73,157$, and is mixed at $p=97,337$. The separate units and three-seed ranges are shown in the figure. Hash/table costs and IC-stream group additions remain unpriced; no whole-method speedup is claimed. Exact command and links are in `README.md`, with interpretation in `QUARTIC_TWIST24_REPORT.md`.
