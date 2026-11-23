# L05 implementation specification (frozen with PROTOCOL.md)

Implement exactly this. Where this file is silent, choose the simplest code that
changes no count, and record the choice in `IMPLEMENTATION_NOTES.md`. Do not
change any threshold, prime, seed, count rule or output field. If a gate fails,
stop that run, write the failure record (section 10.3), and report it. Never
"fix" a count or a check to make a gate pass. Mathematical background is in
`DERIVATION.md`. It explains things but adds no requirement.

## 1. Language and layout

Python 3, standard library only (no numpy or sympy). One thread. All files go
inside `/home/user/crypto/research/g2plus_leads_20261116/L05_even_degree_function_relations/`.

```
l05/__init__.py        empty
l05/rng.py             mix64, SM64, stream_seed                          (3.1, 3.3)
l05/field.py           Counter, F, horner_F, batch_inv, inv_table        (4)
l05/instance.py        random_f, random_f_even, squarefree, roots, select_curve, Tables, Base (3)
l05/models.py          sqrt_part s, top_part t_e, odd_transform, map_point (5)
l05/poly.py            uncounted polynomial helpers and Cantor            (6.1, 6.2)
l05/verify.py          V1, V_inf, V2, mutations                           (6.3–6.6)
l05/zeta.py            F_{p^k} arithmetic, N_k, L-polynomial, #J, ord_delta, random_divisor (6.7–6.9)
l05/pencil.py          ARMS table, anchors, interpolate, FD, sieve_pencil, run_arm (7)
l05/rows.py            canonical key, row statistics, digest            (7.7, 7.8)
l05/gate.py            gate procedure                                     (9)
run.py                 CLI: gate | relphase                               (10)
analyze.py             readings and verdicts                              (10.5)
tests/test_l05.py      unittest suite                                     (12)
IMPLEMENTATION_NOTES.md  written by the implementer
```

## 2. Conventions

- Polynomials over `F_p` are Python lists of ints in `[0, p)`, **lowest degree
  first**, trimmed (no trailing zeros). The zero polynomial is `[]` and
  `deg([]) = -1`. Exception: in section 7, the coefficient lists `c` and `Pi`
  are not trimmed, and their lengths are stated.
- `p` is an odd prime and `g` is in {2, 3}. `M64 = 2**64 - 1` and
  `GOLD = 0x9E3779B97F4A7C15`.
- `eps(y) = +1 if 1 <= y <= (p-1)//2 else -1` (requires `y != 0`).
- Points are tuples `(x, y)` of ints. The strings `"inf+"` and `"inf-"` denote the
  two points at infinity of an even model.
- JSON output has one object per line, written with `json.dumps(obj, sort_keys=True)`.
- `GateFailure(Exception)` carries a message string.

## 3. RNG, curves, tables

3.1 `mix64(z)`:
`z=(z+GOLD)&M64; z=((z^(z>>30))*0xBF58476D1CE4E5B9)&M64; z=((z^(z>>27))*0x94D049BB133111EB)&M64; return z^(z>>31)`.
`class SM64(state)`: `next()`: `state=(state+GOLD)&M64; return mix64(state)`.
`below(n)` returns `next() % n` and `bit()` returns `next() & 1`.

3.2 Curve rules.
- `random_f(p, g, s)` (odd, the lab rule): `st = ((s*GOLD)&M64) ^ p ^ (g<<48)`. Repeat
  `2g+1` times: `st=(st+GOLD)&M64; f.append(mix64(st) % p)`. Then `f.append(1)`.
- `random_f_even(p, g, s)`: `st = ((s*GOLD)&M64) ^ p ^ (g<<48) ^ (0xE5<<40)`. Repeat
  `2g+2` times as above, then `f.append(1)`. The result has length `2g+3`.
- `squarefree(f)`: `gcd(f, f')` computed with uncounted Euclid (6.1) has degree 0.
- `roots(f)`: `[x for x in range(p) if horner(f, x) == 0]`, ascending, uncounted.
- `select_curve(p, g, cls, c)`: iterate `s = 1, 2, ...` up to 10000 and keep `s` when:
  - `cls == "root"`: `f = random_f_even` is squarefree and `roots(f) != []`;
  - `cls == "rootless"`: `f = random_f_even` is squarefree and `roots(f) == []`;
  - `cls == "oddrand"`: `f = random_f` is squarefree.

  Return the c-th kept `(s, f)`, with `c` 1-based. Root curves also get
  `alpha = roots(f)[0]`.

3.3 Streams: `stream_seed(tag, g, p, cs, ss) = mix64(mix64(mix64(tag ^ (g<<56)) ^ p) ^ (cs<<20) ^ ss)`
(every intermediate `& M64`). The stream is `SM64(stream_seed(...))`. `cs` is the
curve seed `s` and `ss` is the solver seed. The tags are:

| tag | used by |
|---|---|
| `0x4F30` | O and O_T |
| `0x4530` | E_0 and E_0_T |
| `0x454C` | E_low |
| `0x4531` | E_1a and E_1a_T |
| `0x4754` | gate random divisors (`ss = 0`) |

E_g uses no randomness.

3.4 Tables, built on a setup `Counter` (section 4), one set per model polynomial `h`:
- `sq = [-1]*p; sq[0] = 0`. For `y in 1..(p-1)//2`: `sq[F.mul(y, y)] = y`.
- `chi[a] = 0 if a == 0 else (1 if sq[a] > 0 else -1)`.
- `inv2 = (p+1)//2`, an uncounted constant.
- `hx[x] = horner_F(h, x)` for `x in 0..p-1`.
- `B = [x for x in 0..p-1 if chi[hx[x]] == 1]`, ascending.
- `col[x]` is the index of `x` in `B`, or `-1`. `n = len(B)`.
- `ylift[x] = sq[hx[x]]` for `x in B`. The two points over `x` are
  `(x, ylift[x])` and `(x, p - ylift[x])`.

Column counts: even model `n_cols = n + 1` (the `delta` column); odd model
`n_cols = n`. `N_target = n_cols + 8 + n_cols // 100`.

## 4. Counted arithmetic

4.1 `class Counter` has integer fields `mul, inv, add, fd_add, bucket_ops, bucket_scan`.
`fpm(I=10) = mul + I*inv`. Arms take deltas (snapshot, subtract).

4.2 `class F(p, ctr)`:
- `mul(a, b)`: `ctr.mul += 1`, returns `a*b % p`. Squarings use `mul`.
- `inv(a)`: asserts `a % p != 0`, `ctr.inv += 1`, returns `pow(a, p-2, p)`.
- `add`, `sub`, `neg`: `ctr.add += 1`.

Table reads are free.

4.3 `horner_F(c, x)` (`c` lowest first, `len(c) = m >= 1`): `acc = c[m-1]`. For
`i = m-2 .. 0`: `acc = F.add(F.mul(acc, x), c[i])`. Exactly `m - 1` multiplications.

4.4 `batch_inv(vals)` (Montgomery, all non-zero, `k = len(vals) >= 1`):
- `pre[0] = vals[0]`, and `pre[i] = F.mul(pre[i-1], vals[i])` for `i = 1..k-1`;
- `t = F.inv(pre[k-1])`;
- for `i = k-1 .. 1`: `out[i] = F.mul(t, pre[i-1]); t = F.mul(t, vals[i])`;
- `out[0] = t`.

Count: `3(k-1)` mul and 1 inv.

4.5 `inv_table(p, F)`: `inv = [0]*p; inv[1] = 1`. For `i = 2..p-1`:
`inv[i] = F.mul((p - p//i) % p, inv[p % i])`. Count: `p - 2` mul and 0 inv. The
integer expression `(p - p//i) % p` is an index computation and is not counted.

## 5. Models (setup, counted on the setup Counter, excluded from arm FPM)

5.1 `sqrt_part(f)` for even `f` (length `2g+3`, monic): `s = [0]*(g+2); s[g+1] = 1`.
For `j = 1..g+1`: `acc = f[2g+2-j]`; for `i = 1..j-1`:
`acc = F.sub(acc, F.mul(s[g+1-i], s[g+1-j+i]))`; then `s[g+1-j] = F.mul(acc, inv2)`.
Assert (uncounted) that `deg(f - s*s) <= g`.

5.2 `top_part(s, e)` (`0 <= e <= g`): `t = [0]*(g+2)`. For `k = g+1-e .. g+1`:
`t[k] = s[k]`. So `t_0 = x^(g+1)` (`[0,...,0,1]`), `t_{g-1}` holds the coefficients of
`x^2..x^(g+1)`, and `t_g` those of `x^1..x^(g+1)`.

5.3 `odd_transform(f, alpha)` (uncounted, plain ints):
- Taylor coefficients: `cur = f`; for `i = 0..2g+2`: `(cur, r) = divmod by (x - alpha)`
  (synthetic division) and `c[i] = r`. So `c[0] = 0`, `c[1] != 0` and `c[2g+2] = 1`.
- `Fodd = [0]*(2g+2)`. For `i = 1..2g+2`:
  `Fodd[2g+2-i] = c[i] * pow(c[1], (i-2) % (p-1), p) % p`. Assert `Fodd[-1] == 1`
  and `len(Fodd) == 2g+2`.
- `cg = pow(c[1], g, p)`.

5.4 `map_point(P)` from the even model to the odd model (uncounted):
- `"inf+"` maps to `(0, cg)` and `"inf-"` to `(0, (p - cg) % p)`.
- `(x, y)` with `x != alpha`: `X = pow(x - alpha, p-2, p)`; returns
  `(c[1]*X % p, cg*y*pow(X, g+1, p) % p)`.
- `(alpha, 0)` maps to `None` (the odd point at infinity). Base points never
  have `y = 0`, so it does not occur in relations.

Assert that every mapped affine point satisfies `Y^2 = Fodd(X)`.

5.5 The odd model's own `Tables`/`Base` (3.4) use `h = Fodd`. The O and O_T arms
on a root curve run on these. On an oddrand curve they run on `h = random_f(...)`.

## 6. Uncounted verification arithmetic

6.1 Polynomial helpers over `F_p`, plain Python, no counters: `trim, add, sub,
neg, mul, divrem` (by any non-zero divisor, using the inverse of the leading
coefficient), `xgcd` (with the gcd made monic), `ev` (Horner) and `monic`.

6.2 Cantor on an odd model `y^2 = Fodd` (generic g, uncounted). Here `Fodd`
means the odd model polynomial in use: the 5.3 transform for root curves, and
`random_f(...)` itself for oddrand curves. Divisors are
`(u, v)` with `u` monic, `deg v < deg u <= g`; the identity is `([1], [])`.
- `point(X, Y) = ([(-X) % p, 1], trim([Y]))` and `negate((u, v)) = (u, neg(v))`.
- `cantor(A, B)`:
  - if one argument is the identity, return the other;
  - `(d1, e1, e2) = xgcd(u1, u2)`;
  - `(d, c1, c2) = xgcd(d1, add(v1, v2))`; `s1 = c1*e1`; `s2 = c1*e2`;
  - `u = u1*u2 / d^2`, asserting zero remainder;
  - `v = ((s1*u1*v2 + s2*u2*v1 + c2*(v1*v2 + Fodd)) / d) mod u`, asserting zero
    remainder of the division by `d`;
  - reduce: while `deg u > g`: `u = monic((Fodd - v^2) / u)` (assert exact) and
    `v = (-v) mod u`;
  - return `(monic(u), v)`.
- `smul(D, k)`: left-to-right double-and-add.
- `div_class(D)`: `D` is a list of `(P, m)`, where `P` is an odd-model point
  `(X, Y)` and `m` a non-zero int. Fold `cantor` over `|m|` copies of `point(P)`,
  negated when `m < 0`.

6.3 **V1** (affine identity), for a relation with polynomial `a` (plain list), its
affine points `pts` (anchors plus residual), and its infinity coefficients:
1. `N = sub(h, mul(a, a))`, where `h` is the model polynomial.
2. Require `deg N == len(pts)`.
3. Require `N == lc(N) * prod (x - x_k)` (coefficientwise).
4. Require `ev(a, x_k) == y_k` for every point.
5. Require that all `x_k` are distinct.
6. Degree sum: `len(pts) + ip + im == 0` (even model) or `len(pts) == 2g+1` (odd model).

6.4 **V_inf** (even model only). Let `M = 2g+3` and
`U[i] = f[2g+2-i]` for `i = 0..min(M-1, 2g+2)`, else 0. Compute `S` by
`S[0] = 1` and, for `k = 1..M-1`, `S[k] = (U[k] - sum_{i=1}^{k-1} S[i]*S[k-i]) * inv2 mod p`.
Set `A[i] = a[g+1-i]` for `i = 0..g+1` (0 when that index is beyond `a`), and
`A[i] = 0` for `i > g+1`. For `sigma` in `(+1, -1)`:
`W[i] = (sigma*S[i] - A[i]) mod p`, `k0` = the least `i` with `W[i] != 0` (if
none, GateFailure "V_inf precision"), and `ord = k0 - (g+1)`. Require
`ord(+1) == ip` and `ord(-1) == im`.

6.5 **V2** (group law). Even model on a root curve: map every affine point and
`inf+-` with `map_point`; the divisor is `[(map(P), 1) for P in pts] + [(map("inf+"), ip), (map("inf-"), im)]`,
dropping zero multiplicities. Require `div_class == identity`. Odd model: the
divisor is `[(P, 1) for P in pts]`, and the same requirement applies.

6.6 Mutations (even arms). Each must be **rejected**, meaning V2 gives a
non-identity result:
- m1: the same `pts` with `(ip + 1, im - 1)`;
- m2: the last residual point `(x, y)` replaced by `(x, p - y)`, with the same
  `ip, im`.

Record `m1_checked`/`m1_accepted` and `m2_checked`/`m2_accepted`. Any acceptance
is a gate failure.

6.7 `F_{p^k}` for `k` in {1, 2, 3}. Elements are tuples of length `k` (lowest
first).
- Moduli: `k = 2`: `t^2 - ns`, where `ns` is the least `c >= 2` with `chi[c] == -1`.
  `k = 3`: `t^3 + t + c`, where `c` is the least value in `1..p-1` such that
  `t^3 + t + c` has no root in `F_p` (assert one is found).
- Multiplication is schoolbook followed by reduction by the monic modulus.
- `norm(z) = det(M_z)` mod p, where column `i` of `M_z` is the coefficient vector
  of `z * t^i mod m` (`k x k`; use the explicit 2x2 or 3x3 determinant).
- `chi_k(z) = 0 if z == 0 else chi[norm(z)]`.

6.8 Zeta. `N_k(h, n_inf) = n_inf + sum over all x in F_{p^k} of (1 + chi_k(h(x)))`,
with `h(x)` by Horner in `F_{p^k}` and `x` iterating over all `p^k` tuples.
`n_inf = 2` for an even monic model and `1` for an odd one. Then:
- `L[0] = 1`; for `k = 1..g`: `L[k] = (sum_{i=1}^{k} (N_i - p^i - 1) * L[k-i]) / k`
  (assert exact division);
- `L[2g-k] = p^(g-k) * L[k]` for `k = 0..g-1`;
- `#J = sum(L)`.

6.9 `random_divisor(rng)` on the odd model: sum (Cantor) of `g` points. Each point
comes from `h = rng.next()`, `X = h % p`; if `chi[Fodd(X)] != 1`, draw again;
`Y = sq[...]`, negated if `(h >> 63) & 1`.

`ord_delta`: let `Dd = cantor(point(*map("inf+")), point(*map("inf+")))`. (This
is the class `[inf+ - inf-]`, because `map("inf-")` is the negation of
`map("inf+")`.) Factor `#J` by trial division, start with `o = #J`, and for each
prime `q` of `#J`, while `o % q == 0` and `smul(Dd, o//q)` is the identity, set
`o //= q`. Report `o`.

## 7. Pencil arms

7.1 `ARMS`:

| arm | model | t | j | R | inv_mode | ip, im (even) |
|---|---|---|---|---|---|---|
| O | odd | none | g | g+1 | batch | — |
| O_T | odd | none | g | g+1 | table | — |
| E_0 | even | `top_part(s, 0)` | g | g+1 | batch | `(-g, -(g+1))` |
| E_0_T | even | `top_part(s, 0)` | g | g+1 | table | `(-g, -(g+1))` |
| E_low | even | none | g | g+2 | batch | `(-(g+1), -(g+1))` |
| E_1a | even | `top_part(s, g-1)` | 1 | g+1 | batch | `(-1, -(g+1))` |
| E_1a_T | even | `top_part(s, g-1)` | 1 | g+1 | table | `(-1, -(g+1))` |
| E_g | even | `top_part(s, g)` | 0 | g+1 | none | `(0, -(g+1))` |

The general rule is `ip = e - g` for depth `e`. The `delta` coefficient doubled,
`dcoef2`, is `e+1` for the `top_part` arms (E_0: 1, E_1a: g, E_g: g+1) and `0`
for E_low and O.

7.2 Anchors (the model's `B`, `ylift`; the stream from 3.3 with the arm's tag).
- `j = g`: `perm = list(range(n))`, shuffled by Fisher–Yates: for
  `i = n-1 down to 1`, `jj = rng.below(i+1)`, then swap. Pencil `t` uses
  `perm[g*t' : g*t'+g]`, where `t'` counts blocks within the current permutation.
  When `g*(t'+1) > n`, draw a fresh permutation the same way (same stream) and
  reset `t' = 0`.
- `j = 1`: the same permutation; pencil `t` uses `perm[t']`, and a fresh
  permutation is drawn when it is exhausted.
- For each anchor index, in order: `x = B[idx]` and
  `y = ylift[x] if rng.bit() == 0 else p - ylift[x]`.
- `j = 0`: a single pencil with no anchors.

7.3 `interpolate(anchors, t)` on the arm's `F`. Here `j = len(anchors)` and
`(x_i, y_i)` are the anchors.
1. Targets: if `t` is not none, `z_i = F.sub(y_i, horner_F(t, x_i))`, which costs
   `g+1` mul each; otherwise `z_i = y_i`.
2. `j = 0`: `Pi = [1]`, `b0 = []`.
3. `j = 1`: `Pi = [F.neg(x_1), 1]`, `b0 = [z_1]`. No multiplication.
4. `j >= 2` (the L01 recipe):
   - `Pi = [F.neg(x_1), 1]`; for `i = 2..j`, multiply the monic degree-`k` `Pi` by
     `(x - x_i)`: `new[0] = F.neg(F.mul(x_i, Pi[0]))`,
     `new[m] = F.sub(Pi[m-1], F.mul(x_i, Pi[m]))` for `1 <= m <= k-1`,
     `new[k] = F.sub(Pi[k-1], x_i)` and `new[k+1] = 1`. That is `k`
     multiplications.
   - `w_i = prod_{l != i} F.sub(x_i, x_l)`, which takes `j-2` mul each.
   - `winv = batch_inv(w)`, costing `3(j-1)` mul and 1 inv.
   - `c_i = F.mul(z_i, winv_i)`, which is `j` mul.
   - `q_i = Pi / (x - x_i)` (monic, degree `j-1`): `q[j-1] = 1`; for `m = j-1 down to 1`,
     `q[m-1] = F.add(Pi[m], F.mul(x_i, q[m]))`. That is `j-1` mul each.
   - `b0 = [0]*j`. For each `i`: `b0[m] = F.add(b0[m], F.mul(c_i, q_i[m]))` for
     `m < j-1` (`j-1` mul), and `b0[j-1] = F.add(b0[j-1], c_i)`.

   Totals for `j >= 2`: `j = 2` takes 10 mul and 1 inv; `j = 3` takes 27 mul and 1 inv.
5. `c`: if `t` is not none, `c = [F.add(t[m], b0[m] if m < len(b0) else 0) for m in 0..g+1]`
   (length `g+2`); otherwise `c = b0` (length `j`). `Pi` has length `j+1`.

`interp_mul(j, has_t) = {0: 0, 1: 0, 2: 10, 3: 27}[j] + (j*(g+1) if has_t else 0)`
and `interp_inv(j) = 1 if j >= 2 else 0`.

7.4 Forward differences `FD(cl)` for a list of length `d+1`, `d >= 0`:
- `vals[k] = horner_F(cl, k)` for `k = 0..d`, costing `d` mul each;
- build the difference table `D[0..d]` from `vals` by repeated `F.sub` (counted in `add`);
- `value()` returns `D[0]`;
- `step()`: for `i = 0..d-1`, `D[i] = (D[i] + D[i+1]) % p` (plain), and
  `ctr.fd_add += d`.

The mul cost is `(d+1)*d`. If `c == []` (only possible for `j = 0` without `t`,
which no arm has), treat it as `[0]` with `d = 0`.

7.5 `sieve_pencil(arm, anchors, F, inv_tab)`:
1. `c, Pi = interpolate(anchors, t)`. Let `A` be the set of anchor abscissae.
2. Set `fdC = FD(c)`. If `inv_mode == "batch"` and `j >= 1`, also set `fdP = FD(Pi)`.
3. For `x = 0..p-1`: if `col[x] >= 0` and `x not in A`, record `(x, fdC.value(), fdP.value() if fdP else None)`.
   Then, if `x < p-1`, step every FD.
4. Let `X` be the recorded list and `k = len(X)`.
   - batch with `j >= 1`: `dinv = batch_inv([Pix for X])`;
   - table: for each `x`, `di = inv_tab[F.sub(x, x_1)]`, then for `i = 2..j`:
     `di = F.mul(di, inv_tab[F.sub(x, x_i)])`;
   - none: no inverse.
5. `cnt = [0]*p` and `pts = {}`. For each recorded `x` (with `cx` and `di`), for
   `y` in `(ylift[x], p - ylift[x])`: `lam = F.mul(F.sub(y, cx), di)` if `j >= 1`,
   else `lam = F.sub(y, cx)`. Then `cnt[lam] += 1`, `ctr.bucket_ops += 1`, and
   append `(x, y)` to `pts[lam]`. If `cnt[lam] > R`, raise `GateFailure("G4 bucket")`.
6. `ctr.bucket_scan += p`. The candidates are the `lam` in ascending order with
   `cnt[lam] == R`.
7. Each candidate becomes a relation: `{lam, anchors, residual: sorted(pts[lam]),
   a: [(c[m] + lam*Pi[m]) % p ...]}`. The polynomial `a` is formed in plain
   uncounted arithmetic: pad `Pi` with zeros to `len(c)` (or pad `c` to `len(Pi)`
   when shorter), then trim. Infinity coefficients come from 7.1.
8. **Count assertion** (G4), on the pencil's Counter delta. The number of points
   is `npts = 2k`.
   - `mul == interp_mul(j, has_t) + (len(c)-1)*len(c) + [batch, j>=1: j*(j+1) + 3(k-1)] + [table: k*(j-1)] + [j>=1: npts]`
   - `inv == interp_inv(j) + [batch, j>=1: 1]`

   A mismatch raises `GateFailure("count formula")`.

7.6 `run_arm(arm, curve, solver_seed, budget)`:
1. Build a fresh arm Counter and `F`.
2. For `inv_mode == "table"`, first run `inv_tab = inv_table(p, F)` (charged to
   the arm) and record `table_mul = p - 2`.
3. Loop over pencils `t = 0, 1, ...`. For each: `sieve_pencil`; V1 (and V_inf for
   even arms) on every candidate; dedup (7.7); accept the new ones; append the
   pencil record.
4. Stop after finishing a pencil when `relations >= N_target`. E_g stops after
   its single pencil. Also stop at 10000 pencils (`status: "pencils_exhausted"`)
   or when `time.process_time()` exceeds the budget (`status: "budget_exhausted"`).
   Otherwise `status: "ok"`.
5. `fpm_per_rel = fpm / relations`, or `null` when there are 0 relations.
6. `rels_per_pencil_norm = candidates / (pencils * p / factorial(R))`.

7.7 Canonical key and dedup.
- Even arms: `K = (tuple(sorted(anchors + residual)), ip, im)` and
  `Ki = (tuple(sorted((x, (p-y) % p) for all points)), im, ip)`; `key = min(K, Ki)`.
- O arms: `K = tuple(sorted(points))`, `Ki` the same with `y` negated, and
  `key = min(K, Ki)`.
- A per-run `seen` set: a candidate whose key is already in `seen` adds 1 to
  `duplicates` and is dropped.

7.8 Rows and statistics (for accepted relations):
- `affine_weight` is the number of points. Assert the abscissae are distinct.
- `has_delta = (dcoef2 != 0)`.
- `colw[col[x]] += 1` for every point.
- Report `row_weight_hist` (affine weight to count, as a dict with string keys),
  `mean_affine_weight`, `delta_fraction`, `max_col_weight = max(colw)` and
  `cols_never_hit = n - #{c : colw[c] > 0}`.
- `keys_sha256 = sha256("\n".join(repr(k) for k in sorted(keys)).encode())`.

## 8. Verification policy

8.1 Every relation of every arm gets V1. Every relation of every even arm also
gets V_inf.

8.2 Gate: every relation of every arm gets V2, and every even-arm relation gets
m1 and m2.

8.3 Relation phase, root class only. For accepted relation index `i` (0-based):
sample it iff `mix64(i ^ 0x5632) % 1000 < min(1000, ceil(1000*200 / N_target))`.
Sampled even-arm relations get V2, m1 and m2. Sampled O and O_T relations get V2
on the odd model. The rootless and oddrand classes get no V2 on even arms; the
O arms there still get V2 on their odd model, by the same sampling.

8.4 Any V1, V_inf or V2 failure, or any accepted mutation, raises
`GateFailure`. The run writes a failure record (10.3) with the full relation and
exits with code 2.

## 9. Gate (`run.py gate --g G --p P`)

For each root curve `c = 1, 2, 3` (3.2):
1. Build the even tables, `s`, the odd transform and the odd tables.
2. Compute `L_even`/`J_even` with `n_inf = 2` on `f`, and `L_odd`/`J_odd` with
   `n_inf = 1` on `Fodd`. Require `J_even == J_odd`.
3. Using stream tag `0x4754`, `cs = curve seed` and `ss = 0`: for 16
   `random_divisor`s `D`, require `smul(D, J_even) == identity`.
4. Compute `ord_delta`.
5. Write `{"type": "gate_curve", ...}` (10.2).
6. Run arms E_0, E_low, E_1a, E_g (even) and O (odd model), each with solver
   seed 1, with full verification (8.2), and write their run records.

A curve passes when steps 2 and 3 hold and every arm has `v1_failed`,
`vinf_failed` and `v2_failed` all 0, with `m1_accepted = m2_accepted = 0`.
Write a final `{"type": "gate_summary", "pass": bool, ...}`.

## 10. CLI, records, analysis

10.1 `run.py relphase --g G --p P --class {root,rootless,oddrand} --curve-index C --seed S --arm ARM --out PATH [--cpu-budget 540]`.
It builds the curve (3.2); the even model for root and rootless; the odd model
(5.3) for root when the arm is O/O_T; and the odd tables of `random_f` for
oddrand. It then calls `run_arm` and writes the pencil records followed by one
run record. Rejected combinations (exit code 1, nothing written): O or O_T on
rootless, and even arms on oddrand.

10.2 Records.
- Pencil record: `{"type":"pencil","arm","t","anchors","k","npts","mul","inv","cands","new"}`.
- Run record: `{"type":"run","arm","g","p","class","curve_index","curve_seed",
  "solver_seed","f","alpha","Fodd","model":"even"|"odd","n","n_cols","n_target",
  "status","pencils","candidates","relations","duplicates","mul","inv","add",
  "fd_add","bucket_ops","bucket_scan","fpm","fpm_per_rel","table_mul",
  "setup_mul","setup_inv","R","rels_per_pencil_norm","row_weight_hist",
  "mean_affine_weight","delta_fraction","max_col_weight","cols_never_hit",
  "keys_sha256","v1_checked","v1_failed","vinf_checked","vinf_failed",
  "v2_checked","v2_failed","m1_checked","m1_accepted","m2_checked",
  "m2_accepted","cpu_seconds"}`. Fields that do not apply are `null`.
  `cpu_seconds` is diagnostic only and is never used by `analyze.py`.
- Gate curve record: `{"type":"gate_curve","g","p","curve_index","curve_seed","f",
  "alpha","Fodd","s","L_even","L_odd","J_even","J_odd","jcheck_ok","ord_delta"}`.

10.3 Failure record: `{"type":"failure","stage","arm","message","relation":{...}}`.

10.4 Exit codes: 0 for ok, 2 for a gate failure, 1 for a usage error.

10.5 `analyze.py --raw raw --out-json --out-md`:
1. **Gates.** All `gate_*.jsonl` have `gate_summary.pass == true`. No relation
   phase record has `type == "failure"`. `raw/unittest.log` ends with `OK`.
2. **G5.** For every root (curve, seed): `keys_sha256` of O_T equals O, E_0_T
   equals E_0, and E_1a_T equals E_1a.
3. **G6.** Each `aa_*` run record equals the matching `rel_*` record on every
   field except `cpu_seconds`.
4. **G7.** Per cell `(g, p)` and arm in {O, E_0, E_1a, E_low}, pooled over the
   cell's root and rootless runs (O uses root only):
   `sum(candidates) / sum(pencils * p / R!)` must lie in [0.85, 1.15].
5. **Readings** per cell over the 6 root pairs (curve 1–3, seed 1–2), as defined
   in PROTOCOL section 5:
   - R1 = E_0 / O;
   - R2 = O_T / E_1a_T;
   - R2b = O / E_1a;
   - R3 = E_low / E_0;
   - R4 = GM(E_0, rootless) / GM(O, root);
   - R5: R1 and R2 recomputed with `fpm + add + fd_add + bucket_ops` (adds priced),
     and with `mul + I*inv` for `I` in {1, 30}.

   Report GM, min, max and n for each.
6. **Verdicts** exactly per PROTOCOL section 8. A missing or non-`ok` run makes
   its cell missing.
7. **Also report** the oddrand O GM per cell against the root O GM (descriptive),
   E_g candidates per curve, the row statistics per arm (mean of
   `mean_affine_weight`, max of `max_col_weight`), and `ord_delta` per gate curve.

Write `analysis.json` (all of the above) and `summary.md`. The summary has a
table per g with columns `p | R1 GM [min,max] | R2 GM | R3 GM | R4 | G7 ok`, the
verdict lines, and the limits paragraph copied from PROTOCOL section 11.

## 11. Budget and memory

`--cpu-budget` defaults to 540 CPU seconds, checked after each pencil with
`time.process_time()`. Arrays are of size `p`, so the memory needed is small. Do
not parallelize.

## 12. Unit tests (`tests/test_l05.py`; all must pass, G1)

Hand values were computed by the coordinator with a throw-away script.

1. `mix64(0) == 16294208416658607535` and `mix64(1) == 10451216379200822465`.
2. `random_f(101, 2, 1) == [31, 46, 14, 30, 51, 1]`.
3. `random_f_even` values:
   - `(101, 2, 1)` gives `[75, 29, 79, 35, 2, 13, 1]`;
   - `(101, 2, 2)` gives `[2, 75, 95, 19, 21, 78, 1]`;
   - `(31, 2, 1)` gives `[26, 19, 4, 19, 18, 20, 1]`;
   - `(37, 3, 1)` gives `[9, 14, 25, 14, 17, 24, 0, 25, 1]`.
4. Let `f1 = random_f_even(101,2,1)`. Then `squarefree(f1)`, `roots(f1) == [16, 68]`,
   `sqrt_part(f1) == [41, 43, 57, 1]` and `sub(f1, mul(s,s)) == [10, 38, 20]`.
   For `f2 = random_f_even(101,2,2)`: `roots == [28]` and `s == [20, 58, 39, 1]`.
5. `odd_transform(f1, 16)`: Taylor `c == [0, 17, 99, 21, 34, 8, 1]`,
   `Fodd == [95, 15, 29, 54, 99, 1]` and `cg == 87`. For `f2` (alpha 28):
   `c[1] == 24` and `Fodd == [92, 34, 13, 43, 65, 1]`.
6. `map_point` on `f1`:
   - `(36,71)` maps to `(16,20)`, `(42,99)` to `(90,14)` and `(88,12)` to `(83,54)`;
   - `"inf+"` maps to `(0,87)` and `"inf-"` to `(0,14)`.
7. Zeta on `f1`:
   - even: `N_1 = 114`, `N_2 = 10366`, `L = [1, 12, 154, 1212, 10201]` and `#J = 11580`;
   - odd (on `Fodd`): `N_1 = 114`, `N_2 = 10366` and `#J = 11580`.

   On `f2`, both models give `#J = 9815` with `L = [1, -5, 123, -505, 10201]`.
   `ord_delta(f1) == 2895`.
8. Base on `f1`: `n == 55` and `B[:5] == [4, 5, 6, 8, 11]`. On `f2`: `n == 47`.
9. E_g on `f1`:
   - exactly 17 candidates and 17 relations;
   - the smallest `lam` is 3, with residual `[(36,71), (42,99), (88,12)]`,
     `a == [3, 43, 57, 1]`, `(ip, im) == (0, -3)`;
   - the pencil Counter has `mul == 12` and `inv == 0`.

   On `f2`: 15 relations, the smallest `lam` is 2 and the residual is
   `[(4,13), (31,87), (91,100)]`.
10. V2 on that `f1` relation is the identity. m1 gives `([0, 0, 1], [87, 68])`
    and m2 gives `([21, 36, 1], [93, 25])`, both non-identity.
11. `V_inf` on `f1` gives `(ord+, ord-)`:
    - `[3,43,57,1]` gives `(0,-3)` and `[41,43,57,1]` gives `(1,-3)`;
    - `[0,0,1]` gives `(-3,-3)` and `[5,0,0,1]` gives `(-2,-3)`;
    - `[7,9,57,1]` gives `(-1,-3)`.

    The first 7 coefficients of `S` are `[1, 57, 43, 41, 10, 55, 76]`.
12. `inv_table(101)`: `i*inv[i] % 101 == 1` for all `i` in `1..100`, at a cost of
    exactly 99 mul.
13. `batch_inv` of `k = 5` values costs 12 mul and 1 inv and is correct.
    `interpolate` with `j = 2` and `t = none` costs 10 mul and 1 inv; with `j = 3`
    it costs 27 mul and 1 inv. In both cases `b0(x_i) == z_i` holds (uncounted
    check).
14. Run one pencil of each arm on `f1` (O on `Fodd`), with solver seed 1 and curve
    seed 1. The count assertion of 7.5 holds; every candidate passes V1, V_inf
    and V2; m1 and m2 are rejected; and the bucket maximum is at most `R`.
15. Run E_0 and E_0_T, E_1a and E_1a_T, and O and O_T on `f1` (`p = 101`) to
    `N_target`. Each pair has equal `keys_sha256`.
16. g = 3: for `f = random_f_even(37, 3, 1)`, `squarefree(f)` returns a bool,
    and the `F_{p^3}` modulus search succeeds at `p = 37` and `p = 53`.
    Whenever `select_curve(37, 3, "root", 1)` exists, its even and odd `#J`
    agree.

## 13. Execution note for the executor

Run the commands of PROTOCOL section 10 verbatim, in order. Do not edit
`PROTOCOL.md` or `SPEC.md` (they are frozen in `frozen.sha256`). If any test or
gate fails, stop, and report the failing record and the exact command. A
repository hook refusal is recorded verbatim and is not worked around.
