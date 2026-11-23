---
title: "Exact Edwards and original Joux–Vitse system checks"
fontsize: 10pt
geometry: margin=17mm
mainfont: DejaVu Serif
---

The frozen cohort completes all 18 calls: two native calibrations and eight
source/control systems, each twice. All 16 msolve outputs match independent
native Buchberger, S-pair and input-remainder checks. Every one of eight
GB repeats is byte-identical. Canonical export/full file checks pass;
flow18planned/attempted/completed, zero incomplete/interrupted/resumed/
excluded. All scientific clocks are invalid. Unsigned same-host evidence.
[pcc:unverified]

| Fixed fixture | GB degree | F4 pair degree | Finite length | Rational zeros |
|:--|--:|--:|--:|--:|
| x²,y² control | 2 | no round | 4 | 1 |
| Edwards affine | 2 | 2 | 2 | 1 |
| Edwards highest parts | 2 | 2 | infinite | 9 |
| Edwards homogenized | 2 | 2 | infinite | 13 |
| JV empty affine | 0 | 3 | 0 | 0 |
| JV empty homogenized | 4 | 5 | 8 | 1 |
| JV split affine | 1 | 3 | 1 | 1 |
| JV split homogenized | 3 | 4 | infinite | 101 |

These are the first-pass values; repeats give identical GB outputs.
Homogeneous rational zeros count affine cone points including the origin,
not projective points. “Infinite” refers to the coordinate algebra's
positive dimension over an algebraic closure. The empty affine unit ideal
has length zero. [pcc:unverified]

The JV empty homogenized case has three quadratics and stated Macaulay
bound4. Its returned basis has degree4. The raw degree5 F4 selected-pair
round is a5×3matrix with zero new polynomials and two zero reductions.
This diagnostic is not silently equated with a nonzero new basis
polynomial of degree5 or a complete audit of every internal polynomial in
the source's intended Assumption2 computation. No refutation of that
qualified bound is claimed. The independent Buchberger peak6 on both JV
homogenized inputs concerns a different algorithm. [pcc:unverified]

Native source checks reconstruct exact field coefficients, bind them to the
input files and validate literal homogenization. They check actual JV
targets/discriminant, irreducibility of t³+t+1 over F101 and the split
example's actual factor-base sum. They do not certify the paper's full
curve order or all listed scalar multiples. Positive controls distinguish
length2/support1 from length1 and detect a deliberately wrong constant.
[pcc:unverified]

Protocol787b3fdd… froze before data, source checkpoint3b231684…,
freeze commit8be3fae3. Readonly original msolve0.6.5/LA1 binary390a41bd…,
independent checker4421041e… and11runtime library hashes are pinned.
All failures, the rejected predata metadata draft, raw outputs and exact
commands remain retained; no postdata retry or instrument change.
[pcc:unverified]

\newpage

![Literal native initial-ideal Hilbert arrays; full degrees0–12 remain in the raw analysis.](hilbert.pdf){width=95%}

The plot copies native values. It shows a finite prefix, with no fitting or
stabilization inference. The JV empty homogeneous quotient has exact
finite staircase length8; the native checker computes Krull dimension
from the complete leading monomial ideal, separately from this displayed
prefix. [pcc:unverified]

The separate Lean project certifies the actual F25 twisted-Edwards input,
factor-base sum, exact source-polynomial symmetrization and basis-coordinate
descent, two nonzero quadratic equations, actual quotient equivalence to
F5[T]/T², and direct nonregularity of the highest homogeneous parts.
Whole gate55/0failures/0trusted-native. Exact local
PCC-20261113-bc105f replay passes; signed standing remains unverified.
The native data, Joux–Vitse source-field descent and full computation-degree
bound have no Lean execution/refinement proof here. [pcc:unverified]

Sources read: FGHR2014 printed603–604/Definitions2–5/§4.1.1/Hypothesis3;
Joux–Vitse author DLFq5.pdf pp4–5/7/9–10, SHA256
6f3268a0ef8c50b51a79f8b64deff0c60374228a827bea73d4e590284cb68377.
Both original JV toy inputs are reproduced. These known in-sample fixtures
do not discharge a full m,n sweep or generic claims. KB unavailable
(MCP absent/wrapper127); bounded primary/repository/Mathlib fallback.
No novelty, timing, independent-host or compiled-refinement assertion.
Paid compute and agent storage uploads: zero. Exact packet/source receipts
accompany this report. [pcc:unverified]
