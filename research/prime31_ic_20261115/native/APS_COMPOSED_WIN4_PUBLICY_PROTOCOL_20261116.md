# EXP-R31-APS-COMPOSED — prospectively frozen before implementation

2026-11-16 UTC; [pcc:unverified]. Scope: ordinary cofactor-one PRIME31 curve over p=2147483647, order n=2147446831, same existing generator, Rust native implementation in `examples/prime31_aps_one_relation.rs`, same factorized Semaev S3 pair-root oracle and the packed x index. No new algorithmic novelty is claimed: APS is published index calculus, with explicit pair-support enumeration still generic meet-in-the-middle equivalent.

## Evidence and source review before experiment

- AGENTS.md and the shared 2026-11-16 session-log policy (the canonical log is currently too large for the GitHub connector's read response; do not replace it).
- Merged #4391, frozen 48-Q fully cold APS 109.734280 ms and unfolded rho64 99.473873 ms; signed PCC still unavailable.
- Open #4451 exact D4/S9 zero-fibre work with successful source-specific CI 43961833386 and 43962417379; mathematically separate from performance.
- Current #4454's *separate target panels*, exact-source successful Actions 43972364656: window4 72.911111 -> 70.035338 ms over 24 Q, 20/24 wins; public-y atop window4 66.855314 -> 64.404234 ms over a DIFFERENT 24 Q, 24/24 wins. Both fail their separately preregistered 5% whole-call improvement gates. On those two panels the respective stronger unfolded rho arms took min(74.631259,72.290834)=72.290834 ms and min(66.371901,73.884110)=66.371901 ms. No cross-panel speedup multiplication permitted.
- Primary published prior art: Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves* (Finite Fields Appl. 51, 2018, DOI 10.1016/j.ffa.2018.01.009); Kudo–Yokota–Takahashi–Yasuda, *Acceleration of index calculus for solving ECDLP over prime fields and its limitation*, established hybrid/symmetry variant. Refer to #4391 for the existing PKM 2016 baseline.
- Shared research-KB MCP and EKS/Cargo are unavailable here; repository/GitHub Actions/primary publisher sources used instead. Absence of a new search hit is not evidence of novelty.

## Concrete hypothesis

The window4 Jacobian public-jump optimization and the public y-ordinate reuse operate in different stages, and *may* give a combined 5%-or-better complete-cold improvement over binary joint-Jacobian APS **on exactly the same fresh target set**, even though each separate 24-Q experiment failed its individual 5% gate. This is an engineering question only. The earlier separate panel timing ratios cannot be multiplied to claim a gain.

## Frozen design

Use **24 fresh distinct hidden-from-solver known-scalar public Q** drawn with deterministic SplitMix64 seed `0x434F_4D50_4150_5331`. Fix rho base seed `0x434F_4D50_5248_4F31`, preexisting public-curve equation and generator, S3 APS partitions `[384,384,16384]`, six complete rounds. Rotate execution order by case index modulo six. Six cold independent arms for each public Q:

1. Existing binary joint-Jacobian APS with sqrt lifts (independent incumbent).
2. 4-bit-window Jacobian APS with sqrt lifts (stage-control).
3. 4-bit-window Jacobian APS using public y ordinates (composed candidate).
4. Identical composed candidate A/A repeat (measurement stability).
5. Existing complete unfolded rho32, no negation-fold stagnation, preexisting DP and cap.
6. Existing complete unfolded rho64, same protocol and cap.

Every arm pays fresh jump tables, field inversions, factor-base partitions, S3 work, rootfinding, misses, rho setup, walks, collision work and scalar checks. No amortization, no fixture scalar supplied to a solver. An external caller verifier checks all recovered `k` via original point scalar multiplication. Compare source S3 pair-quadratic counts, signed witness arrays, round/seed counts, and solution, requiring **identical transcript** for all four APS arms, while allowing zero S3 square roots in the public-y variant. Record all failed rho arms/caps; do not silently drop failures. Persist per-case chronologies, all-arm cold ns, source-setup ns, S3-query ns, sqrt counts, solution and verification evidence.

## Before-data acceptance criteria

- **Completeness:** 24/24 each of four APS arms and both rho arms; 144/144 total for a valid all-arm comparison. Partial results remain explicit and do not establish a complete-rho advantage.
- **Engineering gate:** composed/binary aggregate wall <= 0.95, composed faster on >= 18 of 24 paired Q, and composed A/A total drift <= 5%; exact APS transcript preservation and 96/96 correct APS scalars mandatory.
- **Generic-control disclosure:** report composed/rho32 and composed/rho64 aggregate ratios and paired wins; even a measured win does not establish strongest-Wiener–Zuccherato or external frontier superiority. A 24-case paired win count near half argues against declaring robust generic superiority.
- **Scientific classification:** do not claim a new non-generic algorithm or security weakening. The next non-generic goal remains target-conditioned S9/K961-S5 extraction without generic four-sum enumeration. No leaderboard promotion without independent signed PCC, reference-tool and isolation gates.
- **Negative-result rule:** failed/ambiguous gate is a frozen result, not a reason to change seeds or thresholds after seeing data.
