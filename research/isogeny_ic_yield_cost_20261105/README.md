# Semaev index calculus on a toy Curve25519 and its isogeny class (2026-11-05)

> **Status in this repository.** A negative, confirmatory result. It adds no row to the IC
> leaderboard or `benchmarks/README.md`: there is no improvement over the IC baseline, and the
> directory has no `VALIDATION.json`. The numbers are unverified assertions (no pcc claim).
> The included checks are: every recorded scalar was recovered and verified by the producer,
> `python3 analyze_cost2.py` reproduces `results_cost2/summary.md` byte for byte from the committed
> JSONL, and `python3 plot_ratio.py` draws `results_cost2/ic_vs_rho.{svg,png,pdf}` from the same data.
> The rho baselines are DP rho with the negation map (single and multi-target) priced in group
> additions in the same Python process; native work was not priced at 0/0.1/1 GAE (§5a).
> The atlas leads this bears on are `LEAD-ICP-isogeny-transported-relation-yield` and
> `LEAD-ICP-generic-table-decomposition` (crypto-autoresearcher).
> Not committed: per-shard logs, the unseeded first run and the buggy DP-rho run (`rho_v1_buggy/`).

## Setup
- Base curve: Montgomery y² = x³ + 487554x² + x over p = 1048589 (≈2²⁰, p ≡ 5 mod 8 like 2²⁵⁵−19).
  #E = 8·131267 (q prime), #twist = 4·prime, t = −1546. Arithmetic runs on the short-Weierstrass model.
- One fixed generator G of order q (seeded, identical in every shard). On E′ = φ(E0) the run uses G′ = φ(G), Q′ = φ(kG).
  The same 100 scalars k are used everywhere.
- Isogenous curves: prime ℓ = 2, 5, 13, 17 (low); ℓ = 101, 197 (high prime); 2¹² and 5⁶ non-backtracking chains (high smooth).
- Factor base: FB = {P : x(P) < B}, B = ⌊p^{1/m}⌋, m = 2, 3, 4.
  R decomposes as ±P₁ ± … ± P_m exactly when S_{m+1}(x₁, …, x_m, x(R)) = 0 with every xᵢ < B.
  Over F_p the only option is enumeration, so all m-sums go into a table and each lookup is O(1).
  A check against explicit S₃ and S₄ (S₄ computed as a resultant) agreed on 31 of 31 and 38 of 38 test points.
- Per scalar this is a full index calculus: relations aG + bQ = ΣeᵢPᵢ, multiplied through by the cofactor 8, then linear algebra mod q.
  k was recovered for 3900 of 3900 (curve, mode, m, scalar) runs.
- Extra modes:
  - `montgomery`: FB uses Montgomery x on E0.
  - `coset`: accept R + T for all 8 rational 2-power-torsion points T.
  - `pushforward`: FB′ = φ(FB_E0).

Full tables: `results/summary.md`. Raw data: `results/per_scalar.csv` (3900 rows) and `results/paired.csv`.

## Results: yield = relations/trial over 100 scalars (mean [95% t-CI], min–max)
| m | 1/m! | E0 | best isogenous | worst isogenous | yield / (\|T_m\|/#E) |
|---|---|---|---|---|---|
| 2 | 0.500 | 0.399 [0.396, 0.402], 0.357–0.453 | ℓ=101: 0.421 (\|FB\|=532) | ℓ=17: 0.364 (\|FB\|=487) | 0.996–1.009 |
| 3 | 0.167 | 0.160 [0.157, 0.164], 0.108–0.212 | ℓ=101: 0.212 (\|FB\|=57) | ℓ=13: 0.108 (\|FB\|=45) | 0.990–1.055 |
| 4 | 0.042 | 0.100 [0.096, 0.103], 0.060–0.166 | E0 itself (\|FB\|=20) | 5⁶ chain: 0.013 (\|FB\|=11) | 1.03–1.35 |

- Yield across curves differs by more than the CIs (up to 7× at m=4). The cause is |FB|, the count of x < B that land on the curve.
  - Regressing log yield on log|FB| gives slopes 1.6, 2.7, 3.4 for m = 2, 3, 4 (theory: m), with r² ≥ 0.99.
  - Yield divided by the |T_m|/#E prediction stays flat, with no trend in isogeny degree.
    Spearman correlation with log(degree): −0.28, −0.48, +0.18, all with p > 0.19.
- Changing the model on the same curve (Montgomery x vs Weierstrass x) moves yield as much as any isogeny does.
  At m=4 it goes from 0.100 to 0.030.
- Paired per-trial decomposability (same (a, b) stream, so R′ = φ(R)) on E0 vs any small-x E′: φ-coefficient |φ| ≤ 0.016 across about 25–30k trials.
  Whether R decomposes on E0 tells you nothing about whether φ(R) decomposes on E′.
- Push-forward checks:
  - φ₁₀₁(FB_E0) gives exactly the same relation set as E0 (φ = 1.000).
  - φ₂(FB_E0) gives 2.0× the yield. This is not an isogeny gain: the kernel is a rational 2-torsion point, so relations hold up to that point.
    The `coset` mode gets the full version of the same thing on E0 with no isogeny (8 torsion cosets): 2.5× at m=2 (saturating near 0.98), 4.8× at m=3, 5.9× at m=4.

## Is a high-degree isogeny "like" a summation polynomial (m=2,3,4 ↔ degree 2/3/4)?
No.
- S_{m+1} is an (m+1)-variable relation on one curve, of degree 2^{m−1} in each variable. It encodes "m free points sum to R".
- An ℓ-isogeny's x-map is a one-variable rational function of degree ℓ between two curves. It is a group homomorphism, so it moves relations from one curve to the other but creates none.
- The real links:
  - The diagonal of S_{m+1} (all xᵢ equal) is the [m] map, which has degree m², not m.
  - Vélu's formula sums x(P+Q) over the fixed kernel points Q. Those are S₃-type terms, but the Q are constants, not factor-base unknowns.
- Data agrees: yield does not depend on degree once |FB| is accounted for. Moving to E′ only reshuffles which points count as "small", which is the same as picking a new random factor base of about the same size.

## Reproduce
```
sage -python ic_isogeny.py 100 <m> "E0:smallx,l=5:smallx,..." results/shard_X   # needs unsandboxed Sage
python3 merge.py && python3 summarize.py
```
`results_unseeded/` is an earlier run in which Sage's RNG was not seeded, so G differed between shards. Kept for reference; do not use it for pairing.

## Follow-up: same |FB| on every curve (`fixedK` mode, results_fixedK/)
On each curve the factor base is the K points with the smallest x, where K = |FB_E0| (514, 51 and 20 for m = 2, 3, 4).
B differs per curve (for example 35 to 46 at m=4). 3000 runs, every k recovered.

- **Naive yields** (relations/trial): spread across the 9 curves is 1.5%, 6.9% and 9.0% for m = 2, 3, 4, against up to 7× with a fixed B.
  ANOVA still rejected equality (p = 0.049, 0.005, 0.020), so "equal within CI" was too strong as first stated.
- **Exact yield** P = |T_m ∩ ⟨G⟩|/q, counted exhaustively (`exact_m*.txt`):
  - m=2: 0.3936–0.3991; m=3: 0.1536–0.1587; m=4: 0.0940–0.1001.
  - Small real differences per curve. They come from how the FB points spread over the 8 cofactor cosets, which the 2-part of the group shapes (ℓ=2 lands on the Z/2×Z/4 side).
- **Why naive r/n disagreed with exact:** each run stops right after a success, which inflates r/n by about r/(r−1).
  That is +4% at m=4, where r ≈ 20. A fixed-length Monte Carlo at m=4 (60k trials per curve) matches exact: Σz² = 6.8, dof 9, p = 0.66.
- **Null model:** exact P for 40 random same-size factor bases on E0 alone (`null_m*.txt`).
  - m=3: sd 0.0019 vs 0.0015 across the isogenous curves (Levene p = 0.46).
  - m=4: sd 0.0019 vs 0.0019 (Levene p = 0.94, means p = 0.84).
- **Paired per-trial correlation** with E0: |φ| ≤ 0.015 everywhere.
- **Same-size Montgomery-x FB on E0:** exact-equivalent, though it lands slightly low at m = 3 and 4 (−0.011 and −0.006).

**Conclusion:** once |FB| is equal, moving to an isogenous curve of any degree (2 up to 5⁶) changes relation yield exactly as much as picking a different random factor base of the same size on the original curve. That is, not at all, beyond FB-to-coset luck of a few percent.

## Follow-up 2: the three levers, priced end to end against Pollard rho (`results_cost/`)
Script: `cost_vs_rho.py BITS M NSCAL OUT`. Toy Curve25519-like curves (p ≡ 5 mod 8, #E = 8q, twist 4q′) at p ≈ 2^14 … 2^24.
Each configuration runs a full index calculus with a fixed G:
- Build the decomposition structure.
- Collect relations with a 1-add random walk.
- Linear algebra mod q for the factor-base logs.
- Per-scalar descent Q + bG, with a counted verifying scalar multiplication.

Costs are counted in group additions. Linear algebra is reported separately; it is negligible here at ≤ 0.1 s.
Every one of 20 scalars per configuration was recovered, with no wrong candidates.

Rho baseline: r-adding walk plus Brent, no negation map, 200 targets per size, all recovered.
The walk scales as q^0.502 ± 0.024. There is also about 1–2k ops of fixed setup.
Summaries: `results_cost/cost_summary.md` and `results_cost/best_per_size.md`.

Bug notes found on the way:
- m=2 relations without cosets have an exact rank deficit, one per cofactor-class pair {T, −T}, T ≠ −T. It is harmless for k.
- Tables must contain all ≤ m sums, or MITM and FULL disagree on R that are sums of fewer points.

| lever | effect on total cost |
|---|---|
| (1) FB size c ∈ {0.5, 1, 2} | Constant factors only. FULL prefers small c, because the table is ~ c^m·q entries. MITM at m=3 prefers c=1–2. No c changes an exponent. |
| (2) cofactor cosets | Up to 8× constant gain, no exponent change, but only if the cofactor is folded into the keys: look up hR in tables of sums of ±hPᵢ, which costs 3 doublings and **one** lookup per trial. The naive 8-lookup version makes MITM worse (m=3 c=1 at 2^24: 65× → 120× rho). Folded, it is better (m=3 c=0.5 at 2^24: 79.7× → 15.8× rho, `results_cost/folded_coset.md`, computed analytically from the measured coset trial counts). For FULL it changes nothing, because the table build dominates. |
| (3) decomposition strategy | FULL table: memory ∝ q^1.0, one-DLP cost ∝ q^0.90–0.96 (fit). It is a lookup table covering a constant fraction of the group, i.e. a BSGS/full-table attack in disguise. MITM: memory q^{1/2} (m=2, 4) or q^{2/3} (m=3); theoretical one-DLP time q^1, q^{2/3}, q^{3/4} for m = 2, 3, 4. The 4-point fits (0.51–0.77) are pulled down by about 64 scalar mults of fixed setup, so the evidence is the ratio to rho, which grows with size (m=3: 1.8× → 65×, folded cosets 2.5× → 15.8×). |

Best configuration per size (cost per DLP, ×rho total / ×rho walk-only):
- **1 DLP:** 1.8× / 20× at 2^14, growing to 65× / 111× at 2^24, using MITM m=3 c=1. Index calculus loses, and the gap widens with size.
- **100 DLPs sharing the precomputation:** 0.07× / 0.79× at 2^14, and 0.81× / 1.37× at 2^24, using FULL m=3 c=0.5 with a 355k-entry table.
  - This "win" is against a rho that recomputes everything per target.
  - Multi-target rho, which reuses distinguished points, or Bernstein–Lange precomputed rho (q^{2/3} table, q^{1/3} per DLP) beats it outright.
  - Even against the naive baseline, the ratio is getting worse with size: crossover vs the walk is near 2^22–2^24.

**Conclusion:** none of the three levers changes the picture. On a prime field the only working decomposition step is enumeration. Enumeration costs ~q (full table) or ≥ q^{1/2} per relation batch (MITM), so this index calculus is at best a worse-constant generic attack.

**Linear algebra is a second floor.** Sparse linear algebra costs about |FB|² ≈ q^{2/m}: q^1, q^{2/3} and q^{1/2} for m = 2, 3, 4.
- It is negligible in these runs only because |FB| is tiny for m ≥ 3.
- The m=2 field-op estimate grows from 4.2k at 2^14 to 8.5M at 2^24, roughly ∝ q.
- So even a free decomposition oracle cannot beat rho for m ≤ 4.

To beat rho you would need:
- m ≥ 5, or large-prime / factor-base-reduction rebalancing; **and**
- a decomposition method that finds m-term decompositions of a random R well below q^{1/2 − 1/m}/m! per trial.

That would be a sub-generic algorithm for small-root points of S_{m+1} over F_p, and none is known. That is the open problem; isogenies, factor-base tuning and cosets do not touch it.

Checks: 84 configurations, 1680/1680 k recovered, 0 wrong candidates.
Rank deficiencies occur only at m=2: 2–3 dims without cosets (the class-pair kernel) and 1 dim with cosets. All are harmless, since every k verified.
The m=4, c=0.5 rows were skipped because predicted yield < 1% (|FB| ≤ 3), not because the factor base can't be built.

## Follow-up 3: more data, everything measured (`results_cost2/`, `cost_vs_rho2.py`, `analyze_cost2.py`)
Changes from Follow-up 2:
- **Curves:** 3 curves per size (the n-th qualifying A), at p ≈ 2^14 … 2^32.
- **Scalars:** 100 per configuration.
- **Measured, not computed:** MITM is run directly (tables of ≤⌈m/2⌉ sums, a loop over ≤⌊m/2⌋ sums, early exit). Folded cosets (one lookup of 8R in tables of ±8Pᵢ) are also run for real. Neither is computed from trial counts any more.
- **m=5 added**, with a c = 4, 8 sweep for m=4 and m=5.
- **Rho baselines:**
  - Brent with no negation map.
  - Distinguished-point rho with the negation map. It uses 2-cycle reduction, plus in-walk Brent detection of fruitless cycles, escaped by doubling the cycle's minimum point.
  - The same DP rho, multi-target (Kuhn–Struik) over the 100 targets.
- **The first DP-rho run had a bug.** Fruitless cycles under the negation map were only escaped by the walk-length cap, so cost blew up at 2^30–2^32 (98k ops vs ~15k). That run is kept in `rho_v1_buggy/` and is not used.
- **Checks:**
  - 1001 IC configurations, 100,100/100,100 scalars recovered. 2 wrong candidates appeared and were rejected by the verifying multiplication.
  - 9000 rho DLPs, all correct.
  - Rank deficiencies occur only at m=2, as before.

**Headline (full tables in `results_cost2/summary.md`):**

| bits | best IC, 1 DLP | ÷ rho DP+neg | best IC per DLP, D=100 | ÷ single-target rho | ÷ multi-target rho |
|---|---|---|---|---|---|
| 20 | MITM m=3 c=1 folded | 3.4× | MITM m=3 c=1 folded | 0.07× | 1.3× |
| 24 | MITM m=5 c=2 folded | 12× | MITM m=3 c=1 folded | 0.17× | 2.3× |
| 28 | MITM m=5 c=2 folded | 25× | MITM m=3 c=1 folded | 0.37× | 4.3× |
| 32 | MITM m=5 c=2 folded | 40× | MITM m=5 c=2 folded | 0.51× | 4.8× |

- **Multi-target rho beats index calculus at every size from 2^18 on, and the gap widens.** That settles the D=100 "win" from Follow-up 2: it held only against a rho that starts over for every target.
- **The ratio to rho grows with size for every configuration** (`ratio vs rho` by m in summary.md). Best MITM at 2^32 is:

  | m | ÷ rho DP+neg |
  |---|---|
  | 3 | 68× |
  | 4 | 124× |
  | 5 | 40× |

  m=5 is the best but still loses, and its gap also grows (12× → 40× from 2^24 to 2^32).
- **Folded cosets, measured, confirm the earlier computed estimate.** MITM gains up to about 8–12×: largest at small c and at m=4, shrinking toward 1.5× at m=3 c=2, where yield is already near 1. FULL gains nothing.
- **A floor for MITM enumeration.** Once yield saturates (the regime the best configurations reach), each add tests one candidate X − s against a table covering |Tm|/q of the group.
  - So cost ≈ |FB|·q/|Tm| + |Tm|, with |Tm| ~ |FB|^k and k = ⌈m/2⌉.
  - Minimizing over |FB| gives q^{k/(2k−1)}: 2/3 for m = 3, 4, 3/5 for m = 5 and 6, and 4/7 for m = 7, 8.
  - The exponent falls toward 1/2 as m grows but never reaches it, so no m and no factor-base size beats rho this way.
  - Measured local slopes for the best m=3 config are 0.63–0.71 at 2^22–2^32. For m=5 they are noisier, at 0.48–0.72.
- **Exponent fits.** Raw fits over 20–32 bits come out below 1/2 for some large-c series (m=4 c=8: 0.48). Fixed overhead does not explain it, because single-target DP rho fits only 0.36 over the same sizes for the same reason (per-walk start multiplications and 64-point setup), while Brent's walk alone fits 0.500 ± 0.01. The size-by-size ratios are the evidence that holds up: m=4 c=8 goes from 17× to 73× rho between 2^18 and 2^26.

**Bottom line, unchanged but now measured to 2^32 with multi-target rho as the baseline:**
- Isogenies, factor-base size, cosets, decomposition strategy and m up to 5 move constants only.
- Enumeration-based decomposition over F_p has a q^{k/(2k−1)} floor, above rho's q^{1/2}.
- Beating rho needs a decomposition step that does not enumerate.
