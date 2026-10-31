# Untallied common-cost local results

Date: 2026-10-31

Decision: **the four-size toy gate rejects an exact `n=83` generic-solver
attempt.** Both destination controls are already slower than the strongest
source control at every measured toy size, and the gap grows with `n`. This is
descriptive L0 evidence, not an admitted cross-host performance result or an
impossibility result for a future image-compatible calculus.

## Frozen experiment

The session executed
[`n83-genus3-image-common-cost-l2.json`](../../docs/ecbench/specs/n83-genus3-image-common-cost-l2.json)
without modification from clean commit
`9c88460934be37f5ad4c8af3157f9db43c324f3c`. It used eight deterministic
targets at each of `n=9,13,19,23`, one warm-up and six balanced measured rounds,
and these four arms:

- strongest source control `rho.signed_frobenius_strong`;
- destination-image control `jacobian_image.rho3`;
- destination-image control `jacobian_image.bsgs`; and
- a second source arm as the A/A calibration.

The timing adapters deliberately disable the fine-grained field-operation
tally. The timed route still constructs the cover, pulls the generator and
target into the Jacobian, solves inside the cyclic image, replays the recovered
scalar in the destination, and leaves the runner to verify it independently in
the source.

Session `ECBS1hc9afe04b8e3f` produced 896/896 verified records. All 768 measured
records replayed byte-identically. The audit reports zero problems and has
receipt SHA-256
`a39597e0d5250a612abf4ec2bf0421fb5370be69c359eff98e0c2ae1a5f72a62`.
The production binary SHA-256 was
`5953efb706195b16dad4220187bdcf9413c5c09d4670f726f7e4180a63224afe`.

## Paired local result

Each entry below is the median of 48 paired `solve_wall_ns` ratios: eight
targets times six measured rounds. A value above one means the named arm took
longer than the source control on the same target and round.

| field degree | destination rho / source | destination BSGS / source | source A/A |
|---:|---:|---:|---:|
| 9  | 9.819 | 5.360 | 1.002 |
| 13 | 24.160 | 10.662 | 0.976 |
| 19 | 124.627 | 50.312 | 0.989 |
| 23 | 364.978 | 131.317 | 0.999 |

The A/A medians stay within 2.4% of one. The destination/source ratios grow
sharply rather than approaching a crossover. BSGS is consistently the faster
of the two destination controls, but it remains slower than the source control
at all four sizes.

The separate deterministic
[`FIELD_ACCOUNTING_RESULTS.md`](FIELD_ACCOUNTING_RESULTS.md) ledger reaches the
same qualitative ordering without contaminating these timings. For example,
at `n=23` the complete BSGS route recorded median counts of 284,091 additions,
208,138 multiplications, and 256,769 squarings, while destination rho recorded
1,105,662 additions, 804,010 multiplications, and 963,552 squarings. Inversion
counts are informational because their internal multiplications and squarings
are already included.

## Admission and exact-family decision

Every local record earned only L0 isolation. Performance counters were
unavailable, the host could not evacuate unrelated threads, and some solves
were shorter than the timing floor. Therefore these ratios are useful for the
toy go/no-go decision but cannot satisfy the preregistered L2 net-measurement
gate.

An exact generic attempt at `n=83` is not a useful experiment. The image order
has 81 bits, so both generic controls retain square-root work of roughly
`2^40.5` group steps; extrapolating this implementation would be computationally
infeasible and the four toy sizes provide no crossover signal. The
[`DESTINATION_CALCULUS_SCREEN.md`](DESTINATION_CALCULUS_SCREEN.md) separately
rejects the off-the-shelf full-Jacobian genus-three index calculus: its
`q^(4/3)` ledger is `2^110.67` here and its ambient factor-base relations have
no proved route into the order-`r` image.

The exact `n=83` transfer certificate remains valid. What is rejected is the
solver-speed promotion and an exact generic benchmark, not the pullback,
image-order, exceptional-fiber, push/pull, or scalar-recovery certificates.

## Evidence and remaining remote run

The sealed local evidence is in
[`sessions/common-cost-local-untallied-20261031T070000Z/`](sessions/common-cost-local-untallied-20261031T070000Z/),
including `records.jsonl`, `resources.json`, and the all-record replay receipt.
The same immutable spec is still intended to run once on each registered
`c8a.2xlarge` and `m8a.2xlarge` EKS Spot class. This environment exposes no Iso
Lab submission tool, AWS CLI/profile/identity, or valid GitHub token, so no
paid job was submitted and there is no ambiguous remote state to recover.
