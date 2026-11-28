# Prime31 prospective generic-control experiment: 16/32/64 jump r-adding rho

**Registered 2026-11-16 18:25 UTC, before any source edits or new measurements.** This is a control experiment for the existing complete Rust APS S3 one-relation solver, not a novel cryptographic algorithm.

## Frozen evidence and literature

- Existing 48-Q exact-source [report](APS_RHO_FOLD_SENSITIVITY_48Q_RESULT_20261116.md), [raw](APS_RHO_FOLD_SENSITIVITY_48Q_EXACT_CI_20261116.json), [CI run 43921120382](https://github.com/isogeny-labs/crypto/actions/runs/43921120382): 240/240 complete recoveries; APS 109.734280 ms, unfolded32 101.453752 ms, unfolded64 99.473873 ms; paired bootstrap intervals overlap parity. Folded watchdog rho restarted 1,627 times, so cannot serve as sole generic control.
- Existing independent native reference src/cryptanalysis/ic_boundary.rs::rho.negation has WZ look-ahead and fruitless-cycle escape but **has not been run on these matched Q**. The new jump-width sweep does not impersonate that faster generic reference.
- Historical negative controls: anchored m6 F4 produced no completed rational roots and has <=3.7407208% natural support; S9/K-S5 original-ideal selected-root quotient (#4228 merged; #4349 open) has not recovered natural B31 target-conditioned f. Prior Vieta-conic / tower-F4 / same-field-genus2 negative leads: crypto-autoresearcher Semaev alternatives SUMMARY and ERRATA. Auxiliary direct/special/random IC (#1536) lacks admitted complete cost.
- Primary prior art: Amadori/Pintore/Sala, Finite Fields Appl. 51 (2018), doi:10.1016/j.ffa.2018.01.009; Wiener/Zuccherato, SAC 1998, doi:10.1007/3-540-48892-8_15; Bos/Kleinjung/Lenstra, ANTS 2010, doi:10.1007/978-3-642-14518-6_9.
- KB MCP and wrapper, local Conductor/cargo unavailable; GitHub result files and accessible primary publisher descriptions were used. This is a bounded review [pcc:unverified].

## Prespecified implementation and data contract

Reuse research/prime31_ic_20261115/native/examples/prime31_aps_one_relation.rs in pinned Rust 1.100 release CI. Parameterize the existing unfolded signed-x-certified Jacobian batch rho jump count (power-of-two 16/32/64), preserving the old 16-jump entry-point exactly. Charge jump generation, all walks, restarts, DP table and final scalar replay. Do not replace the Rust APS implementation.

Curve p=2147483647, n=2147446831 prime, cofactor=1, same PRIME31.g. Fresh 24 distinct public unknown-target Q derived by caller-only deterministic seed 0x4a554d5053323431; rho seed base 0x4a554d5053525231, max 2,000,000 charged walk steps, unfolded rho32 lanes / DP4 / watchdog512. APS partition [384,384,16384] / at most six rounds. Five independent cold arms per Q, chronological order rotated five ways: incumbent APS; unfolded rho16; rho32; rho64; rho32 A/A.

Every completed log must equal caller-known k and reproduce [k]G=Q; known k is never input to solver. Preserve source-exact per-case JSON, steps, restarts, memory high water, setup and verification, order and source hashes. Censored cap hits are not counted as successful solves. Cost comparisons are accepted only if 24/24 logs complete for each arm. Record cold total ratios, paired wins, A/A drift (development gate <=5%), and paired bootstrap intervals; do not cherry-pick a winner from repeated tuning.

All results are producer-development [pcc:unverified]. No new asymptotic exponent, external fastest rho claim, independent signed PCC or leaderboard promotion is authorized by a width sweep.

## Next mathematical gate

Regardless of generic-control ranking, prioritize target-conditioned original-ideal-equivalent S9/K-S5 eight-leaf symmetric f(T) extraction at B4/B8 without candidate multiset enumeration; verify positive, negative, repeated-root, infinity and signed-point branches. Then test natural B31 completed unknown-target logs and compare matched complete cold cost to tuned WZ rho and mature PKM/APS/CAS.
