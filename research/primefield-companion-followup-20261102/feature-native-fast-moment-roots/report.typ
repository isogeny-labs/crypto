#set page(paper: "a4", margin: 18mm)
#set text(size: 10.5pt)
= Fast moment-to-root conversion
T-1799 • 2026-11-07 • PCC-20261107-cc6428 • unsigned producer evidence

The public-point recovery path now builds its candidate root polynomial by coupled formal-series precision doubling. It preserves the existing seeds, sign transcripts, root updates and mandatory scalar-to-point verification. The change replaces the quadratic coefficient-wise Newton conversion with fast polynomial products.

#image("flow.svg", width: 100%)

== Identity and construction
For n candidate roots with power sums mu_j, define q(Z)=product of (1−t_i Z). Then q=exp(H), where H=−sum mu_j Z^j/j and W=H′. Prime characteristic r and n≤r−1 ensure every integration denominator is invertible.

Maintain q and its reciprocal g to precision m. Only the high coefficients of Wq enter the differential correction; q′ has no terms from degree m−1 onward. Multiply this residual slice by g, integrate at the correct absolute degrees, and apply the correction to q. Update g for the next precision, skipping its final unused update. Reverse exactly n+1 coefficients, retaining zero roots.

== Exact checks
Validation-02 passes three selected methods; 320 imported methods are filtered. All 522 small multiset cases, including repeated and zero roots and degrees through r−1, match both direct linear-factor products and the separate quadratic Newton reference. Seven larger degrees—127,128,129,511,512,513 and1024—match both references and exercise Karatsuba multiplication.

All88 triple sign-class root polynomials agree with the previous adapter. All20 saved public-point cases on F509, y²=x³+2x+10, P=(3,68), r=479 recover with identical complete sign transcripts and initial root degrees. Third endpoint477, wrong scalar, invalid coordinate, moment shape/canonicality, characteristic boundary, query and degree caps have controls.

Thirty-one source/input hashes are bound. The unchanged RootSet and recovery implementation are extracted with source/output hashes. The initial passing exp/log implementation is preserved with validation-01 and its exact source snapshot; validation-02 binds the coupled algorithm. No native build/test failure occurred. Test/normal builds retain34/90 warning lines, including summaries, from imported and unused code.

== Cost and next construction
Each doubling uses a constant number of products at that precision. Summing the geometric sizes gives O(M(n)+n) field work for installed Karatsuba multiplication, with M(n)=O(n^log₂(3)). This is an arithmetic derivation, not a timing result or a performance comparison with FLINT. Small-degree overhead and dispatch thresholds remain unmeasured.

Root arrays remain O(n), with multiplication workspace also charged. Consecutive triple sets are generally of order r/8. Seed preparation, the full moment prefix, source evaluation, later gcds and final verification remain distinct costs. The next mathematical task is larger-transcript aggregation without expanding all subsets or materializing the full root set. The original4,563-case cohort is unchanged.

== Sources and evidence
Existing lifted-sign-moments already has a2^J subset extractor; this round targets a separate concrete cost. FLINT exp/log documentation and linked source informed the standard construction; DERIVATION.md supplies the proof, links and characteristic restrictions. KB unavailable. Receipts, commands, warnings, hashes and manifest are retained. No independent signed verdict, timing panel or cloud work.
