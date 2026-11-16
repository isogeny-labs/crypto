# L06 implementation specification

Frozen with PROTOCOL.md and DERIVATION.md (`frozen.sha256`). The implementer makes no
research choices: where this file is silent, choose the simplest correct option and
record it in IMPLEMENTATION_NOTES.md. Any change to an algorithm, parameter, seed or
threshold goes in DEVIATIONS.md, and the protocol's verdict rule is not changed.

## 1. Files and language

All paths are relative to `research/g2plus_leads_20261116/L06_odd_ell_isogeny_census/`.

```
l06/Cargo.toml        package "l06", edition 2021, NO dependencies, and an empty
                      [workspace] table so it is not part of the repository workspace.
                      [profile.release] opt-level = 3, debug = false.
l06/.gitignore        contains: target/
l06/src/main.rs       CLI: subcommands lagr2, gsp, gate-enum, census (section 10)
l06/src/rng.rs        SplitMix64, uniform sampling (section 2)
l06/src/fp.rs         arithmetic mod p, Legendre, sqrt, Miller-Rabin (section 3)
l06/src/poly.rs       polynomials over F_p and over F_l (section 3)
l06/src/cantor.rs     Mumford/Cantor arithmetic, random divisors (section 4.1-4.2)
l06/src/order.rs      BSGS order search and chi assembly (section 4.3-4.5)
l06/src/enumchi.rs    chi by enumeration over F_p, F_{p^2}, F_{p^3} (section 5)
l06/src/crit.rs       the mod-l criterion, factorisation, classes (section 6)
l06/src/gsp.rs        reference sampler and charpoly (section 7)
l06/src/lagr2.rs      J[2] Lagrangian table (section 8)
l06/tests/*.rs or #[cfg(test)] modules: the unit tests of section 11
analyze.py            Python 3 standard library only (section 12)
```

Rust standard library only (no crates). Single thread. JSON is written by hand (small
helper that formats integers, booleans, strings, arrays, objects). All integers are
exact (`u64`, `i64`, `i128`, `u128`); floating point appears only in `analyze.py`
statistics and in the `gsp` z-scores.

## 2. Randomness

SplitMix64, exactly:
```
state = seed
next(): state = state.wrapping_add(0x9E3779B97F4A7C15)
        z = state
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9)
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB)
        return z ^ (z >> 31)
```
Check: seed 0 gives `0xE220A8397B1DCDAF` then `0x6E789E6AA1B965F4`.

`uniform(n)` for `n >= 1`: `zone = u64::MAX - (u64::MAX % n)`; loop `x = next()`; if
`x < zone` return `x % n`. `bit()` = `next() >> 63`.

Seeds (`^` is XOR, `<<` shift, all `u64`):
- `curve_seed(p, i) = 0x4C06_0000_0000_0000 ^ (p << 24) ^ i`.
- coefficients: `SplitMix64(curve_seed)`, draw `c0, c1, ..., c6 = uniform(p)` in that
  order; `f = x^7 + c6 x^6 + ... + c0`.
- order search divisors: `SplitMix64(curve_seed ^ 0xD1B5_4A32_D192_ED03)`; the same
  stream is used first for `J`, then continued for the twist.
- gate G2 divisors: `SplitMix64(curve_seed ^ 0x8CB9_2BA7_2F3D_8DD7)`; first 3 on `J`,
  then 3 on the twist.
- reference sampler: `SplitMix64(0x4C06_5500_0000_0000 ^ (g << 32) ^ (l << 16) ^ m)`.

## 3. Base arithmetic

`fp.rs`, for prime `p < 2^13` (assert `p` prime by trial division at start):
- `add, sub, mul, neg, inv` (inverse by Fermat `a^(p-2)`; assert `a != 0`).
- `legendre(a)`: 0 if `a = 0`, else `a^((p-1)/2)` mapped to `1` or `-1`. A table
  `is_square[0..p)` may be precomputed and must agree.
- `sqrt(a)` for a square `a`: Tonelli-Shanks with the smallest non-residue `z`.
  Returns some root; the caller chooses the sign.
- `nu` = the smallest quadratic non-residue mod `p` (twist constant).
- `is_prime_u64(n)`: `n < 2` false; small primes up to 37 by division; otherwise
  Miller-Rabin with bases 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37 using `u128` for
  `mulmod` (deterministic for `n < 3.3 * 10^24`).

`poly.rs`, polynomials over `F_q` (`q = p` or `q = l`) as `Vec<u64>` low-to-high,
trimmed so the last coefficient is nonzero; the zero polynomial is the empty vector.
Functions: `add, sub, mul, divrem` (by a nonzero divisor), `monic`, `gcd` (monic
result), `xgcd(a, b) -> (d, s, t)` with `d = s a + t b`, `d` monic, `deriv`,
`powmod(base, e, m)`, `eval`. Performance note for `cantor.rs`: degrees never exceed
14, so fixed arrays `[u64; 16]` with a length field are allowed and recommended;
products of two reduced coefficients are below `2^26`, so up to `2^37` of them can be
summed in a `u64` before one reduction.

Distinct-degree factorisation of a squarefree `f` of degree 7 over `F_p`:
```
rem = f; h = x; degrees = []
for d in 1..=7:
    h = powmod(h, p, f)
    g = gcd(rem, h - x)
    if deg g > 0: push d (deg g / d) times onto degrees; rem = rem / g
    if deg rem == 0: break
```
`cycle_type` = `degrees` sorted descending, then a trailing `1` for the point at
infinity (8 points total). `k = len(degrees)`. `f` irreducible iff `degrees == [7]`.
Squarefree test: `deg gcd(f, f') == 0`.

## 4. chi for census curves

### 4.1 Mumford representation and Cantor's algorithm

Curve `y^2 = F(x)`, `F` of degree 7 with leading coefficient `lc` (`lc = 1` for `J`,
`lc = nu` for the twist `F = nu f`). Genus 3. A reduced divisor class is `(u, v)` with
`u` monic, `deg u <= 3`, `deg v < deg u`, `u | v^2 - F`. Identity: `(1, 0)`.
Negation: `(u, -v)`.

Composition `add((u1, v1), (u2, v2))`:
```
(d0, e1, e2) = xgcd(u1, u2)                 # d0 = e1 u1 + e2 u2
(d, c1, c2)  = xgcd(d0, v1 + v2)             # d = c1 d0 + c2 (v1 + v2)
s1 = c1 e1; s2 = c1 e2; s3 = c2
u = (u1 u2) / d^2                             # exact division
v = ((s1 u1 v2 + s2 u2 v1 + s3 (v1 v2 + F)) / d) mod u   # exact division by d
reduce:
while deg u > 3:
    u' = (F - v^2) / u                        # exact division
    v' = (-v) mod u'
    u = monic(u'); v = v'
u = monic(u); v = v mod u
```
`xgcd` with a zero argument: `xgcd(a, 0) = (monic(a), 1/lc(a), 0)`; `xgcd(0, 0)` does
not occur for valid inputs. `double(D) = add(D, D)`. Scalar multiplication
`mul(n, D)` for `n: i64`: if `n < 0` use `mul(-n, neg(D))`; left-to-right
double-and-add. Assert every exact division has zero remainder (debug and release).

### 4.2 Random divisor classes

`random_point(F, rng)`: loop: `x0 = uniform(p)`, `w = F(x0)`; if `w = 0` return
`(x0, 0)`; if `legendre(w) = 1`: `y0 = sqrt(w)`, if `bit() = 1` then `y0 = -y0`,
return `(x0, y0)`; else repeat.
`point_divisor(x0, y0) = (x - x0, y0)`.
`random_class(F, rng)`: loop: `D = add(add(pd(P1), pd(P2)), pd(P3))` for three
independent `random_point` draws; if `D != identity` return `D`.
(This samples the subgroup spanned by `F_p`-rational points. The protocol accounts for
that; do not change it.)

### 4.3 Order search for J (BSGS returning all candidates)

```
N1 = 1 + sum_{x in F_p} (1 + legendre(f(x)))     # one point at infinity
s1 = p + 1 - N1
C  = 1 - s1 (1 + p^2) + p^3          # chi(1) = C + s2 (1 + p) - s3
Ct = 1 + s1 (1 + p^2) + p^3          # chi(-1) = Ct + s2 (1 + p) + s3
B  = 15 p (p + 1) + 20 * isqrt(p^3) + 20
```
`bsgs_all(F, D, L, R, s)`: all `n` with `L <= n <= R`, `n = L (mod s)`, `n D = 0`.
```
T = (R - L) / s                  # integer division
E = mul(s, D);  m = isqrt(T + 1); if m*m < T + 1 then m += 1
table: key(j E) -> list of j, for j = 0 .. m-1      (j E built by repeated add)
G = mul(L, D); step = mul(m, E)
for i = 0 .. ceil((T + 1) / m) - 1:
    for j in table.get(key(neg(G))): t = i m + j; if t <= T: record L + s t
    G = add(G, step)
return sorted unique records
```
`key(u, v)`: pack `deg u` (2 bits), the 3 low coefficients of `u` and of `v` (13 bits
each, zero-padded) into a `u128`. Use `std::collections::HashMap<u128, Vec<u32>>` with
a custom `BuildHasher` (a multiply-xorshift hasher on the `u128`) or the default
hasher; either is allowed.

For `J`: `L = C - B`, `R = C + B`, `s = 1`; assert `L > 0`.
1. `D1 = random_class(f)`; `cand = bsgs_all(f, D1, L, R, 1)`. If `len(cand) > 4096`,
   draw a new `D1` and repeat, at most 3 times; then `status = "ambiguous"`.
2. Filter: draw 7 more `D = random_class(f)`; keep `n in cand` with `mul(n, D) = 0`
   for all of them.
3. If `cand` is empty: `status = "error"` (instrument failure, counted).

The uniqueness bound (PROTOCOL.md G8): for all census `p`, `(sqrt p - 1)^6 > 2B + 1`,
so if `#J` is prime, every nonzero class has order `#J` and exactly one candidate
remains after step 1.

### 4.4 Order search for the twist

`F_t = nu f`. For each `n` in the filtered `cand` (at most 16; if more,
`status = "ambiguous"`):
```
modulus = 2 (p + 1); r = (2 + 2 p^3 - n) mod modulus      (in [0, modulus))
L' = smallest value >= Ct - B with L' = r (mod modulus); R' = Ct + B
cand_t(n) = bsgs_all(F_t, D', L', R', modulus) for D' = random_class(F_t),
            filtered by 7 more random_class(F_t) as in 4.3 step 2
pairs += [(n, n') for n' in cand_t(n)]
```
The congruence encodes `nJ + nJtw = 2 (1 + p^3) + 2 s2 (1 + p)`: it makes `s2`
integral and `n' - n` even, and the true twist order always satisfies it. Draw the twist divisors once (1 + 7 classes) and
reuse them for every `n`.

### 4.5 Assembly and checks

If `len(pairs) != 1`: `status = "ambiguous"` (record `n_candidates_J` and
`n_candidates_pair`). Otherwise, with `(nJ, nJtw)`:
```
s2 = ((nJ + nJtw) / 2 - 1 - p^3) / (1 + p)       # assert exact
s3 = (nJtw - nJ) / 2 - s1 (1 + p^2)               # assert exact
assert C + s2 (1 + p) - s3 == nJ and Ct + s2 (1 + p) + s3 == nJtw
```
`chi = [p^3, -p^2 s1, p s2, -s3, s2, -s1, 1]` (coefficients of `T^0..T^6`).
- `gate_weil = (s1^2 <= 36 p) and (|s2| <= 15 p) and (s3^2 <= 400 p^3)` (G3, i128).
- `gate_annihilation` (G2): with the G2 seed stream, 3 `random_class(f)` satisfy
  `mul(nJ, D) = 0` and 3 `random_class(F_t)` satisfy `mul(nJtw, D) = 0`.
- `gate_mod2` (G5): `chi mod 2 == chi2_from_cycle_type(cycle_type)` (section 8.4) and
  `v_2(nJ) >= k - 1` and `(nJ odd) == (k == 1)`.
- `prime = is_prime_u64(nJ)`; `c2 = prime or (nJ even and is_prime_u64(nJ / 2))`.

## 5. chi by enumeration (gate G1 only)

Fields: `F_{p^2} = F_p[t]/(t^2 - nu)`. `F_{p^3} = F_p[t]/(t^3 + a1 t + a0)` with
`(a0, a1)` the first pair in the order `a1 = 0, 1, ...` outer, `a0 = 1, 2, ...` inner,
for which the cubic has no root in `F_p`. Elements are index vectors
`a0 + a1 p (+ a2 p^2)`.
Square tables: `sq_k[z] = true` iff `z` is a nonzero square in `F_{p^k}`: loop over all
nonzero `y`, mark `sq_k[y * y] = true`. (Size `p^3` bytes: 17 MB at `p = 257`.)
`N_k = 1 + sum_{x in F_{p^k}} (1 if f(x) = 0, 2 if sq_k[f(x)], else 0)` with `f(x)` by
Horner in `F_{p^k}`. Then `S_k = p^k + 1 - N_k`, and Newton:
```
s1 = S1;  s2 = (s1 S1 - S2) / 2;  s3 = (s2 S1 - s1 S2 + S3) / 3     (assert exact)
```
`gate-enum --p P --count K`: for the first `K` squarefree draws `i = 0, 1, ...` with
`curve_seed(P, i)`, compute both `chi_enum` and the census `chi` (sections 4.3-4.5) and
write one JSONL line each (section 9).

## 6. The criterion mod l (`crit.rs`)

Input: `chi` as integers, prime `l in {2, 3, 5, 7}`, `q = p mod l`. Reduce `chi` mod `l`
with `rem_euclid` to `c` (monic, degree 6, `c[0] = q^3 mod l`; assert).

6.1 Reciprocal. For monic `h` of degree `d` with `h[0] != 0`:
`h^*[i] = h[d - i] * q^(d - i) / h[0]` for `i = 0..d` (so `h^*[d] = 1`).

6.2 Factorisation of `c`. Repeat for `d = 1, 2, 3`: for each monic `g` of degree `d` in
lexicographic order of `(g[0], ..., g[d-1])` that has no root in `F_l` (all monic
degree-1 polys qualify; for `d = 2, 3` "no root" is irreducibility), while `g | c`:
record `g`, `c = c / g`. If at the end `deg c >= 1`, `c` is irreducible (degree 4, 5
or 6 with no factor of degree <= 3); record it. Output: list of `(g, multiplicity)`.

6.3 Brute-force test. For `g = deg c / 2` (3 here; 1 in the genus-1 sampler gate),
loop over all monic `h` of degree `g` with `h[0] != 0`: valid iff
`h * h^* == c` and `deg gcd(h, h^*) == 0`. `n_h` = number of valid `h`;
`fire = n_h > 0`.

6.4 Factor test (G4): `fire_fac` = no recorded irreducible `g` with `g^* == g`.
`n_pairs` = (number of distinct recorded `g` with `g^* != g`) / 2. G4 holds iff
`fire == fire_fac` and (`fire` implies `n_h == 2^n_pairs`).

6.5 Class: `squarefree` iff every multiplicity is 1. `class = "fire"` if `fire`, else
`"none"` if squarefree, else `"undetermined"`.

6.6 `ext_deg`: for each valid `h`, the least `k >= 1` with `T^k = 1 mod h` (iterate
`t = t * T mod h`; cap 20,000 iterations, exceeding the cap is an error). `ext_deg` is
the minimum over valid `h`; `null` if not firing.

Report per `l`: `chi_mod_l` (7 coefficients), `factor_degrees` (with multiplicity,
sorted descending), `fire`, `fire_fac`, `class`, `n_h`, `n_pairs`, `ext_deg`.

## 7. Reference sampler (`gsp.rs`)

Dimension `2g` (`g = 3` for the reference, `g = 1` for gate G7). Symplectic form
`om(x, y) = sum_{i < g} (x[i] y[g+i] - x[g+i] y[i])` mod `l`; its matrix `Om` has
`Om[i][g+i] = 1`, `Om[g+i][i] = -1`.

Uniform `M in Sp_2g(F_l)` (columns `e_1..e_g, f_1..f_g`), built pair by pair:
```
proj(x) = x - sum_{j < k} ( om(x, f_j) e_j - om(x, e_j) f_j )      # onto W^perp
e_k: loop: v = uniform vector (2g draws of uniform(l)); w = proj(v); if w != 0 break
f_k: loop: v = proj(uniform vector); c = om(e_k, v); if c != 0: f_k = v / c; break
```
`G = M * diag(1,..,1, m,..,m)` (columns `f_k` multiplied by `m`). Assert
`G^T Om G == m Om` for every sample (G7).

Characteristic polynomial `det(T I - G)` by the subset expansion (entries of `T I - G`
are polynomials of degree <= 1 over `F_l`): `dp[0] = 1`; for every nonempty column set
`S` (bitmask) in increasing order, with `r = |S| - 1`,
`dp[S] = sum_{j in S} (-1)^{#{i in S : i > j}} * A[r][j] * dp[S \ {j}]`;
the result is `dp[full]`.

Runs (`gsp --samples N`, `N = 200000`):
- `g = 3`: `l in {2, 3, 5, 7}`, every `m in 1..l-1`. Per `(l, m)`: `samples`, `fire`,
  `nz1` (samples with `charpoly(1) != 0 mod l`), `fire_nz1`, `squarefree`, `none`,
  `undetermined`, and G4 agreement count.
- `g = 1` (gate G7): `l in {3, 5, 7}`, every `m`. Per `(l, m)`: `samples`, `fire`,
  `exact = (l - 1 - s) / (2 (l - 1))` with `s = #{a in F_l : a^2 = m}`, and
  `z = (fire/samples - exact) / sqrt(exact (1 - exact) / samples)` (`z = 0` if
  `exact` is 0 and `fire` is 0; if `exact = 0` and `fire > 0`, the gate fails).
  G7 passes iff `|z| <= 4` for every `(l, m)` and every similitude assert held.

## 8. The J[2] table (`lagr2.rs`)

8.1 Representation. Points `0..7`, point 7 is infinity. An element of `J[2]` is an
even-size subset of `{0..7}` modulo complement, stored as the representative with bit 7
clear: a `u8` mask `< 128` with even popcount (64 elements). Addition is XOR. Pairing:
`popcount(a & b) mod 2`.

8.2 Lagrangians. For all nonzero `a < b < c` with pairwise pairing 0 and
`c not in {a, b, a^b}`, the span `{0, a, b, c, a^b, a^c, b^c, a^b^c}` sorted is one
subgroup; deduplicate in a `BTreeSet<[u8; 8]>`. Expect 135.

8.3 Pair type. For nonzero `x` in `L`: `two(x) = x` if `popcount(x) = 2`,
`0xFF ^ x` if `popcount(x) = 6`, else none. `L` is pair type iff four of the `two(x)`
are pairwise disjoint. Expect 105 pair type, 30 other.

8.4 Cycle types. For each of the 15 partitions of 7, the canonical permutation puts
the parts in descending order on consecutive blocks of `0..6` (within a block of length
`d` starting at `b`: `b -> b+1 -> ... -> b+d-1 -> b`) and fixes 7. `sigma(L)` maps
each mask bitwise. Record `stable_pair`, `stable_other` (counts of `L` with
`sigma(L) == L` by type), `pair_rule` (true iff for every odd length, the number of
cycles of that length, counting the fixed point 7 as a cycle of length 1, is even),
`chi2 = chi2_from_cycle_type` = `prod over the cycles of the 8-point permutation of
(T^len + 1)` divided by `(T + 1)^2` over `F_2`, and `fire_2` (section 6 with `l = 2`,
`q = 1`, on `chi2`).
G6 passes iff totals are 135/105/30 and `pair_rule == (stable_pair > 0)` for all 15.
The census looks up `stable_pair`, `stable_other`, `pair_rule` by cycle type.

## 9. Output formats

All JSON keys in snake_case; integers as JSON integers.

`raw/census_p{P}.jsonl`, one line per draw:
- not squarefree: `{"p","i","block","status":"not_squarefree"}`
- P stream, `f` reducible: `{"p","i","block":"P","status":"skipped_reducible","cycle_type"}`
- otherwise: `{"p","i","block","status","f":[c0..c6],"cycle_type","k",
  "n_candidates_J","n_candidates_pair"}` and, when `status = "ok"`, also
  `"s1","s2","s3","nJ","nJtw","prime","c2","gate_weil","gate_annihilation",
  "gate_mod2","l2":{"stable_pair","stable_other","pair_rule"},
  "crit":{"2":{...},"3":{...},"5":{...},"7":{...}}` with the section 6 fields.
- final line: `{"type":"run_summary","p","draws","stop_reason":"prime_target"|"draw_cap",
  "u_count","prime_count","status_counts":{...}}`.

Census loop: block `U` while fewer than `u_count` squarefree draws have been seen
(counting the current one), then block `P`. Stop after the draw at which
`prime_count == prime_target` (counting U and P), or after draw index `draw_cap - 1`.

`raw/gate_enum_p{P}.jsonl`: `{"p","i","N":[N1,N2,N3],"chi_enum":[s1,s2,s3],
"status","chi_census":[s1,s2,s3] or null,"agree": true|false|null}`.

`raw/gsp_reference.json`: `{"samples":N,"g3":[{"l","m","samples","fire","nz1",
"fire_nz1","squarefree","none","undetermined","g4_agree"}...],"g1":[{"l","m","samples",
"fire","exact","z"}...],"similitude_failures":0}`.

`raw/l2_lagrangians.json`: `{"total","pair","other","types":[{"cycle_type",
"stable_pair","stable_other","pair_rule","chi2","fire_2"}...]}`.

## 10. CLI

```
l06 lagr2 --out PATH
l06 gsp --samples N --out PATH
l06 gate-enum --p P --count K --out PATH
l06 census --p P --u-count U --prime-target T --draw-cap C --out PATH
```
Progress lines go to stderr only. Exit code 0 on completion, nonzero on any assert.

## 11. Unit tests (all must pass: gate G0)

1. SplitMix64 seed 0: `0xE220A8397B1DCDAF`, `0x6E789E6AA1B965F4`.
2. `f = x^7 + 1` over `F_5`: squarefree; `N1 = 6`, so `s1 = 0`; DDF degrees `[6, 1]`
   (`x + 1` and `Phi_14`, irreducible because 5 has order 6 mod 14); cycle type
   `[6, 1, 1]`, `k = 2`.
3. `F_7`: `legendre(3) = -1`, `sqrt(2)` is 3 or 4. `nu(5) = 2`, `nu(7) = 3`.
4. Miller-Rabin: `2^31 - 1` prime; `3215031751 = 151 * 21291601` composite (a strong
   pseudoprime to bases 2, 3, 5, 7); 1 not prime; 2 prime.
5. Cantor on `y^2 = x^7 + 1` over `F_5`: `add((x, 1), (x, 4)) = identity`;
   `double((x + 1, 0)) = identity` (the point `(4, 0)`); `add(D, neg(D)) = identity` for
   20 random classes; associativity and `mul(a+b, D) = add(mul(a, D), mul(b, D))` for
   20 random triples on a random curve over `F_101`.
6. Enumeration vs Cantor: for 20 random squarefree curves each over `F_11` and `F_13`,
   `chi_enum(1)` annihilates 5 random classes and `chi_enum(-1)` annihilates 5 random
   twist classes. For 5 curves over `F_101`, census `chi == chi_enum` (or the census
   reports `ambiguous`, which fails the test only if all 5 are ambiguous).
7. `chi mod 2` for `x^7 + 1` over `F_5` from enumeration equals
   `chi2_from_cycle_type([6,1,1]) = T^6 + 1`.
8. Criterion (section 6), constructing `chi` as the product of the listed factors and
   asserting `c[0] = q^3`:
   - `l = 5, q = 1`: `(T-2)^3 (T-3)^3`: fire, `n_h = 2`, class fire.
   - `l = 5, q = 1`: `(T-2)^2 (T-3)^2 (T-1)^2`: no fire, class undetermined.
   - `l = 5, q = 2`: `(T-1)^2 (T-2)^2 (T-3) (T-4)`: fire, `n_h = 4`.
   - `l = 2, q = 1`: `T^6 + T^5 + ... + 1`: fire, `n_h = 2`, `ext_deg = 7`.
   - `l = 3, q = 1`: `(T-1)^6`: no fire, undetermined.
   - `l = 3, q = 2`: `(T-1)^3 (T-2)^3`: fire, `n_h = 2`, `ext_deg = 3`.
   - `l = 7, q = 1`: `(T^2 + 1)(T-2)(T-4)(T-3)(T-5)`: no fire, class none.
   - `l = 7, q = 3`: `(T-1)(T-2)(T-3)(T-4)(T-5)(T-6)`: fire, `n_h = 8`, class fire.
   - `h^*`: `l = 5, q = 2`, `h = (T-1)^2 (T-3)` gives `h^* = (T-2)^2 (T-4)`.
9. Charpoly: identity (6x6) gives `(T-1)^6`; the companion matrix of the `l = 7, q = 3`
   polynomial gives that polynomial; `[[a, b], [c, d]]` gives
   `T^2 - (a+d) T + (ad - bc)` for 10 random 2x2 matrices over `F_7`.
10. Sampler: 1,000 samples for `(g = 3, l = 5, m = 2)`: all pass the similitude check
    and have `charpoly(0) = m^3 = 3`.
11. Lagrangian table: 135 / 105 / 30. Cycle type `[7, 1]`: `stable_pair = 0`,
    `stable_other = 2` (the two cubic submodules of `F_2[x]/(Phi_7)`), `pair_rule`
    false, `fire_2` true. Identity type `[1^8]`: `stable_pair = 105`,
    `stable_other = 30`.

## 12. analyze.py

`python3 -I analyze.py raw` reads every file of section 9 plus `raw/test.log` and
writes one JSON object to stdout:
- `gates`: G0 (pass iff `test.log` contains `test result: ok.` and no line containing
  `FAILED` or `panicked`), G1-G8 exactly as PROTOCOL.md section 5, each with counts.
- `populations` per `p` and pooled: ALL, PRIME, C2, ODD sizes (PROTOCOL.md section 2;
  ALL, C2 and ODD use only `block = "U"`, PRIME uses all `ok` draws).
- per `l in {3,5,7}`, per population, per `p` and pooled: `fire` count, fraction,
  Wilson 95% interval, `class` counts, `n_h` and `ext_deg` histograms on firing PRIME
  curves.
- A2: on ALL and ODD, counts of `fire_2`, `pair_rule`, `stable_other >= 2`.
- S4: for each `(p, l)`, `z` against the `g3` reference row with `m = p mod l`
  (ALL against `fire/samples`, PRIME against `fire_nz1/nz1`).
- `verdict`: computed by the PROTOCOL.md section 6 rule, one of `"void (instrument)"`,
  `"inconclusive (sample)"`, `"killed"`, `"supported"`, `"inconclusive"`.

Wilson 95% (`z = 1.959964`): for `k` of `n`, `ph = k/n`,
`center = (ph + z^2/(2n)) / (1 + z^2/n)`,
`half = z sqrt(ph(1-ph)/n + z^2/(4n^2)) / (1 + z^2/n)`, interval `[center - half,
center + half]`.
