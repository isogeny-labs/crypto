#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Target-conditioned rational relations: exact orbit boundary
16 November 2026 · hec_target_rational_20261116 · frozen finite algebra gate

== Construction and checked identity
Let D=(u,v) have full degree and nonbranch support, allowing repeated roots of u. Set B=b0+b1x, C=c0+c1x and A=vB+uC. Three signed anchor constraints give a3by4homogeneous linear system. For an admitted primitive pair with valid denominators, the norm divided by u times the anchor polynomial has residual degree at most g. The resulting reduced E satisfies D+P1+P2+P3+E=0.

Unique reduced representatives therefore force E=reduced(-D-P1-P2-P3). The family changes how that group orbit is evaluated; it supplies no different residual or splitting decision for the same admitted signed input. This uses classical Riemann–Roch and Cantor arithmetic, with no novelty or speed claim.

== Frozen gate results
Source61bcd604593f; prior S3 checkpoint20261116T161039Z. Source/import/binary/protocol/theory/lock freeze checked before once-only execution. Exit0, stderr empty. Fixed genus2/F251, genus3/F211, genus4/F61 and two targets each; reused panel, no fresh holdout claim. Prefix H8/9/10; up to64lexicographic triples, all8anchor masks with fixed target. No sign folding, adaptive replacement or rerun.

#image("../../COUNTS.svg",width:100%)

All2856admitted sign-class cases equal direct Cantor exactly. There are88nonprimitive rejections,640original-group checks on split residuals,25degree drops and503admitted cases on the repeated-support target. Other rejection counters are zero but retained. All2944attempts remain. Attempts are not independent relations or rank; branch sign choices may coincide. Exact deterministic counts, no timing, population estimate or uncertainty interval.

#pagebreak()
== Full finite summary
#table(columns:7,inset:4pt,
[g / p],[Target],[Attempts],[Admitted],[Rejected],[Split],[Degree drop],
[2 / 251],[0],[448],[436],[12],[209],[2],
[2 / 251],[1],[448],[448],[0],[201],[3],
[3 / 211],[0],[512],[492],[20],[75],[5],
[3 / 211],[1],[512],[503],[9],[82],[2],
[4 / 61],[0],[512],[485],[27],[39],[6],
[4 / 61],[1],[512],[492],[20],[34],[7])

Every admitted case checks exact norm divisibility, degree bound, denominator invertibility, valid Mumford residual, and exact equality to the direct reduced group result. Split residuals reconstruct the original principal identity with multiplicities. Smoothness uses the same root oracle on identical inputs; this is not independent root-finding validation. Shared field/group arithmetic remains a dependency.

Repeated nonbranch input support is retained through the congruence A=vB modulo the full u. The earlier repeated-support target now has503admitted comparisons; Cantor already supported it. Branch target support, nonprimitive normalization, canceled anchor conditions and higher-nullity spaces remain outside the admitted primitive gate. No source descent or complete scalar solver is claimed.

== Fixed target forbids the old global sign fold
Global negation changes D to -D as well as changing anchor signs. With D fixed, the target-free2^(3−1)fold has no such justification. All2^3masks remain in this gate. Any smaller sampler needs its own proven equivalence and must account for the changed target workload. Sign attempts are not doubled independent rank.

== Decision at the complete-cost boundary
For identical admitted signed inputs, a different residual presentation cannot give new relation information. A useful successor must demonstrate cheaper evaluation/batching or a sampler reaching useful inputs at lower complete cost. Charge nullspace, primitive checks, inverses, residual construction, roots, catalogue, matrix and all source/target work against Compact. Comparing only with Gaussian interpolation would not establish that case.

This exact equivalence does not exclude every arithmetic speedup or every sampling policy. It prevents claiming a mathematical density gain from the representation alone. No timing campaign follows from this gate; the tenfold objective remains open.

== Reproducibility and scope
Raw CONTROL.tsv, CONTROL.stderr and EXIT.txt; pre-run PROTOCOL.md and FREEZE.sha256; proof/domain/source receipt THEORY.md. Native reduce.rs summarizes fixed counters and renders the graph. Both PDF pages visually inspected. No scientific Python, independent scientific validation, machine-checked group certificate, benchmark-index/leaderboard entry or performance claim. Existing sealed reports/canonical timing graphs unchanged.

KB lexical snapshot revision9141e386254340a9bdba3036f2bb706a predates current studies; sources and queries are recorded in THEORY.md. Current rational interpolation source and prior exact-equivalence results were read directly. Primary external PDF access gap is retained; no priority claim. Today's shared log remains reserved by T-6720; this study's SESSION.md holds the handoff. PR4357 remains unmerged.
