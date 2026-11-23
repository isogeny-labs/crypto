# Bounded rank-directed jet completion — DRAFT

## Question and decision

For the initial `L=4,8` problem, does any rational coefficient matrix `H` with `deg H <= 2L-1` produce a lower precision-adjusted crossing-mode cost than the original kernel or the ordinary flattening control, while supplying entry-by-entry integral `U,V` factorization witnesses at every retained node and stipulated precision?

A valid witness would justify investigating a larger uniform construction. A negative result under a declared coefficient and denominator budget would constrain this finite search domain. Neither result by itself proves an asymptotic gain, a DLP solver, or a runtime improvement.

## Frozen inputs proposed for a future protocol

- Initial sizes: `L=4,8`.
- Exact precisions: `8,12,16` at each retained level.
- Candidate family: `W_new(T)=W_base(T)+C(T)H(T)` with `deg H <= 2L-1`.
- Controls: `H=0` (original kernel), the ordinary first-jet flattening construction in the study, and a negative control that rejects an invalid even-denominator inverse.
- Required output: candidate polynomial plus explicit integral `U,V` factors, reconstructed entry by entry at every retained node and each requested precision. Rank/minor counts alone do not pass.
- Before execution: specify rational coefficient encoding, coefficient bounds, denominator budget, crossing-cost metric, and exact verifier; charge polynomial degree and precision. Keep all failures and invalid candidates in the output.

The source study also considers a nonscalar origin channel `R(T)` with low weighted crossing rank. That proposal is related motivation, but this card's concrete search is the rank-directed free-jet family above; a successful result would not certify the separate `R(T)` proposal.

## Prerequisites and execution state

The native runner `target/release/jet_completion_search`, witness schema, independent verifier, frozen protocol, and defensible resource estimate do not exist yet. The planning envelope in `card.json` is provisional, not a measured resource estimate. This card is **draft, unfunded, and not scheduled**: no Cairn space, task, lease, reservation, or run has been created. Do not execute until the missing implementation and protocol are reviewed and resource/budget authorization is explicit.

For context on the existing finite observations and their limits, see the [study report](../../research/prime_jet_completion_20261030/reports/review-followup-20261101/REPORT.md) and [study README](../../research/prime_jet_completion_20261030/README.md).
