# Ordinary-chart sign information and the native relation equation

Source-bound written derivation, 2026-11-01. These are mathematical implications with stated premises, not a new cohort result or a proof that the native coordinate formulas satisfy every premise.

## 1. Actual retained native state

source/pruned.rs walk::step (lines41–61) carries two nonzero projective four-vectors k and kp, anchor kg, identity k0, constants and repair matrices. It evaluates the mixed differential addition plus exactly the selected double. The intended invariant is k=pi(D), kp=pi(D+G), D=aG+bQ. Branch c in{0,1} updates labels to (2a+c,2b) modulo r, with successor pair pi(2D+cG),pi(2D+(c+1)G). This label recurrence requires rG=rQ=0 and safe native label arithmetic. The row-fold proof at publication5cd0a45205 supplies bounded matrix arithmetic/nonzero preservation; it does not establish these geometric transition equations.

advance returns Some without a signed check when validate_transition=false. On None, signed_fallback reconstructs (2a+c)G+2bQ with native Jacobian arithmetic and projects it and its neighbour. That fallback transports labels by construction under group/arithmetic correctness; it must be charged as actual work. A wrong but nonzero xadd output does not activate this fallback.

## 2. New local fiber derivation: the ordinary chart needs only a shared square

Work over a field of characteristic other than2. Let u=X²+a1 X+a0, and let f mod u=R0+R1 X. For v=b0+b1 X, the two Mumford equations are

    b0²-a0 b1²=R0,       2 b0 b1-a1 b1²=R1.               (1)

The native delta_lift (pruned.rs150–160) extracts a0=q[2],a1=-q[1], z=a1²-a0, then

    S=q[3]-z(f5 a1-f6 z)-a1(f3-f4 a1)+f2.                (2)

Its intended coordinate correspondence is S=b1². Assume that correspondence, valid input divisor and correct field square-root/negation/inversion operations. No assumption a0!=0 is needed for the following argument.

If S!=0, any square root t of S is nonzero. Every possible b1 equals t or -t, since (b1-t)(b1+t)=0. For each b1, equation(1) forces

    b0=(R1+a1 S)/(2 b1).                                 (3)

Thus the two generated pairs are opposites. They satisfy the second equation automatically; the first equation is an explicit validity filter, and at least the true pair satisfies it.

If S=0, a field forces b1=0. Validity requires R1=0, and b0²=R0. Therefore b0 has at most two roots, again opposites. This is exactly the source's b1==0 branch. The source loops over [root,-root], but candidates.contains deduplicates its repeated b1=0 branch before projection filters.

Consequences: the source candidate vector has at most TWO distinct divisors under these assumptions, not merely the asserted bound four. It contains the true divisor and its negation, identified when v=0. Every two valid ordinary divisors sharing u and S differ by simultaneous negation. This proves an ordinary-chart fiber statement once the chart inverse/extraction correspondence is supplied. It does not require squarefree u, so repeated support does not invalidate this local algebra; canonical divisor/group interpretation remains a separate premise.

This is stronger than the previously retained abstract quotient-fiber assumption: the ordinary fiber reduces to the specific coordinate identity(2), plus invertibility of the source's linear chart map and normalization. To complete it for general_coords, prove project_any(D) under general_homogeneous/normalization returns q with u as above and S=b1² for every claimed ordinary input, including cancelled-chart a0=0 cases. Do not substitute C2 for the source C1 degree-one chart.

## 3. What extra information fixes the sign

Suppose pi has fibers {D,-D} on every locus used in both filters and 2G!=0. Two candidates D and -D already agree under pi. If both pass the neighbour test pi(G+D)=pi(G-D), the fiber property implies either G+D=G-D, hence2D=0 and the candidates coincide, or G+D=-(G-D), hence2G=0, excluded. Therefore the ordered adjacent pair fixes the signed divisor uniquely.

The extra datum is the neighbour projection relative to the SAME signed non-two-torsion anchor G, not merely another arbitrary quotient point. The ordinary derivation above discharges the candidate fiber locally; the neighbour may lie in a boundary chart, so its fiber/domain proof remains separately necessary. Candidate completeness and project_any success are needed for existence, beyond uniqueness.

Once D is known, a split u=(X-x0)(X-x1) gives signed ordinates yi=b0+b1 xi. The collector expressly requires x0!=x1. It maps node xi to the convention yi<=p/2 for positive sign. Define the base point at xi using the low canonical root of f(xi); then yi selects its sign. For yi=0 the point is two-torsion, equal to its negative; the positive convention is consistent, but the graph's subgroup/order treatment still needs its existing contract.

The graph equation is therefore

    s0 L(x0)+s1 L(x1)=aG+bQ.                             (4)

Its scalar encoding is s0 ell(x0)+s1 ell(x1)=a+b k modulo r, only in the subgroup/module specified by the graph contract. Choosing -D without changing labels negates the left side of(4), not its right side. Correct alternatives are to recover D's sign, or negate BOTH node signs AND (a,b). Arbitrary local node-sign choices are not allowed.

## 4. Why square-scalar matrices do not prove the repair geometry

An arbitrary M²=cI,c!=0 preserves projective nonzero vectors. Even M=I satisfies this; it cannot represent translation by a designated nonzero two-torsion T on the whole quotient, since at zero the required equality pi(T)=pi(0) contradicts the fiber property unless T=0.

The required source equation is L_T pi(P)=pi(P+T), with2T=0, in the exact chart/model and valid domain. Then transforming x and diff sends P to P+T and P-Q to P-Q+T; a correct differential addition returns pi(P+Q+T), and the second L_T returns pi(P+Q+2T)=pi(P+Q). Matrix-square arithmetic ensures the returned vector is nonzero, while this separate equation identifies its geometric value. Doubling inputs also need a justified xadd specialization, since the abstract ordinary xADD domain can exclude equal operands.

## 5. Relation-information and accounting consequence

The adjacent pair carries orientation information for one divisor. It does not produce two independent linear relations. The two candidates D,-D give scalar-multiple rows, hence rank at most one together. Native relation_seen inserts min(D,-D), which removes both-sign duplicates before graph.edge; it does not itself change the chosen divisor or labels.

Record one physical walk attempt and its selected evaluations; then separate ordinary conversion, split/root work, candidate generation, validity/projection/neighbour filters, duplicate disposition, graph insertion and rank/anchor effect. Even discarded opposite duplicates incur transition and recovery costs. The loose tested<=4 bound must not be substituted for measured tested counts, nor the new mathematical bound2 used to rewrite old counters. Boundary lifting has a different tested counter: its dedup tests matches, so failed candidates can be tested twice. No new operation count or rate follows from this written bound.

A useful next contract is: for each physical row, return (candidate_count, valid_count, signed_match_count, duplicate_disposition, admitted_edge, rank_effect), with source phase counters and setup keyed once by archive identity/row. Information admitted to elimination is counted only after signed recovery and all filters, not per quotient state or square-root sign.
