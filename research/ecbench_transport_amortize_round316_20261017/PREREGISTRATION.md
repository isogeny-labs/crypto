# Round 316 preregistration: four-size scaling of the amortized multi-target candidate

Date: 2026-10-17

## Open question

Round 315 measured per-target parity at T=16 on the degree-37 cell only
(0.686x rho [0.630, 0.740]).  Does the amortized per-target advantage
survive at the three larger frozen sizes?

## Frozen inputs

Everything is Round 315's frozen machinery with Round 314's per-size `K`
rule; no parameter was tuned for this round:

- method `ic.transported_logs_hash_multi`, `T=16`, `co_target_seed
  20261017315002`, hash schedule seed `20261014314005`,
  `target_max_attempts 2048`;
- `K` per size from Round 314's frozen rule: n37=11, n41=71, n43=21,
  n47=43 (a=0 for n37/n41, a=1 for n43/n47, the registered constructor's
  usable curves);
- holdout target seed `20261017315003`, measurement seed `20261017315004`,
  8 public targets, 5 rounds, 1 warmup, `rho.signed_frobenius_strong`
  reference with Round 314's parameters.

The three new cells were specified mechanically from these frozen values
before any was run.

## Gates

- every record verifies and replays (`ecbench verify --replay-all`);
- exact counted A/A;
- amortized per-target ratio with a 95% paired interval excluding 1.00
  at each of the three new sizes;
- exponent fit of the amortized per-target GAE against rho's over the four
  sizes, reported next to the point estimates.

Failing the ratio gate at any larger size closes the scaling question
negatively: the Round 315 parity is then size-bound, not a crossover.

## P-256 amortization economics (no new measurement)

Round 312's imported projection prices the shared, target-independent
relation harvest at `2^165.155613` FME for 138,031 rows and the
per-relation descent at `2^148.080980` FME.  Amortizing the harvest over
`T` targets gives a per-target cost `2^148.081 + 2^165.156 / T`.  Parity
with a rho at `~2^128` per target requires `T >= 2^37` targets sharing one
relation harvest, and the free-relation-oracle floor of `394.425280 x
rho` bounds the best case regardless of `T`.  Multi-target amortization
therefore cannot rescue the P-256 Dickson line at any realistic target
count; this section records the arithmetic, it does not re-measure anything.
