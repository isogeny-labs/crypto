# rho-fold: the strongest Pollard rho for the graph-IC lane's targets

Lane key `rho-fold`, round 2, 2026-11-18. Every number below is
[pcc:unverified]. No paid compute, no new timing runs, no remote side effects.

## Question

Which Pollard-rho speed-ups legitimately apply to the lane's public targets?
The candidates are negation, Frobenius or extra-automorphism classes, batch
inversion, distinguished points and tag tracing. For each one, the exact
condition is stated and checked on the actual curves. The lane's targets are
weak-cover curves `E: y^2 = x(x - a)(x - sigma a)` over `F_{p^6} = F_{q^3}`,
with `q = p^2` and `#E = 4l`. The result is then compared with the rho arms the
lane actually ran: plain or negation, each with batch 1 or 16. The last
question is whether the registered rho reference understates rho, and so
misstates the distance remaining.

## Commands

```
df -h /                      # 14 GB free before the build
./run.sh                     # rustc -O of src/rho_fold.rs into
                             # /home/user/cargo-target-round2/rho-fold (single file, no crates,
                             # no cargo workspace build), then runs it on inputs.txt
```

`run.sh` uses jq to extract every distinct public target of the lane into
`inputs.txt`:

- Runs 41 to 51 and nagao-e2e share one target: p1009, seed 389,
  `l = 263807419614139553`. Run 51's plan pins public SHA-256 `b3ae4f93…`,
  which is identical to the public.json of runs 41 and 50. nagao-e2e/public.json
  has the same content in a different serialisation.
- Runs 29, 31, 33, 34, 35, 37 and 39/40, all p1009.
- Run 32, p101.

It also adds the four recorded run50 rho arms: steps from `*.raw.json` and
observed bitops from that run's README table. Runs 42 to 45 and 51 have no
separate public file; they reuse the run41 target. Output: `results.txt`.

The Rust checker rebuilds the `jv_cover` tower with the same choice of `w` and
`s` (p1009 gives `w = 11` and `s = 2`). It then checks that the repo's
zeta-form `sigma` equals `x^q`, that G and Q lie on the curve, that
`[l]G = [l]Q = O`, that l is prime (Miller–Rabin) and that `t = p^6 + 1 - 4l`
satisfies Hasse. All checks pass on all 9 targets.

## Results: which folds apply (all 9 targets, same outcome)

| Variant | Exact condition | Check on the targets | Applies? |
|---|---|---|---|
| Negation map | `-P` is computable for free (always true) | – | **yes, fold 2.** Already in the lane's negation arms. |
| Extra automorphisms | `j(E) = 0` (order 6) or `1728` (order 4) | `j` lies in no proper subfield and is neither 0 nor 1728. The Frobenius discriminant `4p^6 - t^2` is neither `f^2` nor `3f^2`, so no curve in the isogeny class has CM by `Z[i]` or `Z[zeta3]`. | no |
| Frobenius / subfield fold (fold `2*6/k`) | A degree-`p^k` endomorphism over `F_{p^6}` for some k in {1,2,3}. That means `j^{p^k} = j` and `E` is `F_{p^6}`-isomorphic to `E^{(p^k)}`; GLS-type twists are included. | `deg(a) = deg(a2) = deg(a4) = deg(j) = 6`. For every k, `j^{p^k} != j`. | no |
| The same fold after transport by isogeny | Some curve in the isogeny class is a base change, or a twist of one, from `F_{p^k}`: `V_{6/k}(tau) = +-t` for some integer `|tau| <= 2 sqrt(p^k)` | Exhaustive search over tau finds no match for k = 1, 2 or 3 | no |
| Batch inversion (Montgomery) | Pays off only if `I > 3M` in the chosen unit | In bitop/v1 at b = 10, `I/M` is 1.45 with the registered native arithmetic and 1.97 with Karatsuba arithmetic. | **counterproductive in bitops**. Batch 16 costs 1.32x more per step than batch 1, which matches the recorded arms (441.0k against 334.4k bitops per step). It helps only in wall time. |
| Distinguished points | Memory and parallelism only | dp_bits = 16, r = 256: tail overhead is under 1% of `E[iter]` | Neutral. Already used. |
| Tag tracing (Cheon–Hong–Kim) | Defined for `F_p^*` and finite-field multiplicative groups | The repo's literature notes (`research/curve25519_autolab_20261104/ideas/LITERATURE-20261110.md` §2.4) found no elliptic-curve form | no (no EC form known here) |
| MOV / anomalous (not rho; a sanity check) | Small embedding degree, or `l = p` | Embedding degree is above 200; `l != p` | no |

Detector controls pass ([pcc:unverified]):

- A curve over `F_p` reports an endomorphism for k = 1, 2 and 3.
- Its quadratic twist by a nonsquare of `F_{p^6}` (GLS type) also reports
  k = 1, 2 and 3.
- A curve over `F_{p^2}` reports only k = 2; one over `F_{p^3}` reports only
  k = 3.

So a missing fold on the targets is not a blind detector.

**The strongest legitimate fold is 2, from negation, and it is already in the
registered arms.** `E[iter] = sqrt(pi l/4) * sqrt(1/(1-1/r)) * (1 + 1/r) =
4.579e8`. This includes the r = 256 adding-walk factor 1.00196 and about 2
wasted steps per fruitless 2-cycle (recorded cycle rate 1.95e-3 per step,
about 1/(2r)). For comparison, plain rho needs `sqrt(pi l/2)` = 6.437e8
iterations. [pcc:unverified]

## Results: cost per iteration and the reference

The registered native arithmetic (`jv_rho_strong::Arith`, source-derived
counts) prices an `F_{p^6}` product at 55 M + 44 A and an inversion at
80 M + 42 A + 1 I. bitop/v1 at b = 10 gives M = 1230, A = 130 and I = 2600.

The counted Karatsuba tower in `src/rho_fold.rs` was checked against
schoolbook products and `x * x^-1 = 1` on 2000 random operands. It costs:

| Operation | Counts | bitops | Registered native |
|---|---|---|---|
| Product: Fq Karatsuba (w-product charged as M), cubic Karatsuba | 28 M + 60 A | 42,240 | 73,370 |
| Squaring: Chung–Hasan | 24 M + 36 A | 34,200 | 73,370 |
| Inversion: the same adjugate built from Karatsuba Fq products | 59 M + 63 A + 1 I | 83,360 | 106,460 |

Per-step arithmetic is 2M + S + I/k + 3(k-1)/k M + 42 A. At batch 1 it is
332,030 bitops registered and 207,500 with Karatsuba, a ratio of 1.600. The
bookkeeping overhead calibrated from the recorded arms is 2.4k bitops per step
plain and 6.2k with negation. The model reproduces the recorded per-step bitops
to within 1.9%.

| Rho variant (expected cost, run41–51 target) | E[iter] | bitops/iter | E[bitops] |
|---|---:|---:|---:|
| plain, batch 1, registered | 6.450e8 | 334,402 | 2.157e14 |
| plain, batch 16, registered | 6.450e8 | 440,949 | 2.844e14 |
| **negation, batch 1, registered** (best registered arm) | 4.579e8 | 338,229 | **1.549e14** |
| negation, batch 16, registered | 4.579e8 | 444,776 | 2.036e14 |
| **negation, batch 1, Karatsuba tower** (strongest found) | 4.579e8 | 213,699 | **9.784e13** (0.632x) |

Recorded run50 arms: one cold run each. The ratio is steps divided by E[steps].

| Arm | Steps / E[steps] | Observed bitops | Inner s |
|---|---:|---:|---:|
| plain b1 | 0.913 | 1.966e14 | 119.8 |
| negation b1 | 1.176 | 1.811e14 | 117.6 |
| plain b16 | **0.387** | **1.100e14** | **36.9** |
| negation b16 | 1.356 | 2.745e14 | 102.9 |

Multiplying each arm's own recorded ns/step by E[iter] gives the indicative
expected wall times below. These are derived figures, not new timings.

| Arm | Expected wall |
|---|---:|
| plain b1 | 131.4 s |
| negation b1 | 100.5 s |
| plain b16 | 95.4 s |
| negation b16 | 76.3 s |

## Conclusion: does the registered reference understate rho?

1. **Iteration count: no.** Negation (fold 2) is the maximum legitimate fold
   on every lane target. No Frobenius or automorphism class exists, even
   anywhere in the isogeny class. The registered negation arms already use
   it. [pcc:unverified]
2. **Per-iteration cost: yes, by about 1.58x in bitop/v1.**
   - The registered arms price a schoolbook-style tower.
   - A Karatsuba/Chung–Hasan tower with batch 1 brings the strongest expected
     rho to 9.78e13 bitops, against 1.55e14 for the best registered
     configuration.
   - Batching to 16 makes bitops worse, not better.
   - If multiplications by the small constants w = 11 and s = 2 were treated
     as additions, the gap would grow further. That was not computed.

   This is an arithmetic re-pricing of the reference, not an algorithmic
   improvement of anyone's method. [pcc:unverified]
3. **The quoted "fastest registered rho" is a lucky single draw.** The run50
   note compares the graph process (27.8 s) with plain b16 at 36.9 s.
   - That run used 0.387 x E[steps]; for a Rayleigh stopping time,
     P(X <= 0.387 E) = 1 - exp(-pi * 0.387^2 / 4) = 0.11.
   - The minimum of four N = 1 draws is a biased estimator.
   - In expectation, the strongest recorded wall configuration is negation
     b16 at about 76 s, and the strongest bitop configuration is negation b1
     at 1.55e14.
   - So the single-run minimum *overstates* rho's speed: about 2.1x in wall
     time and about 1.41x in bitops against the registered expectation. It is
     closer in bitops to the Karatsuba expectation (1.10e14 against 9.78e13)
     only by chance.
4. **Distance remaining.** Against the strongest legitimate rho
   (9.78e13 expected bitops), the run50 observed IC subtotals of 4.5e12 to
   6.2e12 are 16x to 22x lower. They are still **incomplete**: the descent
   linear stage is outside the hooks, and complete-cost, band and parity
   fields are null. The IC-to-rho ratio of wall time per bitop (about 12x)
   says the IC counts are far less complete than rho's. So the honest
   statement is about the reference only:
   - In bitops, the strongest rho is about 1.58x cheaper than the registered
     expectation.
   - In wall time, the strongest expected rho is about 2x slower than the
     quoted 36.9 s.
   - The IC-side distance stays undetermined until complete accounting lands.
   - No parity claim follows. [pcc:unverified]

## Failures and limits

- research-kb was unavailable: `scripts/research-kb` reports that no
  executable was found, and no MCP tool is exposed. Local sources were read
  instead.
- The Karatsuba variant is an op-count model run on real operands. It has not
  been measured as a rho walk, and no wall-time claim is made for it.
- Expected-iteration figures use the standard random-walk heuristics:
  - `sqrt(pi l/(2*fold))`
  - Teske/Brent–Pollard `sqrt(1/(1-1/r))`
  - an approximate fruitless-cycle term

  The recorded arms are N = 1 each and cannot test them.
- Bookkeeping overhead (hash, DP and cycle checks) comes from the registered
  arms. A differently engineered walk could change it, but it is under 2% of
  each step.

## Next steps

- Draft protocol (no dispatch): `protocol.draft.json`. It covers an opt-in
  Karatsuba/Chung–Hasan `Arith` in `src/cryptanalysis/jv_rho_strong.rs`,
  outside this lane's paths. It would be validated against the existing
  native-tower hand vectors, then run as negation b1 and b16 with N >= 20
  seeds on the run41 target. The reference would become the mean with an
  interval, never a minimum.
- Report the reference as expected cost (E[iter] x measured per-step cost),
  and drop "min over N = 1 arms".

## Sources read

- `src/cryptanalysis/jv_rho_strong.rs`: walk, Arith, TUNED, prices
- `src/cryptanalysis/jv_cover.rs`: Fq, Fq3, sigma, EllE, generate_spec
- `docs/units/BITOPS.md`: F_p primitive prices
- `NATIVE_RHO_PRICE_REVIEW.md`
- `STRONG_RHO_ACCOUNTING_FOLLOWUP.md`
- `RESUME_UNIT51.md`
- `unit-deferred-plan.json`
- `runpod-deferred-pair-20261117-50/{README.md,NEXT.md,*.raw.json}`
- `research/curve25519_autolab_20261104/ideas/LITERATURE-20261110.md` (tag
  tracing, Bos–Kleinjung–Lenstra)
- `docs/bounds/FRONTIER.md` (signed-Frobenius rho exists for Koblitz curves
  only)

## Files

- `src/rho_fold.rs`: the checker, standalone Rust
- `run.sh`: glue
- `inputs.txt`: extracted public targets and recorded arms
- `results.txt`: full output
- `protocol.draft.json`
