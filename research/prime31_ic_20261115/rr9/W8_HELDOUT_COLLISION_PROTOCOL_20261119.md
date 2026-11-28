# W8 held-out original Prime31 signed-weight certificate protocol
Date: 2026-11-19 UTC. Freeze before source creation or runner observation. PR #4807. [pcc:unverified]

## Hypotheses and scope
H1: the previous power-of-two W1 half-table deficit 65536-65496=40 is caused by exactly reproducible elliptic-group collisions between **distinct integer signatures**, unlike the old weights' 799 signature losses.
H2: on unseen uniformly distributed public points, the power-of-two schedule [1,2,4,8,16,32,64,128] gives more weighted-eight-slot *generic-graded* hits than the previously used [1,3,5,7,11,13,17,19], but effect size is unknown. Null is no difference. Even a positive result does not imply faster algebraic extraction.
H3: changing to power-of-two weights may make F4 elimination *harder* even if generic-graded yield improves, so no solver-speed promotion absent matched nonenumerative extraction.

## Frozen inputs
- **Curve fixed:** p=2147483647, E: y²=x³+847514846 x+2087444724; n=2147446831; cofactor 1; G=(625037715,2100627251).
- **Original arithmetic checkout:** `1df714645aa683cf975bc76fcacb9842cb4780fd`; reuse `prime31-algebraic-ic`, no reimplementation of elliptic operations in the experimental Rust candidate.
- **Factor base:** canonical first 31 rational x with y=min(root,p-root), four eight-point windows [0..7], [8..15], [16..23], [23..30].
- **Held-out deterministic target seed:** SplitMix64 state `0x48454c444f555431`, different from #4805 `0x5758435349474e31`. Use rejection sampling to draw k mod n (nonzero), construct Q=[k]G, **discard the scalar** before grading. First 256 targets assigned to W0, next 256 W1, next 256 W2, next 256 W3: 1024 paired public points.
- **Word count:** 16^4=65,536 per half, 16^8 ordered words per whole eight-position relation. Witness/word checking is done out-of-band by an explicit *generic* EC-addition oracle, not an unknown-index algebraic extractor.
- **Collision proof:** in W1 only enumerate both power-of-two 4-slot halves, map each exact EC sum to a first word, and collect every later matching word. For each mismatch of exact signed `Z^8` coefficient vectors, record first and later words plus all vectors; test EC sum equality by independent replay. Report counts even if they differ from expected 40; do not silently change inputs.
- **Order control:** old/new order alternating by window (0 and 2 old-first, 1 and 3 power-two-first); output arm-specific times and addition counts as diagnostic, **not** an ABBA whole-solver comparison.
- **Primary measured outcome:** paired hit counts (both, old-only, new-only, neither), per-window hit counts, collision cardinalities. Descriptive McNemar exact p and 95% paired difference CI may be computed afterwards but not used as formal security inference. Preserve raw JSON and source commit SHA.

## Strict boundaries and decision rules
- Candidate algorithm never receives k, witness nibble, generic half-table or grader internals. This specific study is a *grading control*, so **no candidate extractor is actually implemented**.
- No same-curve full ECDLP speedup; no alpha < 1 complexity claim; no promotion to a whole-solver scoreboard.
- Independently verify at least one non-formal collision using affine arithmetic that is separately authored from the Rust crate.
- If old arm does better held-out, preserve the negative result and reject weight promotion. If candidate does better, send weighted x lists to existing #4692 F4 owner, then require public-only 46+/18- corpus and ranked 1/64/1024 complete solves before any advantage claim.
- Archive CI outcomes, mark failed steps and differences explicitly. Never count generic grading as non-generic extraction.

## Literature / previous work checked (novelty audit is bounded)
- GitHub #4692 (RR9/F4 owner), #4728 (four-window rank31/1024 Rust generic control), #4805 (prior 256-point pilot), #4803 (unweighted cancellation ceiling), #4774 (full-cost control), #4793 (extractor budget), #4532 (optimized BSGS). #4722 (M6 weight ceiling).
- `crypto-autoresearcher` SA-04/SA-18 negative algebraic experiments as reported in #4805. Research KB MCP and wrapper unavailable; no novelty-by-absence claim.
- Petit–Kosters–Messeng, PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, FFA 2018 DOI 10.1016/j.ffa.2018.01.009; Kudo–Yokota–Takahashi–Yasuda, *Acceleration of index calculus for solving ECDLP over prime fields and its limitation*, 2018. These highlight prime-field polynomial solving limitations; the weight census is not a novel ECDLP algorithm.
