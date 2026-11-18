#set page(paper:"a4",margin:20mm)
#set text(size:10.5pt)
= Confluent principal relations: algebra and useful rank
Genus 2, 3 and 4 · 15 November 2026

== Construction and a cancellation distinction
Impose local multiplicity conditions on a-b y, using x-xi at nonbranch points and y at branch points. The norm is divisible by the selected multiplicity polynomial. Removing gcd(a,b) changes the surviving conditions: max(m-h,0) at nonbranch anchors, max(m-2h,0) at branch anchors. Keeping the wrong conditions can create invalid relations.

Gaussian construction agrees with independent Hermite-CRT / Euclidean primitive reconstruction on all nonbranch controls. Classical rational interpolation is prior art; see D'Andrea / Krick / Szanto, sections 2 and 4, in PROOF.md.

#table(columns:(1fr,1fr),[Algebra gate],[Result],[Total cases],[600 pass],[Independent nonbranch reconstructions],[561 pass],[Branch profiles / fixtures],[39 / 3 pass],[Common factors / degree drops],[47 / 49 covered],[Split principal group identities],[124 pass],[Higher nullities],[8 covered],[Corrupted jet / cancellation controls],[Pass])

== Rank comparison after primitive and row deduplication
Identical nonbranch anchor panel per genus; d0 uses M=g+1, d2 uses M=g+4. The mixed policy includes all multiplicity multisets, with preregistered multinomial sampling bias. Every constructed pair passes independent reconstruction; every admitted unique row passes a full Jacobian identity. Independent final full / off ranks agree in all twenty policy records.

#table(columns:(0.8fr,0.4fr,1fr,1fr,1fr),[Genus / H],[d],[Distinct first completion],[All-multiset first completion],[Observation],
[2 / 8],[0],[Not reached],[Not reached],[Both exhaust],
[2 / 8],[2],[719],[749],[More construction],
[3 / 9],[0],[Not reached],[1414],[Extra capacity helps],
[3 / 9],[2],[1436],[1489],[More construction],
[4 / 10],[0],[1711],[1805],[More construction],
[4 / 10],[2],[1350],[1477],[More construction])

First positions are incremental-elimination records; independent verification covers final ranks. Two-LP genus-3/4 subsets all remain at zero retained rank. No incomplete policy is a finite speed denominator. Exact streams and all caps remain in raw output.

== Decision
Repeated support adds a real finite family-capacity benefit, but the mixed policy never improves first-completion count where the same-d distinct family already completes. Most added primitive functions survive deduplication; useful-rank density, not simply duplicate removal, is the problem. No tenfold premise, full solver or timing campaign follows from this screen. Retain the correct algebra; seek a constructive useful-relation density mechanism before another prototype.

#pagebreak()
= Complete family counts and exact capacity
#table(columns:(0.65fr,0.3fr,0.8fr,0.8fr,0.8fr,0.8fr,0.6fr,1fr),[g / H],[d],[Profiles],[Constructed],[Unique functions],[First full rank],[Rank],[Terminal],
[2 / 8],[0],[Distinct],[224],[224],[—],[0 / 7],[Exhausted],
[2 / 8],[0],[All],[344],[338],[—],[0 / 7],[Exhausted],
[2 / 8],[2],[Distinct],[896],[892],[719],[7 / 7],[Exhausted],
[2 / 8],[2],[All],[4096],[4063],[749],[7 / 7],[Function cap],
[3 / 9],[0],[Distinct],[1008],[989],[—],[0 / 8],[Exhausted],
[3 / 9],[0],[All],[2241],[2176],[1414],[8 / 8],[Exhausted],
[3 / 9],[2],[Distinct],[2304],[2276],[1436],[8 / 8],[Exhausted],
[3 / 9],[2],[All],[4096],[4058],[1489],[8 / 8],[Function cap],
[4 / 10],[0],[Distinct],[4032],[3818],[1711],[9 / 9],[Exhausted],
[4 / 10],[0],[All],[4096],[3899],[1805],[9 / 9],[Function cap],
[4 / 10],[2],[Distinct],[4096],[3798],[1350],[9 / 9],[Function cap],
[4 / 10],[2],[All],[4096],[4005],[1477],[9 / 9],[Function cap])

Table shows maximum arity (2, 3, 4). All twenty policy records, primitive / row duplicates, common factors and draw attempts remain in RANK.txt. No rows dropped. These are deterministic construction counts, not time units; independent reconstruction is verification work rather than a proposed production kernel.

== Post-run combinatorial derivation
For H nonbranch anchor locations and total multiplicity M, choose r support locations, a positive composition of M, and 2^(r-1) sign classes. Hence

$ F_("all") = sum_(r=1)^(min(H,M)) binom(H,r) binom(M-1,r-1) 2^(r-1), quad F_("distinct") = binom(H,M) 2^(M-1). $

Exhausted H8 / M3 gives 344 versus 224; H9 / M4 gives 2241 versus 1008, matching recorded counts. For fixed M and growing H, the ratio is 1+M(M-1)/(2H)+O(H^(-2)). This counts input profiles, not independent primitive functions or relations. It supplies no leading-exponent improvement by capacity alone; it is not a lower bound on targeted repeated-support algorithms. See CAPACITY.md for assumptions and limits.

== Custody and scope
Algebra freeze 2d6ff1170207, S3 checkpoint 20261116T051710Z. Rank freeze 3ec82e6d3468, checkpoint 20261116T052001Z. Both ran once, exit zero, empty stderr. Source / lock / native arithmetic / binary hashes precede execution. No scientific or build failure. Shared field / polynomial primitives remain a verification dependency.

Reused curves: g2 / p1009 / c12 / ell241259; g3 / p503 / c26 / ell29977319; g4 / p127 / c15 / ell134781029. Nonbranch anchors and exact abscissae frozen in raw output; no holdout or population estimate. No timing claim, novelty claim, PR merge or completion of the tenfold objective.
