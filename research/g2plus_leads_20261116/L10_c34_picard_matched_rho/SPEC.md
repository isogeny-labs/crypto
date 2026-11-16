# L10 implementation specification (frozen with PROTOCOL.md)

This file is the executor's complete contract. It leaves no research choice open. If
something here is impossible as written, stop, record the problem in `raw/spec_issues.txt`,
and do not improvise a substitute. Lead directory:
`L = /home/user/crypto/research/g2plus_leads_20261116/L10_c34_picard_matched_rho`.
Write only inside `L`.

## 1. Layout and language

```
L/code/Cargo.toml        standalone crate "l10", edition 2021, no dependencies, empty [workspace]
L/code/src/main.rs       CLI + `#[path]` include of the lab Pic arithmetic
L/code/src/fp.rs         counted F_p layer (Cnt, Fp)
L/code/src/curves.rs     curve generation (families Q, C, P), zeta, F-evaluation cost rule
L/code/src/enumerate.rs  counted Picard enumeration; uncounted brute-force enumeration
L/code/src/lines.rs      line relation search, arms Q_full/C_full/P_full/P_orbit, dedup keys
L/code/src/la.rs         counted Wiedemann (Krylov, BM, reconstruction); dense GE mod r (uncounted)
L/code/src/picrho.rs     rho on PicClass; random classes
L/code/src/jac.rs        #J of Picard curves from point counts over F_p, F_p^2, F_p^3
L/code/src/tiny.rs       subcommand `tiny`
L/code/src/measure.rs    subcommand `measure`
L/code/src/rhocost.rs    subcommand `rhocost`
L/code/analyze.py        Python 3 stdlib analysis
L/raw/                   all outputs (create it)
```

`Cargo.toml`:

```
[package]
name = "l10"
version = "0.1.0"
edition = "2021"

[workspace]

[profile.release]
opt-level = 3
debug = false
```

`main.rs` starts with:

```rust
#[allow(dead_code, clippy::all)]
#[path = "../../../../../src/cryptanalysis/plane_quartic_pic.rs"]
mod plane_quartic_pic;
```

The path is relative to `L/code/src/`. The lab file is std-only and compiles standalone.
This was checked before the freeze. Do not copy or modify it. Use only its public API:
`PlaneQuartic::{new, counter, stats, add, double, neg, point_class, mul_scalar, zero,
is_zero, normalize_point, class_from_conics, rational_points_bruteforce}`, `PicClass`,
`PicClass::key`, `FqCounter::since`, `smoothness_certificate`, `xorshift_next`,
`next_smooth_quartic_with_point`, `find_rational_point`, `mons_of_degree`.

CLI parsing: hand-written (`std::env::args`). Every subcommand appends JSON lines to `--out`
(open with append and create). JSON is written by hand (`format!`), with keys in the order
listed below. Integers are printed as integers, `f64` with `{:.6}`, and `bool` as
`true`/`false`.

## 2. Notation

- `p`: field prime. `Pt = [u32; 3]` with `[X0, X1, X2]`. Affine chart `X0 = 1`,
  `x = X1`, `y = X2`.
- Points are normalised as in the lab: the first nonzero coordinate is 1.
- `mons_of_degree(4)` index table (a, b, c = exponents of X0, X1, X2):

```
0:(4,0,0) 1:(3,1,0) 2:(3,0,1) 3:(2,2,0) 4:(2,1,1) 5:(2,0,2) 6:(1,3,0) 7:(1,2,1)
8:(1,1,2) 9:(1,0,3) 10:(0,4,0) 11:(0,3,1) 12:(0,2,2) 13:(0,1,3) 14:(0,0,4)
```

- `ell = 2^61 - 1` (LA proxy modulus). `lambda_proxy = 1669582390241348315` (equal to
  `5^((ell-1)/3) mod ell`; check `lambda^2 + lambda + 1 = 0 mod ell` in a unit test).

## 3. Counted F_p layer (`fp.rs`)

```rust
pub struct Cnt { pub mul: u64, pub inv: u64 }
pub struct Fp { pub p: u64 }
impl Fp {
  fn mul(&self, a: u64, b: u64, c: &mut Cnt) -> u64  // (a*b)%p, c.mul += 1
  fn inv(&self, a: u64, c: &mut Cnt) -> u64          // extended Euclid, c.inv += 1; panics on 0
  fn add/sub/neg(...) -> u64                         // uncounted
  fn mul_u(&self, a, b) -> u64                        // UNCOUNTED, for setup/bookkeeping only
}
fn fpm(c: &Cnt) -> u64 { c.mul + 10 * c.inv }
```

Mod-`ell` arithmetic for LA (`la.rs`): `mulmod_ell(a, b)` via u128 product and Mersenne
reduction. Mod-`r` arithmetic (tiny tier): u128 `%`. Every LA mulmod increments
`la.mul`, and every LA inversion increments `la.inv`.

## 4. Curves (`curves.rs`)

RNG: the lab's `xorshift_next(&mut u64)`. Draw `v = xorshift_next(&mut s) % p`.

**Family Q (generic quartic).** `s = 1000 + curve_seed`. Call
`next_smooth_quartic_with_point(p, &mut s)` and use the returned `(f, p0)` unchanged
(`p0` normalised by `normalize_point` after building).

**Family P (Picard).** `s = 2000 + curve_seed`. Loop:
`a2 = draw, a1 = draw, a0 = draw`. Then build `f` with `f = [0; 15]`:

- `f[0] = (p - a0) % p`
- `f[1] = (p - a1) % p`
- `f[3] = (p - a2) % p`
- `f[9] = 1`
- `f[10] = p - 1`

This is `F = X0 X2^3 - X1^4 - a2 X0^2 X1^2 - a1 X0^3 X1 - a0 X0^4`. Accept when
`smoothness_certificate(p, &f)`, else continue the same stream. `p0 = [0, 0, 1]`.

**Family C (generic C_{3,4}).** `s = 3000 + curve_seed`. Loop: draw 9 values, assigned in
this order to `f[0], f[1], f[3], f[6], f[2], f[4], f[7], f[5], f[8]` (that is
`c00, c10, c20, c30, c01, c11, c21, c02, c12`). Then set `f[9] = 1`, `f[10] = p - 1`, and
`f[11..=14] = 0`. Accept when smooth. `p0 = [0, 0, 1]`.

In all families, build `PlaneQuartic::new(p, f, p0)`. It must return Ok; panic otherwise.

**zeta** (`p = 1 mod 3`): the smallest `g >= 2` with `t = g^((p-1)/3) mod p != 1`; then
`zeta = t`. Uncounted setup. Expected values: p=7:4, 13:3, 19:7, 31:25, 37:26, 43:36,
271:242, 523:462, 1033:837, 2053:1855, 4099:2017, 8209:3265, 16411:13654.

**F-evaluation cost rule** `eval_F(f, pt, &mut Cnt) -> u64`:

1. For each coordinate `i` in 0..3 compute `xi2 = xi*xi`, `xi3 = xi2*xi`, `xi4 = xi2*xi2`.
   That is always 9 counted multiplications.
2. For each monomial index `m` with `f[m] != 0`:
   - let `k` = the number of exponents among `(a, b, c)` that are > 0;
   - form the product of those `k` power values (`k - 1` counted multiplications);
   - then multiply by `f[m]` (1 counted multiplication) **unless** `f[m] == 1` or
     `f[m] == p - 1` (then add or subtract, uncounted);
   - accumulate uncounted.

Hand values (U1). Picard with `a2, a1, a0` all outside {0, 1, p-1}:

| monomial | count |
|---|---|
| (4,0,0) | 1 |
| (3,1,0) | 2 |
| (2,2,0) | 2 |
| (1,0,3) | 1 |
| (0,4,0) | 0 |

That gives 9 + 6 = **15**. With `a2 = 0` the total is **13**. With `a1 = 1` (so
`f[1] = p - 1`) it is **14**.

## 5. Enumeration (`enumerate.rs`)

**Uncounted brute force** `points_bruteforce(f, p) -> Vec<Pt>`, in the lab order:

1. For `x1` in 0..p: the coefficients of `F(1, x1, X2)` in `X2` (degree <= 4, uncounted).
   For `x2` in 0..p evaluate by Horner; on 0, push `[1, x1, x2]`.
2. For `x2` in 0..p: if `F(0, 1, x2) == 0`, push `[0, 1, x2]`.
3. If `F(0, 0, 1) == 0`, push `[0, 0, 1]`.

This order is **the** point order of every arm. `idx: HashMap<Pt, usize>` maps each point
to its position.

**Counted Picard enumeration** `points_picard_counted(a2, a1, a0, p, &mut Cnt) -> Vec<Pt>`:

1. Cube table. For `y` in 1..p: `c = y*y` (1 mul), `c = c*y` (1 mul); push `y` into
   `cubes[c]` (`Vec<Vec<u32>>` of size p). That is `2(p-1)` mul.
2. For `x` in 0..p: `x2 = x*x` (1); `v = x2*(x2 + a2)` (1); `v = v + a1*x` (1); `v += a0`.
   That is 3 mul per `x`.
   - If `v == 0`, push `[1, x, 0]`.
   - Else push `[1, x, y]` for each `y` in `cubes[v]`.
3. Push `[0, 0, 1]`.

So `enum_mul = 2(p-1) + 3p` and `enum_inv = 0`. Gate M1: the set equals the brute-force set.

**Orbits** (P family, bookkeeping uncounted except where stated). For each affine point with
`y != 0`, its orbit is `{(x, y), (x, zeta y), (x, zeta^2 y)}`.

- `rep` is the member with the smallest `y` as an integer.
- Orbit ids are assigned in increasing order of the rep's index in the brute-force list.
- For each point, `k` in {0, 1, 2} is defined by `y = zeta^k y_rep`.
- `P_orbit` adds **2 counted mul per orbit** to its `enum_mul`, for computing `zeta y_rep`
  and `zeta^2 y_rep`.
- Fixed points: `P0 = [0,0,1]` and affine points with `y = 0`. Their orbit is `None`.

## 6. Line relation search (`lines.rs`)

Setup (charged to the line phase of every arm): the square-root table
`sqrt_of: Vec<Option<u32>>` with `sqrt_of[0] = Some(0)`; for `y` in 1..=(p-1)/2,
`s = y*y` (1 counted mul) and `sqrt_of[s] = Some(y)`. So `line_setup_mul = (p-1)/2`.
Constants `i2 = 2^-1`, `i3 = 3^-1` are uncounted setup.

Pair stream: `state = 7_000_003 * solver_seed + 1_000 * curve_seed + p` (u64, wrapping).
A trial is: `i = xorshift_next(&mut state) % N`, `j = xorshift_next(&mut state) % N`;
if `i == j`, redraw both (no trial counted). `N` = number of points including `P0`.
`P = pts[i]`, `Q = pts[j]`.

Per trial (`trials += 1`):

1. `E1 = P + Q`, `E2 = P - Q`, `E3 = P + 2Q` (coordinate-wise mod p, uncounted).
   `e_k = eval_F(E_k)` (three counted evaluations).
2. Solve the residual coefficients (4 counted mul):
   - `c2 = (e1 + e2)*i2`
   - `u = (e1 - e2)*i2`
   - `w = e3*i2 - 2c2` (`2c2` by addition)
   - `c3 = (w - u)*i3`
   - `c1 = u - c3`
3. If `c1 = c2 = c3 = 0`: panic (gate failure; impossible on a smooth curve).
4. `delta = c2*c2 - 4*(c1*c3)` (2 counted mul; `4*` by additions).
5. Residual points:
   - **If `c1 == 0`:** root 1 is `P` (no work).
     - If `c2 != 0`: root 2 is `(s, t) = (-c3, c2)`; compute
       `R = s*P + t*Q` (6 counted mul) and normalise (1 inv + 2 mul). Then `tangent = false`.
     - If `c2 == 0`: both roots are `P` (`tangent = true`). Residuals are `[P, P]` or
       `[P, R]`.
   - **Else**, look up `sqrt_of[delta]`:
     - `None`: `nonsquare += 1`, end of trial.
     - `delta == 0`: `tangent += 1`. One point `(s, t) = (-c2, 2c1)`: 6 mul + normalise.
       Residuals are `[R, R]`.
     - Otherwise, with `d = sqrt_of[delta]`: `(s, t) = (-c2 + d, 2c1)` and
       `(-c2 - d, 2c1)`, each 6 mul + normalise. Residuals are `[R1, R2]`.

   Normalise: find the first nonzero coordinate `z`, `iz = inv(z)` (1 inv), and multiply
   the other two coordinates by `iz` (2 mul, always counted), setting `z` to 1.
6. Look up each residual in `idx`. It must exist (panic otherwise: "residual not on curve").
   The relation multiset is `M = [P, Q, Ra, Rb]`.
7. Build the row and its key for the arm (below). If the key was seen, `dup += 1`. If the
   row is empty, `trivial += 1`. Otherwise `accepted += 1` and store the row.
8. Stop when `accepted == n_cols + 10` (`rows_target`). Cap: if
   `trials > 60 * rows_target`, stop with `ok = false`.

Rows (sparse: `Vec<(col, coeff)>` sorted by col; coefficients stored exactly):

- **Q_full**:
  - Columns are the points other than `p0`, in brute-force order, then `kappa` as the last
    column (`n_cols = N - 1 + 1`).
  - Coefficient `+1` per occurrence of each non-`P0` point of `M` (duplicates add), and
    `kappa` coefficient `-1`.
  - Key: sorted `[idx of M]` (4 entries, `P0` included).
- **C_full**, **P_full**: the same without `kappa` (`n_cols = N - 1`).
- **P_orbit**:
  - Columns are the orbit ids (`n_cols = n_orbits`).
  - For each point of `M` with orbit `Some(o)` and exponent `k`, add 1 to `trip[o][k]`.
  - Then reduce each triple by subtracting `min(m0, m1, m2)` from all three, and drop
    all-zero triples. The row is `Vec<(o, [m0, m1, m2])>` sorted by `o`.
  - Key: the lexicographic minimum over the three rotations `r^0, r^1, r^2` of the row,
    where `r` maps every triple `(m0, m1, m2)` to `(m2, m0, m1)` (multiplication by
    `lambda`).
  - Coefficient value modulo a modulus `M` with cube root `lam`:
    `m0 + m1*lam + m2*lam^2 mod M` (uncounted matrix construction).

Line-phase counters: `line_mul`, `line_inv` (including `line_setup_mul`); plus `trials`,
`accepted`, `dup`, `trivial`, `nonsquare`, `tangent`.

`P_full` and `P_orbit` call the same search with the same `state` initialisation.

**Identity-quotient flag (test only):** `orbit_identity = true` makes every point other
than `P0` its own orbit with `k = 0`. Then `P_orbit` must reproduce `P_full`'s `trials`,
`accepted`, `dup`, `nonsquare`, `tangent`, `line_mul` and `line_inv` exactly (U7).

## 7. Linear algebra (`la.rs`)

Input: the first `n = n_cols` accepted rows (a square `n x n` matrix `A`), with coefficients
reduced modulo `M` (`M = ell` at the measurement tier). `w` = the number of nonzero
entries of `A` after reduction.

LA RNG: `st = 0xDEADBEEF ^ (solver_seed << 32) ^ p`. `u[i]`, `b[i]` = `xorshift_next % M`
(uncounted).

1. **Krylov:** `v = b`; for `i` in 0..2n:
   - `s[i] = sum_j u[j]*v[j]` (n mul);
   - `v = A v` (one mul per stored nonzero: `w` mul).

   So `krylov_mul = 2n(n + w)` exactly (gate M4).
2. **Berlekamp-Massey** on `s[0..2n]`, counted:

```
C=[1]; B=[1]; L=0; m=1; bb=1
for k in 0..2n:
    d = s[k] + sum_{i=1..L} C[i]*s[k-i]            # L mul
    if d == 0: m += 1; continue
    coef = d * inv(bb)                               # 1 inv, 1 mul
    T = C.clone(); extend C with zeros to len max(len C, len B + m)
    for i in 0..len(B): C[i+m] -= coef*B[i]          # len(B) mul
    if 2L <= k: L = k+1-L; B = T; bb = d; m = 1
    else: m += 1
```

   Record `bm_mul`, `bm_inv`, `L_bm = L`.
3. **Reconstruction (counted surrogate of the solve):** `c0inv = inv(C[L_bm])` if nonzero,
   else `inv(1)` (1 inv). Then `acc = 0`, `v = b`. For `i` in 0..L_bm:
   - `acc += C[i] * v` (n mul);
   - `v = A v` (w mul).

   Then `acc *= c0inv` (n mul). So `recon_mul = L_bm(n + w) + n` and `recon_inv = 1`.

   No solution is used at the measurement tier.

`la_mul = krylov_mul + bm_mul + recon_mul`, `la_inv = bm_inv + recon_inv`,
`fpm_la_unw = la_mul + 10 la_inv`, `fpm_la_w = 9 * fpm_la_unw`.

**Dense GE mod r** (tiny tier, uncounted): `rank_and_kernel(rows, n, r)` returns the rank and
a kernel basis by reduced row echelon form.

## 8. rho on Pic and random classes (`picrho.rs`)

`zeta_inv = zeta^-1`, `zeta_inv2 = zeta_inv^2` (uncounted setup).

`rho_class(pq, x, own: &mut Cnt) -> PicClass`:

- `Exc(P)`: return `Exc(normalize_point([P0, P1, zeta*P2]))` (1 counted own mul).
- `Typ(m)`: copy the three rows. Degree-2 monomial order `mons_of_degree(2)` is
  `(2,0,0),(1,1,0),(1,0,1),(0,2,0),(0,1,1),(0,0,2)`, with X2-exponents
  `c = [0,0,1,0,1,2]`.
  - Multiply entry `j` of each row by `zeta_inv^c[j]` when `c[j] > 0`. That is 3 counted
    own mul per row, 9 in total.
  - Return `pq.class_from_conics(rows)` (its `F_q` work appears in the lab counter delta).

The R cost per application is the lab counter delta plus the own count.

`rho_point(P) = normalize([P0, P1, zeta*P2])` (uncounted helper).

`random_class(pq, pts, rng)`: three points `pts[xorshift_next(rng) % len]`;
`x = add(add(pc(a), pc(b)), pc(c))` with `pc = point_class`. Outside measured windows.

## 9. #J of a Picard curve (`jac.rs`, tiny tier)

`F_{p^k}` for k = 2, 3 is `F_p[t]/(t^k - c)`, with `c` the smallest integer `>= 2` such that
`c^((p-1)/k) != 1 mod p`. Elements are `[u64; k]`; multiply by schoolbook, then reduce
with `t^k = c`.

`N_k = 1 + sum over x in F_{p^k} of cnt_k[f(x)]`, where `cnt_k[v]` is the number of
`y in F_{p^k}` with `y^3 = v` (build a histogram over all `y`). The 1 counts `P0`, which is
the only point at infinity over every extension.

```
s_k = p^k + 1 - N_k   (i128)
e1 = s1; e2 = (s1^2 - s2)/2; e3 = (s1^3 - 3 s1 s2 + 2 s3)/6   (assert exact division)
#J = 1 - e1 + e2 - e3 + p e2 - p^2 e1 + p^3
```

Expected values (U3, from the coordinator's scratch script, to be confirmed independently by
U3b):

| p | (a2, a1, a0) | (N1, N2, N3) | #J |
|---|---|---|---|
| 7 | (0, 0, 1) | (4, 20, 364) | 144 |
| 13 | (0, 0, 1) | (4, 212, 2452) | 1200 |
| 13 | (1, 2, 3) | (11, 149, 2408) | 1701 |

U3b: for these curves, `mul_scalar(x, #J)` is zero for 8 random classes. This uses the lab
arithmetic, independent of the counting code.

Factorisation: trial division.

## 10. Subcommands

### 10.1 `tiny --p P --out FILE`

1. **Picard.** For `curve_seed` in 1..=200, generate the Picard curve, compute `#J` and
   `r` (largest prime factor), and accept if `r > p`, `r != 3` and `r^2` does not divide
   `#J`. Stop after 2 accepted; count the rejects. For each accepted curve:
   - T1: 8 random classes are killed by `#J` (rng `s = 4242 + curve_seed`). Hasse-Weil:
     `(sqrt p - 1)^6 <= #J <= (sqrt p + 1)^6`.
   - `h = #J/r`. `G = mul_scalar(random_class, h)`, redrawn until nonzero.
   - Table: `cur = zero`; for `k` in 0..r insert `cur.key() -> k`, then `cur = add(cur, G)`.
     T2: after the loop `cur` is zero and the table has r entries.
   - Logs: `l(P) = table[mul_scalar(point_class(P), h).key()]` for every point `P != P0`.
   - T3: `lambda = table[rho_class(G).key()]` and `(lambda^2 + lambda + 1) % r == 0`.
   - T4: for non-fixed `P`, `l(rho_point P) == lambda*l(P) mod r`; for fixed `P`,
     `l(P) == 0`.
   - T5 on 20 random classes `x, y` and 20 points:
     - `rho^3 x == x`
     - `add(add(x, rho x), rho^2 x)` is zero
     - `rho(add(x, y)) == add(rho x, rho y)`
     - `rho_class(point_class(P)) == point_class(rho_point(P))`
   - Relations. Run the search for `P_full` and `P_orbit` (solver seed 1) with
     `rows_target = 3 n_cols + 20` and the cap `200 * rows_target`. At the tiny tier,
     hitting the cap is **not** a failure: tiny curves have few distinct lines, so the run
     continues with the rows collected, and T7b classifies the result. Then:
     - T6: every accepted row's points satisfy `sum point_class == zero`.
     - T7a: every `P_full` row dotted with `l` is 0 mod r; every `P_orbit` row (with
       `lambda`) dotted with `l_rep` is 0 mod r.
     - T7b: dense GE kernel dimension and proportionality to `l` (resp. `l_rep`). Status is
       `determined` (dim 1, proportional), `underdetermined` (dim > 1 and logs in the kernel)
       or `fail`.
     - T7c: the `P_orbit` matrix with `lambda' = lambda^2 mod r` has kernel dimension 0
       (checked only when the T7b status is `determined`).
   - T8: run the LA counter on the `P_orbit` square system mod r (with `lambda`) and mod
     `ell` (with `lambda_proxy`); `krylov_mul` must be equal.
2. **C and Q.** curve seeds 1, 2: run `C_full` / `Q_full` with `rows_target = n_cols + 10`;
   T6 on every row. For Q, the class `sum point_class` must be identical for all rows.

Output, one line per curve:

```
{"schema":"l10.tiny/v1","family":..,"p":..,"curve_seed":..,"coeffs":[15 ints],"n_points":..,
 "jac":..|null,"r":..|null,"h":..|null,"rejects_before":..|null,"lambda":..|null,
 "n_cols_full":..,"n_cols_orbit":..|null,"accepted_full":..,"accepted_orbit":..|null,
 "T1":bool|null,"T2":..,"T3":..,"T4":..,"T5":..,"T6":bool,"T7a":..,"T7b":"determined|underdetermined|fail"|null,
 "T7c":bool|null,"T8":bool|null,"ok":bool}
```

### 10.2 `rhocost --family F --p P --curve-seed S --out FILE`

1. Build the curve. `rng = 1_000_003 * S + P`.
2. Collect 300 points with `find_rational_point(p, &f, &mut rng, 256)` (normalised).
3. Classes `x_i` for i in 0..100 from consecutive point triples.
4. For i in 0..99, measure each with counter deltas:
   - `s_i = add(x_i, x_{i+1})`: skip and count `n_add_skipped` if `x_i == x_{i+1}`;
   - `n_i = neg(s_i)`; R1 check: `add(s_i, n_i)` is zero (outside the window).
5. For i in 0..50: `double(s_i)`.
6. P only, for i in 0..99: `rho_class(s_i)`. Lab counter delta, plus own count.
7. T5 checks (P only) on the first 20 classes.
8. R1 commutation: `add(x_i, x_{i+1}) == add(x_{i+1}, x_i)` for i < 10.
9. Also record `stats()` deltas (exc_results, neg_pencil, lifts, p0_in_support) over the
   measured windows.

Output, one line per curve:

```
{"schema":"l10.rhocost/v1","family":..,"p":..,"curve_seed":..,"coeffs":[..],"p0":[..],
 "n_add":..,"add_mul":..,"add_inv":..,"n_neg":..,"neg_mul":..,"neg_inv":..,
 "n_dbl":..,"dbl_mul":..,"dbl_inv":..,"n_rho":..|0,"rho_mul_lab":..,"rho_inv_lab":..,"rho_mul_own":..,
 "stats":{"exc_results":..,"neg_pencil":..,"lifts":..,"p0_in_support":..},
 "R1":bool,"T5":bool|null,"ok":bool}
```

The `*_mul` and `*_inv` fields are sums over the measured operations.

### 10.3 `measure --p P --families LIST --out FILE`

Fixed order: for `curve_seed` in 1..=3, for `solver_seed` in 1..=2, for `family` in LIST
order (P expands to `P_full` then `P_orbit`):

1. Generate the curve and the brute-force points. M3: Hasse-Weil
   `|N - (p+1)| <= 6 sqrt(p)`.
2. For P: run the counted enumeration and check M1.
   - `enum_mul` is the counted value for `P_full`, and that value plus `2*n_orbits` for
     `P_orbit`.
   - For C and Q, `enum_mul = 0` ("setup, uncharged").
3. Run the line search (`rows_target = n_cols + 10`).
4. M2: verify the first 50 accepted rows in Pic. The reference class is `zero` for C/P and
   the first row's class for Q. Use a separate `PlaneQuartic` instance so the counters are
   untouched.
5. Run LA mod `ell` (`lambda_proxy` for `P_orbit`). M4: `krylov_mul == 2n(n + w)`.
6. Totals:
   - `fpm_enum = enum_mul + 10 enum_inv`
   - `fpm_line = line_mul + 10 line_inv`
   - `fpm_total = fpm_enum + fpm_line + fpm_la_w`

Output, one line per run:

```
{"schema":"l10.measure/v1","family":..,"arm":"Q_full|C_full|P_full|P_orbit","p":..,"curve_seed":..,
 "solver_seed":..,"coeffs":[..],"zeta":..|null,"n_points":..,"n_fixed":..|null,"n_orbits":..|null,
 "n_cols":..,"rows_target":..,"trials":..,"accepted":..,"dup":..,"trivial":..,"nonsquare":..,
 "tangent":..,"enum_mul":..,"enum_inv":..,"line_mul":..,"line_inv":..,"line_setup_mul":..,
 "la_n":..,"la_w":..,"krylov_mul":..,"bm_mul":..,"bm_inv":..,"L_bm":..,"recon_mul":..,"recon_inv":..,
 "la_mul":..,"la_inv":..,"fpm_enum":..,"fpm_line":..,"fpm_la_unw":..,"fpm_la_w":..,"fpm_total":..,
 "M1":bool|null,"M2_verified":..,"M3":bool,"M4":bool,"M5":bool,"ok":bool}
```

## 11. Unit tests (`cargo test`; all must pass before any run)

- **U1:** eval_F counts are 15 / 13 / 14 for the three Picard coefficient patterns of §4,
  at p = 2053 with `(a2, a1, a0)` = (5, 7, 11), (0, 7, 11), (5, 1, 11).
- **U2:** zeta table of §4. `lambda_proxy^2 + lambda_proxy + 1 == 0 mod ell`.
- **U3:** `#J` and `(N1, N2, N3)` of §9. **U3b:** `#J` kills 8 random classes.
- **U4:** on the Picard curve p = 2053, `(a2, a1, a0)` = (5, 7, 11), for 200 trials:
  - every residual point satisfies `F = 0`;
  - `det[P; Q; R] == 0` mod p for every residual R (collinearity);
  - the counted cost of a nonsquare trial is exactly `3*15 + 4 + 2 = 51` mul and 0 inv.
- **U5:** `A = [[1,2,0],[0,1,3],[4,0,1]]` mod ell: `w = 6` and `krylov_mul = 2*3*(3+6) = 54`.
- **U6:** BM on Fibonacci mod ell (`s0 = 0, s1 = 1`, 40 terms) gives `L = 2` and
  `C = [1, ell-1, ell-1]`.
- **U7:** identity-quotient `P_orbit` equals `P_full` (fields of §6) at p = 271,
  curve seed 1, solver seed 1.
- **U8:** P_orbit keys:
  - a row containing only `(5, [1,1,1])` reduces to empty;
  - rows `{1:(1,0,0), 2:(0,1,0)}` and `{1:(0,1,0), 2:(0,0,1)}` have equal keys;
  - rows `{1:(1,0,0)}` and `{1:(1,1,0)}` have different keys.
- **U9:** p = 13, Picard `(0, 0, 1)`:
  - the counted enumeration returns 4 points (`N1 = 4`): `P0` and one orbit of 3;
  - `enum_mul = 2*12 + 3*13 = 63`.
- **U10:** T5 identities on p = 31, Picard curve seed 1.
- **U11:** family Q at p = 2053, seed 1, equals `next_smooth_quartic_with_point(2053, &mut 1001)`.

## 12. `analyze.py` (stdlib only)

Inputs: `raw/tiny.jsonl`, `raw/rhocost.jsonl`, `raw/measure.jsonl`, `raw/pc.jsonl`,
`raw/replay_cmp.txt`, and `--bound-r` (read-only).

1. **Gates.**
   - Tiny: all T1-T6 true; T7a true; T7b `determined` on at least 8 of 10 Picard curves and
     never `fail`; T7c true wherever checked; T8 true.
   - Measure/pc: M1 (P), M2_verified == min(50, accepted), M3, M4, M5 on every row.
   - rhocost: R1 and T5.
   - Replay: `exit=0`.
   - Write `gates` with per-gate pass/fail and failing record keys.
2. **Part 1.** Per family and prime: `A = (add_mul + 10 add_inv)/n_add`, and the same for
   `N`, `D` and `R = (rho_mul_lab + rho_mul_own + 10 rho_inv_lab)/n_rho`, pooled over the
   3 curves (sums over sums).
   - `F_Q = (A+N)/(sqrt(2) A)` for Q.
   - S1, S2, S3, S6 for P (PROTOCOL §7), with argmin and `min/S2`.
   - R2: the median of `cantor_mul/group_ops` over `bound_r.jsonl` rows with
     `arm == "Q_rho_neg"`, `ok == true` and the same p, compared with `A_mul + N_mul`
     (`= (add_mul/n_add) + (neg_mul/n_neg)` for Q), tolerance ±5%.
   - Verdict per PROTOCOL §7. Include the affected file:line list verbatim from PROTOCOL §7.
3. **Part 2.** For each p in the measurement set, pair the `P_full` and `P_orbit` rows on
   `(curve_seed, solver_seed)`. Ratios:
   - `fpm_total`
   - `trials`
   - `fpm_la_w`
   - `fpm_line`
   - `fpm_enum`

   Report the geomean, min and max; for `fpm_total` also a bootstrap 95% interval
   (random.Random(12345), 2000 resamples of the 6 pairs, percentile 2.5/97.5).
   - Verdict H3 and H3' per PROTOCOL §7.
   - A/A: `P_full` seed 1 / seed 2 `fpm_total` per curve, geomean per p, pass in [0.9, 1.1]
     for p >= 1033.
   - PC: per pair (1033/349, 4099/1381, 16411/5479), `C_full` geomean trials ratio against
     the geomean `(n_cols+10)` ratio, pass within ±15%. The C rows at 1033, 4099 and 16411
     come from `measure.jsonl`.
   - Yield `accepted/trials` per family and p.
   - `G(p)` and the proxy distance per PROTOCOL §7.
4. Write `raw/analysis.json` (all of the above, with verdict strings exactly
   `supported|killed|inconclusive|no material move|negation walk legitimate|gate failed`)
   and `raw/tables.md` (markdown tables of the same numbers). All numbers carry the label
   `[pcc:unverified]` in `tables.md`'s header.

## 13. Resource limits

One thread (`-j1`, `--test-threads=1`), with no parallelism in code. If any single command
exceeds 10 CPU-minutes or 2 GB, stop it, record this in `raw/spec_issues.txt`, and do not
change the instance set.
