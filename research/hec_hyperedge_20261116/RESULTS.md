# Higher-arity principal relations: rank recovery and first-prefix work

## What changed

Retaining split principal relations with three/four off-anchor rational places closes several previously observed small-anchor rank gaps. The generated polynomial family is unchanged: degree-g interpolation through g+1 selected signed anchors, followed by exact norm division and factorization. Policies differ only in admitting at most2,3or4off-anchor places. General elimination is established index-calculus algebra; no new large-prime complexity or novelty is asserted.

All6fixed curve/anchor cells and12policy records completed. Every2,385unique principal row across cells passed its unprojected Jacobian zero check. Independent original-column-order Gaussian calculations agree with both final full and off-anchor ranks in every policy. Original run exits0, empty stderr. The field/Jacobian primitives are shared, so this is an independent elimination algorithm, not an independent implementation of curve arithmetic.

## Complete fixed-stream results

Each cell uses one identical tuple/sign stream for all policies. Generation continues to its frozen tuple/class budget even after rank completion. Rank is the dimension of the row-space intersection with anchor coordinates, computed as fullrank-offrank. Target rank is H-1. First completion counts include all visited sign classes, including nonsplit, duplicate and discarded outputs. A dash means no completion in the complete fixed stream.

|g,p|H|maxLP|Visited classes|Admitted rows|Final anchor rank|First complete class|Full-stream elimination multiplications|
|---|---:|---:|---:|---:|---:|---:|---:|
|2,1009|8|2|224|104|0/7|—|624|
|2,1009|12|2|880|395|11/11|714|7095|
|3,503|9|2|1008|11|0/8|—|67|
|3,503|9|3|1008|163|0/8|—|6418|
|3,503|14|2|8008|69|0/13|—|601|
|3,503|14|3|8008|1075|13/13|1447|810587|
|4,127|10|2|4032|6|0/9|—|44|
|4,127|10|3|4032|35|0/9|—|738|
|4,127|10|4|4032|139|9/9|2139|40423|
|4,127|16|2|16384|55|13/15|—|1964|
|4,127|16|3|16384|272|15/15|3304|72646|
|4,127|16|4|16384|509|15/15|1504|195681|

Exact curve identities: g2p1009c12ell241259FB478; g3p503c26ell29977319FB236; g4p127c15ell134781029FB66. FirstHcanonical places as anchors; frozen original tuple seed2026111503 xor(g<<40) xor(H<<16). The g2control is unchanged and reproduces historicalH8counts and H12firstcompletion714. This is not a fresh-curve or holdout panel.

The g3H9case illustrates why gross admission is insufficient:163versus11admissions still yields zero anchor rank. At g4H10, allowing three off-anchor places also remains insufficient; four are needed in this stream. No noncompletion is replaced or discarded.

## Independently verified first-completing prefixes

A separately preregistered replay captures arithmetic at the actual first rank-completing prefix. It independently checks full/off Gaussian ranks at that prefix and the preceding signclass, requiring H-1now and less thanH-1before. Every original summary field and terminal footer compares byte-identically after filtering only the new FIRST_PREFIX lines. Replay exits0, empty stderr; REPLAY-COMPARE.txt contains0.

|g,p|H|maxLP|First class|Admissions at prefix|Full / off rank|Previous anchor rank|Elimination multiplications|Max pivot nonzeros|
|---|---:|---:|---:|---:|---|---:|---:|---:|
|2,1009|12|2|714|329|328 / 317|10|4707|14|
|3,503|14|3|1447|214|214 / 201|12|30036|33|
|4,127|10|4|2139|66|65 / 56|8|9578|27|
|4,127|16|3|3304|65|65 / 50|14|6861|23|
|4,127|16|4|1504|65|65 / 50|14|8654|23|

Thus the genus3firstcompletion does not yet require full catalogue rank235: rank214suffices in this stream. The genus4completing prefixes DOalreadyhave full catalogue rank65. A small final anchor space does not imply a small intermediate linear-algebra problem. The complete-stream cost810587forg3is not the firstcompletion cost30036; overcollection is substantial and explicitly separated.

Multiplication counts are those executed by this fixed, ordinary incremental Gaussian implementation (nonzero pivot entries only, fixed off-before-anchor order). They exclude inversions, allocation/zero scans, generation/factoring, group checks, setup, independent verification and target handling. Max pivot nonzeros is not a claim about maximum intermediate fill or optimal elimination order. Counts do not establish a wall-time ratio or an optimized2LPgraph comparison. An incomplete policy cannot serve as a finite-time speed denominator.

## Algebra and scope

For policyk, let R_kcontain the normalized admitted rows, O_kits off-anchor restriction. Then retained dimension is rank(R_k)-rank(O_k), by rank-nullity of coordinate projection on rowspace(R_k). The nested admission policies satisfy rowspace(R_2)subset rowspace(R_3)subset rowspace(R_4), so retained rank cannot decrease. Exact full-group principal identities establish each raw row. The conservative integer Hasse upper bound and selected prime-order generator give a cyclic order-ell primary component; firstHincludes the source place, hence a nonzero anchor log functional and the rank upperboundH-1. These finite checks establish real additional constraints, rather than a graph counter or witness-residual artifact.

The construction/admission mechanism is related to ordinary full-base relation matrices and prior filtering/large-prime methods; it is not a new asymptotic exponent. Relevant primary/reference boundaries are in PROTOCOL.md. The publisher harvesting article was read at abstract level only, not as a complete implementation reference.

## Decision and next technical gate

There is now a concrete finite premise for testing higher-arity polynomial relations in a complete solver at genus3/4: previous two-LP rejection loses enough useful constraints to prevent completion in these small-anchor cells. It does not support a tenfold result, and genus2has no extra residual arity to admit. Source/target descent, the support it introduces, setup, actual elimination and strongest same-target controls must all be charged. A fixed-anchor rank computation alone does not establish target recovery.

Next discriminate whether keeping source/target descent as equations over catalogue columns (rather than forcing every descent support into anchors) preserves this rank advantage in a complete supplied-target solve. Keep failed/exhausted/capped arms. Only after exact complete-solve controls should a frozen full-cost timing comparison be considered. There is no timing cohort in this study; no component/count ratio is promoted to a total gain. The mathematical tenfold goal remains active.

## Custody and errors

Original freeze4e0d00f0bc22/checkpoint20261116T044956Z; first-prefix amendment5270937158e7/checkpoint045200Z; replaybinaryfreeze8887a6af9e70/checkpoint045238Z. Each execution once, source/lock/native/binary hashes committed/checkpoint-gated beforehand. Raw RANK.jsonl contains JSON records plus a terminal text footer (not strict JSONL); it is preserved unchanged. VERIFY.txtaddsFIRST_PREFIXlines. No build, scientific identity, rank, timeout or unavailable-cell failure occurred. Dead-code warnings from imported table helpers retained in build logs. Shared dailylog is reserved by a verifying task; SESSION.mdretains this project's handoff without editing that file.
