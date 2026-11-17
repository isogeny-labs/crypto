# L6: Frobenius-stable non-subspace factor bases (n=11, 23)

Conductor task T-5736. Pre-registration: PROTOCOL.md. Data: RESULTS.json; raw/ (yields.json, n1.json, solve.json.gz, logs .gz); code: src/.
Curve: Koblitz y^2+xy=x^3+x^2+1, #E = 2·991 (n=11), 2·4196903 (n=23). Field F_2[z]/(f), with f a degree-n factor of Phi_{2n+1}, gamma=z, ONB beta_i = gamma^{2^i}+gamma^{-2^i}.
Self-tests: S3 = (x1x2+x1x3+x2x3)^2+x1x2x3+1 vanished on all sampled triples P1+P2+P3=O (0 bad). Every solver verdict that ground truth could check agreed with brute force (gt computed for bases <=300 x-values; for larger bases the subgroup-point count from the C code is reported).

## Structural facts (exact, measured)
- Trace lemma: Tr(beta_i)=1, so Tr(x)=ONB weight mod 2. The prime-order subgroup (=2E for a=1) needs Tr(x)=1. **Even-weight ONB elements are never subgroup-usable**: HAM2 at n=11 gives 66 x-values but 11 usable; at n=23, 276 x-values but 23 usable (only weight 1). Tr(gamma^k) depends on the QR class of k, so CYC_w usability is set by the QR/non-QR split of the exponent set.
- gamma-weight representations are unique for w<=2 at n=11 and n=23 (CYC2 has 253 and 1081 distinct x, no multi-rep). At n=11, CYC3 has 1771 x and the weight<=3 sets tile F (1+23+253+1771 = 2^11), so CYC3 is degenerate.
- The ONB weight-1 set lies inside CYC2: the overlap of CYC2 with HAM_r is 11 at n=11 and 23 at n=23.
- Hamming-ideal membership: C(n,r+1) squarefree degree-(r+1) generators plus field equations. CYC2 membership: C(p,3) cubics plus e2=1. msolve polynomial counts per system: CYC2 3601 (n=11) and 32549 (n=23); HAM2 363 and 3611; HAM3 17779; SUB 23-45.

## Yields (subgroup points B; columns = Frobenius orbits for stable bases, points/x for SUB)
N3' is N3 with the trivial tuples (some pair cancels, 3m^2-3m) removed. For SUB the first seed is shown; RESULTS.json has all 5.

| n | base | size | usable x | cols | N2 (ratio to null) | N2/col | N3' (ratio) | N3'/col | mean decomps/target | B·D²/N |
|---|---|---|---|---|---|---|---|---|---|---|
|11|CYC2|253|77|7|2640 (0.72)|377|547184 (0.96)|78169|23.8|3685|
|11|HAM2|66|11|1|0|0|528 (2.2)|528|1.0|10.7|
|11|SUB8|255|53|53|1098 (0.91)|20.7|119736 (0.94)|2259|11.6|13220|
|23|CYC2|1081|391|17|276 (2.42)|16.2|101016 (1.13)|5942|0.25|114|
|23|HAM2|276|23|1|0|0|1104 (structural)|1104|0|0.02|
|23|HAM3|2047|759|33|1380 (1.66)|41.8|1306032 (1.03)|39577|0.5|833|
|23|SUB10|1023|264|264|48 (1.37)|0.18|18384 (0.99)|70|0|807|
|23|SUB11|2047|528|528|264 (0.94)|0.5|296664 (1.0)|562|0.12|6453|

- Per point, every base matches the random-set null within Poisson and orbit-clumping noise. Relations come in Frobenius orbits of 23, so N2=276 at n=23 is only 12 orbits.
- The per-column advantage of the stable bases is about n, the Frobenius orbit factor that any Frobenius-stable orbit union already gets. It is not specific to the cyclotomic or Hamming description.
- The single-orbit bases (HAM1/HAM2 usable part, CYC1) give only Koblitz-endomorphism relations (tau^2 - mu·tau + 2 = 0), which are structural.
- B·D²/N >= 1 for every base with nonzero yield. With D = |B| this is an accounting device, not a non-generic signal.

## Decomposition systems (2-sum, 16 targets, timeout 120 s, skip after 2 consecutive timeouts)

| n | base | vars/pt | CMS (SAT/UNSAT, median s) | msolve (status, max F4 degree) |
|---|---|---|---|---|
|11|CYC2|23|16 SAT, 0.09|TIMEOUT x2, then skipped|
|11|HAM2|11|15 SAT 0.04 / 1 UNSAT 0.05|all finished, degree 4, ~6 s|
|11|SUB8|8|16 SAT, 0.05|degree 4, 0.66 s|
|11|SUB6|6|14 SAT / 2 UNSAT, 0.06|degree 3, 0.06 s|
|23|CYC2|47|TIMEOUT x2, skipped|TIMEOUT x2, skipped|
|23|HAM2|23|16 UNSAT, 4.1|TIMEOUT x2, skipped|
|23|HAM3|23|4 SAT (median 43 s), 1 UNSAT (89 s), 3 TIMEOUT, 8 skipped|TIMEOUT x2, skipped|
|23|SUB8 / SUB10 / SUB11|8/10/11|UNSAT 0.04 / 1.0 / 18.7 s (SUB11: 1 SAT)|degree 3 / 4 / 4; 0.06 / 1.9 / 7.9 s|

- Blockers. msolve did not finish any CYC2 instance at n=11 (46 Boolean vars, 3601 polys) or any CYC2/HAM instance at n=23 within 120 s. CMS did not finish CYC2 at n=23 (94 indicator vars plus 2209 AND-aux) within 120 s.
- So no solving degree exists for the cyclotomic system. The A2 rule (fail if the cyclotomic solving degree is not lower, or its yield per column is lower) gives **FAIL**: the degree is unmeasurable or higher (TIMEOUT against Hamming degree 4 at n=11), and per-column yield is below HAM3 at n=23 (16.2 vs 41.8 N2/col).
- KN-OPEN-5e3dd0 (does a Groebner engine resolve a non-subspace stable base before enumeration?): **no** at n=11 and n=23 for CYC2/HAM2/HAM3. Enumeration takes |B| point additions per target, under 1 ms. msolve took 6 s or timed out. Scope: 2-sum (m=2), these encodings, msolve 1 thread, CMS 1 thread, 120 s.

## N1 falsifier (F_2[Z/47], CYC2, direct convolution mod z^47-1)
- 256 targets: 64 ordered S3 solutions, all with x1, x2 on the curve. 4096 targets: 894 (the first 256 targets are shared).
- L1 (b -> e_0): 0 exact lifts, forced by a lemma. The augmentation of the lifted S3 equals 1 for weight-2 X1, X2.
- L3 (b -> e_0+J, polynomial-basis lift of x_R): 0 / 894.
- L2 (b -> e_0+J, ONB-symmetric lift of x_R): 2 / 894. Both are the pair where X1 and X2 are both symmetric (ONB weight-1) elements. They lift automatically, because the conjugate CRT component then reproduces the field identity.
- Non-trivial exact lifts: 0 / 892, so the fraction is below 3/892 ≈ 0.0034 (95%). **N1 is dead as predicted.**

## Predictions check
P1: holds per point. P2: holds; CYC is worse than predicted (msolve times out already at n=11). P3: FAIL as predicted. P4: holds.
Deviation from PROTOCOL: CYC3 at n=23 (16215 x) and its matched SUB14 were dropped from the yield run. The O(|B|^2) N2 loop over ~8k points times 6 bases ran past 45 min (with the n=11 bases), against the 20-CPU-minute rule, so the run was stopped and restarted without them. n=131 / mu_263 not executed.
Classification: relabelling / engineering at best. No advance, no non-generic signal, novelty_unverified (no literature search done here). No security claims.
