# L09 implementation specification (frozen with PROTOCOL.md)

The implementer makes **no research choices**. Where this file is silent on a
detail that changes a number, stop and record the question in `raw/questions.txt`
instead of choosing. Status of every output: `[pcc:unverified]`.

## 1. Layout and language

All paths relative to `/home/user/crypto/research/g2plus_leads_20261116/L09_gonal_pencil_seeds/`.

```
code/Cargo.toml        standalone crate, no dependencies
code/src/main.rs       CLI (section 10)
code/src/rng.rs        SplitMix64, seed derivation (section 3)
code/src/field.rs      counted F_p (section 2)
code/src/poly.rs       polynomials over F_p, split test, Cantor-Zassenhaus (section 4)
code/src/curve.rs      curve generation, points, fibres, line trial (sections 5-7)
code/src/graph.rs      parity union-find yield (section 8)
code/src/exact.rs      exact rank engine over F_ell (section 9)
code/src/arms.rs       arm runners, stopping, JSONL output (sections 6, 11)
code/analyze.py        Python 3 standard library analysis (section 13)
raw/                   outputs only
```

`code/Cargo.toml`:

```
[package]
name = "l09"
version = "0.1.0"
edition = "2021"

[dependencies]

[workspace]

[profile.release]
opt-level = 3
debug = false
```

Single thread. No `unsafe`. Use `u64` for F_p elements (p < 2^15) and `u128` for
mod-ell products.

## 2. Counted field F_p

```
pub struct Fp { pub p: u64, pub mul: Cell<u64>, pub inv: Cell<u64> }
```
- `add(a,b)`, `sub(a,b)`, `neg(a)`: uncounted, results in `[0,p)`.
- `mul(a,b) = a*b % p`: `mul += 1`. Squaring is `mul(a,a)` (counted as one).
- `inv(a)`: panics if `a == 0`; computes `a^(p-2) mod p` with **uncounted** internal
  arithmetic; `inv += 1`. (An inversion is priced at 10 FPM by the caller.)
- `fpm() = mul + 10*inv`.
- Phase accounting: one `Fp` instance per run; before and after each phase take
  `(mul, inv)` snapshots; phase cost = difference. Phases: `setup`, `pencil`,
  `pcfree`, `line`.
- Gate checks (section 7.4) use a separate uncounted function `raw_eval_F(a, x, y, p)`
  with plain `%` arithmetic, never `Fp`.

## 3. Randomness

SplitMix64, exactly as `crypto/src/cryptanalysis/diem_quartic.rs`:
```
next_u64: state += 0x9E3779B97F4A7C15 (wrapping); z = state;
          z = (z ^ (z>>30)) * 0xBF58476D1CE4E5B9; z = (z ^ (z>>27)) * 0x94D049BB133111EB;
          return z ^ (z>>31)          (all wrapping)
below(n) = ((next_u64() as u128 * n as u128) >> 64) as u64
mix64(z) = the three finalizer lines above applied to z (no state increment)
```
Seed derivation, all wrapping u64:
```
derive(p, s, b, tag) = mix64( p*0x9E3779B97F4A7C15 ^ (s+1)*0xC2B2AE3D27D4EB4F
                              ^ (b+1)*0x165667B19E3779F9 ^ tag*0xD6E8FEB86659FD93 )
```
- curve stream: `derive(p, s, 255, 1)`; point shuffle: `derive(p, s, 255, 2)`
  (so curve, O and the nested FB do not depend on beta);
- pair stream 1: `derive(p, s, b, 11)`; stream 2: `derive(p, s, b, 12)`; stream 3: `derive(p, s, b, 13)`;
- Cantor-Zassenhaus: per polynomial `f` (section 4.5), `SplitMix64::new(hash(f))`,
  `hash(f)`: `h = 0x243F6A8885A308D3; for c in f (low to high): h = mix64(h ^ c)`.

## 4. Polynomials

Representation: `Vec<u64>`, coefficient of `x^k` at index `k`, **always trimmed**
(no trailing zeros); the zero polynomial is the empty vector. All coefficient
multiplications go through `Fp::mul`; never skip a multiplication because a
coefficient is zero (counts must depend only on lengths).

4.1 `pmul(a, b)`: schoolbook, `len(a)*len(b)` multiplications, then trim.

4.2 `prem_monic(a, f)` (f monic, degree d): copy `a`; for `k` from `len(a)-1` down
to `d`: `t = a[k]`; for `i in 0..d`: `a[k-d+i] -= mul(t, f[i])`; `a[k] = 0`. Trim.
Cost `d` multiplications per step.

4.3 `mulmod(a, b, f) = prem_monic(pmul(a, b), f)`.
`powmod(base, e, f)`: `base` already reduced; left-to-right over the bits of `e`
from the bit below the top bit: `h = base`; for each bit: `h = mulmod(h, h, f)`;
if bit is 1: `h = mulmod(h, base, f)`. `e >= 1`.

4.4 `make_monic(c)`: `l = inv(c[last])`; multiply each of the other `len-1`
coefficients by `l` (counted); set last to 1.

`split_test(f)` (f monic, degree 1..3): `powmod([0,1], p, f) == [0,1]` (trimmed
comparison; for degree 1 reduce `[0,1]` first: `prem_monic([0,1], f)`, and use that
as base). True iff `f` is a product of distinct linear factors over F_p.

`gcd_monic(a, b)`: Euclid; while `b` non-empty: make `b` monic (4.4), `r =
prem_monic(a, b)`, `a = b`, `b = r`. Return `a` made monic. (Inversions counted.)

`divexact_monic(a, g)`: long division by monic `g` producing quotient `q`; each step
`deg g` multiplications; panic if the remainder is non-zero.

4.5 `roots_split(f)` (f monic, squarefree, splits completely, degree 1..3):
- degree 1: return `[neg(f[0])]`.
- degree 2 or 3: `rng = SplitMix64::new(hash(f))`; loop { `d = rng.below(p)`;
  `h = powmod(prem_monic([d,1], f), (p-1)/2, f)`; `h[0] = sub(h[0], 1)` (then trim);
  if `h` empty continue; `g = gcd_monic(f.clone(), h)`; if `1 <= deg g < deg f`:
  return sorted(`roots_split(g)` ++ `roots_split(divexact_monic(f, g))`) }.
Output sorted ascending.

`roots_all(c)` (setup only, c monic cubic): `w = powmod([0,1], p, c)`; `w = w - x`
(subtract 1 from coefficient 1, trim); if `w` empty return `roots_split(c)`;
`g = gcd_monic(c, w)`; if `deg g == 0` return `[]`; return `roots_split(g)`.

`synth_div(a, r)`: divide by `(x - r)`: `n = len(a)-1`; `acc = 0`; for k = n down to
0: `acc = add(mul(acc, r), a[k])`; if `k > 0` then `q[k-1] = acc`. Remainder `acc`.
Cost `n+1` multiplications (count all, including the first `mul(0, r)`).

## 5. Curve

5.1 Coefficients `a[i][j]` (coefficient of `x^i y^j`, `0 <= i, j <= 3`), drawn in the
order `i = 0..3` outer, `j = 0..3` inner, each `= curve_rng.below(p)`. Derived:
`Ax(y) = [a[3][0], a[3][1], a[3][2], a[3][3]]` (coefficient of x^3, as a polynomial in y);
`Ay(x) = [a[0][3], a[1][3], a[2][3], a[3][3]]`.

5.2 Acceptance (redraw all 16 from the same stream on failure; count `redraws`):
(i) `a[3][3] != 0`; (ii) `Ax` and `Ay` have no root in F_p (evaluate at every
`t in F_p` with uncounted arithmetic); (iii)-(v) after enumeration (5.3): no point
with `F_x = F_y = 0` (uncounted evaluation of the partial derivatives
`F_x = sum i a_ij x^(i-1) y^j`, `F_y = sum j a_ij x^i y^(j-1)`);
`|N1 - (p+1)| <= 8*sqrt(p)` (compare `(N1-(p+1))^2 <= 64 p` in integers);
`N1 >= 16`.

5.3 Points (phase `setup`): for `x = 0..p-1`: `c_j = sum_i a[i][j] x^i` by Horner
(3 mults each, 4 values), `c = make_monic([c0,c1,c2,c3])`, `ys = roots_all(c)`.
Points `(x, y)` for `y in ys`. Sort all points lexicographically by `(x, y)`;
point id = index. `N1` = count. `HashMap<(u64,u64), u32>` from point to id.

5.4 Shuffle: `perm = [0..N1)`; Fisher-Yates with the shuffle stream: for `i` from
`N1-1` down to 1: `j = below(i+1)`; swap `perm[i], perm[j]`. `O = perm[0]`.
`beta` grid as `(num, den)`: idx 0..5 = (1,8), (3,16), (1,4), (3,8), (1,2), (3,4).
`n_FB = max(4, (2*(N1-1)*num + den) / (2*den))` (integer division).
`FB = perm[1 ..= n_FB]`; FB position `k` (0-based in that slice) is column `k`.

## 6. Rows, columns, arms

```
struct Row { kind: u8 /*0 line, 1 x-fibre, 2 y-fibre*/, pts: Vec<u32> /*sorted ids*/,
             small: Vec<(u32, i64)>, lps: Vec<u32> /*point ids, sorted*/ }
```
Columns: FB pos `k` -> `k`; `O` -> `n_FB`. A-type arms (A, A2, PC1, PC4): `K -> n_FB+1`,
`n_cols = n_FB + 2`. B-type arms (B, B3): `Dx -> n_FB+1`, `Dy -> n_FB+2`,
`n_cols = n_FB + 3`. Line row: `+1` on each small point's column, `-1` on `K`
(A-type) or `-1` on `Dx` and `-1` on `Dy` (B-type); LP points go to `lps`. x-fibre
row: `+1` per small point, `-1` on `Dx`; y-fibre: `-1` on `Dy`.
`target = n_cols - 1`.

Dedup: per run `HashSet<(u8, Vec<u32>)>` keyed by `(kind, pts)`; a row whose key is
present is rejected as `duplicate`.

## 7. Relation generators

7.1 **Line trial** `line_trial(O, P1, P2)` (phase `line`, or `pcfree` for PC stream 3):
1. `w_k = mul(x_k, y_k)` for `k in {O, P1, P2}` (3 mults). Matrix rows `(1, x_k, y_k, w_k)`.
2. For `m = 0..3`: `b_m = (-1)^m * det3(matrix with column m deleted)`;
   `det3` = `a(ei - fh) - b(di - fg) + c(dh - eg)` (9 mults). Names:
   `(b00, b10, b01, b11) = (b_0, b_1, b_2, b_3)`.
3. If `sub(mul(b00, b11), mul(b10, b01)) == 0`: outcome `degenerate`. Stop.
4. `u = trim([neg(b00), neg(b10)])`, `v = trim([b01, b11])`. `U_0 = [1]`,
   `U_j = pmul(U_{j-1}, u)`; `V_k` likewise with `v`, for `j, k = 1..3`.
   `G = sum_{j=0..3} pmul(A_j, pmul(U_j, V_{3-j}))` with `A_j = [a[0][j], a[1][j], a[2][j], a[3][j]]`
   (trimmed). Panic unless `deg G == 6`.
5. `R = synth_div(synth_div(synth_div(G, x_O), x_1), x_2)`; panic unless each
   remainder is 0. `R = make_monic(R)` (degree 3).
6. If `!split_test(R)`: outcome `nosplit`. Stop.
7. `xs = roots_split(R)`. If any root equals `x_O`, `x_1` or `x_2`: outcome `tangent`. Stop.
8. For `r = 0..2`: `num_r = add(b00, mul(b10, xs[r]))`, `den_r = add(b01, mul(b11, xs[r]))`;
   panic if `den_r == 0`. Batch inverse: `c1 = den_0; c2 = mul(c1, den_1); c3 = mul(c2, den_2);
   i3 = inv(c3); inv_2 = mul(i3, c2); t = mul(i3, den_2); inv_1 = mul(t, c1); inv_0 = mul(t, den_1)`.
   `y_r = neg(mul(num_r, inv_r))`.
9. Look up `(xs[r], y_r)` in the point map; panic if absent.
10. Classify the 6 points (`O, P1, P2`, residuals): small if `O` or in FB; else LP.
    `#LP > 2`: outcome `toomanylp`. Else build the row; dedup; outcome `duplicate`
    or `accepted` (by LP count 0/1/2).

7.2 **Pencil phase** (arms B, B3; phase `pencil`): for `t = 0..p-1` (x-fibres):
`c_j = sum_i a[i][j] t^i` (Horner, 3 mults each, `j = 0..3`); `c = make_monic(c)`;
if `split_test(c)`: `ys = roots_split(c)`, points `(t, y)`. Then for `t = 0..p-1`
(y-fibres): `c_i = sum_j a[i][j] t^j`, same, points `(x, t)`. Each split fibre:
classify; count `pencil_lp{0,1,2,3}`; arm B keeps rows with `<= 2` LP, arm B3 keeps
all; dedup as usual. Rows are inserted in this order before any line trial.

7.3 **Pair stream** for a run: `rng = SplitMix64::new(stream seed)`; `used: HashSet<u64>`;
`next_pair()`: if `used.len() == C(n_FB,2)` return `None` (exhausted). If
`used.len() >= 0.9 * C(n_FB,2)` (first time): build the list of unused pairs
`(i<j)` in lexicographic order, Fisher-Yates shuffle it with `rng`, and from then on
serve pairs from that list in order (skipping any in `skip`). Otherwise: loop { `i =
below(n_FB)`, `j = below(n_FB)`; if `i == j` continue; `(i, j) = (min, max)`;
`key = i*n_FB + j`; if `key in used` continue; insert; if `key in skip` continue;
return `(FB[i], FB[j])` }. Pairs in `skip` (PC arms: the stream-3 pairs) are marked
used but not returned and not counted as trials.

7.4 **Row gate G2**: when `p <= 1021` every accepted row, otherwise every 97th
accepted row (counter over the run), check with `raw_eval_F` that every point
satisfies `F = 0`; for line rows that every point satisfies `b00 + b10 x + b01 y +
b11 x y = 0` (uncounted); for x-fibres that all points share x = t (y-fibres: y = t).
Panic on failure. Count `gate_rows_checked`.

## 8. Parity union-find yield (`graph.rs`)

Vertex 0 = sink. LP point ids map to vertices `1, 2, ...` in order of first
appearance (per run). Arrays `parent, parity, sink, odd`; counters `edges, verts,
bip, ops, F0`.
- `new_vertex()`: push `parent = self`, `parity = 0`, `sink = false`, `odd = false`;
  `verts += 1`; `bip += 1`.
- `find(v)`: walk to the root accumulating parity (each step `ops += 1`), then path
  compression keeping parities consistent (as in diem_quartic `LpGraph::find`).
- `is_b(r) = !sink[r] && !odd[r]` (0/1).
- `add_edge(a, b)`: `edges += 1`; `(ra, pa) = find(a)`, `(rb, pb) = find(b)`; if
  `ra == rb`: if `pa == pb && !odd[ra]`: `bip -= is_b(ra)`; `odd[ra] = true`.
  Else: `bip -= is_b(ra) + is_b(rb)`; `parent[rb] = ra`; `parity[rb] = pa ^ pb ^ 1`;
  `sink[ra] |= sink[rb]`; `odd[ra] |= odd[rb]`; `bip += is_b(ra)`.
- `sink[0] = true` at construction (vertex 0 is not counted in `verts`, not in `bip`).
- Insert a row: 0 LP: `F0 += 1`; 1 LP `v`: `add_edge(0, v)`; 2 LP `v, w`: `add_edge(v, w)`.
  3 LP rows (B3 only) are not inserted into the union-find.
- `Y = F0 + edges - verts + bip`; `rank_lp_uf = verts - bip`.

## 9. Exact rank engine (`exact.rs`)

`ell = 2^61 - 1`; coefficient `-1` stored as `ell - 1`. Two incremental echelon
structures: `full` over columns `0 .. n_cols + N1` (LP vertex `v` -> column
`n_cols + v - 1`), and `lp` over LP columns only. Representation: dense `Vec<u64>`
rows; `pivots: HashMap<usize, Vec<u64>>` (pivot entry normalised to 1).
`insert(row) -> bool`: loop { find the first non-zero column `c` of `row`; if none
return false; if `c` has a pivot row `q`: `row -= row[c] * q` (mod ell) and continue;
else normalise `row` by `inv(row[c])` mod ell, store as pivot of `c`, return true }.
`la_ops += 1` per mod-ell multiplication (informational). `dim = rank_full - rank_lp`.
The exact engine inserts every accepted row (3-LP fibre rows included in B3).

## 10. CLI

```
l09 census --p P --seeds N [--seed-from K]
l09 cell   --p P --beta-idx b --seeds N [--seed-from K] --arms A,A2,B,PC1,PC4
l09 exact  --p P --beta-idx b --seeds N [--seed-from K] --arms A,B,B3
```
Seeds `K .. K+N-1` (default K = 0). For each seed, arms in the order given; for
PC arms, the B pencil counts `(n0, n1, n2)` of the same seed are needed: compute
them by running the pencil phase (with a throw-away `Fp`) if B has not run yet.
Output: one JSON object per line on stdout, written by hand with `format!` in
exactly the key order of section 11 (no serde), integers in decimal, booleans
`true/false`, strings quoted. Progress messages go to stderr only.

## 11. Run procedure and output

Per `(p, b, s)`: build curve (phase `setup`, record `fpm_setup`), shuffle, FB.
Per arm, fresh `Fp` counters for non-setup phases, fresh union-find, dedup set, pair stream.

- A: stream 1. A2: stream 2. B: pencil phase, then stream 1. 
- PC_m (`m = 1` or 4): targets `m*n0, m*n1, m*n2`; phase `pcfree`: draw pairs from
  stream 3 (its own used-set, no skip) and run `line_trial`; an accepted row with
  `k` LP goes in if `got_k < m*n_k`, else it is discarded (and its dedup key is not
  inserted); stop when all three are full or after `64*p` stream-3 trials. Insert
  kept rows (in order drawn). Then the main stream: stream 1 with `skip` = all pairs
  drawn from stream 3.
- After each inserted row (pencil, pcfree, or line), if `Y >= target`, stop: `T` =
  number of main-stream trials so far (0 if met before the first trial).
- Cap: `cap = min(64 * ceil(p*den/num), C(n_FB, 2))`. If trials reach `cap` or the
  stream is exhausted without meeting the target: `censored = true`,
  `censor_reason = "trial_cap"` or `"pairs_exhausted"`, `T = cap` (or number of
  trials done when exhausted).

Run line (`cell`), key order:
```
{"kind":"run","p":,"beta_idx":,"beta_num":,"beta_den":,"seed":,"arm":"","N1":,"redraws":,
 "n_fb":,"n_cols":,"target":,"T":,"censored":,"censor_reason":"","cap":,
 "t_degenerate":,"t_nosplit":,"t_tangent":,"t_toomanylp":,"t_duplicate":,"acc_lp0":,"acc_lp1":,"acc_lp2":,
 "pencil_lp0":,"pencil_lp1":,"pencil_lp2":,"pencil_lp3":,"pencil_rows":,
 "pc_m":,"pc_target0":,"pc_target1":,"pc_target2":,"pc_got0":,"pc_got1":,"pc_got2":,"pc_free_trials":,
 "F0":,"E":,"V":,"B":,"Y":,"uf_ops":,
 "fpm_setup":,"mul_line":,"inv_line":,"mul_pencil":,"inv_pencil":,"mul_pcfree":,"inv_pcfree":,
 "fpm_line":,"fpm_pencil":,"fpm_rel":,"gate_rows_checked":}
```
(on one line; `fpm_rel = fpm_line + fpm_pencil`; fields that do not apply are 0;
`pc_m` is 0 for non-PC arms; `t_*` and `acc_*` count main-stream trials only.)

Exact line (`exact`): same procedure but the stop condition is `dim >= target`
(exact), the union-find is maintained alongside (except it ignores 3-LP rows), and
the record adds, after `"Y"`: `"T_uf":` (first trial count with `Y >= target`, -1 if
never, -2 for B3), `"T_exact":`, `"rank_full":`, `"rank_lp":`, `"rank_lp_uf":`,
`"dim":`, `"la_ops":`, and `"kind":"exact"`. Gate G4: at the stop of A and B exact
runs, `rank_lp == rank_lp_uf`; record `"g4_ok":true/false` (do not panic).

Census line: `{"kind":"census","p":,"seed":,"N1":,"redraws":,"split_x":,"split_y":,
"pts_x":,"pts_y":,"line_trials":,"line_nondegen":,"line_split":}` where `split_x/y` =
number of `t in F_p` whose fibre cubic splits completely, `pts_x/y` = sum of all
rational points over all x-fibres (y-fibres; must equal `N1`, panic otherwise),
line trials = the first `min(20000, C(n_FB,2))` trials of stream 1 at `beta_idx 5`,
`line_nondegen` = trials not `degenerate`, `line_split` = trials whose step 6 passed.

## 12. Unit tests (`cargo test --release`; G0) with expected values

Fixture curve over `p = 31`:
`a = [[3,10,17,24],[12,20,28,5],[21,30,8,17],[30,9,19,29]]` (`a[i][j]`, x^i y^j).

1. Acceptance (i)-(v) passes; `N1 = 36`; sorted points are
   `(0,6) (1,9) (1,10) (1,16) (2,18) (3,19) (4,17) (7,18) (8,26) (9,4) (9,13) (9,15)
   (10,4) (10,13) (10,15) (12,26) (13,17) (15,17) (16,4) (16,13) (16,15) (17,1)
   (18,25) (19,8) (19,12) (19,29) (20,19) (22,0) (23,19) (24,30) (26,3) (26,20)
   (26,23) (28,18) (29,26) (30,11)`.
2. Pencil census: fully split x-fibres = 6, y-fibres = 7; `pts_x = pts_y = 36`.
   x-fibre `t = 9`: monic cubic in y = `[26,28,30,1]`, roots `[4,13,15]`.
   y-fibre `t = 4`: points `(9,4) (10,4) (16,4)`.
3. Line trials with `O = (0,6)`, `P1 = (1,9)`:
   - `P2 = (1,10)`: `b = [25,6,1,30]`, outcome `degenerate`.
   - `P2 = (2,18)`: `b = [16,0,18,25]`, `G = [0,18,0,24,29,23,30]`,
     monic `R = [22,2,11,1]`, outcome `nosplit`.
   - `P2 = (3,19)`: `b = [6,8,30,27]`, `G = [0,20,2,18,28,24,1]`,
     `R = [17,13,28,1]`, splits; residual points `(9,13) (26,23) (30,11)`.
   - `P2 = (4,17)`: `b = [25,19,1,1]`, `G = [0,22,3,22,9,30,7]`,
     `R = [3,26,27,1]`, outcome `nosplit`.
4. Polynomials over `p = 7`: `split_test((x-1)(x-2)(x-3)) = true`, `roots_split` =
   `[1,2,3]`; `split_test(x^3 - 2) = false`; `split_test(x^3 - x) = true`, roots
   `[0,1,6]`; `split_test((x-1)^2 (x-2)) = false`. `pmul([1,2],[3,4])` uses exactly
   4 multiplications and equals `[3,10,8] mod 7 = [3,3,1]`.
5. Counters: `Fp::mul` increments `mul` by 1 per call; `inv` increments `inv` by 1
   and `mul` by 0. Running the same line trial twice gives identical `(mul, inv)` deltas.
6. Union-find, with `Y = F0 + E - V + B` (`F0 = 0`):
   sink edge (0,1) then edge (1,2): `Y = 0`; triangle 1-2, 2-3, 3-1: `Y = 0`;
   square 1-2, 2-3, 3-4, 4-1: `Y = 1`; edge 1-2 twice: `Y = 1`; sink edge (0,1)
   twice: `Y = 1`; two disjoint edges 1-2, 3-4: `Y = 0`, `rank_lp_uf = 2`.
7. Exact engine over 3 small columns: rows `[1,-1,0]`, `[0,1,-1]`, `[1,0,-1]` -> rank 2;
   and for each graph of test 6 (rows with only LP entries `+1`), `rank_lp` equals
   `rank_lp_uf`.
8. Dedup: inserting the same `(kind, pts)` twice yields one `accepted`, one `duplicate`.
9. Determinism: `cell --p 251 --beta-idx 4 --seeds 1 --arms A,B` run twice in-process
   produces identical output strings.
10. Integration (p = 251, beta_idx 4, seed 0): every arm's stopped run has
   `Y >= target`; B's `T <= A's T + 64`; the exact run's `g4_ok` is true.

## 13. `analyze.py` (Python 3 standard library only)

Input: directory `raw/`. Reads `census_*.jsonl`, `exact_*.jsonl`, `cell_*.jsonl`,
`g6_rerun.jsonl`. Writes one JSON object to stdout.

- `median(xs)`: sorted; odd n middle; even n mean of the two middle values.
  `quantile(xs, q)`: linear interpolation, `h = (n-1)q`, `x[floor h] + (h - floor h)(x[floor h + 1] - x[floor h])`.
- Bootstrap: Python SplitMix64 identical to section 3 (mask `& 0xFFFFFFFFFFFFFFFF`),
  seed `0x4C09B007` per cell (fresh), 10,000 resamples, each draws `n` indices with
  `below(n)`; statistic = median of resampled `r_B`; CI = `(sorted[249], sorted[9749])`.
- Per `(p, beta_idx)`: join runs by seed; `r_B = 1 - T_B/T_A`, `r_AA = 1 - T_A2/T_A`,
  `r_PC1`, `r_PC4`, `f_B = 1 - fpm_rel_B/fpm_rel_A`, `f_AA`; report `n`, median, q25,
  q75 of each; `N = median |r_AA|`, `N_f = median |f_AA|`; censored counts per arm;
  `median(r_B - r_PC1)`; `cost_A2 = median(fpm_rel_A2 + 24*n_cols_A2^2)`.
- `beta*(p)`: PROTOCOL section 5 (ties: smaller beta_idx).
- Gates: G3 (pooled `sum split_x / (20 p)`, `sum split_y / (20 p)`,
  `sum line_split / sum line_nondegen` against PROTOCOL section 7 tolerances);
  G4 (all `g4_ok` true in A and B exact records); G5 (PROTOCOL section 7: per run
  `T_exact/T_uf - 1` for non-censored A and B exact records with `T_uf > 0`, medians,
  and the paired B-A median); G6 (the lines of `g6_rerun.jsonl` equal, as strings,
  the lines of `cell_p1021_b2.jsonl` with seed 0 and 1, in order); G7 (fraction of
  `(p, b, s)` with `T_B <= T_A + 64`).
- Verdicts: PROTOCOL section 6, per p at `beta*`, and secondary per (p, beta); the
  FPM verdict; the headline = verdict at `p = 16381`. B3 (exact records) reported
  descriptively: median `T_exact` for A, B, B3 per cell.
- Output keys: `"gates"`, `"cells"`, `"beta_star"`, `"verdicts"`, `"headline"`,
  `"b3_exploratory"`, `"census"`. Every number carries the tag
  `"status": "[pcc:unverified]"` at the top level.
