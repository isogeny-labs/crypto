# L02 implementation specification

Implements [`PROTOCOL.md`](PROTOCOL.md). Every choice is fixed here; the
implementer makes none. Where this file and PROTOCOL.md disagree, PROTOCOL.md wins
and the implementer stops and reports the conflict instead of choosing.

## 1. Language, layout, limits

- Python 3, standard library only (`hashlib`, `heapq`, `json`, `math`, `random`,
  `argparse`, `sys`, `time` only for a `wall_s` field that is never analysed).
- All files under `code/` in the lead directory; outputs under `raw/`. Write
  nothing elsewhere. No git commands.
- Modules: `ff.py`, `poly.py`, `jac.py`, `order.py`, `aut.py`, `ic.py`,
  `rho.py`, `seeds.py`, `select_instances.py`, `gates.py`, `run_ic.py`,
  `run_rho.py`, `analyze.py`, `selftest.py`. Each runnable script uses `argparse`
  with exactly the flags in PROTOCOL section 9. Scripts import siblings by putting
  `os.path.dirname(__file__)` first on `sys.path`.
- Single thread. No multiprocessing.

## 2. Seeds (`seeds.py`)

```
def seed_of(label: str) -> int:
    return int.from_bytes(hashlib.sha256(label.encode()).digest()[:8], 'big')
def rng(label: str) -> random.Random:
    return random.Random(seed_of(label))
```

Labels (exact strings, `{}` filled with decimal integers or the curve id):

| use | label |
|---|---|
| control polynomial | `ctrl:{q}:{p}:{j}` |
| instance `G`, `T`, `t` | `inst:{curve_id}:{s}` |
| random divisors for `#J` checks | `order:{curve_id}` |
| transport check (G2) | `transport:{curve_id}` |
| IC walk | `icwalk:{curve_id}:{s}` |
| IC root finding (Cantor-Zassenhaus) | `cz:{curve_id}:{s}` |
| rho | `rho:{curve_id}:{s}` |
| bootstrap | `random.Random(20261116)` |

`curve_id` = `F5:{p}:{a}`, `F7:{p}:{a}`, `C5:{p}:j{j}`, `C7:{p}:j{j}`.

## 3. Counted field arithmetic (`ff.py`)

```
class Ctr:  # one global instance CTR; fields are ints
    mul, inv, rmul, rinv  # F_p mults, F_p inversions, mod-r mults, mod-r inversions
    enabled = True
    def reset(self); def snapshot(self) -> dict
```

Functions over `F_p` (`p` passed or held in a `Field` object with attribute `p`):

- `fmul(a, b)`: `CTR.mul += 1` (if enabled); returns `a * b % p`. Squaring is
  `fmul(a, a)` (counts 1).
- `finv(a)`: asserts `a != 0`; `CTR.inv += 1`; returns `pow(a, p - 2, p)`
  (the builtin pow inside `finv` is not counted as multiplications).
- `fadd`, `fsub`, `fneg`: not counted.
- `fpow(a, e)`: left-to-right square-and-multiply using `fmul` (counted).
- `is_square(a)`: `a == 0 -> False`; else `fpow(a, (p-1)//2) == 1` (counted).
- `sqrt_mod(a)`: Tonelli-Shanks with `fmul`/`fpow` (counted); the non-residue is
  the smallest `z >= 2` with `pow(z, (p-1)//2, p) == p - 1` (builtin, uncounted,
  computed once per field and cached).
- Mod `r`: `rmul(a, b)` (`CTR.rmul += 1`), `rinv(a)` (`CTR.rinv += 1`, builtin pow).
- Multiplication by a small constant (2, 4, ...) is `fmul` (counted 1).

Uncounted contexts: instance set-up, `#J`, gates, certification, analysis. Use
`with uncounted():` (a context manager that sets `CTR.enabled = False` and
restores it).

## 4. Polynomials (`poly.py`) and the group law (`jac.py`)

Polynomials: Python lists of ints, low degree first, trimmed (no trailing zeros;
zero polynomial is `[]`). Functions, all using `fmul`/`finv`:

- `padd`, `psub`, `pneg` (uncounted adds).
- `pmul(a, b)`: schoolbook, one `fmul` per pair of coefficients `(i, j)`, zero
  or not (count `len(a) * len(b)`; 0 if either is `[]`).
- `pdivmod(a, b)`: long division. If `b[-1] != 1`, one `finv(b[-1])`, and one
  `fmul` per quotient coefficient for `c = a_top * inv`; if `b[-1] == 1`, no
  inversion and `c = a_top`. For each quotient coefficient, one `fmul` per
  coefficient of `b` below the top (`len(b) - 1` mults).
- `pmonic(a)`: if `a[-1] != 1`: one `finv`, `len(a) - 1` `fmul`.
- `xgcd(a, b) -> (d, s, t)` with `d` monic, `s a + t b = d`: Euclid with
  `pdivmod`, Bezout updates with `pmul`/`psub`, final scaling of `d, s, t` by
  `finv(lead(d))` (1 inv, `len(d) - 1 + len(s) + len(t)` mults; skip if lead is 1).
- `peval(a, x)`: Horner, `len(a) - 1` `fmul`.

Divisor: tuple `(u, v)`, `u` monic, `deg v < deg u`, identity `([1], [])`.

`jac.add(D1, D2, C)` = Cantor (composition then reduction), exactly:

```
d1, e1, e2 = xgcd(u1, u2)
d, c1, c2  = xgcd(d1, padd(v1, v2))
s1, s2, s3 = pmul(c1, e1), pmul(c1, e2), c2
u = pdivmod(pmul(u1, u2), pmul(d, d))[0]
num = s1*u1*v2 + s2*u2*v1 + s3*(v1*v2 + f)        # pmul/padd, left to right
v = pdivmod(pdivmod(num, d)[0], u)[1]
while deg u > g:
    u = pdivmod(psub(f, pmul(v, v)), u)[0]
    v = pdivmod(pneg(v), u)[1]
u, v = pmonic(u) (scale), v unchanged
```

`jac.neg(D) = (u, pneg(v))` (free). `jac.dbl(D) = add(D, D)`. `jac.smul(k, D)`:
left-to-right double-and-add from the most significant bit, `k >= 0`, `k = 0`
gives identity; negative `k` uses `neg`. `jac.smul_rl(k, D)`: right-to-left
binary, used only for certification inside `uncounted()`.
`jac.key(D, g, p)`: list `[deg u] + u[0..d-1] padded with 0 to length g + v
padded to length g`; `jac.K(D) = sum(c * p**i for i, c in enumerate(key))`.
Every group addition inside counted code increments `CTR`-independent integer
`grp_ops` held by the caller (IC and rho count their own additions).

`jac.random_divisor(C, rnd)`: sum (Cantor, uncounted) of `g` points
`P - inf`, each: draw `x = rnd.randrange(p)` until `f(x)` is a nonzero square,
`y = sqrt_mod(f(x))`, if `rnd.randrange(2)` then `y = p - y`; point divisor is
`([(-x) % p, 1], [y])`.

## 5. Curves and orders (`order.py`, `select_instances.py`)

`Curve` fields: `family, curve_id, p, q, g, f (list, low first, monic, deg q),
a (CM) or j (control), zeta (CM else 1), lam (CM else 1), h, r, nA, delta0,
nB (= (nA - delta0)//q for CM, = nA for controls)`.

- `prim_root(p)`: smallest `gamma >= 2` with `pow(gamma, (p-1)/l, p) != 1` for all
  prime `l | p - 1`. `zeta = pow(gamma, (p-1)//q, p)`.
- `N1 = 1 + sum_{x in F_p} (1 + legendre(f(x)))` with `legendre(0) = 0`.
- Genus 3 also `N2`: `F_{p^2} = F_p[s]/(s^2 - dn)`, `dn` = smallest non-residue;
  for each `x = x0 + x1 s`, evaluate `f(x)` in `F_{p^2}` (Horner), norm
  `n = c0^2 - dn c1^2 mod p` of `f(x) = c0 + c1 s`, and
  `N2 = 1 + sum_x (1 if f(x) == 0 else 1 + legendre(n))`. Precompute a Legendre
  table for `F_p`. All uncounted.
- `s_k = p^k + 1 - N_k`; `e1 = s1`, `e2 = (s1^2 - s2)/2`; `a1 = -e1`, `a2 = e2`.
- Genus 2: `#J = 1 + a1 + a2 + p a1 + p^2`; `a2` unknown in
  `[ceil(2 sqrt(p) |a1|) - 2p - 1, floor(a1^2 / 4) + 2p + 1]`; base
  `N0 = 1 + a1 + p a1 + p^2`; candidates `N0 + a2`.
- Genus 3: `#J = 1 + a1 + a2 + a3 + p a2 + p^2 a1 + p^3`; `a3` in
  `[-20 p^1.5 - 1, 20 p^1.5 + 1]` (integer floor/ceil); base
  `N0 = 1 + a1 + a2 + p a2 + p^2 a1 + p^3`.
- `bsgs_candidates(C, D, lo, hi)`: `m = ceil(sqrt(hi - lo + 1))`; baby table
  `key(jD) -> [j]` for `j = 0..m-1`; `Q = [N0 + lo] D`, step `S = [m] D`; for
  `i = 0..ceil((hi - lo + 1)/m)`: look up `key(neg(Q))`, for each `j` found the
  candidate `N0 + lo + i m + j` if within `[N0 + lo, N0 + hi]`; `Q = Q + S`.
- `jacobian_order(C)`: 32 random divisors from `rng("order:{curve_id}")`;
  candidates from the first divisor; keep those with `[N] D_i = identity` for
  all 32. Return the unique survivor, else `None` (curve rejected, reason
  `order_ambiguous`).
- Admission: `N = h r`, `r` = largest prime factor of `N` (trial division), `h = N / r`,
  admitted iff `r > h` and `h <= 64`.
- `lambda` (CM only): `l0 = pow(w, (r-1)//q, r)` for `w = 2, 3, ...` until
  `l0 != 1`; candidates `l0^j mod r`, `j = 1..q-1`; `lam` = the unique candidate
  with `aut.rho_apply(G0) == smul(lam, G0)` where `G0 = [h] D` for the first
  random divisor with `G0 != identity`. Zero or two matches: curve rejected
  (`lambda_fail`).
- `nA = #{x in F_p : f(x) nonzero square}`; `delta0 = 1` iff `f(0)` is a nonzero
  square (CM only; controls `delta0 = 0`).

Scan order exactly as PROTOCOL section 3. Output `raw/instances_{family}.jsonl`,
one JSON object per admitted curve with every `Curve` field plus `bits` (the `k`
of the scan), and one object per rejected candidate with
`{"rejected": true, "curve_id", "p", "reason"}` (`reason` in
`not_squarefree, order_ambiguous, not_near_prime, lambda_fail, trivial_ctrl`).
Controls take `bits` from their CM curve.

## 6. Automorphism maps (`aut.py`)

- `rho_apply(D, C, k=1)`: `d = deg u`; `u'_i = zeta^((d - i) k) u_i`,
  `v'_i = zeta^(-i k) v_i` (exponents mod `q`, table `ZP[e] = zeta^e`, built
  uncounted). Counted: one `fmul` per coefficient `u_0..u_{d-1}, v_0..v_{d-1}`
  whose exponent mod `q` is nonzero.
- `canon_neg(D)`: let `v` top nonzero coefficient (highest index) be `c`; if
  `c > (p-1)/2` return `(neg(D), -1)` else `(D, +1)`; `v = []` returns `(D, +1)`.
  No multiplications.
- `canon_aut(D, C)` (requires `q > 1`; for `q == 1` it is `canon_neg` with `k = 0`):
  1. `d = deg u`. `j0` = smallest `j` in `1..d` with `u_{d-j} != 0`.
  2. If `j0` exists: values `c_k = ZP[(j0 k) % q] * u_{d-j0}` for
     `k = 1..q-1` (`q - 1` `fmul`), `c_0 = u_{d-j0}`; `k*` = smallest `k`
     attaining `min c_k`. `D1 = rho_apply(D, C, k*)` (counted as above); then
     `(D2, s) = canon_neg(D1)`. Return `(D2, s, k*)`.
  3. Else (`u = x^d`): `i0` = smallest `i` in `1..d-1` with `v_i != 0`. If it
     exists: over all `(s, k)` in `{+1, -1} x {0..q-1}` the value
     `s * ZP[(-i0 k) % q] * v_{i0} mod p` (`q - 1` `fmul` for `k >= 1`; the
     `s = -1` values by negation); choose the minimum value, ties impossible;
     apply `rho_apply(D, C, k*)` then negate if `s* = -1`. Return `(D2, s*, k*)`.
  4. Else: `(D2, s) = canon_neg(D)`, `k* = 0`.
- Bookkeeping: a walk point `W = aG + bT` becomes `(s lam^k* a, s lam^k* b) mod r`,
  computed with `rmul` (counted in `CTR.rmul`, reported, **excluded from FPM**,
  PROTOCOL 5).
- `orbit_rep_point(x, C)` (IC arm B): if `q == 1`: return `(x, 0)`. Else
  `imgs[k] = ZP[k] * x` for `k = 1..q-1` (`q - 1` `fmul`, computed iteratively
  `imgs[k] = fmul(imgs[k-1], zeta)`), `imgs[0] = x`; `xs = min(imgs)`;
  `kp` = smallest index with `imgs[kp] == xs`; return `(xs, (-kp) % q)` meaning
  `x = zeta^k xs`.

## 7. IC relation phase (`ic.py`)

Inputs: curve `C`, seed `s`, arm in `{A0, A, B}`, `G`, `T` (from `inst` label;
`G = [h] D` with `D = random_divisor`, redraw while `G == identity`; `t =
rnd.randrange(1, r)`; `T = smul(t, G)`; all uncounted).

`ncols`: `A0: 2 nA`; `A: nA`; `B: nB` (CM) or `nA` (control).
`target = ncols + 10`.

Walk (counted from here): `w = rng("icwalk:{curve_id}:{s}")`. Jumps
`R_j = smul(alpha_j, G) + smul(beta_j, T)` for `j = 0..15`, `alpha_j = w.randrange(r)`,
`beta_j = w.randrange(r)` (the `+` is one `jac.add`). Start
`(a, b) = (w.randrange(r), w.randrange(r))`, `W = smul(a, G) + smul(b, T)`.
Loop: `j = H16(W) = ((K(W) * 0x9E3779B97F4A7C15) & (2**64 - 1)) >> 60`;
`W = add(W, R_j)`; `a += alpha_j; b += beta_j (mod r, uncounted)`; `trials += 1`;
test `W`. Every 4096 trials (after the test of trial 4096, 8192, ...) restart
from a fresh `(a, b)` drawn from `w`. `grp_ops` counts every `jac.add` including
those inside `smul` and set-up.

Split test of `W = (u, v)`: if `deg u != g`: no relation. Else
`xp = x^p mod u` by left-to-right square-and-multiply of the polynomial `[0, 1]`
(square = `pmul` then `pdivmod(., u)`; multiply-by-x = shift then `pdivmod`).
If `xp != [0, 1]`: no relation. Else roots by `cz_roots(u)` with
`z = rng("cz:{curve_id}:{s}")` created once per run:

```
cz_roots(u): if deg u == 1: return [(-u[0]) % p]
  loop: delta = z.randrange(p)
        hp = (x + delta)^((p-1)/2) mod u        # square-and-multiply, counted
        g1 = pmonic(xgcd(u, psub(hp, [1]))[0])  # counted
        if 0 < deg g1 < deg u: return cz_roots(g1) + cz_roots(pdivmod(u, g1)[0])
```

For each root `xi`: `yi = peval(v, xi)`. If any `yi == 0`: no relation.
Otherwise a relation `(a, b, [(xi, yi)])` (roots are distinct because
`x^p = x mod u`).

Row entries (dict `col -> coef mod r`, summing duplicates, dropping zeros):

- `A0`: col `xi * p + yi`, coef `1`.
- `A`: col `xi`, coef `1` if `yi <= (p-1)//2` else `r - 1`.
- `B`: if `q > 1` and `xi == 0`: skip the point. Else `(xs, k) = orbit_rep_point(xi)`,
  sign as in `A`, coef `sign * LP[k] mod r` (`LP[k] = lam^k mod r`, table built
  uncounted). Col `xs`.

Row: `{"cols": dict, "tc": (-b) % r, "rhs": a % r}`; it encodes
`sum coef * l(col) - b t = a`.

Collection: append rows until `len(rows) >= target`; then `solve`. On failure,
continue the walk until 10 more rows, `solve` again on all rows (fresh copy),
at most 5 extra rounds; then `status = "la_fail"`.

Trial digest: `hd = sha256()`; after each trial `hd.update(str(K(W)).encode() + b";")`;
at every trial count that is a multiple of 1000 append `hd.hexdigest()[:16]` to
`digests`. Report also the final digest and trial count.

## 8. Linear algebra (`ic.py: solve`) — counted with `rmul`, `rinv`

```
rows: list of dicts (copy), tc[i], rhs[i]
col_rows: dict col -> set(row ids)
heap of (len(col_rows[c]), c); push (count, c) whenever a count changes
active = set(all row ids)
while heap:
    (n, c) = heappop(heap)
    if c not in col_rows or len(col_rows[c]) != n or n == 0: continue
    piv = min(col_rows[c], key=lambda i: (len(rows[i]), i))
    others = sorted(col_rows[c] - {piv})
    if others: inv = rinv(rows[piv][c])
    for R in others:
        fac = rmul(rows[R][c], inv)
        for (j, val) in rows[piv].items():
            if j == c: continue
            new = (rows[R].get(j, 0) - rmul(fac, val)) % r
            update rows[R][j] / remove if 0; update col_rows[j]; push new count
        delete rows[R][c]; col_rows[c].discard(R)
        tc[R]  = (tc[R]  - rmul(fac, tc[piv]))  % r
        rhs[R] = (rhs[R] - rmul(fac, rhs[piv])) % r
        track fill: max_fill = max(max_fill, len(rows[R]))
    retire piv: for j in rows[piv]: col_rows[j].discard(piv), push new count
    active.discard(piv); del col_rows[c]
for i in sorted(active):          # rows[i] is now empty
    if tc[i] != 0:
        t = rmul(rhs[i], rinv(tc[i]))
        if certify(t): return t
return None
```

`certify(t)`: inside `uncounted()`: `smul_rl(t, G) == T` and `t == t_planted`.
Report `dim` = number of distinct columns over all rows before elimination,
`nnz` = total entries before elimination, `max_fill`.

## 9. Rho (`rho.py`)

Inputs: curve, seed `s`, arm `Rneg` or `Raut`, same `G, T, t` as section 7 for
the same `s` (from `inst:{curve_id}:{s}`).

- `canon = canon_neg` (Rneg, class size `m = 2`) or `canon_aut` (Raut,
  `m = 2q`; `m = 2` when `q == 1`). The returned `(s, k)` updates `(a, b)`.
- `x = rng("rho:{curve_id}:{s}")`. 32 jumps `R_j = [alpha_j] G + [beta_j] T`
  (counted). `E = sqrt(pi r / (2 m))`, `k_dp = max(2, floor(log2(E) / 2))`,
  `maxlen = 20 * 2**k_dp`, `cap = 40 * ceil(E)` steps.
- `H1(W) = ((K(W) * 0x9E3779B97F4A7C15) & (2**64-1)) >> 59` (0..31);
  `isDP(W) = (((K(W) * 0xC2B2AE3D27D4EB4F) & (2**64-1)) >> (64 - k_dp)) == 0`.
- New walk: `(a, b)` from `x`, `W = canon(smul(a, G) + smul(b, T))`, `len = 0`.
- Step: `j = H1(W)`; for `i = 0..31`: `jj = (j + i) % 32`;
  `W' = canon(add(W, R_jj))`; `steps += 1`; if `i > 0`: `lookahead_extra += 1`;
  if `H1(W') != jj` or `i == 31`: accept `W'` (with its `(a, b)`) and stop trying.
  `len += 1`.
- After each accepted step: if `isDP(W)`: look up `key(W)` in `dp` (dict
  `tuple(key) -> (a, b)`). Absent: store. Present with `(a', b')`: if
  `(b - b') % r != 0`: `t = (a' - a) * inverse(b - b') mod r` (uncounted
  bookkeeping), certify; on success stop. Then start a new walk. If
  `len > maxlen` without DP: `walks_abandoned += 1`, `abandoned_steps += len`,
  new walk. If `steps > cap`: `status = "cap"`, stop.
- Check of `(a, b)`: in `selftest.py` only, after every step for 200 steps,
  `W == aG + bT` (uncounted).

## 10. Output formats

IC (`raw/ic_*.jsonl`), one JSON object per run:
`run_id ("{curve_id}|{arm}|{s}"), family, curve_id, bits, p, g, q, h, r, lam, seed,
arm, ncols, nA, delta0, nB, trials, walk_restarts, relations_collected,
relations_target, la_attempts, grp_ops, rel_mul, rel_inv, la_mul, la_inv, dim,
nnz, max_fill, t_found, t_planted, certified, digests (list), final_digest,
status ("ok" | "la_fail"), wall_s`.

`rel_mul, rel_inv` = `CTR.mul, CTR.inv` at the end of the relation phase,
including all `solve`-interleaved walking; `la_mul, la_inv` = `CTR.rmul, CTR.rinv`
accumulated over all `solve` calls. Reset `CTR` at the start of each run.

Rho (`raw/rho_*.jsonl`): `run_id, family, curve_id, bits, p, g, q, h, r, seed,
arm, m, k_dp, E, steps, lookahead_extra, walks_started, walks_abandoned,
abandoned_steps, dps_stored, grp_ops, mul, inv, rmul_bookkeeping, t_found,
t_planted, certified, status ("ok" | "cap"), wall_s`.

Gates (`raw/gates_instances.jsonl`): per curve `{curve_id, G1_order_unique,
G1_r_prime, G1_annihilate32, G1_lambda_unique, G1_r_mod_q, G2_transport (20 bools
reduced to all()), pass}`. G2: points from `rng("transport:{curve_id}")` as in
`random_divisor` but single points; check
`smul(h, rho_apply(P)) == smul(lam, smul(h, P))`.

## 11. Analysis (`analyze.py`)

1. Load all `raw/*.jsonl` rows; load `gates_instances.jsonl`; drop curves failing
   G1/G2 (list them). Check G3 (count uncertified per cell and arm; void cell if
   > 5%), G4 (for each CM `(curve, s)`, digest lists of A0, A, B agree on the
   common prefix), G5, G6 (for controls: every counter field except `arm`,
   `run_id`, `wall_s` equal between A and B and between Rneg and Raut on the same
   `s`), G7 (cell mean of `relations_collected_A0 / relations_collected_A` over
   paired runs in `[1.8, 2.2]`). Write each gate's result to the output.
2. FPM per section 5 of PROTOCOL for `(I, kappa)` in `(1,1), (10, g^2), (30, g^2)`.
3. Per cell `(family, bits)`: per-curve per-arm lists over seeds; means; `D1`,
   `rho_gain`, `N`, `rel_ratio`, predicted `(nA+10)/(nB+10)` (mean over curves),
   `pos_ratio`, `gap_A`, `gap_B`, `LA_model = 2 dim nnz` means, mean
   `abandoned_steps/steps` for Raut and Rneg.
4. Bootstrap per PROTOCOL 5: 2000 replicates; replicate = resample curve indices
   with replacement (shared across arms of the cell), then for each drawn curve
   and each arm resample its seed values with replacement; recompute arm means
   and statistics. Pooled `N`: geometric mean of per-cell `N` across the
   decision cells, computed inside each replicate.
5. Verdicts per PROTOCOL 6 for F5 (cells 12, 13, 14) and F7 (cells 9, 10) at all
   three band points; `band_dependent` flag.
6. Outputs: `raw/analysis.json` (everything above, nested by family and cell),
   `raw/summary.tsv` with header
   `family	bits	reading	n_curves	IC_A	IC_B	R_neg	R_aut	D1	D1_lo	D1_hi	rho_gain	N	N_lo	N_hi	rel_ratio	rel_pred	pos_ratio	gap_A	gap_B`
   (one row per cell and reading, plus `bits = pooled` rows).

## 12. Unit tests (`selftest.py`), expected values computed by hand/brute force

Print `PASS name` or `FAIL name: detail` per test; exit 1 on any failure.

1. `prim_root` / `zeta`: `(p, q) -> zeta`: `(11,5) -> 4`, `(31,5) -> 16`,
   `(41,5) -> 10`, `(29,7) -> 16`, `(43,7) -> 41`.
2. `N1, N2`: `x^5+2, p=11`: `N1 = 21, N2 = 123`. `x^7+2, p=43`: `N1 = 43, N2 = 1879`.
3. `#J` by `jacobian_order`: `p=11, x^5+1 -> 80`; `p=11, x^5+2 -> 271`;
   `p=31, x^5+1 -> 880`; `p=41, x^5+3 -> 1136`; `p=43, x^7+1 -> 50624`;
   `x^7+2 -> 77813`; `x^7+3 -> 108431`; `x^7+5 -> 101459`. (For genus 2 the
   test may also brute-force count reduced divisors for `p = 11` and compare.)
4. Group axioms on `p = 31, x^5+1` and `p = 43, x^7+2`: 50 random triples:
   associativity, `D + neg(D) = 0`, `[#J] D = 0`, `smul == smul_rl`.
5. `rho_apply`: `p=11, f=x^5+2, D = ([10,1],[5])` -> `([7,1],[5])`.
   `p=43, f=x^7+2, D = ([41,1],[1])` -> `([4,1],[1])`.
6. `lambda`: `p=11, x^5+2, r=271`: roots of `Phi_5 mod 271` are
   `{10, 100, 187, 244}`, `lam = 100`; `smul(10, D) = ([2,1],[5])`,
   `smul(100, D) = ([7,1],[5])`, `smul(187, D) = ([8,1],[5])`,
   `smul(244, D) = ([6,1],[5])` for `D` of test 5. `p=43, x^7+2, r=77813`:
   `lam = 19186`.
7. `canon_aut`, `p=11, x^5+2`: `([10,1],[5]) -> (([2,1],[5]), s=+1, k=3)`;
   `([10,1],[6]) -> (([2,1],[5]), s=-1, k=3)`. For 200 random `D` on
   `p=31, x^5+1` and `p=43, x^7+2`: all `2q` images `s rho^k D` have the same
   canonical form.
8. `orbit_rep_point`, `p=11, zeta=4`: `x -> (xs, k)`: `1->(1,0) 2->(2,0) 3->(1,4)
   4->(1,1) 5->(1,2) 6->(2,4) 7->(2,3) 8->(2,1) 9->(1,3) 10->(2,2)`; and
   `ZP[k] * xs == x` for each.
9. Base sizes: `p=11, x^5+2: nA=10, delta0=0, nB=2`; `p=31, x^5+1: nA=11,
   delta0=1, nB=2`; `p=43, x^7+2: nA=21, delta0=0, nB=3`.
10. `solve` on a hand system mod `r = 101`, true `l1 = 3, l2 = 5, t = 7`:
    rows `(cols, b, a)`: `({1:1, 2:1}, 1, 1)`, `({1:1, 2:100}, 2, 85)`,
    `({2:1}, 3, 85)`, `({1:1}, 1, 97)` -> `t = 7` (use a certify stub `t == 7`).
11. Transport: on `p=11, x^5+2` and `p=43, x^7+2`, every base point `P`:
    `rho_apply(P) == smul(lam, P)` (here `h = 1`).
12. Counter sanity: `fmul` increments `mul` by 1; `finv` increments `inv` by 1;
    inside `uncounted()` neither changes.
13. End to end: curve `F5:11:2` (h=1, r=271) and `F7:43:2` (h=1, r=77813),
    seed 1: IC arms A0, A, B and rho Rneg, Raut all certify; `ncols` A0/A/B =
    `20/10/2` and `42/21/3`; digests of A0, A, B agree on the common prefix.
14. Control degeneration: a control curve at `p = 31` (first admitted `j`, or if
    none, `f = x^5 + x + 1` used directly without admission): B with `q = 1`
    equals A, Raut equals Rneg, field by field.
15. Rho bookkeeping: test of section 9 (`W == aG + bT` for 200 steps) for both
    arms on `F5:11:2` and `F7:43:2`.
