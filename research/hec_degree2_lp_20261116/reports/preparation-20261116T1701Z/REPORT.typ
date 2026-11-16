#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Quadratic large-prime endpoints: prepared rank gate
16 November 2026 · hec_degree2_lp_20261116 · design/status report

== Exact mechanism and budget
A nonbranch irreducible quadratic residual defines a Mumford divisor E=(Q,v). Its canonical signed identity requires both Q and v modulo negation. Each retained principal row has anchor part a_i plus one signed endpoint variable. With N rows and L distinct nonzero endpoint labels, the endpoint-column block has rank L. Eliminate every later appearance against the first signed appearance. At most N−L independent anchor constraints can result. Z known-zero branch endpoint rows add at most Z. Repeated hits may still be dependent.

This is standard rank-nullity and closed-place arithmetic, not mathematical novelty. The planned gate will compare the eliminated construction against independent full/off-column ranks of the matrix containing every quadratic variable. It will also check canonical signs and original Jacobian identities. None of those finite checks has executed yet.

== Concrete preparation completed
Native binary builds successfully. Scientific source, protocol, theory, lock, imported modules and binary hashes are frozen in commit ff43eaeaff4f. The planned genus-2 bottleneck panel uses F251/c11, ell61667; H7/H13 and prefix/rotation/fixed-shuffle membership. Each case lexicographically exhausts all unsigned triples and four target-free Gray classes. These are planned counts, not observed outcomes. No source/target scalar recovery or elapsed-time measurement is claimed.

#image("../../PREPARATION.svg",width:100%)

A status diagram is supplied because there are no measured values to graph. CONTROL.tsv, PREFIX.tsv and EXIT.txt do not exist. The candidate has not been run or judged against its numerical rank gate.

#pagebreak()
== Conditional cost warning
Under an additional independent uniform-label model with pool size M, the expected duplicate budget is N−M(1−(1−1/M)^N), approximately N(N−1)/(2M) when N is small relative to M. Degree-two endpoint pools have order q², rather than the order-q rational-point LP pool. If N is of order H³, a route relying on order-H quadratic-collision constraints would need H of at least order q^(2/5), hence order q^(6/5) generated outputs under this model.

This is conditional heuristic reasoning, not an unconditional lower bound or a distribution theorem for these anchor subsets. Deliberate concentration or correlated sampling can invalidate it. The prepared finite gate exists to measure actual labels and rank; extra partial-row yield alone is not new useful rank. No higher-genus implementation or timing campaign follows before that test supplies a complete cost premise.

== Primary source checked
The accessible university copy of Gaudry, Thériault and Thomé's 16 February 2005 preprint was opened and read at its abstract and sections 2–3. It defines the ordinary LP set using curve points and discusses birthday combinations and graph elimination. Its stated main small-genus analysis is for g≥3; it does not establish the degree-two genus-2 conclusion above. URL and KB/current-source receipt are in PROTOCOL.md. The older Joux–Vitse access gap concerns a different paper. No priority claim is inferred.

== External custody failure and exact resume boundary
The default presigned checkpoint transport returned HTTP403. The ecc-challenges-new credential profile returned InvalidAccessKeyId. A default profile authenticates, but the required setec-astronomy-artifacts destination returns AllAccessDisabled on upload. Known transports/profiles were checked without printing secrets or changing storage policy. No approved alternative destination was identified in the current shared log.

The frozen protocol requires a successful checkpoint before scientific execution. The gate remains unexecuted. Restore the existing destination, or document an approved replacement in a custody amendment, then run resume.sh. The script refuses existing outputs, checks frozen hashes, requires checkpoint push/gate success and only then starts the once-only180second bounded binary. Failed custody does not create an experiment EXIT.txt or silently authorize a different run.

== Evidence and remaining work
PROTOCOL.md, THEORY.md, FREEZE.sha256, BUILD.txt/stderr and committed main.rs make the design reviewable. CHECKPOINT-STATUS.md records access results. No scalar output, collision count, rank comparison, independent scientific validation, benchmark admission or speed result exists for this preparation. All pages visually inspected. The PR remains unmerged and the full mathematical tenfold objective stays active. Execution awaits approved checkpoint access; subsequent native reduction/reporting remains required.
