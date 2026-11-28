# Fast group law for the genus-3/4 rho reference (SMOKE, 2026-11-16)

All numbers are `[pcc:unverified]` smoke results from one shared, loaded host. Not a verdict, no frozen protocol.

## What was built
`src/cryptanalysis/genus_ladder_fastgroup.rs` (module line added to `mod.rs`): `FastJ`, genus 3 and 4, deg f = 2g+1, h = 0 (monic or not).
Same `JDiv` type, results equal to `JCurve::cantor`.
Algorithm (generic case deg u1 = deg u2 = g, coprime): Cantor with the first reduction folded into the composition (Lange/Nagao style).
- s = (v2-v1)/u1 mod u2 (doubling: s = k/(2v) mod u, k = (f-v^2)/u) by a g x g linear system: columns x^j u1 mod u2, division-free Gaussian elimination, projective back substitution giving s~ = r*s, no inversion.
- u' by an exact top-down quotient needing only the coefficients of degree >= g (k contributes only its top two); Karatsuba for the g x g products (6 / 9 mults for g = 3 / 4).
- Inversion 1 of r*sigma (sigma = s~_{g-1}) yields 1/r and 1/sigma^2. Second reduction u'' = (f - v'^2)/u'; at g = 4 its leading coefficient -v'_5^2 needs inversion 2 (none at g = 3: it is f's leading coefficient).
- `add_batch` runs W additions in lock step; each inversion stage is one Montgomery simultaneous inversion (1 inv + 3(W-1) mults).
- Degenerate inputs (identity operand, deg < g, common root, a = -b, singular system, sigma = 0, v'_5 = 0) fall back to `JCurve::cantor`, tallied separately (`fallback_*`). Their frequency at p = 8209, g = 4 was about 0.13 % (4/3000 single, 11/6400 batched W=16).
So "verified on degenerate cases" means the fallback is exercised and equal by construction; the fast path itself is verified only where it is taken.
Tallies: every `F_p` multiplication goes through `mm` (counted as executed), inversions through `inverse`; additions/subtractions are not counted (as in `JCurve`). Squarings count as mults.

## Tests (all pass in the real crate, release, `-- --include-ignored`)
`kmul_matches_schoolbook` (also asserts 1/3/6/9 mults), `small_primes_all_cases_g3_g4` (p = 7, 11, 13, 17: every ordered pair of a pool with identity, points, partial sums, negatives; single and batched vs `cantor`; associativity and a + (-a) = 0), `medium_primes_g3_g4` (p = 101, 103), `large_prime_random_and_exact_counts` (p = 8209, 32749, g = 3, 4; 1500 random add + double each; fast path >= 99 %; cost per fast add/double is constant), `non_monic_f_is_handled`, plus the ignored `fastgroup_smoke_bench`.
Caveat: at p = 7..17 almost every pair is degenerate, so those tests mostly exercise the fallback; the fast path is exercised at p >= 101.

## Smoke numbers (p = 8209, per generic addition, best of 5, loaded host: load ~8 on 4 vCPUs)
| | mul | inv | ns |
|---|---|---|---|
| g=4 Cantor | 393.1 | 7.0 | 1938 |
| g=4 fast, W=1 | 180.1 (double 206) | 2 | 620 |
| g=4 fast, W=16 | 185.7 | 0.125 | 724 |
| g=4 fast, W=64 | 186.0 | 0.031 | 604 |
| g=3 Cantor | 219.0 | 5.0 | 1161 |
| g=3 fast, W=1 | 97.0 (double 114) | 1 | 384 |
| g=3 fast, W=16 | 99.8 | 0.063 | 407 |
| g=3 fast, W=64 | 100.0 | 0.016 | 397 |
Euclid inversion costs 9.3 mulmods in time here (43.6 ns vs 4.68 ns). So with that price a g=4 add is 180+19 = 199 mult-equivalents vs Cantor 393+65 = 458 (2.3x), 4.7x less time unbatched at g = 4 (3.0x at g = 3, time ratios from a loaded host). Batching trades 2 inversions (~19 mult-equivalents) for ~6 mults and, here, gives no time gain (inversion is cheap at p < 2^31; batch bookkeeping costs about as much). The Montgomery gain would matter only if inversions were costlier.
Not reached: published genus-3/4 counts are lower than this module's (I recall about 1I + 70M-class for genus 3 but could not verify); the elimination (about 70 of the 180 mults at g=4) is the obvious place to improve, e.g. by exploiting the structure of the matrix or Euclid-based inverse in projective form.

## Projection (arithmetic only; `projection.py`, `projection.md/json`)
Counts from `genus4_rho_refs_20261116/run.jsonl` unchanged, only the group-op price changes (fast adds priced as additions; doubles not separated). The frozen Qcal is reproduced: 0.008 [0.006, 0.012].
Qcal, arm w / cheapest rho, timed ratio (fast ns / Cantor ns from this smoke run), rho native work priced 0 as frozen:
- frozen: 0.008 [0.006, 0.012]
- rho fast W=1, IC still Cantor (the literal "rho uses the faster law"): 0.024 [0.017, 0.037]
- fast law in both arms W=1: 0.017 [0.013, 0.027]; rho W=16: 0.015; rho W=64: 0.018.
So the ratio rises about 2-3x (still far below 1); the Qcal margin is not near closing. Variant with rho's ~22 native ops/step priced at one add each barely changes it (0.023).
Field-multiplication reading (kappa = inv/mul = 9.3): F1 (mults only) 0.005 -> 0.012 (rho-only fast) / 0.008 (both); F3 alpha=1 (all native ops = 1 mult) 0.009 -> 0.019 / 0.015; F3 alpha=0.35 0.006 -> 0.014 / 0.011. Full table incl. kappa = 0 and 30 in `projection.md`.
Reading caveats: nominal-w readings Q1-Q4 (w in group-add equivalents) are unchanged by a law change by construction; only Qcal and a field-mult reading move. Fast-law ns come from a loaded host and differ from the study's calibration (Cantor 1938 ns now vs about 1680 ns calibrated); the ratio uses same-bench numbers. IC's own group ops could also be batched; not modelled. Doubling cost not separated.

## Open issues
- Re-time on a quiet/isolated host (isolab-eks) before any quotation; best-of-5 on a loaded host is only indicative.
- Papers (Cantor 1987, Harley, Lange 2005, Nagao 2000, Pelzl-Wollinger-Guajardo-Paar 2003, Montgomery 1987) were not consulted; the algorithm is derived here and verified by tests, not copied; no published operation count was checked.
- g = 2 and g = 5 unsupported (g = 5 needs a third reduction step); deg < g inputs delegated to Cantor.
- No `rho` driver wiring (`run_rho_walk` still uses Cantor); no commit/PR made (per instructions).
