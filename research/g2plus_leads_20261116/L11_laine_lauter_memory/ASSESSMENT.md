# L11 assessment: memory-bounded Diem 2LP on plane quartics (time-memory curve)

Assessor: independent of the producer. Date: 2026-11-16. All numbers are
`[pcc:unverified]`: no pcc claim exists for them. This is not a benchmark. It adds
no leaderboard row, no `benchmarks/` row and no speed claim.

## Verdict

**INCONCLUSIVE**, from the frozen rule in PROTOCOL §8.

- No arm meets SUPPORTED. That needs `r_T_hi < 2.0` and `r_M_hi <= 0.5` at both
  k = 12 and k = 14. The best candidates miss it:
  - G: r_T 2.104 [2.04, 2.16] at k=12 and 2.366 [2.34, 2.39] at k=14; r_M_hi at
    k=12 is 0.580.
  - AS2: r_T 2.204 / 2.199, with r_T_hi 2.254 / 2.224.
- KILLED is also not met. At k=14 the only cell with r_M <= 0.5 is G (0.426), and
  its r_T lower bound is 2.34, not above 4.0.
- The verdict does not depend on how the G4 exclusions are read. I re-ran the rule
  with the excluded arms (AF2, AF4, AF8, AS4, AS8) put back in, and it is still
  INCONCLUSIVE:
  - AF2 has r_M_hi 0.5745 at k=12, so it cannot support.
  - AF4 at k=12 is exhausted, and its supply bound is 3.01 T_U, below 4 T_U, so it
    cannot kill.
  - AS4 and AS8 have r_T above 4, so they cannot support.

Secondary verdicts:

- **H_C4 is rejected (outside the window).** The elasticity of time in memory at
  k=14 (arms U, AS2, AS4, AS8) is -1.747 [-1.757, -1.738], against a window of
  [-1.3, -0.7]. Halving memory with a static admissible set costs more than 2x the
  time, which is worse than the -1 derived in C4.
- **H_C5 is supported, on point estimates.** G's memory exponent is 0.747 against
  U's 0.920: the difference is -0.173 [-0.178, -0.167], and the rule needed -0.10
  or lower. G's time exponent is 0.989 against U's 0.883: the difference is +0.105
  [0.096, 0.115], and the rule allowed +0.15 or less.
- The measured curve for G over k = 8, 10, 12, 14:
  - r_T = 1.55, 1.76, 2.10, 2.37
  - r_M = 0.87, 0.72, 0.57, 0.43
  - Memory falls by a power of p, and the time factor grows with p. This matches
    the `log p` time factor that C5 predicted, but the size is larger: C5
    predicted 1.6 / 1.9 at k = 12 / 14, and the run measured 2.10 / 2.37.

## Audit

1. **Freeze.** `sha256sum -c frozen.sha256` passes for PROTOCOL.md and SPEC.md.
   All 624 run lines (13 arms x 12 curves x 4 k) carry `l11_sha256` = sha256 of
   the current `l11.py` (b3f2f68e…). `raw/SHA256SUMS` matches every file except
   `raw/COMMANDS.log`, which was appended after the hashes were taken. That is
   bookkeeping only.
2. **Counters, checked against the census for every run** (own script, all 624
   lines):
   - `inv_lines = 2 * split`.
   - `mul_lines` lies within [split(29+S) + nosplit(16+S) + 6 tangent,
     … + 15 tangent], as SPEC §7 specifies.
   - `mul_setup = 82 b`.
   - `FPM_total = mul + 10 inv`.
   - tangent + nosplit + split = T.
   - full + lp1 + lp2 = split.
   - `mem_total = V + E + Fr + D + b`.
   - Every exhausted run has T = supply.
   - `cycle_rows = count - Fr` (G3).
   - Every run has 0 BAD lines and 0 G1 failures.

   There were 0 violations. The unit is the same for every arm: one `Fp` class
   and one line routine.
3. **Independent re-implementation.** I wrote my own code with:
   - brute-force line sections (all curve points on the line through Pi and Pj);
   - tangency from the gradient;
   - my own DSU;
   - G admission tested literally, as "endpoint in the component of 1", rather
     than the producer's `in vid`.

   It reproduces T, count, Fr, E, V, D, mem_total and the tangent / nosplit /
   split census exactly on 9 runs: U on (k=8, curves 0 and 1) and (k=10, curve 0);
   G on (k=8, curves 2 and 3) and (k=10, curves 0 and 5); AF2 on (k=8, curve 4)
   and (k=10, curve 1). This confirms three things:
   - the production line routine;
   - the stop rule;
   - that `R in vid` is equivalent to root-component membership. Every node G
     creates is attached to "1".
4. **G2 Pic check.** I rebuilt it independently by brute force for the Fermat
   quartic at p=103: N1 = 104, 420 distinct split lines, rank 104, lattice
   index 1124864 = 104^3. This matches the producer's gate. The relations
   generate the full degree-0 class group, so the relation semantics are right.
5. **Headline numbers re-derived from raw/runs_k*.jsonl** with my own bootstrap
   (different RNG, 4000 resamples):

   | cell | r_T | 95% interval | r_M | 95% interval |
   | --- | --- | --- | --- | --- |
   | G, k=14 | 2.366 | [2.340, 2.391] | 0.426 | [0.423, 0.428] |
   | AS2, k=14 | 2.199 | [2.177, 2.223] | 0.573 | |
   | AS8, k=14 | 9.400 | | | |
   | UAA, k=14 | 1.006 | | | |

   The exponents (U T 0.883, U mem 0.920, G T 0.989, G mem 0.747, AS2 mem 0.850)
   all agree with `analysis.json`. Spread over the 12 curves is narrow:
   - G at k=14: r_T ranges 2.277 to 2.448; r_M has a maximum of 0.433.
   - AS2 at k=14: r_T ranges 2.137 to 2.297.
6. **Is the baseline fair?** Yes.
   - U, AF and G share the same curves, base, pair order and seeds (common random
     numbers), and AF1 reproduces U bit for bit (G5).
   - U is not needlessly expensive. U2 (c=2) costs r_T 0.989 at k=14, so c=3 sits
     at about the time optimum.
   - U counts only the cycles of the root component, as G does. U's memory
     includes vertices and edges in components other than the root's that never
     contribute. That is the definition of the unbounded arm, so it is not padding.
   - The replay is byte-identical, 69 of 70 files. The only difference is a
     wall-time line in the unit-test log. The replay uses the same code, so it
     checks determinism, not correctness; point 3 covers correctness.
7. **Can a bug produce the effect?** Points 2 to 5 rule out the obvious paths:
   - counter drift;
   - wrong residuals;
   - a stop-rule miscount;
   - G admitting too much or too little.

   The memory reduction in G is a structural consequence of discarding edges that
   start a new component.

## Concerns

- **G4 excluded 7 of 13 arms**, mostly by design and not by bugs:
  - k=8 is too small. At b=49 the supply is only 1176 pairs, so U2, AF2, AF4 and
    AF8 exhaust at k=8. AF4 and AF8 also exhaust at every k.
  - The non-exhausted arms AS4, AS8 and S have rank deficits of 1 to 5 at
    k = 8 and 10. Their stop rule fires on `count >= b + 8 + b/100` while the rank
    is still below b-1. For those arms (AS8 at k=8 has b/N1 about 0.5), the margin
    of 8 + b/100 is too small, so T understates a rank-complete stop.
  - None of this changes the verdict (see above).
- **H_C4 regression.** H_C4 fits a straight line through 4 arms, and those arms
  differ in b. Two of the four (AS4, AS8) are G4-excluded. The interval is narrow
  because the ratios are paired, but it does not include model misfit. AS8's time
  exponent is 1.237, so the curve steepens with p.
- **Scope of the cost.** Only relation collection is costed. Linear algebra and
  point enumeration are excluded for every arm. G lowers the Wiedemann proxy
  `b*nnz` (2.22e6 against 4.38e6 for U at k=14); the AS arms raise it (AS8 is
  1.01e7). Unpriced counters (dict lookups, find steps) are reported but not
  charged.
- **Primes are small** (263 to 16411), and the protocol allows no asymptotic claim.
  G's r_T rises by about 0.25 to 0.35 per k step of 2. That is consistent with the
  log-p factor of C5, so under this rule the SUPPORTED condition would get harder,
  not easier, at larger p. This is an observation, not a claim.
- **The Laine-Lauter paper was not read** (the host did not resolve). The arms are
  derived here, so the result is not a test of the paper's algorithm.
- **research-kb was not available** to the producer. Prior art was checked by grep
  only.

## Distance to matched rho

**Not applicable as an improvement.**

- Every memory-bounded arm costs more relation-phase FPM than the unbounded
  baseline U:
  - G: 2.04x at k=12 and 2.33x at k=14;
  - AS2: 2.16x at k=12 and 2.17x at k=14.
- In exchange, memory falls to 0.43 to 0.61 of U's.
- So on the lab's time metric these arms widen the IC-to-rho gap by about a factor
  of 2. They do not narrow it.
- No improvement over the index-calculus time baseline was measured. This lead has
  no matched rho reference and does not touch the leaderboard.

## Next step

None for the time goal. Bounding memory never reduced time here, and memory is not
the lab's bottleneck at these sizes.

If memory ever becomes binding, the single most valuable cheap follow-up is a new
frozen protocol for G with a larger base (c = 4 or 6). C5 gives
T_G/p ~ 0.25 ln p + ln(2/sqrt c), so a larger c is the one lever that could pull
G's r_T below 2 while it keeps its p^(-0.17) memory exponent. The same protocol
should raise the stop margin so that the rank gate holds for large-b arms.

## Files

- Lead: `/home/user/crypto/research/g2plus_leads_20261116/L11_laine_lauter_memory/`
  (PROTOCOL.md, SPEC.md, DEVIATIONS.md, RESULTS.md, raw/, replay/REPLAY.md).
- Assessor scratch scripts (not in the repository):
  - `/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/l11/a1.py`
    (counter identities, ratios, exponents, G4 detail)
  - `/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/l11/indep.py`
    (brute-force re-implementation of U, G and AF2)
  - `/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/l11/fermat.py`
    (independent Fermat Pic index)
