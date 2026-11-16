#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Checked initial-window reciprocals
2026-11-05 · T-1692

The public-point source now batches reciprocals of 2y for its finite initial-window points and supplies them to a checked value-division constructor. Calibration and incremental extra characters retain the scalar path. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== Arithmetic and validation
For m nonzero operands, prefix products and a backward sweep require 3(m-1) products and one inverse. Empty or all-identity windows need no inverse. Identity holes and duplicate operands preserve their positions. Each supplied reciprocal is canonical, nonzero and checked by (2y)z=1 after the original curve/point checks. The division-polynomial recurrence is unchanged. [pcc:unverified]

The reciprocal checks add m products. The multiplication/inversion substitution is therefore (4m-3)M-(m-1)I, plus batch doubling, branches and O(m) storage/traffic. A singleton has no inverse reduction and one added validation product. This is an arithmetic tradeoff, not a timing result. [pcc:unverified]

== Development evidence
The native suite passes 125 tests with 26 source/data hashes and 15 retained compiler warnings. New coverage includes 165 batch-kernel cases over five prime moduli, identity holes, duplicates, wrong reciprocals, noncanonical inputs, zero y and a nonunit composite operand. Twenty recoveries agree with the frozen implementation. Three small-prime source fixtures check identity-containing windows against standalone signs. [pcc:unverified]

The accounting build passes three tests; 185 imported tests are filtered. It checks 20 frozen-output agreements and 20 forward point verifications. Eleven generated modules pass exact forward/reverse source-map checks and 22 corruption controls. Thirty-six source/data hashes and 22 warnings bind the build. [pcc:unverified]

The complete raw projection has 1,280 event rows. For k25 the batch stage records 39 products, one inverse and 1,120 requested allocation bytes. Initial-character value division records 3,430 products, including 14 reciprocal checks, and zero inverses. Extra-character work remains charged separately. Allocation bytes are not peak/net memory. [pcc:unverified]

== Scope and next construction
No timing panel or cloud job ran; original 4,563-case cohort preserved. Next investigate private validated input objects to reuse source validation, then shared polynomial work across shifted points. Public coefficient-field enumeration and dense synthesis remain a separate representation task.

== Evidence and sources
Claims PCC-20261105-af2637 and PCC-20261105-8190fe have local verdict not_covered and no signed standing. The producing session authored these checks. Sources: saved source/accounting artifacts and #link("https://docs.rs/ff/latest/src/ff/batch.rs.html")[ff batch inversion], which identifies Montgomery's trick. This applies established arithmetic; no novelty claim. KB unavailable: repository wrapper found no executable.
