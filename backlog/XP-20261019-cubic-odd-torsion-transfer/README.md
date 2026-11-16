# L1: Cubic Weil restriction: actual odd-torsion kernels and quotient transfer

Status: **draft, not running**. This is an experiment specification, not a measured result. The executable producer/verifier and checkpoints marked TODO in the card do not yet exist.

## Question and decision

Can a Frobenius-stable odd-torsion kernel on an actual cubic Weil restriction produce a verified rational quartic transfer?

The exact module screen has positive and zero cells; source torsion, polarization and rational descent determine whether a candidate can become a transfer. A positive construction opens a route beyond the existing full-two-torsion GHS cohort; a negative result rejects only its named kernel and field.

## First discriminating experiment

Replay the published q=523 example of Tian (2012.07173), including source order, actual 3-torsion, Frobenius-stable isotropic kernel and rational quartic. Register the exact curve/subgroup before executing it. Verify dual composition and scalar transport on 100 distinct public known-answer targets. Then construct a disjoint small-field cohort before any larger comparison.

## Controls

Positive control: the published kernel/quotient fixture. Negative controls: change one kernel coordinate, use a non-isotropic subspace, and use a kernel not stable under Frobenius. Each must reject. Do not infer the actual module from trace alone.

## Acceptance and negative outcomes

Pass the construction gate only with source-order and kernel certificates, smooth rational quartic, non-product polarization/descent evidence, exceptional-point handling and checked map action on the exact prime subgroup. Then compare cold source rho, destination generic solving and specialized quartic IC, including kernel search and transfer. No speed estimate is currently measured.

## Execution readiness

Freeze exact identities, seeds, protocol, runtime and resource budget before execution. Keep setup, online work and verification separate; preserve complete cold totals and failed attempts. Native Rust performs all mathematics. The card remains draft until code, independent checker and resume path exist; estimates are intentionally unknown. No cloud job has been started for this card.

Source: [https://arxiv.org/html/2012.07173](https://arxiv.org/html/2012.07173). This source motivates the mechanism; the specific proposed experiment above is this search round's design. See the [search report](../../research/new_leads_20261019/README.md).
