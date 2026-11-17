---
title: "The class number of the CM field of NIST P-224 (and P-256)"
date: "2026-10-22"
---

# Status

**Estimate plus exact genus theory; no exact class number.** Protocol:
[`PROTOCOL.md`](PROTOCOL.md), committed before the computation. Code:
`src/cryptanalysis/class_number_estimate.rs` and
`examples/class_number_estimate.rs`. Records: [`run/results.json`](run/results.json),
[`run/convergence.svg`](run/convergence.svg).

This closes the item "class number of `D_K`: open" in
`research/p224_hypothesis_ranking_20261008/README.md` as far as is honest: the
size of `h(D_K)` is now known to a stated interval under stated assumptions,
and its 2-part is known exactly. The exact value is **not** known and is not
computable here (section "Can `h` be computed exactly?").

This is not an ECDLP cost measurement. It has no rho ratio, no phase is priced,
no scoreboard row is owed, and it is not an improvement of the index-calculus
baseline (`AGENTS.md` section 7a and the success criterion). A bound on the
size of a class group says nothing, by itself, about the hardness of the
curve's discrete logarithm.

Evidence labels: **EXACT** (a theorem plus exact integer arithmetic),
**MEASURED** (computed by this run), **GRH-COND.** (valid if GRH holds, and
where stated if a named constant holds), **HEURISTIC** (random model),
**DERIVED** (arithmetic on the above).

# Result

![Euler-product convergence, the stated error half-widths, and the observed
ratio against the assumed GRH shape. Source: `run/results.json`, example
`class_number_estimate`, B = 10^10, 4 threads.](run/convergence.svg)

| | P-224 | P-256 |
|:--|:--|:--|
| `D_K` | `-3·29·79·7523·40927·11549194661·L₄₅` (`L₄₅` = 388425074903852603481408727235725774844022299) | `-3·5·456597257999·1428624589419343516204097·46523541035814968339936406074986559003387` |
| conductor of `Z[π]` | 3 | 1 |
| `log₂|D_K|` | 222.494 | 257.975 |
| `D_K mod 8` | 5 (so `χ(2) = -1`) | 5 (so `χ(2) = -1`) |
| prime discriminants `t` | **7** | **5** |
| 2-rank of `Cl(D_K)` | **6** (EXACT) | **4** (EXACT) |
| Rédei 4-rank | **0** (EXACT) | **0** (EXACT) |
| 2-part of `Cl` | `(Z/2)^6`, so `h = 64·m`, `m` odd | `(Z/2)^4`, so `h = 16·m`, `m` odd |
| `L(1,χ)`, Euler product to `B = 10^10` | 0.358762 (MEASURED) | 0.722539 (MEASURED) |
| `h` central value (`√|D| L/π`) | **3.5185 × 10^32 = 2^108.1167** | **1.5517 × 10^38 = 2^126.8672** |
| HEURISTIC 3σ interval (relative ±6.3×10⁻⁶ in `ln L`) | [3.5185, 3.5185] × 10^32 | [1.5517, 1.5518] × 10^38 |
| GRH-COND. interval, assumed `c = 1/(8π)` | [3.5128, 3.5242] × 10^32 (2^108.1143 to 2^108.1190) | [1.5485, 1.5550] × 10^38 |
| GRH-COND. interval, assumed `c = 1/π` (headline) | **[3.4735, 3.5641] × 10^32 (2^108.098 to 2^108.135)** | **[1.5261, 1.5778] × 10^38 (2^126.843 to 2^126.891)** |
| sweep wall time (4 threads, shared host) | 93 s | 93 s |

Reading the table:

* The headline interval for `h(D_K)` of P-224 is
  **[3.47, 3.56] × 10^32, i.e. 2^108.10 to 2^108.14**, **conditional on GRH
  and on the assumed explicit constant `c = 1/π`** below. It sits inside the
  earlier "2^106 to 2^111" range and above the "GRH asymptotic lower bound of
  about 2^106". The earlier text's `2^109.6 · L(1,χ)` is now `2^109.6 × 0.3588
  = 2^108.1`. `L(1,χ)` is small because 2 is inert (`D ≡ 5 mod 8`) and 3 is
  ramified, which removes the factors `(1+1/2)^-1` and `(1-1/3)^-1`.
* The `D ≡ 1 mod 4`, odd, squarefree shape and the identity
  `t² − 4p = −f²·∏factors` were re-checked by the program before the sweep (a
  mismatch would stop it). All factors pass a 12-base strong probable-prime
  test here. For P-224 the earlier report records them as proved
  (`cm_discriminant.json`); for P-256 they are the SafeCurves factorisation,
  "probable" in `published_factorizations.json`. Genus theory needs the
  factors to be prime.
* **Nothing about the interval is PROVED.** What is proved: the 2-rank, the
  Rédei 4-rank and so the 2-adic valuation `v₂(h)`.

# Method

## 1. Euler product (MEASURED)

`χ(p) = (D_K/p)` for every prime `p ≤ B = 10^10` (about 4.55 × 10^8 primes), by
a segmented sieve and a Jacobi symbol of `D_K mod p` (limbs of `|D_K|` in
base 2^29, `u64` reduction). The result is deterministic: segments are merged
in order with Kahan summation, and a unit test checks that 1 and 3 threads give
bit-identical sums. `ln L_B = Σ_{p≤B} −ln(1 − χ(p)/p)`, then
`h = √|D_K| · L / π` (`w = 2`).

Convergence (`run/results.json`, `checkpoints`; panel (a) of the figure):
`ln L_B` for P-224 is −1.00132 at `B = 10^3`, −1.02631 at `10^4`, −1.02510 at
`10^6`, and −1.0250974 at `10^10`; the last decade moves it by less than
5 × 10^-6. About half the primes split and half are inert at every checkpoint
(P-224 at `10^10`: 227,526,100 split, 227,526,406 inert, 5 dividing `D`:
3, 29, 79, 7523, 40927; the two largest prime factors exceed `B`).

## 2. Error statements

* **HEURISTIC.** If the signs `χ(p)`, `p > B`, behave as independent fair
  signs, the tail of `ln L` has standard deviation `(Σ_{p>B} p^-2)^{1/2} ≈
  1/√(B ln B)`: 2.1 × 10^-6 at `B = 10^10`. This is a model, not a theorem.
  Calibration (below) supports it for discriminants of size 10^7 at small `B`.
* **GRH-COND., with a named assumed constant.** Assume
  `|Σ_{n≤x} Λ(n) χ(n)| ≤ c √x ln²(|D| x)` for `x ≥ B`. The constant `c = 1/(8π)`
  is the Schoenfeld-type shape I recall for this kind of bound; I did not
  re-derive it or look it up here, so the headline uses `c = 1/π`, eight times
  larger, as a safety factor. Then, with `F(t) = c√t ln²(|D|t) + 1.2√t` (the
  second term covers prime powers, `ψ(t) − θ(t) < 1.2√t` for `t ≥ 1000`),
  partial summation gives `|ln L − ln L_B| ≤ F(B)/(B ln B) + ∫_B^∞ F(t)(ln t +
  1)/(t² ln² t) dt + 1/B`, where the last term bounds the `u²/2` Euler-factor
  remainder. The integral is evaluated numerically (Simpson in `ln t`, 24,000
  steps to `t = B e^120`). At `B = 10^10` the half-width of `ln L` is 0.0129
  (`c = 1/π`) and 0.0016 (`c = 1/(8π)`) for P-224, 0.0167 and 0.0021 for P-256.
  The bound decays like `ln²|D| / (√B ln B)`; panel (b) of the figure shows
  both half-widths against `B`.
* **Observed against the assumed shape (MEASURED).** The measured maximum of
  `|Σ_{p≤x} χ(p) ln p| / (√x ln²(|D|x))` over the 21 checkpoints is 3.1 × 10^-4
  for P-224 and 1.3 × 10^-4 for P-256, about 130 times below `1/(8π)` = 0.0398.
  This is a check on the plausibility of the assumed form, not a verification
  of it for all `x`.

## 3. Controls (pre-registered; `run/results.json`, `controls`)

400 consecutive fundamental discriminants below −10^7 (`D` from −10,000,003 to
−10,001,323), exact `h` by counting reduced forms:

| `B` | within 3σ (heuristic) | GRH band (`c = 1/π`) covers | sd of `z = (ln h − ln h_B)/σ` | max `|z|` |
|--:|--:|--:|--:|--:|
| 10^3 | 99.75 % (399/400) | 100 % | 0.85 | 3.11 |
| 10^4 | 100 % | 100 % | 0.84 | 2.81 |
| 10^5 | 100 % | 100 % | 0.93 | 2.31 |

The pre-registered criteria (GRH band covers every `D`; 3σ covers at least 99 %)
hold. The heuristic σ is not underestimated on these small discriminants
(sd of `z` is below 1). That is a calibration at `|D| ~ 10^7`, `B ≤ 10^5`; it
does not establish the model at `|D| ~ 2^222`, `B = 10^10`, where `B` is
astronomically below `√|D|`. Further controls, all unit tests in the module:
exact class numbers of 23 tabulated discriminants (−3, −4, −7, −8, −11, −15,
−20, −23, −31, −39, −47, −71, −79, −84, −87, −95, −103, −127, −151, −163, −199,
−239, −427: h = 1,1,1,1,1,2,2,3,3,4,5,7,5,4,6,8,5,5,7,1,9,15,2); the theta
series agrees with exact `h` to 10^-8 for every fundamental `D > −3000`; the
Euler product agrees with exact `h` within 1.5 % at `B = 10^6` for four `D`
near 10^4 to 10^6; genus theory gives 0 violations on every fundamental
`D > −3000` and on the 400 control discriminants.

## 4. Cohen-type series (cross-check only)

`L(1,χ) = Σ χ(n) e^{−πn²/|D|}/n + (π/√|D|) Σ χ(n) erfc(n √(π/|D|))`, from
the functional equation of the odd quadratic character (root number 1). The
terms fall below 10^-17 once `n > √(12.4|D|)`, so it needs about √|D| terms.
It is exact to double precision for small `D` and is what the unit test
compares against exact `h`. At `|D_K|` it needs about
`√(12.4·|D_K|) = 2^113.1` terms: **not runnable**. The Euler product is the
only analytic method used for `D_K`.

## 5. Genus theory and the Rédei 4-rank (EXACT)

For a negative fundamental discriminant with `t` prime discriminants the
class group has 2-rank `t − 1` (genus theory). `D_K` of P-224 is odd with 7
prime factors, so `Cl[2] ≅ (Z/2)^6` and **`2^6 | h`, `h` even**; of P-256, 5
factors, `(Z/2)^4`, `2^4 | h`. The 4-rank is `t − 1 − rank_F2(R)` for the Rédei
matrix `R` (off-diagonal `r_ij = 0` iff `(d_j / p_i) = +1` for the prime
discriminants `d_j`, diagonal = row sum). The rank is 6 (P-224) and 4 (P-256),
so the 4-rank is **0 in both**: the 2-Sylow subgroup is elementary abelian,
`v₂(h) = t − 1` exactly, and `h / 2^(t-1)` is odd. The method is checked on
every fundamental `D > −3000` against `v₂` of the exact class number (4-rank 0
implies `v₂ = t − 1`; positive 4-rank implies `v₂ ≥ t`; the control is not
vacuous, more than 20 discriminants have positive 4-rank).

DERIVED consequence: the odd cofactor `h/64` is about 5.50 × 10^30 (P-224) and
`h/16` about 9.70 × 10^36 (P-256), each with the relative uncertainty of the
`h` interval. The 8-rank and higher 2-power structure are moot because the
2-Sylow subgroup is already `(Z/2)^{t-1}`. Nothing is said here about the odd
part's structure, in particular whether `Cl(D_K)` is cyclic away from 2.

# Can `h` be computed exactly?

**Not here, and not unconditionally anywhere I can point to.** Nothing in
this section was run; the operation counts are scalings from the literature,
with the `o(1)` terms dropped, not measurements, and I did not look the
published constants up while writing it.

* **Analytic formula, exact.** Cohen's series needs `2^113` terms (above).
  Reaching `h` exactly from `L(1,χ)` needs `L` to relative precision about
  `1/h = 2^-108`, so the truncated Euler product, whose error is a conditional
  `~10^-2` and a heuristic `~10^-6`, is about 30 orders of magnitude short.
* **Unconditional.** A proof that a candidate value equals `h` without GRH
  needs either every prime ideal up to the Minkowski bound
  (`√(|D|/3) ≈ 2^110`) or an exponential-time algorithm. Shanks-type
  baby-step giant-step in the class group costs about `√W` group compositions
  for an enclosure of width `W`. With no enclosure `W ~ h ~ 2^108` and
  `√W ~ 2^54`, i.e. order `|D|^{1/4}`; at 1 to 5 µs per 222-bit composition
  (an assumed figure) that is about 1.8 × 10^16 compositions, i.e. 600 to 3,000
  core-years. Even a fully trusted enclosure does not make it small: the
  GRH-conditional `c = 1/π` interval has `W ≈ 2^102.8` and `√W ≈ 2^51.4`; the
  heuristic 3σ interval has `W ≈ 2^91.8` and `√W ≈ 2^45.9`, still about 2 to 11
  core-years of compositions with a baby-step table of `2^46` entries that
  does not fit in 15 GB (a memoryless kangaroo variant avoids the table but not
  the time, and a conditional enclosure makes the result conditional).
* **Subexponential (Hafner–McCurley, Buchmann, Jacobson).** The cost scales
  as `L_|D|[1/2, c] = exp(c √(ln|D| ln ln|D|))`: here `ln|D| = 154.2`,
  `√(ln|D| ln ln|D|) = 27.9`, so `c = 1` gives `e^27.9 ≈ 2^40` and `c = √2`
  gives `e^39.4 ≈ 2^57` (P-256: `e^30.5 ≈ 2^44` and `e^43.1 ≈ 2^62`). As I recall, published
  implementations reach discriminants of about 100 digits,
  and `|D_K|` has 67 digits, so the class of algorithm is in range
  for this size on a multi-core machine; I make no claim about how long it
  would take on this host. **It is not available here:** this host has no
  PARI/GP, Magma or Sage, and `AGENTS.md` requires the computation to be Rust.
  Writing a relation-collecting class-group engine (factor base up to the
  GRH bound of order `12 ln²|D| ≈ 2.9 × 10^5`, sparse linear algebra over Z,
  Hermite or Smith normal form of the relation lattice) is a project of its
  own, and I did not start it.
* **What the result would be.** The relation-lattice determinant is a
  multiple of `h`; it equals `h` if it lies within a factor 2 of an
  independent estimate of `h`. The estimate above (error ≲ 2.6 % under
  GRH) is already far tighter than that step needs. But that makes the final
  `h` **GRH-conditional**: the factor-base bound that makes the relations
  generate `Cl(D_K)` is itself a GRH statement. I know of no way to turn it
  into an unconditional proof at 222 bits.

So the honest summary is: an interval (conditional) and the exact 2-part are
what can be had here; a GRH-conditional exact `h` is a feasible but unbuilt
project; an unconditional exact `h` is out of reach of any known method at
this size.

# Limits

* The GRH-conditional intervals assume an explicit constant that I did not
  derive (`c = 1/π`, eight times the shape quoted from memory). If the true
  explicit bound is weaker, the interval widens; the measured shape ratio
  (130 times below `1/(8π)`) makes that unlikely but is not proof.
* The heuristic interval is a random-sign model; its calibration is on small
  `D` and small `B`.
* The sweep used `B = 10^10` only (protocol). Extending to `B = 10^11` would
  shrink the conditional half-width by about √10, at about 15 minutes of
  4-core time per curve; not done.
* Primality of the factors is the 12-base strong test here; the Rédei and
  genus statements are exact given that the listed factors are prime.
* Wall times are from a shared host and are not a benchmark. No timing is
  claimed as a result, and nothing goes to `benchmarks/`.

# Reproduce

```bash
cargo test --release --lib cryptanalysis::class_number_estimate
cargo run --release --example class_number_estimate -- \
    --curve both --bound 10000000000 --threads 4 --out /tmp/cn-check
# compare /tmp/cn-check/results.json with run/results.json (identical except
# sweep_wall_seconds)
```

SHA-256 of the committed outputs:

```
61016700c9aeef3478c3d9ca11425a653a7dd09b177ca7874648aefefe1e19d5  run/results.json
74d5a0e907eb6b479081f190ee24e9fbdf68d2cdcf87795dc95941247020158b  run/convergence.svg
```
