# L01 implementation specification (frozen with PROTOCOL.md)

Implement exactly this. Where this file is silent, choose the simplest code that
changes no count, and write the choice in `IMPLEMENTATION_NOTES.md`. Do not change
any threshold, prime, seed, count rule or output field. If a gate fails, stop and
report it; do not "fix" a count to make a gate pass.

## 1. Language and layout

Python 3 standard library only (no numpy/sympy). One thread. All files inside
`/home/user/crypto/research/g2plus_leads_20261116/L01_anchored_pencil_sieve/`.

```
l01/__init__.py        empty
l01/rng.py             mix64, SM64, stream_seed                    (section 3)
l01/field.py           Counter, F (counted field ops), batch_inv   (section 4)
l01/instance.py        random_f, squarefree, Tables, base, curve selection, N_k, jac_order, split_order (3, 9)
l01/curve.py           Curve: counted Cantor (Fk semantics), key_of, mul_ltr, point, random_divisor (5)
l01/native.py          disc_character, ModU, splits_completely, roots_of, vrem, vgcd_monic, vdiv_monic, horner_nat (6)
l01/pencil.py          anchors, interpolate, fd stepping, sieve_tuple, naive_tuple, p_full, p_naive1, p_2lp (7)
l01/baseline.py        b_lab, b_2lp, b_labelled (8)
l01/rows.py            canonical key, row building, rows file io, DSU (7.5, 8.4)
l01/verify.py          V1, V2 (uncounted) (7.7)
l01/linalg.py          sparse_eliminate (counted), solve_mod_r (dense, uncounted) (9)
l01/gate.py            the tiny DLP gate (9)
run.py                 CLI: gate | relphase | lp2 | la (10)
analyze.py             decision analysis (10.5)
tests/test_l01.py      unittest suite (section 11)
IMPLEMENTATION_NOTES.md  (written by the implementer: deviations, none expected)
```

## 2. Conventions

- Polynomials over F_p: Python lists of ints in `[0, p)`, **lowest degree first**,
  trimmed (no trailing zeros); the zero polynomial is `[]`. `deg([]) = -1`.
- Monic Mumford divisor: pair `(u, v)` of such lists, `u` monic, `deg v < deg u <= g`.
  Identity: `([1], [])`. Store divisors as tuples `(tuple(u), tuple(v))` (hashable).
- `p` odd prime, `g` in {2, 3}. `M64 = 2^64 - 1`, `GOLD = 0x9E3779B97F4A7C15`.
- `eps(y) = +1 if 1 <= y <= (p-1)//2 else -1` (y != 0).
- JSON output: one object per line, `json.dumps(obj, sort_keys=True)`.

## 3. RNG and instances

3.1 `mix64(z)`: `z=(z+GOLD)&M64; z=((z^(z>>30))*0xBF58476D1CE4E5B9)&M64;
z=((z^(z>>27))*0x94D049BB133111EB)&M64; return z^(z>>31)`.

`class SM64(state)`: `next()`: `state=(state+GOLD)&M64; return mix64(state)`.
`below(n)`: `next() % n` (n >= 1). `bit()`: `next() & 1`.

3.2 `random_f(p, g, s)`: `st = ((s*GOLD)&M64) ^ p ^ (g<<48)`; repeat `2g+1` times:
`st=(st+GOLD)&M64; f.append(mix64(st) % p)`; then `f.append(1)`. (Lab rule,
`genus_ladder.rs::random_f`.)

`squarefree(f, p)`: `gcd(f, f')` (plain uncounted Euclid with modular inverses) has
degree 0.

Relation-phase curve index c (1-based): iterate `s = 1, 2, 3, ...`, keep `s` with
`random_f(p,g,s)` squarefree; the c-th kept `s` is `curve_seed`. Record it.

3.3 Streams: `stream_seed(tag, g, p, cs, ss) = mix64(mix64(mix64(tag ^ (g<<56)) ^ p) ^ (cs<<20) ^ ss)`
(all `& M64`), `SM64(stream_seed(...))`. Tags:
`0x5046` P_full anchors, `0x504E` P_naive1 roots, `0x5032` P_2lp anchors,
`0x424C` B walk (setup + restarts), `0x4252` B roots, `0x4754` gate (all gate
randomness, one stream, `ss = 0`). P_naive1 uses the P_full anchors (tuple 0).

3.4 Tables (setup counter, section 4): `sq = [-1]*p; sq[0]=0`; for `y in 1..(p-1)//2`:
`a = F.mul(y, y)`; `sq[a] = y`. `chi[a] = 0 if a==0 else (1 if sq[a]>0 else -1)`.
`inv2 = (p+1)//2` (uncounted constant).

Base: for `x in 0..p-1`: `fx[x] = horner_F(f, x)` (counted in setup: `2g+1`
multiplications, see 4.3). `B = [x : chi[fx[x]] == 1]` ascending, `col[x] =`
index in `B` (else -1); `n = len(B)`; `Z = [x : fx[x] == 0]`.
`N_target = n + 8 + n//100`.

## 4. Counted arithmetic

4.1 `class Counter`: integer fields `mul, inv, add, fd_add, bucket_ops, bucket_scan,
lookups`. `fpm(I=10) = mul + I*inv`. Counters are never reset; arms take deltas
(snapshot before, subtract after).

4.2 `class F(p, ctr)`: `mul(a,b)` -> `ctr.mul+=1; a*b%p`; `sqr(a)` -> `ctr.mul+=1`;
`inv(a)` -> assert `a%p != 0`; `ctr.inv+=1`; `pow(a, p-2, p)`; `add/sub/neg` ->
`ctr.add+=1`. Table reads are free unless stated (`lookups` is informational).

4.3 `horner_F(c, x)` (c lowest-first, `len(c) = m >= 1`): `acc = c[m-1]`; for
`i = m-2 .. 0`: `acc = F.add(F.mul(acc, x), c[i])`. Exactly `m-1` multiplications.
(The lab charges `m`; documented deviation, tiny.)

4.4 `batch_inv(vals)` (Montgomery, all non-zero, `k = len(vals) >= 1`):
`pre[0]=vals[0]; pre[i]=F.mul(pre[i-1], vals[i])` for `i=1..k-1`;
`t = F.inv(pre[k-1])`; for `i = k-1 .. 1`: `out[i] = F.mul(t, pre[i-1]);
t = F.mul(t, vals[i])`; `out[0] = t`. Count: `3(k-1)` mul, 1 inv.

## 5. Curve arithmetic (transcription of `genus_ladder.rs` `Fk` and `JCurve`)

`Curve(p, g, f)` owns a `Counter` (`cantor_ctr`): only `mul` and `inv` are ticked
here (additions are free in Cantor, as in the lab). A second `Curve` object built
on a throw-away Counter is used for verification and the gate.

Poly helpers (`k_*`), with `tick(n)` = `cantor_ctr.mul += n`:
- `k_add(a,b)`, `k_sub(a,b)`, `k_neg(a)`: coefficientwise mod p, trimmed, no tick.
- `k_mul(a,b)`: `[]` if either is `[]`; schoolbook; `tick(len(a)*len(b))`; trim.
- `k_scale(a,s)`: `tick(len(a))`; trim.
- `k_inv(a)`: `cantor_ctr.inv += 1`; `pow(a, p-2, p)`.
- `k_divrem(a,b)`: assert `b != []`. If `len(a) < len(b)`: return `([], a)`.
  `monic = (b[-1] == 1)`; `li = 1 if monic else k_inv(b[-1])`; `db = deg b`;
  `r = list(a)`; `q = [0]*(len(a)-len(b)+1)`. While `r != []` and `len(r) >= len(b)`:
  `shift = deg r - db`; `coef = r[-1]` if monic else (`tick(1)`; `r[-1]*li % p`);
  `q[shift] = coef`; for `j < db`: `r[shift+j] = (r[shift+j] - coef*b[j]) % p`;
  `tick(db)`; set `r[shift+db] = 0`; trim r. Trim q. Return `(q, r)`.
- `k_rem(a,b) = k_divrem(a,b)[1]`.
- `k_monic(a)`: `a` if `a == []` or `a[-1] == 1`, else `k_scale(a, k_inv(a[-1]))`.
- `k_xgcd(a,b)`: `r0,r1=a,b; s0,s1=[1],[]; t0,t1=[],[1]`; while `r1 != []`:
  `q,r = k_divrem(r0,r1); r0,r1 = r1,r; s = k_sub(s0, k_mul(q,s1)); s0,s1 = s1,s;
  t = k_sub(t0, k_mul(q,t1)); t0,t1 = t1,t`. If `r0 == []` or `r0[-1] == 1`: return
  `(r0,s0,t0)`; else `li = k_inv(r0[-1])`; return `(k_scale(r0,li), k_scale(s0,li), k_scale(t0,li))`.

`cantor(A, B)`: if A is identity return B; if B is identity return A.
`(u1,v1),(u2,v2)`; `(d1,e1,e2) = k_xgcd(u1,u2)`. If `len(d1) == 1`:
`u = k_mul(u1,u2); t = k_mul(k_mul(e1,u1), k_sub(v2,v1)); v = k_rem(k_add(v1,t), u)`.
Else: `(d,c1,c2) = k_xgcd(d1, k_add(v1,v2)); s1 = k_mul(c1,e1); s2 = k_mul(c1,e2);
(u, ur) = k_divrem(k_mul(u1,u2), k_mul(d,d))` (assert `ur == []`);
`t1 = k_mul(k_mul(s1,u1),v2); t2 = k_mul(k_mul(s2,u2),v1);
t3 = k_mul(c2, k_add(k_mul(v1,v2), f)); (vq, vr) = k_divrem(k_add(k_add(t1,t2),t3), d)`
(assert `vr == []`); `v = k_rem(vq, u)`. Return `reduce(u, v)`.

`reduce(u, v)`: while `deg u > g`: `t = k_sub(f, k_mul(v,v)); (q, r) = k_divrem(t, u)`
(assert `r == []`); `un = k_monic(q); v = k_rem(k_neg(v), un); u = un`.
`u = k_monic(u)`; return `(tuple(u), tuple(v))`.

`negate((u,v)) = (u, k_neg(v))`. `point(x, y) = (((p-x)%p, 1), (y,) if y else ())`
(assert `y*y % p == f(x)`). `is_identity(D) = (D[0] == (1,))`.

Group-op tally (attribute `ops = {"add":0, "dbl":0}` on the Curve, reset never,
deltas taken): `cantor(A,B)` with both non-identity increments `dbl` if `A == B`
else `add`.

`mul_ltr(D, k)` (k >= 1): `acc = identity`; for bit `i` from `k.bit_length()-1`
down to 0: `acc = cantor(acc, acc)`; if bit set: `acc = cantor(acc, D)`. Return acc.

`key_of(D)`: `h = 0x243F6A8885A308D3 ^ deg`; for `i < deg`: `vi = v[i] if i < len(v) else 0`;
`h = mix64(h ^ ((u[i] << 32) | vi))` (`& M64`). Return h.

`random_divisor(rng)` (rng an SM64): `D = identity; placed = 0`; while `placed < g`:
`h = rng.next(); x = h % p; fxv = fx[x]`; if `chi[fxv] == -1`: continue;
`s = sq[fxv]`; `y = (p - s) % p if (h >> 63) & 1 else s`; `D = cantor(D, point(x,y))`;
`placed += 1`. Return D.

## 6. Native relation-phase routines (transcription; counters on a `Counter` passed as `n`)

Inversions inside these routines (`cinv`) are counted as `n.inv += 1` (lab counts
Fermat multiplications; documented, conservative for the pencil).

- `disc_character(p, u, chi, n)` (u monic, `d = deg u >= 2`): exactly as
  `genus_ladder.rs::disc_character`: `a = u[:]`; `b = [u[i]*i % p for i in 1..d]`,
  `n.mul += d`, trim b; `sign = chi[p-1] if (d(d-1)/2) odd else 1`. Loop: if `b == []`
  return 0; `m = deg a; nn = deg b`; if `nn == 0`: return `sign*chi[b[0]]` if m odd
  else `sign`. `lb = b[nn]; delta = m - nn + 1; r = a[:]`; for `step in 0..delta-1`:
  `top = m - step; c = r[top]`; for `i < top`: `r[i] = r[i]*lb % p`; for `j < nn`:
  `r[top-nn+j] = (r[top-nn+j] - c*b[j]) % p`; `n.mul += top + nn; n.add += nn`;
  truncate r to length `top`. Trim r; if `r == []` return 0; `dr = deg r`;
  if `(m + dr + delta*nn)` odd: `sign *= chi[lb]`; if `(m*nn)` odd: `sign *= chi[p-1]`;
  `a, b = b, r`.
- `ModU(p, u)`, `d = deg u`, residues are length-d lists.
  `mul(a,b)`: `t = [0]*(2d)`; `t[i+j] += a[i]*b[j]` (mod p), `n.mul += d*d; n.add += d*d`;
  for `k = 2d-2 down to d`: `c = t[k]`; for `j < d`: `t[k-d+j] = (t[k-d+j] + (p-u[j])*c) % p`;
  `n.mul += d; n.add += d`. Return `t[:d]`.
  `mul_xpa(a, aa)`: `top = a[d-1]; out = [0] + a[:d-1]`; for `j < d`:
  `out[j] = (out[j] + (p-u[j])*top + aa*a[j]) % p`; `w = d if aa == 0 else 2d`;
  `n.mul += w; n.add += w`.
  `base(aa)`: if `d == 1`: `[(aa + p - u[0]) % p]` else `[aa, 1, 0, ...]` (length d).
  `pow_xpa(aa, e)` (e >= 1): `res = base(aa)`; for `i = e.bit_length()-2 down to 0`:
  `res = mul(res,res)`; if bit i of e: `res = mul_xpa(res, aa)`.
- `splits_completely(p, u, n)`: `pow_xpa(0, p) == base(0)`.
- `vrem(p, a, m, n)`: as the Rust (`lead != 1` -> `li = cinv`, one extra mul per
  step); `n.mul += dm; n.add += dm` per step.
- `vgcd_monic(p, a, b, n)`: Euclid with vrem; final non-monic -> `cinv` and
  `n.mul += len(a)`.
- `vdiv_monic(p, a, m, n)`: `n.mul += dm; n.add += dm` per quotient coefficient.
- `roots_of(p, u, rng, n)`: d = 0 -> `[]`; d = 1 -> `[(p - u[0]) % p]`; else up to 64
  tries: `aa = rng.below(p)`; `h = ModU(p,u).pow_xpa(aa, (p-1)//2)`; `hm1 = h` with
  `hm1[0] -= 1` (mod p), trimmed; if `hm1 == []` continue; `gg = vgcd_monic(u, hm1)`;
  if `deg gg in (0, d)` continue; `q = vdiv_monic(u, gg)`; return
  `roots_of(gg) + roots_of(q)` (None propagates). After 64 tries return None.
- `horner_nat(c, x, n)`: as 4.3 but on Counter n (`m-1` mul, `m-1` add).

## 7. Pencil arms

7.1 Anchors. P_full: `perm = list(range(n))`, Fisher–Yates with the 0x5046 stream:
for `i = n-1 down to 1`: `j = rng.below(i+1)`; swap. Tuple `t` (0-based) uses
`perm[g*t : g*t+g]`; when `g*(t+1) > n`, draw a fresh permutation the same way
(same stream, continuing) and restart block indexing at 0 on it. When a tuple is
formed, for each of its g abscissae in block order: `x = B[idx]`,
`y = sq[fx[x]] if rng.bit() == 0 else p - sq[fx[x]]`. P_2lp: same over the index
list of `S` (stream 0x5032).

7.2 `interpolate(anchors)` on the arm's F (counts asserted, section 11):
1. `Pi = [(p - x_1) % p, 1]` (neg: 1 add); for `i = 2..g`: `Pi = Pi * (x - x_i)`:
   new monic poly of degree k+1 from monic degree k: `new[0] = -x_i*Pi[0]`,
   `new[j] = Pi[j-1] - x_i*Pi[j]` for `1 <= j <= k-1`, `new[k] = Pi[k-1] - x_i`
   (no mul, leading 1), `new[k+1] = 1`: exactly k multiplications.
2. `w_i = prod_{j != i} (x_i - x_j)`: g-1 subtractions, g-2 multiplications each.
3. `winv = batch_inv(w)`: `3(g-1)` mul, 1 inv.
4. `c_i = F.mul(y_i, winv_i)`: g mul.
5. `Pi_i = Pi / (x - x_i)` (monic, degree g-1): `q[g-1] = 1`; for `k = g-1 down to 1`:
   `q[k-1] = F.add(Pi[k], F.mul(x_i, q[k]))`: g-1 mul. (No remainder check here.)
6. `a0 = [0]*g`; `a0[j] += c_i*q[j]` for `j < g-1` (g-1 mul), `a0[g-1] += c_i` (no mul).
Totals: g = 2: 10 mul, 1 inv; g = 3: 27 mul, 1 inv. `a0` has length exactly g
(no trimming), `Pi` length g+1.

7.3 Forward differences `FD(c)` for coefficient list c of length `d+1` (no trim):
`vals[k] = horner_F(c, k)` for `k = 0..d` (`d` mul each); difference table
`D[0..d]` from vals by repeated subtraction (`d(d+1)/2` subs, counted in `add`);
state `D`; `value()` returns `D[0]`; `step()`: for `i = 0..d-1`:
`D[i] = (D[i] + D[i+1]) % p`, `ctr.fd_add += d`. Mul cost per FD: `(d+1)*d`.
P_full uses `FD(Pi)` (d = g) and `FD(a0)` (d = g-1): 8 mul at g = 2, 18 at g = 3.

7.4 `sieve_tuple(anchors, xs_allowed, need)` (P_full: `xs_allowed = B ∪ Z`):
1. `a0, Pi = interpolate(anchors)`; `A = {anchor abscissae}`.
2. `fdP, fdA = FD(Pi), FD(a0)`. For `x = 0 .. xmax` (P_full: `xmax = p-1`):
   if `x in xs_allowed` and `x not in A`: record `(x, fdP.value(), fdA.value())`;
   then if `x < xmax`: `fdP.step(); fdA.step()`.
3. `X` = recorded list (ascending x). `Dinv = batch_inv([Pi(x) for X])`.
4. Buckets: `cnt = [0]*p`, `pts = {}` (dict lambda -> list). For each `(x, Pix, a0x)` in X
   with its `di`: points `[(x, sq[fx[x]]), (x, p - sq[fx[x]])]` if `fx[x] != 0`
   else `[(x, 0)]`; for each `(x, y)`: `lam = F.mul(F.sub(y, a0x), di)`;
   `cnt[lam] += 1; ctr.bucket_ops += 1`; append to `pts[lam]`; if `cnt[lam] > g+1`:
   raise `GateFailure("G5 bucket")`.
5. P_full: `ctr.bucket_scan += p`; candidates = `lam in 0..p-1` ascending with
   `cnt[lam] == g+1`. (P_2lp: section 7.8.)
6. Each candidate gives `(lam, points = anchors + sorted(pts[lam]))`.
Per-tuple assertion (P_full): `mul_delta == interp_mul(g) + 8or18 + 3(len(X)-1) + npts`
and `inv_delta == 2`, where `npts` = number of points processed in step 4,
`interp_mul = 10 / 27`. A mismatch raises `GateFailure("count formula")`.

7.5 Relations, rows, dedup (`rows.py`). `canon(points)`: `K = tuple(sorted(points))`,
`Kn = tuple(sorted((x, (p-y)%p) for x,y in points))`, return `min(K, Kn)`.
A global `seen` set per run; a candidate whose `canon` is in `seen` increments
`duplicates` and is dropped. Row over the base: `row = {}`; for each point with
`y != 0`: `row[col[x]] = row.get(col[x],0) + eps(y)`; drop zero entries;
serialize as sorted `(col, coef)` list. (For P_2lp/B_2lp: columns are S-indices, see 7.8.)

7.6 `p_full(N_target)`: loop over tuples t = 0, 1, ...; run `sieve_tuple`; verify
(7.7) and dedup each candidate; append rows; after finishing a tuple, if
`relations >= N_target`: stop. Record per tuple:
`{"t", "mul", "inv", "cands" (pre-dedup), "new" (accepted), "nX", "npts"}`.
Record for G7 the pooled `sum cands / (tuples * p / (g+1)!)`.

7.7 Verification (uncounted, `verify.py`). V1 for every pencil relation:
`a = a0 + lam*Pi` (plain ints), check `poly(f) - a^2 == prod over all 2g+1 points
(x - x_k)` coefficientwise mod p and `y_k == a(x_k)` for every point. V2: sum of
`point(x_k, y_k)` with a verification Curve (cantor) equals identity (P) or equals
the walk divisor (B). V2 is applied to every relation when `p <= 4099`, else to the
relations whose accepted index `i` satisfies `mix64(i ^ 0x5632) % 1000 < 1000*200/N_target`
(about 200). Counts: `v1_checked, v1_failed, v2_checked, v2_failed`.

7.8 `p_naive1`: tuple 0 of the P_full anchors, own Counter. For `lam = 0..p-1`:
`a = [F.add(a0[k], F.mul(lam, Pi[k])) for k < g] + [lam]` (g mul);
`sq_a` = schoolbook square of a via `F.mul` (`(g+1)^2` mul);
`N = f - sq_a` (F.sub per coefficient); `R = N / Pi` by synthetic division with
`F.mul` (`g*(g+2)` mul), remainder must be `[0]*g` (else GateFailure); assert
`R` monic of degree g+1. `ch = disc_character(p, R, chi, ctr)`; if `ch != 1`: continue.
If not `splits_completely(p, R, ctr)`: continue. `roots = roots_of(p, R, rng_0x504E, ctr)`;
None -> `root_failures += 1`, continue. If `len(set(roots)) != g+1` or any root in
the anchor abscissae: continue. Points: `(x, horner_F(a, x))` (g mul each), sorted.
Relation set `{(lam, tuple(anchors + points))}`. G4: equals P_full tuple 0's
pre-dedup candidate set. Metric: `fpm_per_rel = fpm / len(set)`.

7.9 `p_2lp`: `nb = min(n, ceil(2.0 * p ** (1.0 - 1.0/g)))`; `S = B[:nb]`;
`sidx[x]` = index in S; `xmax = S[-1]`; anchors from S (7.1, stream 0x5032).
Per tuple: steps 1–4 of 7.4 with `xs_allowed = S` and `xmax = S[-1]`, `pts` a
dict (no `cnt` array). Candidates: `lam` with `k = len(pts[lam]) >= g-1`,
ascending lam. For each (own Counter, all via F):
1. `a = a0 + lam*Pi` (g mul); `N = f - a^2` (`(g+1)^2` mul); `R = N / Pi`
   (`g(g+2)` mul, zero remainder or GateFailure).
2. For each bucket abscissa `xj` (ascending): `Q = Q / (x - xj)`: monic degree e:
   `q[e-1] = 1`; for `k = e-1 down to 1`: `q[k-1] = F.add(Q[k], F.mul(xj, q[k]))`;
   remainder `F.add(Q[0], F.mul(xj, q[0]))` must be 0 (else GateFailure): e mul.
3. `e = g + 1 - k`. e = 0: `lp = []`. e = 1: `xr = F.neg(Q[0])`. e = 2:
   `disc = F.sub(F.mul(Q[1], Q[1]), F.add(F.add(Q[0],Q[0]), F.add(Q[0],Q[0])))`;
   if `disc == 0` or `chi[disc] != 1`: reject (`split_rejects += 1`); `s = sq[disc]`;
   `xr1 = F.mul(F.add(F.neg(Q[1]), s), inv2)`, `xr2 = F.mul(F.sub(F.neg(Q[1]), s), inv2)`.
4. Any root in the anchor abscissae: reject (`anchor_rejects += 1`). Any root in S:
   GateFailure("G5 S-root"). For each root `y = horner_F(a, xr)` (g mul). `lp` =
   roots with `y != 0`.
5. Relation: all points = anchors + bucket points + root points; V1; dedup by
   `canon`; row over S-columns for anchors and bucket points (eps); LP vertices
   by abscissa with eps; DSU edge (8.4). `rels_by_lp[len(lp)] += 1`.
Stop as soon as `cycles >= nb + 8 + nb//100` or `fpm >= 2e9` (`status: exhausted`)
or anchors exhausted after 50 permutations (`status: anchors_exhausted`).

## 8. Baseline arms

8.1 `b_lab(N_target)`, transcription of `genus_ladder::run_ic` (A_full, prefilter
on), without labels. Streams `W = SM64(0x424C...)`, `Rr = SM64(0x4252...)`.
Setup (counted, reported as `setup_*`, excluded from FPM): `M[j] = random_divisor(W)`
for `j < 16`; `R1 = random_divisor(W)`, `R2 = random_divisor(W)`. `L = p.bit_length()`.
Then snapshot counters. `since = infinity`, `cur = identity`. While
`relations < N_target`:
- if `since >= 4096`: `a = 1 + W.below(2^(g*L) - 1)`, `b` likewise;
  `cur = cantor(mul_ltr(R1, a), mul_ltr(R2, b))`; `since = 0`; `restarts += 1`
  (group ops of this step tallied as `restart_add`, `restart_dbl`).
- else: `j = (mix64(key_of(cur)) >> 24) % 16`; `cur = cantor(cur, M[j])`;
  `since += 1` (tallied `walk_add`).
- `trials += 1`. If `deg u(cur) != g`: continue. `candidates += 1`.
- `ch = disc_character(p, u, chi, rel)`; if `ch != 1`: `prefilter_rejects += 1`; continue.
- if `g >= 3` and not `splits_completely(p, u, rel)`: continue.
- `splits += 1`; `roots = roots_of(p, u, Rr, rel)`; None -> `root_failures += 1`; continue.
- points `(x, horner_nat(v, x, rel))` for each root; V2 (7.7) against `cur`; dedup by
  `canon`; row (7.5); `relations += 1`.
FPM (I) = `cantor_mul + I*cantor_inv + rel_mul + I*rel_inv` (deltas after setup).
`strong_fpm` (g = 2 only) = `35*walk_add + 37*restart_dbl + 35*restart_add +
1*candidates + 4*splits`; at g = 3 write `null`.
Budget guard: check `time.process_time()` every 256 trials.

8.2 Rows files (relphase, only if `p <= 4099`): `raw/rows/rows_{arm}_g{g}_p{p}_c{c}_s{s}.txt`,
header line `# L01.rows/v1 arm=... g=.. p=.. curve_seed=.. solver_seed=.. n_cols=.. n_rows=..`,
then the first `N_target` rows, one per line: `col:coef col:coef ...` ascending
col. `rows_digest` = SHA-256 hex of those row lines joined by `"\n"` (computed for
every run, written or not).

8.3 `b_2lp`: as 8.1 with base S of 7.9: after the points, entries with `y != 0`
and `x in S` go to the row (S-columns), others are LPs; if more than 2 LPs:
`lp_rejects += 1`, continue; if row empty and no LP: continue. DSU edge, dedup,
`rels_by_lp`. Stop as 7.9. `strong_fpm` as 8.1 at g = 2.

8.4 DSU: vertex 0 is "no large prime"; LP vertex ids allocated by abscissa.
Edge per relation: 0 LP `(0,0)`, 1 LP `(0,v)`, 2 LP `(v1,v2)`. If
`find(a) == find(b)`: `cycles += 1` else union (by size, path halving). `dsu_ops`
counts finds (informational).

8.5 `b_labelled` (gate only): G, T, r known. `M[j] = cantor(mul_ltr(G, al_j), mul_ltr(T, be_j))`,
`al_j, be_j = 1 + rng.below(min(r, 4096) - 1)`; restarts `cur = [a]G + [b]T` with
`a, b = 1 + rng.below(r - 1)`; track `ca, cb` mod r (add `al_j, be_j` per step).
Collect 50 distinct relations with full-base rows; check
`sum coef*L_true == h*(ca + cb*t) mod r` for each.

## 9. Gate and linear algebra

9.1 `N_k(f, p, k)` (uncounted): `N_1 = 1 + sum_x (1 + chi(f(x)))`.
`k = 2`: `F_{p^2} = F_p[w]/(w^2 - nr)`, `nr` the least quadratic non-residue;
`N_2 = 1 + sum over all p^2 elements z of (1 + chi_p(Norm f(z)))` with
`Norm(a + b w) = a^2 - nr b^2`, `chi_p(0) = 0`.
`k = 3` (requires `p % 3 == 1`): `F_{p^3} = F_p[w]/(w^3 - cn)`, `cn` the least
non-cube (`cn^((p-1)/3) != 1`); `Norm(a + b w + c w^2) = a^3 + cn b^3 + cn^2 c^3 - 3 cn a b c`.
`jac_order(N, p, g)`: `t_k = p^k + 1 - N_k`; `e_0 = 1`,
`e_k = (sum_{i=1..k} (-1)^(i-1) e_{k-i} t_i) / k` (exact); `c_j = (-1)^j e_j` for
`j <= g`, `c_{2g-j} = p^(g-j) c_j`; `#J = sum_j c_j`.
`split_order(J)`: smallest `h` in 1..64 with `J % h == 0`, `J/h` prime, `J/h > h`; else reject.

9.2 Gate procedure (`run.py gate`, stream 0x4754): curve seed = first `s` in 1..400
with f squarefree and `split_order` admitted. Order check: 16 `random_divisor`,
`mul_ltr(D, J) == identity`. `G`: first `random_divisor` D0 with
`mul_ltr(D0, h) != identity`; check `mul_ltr(G, r) == identity`. `t = 1 + below(r-1)`,
`T = mul_ltr(G, t)`. BSGS: `m = isqrt(r) + 1`; baby table `{jG: j}` for `j < m`;
`step = negate(mul_ltr(G, m))`; for each column x of B: `Q = mul_ltr(point(x, sq[fx[x]]), h)`;
for `i = 0..m`: if `Q in table`: `L_true = (i*m + table[Q]) % r`; break; `Q = cantor(Q, step)`.
Not found -> fail.
Rows: `p_full` with `N_target = 3n` (solver seed 1, up to 50 permutations).
Check every row: `sum coef*L_true == 0 mod r`. `b_labelled` (8.5).
Descents: G-rows: repeat `a = 1 + below(r-1)`, `D = mul_ltr(G, a)` until 3 rows with
`deg u = g`, g distinct roots in F_p (brute-force evaluation), all root points with
`y != 0`: row coefs by eps/col, `tcoef = 0`, `rhs = h*a mod r`. T-rows: `a, b`,
`D = cantor(mul_ltr(G,a), mul_ltr(T,b))`, `tcoef = -h*b mod r`, `rhs = h*a mod r`; 3 rows.
`solve_mod_r`: dense Gauss–Jordan mod r over unknowns `L_0..L_{n-1}, t`, rows =
pencil rows (rhs 0) + 6 descent rows. Inconsistent -> fail. Variable i determined
iff some RREF row has its only non-zero unknown coefficient at i. Pass iff all row
checks hold, `t` determined and equal to the planted t, every determined `L_j`
equals `L_true[j]`. Also report `hom_rank` (rank mod r of the pencil rows alone),
`n_cols`, `L_determined`.

9.3 `sparse_eliminate(rows, ncols, q = 2^61 - 1, cap = 5e7)` (own Counter):
rows as dicts `col -> coef mod q`; `col_rows[c]` sets. Loop: among active columns
with weight >= 1 pick minimum `(weight, col)`; none -> stop. Pivot row: minimum
`(len(row), row_id)` among its rows. `ip = inv(row[c])` (1 inv). For every other
row k containing c, ascending id: `fac = mul(row_k[c], ip)`; for each `(j, val)` of
the pivot row with `j != c`: `row_k[j] = row_k[j] - mul(fac, val)` (mod q; delete
if 0; maintain col_rows); delete `row_k[c]`. Remove pivot row and column c;
`pivots += 1`. Track `nnz` (sum of row lengths of active rows) and `peak_nnz`.
If `mul > cap`: `status = "capped"`, stop. Output `uncovered_cols` (initial weight 0),
`pivots`, `mul`, `inv`, `initial_nnz`, `peak_nnz`. Rows: the first N_target rows of
the rows file of arm P_full or B_lab (same g, p, curve, seed 1).

## 10. CLI, records, analysis

10.1 `run.py` subcommands; common options `--g --p --out --cpu-budget 540`.
- `gate --g --p`: one `L01.gate/v1` record.
- `relphase --g --p --curve-index --seed --arms P_full,P_naive1,B_lab [--rows-dir]`:
  runs arms in that order; one `L01.relphase/v1` record per arm. If the budget is
  hit, that record has `status: "budget_exhausted"` and later arms `status: "not_run"`.
- `lp2 --g --p --curve-index --seed`: P_2lp then B_2lp, `L01.lp2/v1` records.
- `la --g --p --curve-index --seed --rows-dir`: one `L01.la/v1` record per arm
  (P_full, B_lab).
GateFailure: write the record with `status: "gate_failure"` and the message, exit 2.

10.2 `L01.relphase/v1` fields: `schema, arm, g, p, curve_index, curve_seed,
solver_seed, f, n_cols, n_target, status, relations, duplicates, mul, inv, add,
fd_add, bucket_ops, bucket_scan, fpm, fpm_per_rel, setup_mul, setup_inv,
row_weight_mean, row_weight_hist (dict str->int), col_weight_max, cols_unhit,
rows_digest, v1_checked, v1_failed, v2_checked, v2_failed, max_bucket,
cpu_seconds_diag`; P_full adds `tuples, per_tuple (list of 7.6 dicts),
split_model_ratio`; P_naive1 adds `relset_equal, root_failures, lambdas`; B_lab adds
`trials, candidates, prefilter_rejects, splits, root_failures, walk_add, restart_add,
restart_dbl, restarts, cantor_mul, cantor_inv, rel_mul, rel_inv, strong_fpm,
split_rate_model_ratio` (= `splits/candidates` divided by `C(n,g) 2^g / p^g`).
Row/column statistics are over the base columns of all accepted rows.

`L01.lp2/v1`: `schema, arm, g, p, curve_index, curve_seed, solver_seed, nb,
target_cycles, status, relations, rels_by_lp, cycles, duplicates, mul, inv, add,
fpm, fpm_per_rel, fpm_per_cycle, candidates, split_rejects, anchor_rejects,
lp_rejects, strong_fpm, v1_checked, v1_failed, v2_checked, v2_failed, cpu_seconds_diag`.

`L01.la/v1`: `schema, arm, g, p, curve_index, curve_seed, solver_seed, rows, cols, q,
status, pivots, uncovered_cols, mul, inv, initial_nnz, peak_nnz, rows_digest`.

`L01.gate/v1`: `schema, g, p, curve_seed, seeds_tried, N, J, h, r, order_check_ok,
t_planted, t_recovered, n_cols, pencil_rows, pencil_rows_bad, labelled_rels,
labelled_rels_bad, descent_rows, hom_rank, L_determined, L_mismatch, pass, status`.

10.3 `cpu_seconds_diag` is `time.process_time()` delta: diagnostic only, never
read by `analyze.py`.

10.4 A/A: `analyze.py` compares each `aa_*` record with the `rel_*` record of the
same arm, g, p, curve, seed on every field except `cpu_seconds_diag`; any difference
fails G6.

10.5 `analyze.py --raw --out-json --out-md`:
- Gates: unittest log ends with `OK`; every gate record `pass`; every relphase/lp2
  record has `v1_failed == v2_failed == 0` and status not `gate_failure`;
  `relset_equal` true; `max_bucket <= g+1`; G6; G7 pooled per (g, p) cell over
  P_full records: `sum over runs and tuples of cands / sum(tuples) / (p/(g+1)!)` in [0.85, 1.15].
- Pairs per (g, p, curve_index, seed) with P_full and B_lab both `ok`.
  `R1 = B.fpm_per_rel / P.fpm_per_rel`;
  `R2 = (B.strong_fpm / B.relations) / P.fpm_per_rel` (g = 2);
  `R3 = B.fpm_per_rel / ((P.fpm + P.add + P.fd_add + P.bucket_ops + P.bucket_scan) / P.relations)`;
  `R4_I = ((B.cantor_mul + B.rel_mul + I(B.cantor_inv + B.rel_inv)) / B.relations) /
  ((P.mul + I P.inv) / P.relations)` for I in {1, 30}.
  Positive control per run: `PC = (naive.fpm / naive.relations) /
  ((t0.mul + 10 t0.inv) / t0.cands)` with t0 = P_full `per_tuple[0]`; pass iff `PC >= 5`.
- Cell (g, p): `GM = exp(mean(log R))`, `MIN`, `MAX`, `n_pairs`; complete iff 6 pairs.
- Verdict exactly per PROTOCOL section 8 (H1 from g = 2 R1; robustness from R2, R3;
  H2 from g = 3 R1; H3 from R2; H5 from lp2: per (g, p) GM over 3 curves of
  `B_2lp.fpm_per_cycle / P_2lp.fpm_per_cycle`, records with status `ok` only).
- H4 per (g, p, curve): `la_ratio = P.la.mul / B.la.mul`;
  `net = (B.fpm_per_rel - P.fpm_per_rel) * n_target - g^2 * ((P.la.mul + 10 P.la.inv) - (B.la.mul + 10 B.la.inv))`
  (seed 1; capped LA marked, net `null`).
- Noise floor: per (arm, g, p) `max/min` of `fpm_per_rel` over its 6 runs.
- Writes `analysis.json` (all of the above) and `summary.md` (tables: gates, cells
  with GM/MIN/MAX of R1–R4, PC, G7, 2LP, LA, verdict lines quoting the thresholds).

## 11. Unit tests (`tests/test_l01.py`, all must pass; hand values computed by the coordinator)

1. `mix64(0) == 0xe220a8397b1dcdaf`, `mix64(1) == 0x910a2dec89025cc1`.
2. `random_f(1031,2,1) == [85,928,641,833,893,1]`, `random_f(131,2,1) == [31,15,114,98,48,1]`,
   `random_f(13,2,1) == [3,3,1,0,9,1]`, `random_f(7,3,1) == [2,2,4,0,0,2,4,1]`;
   `squarefree(random_f(11,2,1), 11) is False`, `squarefree(random_f(11,2,2), 11) is True`.
3. N_k and #J: (p,g,s) = (13,2,1): `N = [16,170]`, `#J = 200`; (11,2,2): `[10,114]`, 96;
   (7,3,1): `[11,55,323]`, 555; (7,3,2): `[7,51,388]`, 316. Also a brute-force count of
   reduced Mumford pairs (all monic u of degree <= g, all v of degree < deg u with
   `u | f - v^2`) equals `#J` for these four, and `mul_ltr(D, #J)` is the identity
   for 8 random divisors on each.
4. `batch_inv` of 5 values mod 101: correct inverses, exactly 12 mul and 1 inv;
   of 1 value: 0 mul, 1 inv.
5. `interpolate`: counts 10 mul / 1 inv (g = 2) and 27 mul / 1 inv (g = 3). Values:
   p = 13, f = random_f(13,2,1), anchors (0,4),(1,2): `a0 == [4, 11]`, `Pi == [0, 12, 1]`.
   p = 101, f = random_f(101,2,1), anchors (0,43),(3,5): `a0 == [43, 21]`, `Pi == [0, 98, 1]`.
   p = 101, f = random_f(101,3,1), anchors (1,44),(2,13),(3,50): `a0 == [42, 69, 34]`,
   `Pi == [95, 11, 95, 1]`.
6. `FD`: for a random degree-3 polynomial mod 101, values at x = 0..100 equal
   `horner_F`; init mul count `4*3 = 12`; `fd_add` grows by 3 per step.
7. Pencil examples, calling `sieve_tuple` / naive scan with the explicit anchors of
   test 5 and the full base: p = 13 gives exactly one relation, `lam = 10`, bucket
   points `[(6,6),(8,2),(11,3)]` (`a_lam = [4,1,10]`); p = 101, g = 2: 12 relations;
   p = 101, g = 3: 3 relations; in all three the naive scan (7.8) returns the same set;
   V1 and V2 pass on every relation.
8. Per-tuple count formula (7.4) holds on the three examples of test 7.
9. `disc_character` (chi table of p) and `splits_completely`:
   p = 13, u = [2,10,1]: chi +1, 7 mul, 2 add; split True, 22 mul.
   p = 7, u = [1,0,1]: chi -1, 7 mul; split False, 16 mul.
   p = 13, u = [1,11,1]: chi 0, 7 mul.
   p = 13, u = [6,2,7,1]: chi +1, 17 mul, 6 add; split False, 51 mul.
   p = 13, u = [7,11,7,1]: chi +1, 17 mul; split True, 51 mul.
   `roots_of(13, [7,11,7,1])` sorted `== [1,2,3]`.
10. Cantor: on (13,2,1) and (7,3,1): associativity on 20 random triples,
    `cantor(D, negate(D))` is identity, `cantor(point P, point Q)` for distinct
    abscissae equals `((x-x1)(x-x2), line through P,Q)`.
11. `sparse_eliminate` on rows `[{0:1,1:1}, {1:1,2:1}, {0:1,2:1}]`, 3 columns:
    `pivots 3, mul 4, inv 3, initial_nnz 6, peak_nnz 6, uncovered_cols 0`.
12. `p_2lp` with `S = B` (override nb = n) on p = 101, g = 2, anchors of test 5:
    its 0-LP relations equal the 12 relations of test 7.
13. `canon` of a point set and of its negation are equal.

## 12. Execution note for the executor

Run exactly the commands of PROTOCOL section 10, in order, from the lead
directory. Do not edit `PROTOCOL.md`, `SPEC.md` or `frozen.sha256`. If a repository
hook refuses a command, record the refusal verbatim in `raw/REFUSALS.txt` and
continue with what remains possible.
