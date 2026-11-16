# L08 — Implementation specification (frozen with PROTOCOL.md)

Implement exactly this; make no research choice. If something here is impossible or
self-contradictory, stop and report it; do not improvise.

## 1. Files and language

- One file: `code/l08.py`, Python 3 standard library only (`random`, `math`, `json`,
  `sys`, `argparse`, `glob`, `itertools`). No numpy/sympy. Single thread.
- Subcommands: `selftest`, `gate`, `run`, `analyze` (argument names as in
  PROTOCOL §10). `--curves` takes a comma list of admitted-curve indices (0-based).
- Outputs: JSON lines (one JSON object per line, `json.dumps(obj, sort_keys=True)`),
  `selftest` prints one line per test `PASS name` / `FAIL name detail` and a final
  `ALL PASS` or `FAILURES n`; exit code 0 only if all pass.
- Integers mod p are Python ints in [0, p).

## 2. Counter (the unit)

```
class Ctr:  mul = 0; inv = 0
def M(a, b, C): C.mul += 1; return a * b % p        # squarings use M(a, a, C)
def INV(a, C):  C.inv += 1; return pow(a, p - 2, p) # a != 0; internal work not counted
fpm(C) = C.mul + 10 * C.inv
```
Additions, subtractions, negations, comparisons, dict lookups: not counted.
Multiplication by a small integer constant (2, 4, 18, 27, i in a derivative) is a
counted `M`. Every counted routine takes the counter `C` explicitly. Set-up work
(curve generation, genus test, Vandermonde inverse) and all verification use a
throw-away `Ctr()` or plain arithmetic and is never added to any reported counter.

`powmod(a, e, C)` (counted): if e == 0 return 1; bits of e from the most significant;
r = a; for each later bit: r = M(r, r); if bit == 1: r = M(r, a). Return r.
`legendre(a, C)` = powmod(a, (p-1)//2, C) mapped to 1 / -1 / 0.
`sqrt_ts(a, C)` Tonelli–Shanks: write p-1 = q·2^s, q odd; z = smallest z >= 2 with
legendre(z) = -1 (found with a throw-away counter); c = powmod(z, q); x =
powmod(a, (q+1)//2); t = powmod(a, q); mloop = s; while t != 1: find least i with
t^(2^i) = 1 by repeated M(t2, t2); b = c raised to 2^(mloop-i-1) by repeated squaring
(counted); x = M(x, b); c = M(b, b); t = M(t, c); mloop = i. Return x.

## 3. Curves

### 3.1 Monomials
`mons(d)` = `[(t - j, j) for t in range(d+1) for j in range(t+1)]` (total degree
ascending; within it j ascending). Affine f(x,y) = Σ a[(i,j)] x^i y^j; homogeneous
F(X,Y,Z) = Σ a[(i,j)] X^i Y^j Z^(d-i-j). Genus g = (d-1)(d-2)/2 - m.

### 3.2 Generation for candidate seed s (s = 1, 2, …, at most 2000)
- rng = `random.Random(f"L08-curve-d{d}-m{m}-p{p}-s{s}")`.
- Draw a value for every monomial in `mons(d)` order: (0,d) gets `rng.randrange(1, p)`,
  every other `rng.randrange(p)`.
- Nodes N_k = (k, k*k % p) for k = 0..m-1 (0^0 = 1). For each node three rows over the
  columns `mons(d)`: f-row x^i y^j; fx-row i·x^(i-1) y^j (0 if i = 0); fy-row
  j·x^i y^(j-1) (0 if j = 0). Rows in order (node 0: f, fx, fy; node 1: …).
- Reduce the 3m × |mons| matrix to reduced row-echelon form mod p choosing pivot
  columns in `mons(d)` order, never the column (0,d) (it is carried through the row
  operations). If rank < 3m: reject reason `rank`.
- For each pivot row r with pivot column c_r: a[c_r] = -Σ_{c not a pivot} R[r][c]·a[c] mod p.

### 3.3 Genus test (uncounted). D = d(d-1).
- For x0 = 0..D: A(y) = f(x0, y) (degree d in y, leading a[(0,d)]), B = dA/dy.
  S_val[x0] = det(Sylvester(A, B)) mod p.
  Sylvester(A, B) with deg A = n_A, deg B = n_B, size n_A + n_B; rows 0..n_B-1: row r
  holds A's coefficients highest→lowest starting at column r; rows n_B..n_B+n_A-1:
  row n_B+s holds B's coefficients highest→lowest starting at column s.
  det by Gaussian elimination mod p (row swaps flip the sign).
- Interpolate S(x) of degree ≤ D through (x0, S_val[x0]) (solve the Vandermonde
  system by Gauss–Jordan mod p).
- Reject `degS` if the x^D coefficient of S is 0.
- For each node k: divide S twice by (x - k); reject `node_mult` if a remainder is
  nonzero. Call the quotient T.
- Reject `node_extra` if T(k) = 0 for some node k.
- Reject `T_sqfree` if gcd(T, T') has degree ≥ 1.
- Assert (deg T - 2d + 2)/2 == g, else reject `genus_mismatch`.
- Admitted curves for (d, m, p) are the admitted seeds in increasing s; curve index c
  is the c-th of them. Record `curve_seed`, `seeds_tried`, rejection counts by reason.

### 3.4 Points (counted, counter `E`)
- Enumeration RNG `random.Random(f"L08-enum-d{d}-m{m}-p{p}-s{s}")` (used only by EDF).
- For x0 in 0..p-1: xp[0] = 1, xp[1] = x0, xp[i] = M(xp[i-1], x0) for i = 2..d.
  Coefficient of y^j: c_j = Σ over monomials (i, j): term a[(i,j)] if i = 0 else
  M(a[(i,j)], xp[i]). roots = `rational_roots([c_0..c_d])`. For each root y0: skip if
  (x0, y0) is a node, else add point (x0, y0, 1).
- Infinity: poly [a[(d,0)], a[(d-1,1)], …, a[(0,d)]] (coefficient of m^j is a[(d-j,j)]);
  for each root m add (1, m, 0).
- `pts = sorted(all points)` (Python tuple order); `index = {pt: i}`; N = len(pts).

## 4. Counted polynomial arithmetic

Polynomials: lists of ints, low → high, trimmed (no trailing zeros); zero = `[]`.
- `pmul(a, b, C)`: schoolbook, every pair a[i]·b[j] is one `M` (no zero skipping).
- `pmonic(a, C)`: if leading == 1 return copy (no cost); else il = INV(lead), every
  other coefficient M(c, il), leading set to 1.
- `prem(a, h, C)` (h monic, deg h = k): copy a; for i from len(a)-1 down to k:
  q = a[i]; if q != 0: for j in 0..k-1: a[i-k+j] -= M(q, h[j]); a[i] = 0. Trim.
- `pdivexact(a, h, C)`: same loop, collecting q into the quotient; assert remainder 0.
- `pmulmod(a, b, h, C)` = prem(pmul(a, b), h).
- `ppowmod(base, e, h, C)`: r = prem(base, h); left-to-right over the bits of e after
  the leading one: r = pmulmod(r, r, h); if bit: r = pmulmod(r, base_reduced, h).
- `pgcd(a, b, C)`: while b != []: b = pmonic(b); a, b = b, prem(a, b). Return
  pmonic(a) (or [] if a = []).
- `roots_edf(h, rng, C)` (h monic, splits into distinct linear factors): deg 0 → [];
  deg 1 → [(-h[0]) % p]; else loop: δ = rng.randrange(p);
  w = ppowmod([δ, 1], (p-1)//2, h); w = w - 1 (trim); gg = pgcd(h, w); if
  0 < deg gg < deg h: return roots_edf(gg) + roots_edf(pdivexact(h, gg)); else retry.
- `rational_roots(poly, rng, C)`: f = pmonic(poly); w = ppowmod([0,1], p, f) - [0,1]
  (trim); h = pgcd(f, w) (if w == [] then h = f); return roots_edf(h).

## 5. Relation collection (`run` for one curve and one arm)

### 5.1 Precompute per run (charged to the arm's counter `C`)
invu[u] = INV(u) for u = 1..d-1 (d-1 inversions). Uncounted: Vinv = inverse of the
(d-1)×(d-1) matrix V[u-1][r] = u^r (u = 1..d-1, r = 0..d-2) mod p.

### 5.2 Factor base
- `fb = random.Random(f"L08-fb-d{d}-m{m}-p{p}-s{s}")`; order = list(range(N));
  fb.shuffle(order).
- FB arm: F = order (nb = N). 2LP arm: nb = min(N, round(cF * p ** (1 - 1/(d-2)))),
  F = order[:nb]. P0 = F[0]. inF = set(F).
- needed = nb + 8 + nb // 100.

### 5.3 Pair order
- run rng = `random.Random(f"L08-run-d{d}-m{m}-p{p}-s{s}-{arm}-cF{cF}-rep{rep}")`
  (FB: arm "FB", cF 0, rep 1). Pm = nb(nb-1)/2; Mp = smallest prime ≥ max(Pm, 2)
  (trial division). A = rng.randrange(1, Mp); B = rng.randrange(0, Mp). For t = 0..Mp-1:
  k = (A·t + B) % Mp; skip if k ≥ Pm; else (i, j) = unrank(k), pair (F[i], F[j]).
- unrank(k): j = (1 + isqrt(1 + 8k)) // 2; while j(j-1)/2 > k: j -= 1; while
  (j+1)j/2 ≤ k: j += 1; i = k - j(j-1)/2. (0→(0,1), 1→(0,2), 2→(1,2), 3→(0,3).)
- The same rng then supplies every EDF δ in this run.
- If the loop ends before the stop rule: exhausted = true.

### 5.4 One candidate line, P1 = pts[a] = (X1,Y1,Z1), P2 = pts[b] (counted in C)
`lines_tried += 1`.
1. L = (M(Y1,Z2) - M(Z1,Y2), M(Z1,X2) - M(X1,Z2), M(X1,Y2) - M(Y1,X2)) (6 M).
2. For each node (xk, yk): v = M(L0,xk) + M(L1,yk) + L2; if v == 0: `node_lines += 1`; next line.
3. For u = 1..d-1: Q = (M(u,X1)+X2, M(u,Y1)+Y2, M(u,Z1)+Z2); cval[u] = M(evalF(Q), invu[u]).
   `evalF(X,Y,Z)`: Xp[0]=1, Xp[1]=X, Xp[k]=M(Xp[k-1],X) for k=2..d; same Y, Z; then for
   each monomial (i,j) in `mons(d)`, k = d-i-j: t = M(a[(i,j)], Xp[i]); t = M(t, Yp[j]);
   t = M(t, Zp[k]); sum += t. Cost exactly 3(d-1) + 3·|mons(d)| M (d = 5: 75; d = 4: 54).
4. coef[r] = Σ_{u} M(Vinv[r][u-1], cval[u]), r = 0..d-2 ((d-1)^2 M). n = d - 2.
5. If coef[n] == 0 or coef[0] == 0: `tangent_lines += 1`; next line.
6. Discriminant Δ:
   - n = 2 (a = coef[2], b = coef[1], c = coef[0]): Δ = M(b,b) - M(4, M(a,c)).
   - n = 3 (a = coef[3], b = coef[2], c = coef[1], e = coef[0]), exactly:
     ab=M(a,b); ce=M(c,e); bb=M(b,b); cc=M(c,c); aa=M(a,a); ee=M(e,e);
     Δ = M(18, M(ab,ce)) - M(4, M(M(bb,b), e)) + M(bb,cc) - M(4, M(a, M(cc,c))) - M(27, M(aa,ee))
     (17 M).
   - n = 4: c' = [M(i, coef[i]) for i = 1..4]; R = det(Sylvester(coef, c')) computed with
     counted Gaussian elimination (each pivot one INV, each elimination product one M);
     Δ_test = M(R, coef[4]) (same quadratic character as the discriminant).
   If Δ (Δ_test) == 0: `tangent_lines += 1`; next line.
   `eligible += 1`.
7. n = 2: if legendre(Δ) != 1: `nosplit += 1`; next line. Else s = sqrt_ts(Δ);
   i2a = INV(M(2, a)); roots = [M(-b + s, i2a), M(-b - s, i2a)] (mod p); `split += 1`.
   n ≥ 3: if legendre(Δ) != 1: `prefilter_reject += 1`; next line. `prefilter_pass += 1`.
   f = pmonic(coef); w = ppowmod([0,1], p, f) - [0,1]; h = pgcd(f, w);
   if deg h < n: `nosplit += 1`; next line. roots = roots_edf(h). `split += 1`.
8. For each root u: R = (M(u,X1)+X2, M(u,Y1)+Y2, M(u,Z1)+Z2); if R[2] != 0:
   iz = INV(R[2]); R = (M(R0,iz), M(R1,iz), 1) else ix = INV(R[0]); R = (1, M(R1,ix), 0).
   idx = index.get(R); if None or R is a node: `g1_fail += 1`, log, drop the line.
9. Relation pts = [a, b] + residual idx (d entries). Verify (uncounted, §6 G1/G2); a
   failing relation increments `g1_fail`/`g2_fail` and is dropped.
10. Classify: out = [i for i in residual idx if i not in inF].
    len 0 → full (`rel_full += 1`, store); 1 → `rel_1lp += 1`, edge (out[0], GROUND);
    2 → `rel_2lp += 1`, edge (out[0], out[1]); ≥ 3 → `rel_discard3 += 1`.
    Edges are stored with their relation.
11. Stop when rel_full + uf.total ≥ needed.

### 5.5 Union-find with parity (uncounted)
Vertex ids: point indices, GROUND = -1. Per vertex: parent, parity (to parent), size.
Per root: E, V (non-GROUND vertices), grounded, bip. New vertex: parent = self,
parity 0, E 0, V = 0 if GROUND else 1, grounded = (GROUND), bip = True.
contrib(r) = E - V + (1 if bip and not grounded else 0). `total` = Σ contrib over roots
(new vertices contribute 0). find(x) returns (root, parity of x to root) with path
compression that keeps parities. add_edge(x, y):
- same root r: total -= contrib(r); E += 1; if px == py: bip = False; total += contrib(r).
- different roots: total -= contrib(rx) + contrib(ry); attach the smaller-size root under
  the larger; parity[child root] = px ^ py ^ 1; E = Ex + Ey + 1; V summed; grounded =
  or; bip = and; size summed; total += contrib(new root).

### 5.6 Matrix and LA model (uncounted bookkeeping)
Contributing roots: roots with contrib > 0 at stop. Rows: all full relations + every
edge relation whose endpoint's root is contributing. cols = nb + Σ V over contributing
roots. Row weight = (number of its d point indices != P0) + 1 (κ column).
W = Σ row weights. la_fpm = 3 · cols · W.
FB arm: rows = full relations, cols = nb = N.

### 5.7 Arms per curve (in this order) and costs
Enumeration once per curve (counter E). Then FB; 2LP cF = 1, 2, 4 (rep 1); 2LP cF = 2
rep 2. Each arm its own counter C.
fpm_enum = fpm(E); fpm_rel = fpm(C); fpm_total = fpm_enum + fpm_rel + la_fpm;
fpm_total_kappa = fpm_enum + fpm_rel + g*g*la_fpm.
rho_fpm_393 = sqrt(pi·p^g/2)·393; rho_fpm_8900 = sqrt(pi·p^g/2)·8900; rho band
factors jlo = (√p-1)^(2g)/p^g, jhi = (√p+1)^(2g)/p^g (multiply rho by sqrt of these).

## 6. Verification

- G1 (per relation, plain arithmetic): for each of its d points P: F(P) == 0;
  L0·X + L1·Y + L2·Z == 0; (∂F/∂X, ∂F/∂Y, ∂F/∂Z)(P) != (0,0,0); P not a node.
- G2: the d indices are pairwise distinct.
- G3 (gate): from ≥ 400 FB relations, rng `random.Random(f"L08-gate-d{d}-m{m}-p{p}-c{c}")`,
  draw 4 distinct relations, keep if their point sets are pairwise disjoint (≤ 100000
  draws), until 200 quadruples. With Λ(L, P) = (L0X + L1Y + L2Z) mod p and S_i the d
  points of relation i:
  lhs = Π_{P∈S3} Λ(L1,P)/Λ(L2,P) · Π_{P∈S4} Λ(L2,P)/Λ(L1,P),
  rhs = Π_{P∈S1} Λ(L3,P)/Λ(L4,P) · Π_{P∈S2} Λ(L4,P)/Λ(L3,P). Pass iff lhs == rhs.
- G3-PC: same 200 quadruples; replace the first residual point of S1 by a point drawn
  with the same rng from all points not in S1∪S2∪S3∪S4 with Λ(L1,P) != 0; recompute
  rhs; count failures (must be ≥ 190).
- G4: split_frac = split / eligible; expected C(p,n)/p^n (n = d-2); report
  |diff|; pass iff |diff| ≤ 0.01 (only evaluated when p ≥ 1031 and eligible ≥ 20000,
  else `g4 = "n/a"`).
- G5 (gate, curve 0, 2LP cF = 2): incidence matrix, one row per stored edge, one column
  per large-prime vertex, entry 1 at each non-GROUND endpoint; rank over F_ℓ,
  ℓ = 2147483647, by Gaussian elimination. Pass iff uf.total == E_total - rank.

## 7. `gate` subcommand
For each p and curve index c < curves: generate, enumerate, run an FB collection with
needed overridden to max(400, needed) (still stops at exhaustion), compute G1, G2,
G3, G3-PC, G4 (report). For c = 0 also run 2LP cF = 2 and G5. One JSONL row per
(p, c): d, m, g, p, c, curve_seed, n_quads, g3_pass, g3pc_fail, g1_fail, g2_fail,
split_frac, g4, g5_total, g5_E, g5_rank, g5_pass, all_pass.

## 8. `run` subcommand output (one JSONL row per arm)
Keys: d, m, g, p, curve_index, curve_seed, seeds_tried, reject_counts, arm, cF, rep,
nb, N, needed, lines_tried, node_lines, tangent_lines, eligible, prefilter_pass,
prefilter_reject, nosplit, split, split_frac, rel_full, rel_1lp, rel_2lp,
rel_discard3, useful, exhausted, components_contributing, mat_rows, mat_cols,
mat_weight, la_fpm, enum_mul, enum_inv, rel_mul, rel_inv, fpm_enum, fpm_rel,
fpm_total, fpm_total_kappa, rho_fpm_393, rho_fpm_8900, jlo, jhi, g1_fail, g2_fail,
g4, python_version.

## 9. `analyze` subcommand
Input rows from `--glob`. Group by (d, m). Cell = (arm, cF, rep, p). Valid cell: 4 rows,
all exhausted == false, g1_fail == g2_fail == 0. Cell mean of each numeric key.
- Series(d,m) "A_2LP": per p, min over valid 2LP rep-1 cells of mean fpm_total
  (record the argmin cF). slope = OLS slope of ln(value) on ln(p) over the primes
  present (need ≥ 5 primes for d = 5 verdict; else verdict INCONCLUSIVE (coverage)).
  Also slope over the 4 largest primes; per-cF slopes; FB slope (all, 4 largest);
  AA: slope cF=2 rep1 vs rep2 over primes valid in both.
- Bootstrap: `random.Random("L08-boot")`, 2000 reps, per p draw 4 indices with
  replacement over the curve_index order, apply to all cells of that p, recompute
  min and slope; report 2.5 / 97.5 percentiles.
- Ratios per p for A_2LP: value / rho_fpm_393, value / rho_fpm_8900, and the band
  (divide by sqrt(jhi) and sqrt(jlo)).
- Verdict (only when both (5,2) and (4,0) present), PROTOCOL §8 order: any main-run
  g1/g2 failure → KILLED-verification; PC1 = d=4 A_2LP slope in [0.55, 1.25];
  PC2 = d=5 FB slope (4 largest) ≥ 1.75; AA |Δ| ≤ 0.10; any false → INCONCLUSIVE-instrument;
  e > 1.60 → KILLED-exponent; e ≤ 1.45 and hi ≤ 1.60 → SUPPORTED; else INCONCLUSIVE.
  For d = 6 files: slopes only, verdict "n/a (optional)".
- `--out` JSON (all of the above); `--table` TSV with header:
  `d m arm cF rep p n_valid mean_fpm_total mean_fpm_enum mean_fpm_rel mean_la_fpm mean_lines_tried mean_useful mean_needed mean_mat_rows mean_mat_cols mean_mat_weight mean_split_frac mean_node_lines mean_tangent_lines n_exhausted ratio_393 ratio_8900`.

## 10. Self-tests (`selftest`; expected values computed by hand)
1. p = 7: powmod(3, 5) = 5 with exactly 3 M.
2. p = 7: legendre(1..6) = [1, 1, -1, 1, -1, -1].
3. p = 7: cubic coef [1, 4, 1, 1] (= (u-1)(u-2)(u-3)): Δ = 4, exactly 17 M.
4. p = 7: rational_roots([1, 4, 1, 1]) sorted = [1, 2, 3].
5. p = 7: coef [5, 0, 0, 1] (u^3 - 2): Δ = 4 (square, prefilter passes); rational_roots = [].
6. p = 7: det(Sylvester([−1, 0, 1], [0, 2])) (A = y^2 − 1, B = 2y) = 3 mod 7 (= −4).
7. unrank(0..5) = (0,1), (0,2), (1,2), (0,3), (1,3), (2,3).
8. Union-find: edge a–b → total 0; 4-cycle a–b–c–d–a → 1; triangle → 0; two edges
   (a, GROUND) → 1; path a–b plus (a, GROUND), (b, GROUND) → 1 (each from a fresh UF).
9. Pair permutation with Pm = 10 (Mp = 11) visits each k in 0..9 exactly once.
10. sqrt_ts: p = 7, a = 2 → {3, 4}; p = 13, a = 10 → {6, 7}; p = 17, a = 2 → {6, 11}.
11. d = 5, m = 2, p = 131, s = 1: generated f has f = f_x = f_y = 0 at (0,0) and (1,1),
    a[(0,5)] != 0.
12. p = 13, d = 4, m = 0, f = x^4 + y^4 + 1: genus test rejects with `T_sqfree`
    (S = 256 (x^4+1)^3).
13. p = 131, d = 4, f = y^4 + x^3 y + y (= y(y^3 + x^3 + 1)): genus test rejects (any reason).
14. First admitted curve for (d,m,p) = (5,2,131) and (4,0,131) exists within 2000 seeds,
    g = 4 and 3.
15. evalF costs 75 M at d = 5 and 54 M at d = 4.
16. On the first admitted (4,0,131) and (5,2,131) curves: an FB collection of 50
    relations has g1_fail = g2_fail = 0, and each pts list has d entries.
17. N for those curves satisfies |N - (p+1)| ≤ 2g√p + 2m (smooth-point count sanity).
