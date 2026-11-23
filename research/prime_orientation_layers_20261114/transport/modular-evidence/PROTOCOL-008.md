# Prospective explicit extraction and odd-unit inverse

Local306 constructs actual word numerator and canonical entry wrapper with
Boolean prefix cap; existing finiteExtract is still opaque/outside trace.
Preserve all old sources/claims/receipts/archives and every failed output.
[pcc:unverified]

Primary review: Brent and Zimmermann,Modern Computer Arithmetic,CUP2010,
author PDF0.5.9,section2.5 printed65-67,equation2.3; section1.4.8. Exact complete
PDF digest/source URLs and changed partial-download resume are recorded in
literature-review001/download001. Standard Newton/Hensel inversion,not novelty.
Odd-divisor/unit premise is required; do not borrow the book fast O(M(n)) bound
for fixed-width schoolbook circuits or count modular reduction as free.
[pcc:unverified]

First smallest proof: drop E low bits of actual Word(p+E),prove widthp and
unsigned quotient value agrees with existing finiteExtract before inversion.
RetainE0,p0,negative source numerators represented canonically,and verify no
ring inverse of2 or premature reduction modulo2^p.
[pcc:unverified]

Then explicit word iteration y_next=y*(2-b*y),starting at1,using proved
multiply/negate/addWord at fixed widthp. Prove universal squared-error identity,
derive initial even error from actual oddPart(d) with d>0,then correctness
modulo2^p. A conservative p-iteration schedule is allowed; derive its actual
trace from emitted calls,not a favorable operation-cost oracle. Expected
per-step circuit expression10p^2+11p and p-step sum require proof. Do not
assume a supplied unit; derive actual denominator positivity/oddness. Keep
p0 and an even-input control demonstrating the unit condition is material.
[pcc:unverified]

After correctness and actual trace bounds,replace final extraction/inversion
in actual entry driver and account separately for input/odd-part/valuation/
bit-list/constant/scalar/AST/denominator preparation,control,arrays,indices,
trace concatenation,storage and memory. Trace append may be nonlinear. No
full bit/runtime,original creation/mode/public relation/finder/scalar extraction/
retry/expected-cost result follows from Boolean counts alone. No paid/cloud/
native fixture/attack/F4/msolve/timing/scaling campaign or protected history,
claim/trust/signature/test/gate changes. [pcc:unverified]
