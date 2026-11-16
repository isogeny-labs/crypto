# Endomorphism-degree census: how often does a low-degree endomorphism exist?

Status: **frozen before any census data** (2026-11-12); hash in `frozen.sha256`.
Later changes are dated amendments below the line, never edits above it.

## Why this census

`research/small_de_endo_fb_20261112/PROTOCOL.md` asks whether a small-`|D_E|`
curve with an endomorphism-compatible factor base lowers index-calculus cost.
Before running that, we need to know how often a useful low-degree
endomorphism exists at all, on the curves the lab studies and on random curves.

## Facts built on (checked in code, not assumed)

For ordinary `E/F_p` with trace `t`, `Δ = t² − 4p = d_K f²` with `d_K`
fundamental and `f` the conductor of `Z[π]` (`src/isogeny/cm.rs`,
`fundamental_discriminant_and_conductor`). `End(E)` is an order of
discriminant `d_K c²`, `c | f`. A non-rational endomorphism of degree `n`
exists in an order `O` only if `n ≥ minnorm(O)`; the minimum over the isogeny
class is on the crater, where `minnorm(O_K) = |d_K|/4` (`d_K` even) or
`(|d_K|+1)/4` (`d_K` odd). Degree 1 is an automorphism (`d_K = −3, −4`).
So degree `≤ 11` exists in the class only if `|d_K| ≤ 44`. The test needs no
factorisation: for each of the 15 negative fundamental discriminants
`d ∈ {−3, −4, −7, −8, −11, −15, −19, −20, −23, −24, −31, −35, −39, −40, −43}`
check whether `Δ/d` is a perfect square integer. The fundamental `d` with
`Δ = d f²` is unique, so at most one `d` passes.

## Populations

- **(a) Registry.** Every `family: "prime"` row of `docs/curves/registry.json`
  (the ICV1 registry), using its recorded `p`, `a`, `b`, and `t = p + 1 − order`.
  Rows of family binary, koblitz, extension and subfield are skipped; their
  counts are reported with the reason. A row is also skipped if `t² > 4p`
  (the recorded order is not the group order). Rows are reported pooled and by
  bit-size tier. Supersingular rows (`p | t`) are counted separately and are
  excluded from the `d_K` statistics. This population is **not random**: it
  holds curves chosen for earlier experiments (volcano, conductor-gap, BN/BLS,
  testbed), so its fraction is a description, not an estimate for random curves.
- **(b) Named standard curves.** The registry rows with a non-empty
  `standard_names` (orders taken from the registry, whose sources are
  `docs/curves/standards/registry.json`, `src/ecc/curve.rs` and
  `src/ecc/curve_zoo.rs`), plus the registry's pairing-friendly rows (aliases
  under `bn/`, `bls/`, `other/Fp*BN`). The registry order of each named curve
  must satisfy Hasse; P-256 and secp256k1 orders are also cross-checked against
  the literals in `src/ecc/curve.rs`.
- **(c) Null.** Uniformly random `(a, b) ∈ F_p²` with `4a³ + 27b² ≠ 0` over
  random primes with exactly `b` bits, `b ∈ {12, 16, 20, 24, 28, 32, 40}`
  (12 is added as a cheap formula check). Sample sizes (curves, primes):
  12: 30000 (300); 16: 100000 (500); 20: 100000 (500); 24: 100000 (500);
  28: 50000 (250); 32: 20000 (100); 40: 2000 (40). Each prime carries
  `curves/primes` curves. The order is counted by the crate's Shanks–Mestre
  counter (`src/isogeny/cm.rs`, `frobenius_trace_bsgs`; `frobenius_trace` below
  `2^20`), with one retry on the quadratic twist; failures are counted and
  reported, never silently dropped. Seed `20261112`; every prime and curve is a
  deterministic function of (seed, bit size, index).
- **(d) Controls.** Positive: `j = 0` (`D = −3`), `j = 1728` (`D = −4`), and
  CM curves built from the class-number-one `j`-invariants for
  `D ∈ {−7, −8, −11, −19, −43}` (`src/isogeny/cm.rs`, `class_number_one_j`),
  200 curves per `D` and per bit size `{16, 24, 32, 40}` over random primes
  with `(D/p) = 1`, each with a random quadratic twist. Each must be detected
  with `d_K = D`, and (class number one) on the crater. Negative control for
  the crater logic: `D ∈ {−12, −16, −27, −28}` (non-maximal, conductor 2 or 3)
  must be detected with `d_K ∈ {−3, −4, −3, −7}` and reported **not** on the
  crater.

## Outputs per curve and per population

`t`, `Δ`, the smallest `|d_K|` or "`> 44`", `f`, the class-wide minimum
endomorphism degree `minnorm(O_{d_K})`, and whether `E` is on the crater.
The crater is decided exactly when `h(d_K) = 1` (`d_K ∈ {−3, −4, −7, −8, −11,
−19, −43}`): `j(E) ≡ j_{d_K} (mod p)`. For the other `d_K` (class number > 1) the
repository has no Hilbert class polynomial, so the row is marked
"class bound only". For the synthetic and control populations the search is
extended to `|d_K| ≤ 4400` (minimum degree `≤ 1100`) when `Δ` fits 128 bits, so
the degree distribution has buckets beyond 11. Degree buckets:
`1`, `2`, `3`, `4–5`, `6–8`, `9–11`, `12–1100`, "`> 11`" (no `d_K` found within the
bound).

## Prediction (frozen before data)

Exact, not heuristic: by Deuring–Lenstra the number of `(a, b)` with trace `t`
is `(p−1) H(4p−t²)/2` (Hurwitz class number `H`), so for a uniformly random
curve over `F_p`

`P(d_K = d) = (1/2p) Σ_{t²+|d|f²=4p, t>0} 2·H(|d|f²)`,
`H(|d|f²) = Σ_{c|f} h(d)·c·Π_{q|c}(1 − (d/q)/q) / (w(d)/2)`,

and `P(|d_K| ≤ 44)` is the sum over the 15 discriminants. This is computed
**before any curve is generated** by `predict.py` (primes only, seeded
20261112), and was checked by exhaustive enumeration of all `(a, b)` for
`p = 101` and `p = 211` (agreement to all printed digits). The scaling is

`P(|d_K| ≤ 44) ≈ C/√p` with `C ≈ 4` (the printed values of `√p·P` for typical
`p = 1.5·2^{b−1}` range 3.7 to 4.5 [pcc:unverified]).

The heuristic origin: the number of `(t, f)` on the ellipse `t² + |d| f² = 4p`
is `≍ 1/√|d|` per `d`, each weighted by `H ≍ h(d) f`, and `h(d) ≍ √|d|`, giving
`P_d ≍ c_d/√p`; the constant `C` is the sum of the 15 `c_d`, including the local
factors, and is read off the exact formula rather than derived in closed form.
Predicted `P(|d_K| ≤ 44)` by size, mean over the seeded primes in `predict.py`
(`predict.json`):

| bits | 12 | 16 | 20 | 24 | 28 | 32 | 40 |
|---|---|---|---|---|---|---|---|
| P | 7.15e-2 | 1.88e-2 | 4.29e-3 | 1.157e-3 | 3.14e-4 | 6.98e-5 | 4.03e-6 |

[pcc:unverified] (all seven values). Expected hits with the sample sizes above:
about 2146, 1876, 429, 116, 16, 1.4 and 0.008 respectively [pcc:unverified].
So the null **can test the prediction at 12 to 28 bits**; at 32 bits it expects
about one hit and at 40 bits none, where it can only show consistency. At those
sizes the exact formula, not sampling, is the evidence, and it is evaluated at
cryptographic sizes only through `C/√p` (a 256-bit curve: `≈ 4·2^{−128}`).

Predicted for the other populations: curves are chosen, not random, so no
fraction is predicted for (a) and (b) except that BN and BLS12 curves have
`j = 0` and must show `d_K = −3`; the null expectation for a registry row of `p`
bits is `C/√p`. Controls: all detected (probability 1 if the instrument is
correct).

## Statistics

Observed hits against the Poisson-binomial expectation `Σ P(p_i)` over the
realised primes (each curve's `P` is exact for its prime), variance
`Σ n_i P_i(1 − P_i)`; z-score and Wilson 95% interval per size. A size is
"tested" only if its expected hits are `≥ 5`.

## Decision rule (frozen)

The census decides whether `small_de_endo_fb_20261112` is worth running for
discriminants beyond `D = −3, −4` (`j = 0, 1728`, already covered by earlier
automorphism work).

1. **INVALID** (fix the instrument, rerun, do not read further) if any of: a
   positive control `D ∈ {−3, −4, −7, −8, −11, −19, −43}` is not detected with
   `d_K = D` on the crater; a negative-control non-maximal curve is reported
   on the crater; the unit tests fail; or at any tested size the null deviates
   from the exact formula by `|z| > 3`.
2. **WORTH RUNNING ON NATURAL CURVES** if, valid, population (a) or (b) contains
   at least one curve with `p ≥ 2^30`, `|d_K| ≤ 44`, `d_K ∉ {−3, −4}` and either on
   the crater or "class bound only". Then the experiment extends to that curve's
   discriminant.
3. **WORTH RUNNING ON CONSTRUCTED CURVES ONLY** otherwise, if valid: such
   curves exist only by CM construction (a one-time cost, never in a
   per-instance total); natural prevalence is `≈ C/√p`, negligible at
   cryptographic `p`, so the experiment answers a question about designed
   curves, not about deployed ones. The experiment is then justified solely by
   its own protocol, not by this census.
4. **NOT WORTH RUNNING BEYOND `j = 0, 1728`** if valid and the experiment's own
   cost model gives no lever (this census cannot establish that; the verdict
   here is only 1, 2 or 3).

The census reports, for every row, the gap in degree to the smallest useful
endomorphism as a distance, not a pass/fail gate. Nothing here is a claim about
index-calculus cost: a low-degree endomorphism existing says nothing about
whether a factor base closed under it is of useful size.

## Not covered

Supersingular curves (quaternion endomorphism ring); curves over extension or
binary fields; the actual endomorphism ring of non-crater curves with class
number > 1; any solving cost.
