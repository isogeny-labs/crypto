# Do Solinas-shape primes change the distribution of `#E(F_p)`? Protocol

**Status: frozen** by the commit that adds this file, before the census run.
Conductor task T-4. Native instrument:
[`examples/solinas_order_census.rs`](../../examples/solinas_order_census.rs).
Background: `research/notes/cm-isogeny/RESEARCH_NIST_SOLINAS_STRUCTURE.md`
(p ± 1 of P-192, P-224 and P-521-shape primes is algebraically smooth) and
`research/elliptic_snfs_survey_20261016/README.md` §1–§2 (why that smoothness
does not reach `E(F_p)` through an SNFS-style argument).

## 1. Question and claim classes

The hypothesis under test is the user's: *Solinas primes put a pattern in the
distribution of elliptic-curve group orders that a sieve-like method could
use.* Over a prime field, `#E = p + 1 − t`, and the only way `p`'s shape can
act on `#E` is through the Frobenius residue class: `det Frob = p`. The
matrix model (Gekeler 2006; Castryck–Hubrechts 2016) says that for a random
curve, `Frob mod m` is uniform on `{M ∈ M₂(Z/m) : det M = p mod m}`. So the
distribution of the `m`-part of `#E` depends on `p` **only through `p mod m`**.
A Solinas prime has an unusual residue (`p ≡ 1` or `−1` mod `2^k`, and fixed
residues mod 3, 5, …). The census asks whether anything beyond that residue
class shows up.

| sentence | class |
|:--|:--|
| the model above gives the exact distribution of `min(v₂(#E), 5)`, of `[3 \| #E]` and of `[5 \| #E]` given `p mod 480` | heuristic estimate (an equidistribution theorem in the limit; finite `p` is tested here) |
| Solinas-shape primes fit the model for their own residue | comparative / statistical (this protocol) |
| the congruence class of a Solinas prime shifts these probabilities by at most `ε` from the generic-prime mixture | exact computation (enumeration of the model, no sampling) |
| anything about ECDLP cost, large `p`, other primes `ℓ`, or odd-part smoothness | out of scope: not established |

## 2. Arms (same instrument on all)

- **Targets.** Every prime of the form `2^n − 2^k + 1` ("p224-shape") or
  `2^n − 2^k − 1` ("p192-shape") with `16 ≤ n ≤ 22` and `6 ≤ k ≤ n − 3`.
- **Residue-matched null (R).** For each target, 2 random primes with the
  same bit length and `p' ≡ p (mod 480)`; `480 = 2⁵·3·5` covers every residue
  the model depends on. Everything else about `p'` is random.
- **Generic null (G).** For each target, 2 uniformly random primes of the same
  bit length, no residue constraint.
- Excluded from the nulls: any target prime and any prime already drawn.
- Sampling: SplitMix64, `--seed 20261023`; per-prime seeds are drawn from it
  in a fixed order (see the source).

For each prime, `N = 4000` curves `y² = x³ + ax + b` with `(a, b)` uniform on
`F_p²`, discriminant `≠ 0`, counted exactly by the Legendre-symbol sum over
all `x`.

## 3. Metrics and tests (fixed)

Per prime, with `model(p)` the exact matrix-model probabilities (2-adic
precision `2⁵`, odd parts mod 3 and mod 5):

- **F1.** Chi-square of the 6-bin counts of `min(v₂(#E), 5)` against
  `model(p mod 32)`. Bins with expected count below 5 are merged.
- **F3.** `[3 | #E]` against `model₃(p mod 3)`, one-degree chi-square.
- **F4.** `[5 | #E]` against `model₅(p mod 5)`, same.
- **F2 (positive control, detector power).** The same chi-square as F1 but
  against the model of the **wrong class mod 4**, `d′ = −p mod 32`. It must
  reject for every prime. This is a known effect (the models for `p ≡ 1` and
  `p ≡ 3` mod 4 differ by about 0.04 in the bins `v₂ = 2, 3`).
- **Reported exactly, not tested:** `tv_generic`, the total-variation distance
  between the prime's own-residue model and the generic model (mean over odd
  `d ≡ p mod 4`). This is the most the Solinas congruence class can move the
  `v₂` distribution relative to a generic prime of that residue mod 4.

## 4. Decision rule

`α = 0.01`, Bonferroni over all F1, F3 and F4 tests (`3 ×` the number of
primes), threshold `α / m`. Per role (target, R, G):

- **"Fits its own-residue model"** for a role means no F1, F3 or F4 test falls
  below the threshold.
- The **instrument is valid** only if F2 rejects for every prime in every role
  (threshold `α / number of primes`). Otherwise the run is *inconclusive*.
- **"Solinas shape adds nothing beyond its residue class"** is licensed only
  if targets fit (above), R and G fit (instrument calibration: a model that
  fails on non-Solinas primes is a bad model, not a Solinas effect), and F2
  holds. Verdict wording: "within the model for its residue class at this
  resolution", never "random" or "no effect".
- A target that fails F1, F3 or F4 is reported as "inconsistent with the
  model", with the counts, and examined before any further claim. Nothing is
  re-run to get a different answer.

## 5. Power check and runtime (before the run)

- The tests are parametric chi-square tests against exact probabilities, not
  empirical rank tests, so `2/(N + 1)` does not bind. What binds is the power
  to see an effect. At `N = 4000` the F2 control has median noncentrality
  about 76 on 4 degrees of freedom (dry run, 14–18 bits). The critical value
  at `α / m ≈ 1.7·10⁻⁵` is about 28, so the power is above 0.99. The control
  is easily detectable.
- **What the run cannot see:** `tv_generic` is at most 0.003 in the dry run.
  An effect that small needs about `N ≈ 10⁶` curves per prime to resolve
  (standard error of a bin probability at 4000 curves is about 0.007). So the
  census can say the data **fit the model**. It cannot say the Solinas class
  differs from the generic class: the model itself says the difference is
  below the resolution.
- **Runtime estimate.** One instrument call: about 22 ms per curve at 22 bits
  (table lookup per `x`), proportional to `p`. There are no fixed per-target
  costs apart from the Legendre table (negligible). Estimate for 16–22 bits,
  about 40 targets × 5 primes, 4000 curves, 4 threads: about 20 min. The
  actual time is recorded in the results.

## 6. Dry run, disclosed

- Dry runs at 14–18 bits with `N = 1000` and `4000` (seeds 1 and 2), output
  in scratch, nothing from it enters the results. They showed: no F1/F3/F4
  rejections in any role; and **the original F2 had no power**. The first F2
  was the generic model, which differs from the own-residue model by
  `tv_generic ≤ 0.003`, so it could not be told apart. That control was
  replaced, before any real data, by the wrong-class model above, which
  rejects in 100% of the dry run. The consequence is part of the result: the
  Solinas congruence class has almost no extra effect on `v₂(#E)` beyond
  `p mod 8`, by the model itself.
- The node count, `k ≥ 6` and `n ≤ 22` limits come from runtime, not from
  the data.

## 7. What does not count

Changing `N`, the bit range, `k`, the nulls, the threshold or the metrics
after the run; dropping a prime; re-running a prime with a new seed. Any
change is a new, labelled protocol version, and both are reported.

## 8. Scope sentence (written before the run)

Even a clean result covers only: primes of 16–22 bits, the three metrics
(`v₂(#E)` up to 5, `3 | #E`, `5 | #E`), uniform random `(a, b)`. It does not
cover the `a = −3` NIST coefficient family, structured curves (GLV, CM),
odd-part smoothness of `#E`, the index-calculus relation yield, or any
statement about 224-bit primes. It says nothing about ECDLP security.
