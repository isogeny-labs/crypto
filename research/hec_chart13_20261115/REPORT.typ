#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Runtime quartic chart construction
Genus 3 · F127 · 15 November 2026
#image("flow.svg",width:100%)

All three preregistered coordinate representations passed once after source freeze 0ec950e6b995 and S3 checkpoint 20261116T001855Z. Candidate charts, tangent residuals, canonical lines and control points were discovered from runtime quartic coefficients. These are representations of one curve, with no new timing comparison.

== Exact section and cancellation checks
The tangent cuts 2O+D, giving K equivalent to 2O+D. Cubics vanishing on D form the eight-dimensional space H⁰(3K−D), isomorphic to H⁰(2K+2O). Each model passes multiplication rank 18, diagonal rank 112 and all exact coefficient checks. Each passes 480 determinant, 480 ordinary split tensor and 120 base-divisor tensor controls. Two models exercise irreducible quadratic cancellation over F127²; the third exercises split roots. Corrupted sections are rejected.

== Independent compatibility controls
Translations (0,0), (1,1), (7,11) select O=(0,4), (0,36), (0,33). They pass respectively 63, 63, 64 negation pairs and 117, 121, 123 graph overlaps. Irreducible cubic, mixed and fully split graph divisors are covered. Each basis-change system has rank 63 and an invertible rank-eight matrix; twelve training and four withheld pairs pass. Coordinate and matrix corruptions reject. Unsupported cases are retained in raw logs.

The legacy chart is used only as an independent comparator transported by each certified affine isomorphism. Its archived points are explicitly retained in that comparator; candidate selection does not read them.

== Scope and next work
This is engineering correctness evidence, adapting Tian 2020 section 4.2.3, https://arxiv.org/html/2007.03172. The complete geometry is not formalized in Lean. Searches are bounded, the field is fixed, and the cohort contains one isomorphism class. Full cold-driver integration, wider fields, scalable linear algebra and genus two/four remain. The tenfold mathematical speedup remains unestablished; the prior failed cold performance comparison is unchanged.

Raw inputs, stdout, stderr, status, source, lockfile and binary hashes are adjacent to RESULTS.md. No dropped case, unexpected failure or scientific rerun occurred.
