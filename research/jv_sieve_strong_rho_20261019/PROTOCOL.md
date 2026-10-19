# Protocol (frozen 2026-10-19, before any measured data of this study)

Question: how does the Joux–Vitse sieve on E(F_{p⁶}) (weak class: full rational
2-torsion, never prime order; Jacobian of genus **3**) compare with a *fair*
Pollard rho on the same instances, and does its cost grow as ℓ^{1/3}?

Class: measurement of an existing algorithm against a stronger reference
(accounting/engineering, not a new algorithm). A result is "an improvement" only
as the measured gap to the strong rho below.

## Instances
`generate_spec(p, seed)`, seeds 1..16, p ∈ {251, 503, 1009, 1511, 1823}
(ℓ ≈ 2^45.8, 2^51.8, 2^57.9, 2^61.4, 2^63.0). IC and every rho arm use the same
instances on the same host. Challenge curve = the weak curve handed to the sieve
(ledger §18.4 prices the isogeny walk separately; not re-measured here).

## Arms
IC: `jv_cover --exp sieve --stop 64`, `JV_LIN=dense` and `JV_LIN=sparse`
(paired, same seeds; descent digest must be equal).
Rho (`jv_rho_strong`, r = 256 multipliers in every arm, DP bits 10/12/14/15/15 by p):
R0 no negation, one inversion per step (the ledger's reference, re-run in this
engine); R1 batched inversion k = 16; R2 negation map (window-8 cycle detection,
doubling escape); R3 negation + batched inversion (**the strong rho**).
Runs: 16 per arm at p = 251, 503, 1009; R3 ×16 and R0 ×8 at 1511; R3 ×16 and R0 ×4 at 1823.
Every run is checked against the planted logarithm.

## Prices (F_p multiplications; GAE = 331 = the harness affine addition)
Harness unit: F_{q³} product 55, squaring 55, inversion 166 (measured from `Fq3`).
Tuned unit (rho only, adversarial to the IC): 24 / 18 / 62.
S = muls / (331·√ℓ). S⁺ = every F_p addition and native operation also charged at
one multiplication (IC: sieve adds, lookups, enumeration adds; rho: group-law
F_p adds, canonicalisation, hash, DP check, scalar updates, table operations,
cycle comparisons).
§5a band: native work at w ∈ {0, 0.1, 1} GAE per operation on every arm, two
layers (A: all non-multiplication counted work incl. F_p adds; B: excluding F_p
adds). IC adds inside F4/linear algebra are **not emitted**; they are bounded by
one add per multiplication outside the sieve/enumeration and reported as a bound.

## Controls
A/A: R0 and R3 at p = 1009 re-run with run index 1 (new walk randomness, same
instances); the two samples must not differ beyond their bootstrap intervals.
IC A/A: sparse at p = 1009 run twice; counts must be identical.
Positive control: the weak arm R0 must reproduce the ledger's rho S ≈ 1.3.
Negative control (isolating factors): R1 vs R0 isolates batching, R2 vs R0 negation.

## Statistics
Per size: mean, bootstrap 95% interval (10⁴ resamples over instances). Ratio of
means IC/rho with bootstrap interval. Exponent: OLS of log(muls) on log(ℓ) over the
5 sizes, bootstrap over instances within size; compared with 1/3 (IC prediction)
and 1/2 (rho, to validate the instrument).

## Kill / narrow rules (fixed now)
K1. If the 95% interval of S⁺_IC / S⁺_R3 (harness price both arms) reaches 1 at any
p ≥ 1009, the claim "the sieve beats rho end to end" is killed at that p.
K2. If it reaches 1 only in the tuned-rho price or only inside the §5a band, the
claim is narrowed to the prices where it holds, and unresolved elsewhere.
K3. The ℓ^{1/3} claim is narrowed if the fitted exponent's interval excludes 1/3.
Wall time is an observation only.
