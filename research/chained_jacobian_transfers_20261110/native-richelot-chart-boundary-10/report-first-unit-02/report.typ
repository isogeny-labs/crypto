#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 10.5pt)
#set par(justify: true)
#set heading(numbering: none)
#align(center)[#text(size: 18pt, weight: "bold")[Richelot model charts over F101]#linebreak()First bounded native unit · 16 November 2026]
#block(fill: rgb("edf2f8"), inset: 8pt)[All 12 frozen fixtures pass complete rational-point and projective-identity checks. Claim 1422f5 remains unsigned. This is a model-conversion component; full Jacobian transfer and cost remain open. [pcc:unverified]]
== Recorded models
The four source fixtures and eight CT49 target sextics produce the following monic models. Each input and output point set has 120 rational points, checked by both enumeration and quadratic characters. All 102 projective arguments per model and both point roundtrips pass. [pcc:unverified]
#set text(size: 8.3pt)
#table(columns: (1.35fr,0.25fr,0.25fr,0.65fr,2.3fr), inset: 4pt,
[Model],[a],[b],[Points],[Output coefficients, ascending],
[Cpi1],
[0],
[31],
[120/120],
[[56, 0, 85, 0, 84, 0, 1]],
[Cpi2],
[2],
[15],
[120/120],
[[26, 9, 50, 59, 49, 7, 1]],
[C10],
[1],
[7],
[120/120],
[[94, 59, 58, 7, 3, 82, 1]],
[C20],
[1],
[28],
[120/120],
[[75, 46, 74, 23, 6, 55, 1]],
[T(Cpi1,1)],
[2],
[20],
[120/120],
[[37, 71, 4, 32, 40, 13, 1]],
[T(Cpi1,2)],
[2],
[16],
[120/120],
[[1, 33, 98, 43, 17, 79, 1]],
[T(Cpi2,1)],
[0],
[28],
[120/120],
[[77, 32, 0, 33, 80, 85, 1]],
[T(Cpi2,2)],
[0],
[28],
[120/120],
[[77, 69, 0, 68, 80, 16, 1]],
[T(C10,1)],
[0],
[7],
[120/120],
[[27, 95, 79, 43, 78, 56, 1]],
[T(C10,2)],
[0],
[7],
[120/120],
[[27, 6, 79, 58, 78, 45, 1]],
[T(C20,1)],
[0],
[5],
[120/120],
[[100, 46, 95, 50, 27, 12, 1]],
[T(C20,2)],
[0],
[5],
[120/120],
[[100, 55, 95, 51, 27, 89, 1]])
#set text(size: 10.5pt)
Source labels Cpi1/Cpi2/C10/C20 refer to the exact frozen rows; T(C,j) denotes the shift-j target of C. The full labels, coefficients and all point witnesses remain in the fixture file and trace archive. [pcc:unverified]
== Reproducibility boundary
Source 3eaf2d5733a9/library 06fe86f1 was publicly tagged before calls. Offline locked Rust 1.99.1, one compiler worker and 120-second caps were used; the native qualifier was copied separately, made read-only and hash-checked before/after. All four tests, strict all-target Clippy, format and source guards pass. No benchmark or performance ratio is reported. [pcc:unverified]
#pagebreak()
#align(center)[#text(size: 17pt, weight: "bold")[Infinity completion and limitations]]
#align(center)[#image("infinity-cases.svg", width: 95%)]
== Exact coordinate contract
For b²=H(a)≠0: u=1/(x−a), v=y·u³/b and F(u)=u⁶·H(a+1/u)/b². The inverse is x=a+1/u, y=b·v/u³. Both signed output infinities are included. A nonsquare input leading coefficient gives no rational input infinity and no output point at u=0. These are checked finite models, not a generic Lean curve or compiler theorem. [pcc:unverified]
== Complete retained trace
The 287462-byte stdout contains 100 base inverses, 1224 homogeneous arguments, 1440 forward, 1440 inverse and 1440 sign records. One 493468-byte whole archive retains 110 source/compiler/qualifier/log files; every member's bytes, SHA and mode are checked. No producer run is repeated for this report. [pcc:unverified]
== Controls and remaining work
Coefficient, reversal, normalization, invalid-point, malformed, singular, degree, parameter, search and lost-infinity controls reject. Both signs of b still satisfy curve equations and roundtrips; Richelot dual orientation remains unestablished. The prospective exhaustion amendment predates implementation. [pcc:unverified]

Next is an independent native trace auditor, then composition with the existing balanced divisor and Richelot code. The monic vectors meet one constructor condition; integration, nonrational support, full Jacobian groups, dual[2], generic formalization and actual ECDLP costs remain open. CT50's forced-return negative is unchanged. No original LP workload, paid job or signed scientific verdict is claimed. [pcc:unverified]
