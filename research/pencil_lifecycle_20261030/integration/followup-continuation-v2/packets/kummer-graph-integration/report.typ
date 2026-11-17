#set page(paper:"a4",margin:16mm)
#set text(size:9pt)
#text(size:17pt,weight:"bold")[Kummer state to signed graph]
#image("coverage.svg",width:100%)

#text(weight:"bold")[Propagated Kummer states connected to signed relation graphs]

#text("The fixed original-G-step software trajectory now passes recovered signed divisors into the existing signed graph collector. On21 retained models with3 transitions each, both arms emit35 identical equations. All35 decompositions per arm pass exact point-divisor validity and sum checks. Every inserted prefix agrees with a separately implemented dense Gaussian elimination reference over F_r.")

#text(weight:"bold")[Observations]

#text("The two added degree1 recovery successes do not increase emitted-edge coverage: the preserved graph interface accepts degree2 divisors with distinct rational roots. No trajectory was extended to seek a cycle. Prior recovery failures remain in source-parent evidence and this run's eager rows.")

#text("Three separate synthetic signed-equation controls over F_101 return the planted value7. A positive cycle stops at prefix3; a negative cycle anchors a node and stops at prefix2; a target-free repeated-edge cycle increments zero_coefficient then stops at prefix4. The dense reference returns the same value at every prefix. Each control has an inconsistent trailing sentinel that is not consumed after stopping. These controls exercise elimination and stopping, separately from the63 geometric states.")

#text(weight:"bold")[Interfaces and correctness boundary]

#text("The paired walker keeps (K(R),K(R+G)) and labels R=aG+bQ. Recovery consumes this propagated pair directly. For recovered degree2 divisor (u,v), the adapter splits u into distinct roots x0,x1, obtains yi=v(xi), uses node xi with sign determined by yi<=p/2, and inserts s0 X0+s1 X1=a+b*k. It deduplicates signed divisors up to negation as the collector does. Exact factor verification occurs after insertion; aG+bQ reference construction also follows recovery and insertion. The adapter receives no expected divisor.")

#text("Dense elimination retains every inserted equation, with node variables and k, and produces a target value only from a row whose node coefficients vanish. Graph operations, constraints and prefix length are recorded. Point-divisors are not assumed individually r-torsion; subgroup assertions apply to G,Q and each hypothetical returned scalar is checked by multiplication and planted-answer equality. No geometric scalar-return branch was exercised here.")

#text("This wrapper attempts recovery on every state before the split gate, continuing the preceding recovery validation. The historical collector normally filters using general Kummer coordinates before attempting recovery. Thus emitted-edge semantics are compared, not production recovery scheduling or total costs. Degree1 skips are explicit. Original-G repair/fallback and Delta-zero branches are not exercised by these fixtures.")

#text(weight:"bold")[Cost boundary and remaining integration]

#text("Separate records contain initialization, transition, recovery, splitting/sign extraction, graph insertion, and factor/reference verification. Ring snapshots and graph-operation deltas accompany each emitted equation. Dense elimination is a checking reference, with no independent operation census; allocation, hashing, fixture parsing, and reference work are not a complete priced cost model. Underlying square-root nonresidue search remains work, and native field counters are incomplete for some modular exponentiation. No timing or rho ratio is reported.")

#text("The next concrete integration gap is a fixed geometric relation-cycle fixture that enters via propagated original-G states and reaches scalar-return verification. The current63-state trajectory contains no cycle; the terminal controls cover only signed equations. Duplicate and repeated-root geometric gates likewise have no observed coverage. Complete production scheduling, restart behavior, stopping and source-elliptic verification remain separate interfaces to qualify.")

#text(weight:"bold")[Custody and references]

#text("Frozen PLAN.md preceded implementation and the bounded offline Cargo invocation. receipt.json records exit0. run.stdout retains both arms, all equations, phase counters and terminal controls; status.json and phases.csv are standardized derivatives. All changes are private. Independent review pending.")

#text("Chung–Costello–Smith, Fast, uniform, and compact scalar multiplication, sections2,6,7: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf. Existing collector protocol: research/kummer_differential_collector_20261022/PROTOCOL.md; SignedGraph implementation in native/src/three_leads.rs. The Gaudry–Thome–Theriault–Diem double-large-prime paper was located at https://eprint.iacr.org/2004/153.pdf but fetch returned403; no theorem claim rests on its search summary. Plan and source hashes are in manifest.json.\n")
