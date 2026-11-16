#set page(paper:"a4",margin:20mm)
#set text(size:10.5pt)
= Higher-arity principal relations: useful rank recovery
Genus 2, 3 and 4 · 15 November 2026

== Change tested
Keep the same degree-g interpolation and signed tuple stream. Admit split principal relations with three or four off-anchor rational places instead of discarding everything above two. Compute exact retained constraints by ordinary elimination, ordering off-anchor columns first.

For input relation matrix R and its off-anchor restriction O,

$"anchor rank" = "rank"(R) - "rank"(O).$

Every raw principal equation is checked in the original Jacobian. Independent original-order Gaussian elimination checks both ranks. Larger admission policies contain the smaller row spaces, so they cannot lose retained constraints. This is established relation-matrix algebra, not a claimed new exponent.

== Results that change the next action
#table(columns:(1fr,0.5fr,1fr,1fr,1fr),[Genus / prime],[H],[2LP final rank],[Higher arity],[First full rank class],
[2 / 1009],[8],[0 / 7],[Same policy],[Not reached],
[2 / 1009],[12],[11 / 11],[Same policy],[714],
[3 / 503],[9],[0 / 8],[3LP: 0 / 8],[Not reached],
[3 / 503],[14],[0 / 13],[3LP: 13 / 13],[1447],
[4 / 127],[10],[0 / 9],[4LP: 9 / 9],[2139],
[4 / 127],[16],[13 / 15],[3LP / 4LP: 15 / 15],[3304 / 1504])

All six cells and twelve policy records complete. All 2,385 unique principal rows pass full-group checks. No failed rank case is replaced. Genus-2 unchanged controls match the previous screen. These are reused curve families, not new holdouts.

== The larger algebra must be paid for
A separate replay independently verifies the first completing prefix and the immediately preceding rank. Every original summary remains byte-identical.

#table(columns:(1fr,0.5fr,0.5fr,1fr,1fr,1fr),[Genus],[H],[LP],[First class],[Full / off rank],[Elimination muls],
[2],[12],[2],[714],[328 / 317],[4707],
[3],[14],[3],[1447],[214 / 201],[30036],
[4],[10],[4],[2139],[65 / 56],[9578],
[4],[16],[3],[3304],[65 / 50],[6861],
[4],[16],[4],[1504],[65 / 50],[8654])

Genus 4 already spans the entire catalogue rank 65 at those completing prefixes. Genus 3 needs rank 214 out of a possible 235. Small final anchor dimension does not mean a small intermediate system.

== Decision
Test complete supplied-target solves next, keeping source/target descent as catalogue equations. This avoids assuming that a small fixed-anchor rank result survives target support expansion. No tenfold or wall-time claim follows yet. Genus 2 has no extra residual arity to admit. Source/target handling and strongest same-target full-cost controls remain required.

#pagebreak()
= Complete admission and work-count table
Policies in each cell see an identical generated stream and run to a fixed budget. A dash denotes no rank completion. No cases dropped.

#table(columns:(0.9fr,0.4fr,0.4fr,0.8fr,0.8fr,0.8fr,0.8fr,1fr),[g / p],[H],[LP],[Classes],[Rows],[Rank],[First class],[Total elim. muls],
[2 / 1009],[8],[2],[224],[104],[0 / 7],[—],[624],
[2 / 1009],[12],[2],[880],[395],[11 / 11],[714],[7095],
[3 / 503],[9],[2],[1008],[11],[0 / 8],[—],[67],
[3 / 503],[9],[3],[1008],[163],[0 / 8],[—],[6418],
[3 / 503],[14],[2],[8008],[69],[0 / 13],[—],[601],
[3 / 503],[14],[3],[8008],[1075],[13 / 13],[1447],[810587],
[4 / 127],[10],[2],[4032],[6],[0 / 9],[—],[44],
[4 / 127],[10],[3],[4032],[35],[0 / 9],[—],[738],
[4 / 127],[10],[4],[4032],[139],[9 / 9],[2139],[40423],
[4 / 127],[16],[2],[16384],[55],[13 / 15],[—],[1964],
[4 / 127],[16],[3],[16384],[272],[15 / 15],[3304],[72646],
[4 / 127],[16],[4],[16384],[509],[15 / 15],[1504],[195681])

== Read the work boundary correctly
Total elimination counts include overcollection after first completion. For example, genus-3 H14 uses 30,036 multiplications at first completion versus 810,587 for the full fixed stream. These are implementation arithmetic counts, not elapsed times.

Counts omit inversions, allocation and zero scans, relation generation / factoring, setup, group checks, independent verification and target handling. The fixed Gaussian implementation is not an optimized graph baseline. An arm that never reaches rank cannot be used as a finite-time speed denominator. More admissions alone can still yield zero useful rank, as genus-3 H9 demonstrates.

== Exact instances
Genus 2: p1009, c12, ell241259, catalogue478.

Genus 3: p503, c26, ell29977319, catalogue236.

Genus 4: p127, c15, ell134781029, catalogue66.

Canonical first H places; original tuple/sign seed and fixed caps in PROTOCOL.md. Conservative integer Hasse bound and the selected generator certify the rank upper bound. No target scalar enters these relation-rank screens.

== Custody
Original gate: 4e0d00f0bc22, S3 checkpoint 20261116T044956Z.

Replay amendment: 5270937158e7, checkpoint 20261116T045200Z.

Replay source/binary: 8887a6af9e70, checkpoint 20261116T045238Z.

Both scientific executions exit zero, empty stderr. Independent final ranks agree in all policies; five first-prefix / preceding-prefix pairs pass. Filtering only new replay lines gives byte-identical original summaries. Raw outputs, build warnings and hashes are retained. No timing cohort or scientific failure occurred.
