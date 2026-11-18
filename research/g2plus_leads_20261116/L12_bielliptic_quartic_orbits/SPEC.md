# L12 implementation specification (frozen with PROTOCOL.md)

Read PROTOCOL.md first; this file says how, PROTOCOL.md says why. Every research
choice is made here: if something below seems ambiguous, take the literal
reading and record the question in `raw/IMPLEMENTER_NOTES.txt`; do not choose a
different algorithm, threshold, prime, seed or counter rule.

Language: Python 3, standard library only (`random`, `json`, `hashlib`, `argparse`,
`subprocess`, `math`, `unittest`, `statistics`, `os`, `sys`, `time`). One thread.
All files live under
`/home/user/crypto/research/g2plus_leads_20261116/L12_bielliptic_quartic_orbits/`.
Write nothing outside it. Do not use git.

## 1. Files

| file | content |
|---|---|
| `code/fp.py` | `Ops` counter class, counted `F_p` arithmetic (section 2) |
| `code/curve.py` | F1/F0 curve object, invariants, counted point enumeration, counted elliptic arithmetic (section 3) |
| `code/counts.py` | uncounted `#C`, `#E` over `F_p` and `F_{p^2}`, `#P` (section 4) |
| `code/select_instances.py` | instance selection CLI (section 5) |
| `code/arms.py` | column maps for arms A, B, C, N (section 6) |
| `code/section.py` | counted line sections, verifier, row builder (section 7) |
| `code/la.py` | counted Wiedemann / Berlekamp-Massey over `Z/r` (section 8.3) |
| `code/ic_run.py` | one IC run CLI (section 8) |
| `code/rho_run.py` | one rho reference run CLI (section 9.2) |
| `code/generic.py`, `code/generic_run.py` | generic quartic control (section 9.3) |
| `code/run_matrix.py` | launches all runs (section 9.1) |
| `code/analyze.py` | tables and mechanical verdict (section 11) |
| `code/test_l12.py` | unit tests (section 10) |

Every CLI script starts with `sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))`
so that `code/` modules import each other.

## 2. Counted field arithmetic (`code/fp.py`)

```python
class Ops:
    def __init__(self): self.mul = 0; self.inv = 0; self.add = 0; self.lookup = 0
    def fpm(self): return self.mul + 10 * self.inv
    def as_dict(self): return {"mul": self.mul, "inv": self.inv, "add": self.add, "lookup": self.lookup, "fpm": self.fpm()}
```

`class Fp` with `__init__(self, p, ops)` and methods (all inputs and outputs are
canonical ints in `[0, p)`):

- `mul(a, b)`: `ops.mul += 1`; return `a * b % p`. Used for every product of two
  field elements, squarings included, and for products with curve coefficients
  (`a1`, `a2`, ...) and with the slope `m`, even when an operand happens to be 0 or 1.
- `inv(a)`: `ops.inv += 1`; `assert a != 0`; return `pow(a, p - 2, p)`.
- `add(a, b)`, `sub(a, b)`, `neg(a)`: `ops.add += 1`; modular result.
- `dbl(a)` = `add(a, a)`; `times4(a)` = `dbl(dbl(a))`; `times3(a)` = `add(dbl(a), a)`.
  Integer literals 2, 3, 4 are applied only through these (no `mul`).

Counted polynomials are lists of coefficients, **low degree first**.

- `pmul(F, A, B)`: schoolbook: `C = [0] * (len(A) + len(B) - 1)`; for every `i`, `j`:
  `C[i+j] = F.add(C[i+j], F.mul(A[i], B[j]))`. Exactly `len(A) * len(B)` muls,
  no skipping of zeros.
- `trim(A)`: remove trailing zero coefficients (highest degree), keep at least one. No count.
- `divroot(F, S, x0)`: synthetic division of `S` (degree `d >= 1` after trim) by
  `(X - x0)`: `q = [0] * d`; `q[d-1] = S[d]`; for `k = d-1 down to 1`:
  `q[k-1] = F.add(S[k], F.mul(x0, q[k]))`; then `rem = F.add(S[0], F.mul(x0, q[0]))`.
  Exactly `d` muls. Return `(q, rem)`. Caller asserts `rem == 0`
  (a nonzero remainder is an instrument error: raise).

Uncounted helpers (verifier, selection): plain Python `% p` arithmetic in
functions whose names start with `u_` (for example `u_inv`, `u_pmul`). They never
touch an `Ops`.

Square-root table (built inside setup, counted): `SQRT = [-1] * p`; `SQRT[0] = 0`;
for `v in 1 .. (p-1)//2`: `SQRT[F.mul(v, v)] = v`. So `SQRT[a]` is the smallest
root in `[1, (p-1)/2]`, or `-1` for a non-residue; every table read does
`ops.lookup += 1`.

## 3. Curves (`code/curve.py`)

### 3.1 Families and invariants

`Curve(p, a1, a3, a2, a4, a6)`; F0 is `a1 = a3 = a2 = 0`.
`h(x) = a1 x + a3`, `f(x) = x^3 + a2 x^2 + a4 x + a6`.
Quartic `C: y^4 + h(x) y^2 - f(x) = 0`; elliptic `E: w^2 + h(x) w - f(x) = 0`.
Uncounted invariants (integers mod `p`):

```
b2 = a1^2 + 4 a2;  b4 = 2 a4 + a1 a3;  b6 = a3^2 + 4 a6
b8 = a1^2 a6 + 4 a2 a6 - a1 a3 a4 + a2 a3^2 - a4^2
DeltaE = -b2^2 b8 - 8 b4^3 - 27 b6^2 + 9 b2 b4 b6
c4 = b2^2 - 24 b4;   c6 = -b2^3 + 36 b2 b4 - 216 b6
disc_f = a2^2 a4^2 - 4 a4^3 - 4 a2^3 a6 - 27 a6^2 + 18 a2 a4 a6
```

### 3.2 Counted point enumeration (`setup_points(F)`, charged to SETUP in every arm)

```
build SQRT (section 2)                       # (p-1)/2 mul
inv2 = F.inv(2)                              # 1 inv
points = []                                  # tuples (x, y, w, hx)
E_aff = 0
for x in range(p):
    hx = F.add(F.mul(a1, x), a3)             # 1 mul
    fx = F.add(F.mul(F.add(F.mul(F.add(x, a2), x), a4), x), a6)   # 2 mul
    D  = F.add(F.mul(hx, hx), F.times4(fx))  # 1 mul
    if D == 0:   ws = [F.mul(F.neg(hx), inv2)]                        # 1 mul
    elif SQRT[D] > 0:
        s = SQRT[D]
        ws = [F.mul(F.add(F.neg(hx), s), inv2), F.mul(F.sub(F.neg(hx), s), inv2)]  # 2 mul
    else: ws = []
    E_aff += len(ws)
    for w in ws:
        if w == 0: points.append((x, 0, 0, hx))
        elif SQRT[w] > 0:
            y = SQRT[w]; points.append((x, y, w, hx)); points.append((x, p - y, w, hx))
points.sort()            # lexicographic on (x, y)
```

`N_aff = len(points)`; `N_ram = #points with y == 0`; `#C(F_p) = N_aff + 1`;
`#E(F_p) = E_aff + 1`. `index[(x, y)] = i` (position in the sorted list).
`pi(P_i) = (x_i, w_i)`.

### 3.3 Counted elliptic arithmetic (points are `None` for `O`, or `(x, w)`)

`eadd(F, P, Q)`:
1. If `P is None` return `Q`; if `Q is None` return `P` (0 ops).
2. If `x1 == x2`: `s = F.add(F.add(F.add(w1, w2), F.mul(a1, x2)), a3)` (1 mul);
   if `s == 0` return `None`; otherwise (then `P == Q`) doubling:
   `num = F.sub(F.add(F.add(F.times3(F.mul(x1, x1)), F.dbl(F.mul(a2, x1))), a4), F.mul(a1, w1))` (3 mul);
   `den = F.add(F.add(F.dbl(w1), F.mul(a1, x1)), a3)` (1 mul);
   `lam = F.mul(num, F.inv(den))` (1 inv, 1 mul).
3. Else chord: `lam = F.mul(F.sub(w2, w1), F.inv(F.sub(x2, x1)))` (1 inv, 1 mul).
4. `x3 = F.sub(F.sub(F.sub(F.add(F.mul(lam, lam), F.mul(a1, lam)), a2), x1), x2)` (2 mul);
   `nu = F.sub(w1, F.mul(lam, x1))` (1 mul);
   `w3 = F.sub(F.sub(F.neg(F.mul(F.add(lam, a1), x3)), nu), a3)` (1 mul). Return `(x3, w3)`.

So chord = 5 mul + 1 inv; doubling = 10 mul + 1 inv (the 1-mul negation test included).
`eneg(F, P) = (x, F.sub(F.neg(w), F.add(F.mul(a1, x), a3)))` (1 mul).
`emul(F, P, k)`: left-to-right binary, `R = None`; for each bit of `bin(k)[2:]`:
`R = eadd(F, R, R)`; if bit is `'1'`: `R = eadd(F, R, P)`. `k = 0` returns `None`.
The verifier uses the same functions with a throw-away `Ops()` (never charged).

## 4. Point counts and `#P` (`code/counts.py`, uncounted)

`F_{p^2} = F_p[t]/(t^2 - d)`, `d` = smallest integer `>= 2` that is a non-residue
mod `p`. Elements `(u0, u1)`. Operations: add, mul
`(u0 v0 + d u1 v1, u0 v1 + u1 v0)`, norm `N(u) = u0^2 - d u1^2`. `is_sq2(u)`:
`u == 0` or `N(u)` is a nonzero square mod `p`. `sqrt2(u)` for a nonzero square:
if `u1 == 0`: if `u0` is a square mod `p` return `(sqrt(u0), 0)` else return
`(0, sqrt(u0 / d))`; else `n = sqrt(N(u))`; `z = (u0 + n)/2`; if `z` is not a square
mod `p` then `z = (u0 - n)/2`; `b0 = sqrt(z)`; `b1 = u1 / (2 b0)`; assert
`(b0, b1)^2 == u`; return. (`sqrt` mod `p`: any exact method, for example a table.)
`nsq2(w)` = 1 if `w == 0`, 2 if `w` a nonzero square, else 0.

`count(q_is_p2)`: over every `x` in `F_q` (`q = p` uses `u1 = 0` only):
`hx = a1 x + a3`, `fx = f(x)`, `D = hx^2 + 4 fx`; if `D == 0`: `nE += 1`,
`nC += nsq2(-hx/2)`; elif `is_sq2(D)`: `s = sqrt2(D)`, `nE += 2`,
`nC += nsq2((-hx + s)/2) + nsq2((-hx - s)/2)`. Then `#E = nE + 1`, `#C = nC + 1`.

`t1E = p + 1 - #E(F_p)`, `t1C = p + 1 - #C(F_p)`, `t2E = p^2 + 1 - #E(F_{p^2})`,
`t2C = p^2 + 1 - #C(F_{p^2})`; `s1 = t1C - t1E`; `T2 = t2C - t2E`; require
`(s1*s1 - T2) % 2 == 0`; `s2 = (s1*s1 - T2) // 2`;
`nP = 1 - s1 + s2 - p*s1 + p*p`; Weil check
`(isqrt-free) (sqrt(p) - 1)^4 <= nP <= (sqrt(p) + 1)^4` using floats;
E check `#E(F_{p^2}) == p*p + 1 - (t1E*t1E - 2*p)`. Return a dict with all of these.

## 5. Instance selection (`code/select_instances.py`)

CLI: `--out raw/instances.jsonl --rejects raw/rejects.tsv`.

```
F1 primes [263, 523, 1031, 2063], want 4, cs = 1000003*p + k
F0 primes [269, 521, 1033, 2069], want 2, cs = 2000003*p + k
G  primes [263, 523, 1031, 2063], want 2, cs = 3000003*p + k   (section 9.3)
```

For F1: `rng = random.Random(cs)`; `a1, a3, a2, a4, a6 = [rng.randrange(p) for _ in range(5)]`
(in this order). For F0: `rng = random.Random(cs)`; `a4 = rng.randrange(p)`;
`a6 = rng.randrange(p)`; `a1 = a3 = a2 = 0`. Scan `k = 0, 1, ...` (cap 10000).
Reject reasons, tested in this order, first failing one recorded:
`h_zero` (F1 only: `a1 == a3 == 0`), `a46_zero` (F0 only: `a4 == 0 or a6 == 0`),
`disc_f`, `deltaE`, `j0` (`c4 == 0`), `j1728` (`c6 == 0`), `r_small`
(largest prime factor `r` of `n_E` below 5), `r_squared` (`r^2 | n_E`),
`cofactor` (`n_E / r > 16`), `few_points` (`N_aff < 8`),
`count_mismatch` (enumeration `#C(F_p)` differs from `counts.py` `#C(F_p)`, or
the E check fails: this is an instrument error, stop the script with exit code 2),
`parity` (same: instrument error, exit 2), `weil` (instrument error, exit 2),
`r_divides_P` (`nP % r == 0`). `n_E` and `N_aff` here come from an uncounted
enumeration (selection is not charged).

Output line (JSON, one per accepted instance, keys sorted):
`{"family": "F1", "p": .., "k": .., "cs": .., "a1":.., "a3":.., "a2":.., "a4":.., "a6":.., "nE":.., "r":.., "c":.., "N_aff":.., "N_ram":.., "nC1":.., "nE2":.., "nC2":.., "s1":.., "s2":.., "nP":.., "nJ": nE*nP, "j": index_within_prime}`.
`rejects.tsv` header: `family\tp\tk\treason`.

## 6. Column maps (`code/arms.py`)

`column_map(arm, points, curve, ss)` returns `(col, sign, reps)`: `col[i]`
the column of point `i`, `sign[i]` in `{+1, -1}`, `reps[j]` the elliptic point
representing column `j`. Columns are numbered by first appearance while scanning
`points` in sorted order. No field operations are counted here (everything
needed was stored by `setup_points`); `w'` below is an uncounted integer formula.

- `A`: `col[i] = i`, `sign = +1`, `reps[i] = (x_i, w_i)`.
- `B` and `B'`: key `(x_i, w_i)`; `sign = +1`; `reps[j] = key`.
- `C`: `w' = (-w_i - hx_i) mod p`; key `(x_i, min(w_i, w'))`; `sign = +1` if `w_i <= w'`
  else `-1`; `reps[j] = key`.
- `N`: `perm = list(range(N_aff))`; `random.Random(ss + 90000).shuffle(perm)`;
  `col[perm[2t]] = col[perm[2t+1]] = t`; an odd last element gets its own column;
  `sign = +1`; `reps[t] = (x, w)` of point `perm[2t]`.

## 7. Line sections (`code/section.py`)

### 7.1 Counted section for a pair `(P1, P2)`, `P1 != P2`, both affine

Returns `(status, terms)` where `status in {"split", "nonsplit"}` and `terms`
is a dict `point_index -> multiplicity` (affine points only; `inf_C` is implicit).

Case V (`x1 == x2`):
`hx = F.add(F.mul(a1, x1), a3)` (1 mul); `fx` as in 3.2 (2 mul);
`S = [F.neg(fx), 0, hx, 0, 1]` (polynomial in `Y`); `S, r1 = divroot(F, S, y1)`;
`S, r2 = divroot(F, S, y2)` (4 + 3 mul); then section 7.2 with `S` of degree 2;
each root `Y` gives point `(x1, Y)`.

Case L (`x1 != x2`):
`m = F.mul(F.sub(y2, y1), F.inv(F.sub(x2, x1)))` (1 inv, 1 mul);
`c = F.sub(y1, F.mul(m, x1))` (1 mul); `L = [c, m]`; `L2 = pmul(F, L, L)` (4);
`L4 = pmul(F, L2, L2)` (9); `HL2 = pmul(F, [a3, a1], L2)` (6);
`S[k] = L4[k] + HL2[k] - [a6, a4, a2, 1, 0][k]` for `k = 0..4` (additions only);
`trim(S)` (degree 4, or 3 exactly when `m == 0`);
`S, _ = divroot(F, S, x1)`; `S, _ = divroot(F, S, x2)` (4 + 3 mul, or 3 + 2 when `m == 0`);
then section 7.2; each root `X` gives the point `(X, F.add(F.mul(m, X), c))` (1 mul per root).
When `m == 0` the line `y = c` also passes through `inf_C` with multiplicity 1.

### 7.2 Residual

Degree 2, `S = [e0, e1, e2]`: `D = F.sub(F.mul(e1, e1), F.times4(F.mul(e2, e0)))` (2 mul).
If `D == 0`: `t = F.inv(F.dbl(e2))`; one root `X = F.mul(F.neg(e1), t)` with
multiplicity 2 (1 inv, 1 mul). Elif `SQRT[D] > 0`: `s = SQRT[D]`; `t = F.inv(F.dbl(e2))`;
roots `F.mul(F.add(F.neg(e1), s), t)` then `F.mul(F.sub(F.neg(e1), s), t)` (1 inv, 2 mul).
Else return `("nonsplit", {})`.
Degree 1, `S = [e0, e1]`: root `F.mul(F.neg(e0), F.inv(e1))` (1 inv, 1 mul).
Terms: start with `{i1: 1, i2: 1}`, add 1 (or 2 for a double root) at
`index[(X, Y)]` for each root. A root point missing from `index` is an
instrument error (raise).

Reference counts per trial (tests check them): Case L, `m != 0`: nonsplit
30 mul + 1 inv; split with two distinct roots 34 mul + 2 inv; double root 32 mul +
2 inv. Case L, `m == 0`: 28 mul + 2 inv. Case V: nonsplit 12 mul; split distinct
14 mul + 1 inv; double root 13 mul + 1 inv.

### 7.3 Verifier (uncounted; gates G2, G3)

`verify_relation(curve, P1, P2, terms)`:
(a) every point in `terms` satisfies the quartic equation;
(b) every point is on the line through `P1, P2` (`x = x1` in case V; `y = m x + c` in case L,
with `m`, `c` recomputed by `u_` arithmetic);
(c) total multiplicity plus (1 if case L and `m == 0` else 0) equals 4;
(d) the product `lead * prod (Z - z_i)^{mult_i}` (where `z` is `Y` in case V and `X` in
case L, `lead` the leading coefficient of the full section polynomial recomputed
by `u_pmul`) equals the full section polynomial after trim;
(e) push-forward: `sum mult_i (x_i, w_i)` computed with `eadd` under a throw-away
`Ops` equals `None` (`O_E`).
Return a dict `{"G2": bool, "G3": bool}` (`G2` = a and b and c and d).

### 7.4 Rows

`make_row(terms, col, sign)`: `acc[col[i]] += sign[i] * mult` (Python ints); drop
zero entries; `key = tuple(sorted(acc.items()))`. Empty key is a `zero_row`.
A key already present in the arm's set is a `duplicate_row`. Otherwise the row
is kept; coefficients are reduced mod `r` only inside the LA.

## 8. One IC run (`code/ic_run.py`)

CLI: `--instance-json '<one line of instances.jsonl>' --s S --arm {A,B,C,Bp,N} --out PATH`.
`Bp` means B' (stream seed uses `s + 100`). The run is cold: it rebuilds
everything from the instance line.

### 8.1 Setup (SETUP counters)

`ops_setup = Ops()`; `F = Fp(p, ops_setup)`; `setup_points(F)`. Anchor: for `i` in
sorted order, `Z = emul(F, (x_i, w_i), c)` (counted in SETUP); the first `i0` with
`Z is not None` is the anchor point. `ss = 7919 * cs + s_eff` with
`s_eff = s + 100` for `Bp`, else `s`. Column map (section 6); `j0 = col[i0]`.

### 8.2 Collection (COLLECT counters) and stop rule

`ops_col = Ops()`, a second `Fp(p, ops_col)` sharing the SQRT table (table reads
count `lookup` in `ops_col`). `rng = random.Random(ss)`. For `t = 0, 1, 2, ...`:
`i = t % N_aff`; `j = rng.randrange(N_aff - 1)`; if `j >= i`: `j += 1`. Compute the
section (7.1). For every split section: verify it (7.3; a G2 failure raises, a G3
failure is recorded and the run stops with `"status": "G3_fail"`), count it in
`relations_verified`, then build the row (7.4).
`stream_digest` (uncharged, computed once at the start of the run): a separate
`random.Random(ss)` generates the first 1000 pairs by the same rule; digest =
sha256 hex of the concatenation of `f"{i},{j};"`.
Track `cover[col]` (number of kept rows containing the column).
Stop collecting and try the LA when `len(rows) >= ncols + 20` and every column is
covered, or abort with `"status": "trial_cap"` at `t = 40 * N_aff`.

### 8.3 Linear algebra (LA counters, `code/la.py`)

`ops_la = Ops()` counting `mul` (= MR) and `inv` (= IR) over `Z/r`; all values in `[0, r)`.
Unknown columns: all columns except `j0`, renumbered `0..N-1` in increasing column
order, `N = ncols - 1`. `A`: the kept rows (all of them), entries
`(newcol, coef mod r)` for `col != j0`; `b_i = (-coef_{i,j0}) mod r` (0 if absent).
`R = len(rows)`, `W` = total entries of `A`.

Attempt `a = 1, 2, ...` with `la_rng = random.Random(ss + 50000 + 1000 * a)`:
`D = [la_rng.randrange(1, r) for _ in range(R)]`, then
`u = [la_rng.randrange(r) for _ in range(N)]`.

- `Av(v)`: `[sum(coef * v[c] for c, coef in row) % r for row in A]`; `ops.mul += W`.
- `ATt(t)`: `out[c] += coef * t[i]` for every entry, then `% r`; `ops.mul += W`.
- `Bv(v) = ATt([D[i] * Av(v)[i] % r for i])`; `ops.mul += R` for the `D` step (so `2W + R` per call).
- `dot(u, v)`: `ops.mul += N`.
- `y = ATt([D[i] * b[i] % r])` (`ops.mul += R + W`).
- Sequence `s_k = dot(u, B^k y)`, `k = 0 .. 2N - 1` (`2N - 1` calls of `Bv`).
- Berlekamp-Massey over `Z/r`:

```
Cp = [1]; Bp = [1]; L = 0; m = 1; b = 1; binv = 1
for n in range(2N):
    d = s[n]; for i in 1..L: d = (d + Cp[i] * s[n-i]) % r        # ops.mul += L
    if d == 0: m += 1; continue
    coef = d * binv % r                                         # ops.mul += 1
    T = Cp[:]; extend Cp with zeros to len(Bp) + m
    for i, bi in enumerate(Bp): Cp[i+m] = (Cp[i+m] - coef * bi) % r   # ops.mul += len(Bp)
    if 2 * L <= n: L = n + 1 - L; Bp = T; b = d; binv = pow(b, r-2, r) (ops.inv += 1); m = 1
    else: m += 1
```

  Trim `Cp` to length `L + 1`. Minimal polynomial `g_k = Cp[L - k]` (`k = 0..L`).
- If `L == 0` or `g_0 == 0`: attempt fails.
- Horner: `z = [g_L * yk % r]` (`ops.mul += N`); for `k = L-1 down to 1`:
  `z = Bv(z)`; `z = [(zi + g_k * yi) % r]` (`ops.mul += N`). Then `ig = inverse(g_0)`
  (`ops.inv += 1`); `x = [(-ig * zi) % r]` (`ops.mul += N`).
- Check `Av(x) == b` (`ops.mul += W`, charged). Equal: success. Else: attempt fails.

After a failed attempt `a`: if `a` is odd, retry the same rows (attempt `a + 1`);
if `a` is even, resume the collection stream (same `rng`, same `t` counter) until
`ceil(0.05 * ncols) + 1` further rows are kept, rebuild `A, b, R, W`, then attempt
`a + 1`. After attempt 12 fails: `"status": "la_fail"`.

### 8.4 Certification (uncounted; gate G6) and the sigma check

Full solution: `xs[j0] = 1`, `xs[col] = x[newcol]`. With a throw-away `Ops`:
`Zj = emul(reps[j], c)` for every column; certified iff for every column `j`,
`emul(Z_{j0}, xs[j]) == Zj`. Arm A also reports
`sigma_equal = #{i < i' : x_i == x_i', y_i' == p - y_i, y_i != 0, xs[i] == xs[i']} / #{such pairs}`.

### 8.5 Tamper test (gate G4, arm A at `p = 263` only)

Take the first kept relation `(P1, P2, terms)`; let `k` be the smallest point index in
`terms` other than those of `P1`, `P2` (or `P2` if none); replace it by `(k + 1) % N_aff`
(if that index is already in `terms`, use `(k + 2) % N_aff`); run `verify_relation`;
record `tamper_rejected = not (G2 and G3)`. Must be `true`.

### 8.6 Output (one JSON object, keys sorted, written to `--out`)

```
family, p, k, cs, j, s, arm, ss, status ("ok" | "G3_fail" | "trial_cap" | "la_fail"),
N_aff, N_ram, ncols, anchor_point, anchor_col,
trials, split_trials, relations, duplicate_rows, zero_rows, nonsplit,
rows_used, R, N, W, la_attempts,
setup {mul, inv, add, lookup, fpm}, collect {...}, la {...},
SETUP_FPM, COLLECT_FPM, LA_FPM, TOTAL,
g2_fail_count, g3_fail_count, relations_verified,
certified (bool), cert_fail_cols, sigma_equal (arm A, else null),
tamper_rejected (arm A at 263, else null),
stream_digest (section 8.2),
python_version, wall_s
```

`TOTAL = SETUP_FPM + COLLECT_FPM + LA_FPM`. `wall_s` is informational and never
used by `analyze.py`.

## 9. Other runs

### 9.1 `code/run_matrix.py`

CLI: `--instances raw/instances.jsonl --outdir raw/runs --log raw/run_matrix.log`.
Order, one subprocess at a time:

1. F1, for `p` ascending, `j` ascending, `s in (1, 2)`: arms in the rotated order
   `["A", "B", "C", "Bp"]` rotated left by `(j + s) % 4`.
2. N: F1 at `p in (263, 523)`, every `j`, `s = 1`.
3. PC: F0, for `p` ascending, `j`, `s in (1, 2)`: arms `A, B, C` rotated left by `(j + s) % 3`.
4. G: `generic_run.py` for each G instance, `s = 1`.
5. R: `rho_run.py` for each F1 instance, walk index `0..4`.
6. Determinism (G8): rerun F1 `p = 523, j = 0, s = 1, arm B` to `raw/runs/repeat_F1_p523_j0_s1_B.json`.

File names: `raw/runs/{family}_p{p}_j{j}_s{s}_{arm}.json`,
`raw/runs/G_p{p}_j{j}.json`, `raw/runs/R_p{p}_j{j}_w{w}.json`. Each command:
`["bash", "-c", "ulimit -t 600 -v 2000000; exec python3 code/ic_run.py ..."]`.
A nonzero exit (including the CPU-limit kill) writes
`{"status": "not_run", "returncode": rc, "stderr_tail": last 2000 chars}` to the
run's file. Log one line per run: file, return code, status.

### 9.2 Rho reference (`code/rho_run.py`)

CLI: `--instance-json .. --w W --out ..`. Setup uncharged: `G = emul(Q0, c)` for the first
affine point `Q0` of `E` in `(x, w)` order with nonzero result (points of `E` from an
uncounted enumeration); `k_true = random.Random(cs + 777).randrange(1, r)`; `T = emul(G, k_true)`.
Charged (`Ops`): `rng = random.Random(cs + 900 + w + 1000 * restart)`;
20 pairs `(al_j, be_j) = (rng.randrange(r), rng.randrange(r))`,
`M_j = eadd(emul(G, al_j), emul(T, be_j))`; start `(a0, b0)` likewise, `X0 = aG + bT`.
Step: `j = X.x % 20` (0 for `O`); `X = eadd(X, M_j)`; `a += al_j`, `b += be_j` mod `r`.
Floyd: tortoise one step, hare two steps per iteration, until `X_t == X_h`. If
`b_t == b_h` mod `r`: restart (`restart += 1`, at most 10, charged). Else
`k = (a_t - a_h) * inverse(b_h - b_t) mod r` (1 inv, 1 mul added to the counter).
Verify `emul(G, k) == T` (uncharged). Output keys: `family, p, j, w, r, k_true, k_found,
correct, restarts, iterations, group_ops, fpm, mul, inv`.

### 9.3 Generic quartic control (`code/generic.py`, `code/generic_run.py`)

Curve: `F(x, y) = sum g[i][jj] x^i y^jj` over `i + jj <= 4`, `g[0][4] = 1`, other 14
coefficients `rng.randrange(p)` with `rng = random.Random(cs)`, drawn in the order
`for jj in 0..4: for i in 0..4-jj: (skip (0,4))`. Accept iff `g[4][0] != 0`, no affine
rational point with `F = F_x = F_y = 0`, no rational point `(1:t:0)` at infinity
with all three homogeneous partials zero, and `N_aff >= 8` (uncounted brute-force
enumeration over all `(x, y)`; affine points sorted). Reject reasons
`g40_zero`, `singular`, `few_points` go to `rejects.tsv` with family `G`.

Run (`--instance-json .. --s 1 --out ..`): trials `t = 0 .. 2 * N_aff - 1`, pairs
exactly as 8.2 with `ss = 7919 * cs + 1`; section: case L: `Y1 = [c, m]`,
`Y2 = pmul(Y1, Y1)`, `Y3 = pmul(Y2, Y1)`, `Y4 = pmul(Y3, Y1)`,
`S = sum_jj pmul([g[i][jj] for i in 0..4-jj], Y_jj)` (`Y0 = [1]`), trim, divide by `x1`, `x2`;
case V: `S[jj] = sum_i g[i][jj] x1^i` with powers of `x1` by repeated `F.mul`,
divide by `y1`, `y2`. Residual as 7.2 (degree 2, 1 or 0; degree 0 means the other two
intersections are at infinity: count as split). Count a trial as split iff the
residual splits. Output: `family "G", p, j, trials, split_trials, yield = split/trials,
collect {mul, inv, fpm}, fpm_per_trial`. No rows, no LA.
F1's yield for the comparison = `split_trials / trials` of the arm-A runs.

## 10. Unit tests (`code/test_l12.py`, `unittest`)

Toy curve `T23 = Curve(23, a1=1, a3=2, a2=0, a4=3, a6=5)`. Expected values were computed by
brute force while writing this spec; they are test vectors, not results.

1. Invariants mod 23: `disc_f = 22`, `DeltaE = 7`, `c4 = 16`, `c6 = 2`.
2. `setup_points`: sorted affine points
   `(5,6) (5,7) (5,16) (5,17) (6,1) (6,22) (7,3) (7,20) (10,0) (14,10) (14,13)`
   (`N_aff = 11`, `N_ram = 1`), `#E(F_23) = 18`; SETUP counts exactly `mul = 120`, `inv = 1`.
3. `counts.py`: `#C(F_23) = 12`, `#E(F_23) = 18`, `#C(F_{23^2}) = 564`,
   `#E(F_{23^2}) = 540`, `s1 = 6`, `T2 = -24`, `s2 = 30`, `nP = 416`.
   Also cross-check `counts.py` against brute force over all `(x, y)` in
   `F_{p^2} x F_{p^2}` for `T23` and for `Curve(7, 1, 1, 0, 1, 3)` and `Curve(11, 0, 1, 2, 3, 4)`
   when those are smooth (skip a curve whose `disc_f` or `DeltaE` is 0).
4. Column maps on T23: `ncols` A 11, B 6, C 5. B columns: `(5,6)->0 (5,7)->1 (5,16)->1
   (5,17)->0 (6,1)->2 (6,22)->2 (7,3)->3 (7,20)->3 (10,0)->4 (14,10)->5 (14,13)->5`.
   C: columns `0,0,0,0,1,1,2,2,3,4,4` with signs `-1,+1,+1,-1,+1,+1,-1,-1,+1,+1,+1`;
   C reps `(5,3) (6,1) (7,5) (10,0) (14,8)`.
5. Section `((5,7), (6,1))`: `m = 17`, `c = 14`, section `S = [2, 11, 12, 14, 8]`
   (low first), residual `[20, 10, 8]`, `D = 12`, `SQRT[12] = 9`, roots `X = 10` then `6`,
   terms `{(5,7): 1, (6,1): 2, (10,0): 1}`; counts `mul = 34`, `inv = 2` (FPM 54);
   verifier `G2 = G3 = True`. Rows: A `((1,1),(4,2),(8,1))`, B `((1,1),(2,2),(4,1))`,
   C `((0,1),(1,2),(3,1))`.
6. Section `((5,6), (6,1))`: `m = 18`, `c = 8`, `S = [10, 11, 2, 3, 4]`, residual
   `[8, 1, 4]`, `D = 11`, nonsplit; counts `mul = 30`, `inv = 1`.
7. Section `((5,6), (5,7))` (case V): `S = [16, 0, 7, 0, 1]`, residual `[19, 13, 1]`,
   `D = 1`, roots `Y = 17` then `16`; counts `mul = 14`, `inv = 1`. Rows:
   A `((0,1),(1,1),(2,1),(3,1))`, B `((0,2),(1,2))`, C zero row.
8. `eadd` on T23: `(6,1) + (6,1) = (5,3)` with counts `mul = 10`, `inv = 1`;
   `(5,3) + eneg((5,3)) = None`; every affine point `P` of `E(F_23)` satisfies
   `eadd(P, eneg(P)) is None` and `emul(P, 18) is None`.
9. Push-forward on T23: every split section over all unordered pairs of affine points
   passes G3 (sum `= O`); the count of split sections over pairs with `x1 != x2` is 6.
10. Tamper: changing `(10,0)` to `(14,10)` in test 5's terms makes `G2` false.
11. LA: over `Z/101`, `rng = random.Random(4242)`; `v = [1] + [rng.randrange(1, 101) for _ in range(24)]`;
    40 rows, each: 4 distinct columns `rng.sample(range(25), 4)`, the first three coefficients
    `rng.randrange(1, 101)`, the fourth chosen so that `sum coef * v[col] = 0 mod 101`
    (if it comes out 0, draw the row again). If some column is uncovered, draw the whole
    matrix again from the same `rng`. `la.solve` with anchor column 0 (same attempt and
    retry logic as 8.3 but with the fixed rows; no new rows) recovers `v[1:]` exactly;
    repeating with the same seeds gives identical `ops` counts.
12. Berlekamp-Massey: for the sequence `s_n = 2^n mod 101`, `L = 1` and `g = [-2 mod 101, 1]`.
13. Stream determinism: for T23 and `ss = 12345` the first 20 `(i, j)` pairs are identical
    across two calls, `i != j` always, and `i = t % 11`.
14. End to end on F1 at `p = 263` (first admissible instance from the selection rule):
    arms A, B, C each return `status = "ok"`, `certified = True`, equal `stream_digest`,
    arm A `sigma_equal = 1.0`; arm N returns `certified = False` or `status = "la_fail"`.

## 11. Analysis (`code/analyze.py`)

CLI: `--instances .. --runs .. --summary raw/summary.tsv --decision raw/decision.json`.

`summary.tsv` header (one row per run file, `not_run` rows included with empty metrics):
`family\tp\tj\ts\tarm\tstatus\tN_aff\tncols\ttrials\trelations\trows_used\tR\tN\tW\tla_attempts\tSETUP_FPM\tCOLLECT_FPM\tLA_FPM\tTOTAL\tcertified\tsigma_equal\tstream_digest`.

Paired ratios per `(family, p, j, s)`: `TOTAL_A/TOTAL_B`, `COLLECT_A/COLLECT_B`,
`LA_A/LA_B`, `relations_A/relations_B`, `ncols_A/ncols_B`, `TOTAL_B/TOTAL_C`,
`TOTAL_A/TOTAL_C`, `TOTAL_B/TOTAL_Bp`; a pair with any member not `ok` is
excluded and counted. Per `(family, p)`: `median, min, max, n` of each ratio (use
`statistics.median`). Rho: median `fpm` per instance; gap `TOTAL_X / rho_median`
for X in A, B, C. Yields: pooled F1 arm-A yield and G yield per `p`.

`decision.json`: the gates (`G0` from `raw/unittest.log` last line containing `OK`;
`G1` from select exit status; `G2`, `G3` totals; `G4` all `tamper_rejected`;
`G5` digests equal per `(p, j, s)` over A, B, C (and N where run); `G6` certified for
A, B, C, Bp, PC; `G7` N certified count; `G8` repeat equality ignoring `wall_s`),
`R_p` per prime, count of values `>= 1.5` at 2063, A/A spread at 2063,
positive-control medians at 2069, and `verdict` in
`{"supported", "killed", "inconclusive", "invalid"}` computed exactly by PROTOCOL.md
section 9 (invalid when a gate other than G3 fails). Also write the same tables as
Markdown to stdout.
