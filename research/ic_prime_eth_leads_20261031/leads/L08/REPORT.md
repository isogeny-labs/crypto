# L08 report: Koblitz/binary transfer of the Bochum conditional theorem, Frobenius-folded hashed bases, ECC2K-130 edge census

Status: measured (all counts, scans and Monte-Carlo are scripted and in this directory); the theorem statement is a desk result. No exponent claim beyond Bochum's.
Verdict: constant_only.

Known vs new. The factor-n saving from a Frobenius-stable base (relations /n, linear algebra /n^2, never more) is KNOWN: atlas
`LEAD-ICB-frobenius-orbit-factor-base` is closed_obstructed (gain exactly n; at n=131 the stable *subspace* bases are empty). Only the
composition of that saving with the tensor-contraction route (a canonical-representative hash instead of a subspace base, one extra vertex
in the relation network) is new here, and it is an accounting identity: it does not touch the contraction exponent. The related open atlas
lead is `LEAD-ICB-nonlinear-frob-stable-bases` (open_next_step).

## 1. Prototype reproduction (script c1.py, output c1_out_*.txt)
Prototype: cryptanalysis/experiments/hashed-ecdl-tensor-reference (tensor_encoding.WordCircuit.statistics via tensor_cli.challenge_network,
plan-only; needs Python >= 3.10 + sympy: used /usr/local/bin/python3.13; /usr/bin/python3 3.9 fails on `tuple[int,int] | None`).
| network | edges | variables | max degree | blocks |
|---|---|---|---|---|
| ECC2K-130, 2 slots | 2,694,680 (reproduced) | 1,195,386 | 1,032 | 35 binary_polynomial (J=27 ring_bits 54, k=6, L=81), 4 integer (ring 129, L=16) |
| ECCp-131, 2 slots | 1,564,506 (reproduced) | 654,523 | 1,032 | 38 integer |
Edge metric (WordCircuit.statistics): for each variable with d incident constraints (interface bits count +1) the fan-out tree contributes 0 if
d=0, 1 if d<=2, 2d-3 otherwise; the total is then doubled (every wire subdivided once to make the graph bipartite). Bochum's Lemma 7 / Theorem 3
count (<1100(d+1) edges) is from the paper (not on disk here; constants taken from LEADS.md section 2), so the two metrics are not proven
identical. Same-metric sanity: one 133-bit integer multiplication gadget = 37,030 edges, degree 1,032, versus 1100*134 = 147,400 and
136*134^(3/4) = 5,356. The max degree 1,032 of BOTH networks comes from the integer hash multiplier (8 words x 129 bits), not from the
binary field gadget (972 at L=81, 324 at L=9).

## 2. Free block width and Frobenius vertex (l08lib.py, t1.py, t3.py, fit.py, princ.py)
FreeCircuit overrides polynomial_multiply with a free ring/blocks pair (J, k), k <= J (needs 2k-1 <= 2J, L=3^a | 3J, L >= 2*ceil(d/k)-1).
At d=131 the choice N=J=27, k=27, L=9 (n=5 chunks) is used ("free27"). Checks: gadget output equals field multiplication on random inputs
and all constraints satisfied (t1.py, 7 cases, d=19 and 131); x^(3J/L) is a principal L-th root in F2[x]/(x^2J+x^J+1) for (J,L) in
(27,9),(27,27),(27,81),(81,3),(81,81),(9,27) (princ.py; Lean covers only N=3,9). One gf_multiply at d=131: prototype 64,850 edges, free27 10,366,
best over (J,k) 10,202 (J=81,k=66, L=3, degenerate and deg 972): the choice barely matters once k~J.
Frobenius vertex: per slot, one constraint on (x,y,x',y') = 4*131 = 524 ports with predicate (x',y') = canonical rep of the tau-orbit
(here least (x,y) over the orbit in polynomial basis, a stand-in for least cyclic rotation in a normal basis; basis change is F2-linear and
lives inside the vertex), and the hash is applied to the canonical point. Whole relation network, 2..6 slots, same seeds/params as tensor_cli:
| m | proto edges | free27 edges | free27+fold | free81k66 | maxdeg |
|---|---|---|---|---|---|
| 2 | 2,694,680 (=1) | 787,740 | 788,788 | 782,016 | 1032 |
| 3 | 4,225,078 | 1,228,458 | 1,230,030 | 1,219,462 | 1032 |
| 4 | 5,755,472 | 1,669,172 | 1,671,268 | 1,656,904 | 1032 |
| 6 | 8,816,260 | 2,550,600 | 2,553,744 | 2,531,788 | 1032 |
Fit edges = a*m + c0 (max residual 1.6 edges; exact increments 440,714 for m>=3): proto a=1,530,395 c0=-366,108; free27 a=440,715 c0=-93,688;
free27+fold a=441,239 c0=-93,688; fold adds exactly 524 edges per slot (1048 at m=2 = 2 x 524). Freeing k cuts the ECC2K-130 network 3.42x
(m=2); folding adds +0.13% edges and no degree change at n=131 (524 << 1032). Per slot about 440k edges = 3,364 per bit, so the Lemma-11 O(mb)
constant is ~3.4e3 per slot-bit (the measured prototype is 1.17e4 per slot-bit); LEADS section 2's derived '~1e5 b per addition' is not what this encoder produces.

## 3. Hand-bound scan, d = 1..5000 (cnt.py exact counter, validated against the full builder on 3 cases; scanA.py, scanC.py, anal.py)
Object: one d-bit GF(2)[x] multiplication gadget (polynomial_multiply, interface = inputs + outputs, same metric as section 1).
Policy A (k=J, smallest valid J; all 5000 d): edges/d ranges 61.7..173.9 (median 121.7, d>=1000), degree <= 24.4 d^(3/4).
 - edges < 80d + 228 sqrt(d): FALSE, first violation d=46 (3,443 of 5000 d violate). Under an undoubled wire count (edges/2) it first fails at d=3322.
 - degree < 102 d^(3/4): no violation (d<=5000). edges < 1100(d+1) and degree < 136(d+1)^(3/4): no violation (margin >= 6.3x on edges).
Best-of search over (L,J,k) (scanC.py, d<=400 all + every 13th to 5000, 755 values; degree cap 102 d^(3/4)): strict L>=9: sup edges/d = 158 (d>=100),
first violation of 80d+228 sqrt(d) at d=3; literal (any L, including the degenerate L=1 single big ring vertex whose degree is ~12d and so is only under
the cap for d<~5200): sup 89.7 d, first violation d=2195. So the earlier hand bound 80d+228 sqrt(d) is not supported; the supported statement is
edges <= ~175 d (this metric) and degree <= ~56 d^(3/4) on 1<=d<=5000, i.e. well inside Bochum's constants (sampled for best-of, exhaustive for policy A,
not a proof for d>5000).

## 4. Sizes and exponents (table.py, table_out.txt; log2 r = 129.000, Bochum L = max(r^(1/m), 10m), N = m*L)
Rho floor 2^60.81 = sqrt(pi*l/(2*262)) additions (REVIEW-20261001.md). Dense elimination N^3, sparse (Wiedemann) N^2 (m+2), field ops; folded: N/n, n=131
(log2 n = 7.03) columns. Contraction per relation = 2^(s/sqrt(log2 s)) with D.1 constant c=1 (optimistic; D.1 itself is inert below s ~ 2^(2^85)).
| network | m | edges s | log2 N | log2 N/n | dense / sparse (bits) | dense / sparse folded | s/sqrt(log2 s) bits | c needed so ONE relation = rho |
|---|---|---|---|---|---|---|---|---|
| ECC2K-130 prototype | 2 | 2,694,680 | 65.50 | 58.47 | 196.5 / 133.0 | 175.4 / 118.9 | 583,028 | 1.0e-4 |
| ECCp-131 integer | 2 | 1,564,506 | 65.50 | - | 196.5 / 133.0 | - | 344,892 | 1.8e-4 |
| char-2 free27 | 2 | 787,740 | 65.50 | - | 196.5 / 133.0 | - | 177,990 | 3.4e-4 |
| char-2 free27 + fold | 2 | 788,788 | 65.50 | 58.47 | - | 175.4 / 118.9 | 178,218 | 3.4e-4 |
| char-2 free27 | 3 | 1,228,458 | 44.58 | - | 133.8 / 91.5 | - | 273,136 | 2.2e-4 |
| char-2 free27 + fold | 3 | 1,230,030 | 44.58 | 37.55 | - | 112.7 / 77.4 | 273,473 | 2.2e-4 |
| char-2 free27 | 4 | 1,669,172 | 34.25 | - | 102.8 / 71.1 | - | 367,133 | 1.7e-4 |
| char-2 free27 + fold | 4 | 1,671,268 | 34.25 | 27.22 | - | 81.6 / 57.0 | 367,578 | 1.7e-4 |
| char-2 free27 | 6 | 2,550,600 | 24.08 | - | 72.3 / 51.2 | - | 552,881 | 1.1e-4 |
| char-2 free27 + fold | 6 | 2,553,744 | 24.08 | 17.05 | - | 51.2 / 37.1 | 553,540 | 1.1e-4 |
Reading: folding takes the relation COUNT N at m=2 from 2^65.5 to 2^58.5 (below the 2^60.81 floor) and the sparse elimination at m=6 to 2^37, but every relation
still costs 2^(1.8e5 .. 5.8e5) at c=1: the distance to the rho floor is 10^5 bits, set by s/sqrt(log s), not by N. Shrinking edges 3.4x moves it by 3.4x in the
exponent (583k -> 178k bits), i.e. a constant. Total relation phase (folded count x 2.5 attempts x contraction) is 2^178,278 at m=2 and 2^553,558 at m=6; the
best row is m=2 free27+fold. Consistent with Lean `arity_two_blackbox` / `no_crossover` (not re-proved here).

## 5. Monte-Carlo, n = 19 toy (ecc2k.toy(19): r = 130,873, cofactor 4, tau acts as lambda = 41,811 of order 19, no tau-fixed points; mc_table.py, mc.py, mc_rows.json, mc_pred.txt)
Exact count X of tuples in F_1 x ... x F_m (nonzero points, sum = R), random seeds (alpha, beta per slot, s = nextprime(2^20)) and R, paired designs (same seeds/R for
unfolded hash H(enc(A)) and folded hash H(enc(least orbit rep))); L = (r-1)tau/s is the expected |F_i|. T = 30,000 trials (m=2), 4,000 (m=3).
| m | L | E[X] unf/fold | Pr[X>0] unf | Pr[X>0] fold | ratio fold/unf | paired diff (se) | Var X ratio |
|---|---|---|---|---|---|---|---|
| 2 | 90 | .062/.061 | .0598 | .0583 | 0.976 | -.0014 (.0019) | 1.03 |
| 2 | 128 | .125/.124 | .1178 | .1145 | 0.972 | -.0033 (.0026) | 1.05 |
| 2 | 256 | .500/.504 | .3925 | .3826 | 0.975 | -.0099 (.0040) | 1.09 |
| 2 | 362 | 1.007/.989 | .6326 | .6100 | 0.964 | -.0226 (.0039) | 1.08 |
| 2 | 512 | 2.000/2.009 | .8637 | .8454 | 0.979 | -.0182 (.0029) | 1.14 |
| 2 | 724 | 4.002/3.991 | .9808 | .9708 | 0.990 | -.0101 (.0013) | 1.18 |
| 3 | 32 | .259/.246 | .2295 | .1643 | 0.716 | -.0653 (.0088) | 1.80 |
| 3 | 48 | .853/.831 | .5563 | .4185 | 0.752 | -.1378 (.0111) | 1.96 |
| 3 | 64 | 2.009/1.946 | .8510 | .6415 | 0.754 | -.2095 (.0093) | 2.94 |
| 3 | 96 | 6.661/6.770 | .9988 | .9203 | 0.921 | -.0785 (.0043) | 4.80 |
E[X] is unchanged (first moment untouched), Pr[X>0] is lower with folding (2-3.5% at m=2, up to 28% at m=3 where L/n is only 2-5), as the second moment predicts.
Heuristic fit: extra variance ~ m (n-1) E[X]^2 / L (orbit-mate pairs share hash membership with probability p instead of p^2; relative size (n-1)/|F_i| per slot);
measured/predicted within 0.8..1.4 for 9 of 10 cells (mc_pred.txt), 1.8 at the noisiest cell. At ECC2K-130 sizes L >= r^(1/m) >= 2^21.5 for m<=6, so (n-1)/L <= 6e-5: the
correction is negligible there; the toy exaggerates it because L/n is 5..40.

## 6. Conditional theorem for E(F_{2^n}) (desk; conditional on Appendix D.1 of the OpenAI contraction preprint, exactly as Bochum)
Setting: E/F_{2^n} with a subgroup of prime order r ~ 2^(n-O(1)), cofactor h fixed power of two (ECC2K-130: h=4), b = n, m >= 2 slots, enc(A) = 2x + lsb(y/x), s prime with
2^(n+2) < s <= 2^(n+3), H_i(z) = (alpha_i z + beta_i) mod s, F_i = {A : H_i(enc(A)) < tau}, L = (r-1)tau/s >= max(r^(1/m), 10m).
(U) Unfolded. Replace Bochum's Theorem 3 (field multiplications) by the Schoenhage triadic char-2 gadget (J=3^nu, k <= J, one split L=L1 L2). Then the relation network
"A_i in F_i, sum A_i = R" has O(mn) edges and degree O(n^(3/4)) (measured: edges <= ~175 n and degree <= ~56 n^(3/4) per multiplication for all n <= 5000 sampled/scanned; the
asymptotic constant is a desk claim, not proved), the integer hash arithmetic still uses Bochum's Theorem 3/Lemma 8 unchanged, and Bochum's Theorem 4 holds verbatim:
2^{O(n/(log n)^{1/4})} for m = ceil((log n)^{1/4}), N = O(m r^{1/m} + m^2).
(F) Folded. Replace H_i(enc(A)) by H_i(enc(canon A)), canon = least tau-rotation representative (tau-stable F_i). Then N counts orbit columns: N_fold = N/n (+ O(m^2)), dense
elimination /n^3, sparse /n^2, attempts needed /n; per-attempt network = (U) plus one canonicalisation vertex per slot (4n ports, +524 edges per slot at n=131). Contraction
exponent per relation: unchanged. The log-factor exponent bookkeeping of Theorem 4 is unchanged (n only enters as log2 n additive bits), so the total exponent is 2^{O(n/(log n)^{1/4})}
again: no exponent improvement, a factor n in N and in the relation count.
Carry over unchanged: Lemma 1 (bitwise witness sampling), Lemma 2 (adders/comparators/copies), Lemma 8 and Theorem 3 (integer hash arithmetic, quotient guessing mod s), Lemma 13
(fixed-target/random-seed to fixed-seed/random-target; averaging), Theorem 4's algebra (balance in m), the black-box use of D.1.
Need re-proof or new proof: (i) the char-2 multiplication gadget (parsimony, carry-less no-wraparound 2k-1 <= 2J, principal root; Lean has N=3,9, checked here by computation for N=27,81); (ii) the
five-selector complete-addition case split for char-2 formulas (the prototype uses generic/doubling/inverse/pass-through; brute-force count checks at n=5,7); (iii) Lemma 11 constants;
(iv) FOLDED ONLY: Lemma 10 second moment (orbit-mates break pairwise independence; need L >= C m n on top of max(r^(1/m),10m), MC above is consistent with the 1/L correction);
Lemma 12 extraction (rows carry coefficient lambda^j with j the rotation read from the vertex; the V-uniform argument needs re-checking, expected to go through); the canonicalisation vertex:
arity 4n is inside the numerical cap at n=131 (524 vs 5,356) but is Theta(n) > O(n^(3/4)) asymptotically, so the theorem needs a barrel-shift + necklace-test network (O(n log n) edges, O(1) degree,
unbuilt; adds a log factor to the Lemma-11 count unless a linear-edge version exists).

## Verdict: constant_only
Binary transfer lands as a conditional theorem with the same exponent; the char-2 gadget is NOT worse than the integer one (ECC2K-130 relation network 3.42x smaller than the prototype,
2.0x smaller than ECCp-131's); Frobenius folding adds 0.13% edges and saves a factor n = 131 (2^7.03) in N, elimination, and relation count. Distance to rho unchanged: ~1.8e5 bits of
contraction exponent per relation at c=1 against 60.81. Constants only.

## Skeptic re-run (about 2 minutes; Python >= 3.10 with sympy/numpy; /usr/local/bin/python3.13 works)
cd <this dir> && python3.13 -B c1.py ecc2k130 && python3.13 -B c1.py eccp131 && python3.13 -B t3.py && python3.13 -B fit.py && python3.13 -B table.py
(scans: scanA.py 5000 ~12 min; MC: mc_table.py then mc.py ~10 min). Note REF path is hard-coded at the top of l08lib.py.

## Next step
(1) Build the O(n log n) barrel-shift/necklace canonicalisation and re-run t3.py with it (decides whether folding really costs only a log factor); (2) re-prove Lemma 10 with orbit-mates
(second-moment bound with the m(n-1)/L term) and add it to the paper section; (3) hand the ECC2K-130 free27 network (787,740 edges) to L06/L09 as the smaller test graph.
Blockers: Bochum paper text not on disk (Lemma 7 count and lemma numbers taken from LEADS.md); stray scanB.py processes (pids 79233, 83763) could not be killed (kill/pkill denied).
