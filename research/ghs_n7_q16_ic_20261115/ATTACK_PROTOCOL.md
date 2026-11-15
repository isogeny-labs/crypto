# Frozen exact q16 GHS index calculus versus rho protocol

15 November 2026. Freeze after the selected-subgroup [map receipt](receipts/map1.json), before any relation or target-decomposition run. This is one exact-instance exploratory comparison inside Teske's published genus-eight family. Its outcome cannot by itself classify the whole family. [pcc:unverified]

## Exact input and gates

Use the field, source equation and point in [MAP_PROTOCOL.md](MAP_PROTOCOL.md), prime `r=44735531`, and public target `Q=1234567P`. Recheck source order, `rP=O`, a genus-eight hyperelliptic target over F16, `G=h(P)≠0`, `rG=0`, and `T=h(Q)=1234567G`. Compute `N=#Jac(C)(F16)` inside the timed GHS setup. Require `N` divisible by r with `gcd(N/r,r)=1`; set the r-primary projection multiplier `m=N/r`. If this fails or the Jacobian-order computation does not finish, record a censored run rather than solving a different subgroup. [pcc:unverified]

## Relation, recovery and stopping rules

- Use the same degree-at-most-two Mumford factor-base rule as the q4 run. For successive nonzero multiples `kG`, factor the first Mumford polynomial. Accept a relation only when every irreducible factor has degree at most two and the sum of its divisor classes reconstructs `kG` exactly. Record attempts, accepted rows, encountered factor-base classes, and reconstruction failures. Stop after at most 1,000,000 multiples or 90 CPU seconds in the relation phase; once at least 20 surplus relations are present, stop early only when the matrix has full column rank.
- Solve the complete accepted system modulo r. Require full column rank and a unique consistent solution. For every factor-base class `B_i` and solved coordinate `L_i`, check the exact Jacobian equality `m B_i=(m L_i)G`. These are projected r-primary coordinates; do not describe a class outside `<G>` as possessing a literal logarithm to base G.
- Descend the nonzero mapped target using `T+jG` with `0≤j<min(r,1,000,000)`, at most 45 CPU seconds. Skip the identity. Accept only exact divisor reconstruction using factor-base classes from the solved system. Derive the scalar modulo r and check it on both `E` and `J`. Otherwise retain the censored descent arm. [pcc:unverified]
- Run a deterministic three-way Floyd Pollard rho on the same `P,Q` in the same Magma process, with explicit walk steps and restarts. Time 100 repeated verified solves to resolve timer granularity. The known scalar 1,234,567 is a fixture used only by setup assertions and final checks; neither search routine branches on it.

## Cost and audit

Time rho and GHS setup, relations, modular linear algebra, target descent and final verification with Magma `Cputime` and `Realtime`; retain service wall time separately. Set the public client timeout to 180 seconds and print exact relation rows and target indices. Use a separate Python auditor to replay the modular equations and target arithmetic. Compare complete one-target GHS CPU with the mean of 100 rho solves only if both succeed; otherwise report a censored arm and its measured lower bound. Keep every failed response immutable. A speedup claim would require a replicated below-rho result, a fair optimized baseline, and prior-work priority checks. [pcc:unverified]

Sources checked: [Teske's selector](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf), [Magma Jacobian API](https://docs.magma-maths.org/v2.30-1/ArithmeticGeometry/HyperellipticCurves/points-jac.html), and KB revision `9141e386254340a9bdba3036f2bb706a`, especially `source:8618328474ca57d3a61b1a8832d41daf` (known binary composite GHS). The exact-parameter search is documented in [SCREEN_REPORT.md](SCREEN_REPORT.md).
