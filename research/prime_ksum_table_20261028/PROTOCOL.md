# Prime-field index calculus with a k-sum table (pre-registration, 2026-10-28)

**Status.** Registered before any timed run. Public synthetic curves; no
security claim; no claim against rho.

## Baseline

The prime-field index-calculus row of record is the two-summand pair table
(`ic.pipeline`, `prime-abscissa:size=A`, `mitm:negation_folded=1`) of the
2026-10-15 frontier campaign (`research/ic_frontier_campaign_20261015/`,
`analysis/band-prime.md`): mean `S` at `w = 0` of 12.5, 14.3, 20.6, 32.1,
49.6, 83.7 on the 20–40-bit `a = −3` ladder, fitted exponent
`α = 0.620 [0.557, 0.663]` against `rho-neg` `0.435 [0.331, 0.501]`.

Its cost is the generic two-summand law. At 40 bits the record splits
13 % table build (`A²` additions), 87 % relation search (`p/(2A²)` trials a
relation, about `0.41·A` relations: the first usable cycle of the relation
graph), linear algebra negligible. `A = 3251` is within 12 % of that law's
optimum. Large primes give the same law for `m = 2` (table `M`, cost
`M + 0.2·p/√M`) and are not pursued.

## Change

`ksum:m=k` (`k = 3, 4`): a negation-folded table of every signed `k`-sum
of the base (`2^{k−1}·C(A, k)` entries), one probe per walk target, `k − 1`
additions to confirm a key match. Generic law
`2^{k−1}·C(A,k) + c_k·A·p / (2^k·C(A,k))`, exponent `k/(2k−1)`: `3/5` for
`k = 3`, `4/7` for `k = 4`, against `2/3` for the pair table. Still generic:
the family converges to baby-step giant-step as `k` grows and cannot pass
it. **Class if it lands: engineering** (the prime IC baseline's `S` falls;
no generic bound moves).

## Predictions (model, `c₃ = 0.92`, `c₄ = 1.0` relations per base abscissa)

| bits | pair `S` (record) | `k = 3`: `A`, `S` | `k = 4`: `A`, `S` |
|--:|--:|--:|--:|
| 20 | 12.5 | 16, 5.8 | 10, 5.2 |
| 24 | 14.3 | 27, 7.4 | 14, 5.9 |
| 28 | 20.6 | 46, 9.5 | 21, 6.9 |
| 32 | 32.1 | 80, 12.4 | 30, 8.1 |
| 36 | 49.6 | 138, 16.3 | 44, 9.7 |
| 40 | 83.7 | 239, 21.5 | 64, 11.6 |

`c₃` and `c₄` are unmeasured (the pair table's `0.41` was); they are read
off the runs and reported.

## Selection (train panel, as the frontier's A4 rule)

Target seed `20261015100`, 4 targets per size, grid
`A ∈ round(A* · {0.5, 0.7, 1, 1.4, 2})` around the model optimum `A*`,
extended until the optimum is interior. Pick the smallest mean `S` at
`w = 0.1` among candidates verified on all four targets. Freeze in
`params.json` before the test panel runs.

## Test panel

The frontier's six prime curves, its target seed `20261015200`, 16 targets
× 2 rounds, warm-up 1, alternate order, the same arms (`rho-neg`
reference, `bsgs-neg`, `kangaroo`, `ic-pair` at the frozen frontier sizes,
`rho-neg-aa` control) plus `ic-ksum3` and `ic-ksum4`. Every answer
verified. `ecbench band` at `w = 0, 0.1, 1`; `ecbench bound fit` per arm.

## Success and falsification

- **Success (engineering):** at 36 and 40 bits the `k = 3` or `k = 4` arm's
  `S` is below `ic-pair`'s at every `w` in the band, or its fitted `α` CI
  lies wholly below `0.557`.
- **Falsified:** neither holds. Also abandoned if any run fails
  verification.
- **Reported regardless:** each row's gap to `rho-neg` and to `bsgs-neg`.
  The headline must say the arm is still above BSGS (the family's limit).
