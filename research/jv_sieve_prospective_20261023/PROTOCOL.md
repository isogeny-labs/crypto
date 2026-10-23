# Protocol: the Joux–Vitse sieve on `E(F_{p⁶})` against the strong rho, prospective re-run

2026-10-23. Frozen before any measured data of this study. Item 3 of the
extension-field programme.

## Why a re-run

`research/jv_sieve_strong_rho_20261019/` measured the cover-and-sieve route
below a batched negation rho at `p ≥ 1009` on the weak class, but its
review (`review-20261020/`) withheld every comparison because

1. its positive control (`R0 S ≈ 1.3`) and its rho A/A rule had **no
   numeric acceptance criterion**, and choosing one after the data is not
   allowed;
2. the rho producer **omitted the distinguished-point hash and two history
   hashes** on negation walks (fixed in source since; the old rows cannot be
   re-priced into complete accounting);
3. no replay of the recorded walks under the repaired engine exists.

This study is the prospective protocol the review asked for. It re-runs
the same instances on the repaired source with every acceptance rule fixed
here, now.

## Scope (what a result can and cannot say)

- **Class:** `y² = x(x − α)(x − σα)` over `F_{q³}`, `q = p²`, `α ∉ F_q` (full
  rational 2-torsion, genus-3 GHS cover over `F_q`); never prime order.
  The **challenge curve is the weak curve** handed to the sieve. The
  isogeny walk from a random curve into the class is **not** priced here
  (ledger §18.4 prices it separately); a result is about this class only.
- **Theory:** index calculus on the genus-3 cover over `F_{p²}` costs about
  `q^{4/3} = p^{8/3}` against rho's `p^3`; a gap is the expected outcome
  (Joux–Vitse, Eurocrypt 2012; Diem). The measurement locates it.

## Instances

`jv_cover::generate_spec(p, seed)`, seeds 1..16, `p ∈ {251, 503, 1009,
1511, 1823}` (`ℓ ≈ 2^45.8 … 2^63.0`), identical to the 2026-10-19 study.

## Arms (driver: [`run.sh`](run.sh))

- **IC:** `jv_cover --exp sieve --stop 64`, `JV_LIN=sparse`, 16 seeds per
  size; IC A/A: the same at `p = 1009` a second time.
- **R3, the strong rho** (`jv_rho_strong`: negation map, batched inversion
  `k = 16`, `r = 256`, DP bits 10/12/14/15/15): 16 seeds per size;
  **R3 A/A**: `p = 1009` with run index 1 (new walk randomness, same
  instances).
- **R0, positive control** (no negation, one inversion per step, the
  ledger's reference walk): 16 seeds at `p ≤ 1009`, 8 at 1511, 4 at 1823.
- **Symmetry check:** `examples/jv_endo_check.rs`: is `j(E)` in `F_{p²}` or
  `F_{p³}` on any instance (then a Frobenius-type map might exist and R3
  would not be the strongest rho)?

## Prices

Unchanged from the 2026-10-19 protocol: `S` = `F_p` multiplications /
(331 `√ℓ`) in the harness unit; the tuned unit (Karatsuba, Chung–Hasan,
adjugate inversion) prices rho only; `S⁺` charges every `F_p` addition and
native operation at one multiplication; the §5a band charges native work
at `w ∈ {0, 0.1, 1}` GAE per operation on every arm, in layer A (all
non-multiplication counted work, `F_p` additions included; IC additions
inside F4 and the linear algebra bounded by one per multiplication) and
layer B (excluding `F_p` additions). Rho hash counts are those of the
repaired source (walk hash, distinguished-point hash, two history hashes
on negation walks).

## Acceptance rules (numeric, fixed now)

| check | statistic | accept when |
|---|---|---|
| correctness | every IC and rho answer | equals the planted logarithm |
| symmetry | `jv_endo_check` | `j ∉ F_{p²} ∪ F_{p³}` on every instance used; an instance that fails is excluded from every claim and reported |
| IC A/A | sparse `p = 1009`, twice | every count identical |
| R3 A/A | `mean S(R3, run 1) / mean S(R3, run 0)` at `p = 1009`, harness unit | point estimate in [0.80, 1.25] **and** 95% bootstrap interval contains 1 |
| positive control | `mean S(R0)` (harness unit), each of `p = 251, 503, 1009` | in [1.05, 1.60] (the ledger's 1.3; `√(π/2) ≈ 1.25` plus walk overhead) |
| batching/negation present | `mean S(R3) / mean S(R0)` (harness unit), `p = 1009` | below 0.80 |

If a control fails, every comparison at the affected sizes is reported as
withheld, not as a result, and no threshold is moved.

## Decision rules (fixed now)

Ratio of means `S⁺_IC / S⁺_R3` with a 95% bootstrap interval (10⁴
resamples over instances) at each `p`.

- **C1 (headline):** at `p`, "the sieve is below the strong rho on this
  class" iff the interval's upper end is below 1 in the **harness** price
  **and** in the **tuned** price (rho priced in its favour), **and** the
  ratio of band means is below 1 at every weight `w ∈ {0, 0.1, 1}` in both
  layers A and B.
- **C2 (narrowed):** if the interval is below 1 only in some prices or
  weights, report exactly where it holds and "unresolved" elsewhere.
- **C3 (exponent):** OLS of `log(IC muls)` on `log ℓ` over the five sizes,
  bootstrap over instances; reported against 1/3 and against R3's fitted
  exponent. Descriptive; it moves no claim.
- Wall time is an observation only.

## Validation gate

A `VALIDATION.json` is written only after (1) an independent replay of
sampled IC and rho jobs reproduces answers and counts, (2) an independent
recompute of the headline ratios from `raw/` agrees, and (3) the rho
reference descriptor records R3 as the strongest available
(`strongest_available` is true only if the symmetry check passes).
