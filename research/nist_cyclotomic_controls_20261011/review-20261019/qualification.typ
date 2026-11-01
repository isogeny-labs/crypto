#set page(paper: "a4", margin: 20mm)
#set text(size: 10pt)
#set par(justify: false)
= NIST controls: review qualification
Reviewed head: #text(size: 8pt)[03e4a7d7609e5dc3d406d58bf44f606410f9a78a]

The retained 9,015 rows reproduce all 15 metric summaries and the recorded accuracy and parallelogram gates. The original artifacts and protocols are preserved; hashes and the independent Rust replay are in this directory.

== Factoring: two budgets, incomplete residuals
There are *7,200 control values overall*: 2,400 per curve. M1 is unchanged after the recorded ECM supplement in all 7,200 comparisons, and 150 planted positive controls pass. This establishes agreement under the two recorded budgets. It does not certify complete factoring or invariance under any factoring budget.

#table(columns: (1fr, 1fr, 1fr, 1fr), [Curve], [Controls], [Unsplit composites], [Positive controls], [P-192], [2,400], [1,301], [50/50], [P-224], [2,400], [1,475], [50/50], [P-256], [2,400], [1,592], [50/50])

The 4,368 unsplit entries have no retained certificates excluding all factors below 2^32. Target comparisons remain finite-budget observations.

== Heights: near-zero numerical values
The claimed 59–66 percent zero pairings correspond to a descriptive threshold of absolute cosine at most 1e-12. The stored floating-point zero counts are smaller. Neither category establishes an exactly zero canonical pairing.

#table(columns: (1fr, 1fr, 1fr, 1fr), [Curve], [Pair controls], [Float zeros], [At most 1e-12], [P-192], [500], [82], [296], [P-224], [500], [101], [328], [P-256], [500], [148], [306])

Five doublings and floating-point logarithms were used. The frozen checks pass, including maximum parallelogram defects of approximately 0.001354, 0.001354 and 0.002708 nats. These are instrument checks, not rigorous error bounds on every pair or certificates of exact logarithmic identities. Conservative empirical ties and the original finite-sample decisions replay correctly.

== Boundary of this review
This is a statistical replay of stored rows, not independent recomputation of factoring or rational heights. The lost first attempt and complete cross-attempt accounting are not reconstructed. No new research run was launched. See README.md, replay.json and original-artifacts.json for scope, counts and provenance.
