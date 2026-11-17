#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Quartic recovery: stage accounting
2026-11-05 · T-1687

The instrument now executes and separates compilation, calibration, reusable preparation, initial-window work, decoder routing, incremental extensions and final public-point verification. Eleven generated modules preserve the original source under exact forward/reverse transforms. [pcc:unverified]

#image("chart.svg", width: 100%)

== What the fixture records
The compiler performs 24,076,995 modular-product-helper calls modulo 479. Calibration records 3,001 and preparation 829 product-helper calls modulo 509. These are distinct stages and moduli, not a time comparison. The chart shows the online stages for one known-answer input; the zero-product decoder is excluded from this product chart but records 128 popcounts and rule comparisons. [pcc:unverified]

For k25, both extension geometry steps are charged even though the character at offset 14 is skipped. Initial phase work is also charged. The final scalar is multiplied by P and checked against Q in a separate harness verification stage. [pcc:unverified]

== Checks and coverage
Three accounting tests pass; 120 imported tests were filtered, not rerun. All 20 recoveries agree with the frozen source and match their public points under forward multiplication. Stage tests cover inverse and character counts, nested restoration, thread isolation and allocator forwarding. Thirty-six source/data hashes bind the build. The static source-map verifier accepts eleven transforms and rejects 22 corruptions. Fourteen compiler warnings are retained. [pcc:unverified]

Counts cover modular-product helpers, inverse calls, Euclidean iterations, decoder popcounts and whole-signature comparisons, character consumers and allocator events. Inline constant arithmetic, additions, comparisons, reductions, branches and memory traffic are not exhaustively priced. Allocation bytes are full requested sizes, not peak or net memory. This is stage-resolved arithmetic accounting, not complete machine-work accounting.

== Next construction
Each finite initial-window point separately computes an inverse of 2y for value division. Since all affine points are already available, implement checked batch reciprocals and retain their added products, storage and validation costs. A smaller public compiler representation remains a separate scaling task. No cloud job or timing panel ran; the original 4,563-case cohort is unchanged.

== Evidence and sources
Unsigned claims PCC-20261105-a67f89 and PCC-20261105-6da7ed have local verdict not_covered. The second corrects a projection that omitted the first test-prefixed counter row; original artifacts remain preserved. The checker was authored in the producing session. Source maps, receipts, 1,140 raw event rows and derivation accompany this report. KB unavailable; fallback was saved accounting/source code and the #link("https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html")[Rust allocator contract]. The first generation attempt rejected a repeated fragment before native execution; its failure note is retained. [pcc:unverified]
