# Lead 3: Weil descent at field degree n = 3 (INTERIM note; usage limit reached before the p = 8209 rows and the PDF)

Status: **partial**. Data for p = 271..4099 (arm P), 271..2083 (arms M, J) is committed under `data/`; the p = 8209 run was
still in flight, and the PDF/SVG, `benchmarks/` row and the final fit including p = 8209 are NOT done. Nothing pushed.
This is a replication plus range extension of `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` section 11 (esp. 11.7, 11.9, 11.15, 11.19),
which already ran this instrument (`gaudry_cubic`) at p <= 2083.

## 1. Literature (step 1)
- Gaudry-Diem index calculus on E(F_{q^n}): factor base {P : x(P) in F_q} (~q/2 points), relations R = +-P_1 +- ... +- P_n via the Weil-restricted
  Semaev S_{n+1} (n equations in n unknowns over F_q, Bezout 2^{n(n-1)}); success probability ~1/n! per random R; constants n! * poly(2^{O(n^2)}).
  Full base: relation phase ~q, sparse LA ~q^2 (worse than rho q^{n/2}). With Gaudry's large-prime rebalancing the stated cost is
  O~(q^{2-2/n}) (Joux-Vitse, Section 2.1: "n! c(n,q) q^{2-2/n}"; they say Gaudry suggests large primes). For n = 3: q^{4/3} = n^{4/9} versus rho q^{3/2} = n^{1/2}: **beats rho in the exponent**, heuristically, with constants huge.
  Sources read: Joux-Vitse, "Elliptic Curve Discrete Logarithm Problem over Small Degree Extension Fields" (J. Cryptology 2013), text fetched; Thome HDR (genus-g analogue: balanced base q^{2-2/(g+1)}, one large prime q^{2-2/(g+1/2)}, double large primes q^{2-2/g}) -- these intermediate exponents are the hyperelliptic analogue, not stated for E(F_{q^3}).
  Diem, "On the discrete logarithm problem in class groups of curves", Math. Comp. 80 (2011), Thm 3 (per a search snippet; full text not retrieved): O~(q^{2-2/n}) for E(F_{q^n}). Gaudry's JSC 2009 paper itself (HAL inria-00337631) was behind bot protection; cited via the above, unverified first-hand.
- Joux-Vitse m = n-1 variant: q^2, **worse than rho for n <= 4** (their text). So at n = 3 only m = 3 with double large primes beats rho in theory.

## 2. Measured (step 2/3), unit = affine additions + Fp muls/63; S = total/sqrt(n); rho on the same group; pooled S_rho = 1.357 +- 0.114 (25 instances)
Replay check vs committed `experiments/21_gaudry_cubic_la.json` (seeds 1,2): residuals, decompositions, independent relations, la_ops identical on all 24 rows;
oracle_fp_muls differ by -0.1..-0.7% and group_ops by +0.01..0.06% (module changed after the JSON, 6 weeks older); not isolated further.

Mean over instances (5 seeds for arm P, 3 for M, J). Ops/residual and ops/decomposition are relation-phase counted ops.
| arm | p | log2 n | residuals | decomp rate | ops/residual | ops/decomp | total ops | S/S_rho (own rho) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 2lp | 271 | 24.2 | 1502 | 0.151 | 1.40e4 | 9.65e4 | 2.11e7 | 4516 |
| 2lp | 523 | 27.1 | 3240 | 0.167 | 1.43e4 | 8.72e4 | 4.62e7 | 3526 |
| 2lp | 1039 | 30.1 | 7540 | 0.180 | 1.45e4 | 8.10e4 | 1.09e8 | 2596 |
| 2lp | 2083 | 33.1 | 19621 | 0.169 | 1.47e4 | 8.71e4 | 2.88e8 | 1911 |
| 2lp | 4099 | 36.0 | 42362 | 0.170 | 1.47e4 | 8.68e4 | 6.24e8 | 3647 |
| dense | 4099 | 36.0 | 11238 | 0.169 | 1.47e4 | 8.72e4 | 1.72e8 | 1021 |
| wiedemann | 4099 | 36.0 | 12246 | 0.169 | 1.47e4 | 8.75e4 | 1.89e8 | 1137 |
| MitM m=3 | 2083 | 33.1 | 5591 | 0.174 | 6.4e4 | 3.7e5 | 3.59e8 | 2858 |
| m=2 (JV) | 1039 | 30.1 | 523580 | 0.001 | 1.1e2 | 2.3e5 | 5.96e7 | 1264 |
(Full table for all sizes: rerun `fit_n3` on `data/*.json`; the per-instance S_rho scatter (1.2-1.8) makes the "own rho" ratio noisy; the fits below use pooled S_rho.)
All m=3 rows verified (logarithm recovered, group-checked). **m=2 at p=2083 hit the 1e6-residual cap in 3/3 seeds (unverified, censored)**, excluded from fits.

Fitted exponents in n (OLS, bootstrap 95%), p <= 4099 (arm P) / <= 2083 (M, J):
| arm | total | relation phase | LA phase | rho |
|---|---|---|---|---|
| 2lp | 0.421 [0.408,0.436] | 0.421 | 0.514 [0.482,0.542] | 0.5 (pooled) |
| dense / wiedemann (full base) | 0.333 / 0.334 | 0.329 | 0.873 / 0.697 | 0.5 |
| MitM m=3 | 0.697 [0.682,0.715] | 0.697 | 0.871 | 0.5 |
| m=2 (JV, 3 sizes) | 0.64 [0.55,0.74] | 0.64 | 0.58 | 0.5 |
Theory: 2lp 4/9 = 0.444 (measured 0.421 total: consistent in the relation phase), plain relation phase n^{1/3}, dense LA n^{2/3}, MitM n^{2/3}, JV n^{2/3}.

Projection (two-phase model, each phase's fitted line): 2lp ratio S/S_rho at p=4099 ~ 1750 (pooled rho); minimum ratio 3.7x at log2 n ~ 183 and **never below 1** because the fitted LA exponent 0.514 > 1/2 overtakes the relation phase near 2^160-2^180; with the *theory* exponent for both phases (4/9) the crossover is at log2 n ~ 230 (field F_{p^3} with p ~ 2^77). Full-base dense/Wiedemann bottom at ~220-300x rho near 2^43-2^48 and then rise (agrees with RESEARCH_RESIDUAL_WALKS 11.15). Extrapolation distance 140-190 bits: indicative only.
Price band (AGENTS.md 5a; unpriced hash probes: IC 3 per residual, rho 1 per step as an upper bound favouring rho): S/S_rho for 2lp at p=4099 is 1753 / 1615 / 1000 at 0 / 0.1 / 1 GAE; order vs rho unchanged across the band for every arm.

## 3. Verdict (interim)
- Theory: n = 3 is the smallest degree where a sub-rho exponent exists (n^{4/9} vs n^{1/2}) and only with m = 3 plus double large primes; m = 2 cannot beat rho.
- Measured: the 2LP arm's relation phase scales as n^{0.42} (below rho) but is 1.8e3x rho at 2^36 and, with the measured LA exponent 0.51, never crosses rho even in extrapolation; at the theoretical 4/9 for both phases the crossover is ~2^230. n = 3 is a *cheaper instrument than n = 4* (Groebner solve per residual ~9e5 Fp muls vs the S_5 solve; k = 4 plain LA reaches 0.52x rho per RESEARCH_RESIDUAL_WALKS 11.19 but its relation phase is ~4e8x rho) but it yields no improvement over rho at any reachable size; consistent with the repo's section 11.15. No new claim beyond section 11.

## 4. Not done
p = 8209 rows (run `armP_p8209`, in flight at stop; `data/armP_p8209.log`), final fit with them, SVG/PNG/PDF (`plot_n3.py` written, not run), `benchmarks/` row, Conductor check, cleanup of the scratch crate/target (`/private/tmp/claude-501/.../scratchpad/{n3crate,target}`).
Reproduce: scratch crate `#[path]`-includes unmodified `gaudry_cubic.rs`/`residual_walk.rs` with no-op `jv_bitops` hook stubs and the FxHasher part of `fx_hash.rs`; commands in `PROTOCOL.md`; `fit_n3 out.json P=... M=... J=...`.
