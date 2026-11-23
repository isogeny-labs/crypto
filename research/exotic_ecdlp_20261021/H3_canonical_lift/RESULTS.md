# H3 results: canonical-lift p-adic digits (C3)

Job: `exotic-h3-canonical-lift-r4` (final: -r5, see below) on isolab (4 cpu, 8Gi). Earlier attempts: the original job and `-r1` failed on harness bugs (empty program, 8KB arg limit; `-r2` ran but my code had a generator-lift bug, `-r3` diagnosed it). Raw output: `out/result.json`; program: `run.py`. Seed 20261022. Total in-program runtime 11 s, no cell censored.

## Lift method actually used: exact canonical lift (not the Teichmuller fallback)
Satoh's `Phi_p(J,J)=0` iteration is degenerate over a prime field (`Phi_p(X,X) = (X^p-X)^2 mod p`), so the canonical `j~` is computed as the p-adic Hensel root, congruent to `j(E)` mod p, of the Hilbert class polynomial `H_d` (`d = disc End(E)`, PARI `polclass`), Newton to `p^4`. The lifted curve is `y^2 = x^3 + a x + B~` with the same `a` as `E` and `B~` solving `j(a,B~) = j~`. The Frobenius-fixed lift of `P` is the unique lift of order dividing `#E(F_p)` (prime to p). It is computed as `G~ = [m]G~_0` (m = 1 mod n, 0 mod p^3), done in Q_p arithmetic for the single generator, then the whole cyclic subgroup `<G~>` is tabulated mod `p^3`. Digits are of `x(P~)` in this model. Curves are random ordinary `E/F_p` (class number h <= 3000 cap, h actually 56 / 120 / 259).

## Per-p results (all pairs/sets drawn from the full cyclic subgroup `<G>`)
| bits | p | n=#E | trace | End disc d (h) | min Delta(L) | argmin L | 1-1/p | kill thr 1-1/sqrt(p) | #L below thr | MI (nats) | MI null mean +- sd | MI z | closure obs / base | closure z |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 4099 | 4102 | -2 | -16392 (56) | 0.99946 | (1,2083) | 0.99976 | 0.98438 | 0 | 0.55355 | 0.55536 +- 0.00859 | -0.21 | 0.0200 / 0.01219 (ratio 1.64) | +2.46 |
| 16 | 65537 | 65480 | 58 | -258784 (120) | 0.99990 | (1,58849) | 0.99998 | 0.99609 | 0 | 1.37749 | 1.37354 +- 0.00307 | +1.29 | 0.003599 / 0.003757 (0.958) | -0.45 |
| 20 | 1048583 | 1050328 | -1744 | -1152796 (259) | 0.99996 | (1,169579) | 0.999999 | 0.99902 | 0 | 3.92177 | 3.92136 +- 0.00093 | +0.43 | 0.000876 / 0.000895 (0.978) | -0.43 |

Setup: M = 50000 random pairs (k,l), P~+Q~ read from the table at (k+l) mod ord; 4000 directions in P^1(F_p) (functionals `alpha*x1 + beta*x2`); MI with sqrt(p) bins on both x1 and k, S = min(ord-1, 20000) random points, 200 permutations; closure: argmin-L sublevel set `{L < sqrt(p)}` over the full subgroup, all pairs k<l inside it (set sizes 50 / 246 / 940), base rate = set fraction. `min_delta` at the largest finished p (2^20) = **0.99996**. The `min Delta` values are exactly the null: the "1-Delta" counts (27, 5, 2 pairs out of 50000) are the chance coincidences expected for ~4000 directions at probability 1/p. Axes alone: Delta(x1) = 0.9998 / 0.99996 / 1.0, Delta(x2) = 0.9997 / 1.0 / 1.0, Delta(x0) = 0.9998 / 1.0 / 1.0. Secondary MI on x2: z = -1.32 / -1.41 / -0.60.

## Validation (all true in every cell)
- `H_d(J~) = 0 mod p^3`; `J~ = j(E) mod p`; `j(a,B~) = J~` mod p^3; `H_d` splits completely mod p (56 / 120 / 259 roots = h); `n = p+1-t`.
- Frobenius-fixed (torsion) lift verified: `ord*G~ = O` mod p^3 and `T[a]+T[b] = T[(a+b) mod ord]` for 400 random pairs per p (about 195 of them wrapping past `ord`), all ok; points reduce to F_p arithmetic and lie on the curve. Control: the raw (non-torsion) lift of G fails the order test, so the test is non-trivial.
- Positive controls for the pipeline: a planted additive digit functional gives Delta = 0 (detected); a planted k-dependent digit gives MI far above the null (3.48 vs 0.56, 4.85 vs 1.37, 6.24 vs 3.92).
- Caveat: `#E~(Z/p^k) = n p^(2(k-1))` holds for every lift, so it is not a check of canonicity; the canonicity evidence is `H_d(J~) = 0`.

## Closure re-test with a permutation null (r5, pre-committed by coordinator)
Job `exotic-h3-canonical-lift-r5` (same exact canonical lift, same seed; only the closure test changed). Null: 500 random relabelings of which points are "in" the set (same size), closure statistic recomputed on the same pair structure. All other numbers are identical to r4 (min_delta, MI, validation).

| bits | set size | observed closure | perm null mean +- sd | corrected z | upper-tail p (500 perms) | binomial z (old) |
|---|---|---|---|---|---|---|
| 12 | 50 | 0.02000 | 0.01181 +- 0.00312 | **+2.63** | 0.0100 | +2.46 |
| 16 | 246 | 0.003599 | 0.003730 +- 0.000351 | -0.37 | 0.627 | -0.45 |
| 20 | 940 | 0.000876 | 0.000896 +- 0.0000446 | -0.46 | 0.681 | -0.43 |

The permutation null does NOT dissolve the p=2^12 blip: the corrected z is +2.63 (p = 0.010), slightly larger than the binomial z, not inside the 2 sd band. It remains small-sample (24 hits on 1200 pairs from 50 points) and is one of three closure tests plus MI tests (a Bonferroni factor of 3 over the closure family gives about 0.03), but I am not claiming it is explained away.

## r6: multi-curve replicate at p=2^12, extra point p=2^14 (job `exotic-h3-canonical-lift-r6`, all cells reported, no selection)
Curves at 2^12 use seeds base+12+1000*rep, rep 0..4 (rep 0 is the r4/r5 curve). Same exact canonical lift and permutation-null closure (500 relabelings). Note: because the generator choice depends on Sage's global RNG state, the 2^16 and 2^20 cells used a different generator than r4/r5, so their numbers differ slightly from the tables above; the r6 values supersede them.

| cell | p | n | trace | End d (h) | min Delta | MI z | set | obs closure | perm null mean +- sd | **perm z** | p (upper) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 rep0 | 4099 | 4102 | -2 | -16392 (56) | 0.99946 | -0.21 | 50 | 0.0200 | 0.01181 +- 0.00312 | **+2.63** | 0.010 |
| 12 rep1 | 4099 | 4011 | 89 | -8475 (24) | 0.99946 | -0.44 | 52 | 0.0277 | 0.01254 +- 0.00300 | **+5.05** | 0.002 |
| 12 rep2 | 4099 | 4197 | -97 | -6987 (12) | 0.99950 | -0.95 | 88 | 0.0180 | 0.02066 +- 0.00230 | -1.17 | 0.876 |
| 12 rep3 | 4099 | 4113 | -13 | -16227 (24) | 0.99944 | -1.51 | 64 | 0.0202 | 0.01491 +- 0.00252 | **+2.08** | 0.030 |
| 12 rep4 | 4099 | 4075 | 25 | -15771 (32) | 0.99946 | -0.29 | 42 | 0.0000 | 0.00958 +- 0.00330 | **-2.90** | 1.000 |
| 14 | 16411 | 16464 | -52 | -62940 (76) | 0.99978 | -1.72 | 130 | 0.0101 | 0.00776 +- 0.00100 | +2.33 | 0.012 |
| 16 | 65537 | 65480 | 58 | -258784 (120) | 0.99988 | +1.44 | 226 | 0.00221 | 0.00340 +- 0.00038 | **-3.14** | 1.000 |
| 20 | 1048583 | 1050328 | -1744 | -1152796 (259) | 0.99996 | +0.29 | 968 | 0.00092 | 0.00092 +- 0.00005 | +0.01 | 0.479 |

p=2^12 over 5 curves: perm z = +2.63, +5.05, -1.17, +2.08, -2.90; mean +1.14, sd 3.16; 3 of 5 exceed +2 (the pre-committed reproduction criterion), 1 is below -2. Validation flags true in every cell; `min_delta` (0.9994 to 0.99996, zero directions below the 1-1/sqrt(p) threshold) and MI (|z| <= 1.72) are null in every cell. Top-level `min_delta` = 0.99996 (p=2^20).

## Final verdict: **KILL (Scale criterion)**, as directed by the coordinator
Wording: additive-defect and MI are null at all p; closure shows no excess at p >= 2^16 (a significant deficit at 2^16, z = -3.14, and base rate at 2^20, z = +0.01); the p=2^12 excess reproduces by the pre-committed count (3 of 5 curves) so it is case (b), a finite-size effect, but it is heterogeneous (z sd 3.2, one curve at -2.90, none systematic in sign) and it does not scale: 2^14 is +2.33, then 2^16 and 2^20 show none. No closure structure survives to cryptographic scale.

Honest qualifications, stated rather than hidden: (1) the script's own strict rule returned "inconclusive" (|closure z| > 2 in several cells); `result.json` keeps `verdict_script_output` and `strict_all_cells_kill = false`, and the Kill is the coordinator's Scale-criterion decision applied after the run. (2) "closure at base rate for p >= 2^16" is not literally true at 2^16 (deficit, z = -3.14). (3) A z sd of 3.2 across curves at 2^12 means the permutation null (random subsets) is miscalibrated for these small sublevel sets (sums of a structured subset of a cyclic group are not exchangeable with random subsets); the 2^12 excesses are small-set finite-size behaviour, not evidence of a size function. (4) One curve per p above 2^12.

Jobs: original and -r1 harness failures; -r2/-r3 generator-lift bug; -r4 first full run; -r5 permutation-null closure; -r6 replicate + 2^14 (final). Censored cells: none.
