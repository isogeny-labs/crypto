#set page(paper:"a4",margin:16mm)
#set text(font:"Libertinus Serif",size:10pt)
#set par(leading:0.5em)
#align(center)[
 #text(size:18pt,weight:"bold")[Prefix reuse and complete cold evaluation]
 #linebreak()
 P-192 primary; P-224/P-256/P-384/P-521 retained
 #linebreak()
 Native implementation qualification · 2026-11-08 · PR 3674
]
= Completed point-action component
The new point bank retains labelled multiples produced by the radix prefix and Q-odd table. A shared digit column uses a retained point only when its exact integer coefficient equals the required combination. Zero combinations cancel. The same policy applies to binary controls. The original marked $mu$ and the identity $mu_0=m k+c$ are unchanged.

#image("flow.svg",width:100%)

All 6,300 boundary point/schedule cases pass. A separate GMP recoder/count model checks the schedule; the chain program is common declared data, while its dependencies and counts are reconstructed independently. All 71,570 retained point labels match independent OpenSSL scalar multiplication. Five positive cache-hit and five wrong-residual controls pass. The full cold worker's final version passes 625 untimed paths and rejects five corrupted expected outputs.

#image("coverage.svg",width:100%)

= Complete cold boundary
The worker starts before serialized request parsing and ends after verified compressed output and required destruction. Five exclusive phases cover setup, plan preparation, arithmetic/normalization, symmetric output verification and destruction; they must sum exactly to total. Each call creates fresh curve/point/table objects.

The primary method deterministically chooses offset0 and window4/5/6 from subgroup bit length, with its quotient/remainder calculation inside the call. It has no installed-plan search dependency. Secondary installed modes load and parse their plan artifact inside the call; the historical search cost remains separately recorded and cannot support a full from-scratch first-use claim by itself. Library/process startup and known-answer workload generation are identified symmetric exclusions, not hidden point preparation.

= Prepared timing design; no timing result
The schema-valid draft uses 4 calibration and 8 held-out full-width nonzero points per curve, 25 configurations, one warmup and six paired rounds. The 60 requests are generated and checked; all regular references have A/A twins. The prospective gate requires the deterministic candidate to beat the fastest reproduced regular/binary reference at every held-out point beyond max(1.01,the maximum paired A/A envelope). All curves must be reported.

The native row/receipt analyzer and its synthetic/corruption tests remain to be completed before source/binary/input binding, freeze/publication and host admission. A direct launch without the reservation descriptor rejects before timed computation. No new wall-clock value, runtime ratio, global noninteger quotient, cloud submission or scientific promotion is claimed by this qualification report.

#text(size:8pt)[Evidence: ../../qualification/cache-build-v1/, ../../qualification/cache-cold-build-v1/ and v2/. Contract: ../../CACHE_COLD_CONTRACT.md. Draft/points: ../../protocol/cache-cold-v2/. Standard background checked: Grabner–Heuberger–Prodinger, helmut-prodinger.at/pdffiles/joint.pdf, introduction/section2; Bernstein–Lange, hyperelliptic.org/EFD/precomp.pdf, precomputation/normalization discussion. KB tools and migrated wrapper unavailable; exit127 recorded. No novelty or optimality claim.]
