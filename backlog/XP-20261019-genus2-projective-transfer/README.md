# L5: Projective genus-2 transfer pipeline with matched source arithmetic

Status: **draft, not running**. This is an experiment specification, not a measured result. The executable producer/verifier and checkpoints marked TODO in the card do not yet exist.

## Question and decision

Can a checked projective or Kummer-assisted backend reduce the actual cost of the genus-2 transfer pipeline?

The verified pipeline uses a generic affine Jacobian backend whose addition was counted as 70M+3S+I. Faster arithmetic is a distinct implementation lead, not a claim that the observed IC-to-rho gap disappears. Favorable inversion prices alone do not prove a backend improvement.

## First discriminating experiment

Check which complete genus-2 Jacobian formulas apply to each of the 42 exact models. Implement a generic projective reference first; test a Kummer arm only if its rationality and recovery prerequisites hold. Replay the entire 1,260 exact-order cohort and retain per-model applicability. Materialize divisor coefficients at the same points where splitting/root extraction needs them.

## Controls

Controls: existing affine backend, identical source points and relation seeds, exceptional divisors, inverse pairs and infinity; matched projective/batched source rho. Keep sign recovery, coordinate conversion, normalization, root work, memory and failed transfers inside the charged pipeline.

## Acceptance and negative outcomes

Pass arithmetic identities and exact source answer checks first, then complete cold comparisons at every size. Publish measured per-operation formulas and total instructions separately. A Kummer pseudo-addition alone is insufficient for arbitrary r-adding walks; full addition or verified recovery must be available and charged. Qualified wall time requires L2 and A/A controls.

## Execution readiness

Freeze exact identities, seeds, protocol, runtime and resource budget before execution. Keep setup, online work and verification separate; preserve complete cold totals and failed attempts. Native Rust performs all mathematics. The card remains draft until code, independent checker and resume path exist; estimates are intentionally unknown. No cloud job has been started for this card.

Source: [https://arxiv.org/abs/1510.03174](https://arxiv.org/abs/1510.03174). This source motivates the mechanism; the specific proposed experiment above is this search round's design. See the [search report](../../research/new_leads_20261019/README.md).
