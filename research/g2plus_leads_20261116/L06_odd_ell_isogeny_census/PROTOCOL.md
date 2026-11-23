# L06 protocol: odd-l rational Lagrangian census on genus-3 hyperelliptic Jacobians

Frozen before any data, 2026-11-16, by the L06 rerun coordinator. Hashes of this file,
DERIVATION.md and SPEC.md are in `frozen.sha256`. Standing: everything `[pcc:unverified]`.
Not a benchmark: no leaderboard row, no `benchmarks/` row, no speed claim, no wall-time
metric. All metrics are exact counts and fractions with recorded seeds.

## 1. Hypothesis

H1. For odd `l in {3, 5, 7}`, a non-negligible share of genus-3 hyperelliptic Jacobians
`J` over `F_p` with `#J(F_p)` prime have `chi mod l = h h^*` with `deg h = 3` and
`gcd(h, h^*) = 1`. By DERIVATION.md section 1, each such curve has an `F_p`-rational
Lagrangian `ker h(pi)` in `J[l]`, the kernel of an `F_p`-rational `(l,l,l)`-isogeny.

Main-session kill criterion (adopted unchanged): H1 is killed if fewer than 1% of
prime-order curves fire, for every `l in {3, 5, 7}`.

What H1 does not claim: that the isogeny is computable at useful cost, or that its
codomain is a plane-quartic Jacobian. The codomain is not computed (DERIVATION.md
section 6).

## 2. Exact quantity counted

For a curve with exactly determined `chi` and a prime `l`:

- `fire_l` = 1 iff there is a monic cubic `h in F_l[T]` with `h * h^* == chi mod l` and
  `gcd(h, h^*) == 1`, where `h^*(T) = T^3 h(p/T) / h(0)` mod `l`. Computed by brute force
  over all `l^3` monic cubics (SPEC.md section 6.3). The factor-based test ("no
  self-reciprocal irreducible factor") must agree on every curve (gate G4).
- `class_l` in {`fire`, `none`, `undetermined`}: `fire` if `fire_l`; `none` if not firing
  and `chi mod l` squarefree; `undetermined` otherwise (DERIVATION.md section 2, Claim B).
- `n_h_l` = number of valid `h` (= 2^pairs). `ext_deg_l` = minimum, over valid `h`, of the
  multiplicative order of `T` in `F_l[T]/(h)` (DERIVATION.md section 6, step 1).

Populations, per `p` and pooled over `p`:
- ALL: every squarefree draw in the U block (section 3).
- PRIME: every squarefree draw in the whole scanned seed range with `#J` prime.
- C2: every squarefree draw in the U block with `#J = q` or `#J = 2q`, `q` prime
  (secondary; reported because the brief asks for it).
- ODD: draws in the U block with `#J` odd (for the `l = 2` arm).

Primary metric: `F_l = (#PRIME with fire_l) / #PRIME`, pooled over the five primes,
with a Wilson 95% interval.

## 3. Instances

- Primes `p in {257, 521, 1031, 2053, 4099}` (about `2^8` to `2^12`). Residues, for the
  multiplier coset: `p mod 3 = 2, 2, 2, 1, 1`; `p mod 5 = 2, 1, 1, 3, 4`;
  `p mod 7 = 5, 3, 2, 2, 4`.
- Curve rule: draw index `i = 0, 1, 2, ...`. `f = x^7 + c6 x^6 + ... + c0`, each `c_j`
  uniform in `F_p` from SplitMix64 seeded with
  `curve_seed(p, i) = 0x4C06_0000_0000_0000 ^ (p << 24) ^ i`, drawn in order
  `c0, c1, ..., c6` (SPEC.md section 2). Draws with `gcd(f, f') != 1` are recorded as
  `not_squarefree` and skipped.
- U block: draw indices from 0 until `U_p` squarefree draws have been seen, with
  `U_p = 3000` for `p in {257, 521, 1031}` and `U_p = 1000` for `p in {2053, 4099}`
  (the order search costs about `11 p` Cantor operations per curve, SPEC.md 4.3, so the
  large primes get a smaller unconditional block). Each gets full `chi`.
- P stream: continue with the next indices. Compute `chi` only for draws with `f`
  irreducible over `F_p` (all others are recorded with `chi_computed = false`). Stop when
  the PRIME population for this `p` (U block plus P stream) reaches 120, or when the
  total draw index reaches the cap 40,000. A draw with `#J` prime has `f` irreducible
  (DERIVATION.md section 4: `#J` odd iff `f` irreducible), so skipping reducible `f` in
  the P stream misses no prime-order curve. PRIME is therefore every prime-order curve
  among draws `0 .. stop-1`, an unbiased sample of the conditional distribution.
- Expected sizes (not thresholds): about 1-2% of draws have prime `#J`, so roughly 120
  PRIME per `p` and 600 pooled.
- Pre-specified fallback (the only one allowed): if a census command hits `timeout 600`,
  rerun that `p` once with half its `--u-count` and `--prime-target 60`, and record
  the timeout message verbatim in DEVIATIONS.md. No other parameter change is allowed.

## 4. Arms

- A3, A5, A7: the census criterion at `l = 3, 5, 7` on ALL, PRIME and C2.
- A2 (corrected `l = 2` arm; DERIVATION.md section 4): on ALL and ODD, report
  (a) `fire_2`; (b) pair-type stable kernel exists (Frobenius cycle type on the 8
  Weierstrass points admits a stable pair partition); (c) number of stable Lagrangians
  among the 135, by type (pair / other), from the explicit table. Predictions frozen
  here, from the derivation: on ODD, (a) = 100%, (b) = 0%, (c) other >= 2 on every
  curve. Any miss means the derivation or the instrument is wrong, and it is reported
  as such.
- R (reference): for each `l in {2, 3, 5, 7}` and each multiplier `m in F_l^*`, 200,000
  uniform samples of the coset `{G in GSp_6(F_l) : multiplier m}` (uniform symplectic
  basis times `diag(1,1,1,m,m,m)`, SPEC.md section 7). Reported: the share firing
  overall, and the share firing among samples with `charpoly(1) != 0 mod l`. This is an
  estimate by sampling, not an enumeration. The odd-`l` reference for prime `p` uses
  `m = p mod l`. For `l = 2` the GSp reference is reported but is not the right null:
  hyperelliptic mod-2 monodromy is `S_8`, not `Sp_6(F_2)`. The A2 comparison uses the
  cycle-type analysis instead.

## 5. Correctness gates (all must pass before any metric is read)

- G0: `cargo test --release` passes, including every unit test listed in SPEC.md section 9.
- G1 (two chi methods agree): `gate-enum` computes `chi` from exact point counts
  `N_1, N_2, N_3` by enumerating `F_p, F_{p^2}, F_{p^3}`, and compares it with the census
  method (`N_1` plus group orders of `J` and its twist). Instances: `p = 101`, the first
  200 squarefree draws; `p = 257`, the first 40 squarefree draws (same `curve_seed`).
  Required: every draw where the census method returns a unique `chi` agrees with the
  enumeration (100%), and at most 2% of the 240 draws return `ambiguous`.
- G2 (annihilation): on every curve with `chi` computed, 3 fresh random divisor classes
  `D` on `J` satisfy `chi(1) D = 0`, and 3 on the twist satisfy `chi(-1) D' = 0`, with
  a seed stream disjoint from the one used to find the orders. Required: 100%.
- G3 (Weil): every computed `chi` satisfies `s1^2 <= 36p`, `|s2| <= 15p`,
  `s3^2 <= 400 p^3`. Required: 100%.
- G4 (two criterion tests agree): brute-force `fire_l` equals "no self-reciprocal
  irreducible factor" for every curve, every `l in {2,3,5,7}`, and every reference
  sample. Required: 100%.
- G5 (mod-2 structure): for every curve with `chi`, `chi mod 2` equals
  `prod over cycles (T^len - 1) / (T - 1)^2` over `F_2` for the cycle type of `f`'s
  factorisation degrees plus a fixed point; `v_2(#J) >= k - 1`; and `#J` odd iff `k = 1`.
  Required: 100%.
- G6: the `J[2]` Lagrangian table has 135 entries, 105 pair type and 30 other.
- G7 (reference sampler): every sample satisfies `G^T Om G = m Om` exactly. A genus-1
  run of the same sampler (`GL_2`, determinant `m`, 200,000 samples per `(l, m)` for
  `l in {3,5,7}`) matches `(l - 1 - s) / (2(l - 1))` within 4 binomial standard errors
  for every `(l, m)` (DERIVATION.md section 3).
- G8 (order determination): at each `p`, at most 2% of `chi`-requested draws end with
  `status = ambiguous`. Ambiguous draws are excluded and counted. They cannot be prime
  order: for every census `p`, `(sqrt(p) - 1)^6 > 2B + 1` (SPEC.md 4.3), so a nonzero
  class of prime order `#J` has exactly one multiple of its order in the search
  interval.

If any of G0-G7 fails: verdict "void (instrument)", no metric is quoted as a result. If
G8 fails at some `p`: that `p` is dropped from the pooled numbers. If the remaining
PRIME pool is below 300, the verdict is "inconclusive".

## 6. Decision rule (frozen; applied to the pooled PRIME population)

Let `n = #PRIME` pooled and `F_l` as in section 2, with Wilson 95% interval `[lo_l, hi_l]`.

1. Any of G0-G7 failed: **void (instrument)**.
2. `n < 300`: **inconclusive (sample)**.
3. `F_l < 0.01` for every `l in {3, 5, 7}`: **killed**.
4. `lo_l >= 0.01` for at least one `l in {3, 5, 7}`: **supported**.
5. Otherwise: **inconclusive**.

"Supported" means rational odd-`l` Lagrangians exist on prime-order Jacobians at the
measured rate. It does not mean the Smith route reopens (DERIVATION.md section 6).

Secondary readings (reported, never change the verdict):
- S1-S3: the A2 predictions in section 4.
- S4: for each `(p, l)` cell, `z = (F_obs - F_ref) / sqrt(se_obs^2 + se_ref^2)` with
  binomial standard errors, for ALL (unconditional reference) and PRIME (reference
  conditioned on `charpoly(1) != 0`). Report the number of cells with `|z| > 3`.
- Per-`p` `F_l`, `class_l` shares (`fire` / `none` / `undetermined`), the distribution
  of `n_h_l` and `ext_deg_l` among firing PRIME curves, and the same table on C2.

## 7. Confounders and how they are handled

- Equidistribution at small `p`: the reference assumes Frobenius equidistributed in the
  `GSp_6(F_l)` coset. At `p <= 4099` with `l = 7` that need not hold. Reported via S4,
  never used to change the verdict.
- Multiplier mix: rates depend on `p mod l`. Reported per `p`. The pooled rate is a
  mixture over the five chosen residues and is labelled as such.
- Model choice: only odd-degree monic models `y^2 = f(x)`, `deg f = 7` (one point at
  infinity). Twists and even-degree models are not in the population, except that each
  `J`'s twist is used for its order.
- Order method: a divisor class sampler using only `F_p`-rational points generates the
  subgroup they span, which can be proper. Ambiguity is detected, counted (G8) and
  excluded, and cannot affect PRIME (section 5).
- Non-squarefree `chi mod l`: the criterion is sufficient only. `undetermined` shares are
  reported, so a reader can bound how many Lagrangians the criterion may miss.
- Analyst: the decision rule above is applied by `analyze.py` mechanically. The verdict
  string is computed, not typed.

## 8. Exact commands (run from this directory, `L06_odd_ell_isogeny_census/`)

```
mkdir -p raw
(cd l06 && cargo build --release 2>&1) | tee raw/build.log
(cd l06 && cargo test --release 2>&1) | tee raw/test.log
./l06/target/release/l06 lagr2 --out raw/l2_lagrangians.json
./l06/target/release/l06 gsp --samples 200000 --out raw/gsp_reference.json
./l06/target/release/l06 gate-enum --p 101 --count 200 --out raw/gate_enum_p101.jsonl
./l06/target/release/l06 gate-enum --p 257 --count 40 --out raw/gate_enum_p257.jsonl
for pu in 257:3000 521:3000 1031:3000 2053:1000 4099:1000; do
  p=${pu%%:*}; u=${pu##*:}
  timeout 600 ./l06/target/release/l06 census --p $p --u-count $u \
    --prime-target 120 --draw-cap 40000 --out raw/census_p$p.jsonl \
    2> raw/census_p$p.log
  echo "p=$p exit=$?" >> raw/exit_codes.txt
done
python3 -I analyze.py raw > raw/summary.json
sha256sum raw/* > data.sha256
```

Every command is single-threaded. Each run is expected to stay under 10 CPU-minutes and
2 GB. If a repository hook refuses a command, the refusal is recorded verbatim in
DEVIATIONS.md and not worked around.

## 9. Outputs

`raw/` holds the JSON and JSONL files above (formats in SPEC.md section 8). The executor
writes RESULTS.md with the gate table first, then the verdict computed by `analyze.py`,
then the secondary readings. Any departure from this file goes in DEVIATIONS.md.
