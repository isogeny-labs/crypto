# Guarded delayed polynomial S3: cold comparison

[Report](report.pdf), [full native summary](summary.json),
[flow/custody](local-evidence.json), [frozen runtime plan](runtime-plan.json).
Source 73efa9eb5, Amendment12. All 810 executions complete in 19.901s:
135 warmups, 675 measured, correct scalars and exact cross-oracle relation
streams. Pseudo/delayed semantic products agree; no retries. [pcc:unverified]

The direct combined delayed/prepared comparison has 8 controlled cases, all 8
upper intervals below 1, cost ratios 0.813–0.931. One case is rejected.
The isolated delayed/pseudo ablation has 8 controlled cases: 2 clear gains
(p523/s157: 0.925, CI 0.863–0.983; p1039/s167: 0.844, CI 0.653–0.982), 6 inconclusive,
1 rejected. Fresh pseudo/prepared control has 9 controlled cases, 5 gains and
4 inconclusive. All 27 direct comparisons retained, no cross-run composition.
[pcc:unverified]

All 27 individual strata recover correctly; 26 pass controls. Delayed p127/s163
is the sole timing rejection. No local-reference parity flag. All 9 diagnostic
targets finish at first repeated decomposable abscissa, consistent with
generic coalescence; no independent density/exponent claim. [pcc:unverified]

Raw/copied binaries/checkpoint/host provenance remain under
/workspace/ic-local-runs/local-delayed-pseudo-pair-20261116-17 with hashes.
The full 49826-byte native summary is copied byte-exact. A plot label was
shortened after timing; regenerated numerical summary remains byte-exact.
The original copied analyzer and original figures remain in local custody.
All six final PDF pages were visually inspected. Intervals are repeated
fixed-curve timing; cold wall is primary and multiplication accounting is
incomplete. No signed independent verdict, strongest reference, standard lab
admission, canonical encrypted-schema validation or admitted index/leaderboard
row. [pcc:unverified]
