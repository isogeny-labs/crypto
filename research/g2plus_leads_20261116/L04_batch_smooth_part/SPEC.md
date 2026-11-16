# L04 implementation specification (frozen with PROTOCOL.md)

The implementer makes no research choices. Anything not specified here must
not be added: no extra arms, cells, seeds or metrics. Language: Python 3,
standard library only. Single thread. All files go under
`/home/user/crypto/research/g2plus_leads_20261116/L04_batch_smooth_part/`:

```
code/l04.py        library (sections 1-6)
code/run_cell.py   CLI that runs one cell / arm group, writes JSON lines (section 7)
code/analyze.py    gates + verdict (section 8)
code/test_l04.py   unittest (section 9)
raw/               outputs only
```

## 1. Conventions

- Polynomials are Python lists of ints in `[0, p)`, little-endian
  (`a[i]` is the coefficient of x^i), always trimmed (no trailing zeros). The
  zero polynomial is `[]`. Monic means `a[-1] == 1`.
- `trim(a)` pops trailing zeros in place.
- Counter class:
  ```python
  class Ops:
      __slots__ = ("mul", "inv", "add")
      def __init__(self): self.mul = 0; self.inv = 0; self.add = 0
  ```
  Every counted routine takes an `ops: Ops` argument and increments it exactly
  where stated. Squarings count as multiplications. `add` is recorded but not
  part of FPM. Uncounted helpers say "uncounted".
- `inverse(a, p) = pow(a, p - 2, p)` computes the value. Each call site that
  needs an inversion does `ops.inv += 1` (one inversion, unit 10 FPM). Do
  **not** add Fermat multiplications to `mul`.
- `fpm(ops) = ops.mul + 10 * ops.inv`.
  `fpm_lab(ops, p) = ops.mul + ops.inv * ((p - 2).bit_length() + bin(p - 2).count("1") - 2)`.

## 2. Setup (phase `setup`, excluded from per-candidate metrics)

- `chi_table(p, ops)`: list of length p with `chi[0] = 0` and -1 elsewhere. For
  `y` in `1 .. (p-1)//2`, set `chi[y*y % p] = 1`. Then `ops.mul += (p - 1)//2`.
- `make_f(p, g, seed_f)`: `rng = random.Random(seed_f)`;
  `f = [rng.randrange(p) for _ in range(2*g + 1)] + [1]`.
- `make_S(p, g, f, chi, N, ops)`: scan `x = 0, 1, 2, ...` while
  `len(S) < N` and `x < p`. Compute `fx` by Horner
  (`fx = 0; for cf in reversed(f): fx = (fx*x + cf) % p`), then
  `ops.mul += len(f); ops.add += len(f)`. Append x to S if `chi[fx] == 1`.
  Return S (a list in ascending order). Also build `inS = bytearray(p)` with
  `inS[s] = 1` (uncounted).
- `N = math.ceil(1.0 * p ** (1.0 - 1.0/g))`, then `N = min(N, p)`, unless
  `--nfb` is given, in which case N is that value.
- `build_FS(S, p, ops)`: `F = [1]`. For each s in S:
  `new = [0]*(len(F)+1)`. For each i: `new[i+1] += F[i]`,
  `new[i] -= s*F[i]` (mod p). Then `ops.mul += len(F)` (taken before the
  update). Result `F` has degree N.
  Uncounted check G8: `len(F) == N + 1` and `peval(F, s) == 0` for every s in S
  (`peval` is uncounted Horner).

## 3. Counted polynomial routines (ports of `src/cryptanalysis/genus_ladder.rs`)

### 3.1 Residues modulo monic u of degree d (`ModU`)

`u` is monic of degree d ≥ 1. Let `uu = u[:d]`. Residues are lists of length
exactly d (zeros allowed; not trimmed).

- `modu_mul(a, b, uu, d, p, ops)`: `t = [0]*(2d)`. For `i, j` in
  `range(d) x range(d)`: `t[i+j] += a[i]*b[j]`. Then
  `ops.mul += d*d; ops.add += d*d`. For `k` from `2d-2` down to `d`
  inclusive: `c = t[k] % p`; for `j` in `range(d)`: `t[k-d+j] -= uu[j]*c`;
  then `ops.mul += d; ops.add += d`. Return `[t[i] % p for i in range(d)]`.
  The count is always `d*d + (d-1)*d`, which is 15 at d = 3 and 6 at d = 2.
- `modu_mul_xpa(a, aa, uu, d, p, ops)` computes `a * (x + aa)`:
  `top = a[d-1]`, `out = [0] + a[:d-1]`, and for j in `range(d)`:
  `out[j] = (out[j] - uu[j]*top + aa*a[j]) % p`. Then
  `w = d if aa == 0 else 2*d; ops.mul += w; ops.add += w`.
- `modu_base(aa, uu, d, p)`: if `d == 1`, return `[(aa - uu[0]) % p]`. Else
  return `[aa, 1] + [0]*(d-2)`.
- `pow_xpa(aa, e, uu, d, p, ops)` computes `(x + aa)^e mod u` for `e >= 1`:
  `res = modu_base(aa, ...)`, `bits = e.bit_length()`. For
  `i in range(bits - 2, -1, -1)`: `res = modu_mul(res, res, ...)`; if
  `(e >> i) & 1`: `res = modu_mul_xpa(res, aa, ...)`.
- `splits_completely(u, p, ops)`: `d = len(u) - 1`, `xp = pow_xpa(0, p, u[:d], d, p, ops)`,
  return `xp == modu_base(0, u[:d], d, p)`.

### 3.2 Euclidean routines

- `vrem(a, m, p, ops)`: `r = list(a); trim(r); dm = len(m) - 1; lead = m[-1]`.
  If `lead != 1`: `li = inverse(lead, p); ops.inv += 1`, else `li = 1`.
  `while len(r) > dm:` `top = len(r) - 1; c = r[top]`. If `li != 1`:
  `c = c*li % p; ops.mul += 1`. For j in `range(dm)`:
  `r[top-dm+j] = (r[top-dm+j] - c*m[j]) % p`. Then
  `ops.mul += dm; ops.add += dm; r.pop(); trim(r)`. Return r.
- `vgcd_monic(a, b, p, ops)`: copy and trim both. While b is non-empty:
  `r = vrem(a, b, p, ops); a, b = b, r`. If a is non-empty and `a[-1] != 1`:
  `il = inverse(a[-1], p); ops.inv += 1`, multiply every coefficient by il, and
  `ops.mul += len(a)`. Return a.
- `vdiv_monic(a, m, p, ops)`: exact quotient by monic m. `dm = len(m) - 1`,
  `r = list(a)`, `q = [0]*(len(a) - dm)`. For `top` from `len(a)-1` down to
  `dm`: `c = r[top]; q[top-dm] = c`. For j in `range(dm)`:
  `r[top-dm+j] = (r[top-dm+j] - c*m[j]) % p`. Then
  `ops.mul += dm; ops.add += dm`. Return q.
- `poly_mul(a, b, p, ops)`: schoolbook. `out[i+j] += a[i]*b[j]` for all i, j,
  then reduce mod p. `ops.mul += len(a)*len(b)`. Return the trimmed result.

### 3.3 `disc_character(u, chi, p, ops)` (exact port)

```python
d = len(u) - 1                         # d >= 2, u monic
a = list(u)
b = [u[i] * (i % p) % p for i in range(1, d + 1)]; ops.mul += d; trim(b)
sign = chi[p - 1] if (d*(d-1)//2) % 2 == 1 else 1
while True:
    if not b: return 0
    m = len(a) - 1; nn = len(b) - 1
    if nn == 0: return sign * chi[b[0]] if m % 2 == 1 else sign
    lb = b[nn]; delta = m - nn + 1; r = list(a)
    for step in range(delta):
        top = m - step; c = r[top]
        for i in range(top): r[i] = r[i] * lb % p
        for j in range(nn): r[top-nn+j] = (r[top-nn+j] - c*b[j]) % p
        ops.mul += top + nn; ops.add += nn
        del r[top:]
    trim(r)
    if not r: return 0
    dr = len(r) - 1
    if (m + dr + delta*nn) % 2 == 1: sign *= chi[lb]
    if (m*nn) % 2 == 1: sign *= chi[p - 1]
    a, b = b, r
```

### 3.4 `roots_of(u, p, rng, ops)` (Cantor-Zassenhaus, exact port)

`u` is monic and assumed squarefree and split. `d = len(u) - 1`. If `d == 0`,
return `[]`. If `d == 1`, return `[(-u[0]) % p]` (0 mults). Otherwise repeat
up to 64 times:
`aa = rng.randrange(p)`; `h = pow_xpa(aa, (p-1)//2, u[:d], d, p, ops)`;
`hm1 = list(h); hm1[0] = (hm1[0] - 1) % p; trim(hm1)`. If empty, continue.
`gg = vgcd_monic(u, hm1, p, ops)`, `dg = len(gg) - 1`. If `dg in (0, d)`,
continue. `q = vdiv_monic(u, gg, p, ops)`. Recurse on gg then on q (each
returns None on failure; propagate None). Return the concatenation. After 64
failed attempts, return None. The caller counts None as `cz_fail`.

## 4. Decision function and arms

`D(u) = 1` iff u is squarefree and split, and at most 2 of its roots are not in
S (PROTOCOL section 1). Each arm returns a decision per input and, for accepted
inputs, a root list.

Phase counters per arm run (fresh `Ops` objects): `pre` (disc_character),
`dec` (x^p test, or reduction / tree / post-test), `roots`. For C arms, `dec`
is three separate counters, `tprod`, `trem` and `post`, whose sum is reported
as `dec`.

### 4.1 Arm A (`A_pf`, `A_nopf`, `A_pf_aa`)

For each u, in input order:
1. If prefilter: `ch = disc_character(u, chi, p, pre)`. If `ch != 1`, reject.
   If `g == 2`, set `decided = True`.
2. If not decided: if `not splits_completely(u, p, dec)`, reject.
3. `rts = roots_of(u, p, rng, roots)`. If None: `cz_fail += 1`, reject (gate
   G5 then fails).
4. `out = sum(1 for r in rts if not inS[r])`. Accept iff `out <= 2`. Record
   `rts` if accepted.

`rng = random.Random(seed_root)` is created once at the start of the arm run.
For `A_pf_aa` it is `random.Random(seed_root_aa)`.

### 4.2 Post-test `post_S(u, R, ops_post)` (shared by B and C)

Input: R = `F_S mod u` (trimmed). Output: `(accept, G, c)`.
1. If `R == []`: return `(True, u, [1])`.
2. `G = vgcd_monic(u, R, p, ops_post)`, `k = len(G) - 1`.
3. If `k < g - 2`: return `(False, G, None)`.
4. `c = vdiv_monic(u, G, p, ops_post)` if `k >= 1`, else `c = list(u)`. Let
   `dc = len(c) - 1` (0, 1 or 2).
5. If `dc == 0`: accept.
   If `dc == 1`: `r = (-c[0]) % p`. Reject if `inS[r]` (a repeated root),
   otherwise accept.
   If `dc == 2`: `disc = (c[1]*c[1] - 4*c[0]) % p; ops_post.mul += 2`. If
   `chi[disc] != 1`, reject. If `k >= 1`:
   `h = vgcd_monic(c, G, p, ops_post)`, and reject if `len(h) - 1 >= 1`.
   Then accept.
Roots (only if accepted, counter `roots`, same rng rule as A with `seed_root`):
`rts = roots_of(G) + roots_of(c)` (`roots_of([1]) = []`).

### 4.3 Arm B (`B`, `B_pf`)

For each u in order: if B_pf, run `disc_character(u, chi, p, pre)` and reject
if `!= 1`. Then `R = vrem(FS, u, p, dec)`, then
`post_S(u, R, dec)`, then roots if accepted.

### 4.4 Arm C (`C_m{m}`, `Cpf_m{m}`)

1. List L: all inputs in order (C), or for Cpf, the inputs with
   `disc_character(u, chi, p, pre) == 1`, in order. The others are rejected.
2. Split L into consecutive batches of m (the last may be shorter; record the
   sizes). For each batch `[u_1 .. u_k]`:
   - Product tree. `level[0] = [u_1, ..., u_k]`. While the last level has more
     than one node: `nxt = [poly_mul(L[2j], L[2j+1], p, tprod) for j in range(len(L)//2)]`,
     and if `len(L)` is odd, append `L[-1]` unchanged (0 cost). Keep all levels.
   - Remainder tree. `rem[top][0] = vrem(FS, root, p, trem)`. When
     `deg FS < deg root`, this returns FS unchanged at 0 cost (the loop does not
     run). Walk down: for a node that was a product of children `2j, 2j+1`,
     `rem_child = vrem(rem_parent, child, p, trem)`. For a node carried up
     unchanged, the child's remainder is the parent's remainder (0 cost, no
     call).
   - Leaves: `R_i = rem[0][i]`, then `post_S(u_i, R_i, post)`, then roots if
     accepted.
3. m = 1 must reproduce B's decisions. Its `dec` count equals B's `dec`, plus
   0 for the trees.

### 4.5 Per-candidate metrics

`T_cell` is the number of inputs in the cell (random plus planted in corr
cells). For each phase: `fpm_phase_per_cand = fpm(phase) / T_cell`.
`fpm_total_per_cand = (fpm(pre) + fpm(dec) + fpm(roots)) / T_cell`.
`fpm_lab_total_per_cand` is the same with `fpm_lab`.

## 5. Inputs

- `seed_f = seed`, `seed_u = seed + 50`, `seed_root = seed + 70`,
  `seed_root_aa = seed + 71`, `seed_planted = seed + 90`.
- Random inputs: `rng = random.Random(seed_u)`. For each of T inputs:
  `u = [rng.randrange(p) for _ in range(g)] + [1]`.
- Planted inputs (only with `--planted`): see section 6. They are appended after
  the random inputs.

## 6. Planted inputs (correctness cells only)

`rng = random.Random(seed_planted)`. 256 inputs; `t = i % 8` at g = 3, `t = i % 6`
at g = 2. "in" means `rng.choice(S)` and "out" means `rng.randrange(p)`,
resampled until `not inS[x]`. Within one input, distinct roots are resampled
until distinct. `lin(r) = [(-r) % p, 1]`. `irrq()` = `[c0, c1, 1]` with
`c0, c1 = rng.randrange(p)`, resampled until `chi[(c1*c1 - 4*c0) % p] == -1`.
`irrc()` = monic cubic with random low coefficients, resampled until it has no
root in F_p (uncounted brute evaluation). Products use an uncounted multiply.

| t (g=3) | u | expected D |
|---|---|---|
| 0 | lin(in1) lin(in2) lin(in3) | 1 |
| 1 | lin(in1) lin(in2) lin(out1) | 1 |
| 2 | lin(in1) lin(out1) lin(out2) | 1 |
| 3 | lin(out1) lin(out2) lin(out3) | 0 |
| 4 | lin(in1)^2 lin(in2) | 0 |
| 5 | lin(in1) irrq() | 0 |
| 6 | irrc() | 0 |
| 7 | lin(out1)^2 lin(in1) | 0 |

| t (g=2) | u | expected D |
|---|---|---|
| 0 | lin(in1) lin(in2) | 1 |
| 1 | lin(in1) lin(out1) | 1 |
| 2 | lin(out1) lin(out2) | 1 |
| 3 | lin(out1)^2 | 0 |
| 4 | irrq() | 0 |
| 5 | lin(in1)^2 | 0 |

Oracle (`--oracle`, uncounted): `rts = [x for x in range(p) if peval(u, x) == 0]`.
`D = (len(rts) == g) and sum(not inS[x] for x in rts) <= 2`.

## 7. `run_cell.py`

Arguments: `--cell STR --g INT --p INT --T INT --seed INT [--nfb INT]
--arms {AB,C,Cpf,all} --msweep CSV [--planted] [--oracle] --out PATH`.

- Arm groups. `AB` = `[A_pf, A_nopf, A_pf_aa, B, B_pf]`. `C` = `C_m{m}` for m in
  msweep. `Cpf` = `Cpf_m{m}` for m in msweep. `all` = AB + C + Cpf.
- Steps: check primality by trial division (G1; exit code 2 if it fails),
  then setup (section 2) with one `Ops` named `setup` covering the chi table,
  S and FS. Then the inputs, then each arm in the listed order.
- Output: JSON lines, one per arm, plus one first line of type `cell`:
  ```
  {"type":"cell","lead":"L04","cell":..,"g":..,"p":..,"N":..,"T_random":..,"T_planted":..,
   "seed":..,"seed_f":..,"seed_u":..,"seed_root":..,"seed_root_aa":..,"seed_planted":..,
   "setup":{"mul":..,"inv":..,"add":..},"FS_deg":..,"G8_ok":bool,"prime_ok":bool,
   "python":"<sys.version>","code_sha256":"<sha256 of l04.py>"}
  {"type":"arm","cell":..,"arm":"A_pf","m":null|int,"prefilter":bool,"T_cell":..,
   "pre":{mul,inv,add},"dec":{mul,inv,add},"tprod":{..}|null,"trem":{..}|null,"post":{..}|null,
   "roots":{mul,inv,add},
   "fpm_pre_per_cand":..,"fpm_dec_per_cand":..,"fpm_roots_per_cand":..,
   "fpm_total_per_cand":..,"fpm_lab_total_per_cand":..,
   "accepted":..,"splits_or_prefilter_pass":..,"cz_fail":..,"batch_sizes":[..]|null,
   "decisions":"0101..","decisions_sha256":..,
   "recompose_ok":bool,"lp_ok":bool,
   "oracle_ok":bool|null,"planted_ok":bool|null}
  ```
  `splits_or_prefilter_pass` is the number of inputs that reached `roots_of`
  (A) or survived the prefilter (pf arms), else null.
  `recompose_ok`: for every accepted input, the uncounted product of
  `lin(r)` over `rts` equals u. `lp_ok`: every accepted input has at most 2
  roots outside S, and for B and C, every root of G is in S and no root of c
  is in S.
  `oracle_ok`: the decisions equal the oracle on every input (null without
  `--oracle`). `planted_ok`: the planted inputs' decisions equal the
  expected-D table (null without `--planted`).
- Only progress on stderr. No timing fields in the output.

## 8. `analyze.py`

Arguments: `--raw DIR --out PATH --verdict PATH`. Read every `raw/*.jsonl` and
group the lines by `cell`.

1. Gates. G1: `prime_ok`. G8: `G8_ok`. G3: one distinct `decisions_sha256`
   per cell across all arms. G4: in `corr_*` cells, `oracle_ok` and
   `planted_ok` are all true. G5: `recompose_ok`, `lp_ok` and `cz_fail == 0`
   for all arms. G7: per cell,
   `abs(fpm_total(A_pf_aa) / fpm_total(A_pf) - 1) <= 0.05`. G2: `raw/unittest.log`
   contains a line `OK` and no `FAILED`. Record each gate as pass or fail with
   its failing cells.
2. Per cell: `A_best = min(A_pf, A_nopf)` and
   `batch_best = min(B, B_pf, all C_m*, all Cpf_m*)` on `fpm_total_per_cand`
   (record which arm attains each). `R = A_best / batch_best`. Also
   `R_lab` with `fpm_lab_total_per_cand`.
3. G6: `R(pc_g3_p65521_N8) >= 2.0`.
4. Verdict. If any of G1-G5, G7 or G8 fails: `NO VERDICT (gate failure)`.
   Else if G6 fails: `INCONCLUSIVE (instrument)`. Else, over the three
   `main_g3_*` cells: if all `R >= 2.0`, `SUPPORTED`; if all `R < 2.0`,
   `KILLED`; otherwise `INCONCLUSIVE (p-dependent)`. If a verdict computed
   with `R_lab` differs, state it in a separate line (the primary verdict stays
   the `fpm` one).
5. Secondary (written, no verdict): a table per cell of every arm's
   `fpm_{pre,dec,roots,total}_per_cand`, accepted counts and the acceptance
   fraction; the C/Cpf curve against m; B's `dec` multiplications per candidate
   divided by `g*(N-g+1)`; `N_star = A_best / (2*g)`; setup FPM.
6. Write `analysis.json` (all of the above) and `VERDICT.md`. The first line
   of `VERDICT.md` is the verdict string, followed by the gate table and the
   per-cell R table. Every number is labelled `[pcc:unverified]`.

## 9. Unit tests (`code/test_l04.py`, unittest; expected values fixed here)

All at p = 7 unless stated. `chi = chi_table(7, Ops())` gives
`[0,1,1,-1,1,-1,-1]`.

1. `chi_table(7)` equals the list above, and its `mul` count is 3.
2. `poly_mul([1,1],[2,1],7)` equals `[2,3,1]`, mul = 4.
3. `build_FS([1,2],7)` equals `[2,4,1]`, mul = 3.
4. `disc_character` (value, mul):
   `[1,4,1,1]` → (1, 17); `[5,0,0,1]` → (1, 12); `[3,4,6,1]` → (-1, 17);
   `[4,0,2,1]` → (0, 17); `[2,4,1]` → (1, 7).
5. `splits_completely([1,4,1,1], 7)` is True with mul = 36.
   `splits_completely([5,0,0,1], 7)` is False with mul = 36.
6. Count for p = 65521: `splits_completely(u, 65521)` uses mul = 261 for any
   monic cubic u (check `[1,2,3,1]`) and mul = 114 for any monic quadratic
   (check `[5,7,1]`).
7. `modu_mul` count: 15 at d = 3, 6 at d = 2.
8. `vrem([0]*10+[1], [1,4,1,1], 7)` = r with `peval(r,1) = 1`,
   `peval(r,2) = 2`, `peval(r,3) = 4`, `len(r) <= 3`, `inv = 0`,
   `mul % 3 == 0` and `mul <= 24`.
9. `post_S` with g = 3, S = {1, 2}, FS = `[2,4,1]`, u = `[1,4,1,1]`:
   `R = vrem(FS, u) = [2,4,1]` with 0 mul. `post_S` → accept, `G = [2,4,1]`,
   `c = [4,1]`, post mul = 8, inv = 0.
10. `post_S` with the same S and FS, u = `[4,0,2,1]` (= (x-1)^2 (x-3)):
    `R = vrem(FS, u) = [2,4,1]` (degree 2 < 3, 0 mul). Inside `post_S`,
    `vrem(u, R)` gives `[1,6]`. `G = [6,1]`, `c = [3,3,1]`, disc = 4 is a
    square, `vgcd_monic(c, G)` has degree 1, so reject (repeated root).
11. `roots_of([1,4,1,1], 7, random.Random(1))`, sorted, is `[1,2,3]`.
12. `vgcd_monic([2,4,1],[1,6],7)` = `[6,1]` with inv = 2.
13. Decision identity on a small cell: g = 3, p = 1009, seed = 99, T = 200,
    planted. Every arm (A_pf, A_nopf, B, B_pf, C_m4, Cpf_m4) gives the oracle's
    decisions, and planted_ok holds.
14. Same as 13 with g = 2.
15. C_m1 and B give equal `dec` totals on the inputs of test 13 (tree costs
    0 at m = 1).
