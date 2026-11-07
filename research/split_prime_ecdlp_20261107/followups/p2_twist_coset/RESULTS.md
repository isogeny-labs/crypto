# P2 results: twist-pure coset selection (run-20261107T214118Z)

**Outcome (frozen rule 4): toy-scale gain confirmed, at stage level**
[pcc:unverified]. On prime-order curves over the four preregistered toy
primes, the F4 cost of the relation system does not depend on which coset
of the 2-power base is used, exact relation coverage follows c_E^m, and
choosing the most E-pure coset among C candidates raises relations per
unit of F4 work by (c_sel/c_rand)^m ≥ 2.25 at t = 4. The gain decays toward
1 as the base grows (as predicted), so it is a constant factor that vanishes
at cryptographic size. `S`, end-to-end cost and any rho comparison are unset.

- Protocol: [`PROTOCOL.md`](PROTOCOL.md) (frozen 535d9da826; Addendum A and
  Clarification A.1 before any claim cell; all merged in #3657).
- Card `CARD-p2-twist-coset-20261107-1` (sha256 `d75b1445…88dd`), receipt
  `cards/p2-run-01.receipt.json`, director assessment
  [`cards/p2-run-01.assessment.md`](cards/p2-run-01.assessment.md):
  **executed as specified** (all 7 card commands exit 0; 48 + 108 + 24 calls,
  144 msolve cells, none capped, timed out or unemittable; every output hash
  matches the receipt).
- Raw: `runs/run-20261107T214118Z/{m1,m2,m3,m4}.jsonl` and per-cell logs.
- Analysis: `analysis/p2analyze` (native Rust; applies the frozen rules) →
  `analysis/derived/*.tsv` and `decision.json`. Figure: `figures/p2_summary.svg`.
- Host: this cloud container, isolation L0; only exact counts decide.

## Measurements against the frozen rules

| rule | test | result |
|---|---|---|
| 1 lemma application | nontrivial E-target relations with a twist part ≠ O, exhaustive: S_3 at p ∈ {1009, 2003, 4001, 7001, 9973}, S_4 at p ∈ {101, 151, 199}, 3 curves each | **0 violations** in 24 runs |
| 2 twist columns inert | F4 work, D, widest matrix: selected vs random coset at equal (m, t), 12 groups × 4 targets | max relative difference **0.39 %** (A/A between two random cosets: 0.45 %); D and widest matrix **identical** in every group; kill line 10 % |
| 3 coverage law | coverage(selected)/coverage(random) vs (c_sel/c_rand)², 36 cells | max deviation **0.93 %**; 0 cells beyond 20 % |
| 4 gain | (c_sel/c_rand)^m at t = 4, curve 0, 8 groups | **2.25 to 125** (≥ 1.2 required) |

M1 (order statistic, 36 cells): the observed c_max/c_mean is within 10 % of
the exact binomial expectation in every cell (largest deviation 7.1 %). Means
over the 12 cells per t: observed 1.878 / 1.458 / 1.225 against exact
1.886 / 1.471 / 1.230 at t = 4 / 6 / 8.

## Reading

- **Gain against the frozen random arm vs the expected gain.** The rule-4 number
  uses one seed-drawn coset as "random"; on the Kummer prime 1,049,089 that coset
  happened to hold only 3 E-columns of 16, which inflates G to 25 (m = 2) and
  125 (m = 3). The expected gain against a typical coset is (c_max/c_mean)^m:
  about 3.5 (m = 2) and 6.6 (m = 3) at t = 4, about 2.1 and 3.1 at t = 6, about
  1.5 and 1.8 at t = 8 (derived from the M1 means).
- **Why it works.** F4 sees the tower equations of the coset, which have the same
  leading forms for every constant c; which roots are E-abscissae is invisible to
  it. Twist roots cost columns but never yield an E-target relation (rule 1, and
  `twist_part_sum_eq_zero` in aburan28/crypto-lean#3).
- **Selection cost.** C · 2^t Legendre symbols (about 3.2·10^6 to 3.8·10^6 field
  multiplications at t = 4) against a relation-search cost of
  W_F4 / coverage ≈ 2.6·10^5 / 1.7·10^-5 ≈ 1.5·10^10 F4 mul-adds per relation
  (Kummer 1,049,089, m = 2): the selection repays itself before the first relation
  (`analysis/derived/rule4_gain_and_tstar.tsv`, T* < 10^-2 relations; derived, and
  it mixes field multiplications with F4 mul-adds as if equal).
- **Scope.** A stage improvement of 2-power-base relation search on prime-order
  curves at 2^20 and 2^24 [pcc:unverified]. It changes no exponent: c_max/c_mean
  → 1 as 2^t grows, and the curve-independent F4 cost is untouched. It is not an
  end-to-end comparison and moves no leaderboard row; the validation gate
  (replay certificate, validator report, matched rho) has not been run.
- **Unexpected.** `m2_distinct_nonzero_R` equals 2·c_E² except in large cells, where
  pair sums collide (expected at c_E ≥ 30); the coverage ratio still holds there.

## Caveats

- The card cannot bind the binary it builds, so a post-run `task_card.py lint`
  flags the five binary-calling commands; the run recorded the built binary's
  hash (`47c827e3…c9ec`). Card-authoring limitation, not an execution deviation.
- The relay added one harness line (the `BENCHREC=` prefix the repository hook
  requires); the receipt's argv shows the card's command unchanged.
