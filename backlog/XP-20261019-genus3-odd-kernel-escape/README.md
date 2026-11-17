# L2: Odd-torsion exits from Smith-negative genus-3 inputs

Status: **draft, not running**. This is an experiment specification, not a measured result. The executable producer/verifier and checkpoints marked TODO in the card do not yet exist.

## Question and decision

Do odd-prime isotropic quotients reach a non-hyperelliptic solver from inputs rejected by the current Smith two-torsion route?

This tests a different quotient kernel, rather than relabeling another walk through the already-audited two-torsion class. A verified exit changes reachable solver coverage; failure is restricted to the tested curves, primes and fields.

## First discriminating experiment

Replay Tian 2007.03172 section 6 over F_257 first. Then take a frozen holdout of 20 Smith-negative odd-characteristic genus-3 inputs from the existing census, screen ell=3 and 5, and attempt a quotient only when its actual torsion module passes. Freeze exact IDs and sample selection before running; no preference for easy successes.

## Controls

Positive control: published genus-3 3-isogeny fixture. Compare with the existing two-torsion/Smith route on the same inputs and include a quotient that is geometrically defined only over an extension as a descent rejection control.

## Acceptance and negative outcomes

Coverage pass requires one verified rational non-hyperelliptic destination and subgroup-injective executable map on a held-out Smith-negative input. Record every rejected kernel, extension-field requirement and failed attempt. Only a complete source-to-answer comparison can establish a useful cost change.

## Execution readiness

Freeze exact identities, seeds, protocol, runtime and resource budget before execution. Keep setup, online work and verification separate; preserve complete cold totals and failed attempts. Native Rust performs all mathematics. The card remains draft until code, independent checker and resume path exist; estimates are intentionally unknown. No cloud job has been started for this card.

Source: [https://arxiv.org/html/2007.03172](https://arxiv.org/html/2007.03172). This source motivates the mechanism; the specific proposed experiment above is this search round's design. See the [search report](../../research/new_leads_20261019/README.md).
