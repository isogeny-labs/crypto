#set page(paper: "a4", margin: 14mm)
#set text(font: "DejaVu Sans", size: 9.2pt)
#set par(justify: true)
#set heading(numbering: none)
#align(center)[#text(size: 17pt, weight: "bold")[Joint Richelot model charts over F101]#linebreak()
First frozen arithmetic unit · 17 November 2026]
#block(fill: rgb("edf2f8"), inset: 7pt)[All eight CT49 pairs admit a common rational nonbranch chart. Exact factor/target coefficient checks pass. Claim42a9c2 remains unsigned. [pcc:unverified]]
== Recorded joint charts
#set text(size: 8.4pt)
#table(columns:(1.2fr,0.7fr,0.3fr,0.3fr,0.3fr,0.5fr,1.1fr),inset:3pt,
[Pair],[Eligible a],[a],[b],[c],[Δ′],[bc / constructor],
[Cpi1/j1],
[38],
[3],
[22],
[4],
[99],
[88 / 13],
[Cpi1/j2],
[38],
[3],
[22],
[12],
[2],
[62 / 39],
[Cpi2/j1],
[36],
[3],
[49],
[39],
[86],
[93 / 8],
[Cpi2/j2],
[36],
[2],
[15],
[44],
[86],
[54 / 47],
[C10/j1],
[31],
[1],
[7],
[47],
[6],
[26 / 26],
[C10/j2],
[31],
[1],
[7],
[8],
[95],
[56 / 45],
[C20/j1],
[33],
[7],
[38],
[31],
[20],
[67 / 34],
[C20/j2],
[33],
[1],
[28],
[42],
[25],
[65 / 36])
#set text(size: 9.2pt)
All101 candidate shifts per pair were recorded before the least eligible a was selected. Full monic coefficient vectors and six transformed branch roots remain in the output CSV. The constructor's least square root and bc can have different signs; both are retained for the evaluator. [pcc:unverified]
#align(center)[#image("joint-chart.svg",width:96%)]
== Exact scale argument
For w=1/(x−a), F=w⁶f(a+1/w)/b² and G=w⁶H(a+1/w)/c² are monic. Let S=∏qᵢ(a)=b²/λ. The binary coefficient matrix has determinant−1 and each row is divided by qᵢ(a), giving Δ′=−Δ/S. The three transformed Wronskians yield R\_new=b²w⁶H(a+1/w)=(bc)²G. Written argumentee5c8b and finite checks42a9c2 remain unsigned. [pcc:unverified]
== Coverage and custody
All808 candidates,48 valid factor reorderings and32 legitimate sign choices are retained. Four zero-determinant matchings and24 wrong-lambda/sign/factor controls are detected. Frobenius, six roots, inverse models and every source/target coefficient pass. [pcc:unverified]
Rust1.99.1/offline guarded vendor/j1/120s; a separate555 qualifier ran one new test, with12 inherited tests compiled and filtered. All13 phases, Clippy/fmt and source/dependency/qualifier guards exit0. Complete75635B stdout and195-member601280B declared archive are retained; disposable build cache was excluded prospectively. Both pre-arithmetic infrastructure failures remain. No arithmetic rerun. [pcc:unverified]
== Next technical step and sources
Execute the existing generic Richelot divisor/closed-fibre evaluator with these models, then test dual[2] and prime-order scalar relations. This unit checks the factor/model bridge. Primary sources: CT49/CT50 and ConsumerE079 nonsplit/map; Galbraith corrected8.3 and10.4. Backend reuse and a same-author transform oracle do not supply signed independent verification. [pcc:unverified]
