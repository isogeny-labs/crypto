# RESULT: LEAD-TRF-padic-canonical-section

## Outcome
- status: ran; exit code 0; wall 9 s (run.status: exit=0 wall=9; run.py wall_s 8.84). run.stderr empty.
- Single run, seed 20261110, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): (a) all n exact property passes on both lift curves; (b) psi(N s(.)) nonzero count 0; (c) hits < h* (smallest h with P(Bin(T,1/p)>=h) <= 1e-4) at each of 3 sizes. Falsified on any (a)/(b) failure or (c) excess.

Measured:
- (a) p=4583, 17807, 81097 (N=4463, 17761, 80809, all prime, != p): 150/150 on-curve, reduction, N*s=O exact, additivity on both lift curves (A/A identical).
- (b) psi(N s) nonzero: 0 at all three sizes.
- (c) hits / expected / h*: 6 / 4.36 / 15; 1 / 1.12 / 8; 0 / 0.25 / 5. All below h*. Degenerate trials skipped: 3, 2, 0.
- Controls: null (random lifts fail additivity) rate 1.0 (>= 0.9) on every arm, passed. Positive control (anomalous N=p=2161): 500/500 recovered, passed. A/A: exact results identical on lift curves 0 and 1; hit halves A/B (3,3), (1,0), (0,0), all < h*, passed.
- Verdict: consistent_with_prediction (matches run.py "verdict"; "reasons" empty). Controls_ok = true.

## Reading
- The toy tests what it states: H1 (homomorphic prime-to-p torsion section with N*s=O exactly), H2 (Smart functional vanishes on it), H3 (random-lift hit rate at chance 1/p). Hit counts are consistent with the binomial expectation.
- No discrepancy between run.py's verdict and the numbers. Hit counts are tiny (expected <= 4.4), so H3 has low power at the chance rate; the 1e-4 threshold only rules out gross excess.
- H1/H2 are structural (the section is multiplication by 1 - N c, killed by the log), so passing is expected and is a check of the implementation and the recorded obstruction rather than a surprise.
- Limits per PROTOCOL.md s6: Z/p^2 not Z_p, p <= 2^16, only the prime-to-p section and uniform random lifts; nothing about structured non-homomorphic sections or p | N other than the control.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the canonical section is homomorphic and Smart's functional is identically zero on it, while random-lift sections give only chance recovery, matching the recorded closed_obstructed disposition.

## Standing
[pcc:unverified], single local toy run, no independent replay.
