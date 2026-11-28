# Requirement ledger

Statuses describe the 2026-11-19 worktree. `implemented-unverified` means a
mechanism or contract exists but no admitted result has supplied the evidence.
`blocked` names a missing upstream result. Nothing in this ledger is a frontier
promotion `[pcc:unverified]`.

| Requirement | Status | Evidence now | Exact promotion condition |
|:--|:--|:--|:--|
| Cross-lane v1 schema | implemented-unverified | `manifest.schema.json` | independent consumer parses the same manifest and rejects unknown fields |
| Native semantic validator | implemented-unverified | `src/main.rs` | reviewed build passes unit, self-test, and corruption gates on the bound revision |
| Create-new authorizer | implemented-unverified | `authorize` subcommand | complete non-test bundle writes one content-bound authorization; blocked bundle writes none |
| Exact source revision | blocked | `revision=null` | published 40-hex revision and clean checkout receipt |
| Binary/image identity | blocked | absent | executable SHA-256, immutable OCI digest, and launcher verification agree |
| Protocol/spec freeze | blocked | absent | exact protocol/spec hashes precede all measured data |
| WS3 parallel rho | implemented-unverified | upstream work in progress | production D00–D10 plus admitted worker panel and exact replay |
| WS4 composition | implemented-unverified | upstream work in progress | baseline/R17/R36/R64 composition passes counted/counter-free and holdout gates |
| WS5 matched GHS grid | implemented-unverified | upstream work in progress | every frozen q/seed/repetition cell is retained and independently recomputed |
| WS6 solver census | implemented-unverified | upstream work in progress | live inventory/audit covers every current registered implementation or names the gap |
| Cold one-target scope | blocked | no session | both arms solve the same unseen point with target-independent setup charged once |
| Strongest matched rho | blocked | no session | same curve, subgroup, target, resource envelope, and usable symmetry |
| Complete phase census | blocked | no session | all five rho and all five IC phases are charged with exact sums |
| L2/L3 isolation | blocked | no EKS receipt | every wall pair earns L2 or L3; excluded records remain in evidence |
| A/A admission | blocked | no session | at least five admitted pairs; 95% interval contains 1 and lies in `[0.9,1.1]` |
| Wall outside noise | blocked | no session | claimed A/B interval is outside the admitted A/A noise floor |
| Pricing band | blocked | no session | complete `0,0.1,1` GAE band on both arms under current table-work rules |
| Host calibration | blocked | no session | 600 interleaved triples per seed, positive calibrated weight, identical use on both arms |
| Scalar correctness | blocked | no session | every completion independently satisfies `[k]G=Q`; no wrong answer accepted |
| Native replay-all | blocked | no session | every deterministic run is identical and the audit is `OK` |
| Independent host replay | blocked | no second host | distinct host class replays both arms and checks every recovered scalar |
| Raw sealing | blocked | no raw data | immutable raw manifest plus object hashes/byte counts |
| Independent recomputation | blocked | no result | producer-distinct validator recomputes every headline from sealed records |
| `VALIDATION.json` | blocked | not created | names source/session/replay/validator/matched-reference identities and is verified |
| PCC standing | blocked | unverified | exact-scope claim has signed `verified-ci-main` standing |
| Benchmark record/export | blocked | no run | record closed; export verifies all referenced files |
| Reports and visuals | blocked | no run | run/result reports, PDF, SVG, and PNG include failures and nulls |
| Frontier generator | blocked | no admissible bound | `ecbench frontier build/check` passes from committed records |
| Leaderboard/browser generators | blocked | no result | generated outputs and `--check` agree with frozen sources |
| Dashboard coverage | blocked | no result | coverage check and progress timeline pass without hand edits |
| Current blocked fixture | complete at fixture scope | `fixtures/current-blocked.json` | native `check` exits 3 and names every absent gate |

The unresolved items remain required. A plan, local correctness test, draft PR,
or merged implementation cannot change their status by itself.

