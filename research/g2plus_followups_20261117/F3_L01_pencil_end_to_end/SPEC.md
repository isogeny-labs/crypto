# F3 — Implementation spec (frozen, 2026-11-17)

Precise enough that a less capable model makes **no research choice**. All names,
formulas, counter placements, output schemas and unit tests are fixed here. Where a
value is a free implementation detail (e.g. a hash constant) it is given explicitly.
Status `[pcc:unverified]`. The decision instrument is a **standalone Rust crate** in
`rust/` (own empty `[workspace]`, `.gitignore` = `target/`); the pilot and analysis
are Python 3 standard library only (`-I`). No numpy/sympy/sage/pari.

The genus-2 arithmetic, #J, pencil sieve, walk engine and dense mod-r solver are a
re-implementation of the frozen L01 package copied into `pilot/l01/` (provenance:
`pilot/COPIED_FROM_L01.sha256`). The Rust port must reproduce the L01 Python counts
on the shared instances (unit test UT12).

## 0. Files

```
rust/Cargo.toml          # [workspace] empty table; [package] name="f3"; edition 2021; no deps
rust/.gitignore          # contains: target/
rust/src/field.rs        # F_p and F_r arithmetic with counters
rust/src/poly.rs         # F_p[x] polynomials (schoolbook; counts per L01 curve.py)
rust/src/jac.rs          # Mumford divisors, Cantor add/double/reduce, mul, negate, key
rust/src/order.rs        # N1 scan, cheap #J (a1 + a2 BSGS), admission, G/T planting
rust/src/pencil.rs       # y-table, anchored pencil sieve, descent split-walk  (arm P)
rust/src/walk.rs         # Gaudry walk, prefilter, roots, relations           (arm W, W2)
rust/src/rho.rs          # negation-map rho with cycle escape                 (arm R)
rust/src/linalg.rs       # counted sparse Gaussian/Markowitz + dense core, mod r
rust/src/main.rs         # `f3 run --p --curve-index --seed --arms --la-prices --out`
analyze.py               # medians, paired bootstrap, exponent fits, decision rule
pilot/pilot.py           # already run; see PILOT.md (gate reachability + calibration)
pilot/l01/*.py           # copied L01 package used by the pilot
```

All JSONL records are one object per line, keys sorted, flushed. Each `run`
invocation writes one file with one object per (arm) actually run (`P`, `W`, `R`,
and `W2` if enabled), schema `F3.run/v1` (§9).

## 1. Field and counters (`field.rs`)

Two moduli: the prime field `F_p` (relation phase, Cantor, descent) and `F_r`
(the mod-r linear solve), `r` the group order. Counters are plain `u64`:
`mul`, `inv` for `F_p` work; `la_mul`, `la_inv` for `F_r` work; `add` untracked by
`Cost_w` but tallied. Rules, identical to L01 `field.py`/`curve.py`:
- `Fp::mul(a,b)` → `a*b % p`, `mul += 1`. `Fp::sqr` counts as one `mul`.
- `Fp::inv(a)` → `a^(p-2) mod p` by square-and-multiply, `inv += 1` (the whole
  inversion is ONE counted inversion, not its internal mults — matches L01, which
  favours the baseline).
- Polynomial `k_mul(a,b)` adds `len(a)*len(b)` to `mul` (schoolbook, L01 `curve.k_mul`).
  `k_scale` adds `len(a)`. `k_divrem` adds `db` per quotient term plus 1 per
  non-monic leading scale. `horner(c,x)` adds `len(c)-1`. `batch_inv(vals)` of
  length `k`: `3(k-1)` muls + 1 inv (L01 `field.batch_inv`).
- `Fr::mul` → `la_mul += 1`; `Fr::inv` → `la_inv += 1` (whole inversion one count).
`Cost_w` base = `mul + 10*inv` summed over phases; `la` term = `la_mul`
(priced at `w`) and `la_inv` folded into base as `10*la_inv` (inversions are always
`I=10`, independent of `w`). Report `la_inv` separately so this is auditable.

## 2. Divisors, Cantor, order (`jac.rs`, `order.rs`)

### 2.1 Mumford representation
Reduced divisor `(u, v)`, `u` monic `deg ≤ 2`, `deg v < deg u`, `v² ≡ f (mod u)`.
`IDENT = ((1), ())`. Cantor add/double/reduce, `negate`, `mul_ltr`
(left-to-right square-and-multiply), `random_divisor` and `key_of` are a byte-for-
counter port of L01 `curve.py` (UT12). `key_of((u,v))` =
`mix64` chain over `0x243F6A8885A308D3 ^ deg` and `(u[i]<<32)|v[i]`, exactly L01.

### 2.2 N1 and a1
`N1(f,p) = 1 + Σ_{x=0}^{p-1} (1 + chi(f(x)))` with `chi` from the square table
(`sqtab`): `O(p)`, one `horner` (4 muls) + one table lookup per `x`. `a1 = p+1−N1`.

### 2.3 Cheap #J by a2-BSGS (correction to the brief)
`base = p*p + 1 − a1*(p+1)`; `#J = base + a2`. With `S = a1/(2√p)` (f64),
`a2 ∈ [a2lo, a2hi]`, `a2lo = floor(2p(2|S|−1)) − 4`, `a2hi = ceil(2p + pS²) + 4`.
Derivation (fixed, for the reader): the Frobenius L-poly is
`L(T)=∏_{j=1,2}(T² − 2√p cosθ_j T + p)`, so `a1 = 2√p(c1+c2)`,
`a2 = 2p + 4p·c1c2`, `c_j = cosθ_j ∈ [−1,1]`. For fixed `S = c1+c2`, `c1c2 ∈
[|S|−1, S²/4]`, hence `a2 ∈ [2p(2|S|−1), 2p+pS²]`, width `p(|S|−2)² ≤ 4p`. Assert
`a2hi − a2lo < 5p + 50`.
BSGS for `#J = base + a2` with `[#J]·D = IDENT` on a random `D`: let `lo = base +
a2lo` (shift up to ≥ 1 if needed), `W = a2hi − a2lo`, `m = isqrt(W)+1`. Baby table
`{ key_of([j]D) → j : j = 0..m−1 }`. `stepD = [m]D`; `target = −[lo]D`; for
`i = 0..m+1`: if `key_of(target) ∈ baby` at `j`, candidate `M = lo + i*m + j`, accept
iff `[M]D == IDENT` (guards hash collisions); else `target += −stepD`. `#J = M`,
`a2 = M − base`. **Verify** (gate G-order): `#J` inside `[(isqrt(p)−1)⁴,
(isqrt(p)+1)⁴]` (±4p slack) and `[#J]·D' = IDENT` for `confirm = 8` fresh random
`D'`; else the curve candidate is rejected. Cross-checked at `p ≤ 101` against the
`O(p²)` F_{p²} formula `jac_order(point_counts(...))` (UT04). The BSGS group
operations ARE charged (instance set-up, §4).

### 2.4 Admission, G, T, t
`random_f(p,2,s)` = L01 rule (`st = (s*GOLD)^p^(2<<48)`; 5 coeffs via `mix64`; monic).
Admit `s` if squarefree and `#J = h·r`, `r` prime (trial division to `√r` — r ≤ 2^32
so ≤ 2^16 trial divisors), `h ≤ 8`, `r > h`. Curve index `k` = the k-th admitted `s`,
`k = 1,2,3`, ≤ 600 seeds tried. `G = [h]·D0` for the first random `D0` with `[h]D0 ≠
IDENT` (then `[r]G = IDENT`, asserted). `t = 1 + (stream below (r−1))`, `T = [t]G`.
All of §2.3–2.4 is the **instance+order set-up**, charged identically to P, W, R.

## 3. RNG and seeding (`field.rs`)

### 3.1 SplitMix64 (L01 `rng.py`)
`GOLD = 0x9E3779B97F4A7C15`; `mix64(z)`: `z=(z^(z>>30))*0xBF58476D1CE4E5B9;
z=(z^(z>>27))*0x94D049BB133111EB; z^(z>>31)`, all mod 2^64. Stream state advances
`st += GOLD`; `next() = mix64(st)`; `below(n) = next() % n`.

### 3.2 Stream keys
`stream(tag, 2, p, curve_seed, σ)` seeds `st` from `mix64(tag ⊕ rot(p) ⊕ …)` exactly
as L01 `rng.stream`. Tags (hex): P family `0x50`, W `0x57`, R `0x52`, descent `0x44`,
curve/order `0x4F`. P, W, R therefore share the curve and `(G,T,t)` but draw
independent anchors/walks.

## 4. Phase charging (which counter each phase ticks)

| phase | counter bucket | arms |
|---|---|---|
| curve search + cheap #J + r + G + T planting (§2.3–2.4) | `setup_mul/inv` | P, W, R (identical) |
| y-table: `sqtab` (`(p−1)/2` sqr), `fx` (`p` horner), base list | `arm_setup_mul/inv` | P |
| base list + `chi`/`sqrt`/root tables | `arm_setup_mul/inv` | W (and W2) |
| rho 32-step table build | `arm_setup_mul/inv` | R |
| pencil sieve relations | `rel_mul/inv` | P |
| walk relations | `rel_mul/inv` | W, W2 |
| descent split-walk of G and T | `descent_mul/inv` | P (W folds descent into `rel`) |
| rho walk (all Cantor ops) | `rel_mul/inv` | R |
| sparse solve mod r | `la_mul`, `la_inv` | P, W, W2 |

`base_fpm = setup + arm_setup + rel + descent` muls `+ 10×` their invs `+ 10×la_inv`.
`Cost_w = base_fpm + w·la_mul`. Rho: `la_mul = la_inv = 0`.

## 5. Factor base, columns, dedup

### 5.1 Base
Columns = abscissae `x` with `chi(f(x)) = +1` (two rational points `(x,±y)`),
ordered by increasing `x`; `col[x]` = index, `n = |base|`. L01 `Tables`.
Base element `D_j = [ (x−x_j), (y_j) ]`, `y_j = sqtab[f(x_j)]` (the root in
`1..(p−1)/2`). Its log unknown `x_j = log_G D_j` (mod r).

### 5.2 Sign convention
A point `(x,y)` contributes `col[x]` with coefficient `+1` if `1 ≤ y ≤ (p−1)/2`,
else `−1` (L01 gate convention). `[(x,−y)] = −[(x,y)]` in J.

### 5.3 Dedup key (G5)
A relation's key = the sorted tuple `((x, s) …)` of its non-2-torsion points with
`s = +1/−1` per §5.2, canonicalised by also forming the all-signs-flipped tuple and
taking the lexicographic min (identify a relation with its negation). A relation
whose key is already seen is a duplicate: counted in `duplicates`, never added to
the matrix or any denominator.

## 6. Arm P — pencil sieve + descent (`pencil.rs`)

### 6.1 y-table (arm set-up)
`sqtab`, `chi`, `fx`, base as §5.1; counts to `arm_setup`.

### 6.2 Anchored sieve (L01 `p_full`)
Seeded permutation of the base (`stream(0x50,…)`). Anchor tuple = next 2 unused base
abscissae `(x1,x2)`; `a0` = line through `(x1,y1),(x2,y2)` (L01 interpolation, counted);
`Π=(x−x1)(x−x2)`. Forward-difference values of `a0,Π` over `x=0..p−1` (adds only,
`fd_add`). One batch inversion of the `Π(x)≠0` values per tuple. For each base point
`(x,±y)` (x not an anchor abscissa, `Π(x)≠0`): `λ = (±y − a0(x))·Π(x)^{-1}`
(1 counted mul per point using the batched inverse); bucket by `λ`. Every `λ` with
exactly `3` hits (a full split fibre, all nonzero-y, distinct x) is a weight-5
homogeneous relation `Σ_{k} s_k x_{col[x_k]} ≡ 0 (mod r)` (the two anchors always
among the 3? no — the fibre is the 3 roots of the residual `R_λ` of degree 3; the
two anchors are the common zeros of every member and are NOT in the residual). Add
via the dedup rule (§5.3). **Counter-formula gate** (G1/UT08): per tuple
`mul = 10 + 8 + 3*(nX−1) + npts`, `inv = 2`, `nX` = abscissae carrying points (L01).

### 6.3 Dense anchor columns
The two anchors `x1,x2` appear in every residual-generating relation of their tuple
only through the point→λ map, not as residual roots; the **dense columns** are the
base columns that recur across a tuple's ~`p/6` relations. Record `max_col_wt` and
the number of columns of weight `> √n` (the dense set); the solver (§7) counts their
elimination fully — this is the measured pencil penalty (L01 H4).

### 6.4 Descent of G and T (the charged phase L01 omitted)
Gaudry split-walk, counts to `descent`. Draw `(α,β)` from `stream(0x44,…)`,
`α ∈ [1,r−1]`, `β ∈ [0,r−1]`; `R = reduce([α]G + [β]T)` (β=0 → `[α]G`). Accept iff
`deg u_R = 2` and `u_R` splits into 2 distinct roots, both base abscissae (covered
columns), both nonzero y (split test via `disc_character`+`roots_of`, §7.W). The
accepted relation: points `P_a,P_b`, row `= s_a·x_{col[a]} + s_b·x_{col[b]}`, and
the inhomogeneous equation `row − (−hβ)·t ≡ hα (mod r)`. Collect exactly
`D_DESC = 4` such (2 pure-G with β=0 to pin the scale, 2 mixed) — more are harmless
but 4 suffice generically; if after `200000` draws fewer than 4 accept, the instance
fails (reported). (PILOT confirms this descent recovers `t` at p=131,257 via the L01
gate, which uses 3+3.)

### 6.5 Stop rule = rank condition (G4, lesson b)
Process tuples until the number of **distinct** pencil relations reaches
`target = covered + 8 + ⌊covered/100⌋` where `covered` = base columns hit so far.
Then build the homogeneous system over the covered columns and run the solver (§7);
require **corank exactly 1**. If corank > 1, process one more tuple and re-solve
(charged), up to `+5` extra tuples; still > 1 → instance fails (reported). On corank
1, append the 4 descent equations and solve the inhomogeneous system for `t`; accept
`t` only if its column is uniquely determined (G4) and `[t]G = T` (G2).

## 7. Arm W — Gaudry walk (`walk.rs`) and the shared solver (`linalg.rs`)

### 7.W Walk relations (L01 `b_run`/`b_labelled`)
16 jump divisors `M_j` and two restart bases from `stream(0x57,…)`. State `cur`,
coefficients `(ca,cb)` with `cur = [ca]G + [cb]T`. Restart every
`min(4096, max(16, ⌊√r/2⌋))` steps (lab amendment 1) by `cur = [α]G+[β]T`,
`(ca,cb)=(α,β)`; else `j = (mix64(key_of(cur))>>24)%16`, `cur += M_j`,
`(ca,cb) += (a_j,b_j)`. Candidate iff `deg u = 2`; `disc_character` parity prefilter
(counts to `rel`); `roots_of` = Cantor–Zassenhaus (L01 `native.roots_of`, counted);
`v` at roots (horner, counted). Relation `Σ s_k x_{col[x_k]} − (−h·cb)t ≡ h·ca`,
dedup §5.3. Stop rule = rank condition (G4): collect
`target = n + 8 + ⌊n/100⌋` distinct relations, require the inhomogeneous system to
determine `t` uniquely; if not, collect `+⌈n/20⌉` more (charged), up to 3 rounds.

### 7.W2 Double large prime (arm W2, only if sound in budget)
Base `S` = first `nb = ⌈2 p^{1/2}⌉` abscissae; ≤ 2 off-base points per relation;
large-prime graph with union-find; stop at `nb + 8 + ⌊nb/100⌋` cycles (L01 `p_2lp`
baseline `b_run(lp_mode)`). Each cycle is reconstructed to a full-base relation and
MUST enter the solve; `t` recovered and verified (G2) or W2 is dropped for that cell
(recorded). W2 is reported alongside W; the **primary P/W uses full-base W**.

### 7.S Counted solver mod r (identical for P, W, W2)
Input: rows as sparse maps `col→coef (mod r)`, an optional `t`-column index, RHS per
row. Structured Gaussian elimination with **Markowitz** pivoting:
1. Among remaining columns pick the pivot column `c` of least live-row count (ties:
   least column index); pivot row = the live row containing `c` of least weight
   (ties: least row id). `la_inv += 1` for the pivot inverse `ip = pivot[c]^{-1}`.
2. For each other live row `k` with `c`: `fac = row_k[c]·ip (la_mul += 1)`; for each
   `(j,val)` in the pivot row except `c`: `row_k[j] −= fac·val (la_mul += 1)`,
   inserting/removing entries. This is the L01 `linalg.sparse_eliminate` loop, but
   mod `r` and **retaining the reduced rows** for back-substitution.
3. Fall to a **dense** Gauss–Jordan mod r (L01 `solve_mod_r`, counting each `*%r` as
   `la_mul`, each inverse as `la_inv`) on the residual core once the active matrix
   has `< CORE = 256` columns or density `> 0.5`.
Output: `consistent`, the set of uniquely-determined columns and their values
(including `t` if present), `pivots`, `la_mul`, `la_inv`, `initial_nnz`, `peak_nnz`,
`max_col_wt`. Homogeneous corank = `(covered − pivots)` on the pencil system (G4).
The solver recovers the determined base logs and `t`; the run then verifies
`[t_recovered]G = T` (G2). Op cap: if `la_mul` exceeds `LA_CAP = 3e9`, return
`status="exhausted"`.

## 8. Arm R — negation-map rho (`rho.rs`)

Exact port of the PILOT's `rho_solve` (validated, PILOT.md). 32 additive steps
`M_k=[c_k]G+[d_k]T` from `stream(0x52,…)`. `rep(X,a,b)` = `(−X,−a,−b)` if
`key_of(−X) < key_of(X)` else `(X,a,b)`. Step: `k=(mix64(key_of(X))>>20)%32`,
`Xn=rep(X+M_k, a+c_k, b+d_k)`. **Fruitless cycle:** keep a rolling window of the last
`W_CYC = 64` keys→(idx,X,a,b); if `key_of(Xn)` is in the window, take the cycle
(entries with idx ≥ first occurrence, plus Xn), find its minimal-key point `e`, set
`Xn = rep([2]e.X, 2e.a, 2e.b)`, clear the window (deterministic, so colliding trails
escape identically). Distinguished point: `key_of(X) & ((1<<dp_bits)−1) == 0`,
`dp_bits = max(1, round(0.5·log2(√(πr/2))))` (so ~√r/const DPs before collision; at
r≈2^23 this is 8, PILOT value). Store `key→(a,b)`; on a second DP equal to a stored
one with `b≠b'`, `t=(a'−a)(b−b')^{-1} mod r`; verify `[t]G=T`. Hard cap
`60·√(πr/2)` steps per attempt; on cap, **retry** with σ-derived fresh seed (`K=4`
retries, each charged); all fail → instance failure. Cost = `rel_mul + 10·rel_inv`
over every Cantor op (group ops); `la_mul = 0`.

## 9. Output schema `F3.run/v1` (one object per arm)

```
{ "schema":"F3.run/v1", "arm":"P|W|W2|R", "p":int, "curve_index":int, "seed":int,
  "curve_seed":int, "f":[6 ints], "hJ":int, "r":int, "h":int, "n_cols":int,
  "covered":int, "t_planted":int, "t_recovered":int|null, "verify":bool,
  "relations":int, "duplicates":int, "hom_corank":int|null,
  "setup_mul":int,"setup_inv":int, "arm_setup_mul":int,"arm_setup_inv":int,
  "rel_mul":int,"rel_inv":int, "descent_mul":int,"descent_inv":int,
  "la_mul":int,"la_inv":int, "base_fpm":int,
  "row_wt_mean":float,"max_col_wt":int,"dense_cols":int,
  "peak_nnz":int,"initial_nnz":int,"pivots":int,
  "retries":int, "rho_steps":int|null,"rho_dps":int|null,"rho_cycle_escapes":int|null,
  "rows_digest":"sha256-hex", "status":"ok|exhausted|fail", "cpu_seconds_diag":float }
```
`base_fpm = (setup_mul+arm_setup_mul+rel_mul+descent_mul) + 10*(setup_inv+
arm_setup_inv+rel_inv+descent_inv+la_inv)`. `Cost_w = base_fpm + w*la_mul`. Keep each
`.jsonl` under 50 KB (≤ 4 arm rows per run; no per-relation rows).

## 10. Analysis (`analyze.py`, fixed)

### 10.1 Per pair
For each `(p, curve, σ)` and `w ∈ {0,0.1,1}`: `Cost_w(arm)`, then `P/W`, `P/R`,
`W/R` (and `P/W2`, `W2/R` if W2 present). A pair is **valid** only if P, W, R all
have `status="ok"` and `verify=true`.

### 10.2 Cell statistic
Per (p, w): **median** of the ratio over the 9 valid pairs; `min`, `max`.

### 10.3 Paired bootstrap 95% interval
Resample with replacement: first the 3 curves, then within each the 3 σ (hierarchical),
10000 resamples, fixed seed `0xF3`, percentile 2.5/97.5 of the median `P/W`. Report
`[L_w,U_w]`. (Same for `P/R`, `W/R`, reported only.)

### 10.4 Exponent fit
OLS of `log Cost_{0.1}(arm)` on `log p` over primes where the arm is `ok` on ≥ 2 of 3
curves (median over pairs per prime); 95% interval by the §10.3 bootstrap over the
per-prime medians.

### 10.5 Decision (print to `summary.md`, verbatim rule from PROTOCOL §9)
Determine `p*`; emit `supported` / `killed` / `inconclusive` with the deciding
`M_w,[L_w,U_w]` for each `w`, the per-prime P/W, P/R, W/R tables, the exponent fits,
and the gate roster (G1–G9 pass/fail). No threshold is computed from the data.

## 11. Unit tests (`cargo test`, each with an expected value; G1)

- **UT01 mix64**: `mix64(0)=0`; `mix64(0x9E3779B97F4A7C15)` equals the L01 Python
  value (compute once in the port; fixture pinned in the test).
- **UT02 random_f**: `random_f(13,2,1)` equals L01 `random_f(13,2,1)` (6 coeffs).
- **UT03 Cantor**: on `y²=f`, `D + IDENT = D`; `D + (−D) = IDENT`; `[r]G = IDENT`;
  group law vs a brute divisor table at `p=13` (all pairs).
- **UT04 #J**: cheap `#J` equals `jac_order(point_counts(f,p,2))` for all admitted
  curves at `p ∈ {13,17,19,23,29,37,53,101}` (PILOT showed 0 mismatches); and
  `[#J]D=IDENT` on 8 random D.
- **UT05 pencil p=13**: the fixed example `f=random_f(13,2,1)`, anchors = first two
  base abscissae: the sieve's relation set equals the naive full-λ scan's set
  (positive control, ≥ 5× cost ratio).
- **UT06 batch_inv**: length-5 vector costs exactly `12 mul + 1 inv`; values correct.
- **UT07 horner**: `len(c)-1` muls; value matches direct evaluation.
- **UT08 pencil counter formula**: for one tuple at `p=251`, `mul = 10+8+3*(nX−1)+
  npts`, `inv = 2` (assert on the live tally).
- **UT09 disc_character / roots_of**: on 50 random degree-2 `u` at `p=101`,
  `disc_character==1` iff `u` has 2 distinct roots with `chi(root-point)` a square,
  matching brute factorisation; `roots_of` returns them.
- **UT10 mod-r counter**: the solver on a hand 3×3 system mod a small prime `r=101`
  returns the known solution and `la_mul`/`la_inv` equal the hand count (pin the
  exact integers in the test).
- **UT11 rho tiny**: at `p=131` (admitted), rho recovers planted `t` and `[t]G=T`
  (PILOT: steps ~40).
- **UT12 parity with L01**: for `(p,curve,σ)=(251,1,1)`, the Rust `rel_mul`, `rel_inv`,
  `relations`, `duplicates`, `rows_digest` of arm W equal the L01 Python
  `b_run`/`Acc` values (the port reproduces the frozen counts).

## 12. Determinism and budget

All randomness from the §3 streams; no wall-clock, PID or thread-id entropy. 1 thread
per run. `cpu_seconds_diag` recorded only as a guard (not a metric). Per-run hard caps:
`LA_CAP = 3e9` `la_mul`; `REL_CAP = 2e9` `base_fpm`; `MEM` guard 2 GB (if the sparse
matrix `nnz > 1.2e8`, status `exhausted`). An exhausted/failed run is a reported
missing cell, never rerun with other parameters.
