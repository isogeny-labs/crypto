# L11 implementation specification

Implements `PROTOCOL.md` (same directory). Make no research choice that is not
written here; if something is missing or contradictory, stop and report it.

## 1. Files, language, CLI

Python 3 standard library only. All files in
`research/g2plus_leads_20261116/L11_laine_lauter_memory/`:

- `l11.py` — everything below; subcommands `gate-pic`, `curves`, `run`, `analyze`
  (argparse). Single thread.
- `test_l11.py` — unit tests of §11 (`unittest`), run as `python3 test_l11.py`.
- outputs only under `raw/`.

Common record fields added to every JSON output: `"python_version"`
(`sys.version.split()[0]`) and `"l11_sha256"` (sha256 of the bytes of `l11.py`).

## 2. Monomials and curve

`mons(d) = [(a, b, d-a-b) for a in range(d, -1, -1) for b in range(d-a, -1, -1)]`.
`M4 = mons(4)` (15), `M3 = mons(3)` (10), `M2 = mons(2)` (6). A curve is a list
`f` of 15 integers in `[0, p)`, `F(x,y,z) = sum f[i] x^a y^b z^c` over
`(a,b,c) = M4[i]`. Points are tuples `(x, y, z)` normalised so that the first
non-zero coordinate is 1. `F_eval(P)` = plain modular evaluation (uncounted).

Curve-level tables (uncounted, built once per curve):

- `GT[k][m]` (k = 0,1,2 for x,y,z; m indexes M3): for each `i`, `(a,b,c) = M4[i]`,
  and each `k` with exponent `e_k > 0`: `GT[k][M3.index(exponents with e_k-1)] += f[i]*e_k (mod p)`.
- `HT[s][m]` (s indexes M2 = exponent `e` of `Q`; m indexes M2 = exponent of `P`):
  for each `i`, each `s` with `M4[i] >= M2[s]` componentwise:
  `HT[s][M2.index(M4[i]-M2[s])] += f[i]*C(a,ea)*C(b,eb)*C(c,ec) (mod p)`
  (`math.comb`).

## 3. Randomness

```
M64 = 2**64 - 1
def mix64(z):                     # SplitMix64 finaliser
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)
class SM:
    def __init__(self, seed): self.s = seed & M64
    def next(self):
        self.s = (self.s + 0x9E3779B97F4A7C15) & M64
        return mix64(self.s)
def shuffle(lst, sm):              # in place Fisher-Yates
    for i in range(len(lst) - 1, 0, -1):
        j = sm.next() % (i + 1)
        lst[i], lst[j] = lst[j], lst[i]
```

Seeds:
- `s_curve(k, i) = 0x4C31310000000000 ^ (k << 32) ^ i`
- `seed_base = mix64(s_curve ^ 0xBA5E000000000000 ^ (b << 8) ^ salt)`
- `seed_pair = mix64(s_curve ^ 0x9A12000000000000 ^ (b << 8) ^ salt)`
- `seed_g1 = mix64(seed_pair ^ 0x5A3)` (G1 sampling)
- `seed_root(x0) = mix64((p << 32) ^ x0 ^ 0xE7)` (root splitting; `x0 = p` for the
  line at infinity)
- `salt = 0` for all arms except `UAA` (`salt = 0xAA`).
- `adm(R, alpha) = mix64(((x << 42) | (y << 21) | z) ^ s_curve) < floor(alpha * 2**64)`
  for `R = (x, y, z)`; `alpha = 1` admits everything (compare with `2**64`).

## 4. Smoothness test (uncounted)

Partial derivatives `Fx, Fy, Fz` as cubic coefficient vectors over M3
(`GT[0], GT[1], GT[2]` with `f` itself: the coefficient of M3[m] in `F_k` is
`GT[k][m]`). For a cubic `G` and `x0` define `G(x0, y, 1)` as a polynomial in `y`
with coefficient of `y^j` = `sum over m with M3[m][1] == j of GT_coeff[m] * x0^(M3[m][0])`.

Affine part (z = 1), up to three attempts `t = 0, 1, 2`:
1. `sm = SM(mix64((p << 8) ^ t ^ 0x53))`; draw `l1, m1, l2, m2, l3, m3`
   (`next() % p` each, in that order). Cubics `Gq = Fx + lq*Fy + mq*Fz`
   (coefficient vectors over M3), q = 1, 2, 3.
2. `R1(x0) = det Syl(G1(x0,y,1), G2(x0,y,1))`, `R2(x0) = det Syl(G1(x0,y,1), G3(x0,y,1))`
   where `Syl(A, B)` for `A = a3 y^3 + a2 y^2 + a1 y + a0` (formal degree 3, even if
   `a3 = 0`) is the 6x6 matrix with rows `[a3,a2,a1,a0,0,0]`, `[0,a3,a2,a1,a0,0]`,
   `[0,0,a3,a2,a1,a0]` and the same three rows for `B`. Determinant mod `p` by
   Gaussian elimination. Evaluate at `x0 = 0..9`, Lagrange-interpolate `R1, R2`
   (degree `<= 9`) over `F_p`.
3. The attempt succeeds if `R1 != 0`, `R2 != 0` and `deg gcd(R1, R2) == 0`.
   The affine part is smooth iff some attempt succeeds; otherwise reject.
   (A singular affine point `(x0, y0, 1)` makes every attempt fail, because
   `x0` is a root of both resultants; random combinations avoid the identically
   zero formal resultant that `Res(Fx, Fz)` gives for the Fermat quartic.)
4. Line at infinity: reject if `Fx(1,0,0) = Fy(1,0,0) = Fz(1,0,0) = 0`. Let
   `u_k(x) = F_k(x, 1, 0)` (univariate, degree <= 3). Reject if all three are zero,
   or if `deg gcd(u_x, u_y, u_z) >= 1` (gcd of the non-zero ones).
5. Otherwise smooth.

## 5. Point enumeration (uncounted)

Polynomials over `F_p` as coefficient lists, low degree first, with `trim`,
`monic`, `divmod`, `gcd` (monic result; `gcd(0, g) = monic(g)`), `mulmod`, `powmod`.

`roots(g, x0)`: error if `g` is zero; `[]` if `deg g == 0`; `g = monic(g)`;
`h = gcd(g, powmod([0,1], p, g) - [0,1])`; return `sorted(split(h))` where
`split(h)`: `deg 0 -> []`; `deg 1 -> [(-h[0]) % p]`; else using
`sm = SM(seed_root(x0))` (one stream per `x0`, shared by the recursion in call
order) repeat: `d = sm.next() % p`;
`w = gcd(h, powmod([d,1], (p-1)//2, h) - [1])`; if `0 < deg w < deg h`:
return `split(w) + split(h // w)`.

Points: for `x0 in 0..p-1`, for `y in roots(F(x0, y, 1) as poly in y, x0)`: `(x0, y, 1)`
normalised (`x0 == 0` gives `(0, y, 1)` normalised to `(0, 1, z')` if `y != 0`, so
always normalise). Infinity: for `x in roots(F(x, 1, 0), p)`: `(x, 1, 0)`
normalised; plus `(1, 0, 0)` if `f[M4.index((4,0,0))] == 0`. Deduplicate,
sort lexicographically. `N1 = len(points)`. Assert Hasse-Weil
`|N1 - p - 1| <= 6 * sqrt(p)`.

## 6. Curves, base, pair order

`curves --k K [--curves a-b]`: `p = p_K` (263, 1031, 4099, 16411). For each
`i`: `sm = SM(s_curve(K, i))`; candidate `j = 0, 1, ...`: `f = [sm.next() % p for _ in range(15)]`;
accept if smooth (§4) and `N1 >= b_max + 10`,
`b_max = max(ceil(3*sqrt(8*p)), ceil(2*p**(2/3)))`. Write points to
`raw/points_k{K}_{i}.json` (`{"p":..,"f":..,"points":[[x,y,z],...]}`) and a
line to `raw/curves_k{K}.jsonl`: `k, p, i, s_curve, attempts, f, N1, points_file,
points_sha256`. Base sizes are exact integer rules (no floating point):
U2/U/U4/UAA/AF/G: smallest `b` with `b*b >= c*c*p`, `c in {2, 3, 4}`;
AS with `alpha = 1/D`: smallest `b` with `b*b >= 9*p*D`; S: smallest `b` with
`b**3 >= 8*p*p`. `b_max = max(smallest b with b*b >= 72*p, smallest b with
b**3 >= 8*p*p)`.

Base: `pts = sorted points`; `order = list(range(N1))`; `shuffle(order, SM(seed_base))`;
`base = [pts[t] for t in order[:b]]`; `P0 = base[0]`. Columns: `base[1..b-1]`
-> `0..b-2`, `h` -> `b-1`.

Pairs: `NP = b*(b-1)//2`; `idx = list(range(NP))`; `shuffle(idx, SM(seed_pair))`.
Pair index `t -> (i, j)`, `i > j`: `i = (1 + isqrt(1 + 8t)) // 2`; while
`i*(i-1)//2 > t: i -= 1`; while `(i+1)*i//2 <= t: i += 1`; `j = t - i*(i-1)//2`.
Line trial `n` uses `Pi = base[i]`, `Pj = base[j]`.

## 7. Counted arithmetic and the line routine

```
class Fp:   # one instance per run; counters mul, inv
    mul(a,b): self.mul += 1; return a*b % p          # squarings too
    inv(a):   self.inv += 1; return pow(a, p-2, p)    # internal products not counted
    powc(a,e): r = a; for bit in bin(e)[3:]: r = mul(r,r); if bit=='1': r = mul(r,a); return r
```
Additions/subtractions are plain `% p`, uncounted. Never skip a multiplication
because an operand is 0 or 1.

`taylor(P)` (counted, 82 mul, cached per base point; counts go to `mul_setup`):
- `v2[s] = mul(P[u], P[v])` for `M2[s]` written as the product of coordinates
  `u <= v` (x=0,y=1,z=2): 6 mul.
- `v3[m] = mul(v2[M2.index(M3[m] - unit(c))], P[c])` with `c` the first
  coordinate with positive exponent in `M3[m]`: 10 mul.
- `g[k] = sum_m mul(GT[k][m], v3[m])` for k=0..2: 30 mul.
- `hq[s] = sum_m mul(HT[s][m], v2[m])` for s=0..5: 36 mul.
- store `(g, hq, v2)`.

`line(Pi, Pj)` with `(gi, hi, _) = taylor(Pi)`, `(gj, _, v2j) = taylor(Pj)`:
```
c1 = sum_k mul(gi[k], Pj[k]);  c3 = sum_k mul(gj[k], Pi[k])        # 6
if c1 == 0 or c3 == 0: return TANGENT
c2 = sum_s mul(hi[s], v2j[s])                                       # 6
disc = mul(c2,c2) - mul(mul(c1,c3), 4)                              # 3
if disc == 0: return TANGENT
d = powc(disc, (p+1)//4)
if mul(d, d) != disc: return NOSPLIT                                 # 1
mu = 2*c1; l1 = d - c2; l2 = -c2 - d            (mod p, uncounted)
mq = [mul(mu, Pj[k]) for k]                                          # 3
for l in (l1, l2): rr = [mul(l, Pi[k]) + mq[k]]                      # 3 each
   i0 = first k with rr[k] != 0 (none -> return BAD)
   iv = inv(rr[i0]); R[i0] = 1; R[k] = mul(rr[k], iv) for the other two k   # 1 inv + 2
return SPLIT(R1, R2)
```
Counts: tangent 6 or 15; non-split `16 + S`; split `29 + S` mul and 2 inv, with
`S = (bitlen(e) - 1) + (popcount(e) - 1)`, `e = (p+1)//4`. Line counts go to
`mul_lines`, `inv_lines`. `FPM_x = mul_x + 10*inv_x`.

## 8. Relation collection (`run`)

Arms (`b` rule, admission rule):

| arm | b | admission |
| --- | --- | --- |
| U2, U, U4 | c = 2, 3, 4 | all |
| UAA | c = 3, salt 0xAA | all |
| AF1, AF2, AF4, AF8 | c = 3 | `alpha = 1, 1/2, 1/4, 1/8`; accept iff every large prime is `adm` |
| AS2, AS4, AS8 | `b*b >= 9*p*D`, D = 2, 4, 8 | as AF with `alpha = 1/D` |
| G | c = 3 | `nlp <= 1`: accept; `nlp == 2`: accept iff `R1 in vid or R2 in vid` |
| S | `b**3 >= 8*p*p` | accept iff `nlp <= 1` |

State: `base_set` (set of base points), `vid` (dict point -> node id), DSU arrays
`parent, verts, edges` with node 0 = "1" (`verts[0]=1, edges[0]=0`), `Fr = 0`,
`dedup = set()`, `stored = []` (relations `(P1, P2, R1, R2, nlp, u, v)`),
`target = b + 8 + b // 100`.

Loop over `t in idx` (in shuffled order):
1. `T += 1`; `res = line(Pi, Pj)`; census `tangent/nosplit/bad/split`. `BAD` is
   counted and the run fails gate G1 if any occurs. Non-SPLIT: continue.
2. `lps = [R for R in (R1, R2) if R not in base_set]`, `nlp = len(lps)`; census
   `full/lp1/lp2`. For AF arms with `alpha < 1`, add each `R` in `lps` to the
   census set `seen_lp` and count it as admissible or not on first sight (not
   memory; instrument only).
3. Admission (table). Rejected: `rejected_admission += 1`; continue.
4. If `nlp < 2`: `key = tuple(sorted([Pi, Pj, R1, R2]))`; if `key in dedup`:
   `dup += 1`; continue; else `dedup.add(key)`.
5. Store: `nlp == 0`: `Fr += 1`. Else `u = node(lps[0])`, `v = node(lps[1])` if
   `nlp == 2` else `0` (`node` creates a DSU node on first sight); `E += 1`;
   DSU edge `(u, v)`: `ru, rv = find(u), find(v)`; equal -> `edges[ru] += 1`; else
   union by `verts` (larger root wins, tie -> smaller id wins), `verts` summed,
   `edges = edges[ru] + edges[rv] + 1`. `find` uses path halving; count each
   parent step in `find_steps`. Count each `vid`/`base_set`/`dedup` lookup in
   `dict_lookups`. Append to `stored`.
6. `r0 = find(0)`; `count = Fr + edges[r0] - verts[r0] + 1`. If `count >= target`:
   stop.

End: `exhausted = (count < target)`. `V = len(vid) + 1`, `D = len(dedup)`,
`lp_records = V + E`, `mem_total = V + E + Fr + D + b`. `rng_draws` = SM draws.

Rows (all runs, uncounted): BFS from node 0 over stored edges (adjacency in
storage order; FIFO queue). Each relation has `A_e = {h: +1}` plus `-1` for each
base point among its four points except `P0` (coefficients added, so a repeated
column accumulates). `expr[0] = {}`; tree edge from `u` to new `w`:
`expr[w] = A_e - expr[u]`. Every stored edge reached by the BFS that is not a
tree edge gives the row `expr[u] + expr[v] - A_e` (zero entries removed; a
1LP edge has `v = 0`). Full relations give the row `{x of each base point != P0: +1, h: -1}`.
Record `cycle_rows`, `rows`, `zero_rows`, `nnz` (total non-zero entries),
`b_nnz = b * nnz`. Gate G3: `cycle_rows == count - Fr`.

Rank (only `k <= 10`): over `F_l`, `l = 2**61 - 1`, rows as dicts; pivots dict
`col -> normalised row (leading coeff 1)`; for each row: repeat `c = min(row)`;
if `c in pivots`: `row -= row[c] * pivots[c]` (mod l, drop zeros) else normalise
and insert, break; until row empty. `rank = len(pivots)`.

G1 (§7 of protocol): `k <= 10`: all of `stored`; else 200 indices
`SM(seed_g1).next() % len(stored)`. Check: 4 distinct points, `F_eval == 0` for
each, `det[Pi; Pj; R] == 0 (mod p)` for `R1` and `R2`. Record `g1_checked`,
`g1_failures`.

Command `run --k K --arms LIST|all [--curves a-b] --out FILE`: loops curves
outer, arms inner in the order U, U2, U4, UAA, AF1, AF2, AF4, AF8, AS2, AS4, AS8,
G, S (filtered by LIST), reading `raw/curves_k{K}.jsonl` and the points files
(verify `points_sha256`); appends one JSON line per run (§10) and flushes.

## 9. `gate-pic --p P` (P in {103, 127})

1. Curve: Fermat `f` with 1 at `(4,0,0), (0,4,0), (0,0,4)`, 0 elsewhere. Smooth
   test must pass; points by §5; assert `N1 == P + 1`.
2. `N2` over `F_{P^2} = F_P[i]/(i^2+1)`: elements `(a, b)`; `fourth = w^4` for
   all `P^2` elements; `cnt[w4] += 1`. Affine: `sum over x of cnt[-(x^4 + 1)]`;
   infinity: `cnt[-1]` (points `(x:1:0)`); `(1:0:0)` is not on the curve.
   `N2 = affine + infinity`; must equal `P^2 + 1 + 6P`. Then `jac = (P+1)^3`.
3. Relations: `taylor` on every point; for all pairs `i > j` of sorted points,
   `line(pts[i], pts[j])`; on SPLIT dedup by sorted 4-tuple; integer row of
   length `n = N1`: `+1` in the column of each point except `P0 = pts[0]`
   (columns `pts[1..]` -> `0..n-2`), `-1` in column `n-1` (`h`). Run the G1 check on
   every relation.
4. Index by integer echelon (Python ints):
   ```
   M = rows; r = 0; piv = []
   for c in range(n):
       loop: nz = [i for i in range(r, len(M)) if M[i][c] != 0]; if not nz: break
             i0 = argmin over nz of |M[i][c]|; swap M[r], M[i0]
             for i in range(r+1, len(M)): if M[i][c]: q = M[i][c] // M[r][c]; M[i] -= q*M[r]
             if all M[i][c] == 0 for i > r: break
       if any M[i][c] != 0 for i >= r: piv.append(abs(M[r][c])); r += 1 else piv.append(0)
   rank = count of nonzero piv; index = prod(piv) if rank == n else None
   ```
5. Output JSON: `p, N1, N2, N2_expected, jac, n, relations, rank, index, pass`
   (`pass = N2 == N2_expected and index == jac and g1_failures == 0`).
   Expected at P=103 (scratch design check): `n = 104`, 420 distinct relations,
   `index = 1124864`.

## 10. Output formats

Run line (`raw/runs_*.jsonl`), keys exactly:
`lead("L11"), k, p, curve, s_curve, arm, alpha (float or null), b, salt, target,
T, supply (=NP), exhausted, mul_setup, inv_setup, mul_lines, inv_lines,
FPM_setup, FPM_lines, FPM_total, count, Fr, V, E, D, lp_records, mem_total,
census{tangent, nosplit, bad, split, full, lp1, lp2, rejected_admission, dup},
adm_seen, adm_admissible (null unless AF alpha<1), rows, cycle_rows, zero_rows,
nnz, b_nnz, rank (null if k > 10), g1_checked, g1_failures,
unpriced{dict_lookups, find_steps, rng_draws}, python_version, l11_sha256`.

`analyze --in GLOB --out raw/analysis.json --tsv raw/summary.tsv`:
- Load runs; abort with a message if any (k, arm, curve) is duplicated.
- Gates: G1 (sum of failures = 0, no `bad`), G3 (identity, zero_rows <= 1% of rows
  per run), G4 (per arm pooled k = 8, 10: fraction with `rank >= b-1` >= 0.95),
  G5 (AF1 == U on T, FPM_total, mem_total per curve), G6 (per AF arm and k:
  `|adm_admissible/adm_seen - alpha| <= 4*sqrt(alpha*(1-alpha)/adm_seen)` with
  pooled sums). Also read `raw/gate_pic_p103.json`, `raw/gate_pic_p127.json` (G2)
  and require `raw/unit_tests.txt` to end with `OK` (G0).
- Per cell (k, arm): `n`, `n_exhausted`, geometric means of `T, FPM_total,
  mem_total, lp_records, nnz, b_nnz`; paired ratios to U on the same curve for
  `T`, `FPM_total`, `mem_total`: GM and 95% bootstrap (2000 resamples; one
  `SM(0x5EED)` stream for the whole analysis, consumed in the order cells are
  listed in the TSV; each resample draws `n` indices `next() % n`; interval = the
  sorted values at positions 50 and 1949).
- Exponents per arm: OLS slope of `ln(metric)` on `ln p` over all per-curve values
  k = 8..14 (`T`, `FPM_total`, `mem_total`); bootstrap: per resample draw 12
  curve indices per k (same indices for every arm in that resample), 2000
  resamples, stream `SM(0x5EED ^ 1)`; report point, interval, and for each arm
  the difference to U's exponent with its interval from the same resamples.
- Elasticity at k = 14: OLS slope of `ln GM(T)` on `ln GM(mem_total)` over arms
  U, AS2, AS4, AS8; bootstrap with `SM(0x5EED ^ 2)` resampling curve indices
  (same for all four arms).
- Verdicts exactly per PROTOCOL §8 (instrument validity, H_brief/K_brief,
  H_C4, H_C5), each with the numbers that decided it.
- TSV header:
  `k	arm	b	n	n_exhausted	gm_T	gm_FPM	gm_mem	gm_lp	r_T	r_T_lo	r_T_hi	r_F	r_F_lo	r_F_hi	r_M	r_M_lo	r_M_hi	gm_nnz	gm_b_nnz`
  (`r_T` prefixed with `>=` when `n_exhausted > 0`).

## 11. Unit tests (`test_l11.py`, expected values computed by hand/scratch)

1. `SM(0).next()` twice: `0xE220A8397B1DCDAF`, `0x6E789E6AA1B965F4`.
2. Pair map: `t = 0,1,2,3,4,5 -> (1,0),(2,0),(2,1),(3,0),(3,1),(3,2)`; and for
   `b = 50` the map is a bijection onto `{(i,j): 0 <= j < i < 50}`.
3. Fermat quartic, `p = 31`: `N1 = 32`; first sorted points
   `(1,2,7),(1,2,24),(1,5,6),(1,5,25)`; `taylor` costs exactly 82 mul.
   `line((1,2,7),(1,5,25))` -> SPLIT `{(1,9,18),(1,19,16)}` with exactly 32 mul and
   2 inv; `line((1,2,7),(1,7,2))` -> SPLIT `{(1,18,22),(1,22,18)}`, 32 mul, 2 inv;
   `line((1,2,7),(1,2,24))` and `line((1,2,7),(1,5,6))` -> NOSPLIT, 19 mul, 0 inv.
4. Fermat at `p = 263`: smooth, `N1 = 264`; at `p = 31`, brute-force `N2` via §9
   step 2 equals `1148`.
5. Smoothness: `y^2 z^2 - x^4 - x^2 z^2` at `p = 263` is rejected (node at (0:0:1)).
6. Enumeration equals brute force (all `(x,y,1)`, `(x,1,0)`, `(1,0,0)`) for the
   accepted curve `(k=8, i=0)` and for Fermat at `p = 263`.
7. Line routine vs brute force: on curve `(k=8, i=0)`, for 300 random base pairs
   (stream `SM(1)`), SPLIT residuals equal the brute-force set of curve points on
   the line minus `{Pi, Pj}` when that set has 2 points and the line is not
   tangent; TANGENT/NOSPLIT otherwise consistent.
8. DSU count: edges `(1,0),(1,0)` -> count 1; edges `(1,0),(2,1),(2,0)` -> 1;
   edges `(1,2)` only -> 0 (not in component of 0).
9. Index: rows `[[2,0],[0,3]]` -> 6; `[[1,1],[1,-1]]` -> 2; `[[1,0],[1,0]]` -> None.
10. Arm `AF1` and `U` on curve `(k=8, i=0)` give identical run records except
    `arm` and `alpha`.
11. `G` on a toy stream: an edge with both endpoints unseen is rejected; after
    edge `(R,"1")`, an edge `(R, R')` is accepted.
12. Rank: rows `{0:1,1:1}, {0:1,1:-1}` -> 2; `{0:1}, {0:2}` -> 1.

`python3 test_l11.py` must print the unittest summary ending in `OK`.
