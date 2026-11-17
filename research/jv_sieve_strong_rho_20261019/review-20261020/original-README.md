# The Joux–Vitse sieve on E(F_{p⁶}) against a strong rho (2026-10-19/20)

**Scope: a weak class only.** Curves `y² = h(x)(x−α)(x−σα)` over `F_{p⁶}` with full rational
2-torsion whose isogeny class contains a weak curve (about 55–63 % of such curves, ledger §18.5).
The class never has prime order. The cover's Jacobian has **genus 3**. Nothing here is a statement
about a prime-order or generic curve.

**Class:** measurement of an existing algorithm (the sieve of `src/cryptanalysis/jv_sieve.rs`)
against a stronger reference than the ledger had. Not a new algorithm. Protocol frozen before any
data: [`PROTOCOL.md`](PROTOCOL.md) (deviations log appended at its end). Numbers: [`TABLES.md`](TABLES.md),
[`summary.json`](summary.json); raw runs: [`raw/`](raw/); report: [`REPORT.pdf`](REPORT.pdf).

## What was measured

Instances `generate_spec(p, seed)`, seeds 1–16, `p ∈ {251, 503, 1009, 1511, 1823}`
(`ℓ ≈ 2^45.8 … 2^63.0`; 1823 is the last `p` with `ℓ < 2^63`). The sieve and every rho arm ran on the
same instances and host. Every run was checked against the planted logarithm: all sieve runs
(80 per `JV_LIN` arm) and all 268 rho runs recovered it.

* **Sieve:** `jv_cover --exp sieve --stop 64`, dense and sparse FGLM descent, paired.
* **Rho:** `src/cryptanalysis/jv_rho_strong.rs`, a 2×2 factorial: R0 plain, R1 batched inversion
  (k = 16), R2 negation map, R3 negation + batched (**the strong rho**), all with 256 multipliers and
  distinguished points. Costs are *counted by operation class* and priced twice: in the **harness
  unit** (`F_{q³}` product 55, square 55, inversion 166 `F_p` multiplications, measured from the
  harness's own `Fq3`; one affine addition is 331) and in a **tuned unit** (Karatsuba, Chung–Hasan
  squaring, adjugate inversion; 24/18/62) that credits rho alone.
* `S = F_p multiplications / (331·√ℓ)`. `S⁺` also charges every `F_p` addition, lookup and native
  operation one multiplication, on both sides.

## Results

**1. The sieve still beats a strong rho, from about p = 1009 up, and the gap widens.** Paired
sieve/rho ratios, 95 % bootstrap (n = 16; below 1 is the sieve cheaper):

| p | S, harness | S, tuned | S⁺, harness | S⁺, tuned |
|--:|--:|--:|--:|--:|
| 251 | 18.5 [5.4, 47] | 44 [13, 109] | 37 [12, 89] | 73 [23, 176] |
| 503 | 0.48 [0.35, 0.64] | 1.15 [0.84, 1.53] | 1.34 [0.94, 1.82] | 2.65 [1.85, 3.62] |
| 1009 | 0.024 [0.019, 0.031] | 0.058 [0.044, 0.074] | 0.067 [0.053, 0.084] | 0.132 [0.105, 0.165] |
| 1511 | 0.0090 [0.0070, 0.012] | 0.022 [0.017, 0.029] | 0.028 [0.022, 0.036] | 0.055 [0.044, 0.071] |
| 1823 | 0.0042 [0.0032, 0.0058] | 0.010 [0.0078, 0.014] | 0.015 [0.0125, 0.020] | 0.030 [0.025, 0.038] |

The pre-registered kill rules did not fire: K1 (S⁺ ratio interval reaching 1 at any `p ≥ 1009`,
harness price) and K2 (the same in the tuned price or inside the §5a band) both hold at 1009, 1511
and 1823. Against the cheapest rho in the tuned price (R2, post hoc) the 1009 ratios are 0.062
[0.045, 0.082] (S) and 0.137 [0.104, 0.183] (S⁺). The distance remaining is therefore a factor of
about 65 (S⁺, harness) to 33 (S⁺, tuned) at `p = 1823`.

**2. The crossover moves up against a strong rho.** The ledger's crossover against the weak rho
was `p ≈ 333–430`. Against the strong rho it is between `p = 251` and `p ≈ 503` in S (harness) and
unresolved at 503 in S⁺ or the tuned price, resolved by 1009.

**3. §5a band.** Native work priced at 0, 0.1 and 1 GAE per operation on every arm, rho included,
two layers (A: all non-multiplication work including `F_p` additions; B: without them), the sieve
side carrying an upper bound for the F4/linear-algebra additions it does not emit
([T2](TABLES.md)). The sieve ranks below rho at every weight and in all four columns at
`p = 1009, 1511, 1823` (layer A, `w = 1`, tuned: 0.37 [0.29, 0.46], 0.16 [0.13, 0.21],
0.088 [0.072, 0.11]). At `p = 251` rho ranks below; at `p = 503` the order changes inside the
band, so the pair is **unresolved** there.

**4. The ℓ^{1/3} claim is not supported over the measured range (K3 fires).** Fitted exponent of total
`F_p` multiplications in ℓ, 251–1823: sieve −0.122 [−0.165, −0.084] against the theory's 1/3;
strong rho 0.508 [0.480, 0.538] against 1/2 (the instrument check passes). Post hoc, over
`p ≥ 1009` (constant `m = 9`): sieve 0.052 [−0.015, 0.118], excluding 1/3. The sieve's cost falls
from `p = 251` to `1009` (the sieve runs at `m = 10` at 251–503 and `m = 9` from 1009, and relations per line grow with `p`) and is then about flat, between `2^{31.6}` and `2^{31.9}`
multiplications. It beats rho because rho grows as `ℓ^{1/2}` while the sieve does not grow over
this range, **not** because it follows `ℓ^{1/3}`. Whether the cost starts to rise like `ℓ^{1/3}` beyond
`p = 1823` is untested here (`ℓ` exceeds 64 bits next), so extrapolating on `ℓ^{1/3}` is an
extrapolation with no measured exponent behind it.

**5. What the rho accelerations buy in this tower** (R0 → R2 → R3, [T3](TABLES.md)):
*Negation* is worth about 1.2–1.6× (theory √2). *Batched inversion buys nothing*: an inversion
costs about 3 products here, so Montgomery's trick (3 products) is neutral in the harness price
and slightly worse in the tuned price (R1 0.542 against R0 0.499 at `p = 1009`). The brief's
"2–3× cheaper per step" comes from cheaper field arithmetic (tuned price: 2.4×), not from batching.
Positive control: the plain arm gives S = 1.14–1.45, the ledger's reference ≈ 1.3. A/A at
`p = 1009` (new walk randomness): R0 1.25 against 1.29, R3 0.823 against 0.808, inside their
intervals; the sieve's counts repeat identically on all 16 instances.

**6. Sparse FGLM in the sieve's descent** ([T4](TABLES.md), PR #2244's linear stage, `JV_LIN`):
the descent rows are bit-identical in 80/80 dense/sparse pairs (same digest and outcome). The
linear stage is 2.08–2.20× cheaper, descent multiplications 1.28–1.29×, and the end-to-end `S`
1.014× (251) to 1.126× (1009, 1511) cheaper, 1.096× at 1823 (intervals in T4). The sparse path
fell back to the dense one 0.4–7.5 times per run; that cost is counted. This is an engineering gain
on a route that already beats rho; it moves no ranking.

## Limits

* Weak class only; `p ≤ 1823` (`ℓ < 2^63`). Harness unit throughout; wall time is an observation
  (sieve 4 threads, rho one core: at `p = 1823` the sieve's mean wall is 9 s against 760 s for one
  strong-rho run; not a speedup claim).
* The isogeny walk to reach the class (ledger §13, §18.4) was **not re-measured**; end-to-end from a
  non-weak curve against the strong rho is not asserted here.
* The tuned price is a cost model, not a faster implementation; rho's native work (canonicalisation,
  hashing, DP checks, scalar updates, cycle-window comparisons) is counted and priced in S⁺ and the
  band. The sieve's F4 additions are not emitted by the harness (implementation gap): the band uses an
  upper bound, labelled as such.
* The post hoc fit and the R2 comparison are labelled post hoc and were not in the protocol.
