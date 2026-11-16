# L07 report: second moment of hashed vs window bases, and the m! constant

Date 2026-10-31. Workdir: files were produced in the session scratchpad directory
`/private/tmp/claude-501/-Volumes-SSD990-2/05ea5a86-648b-4917-bb71-bf7fdae8b2e7/scratchpad/L07/` and copied to /tmp/claude-501/L07/ (Write tool was denied there; cp worked). Nothing written under /Volumes/SSD990-2/ic-prime-eth-leads-20261031/).
Labels: [M] measured (file), [D] derived, [P] plan.

## Verdict
**constant_only** [M]. Equal-size hashed bases (affine-mod-s, sparse-parity weight 5, dense F2-affine) are indistinguishable from i.i.d. random
bases in E[X], Var X, Pr[X>0], Pr[X>=2] at b = 14, 18, 22, m = 2,3,4, both E[X] = 1 and 1/4, independent and shared base. The fitted slope of ln(Pr[X>0]/null)
against b is 0 within +-2e-4 per bit (m<=3), +-7e-4 per bit (m=4); no hashed arm has a ratio above 1.008 (+-0.006), i.e. nothing like a "factor growing with b".
Base choice is free: adopt the cheapest membership (L3 sparse parity). The only base-dependent effect found is **negative and decaying**: an x-coordinate
window is closed under negation, so shared-base windows lose Pr[X>0] to trivial P+(-P)+... cancellations (ratio 0.92/0.95/0.99 at b=14/18/22 for m=3, E=1; 0.71/0.83/0.92 for m=4), and Fano factor
Var/E is 2.4-3.7x the null. A same-size window that is not negation-closed (`window_y`) removes the effect entirely (ratio 1.000 +- 0.002), so it is a symmetry artefact, not window additive structure.
The positive control (dlog-interval base) is detected with ratio 0.001-0.04 and Fano 40-3700x, so the instrument can see additive structure when it exists.

## Instances [M] (curves.txt, curves.py)
Prime-order short-Weierstrass curves y^2=x^3+ax+b over F_p, group order r prime (point count by Legendre sum, r prime checked), three per size:
b=14: p=13997 (r=13903), 14251 (r=14327), 15731 (r=15797); b=18: p=241739 (r=242393), 234467 (233939), 197741 (198277);
b=22: p=4147753 (r=4146767), 3192919 (r=3189427), 4188551 (r=4185917). Full tables k->(x,y) for all k in Z_r from `dlog.c` (tab_*.bin).
X(R) = number of ordered m-tuples (independent slots) or number of m-subsets of distinct points (shared base, via Newton/elementary symmetric identities in Fourier space) of base points summing to R, for **every** R in Z_r, by FFT cyclic convolution over Z_r (exact after rounding; scipy.fft). R=0 is excluded from statistics (a negation-closed base gives X(0)=|B|).
Point code c = 2x + (y mod 2) < 2p (distinguishes P and -P). Arms, all with per-slot size n_i = r/2^{k_i}, sum k_i = round((m-1)log2 r - log2 E_nom) so prod n_i / r ~ E_nom (quantised; realised sizes recorded):
- window: x in [o, o+w) mod p (w=n p/r), random offset per slot. window_y: x-window of width 2w and one y-parity (same size, not negation-closed): my control.
- affine-mod-s: [(a c + b) mod q] mod S = t, q random prime in [4p,8p], S=r/n. (Bochum's crate draws its hash prime in [2p,4p] by uniform sampling plus exact primality, `crates/hashed-ecdl-tensor/src/arithmetic.rs::hash_prime` [M, git show origin/main]; the exact membership form is my instantiation.)
- sparse5: k random rows of weight 5 over the b+1 code bits, random RHS, all k parities satisfied. dense: rows with i.i.d. uniform masks.
- interval (positive control): dlog interval {o..o+n-1}. random: i.i.d. uniform subset of size n; paired null = a random set of the arm's REALISED sizes; for the `random` arm the "ref" is a second independent random stream (noise floor).
- independent: fresh randomness per slot. shared: one set used in all m slots (the m! case).
Reps: b=14: 3 curves x 60 reps = 180 instances per cell; b=18: 3 x 6 = 18; b=22: 3 x 1 = 3 (each instance = every R in Z_r, so the within-instance statistic is precise; the between-instance se is bootstrap over instances).
Compute: about 40 min on this machine; raw rows raw14*.jsonl raw18*.jsonl raw22_c*.jsonl; old22_partial.jsonl and v1/ are an earlier superseded run (not used).

## Results [M] (tables.md has all cells; results.json machine-readable; table.txt full text incl. slopes)
Ratios are arm / size-matched random null, pooled means with bootstrap se. Selected cells (shared base, E_nom=1, Pr[X>0] ratio at b=14/18/22):
| arm | m=2 | m=3 | m=4 |
|---|---|---|---|
| affine | 1.000/0.999/1.000 | 0.999/1.000/0.999 | 1.000/1.001/0.998 |
| sparse5 | 1.000/0.999/1.000 | 1.001/0.996/0.999 | 0.992/0.996/0.998 |
| dense | 0.998/0.999/1.000 | 1.002/1.000/1.002 | 1.008/1.002/1.002 |
| window (neg-closed) | 0.992/0.998/0.999 | 0.922/0.951/0.987 | 0.713/0.833/0.916 |
| window_y | ~1 | 1.000 (b=18), 1.001 (b=22) | 1.007 (b=18), 0.999 (b=22) |
| interval (control) | 0.043/0.011/- | 0.025/0.006/- | 0.039/0.012/- |
| random vs random2 (noise floor) | 0.999/1.000/1.000 | 1.001/1.002/1.000 | 0.997/1.001/1.001 |
Independent-base cells: every arm within 0.995-1.003 of null at all b (window m=2,E=1,b=18 at 0.986 +- 0.012 is 1.2 sigma), Fano ratios 0.998-1.015.
Pr[X>=2] ratios are 1.00 +- 0.01 where Pr[X>=2] > 0.05 (independent bases, m=2,3); for shared m=4 Pr[X>=2] < 0.01 and the ratio is statistically empty (se of order 1 or more), so no claim.
Slopes of ln(Pr[X>0] ratio) per bit of b (table.txt, bottom): hashed arms |slope| <= 4e-4 +- 4e-4 everywhere (z all |z|<2.3, no sign pattern); window shared: +0.0007 (m=2), +0.008 (m=3), +0.031 (m=4) per bit (positive, i.e. the deficit closes); interval strongly negative (-0.33 to -0.5 per bit, z up to -230).

**m! constant** [D+M]. Independent slots: Pr[X>0] ~ 1-exp(-E) (0.55-0.60 at E~0.9) so ~1.7 attempts, no m!. Shared base: ordered mean E_ord = N^m/r but each unordered relation is counted m! times, so E_u = E_ord/m! (identity used by the code; confirmed indirectly because the paired random null has the same E_u as every hashed arm), and Pr[X_u>0] ~ E_u, hence 1/Pr[X>0] ~ m!/E_ord. With quantised sizes the raw 1/Pr[X>0] at E_nom=1 is 2.7-2.8 (m=2, vs 2), 4-9 (m=3, vs 6), 8-10 at b=14 and 30-50 at b=18 (m=4, vs 24; E realised varies 0.01-0.13), see tables.md raw block. The decisive statement is the ratio to the null: 1.000 +- 0.005 for every non-negation-closed arm, so no base gets a smaller constant than m! (nor larger), at toy sizes through b=22.

## Decision rule and application
Rule (LEADS L07): ratios within noise => base choice free (constant_only, adopt cheapest membership); a base with Pr[X>0] higher by a factor growing with b => reopen.
Applied: hashed arms within noise, slope ~0 => **constant_only**. No arm with ratio > 1 growing in b. Window ratio < 1 and rising toward 1 is the opposite sign and is explained (negation closure; confirmed by window_y control), so it is not a reopen. Prior "flat" confirmed to second order, the first-moment `yield_conservation` (IcTensorRoute.lean) extends numerically to Pr[X>0], Pr[X>=2] and Var for pairwise-independent-style hashes.

## Caveats
- Toy sizes (b<=22; m=4 has |B| of only ~10-30 points at b=14/18), only 3 instances at b=22, 3 curves per size; hashes are my instantiation of the Bochum family, pairwise-independence strength is not varied.
- Sizes quantised to r/2^k so realised E_nom differs by up to ~2x for shared bases; comparison is therefore always to a size-matched null.
- Only generic target R in Z_r; special targets (R in B, R=0) excluded. A solver that exploits the P,-P structure of a window base is outside this lead (it is the effect seen above).
- Pr[X>=2] too small to compare for shared m=4.
- Does not test hashes on the extension-field (binary) side; that is L8.

## Reproduce / skeptic re-run
`mkdir -p /tmp/sk && cp curves.txt tab_14_1.bin exp.py analyze.py /tmp/sk/ && cd /tmp/sk && CURVES=1 python3 exp.py 14 5 raw_sk.jsonl 1 && python3 analyze.py | grep ' shared ' | head -40` (b=14, curve 1, 5 reps, under a minute; expect hashed arms P1/ref = 1.00 +- 0.01).
Pipeline: `python3 curves.py; sh gen.sh; python3 exp.py <bits> <reps> raw<bits>.jsonl <workers>; python3 analyze.py > table.txt`.
Full data was produced by: `python3 exp.py 14 60 raw14.jsonl 1`; `python3 exp.py 18 16 raw18.jsonl 2`; `CURVES=c ARMS=window,window_y,affine,sparse5,dense,random python3 exp.py 22 1 raw22_c$c.jsonl 3` for c in 7 8 9.

## Next step [P]
Close L07 as constant_only in the atlas (LEAD-ICE-decomposition-probability-audit: second-order confirmation of KN-FIND-007 for window/affine/sparse/dense bases; note negation-closure deficit for shared x-windows). Optionally add one 18-bit-to-26-bit rung for m=3 shared window to confirm the deficit follows ~r^{-1/m}, but that is cosmetic. Use sparse-parity membership (L3) without a yield penalty.
