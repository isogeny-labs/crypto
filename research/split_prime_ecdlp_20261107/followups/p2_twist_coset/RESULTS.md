# P2 results: twist-pure coset selection (run-20261107T214118Z)

**Outcome (frozen rule 4): toy-scale gain confirmed, at stage level, smaller
than first written** [pcc:unverified]. On prime-order curves over the four
preregistered toy primes, the F4 cost of the relation system on planted
targets does not depend on which coset of the 2-power base is used, exact
relation coverage follows c_E^m, and choosing the most E-pure coset raises
relations per unit of F4 work. The frozen rule-4 statistic, (c_sel/c_rand)^m,
is ≥ 2.25 at t = 4. After the red-team correction below, the gain that matters
for an index calculus is (c_sel/c_mean)^(m−1): about **1.9 (m = 2) and 3.5
(m = 3) at t = 4**, decaying toward 1 as the base grows. It is a constant factor
that vanishes at cryptographic size, and a transfer of the lab's existing
frame-choice lever to prime-order curves, not a new mechanism. `S`, end-to-end
cost and any rho comparison are unset.

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

## Red-team corrections (research/split_prime_ecdlp_20261107/followups/red-team-p1-p2.md)

1. **Exponent m − 1, not m (high, unpriced in the first draft).** An index
   calculus needs about as many relations as base elements, here c_E. A larger
   c_E raises relations per F4 solve as c_E^m but also the number of relations
   needed, so the net relation-stage gain is (c_sel/c_ref)^(m−1). Against the
   frozen random arm that is 1.5 to 25 (not 2.25 to 125); rule 4 (≥ 1.2) still
   holds. Against a typical coset, (c_max/c_mean)^(m−1) is about 1.9 / 3.5 at
   t = 4, 1.46 / 2.1 at t = 6, 1.22 / 1.5 at t = 8 (m = 2 / 3).
2. **The random arm is the wrong reference.** One seed-drawn coset per cell is a
   high-variance reference (on Kummer 1,049,089, five of six idx0/idx1 draws hold
   ≤ 4 of 16 E-columns); the red team found the seeds follow the frozen rule and no
   cause. c_mean is the reference used above. The verdict does not depend on it.
3. **F4 cost was measured on planted targets only.** Real decomposition attempts
   almost never have a solution; whether F4 cost on unplanted targets is also
   coset-independent is untested. Cheapest test: four unplanted targets per arm at
   m2 t4 and m3 t4.
4. **Not a new mechanism.** The Kummer half repeats the frame-choice lever of
   `research/curve25519_frame_search_20261103`; the new part is the prime-order
   setting, the Dickson-torus half, the exact purity check and the decay law.
5. **Analyzer errors that do not change the outcome:** the T* column for the m = 3
   rows of `analysis/derived/rule4_gain_and_tstar.tsv` uses m = 2 coverage (wrong;
   ignore those T* values); the m = 3 "125" is the formula, the measured coverage
   ratio is 102.9.

## Reading

- **Gain.** See the red-team corrections above: the relevant number is
  (c_max/c_mean)^(m−1), about 1.9 and 3.5 at t = 4.
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
