# 2026-11-15 morning research-lead sweep (08:00 PT)

Public-only, non-atomic metadata and source survey. Survey and scientific interpretation are **unsigned and unverified** under PCC. [pcc:unverified]

## Frozen machine evidence

- Earlier immutable snapshot: [snapshot.json](./snapshot.json) (2026-11-15T04:43Z).
- This dated increment: [sweep-0800-PST.json](./sweep-0800-PST.json), finalized at 2026-11-15T16:18:59Z.
- Lead map: [root leads.log](../../../leads.log).
- Original work item: [crypto #4249](https://github.com/isogeny-labs/crypto/pull/4249). No duplicate sweep PR was created.
- 308 public PRs, nine public repositories: crypto 187 (pages 100,87), cryptanalysis 57, crypto-autoresearcher 56, lfd-summation-systems 4, prototype 2, ecbench 1, ecc-challenges 1, autoresearcher 0, experiments 0. The no-result repositories were explicitly queried. [pcc:unverified]
- Relative to the earlier public freeze: **41 newly open**, **14 no longer open** (not necessarily merged), **16 retained PR heads changed**; current minus previous = 27. Exact identities are recorded in the JSON delta. [pcc:unverified]
- All 57 new/changed public head SHAs received first-page check-run queries. The other 251 heads retained their prior *exact-SHA* check observations and older capture timestamps. Aggregated navigation labels: 140 with some failed check, 133 completed without a recorded failure (including successful/skipped mixes), 23 cancelled, six pending, and six without check runs. These are neither required-check conclusions nor research verdicts. [pcc:unverified]
- Raw repo lists and check-run queries were taken sequentially or concurrently at different instants. There is no claim of one atomic GitHub state. We did not independently retrieve every historical status context, review decision, full job log, or live cloud-worker process. [pcc:unverified]

## Research changes worth investigating

**PKM / native algebraic relations.** The 11:20Z session-log Stage-B record now describes completed G2 and G3 scored stages, not merely a launch: 41 G2 draws/zero violations, 123 serial G3 comparisons/zero disagreements, an explicit temp-file race in the prior parallel G3, and oversize/censored (t=14) cells. This is evidence about the producer's fall-degree experiment, not a full original-curve relation solver or measured complete ECDLP advantage. Next: independent scoring/replay, count failures and useful independent relation yield, and price complete calls. [pcc:unverified] Sources: [session log](../../session-logs/2026-11-15.md), [#4098](https://github.com/isogeny-labs/crypto/pull/4098).

**LFD theorem stack.** [#7](https://github.com/isogeny-labs/lfd-summation-systems/pull/7) is now closed, and [#11](https://github.com/isogeny-labs/lfd-summation-systems/pull/11) is open at 517a0f65a587fdac3c6ec7f298b779e99859bdf2 with a reported successful exact-head Linux Lean/axiom audit. The fixed (E_6) theorem concerns the order-two target and a growing admissible **prime sequence**, while the #11 extension establishes complex growth for rational parameters outside a stated finite set. Neither is an all-curve/all-prime non-torsion theorem, and neither acquired signed PCC standing in this sweep. The claimed audit scopes 582, 628, 644 and 856 declarations refer to different source/audit sets; they must not be interchanged. [pcc:unverified]

**Transfer / comparator stack.** [#1550](https://github.com/isogeny-labs/cryptanalysis/pull/1550), [#1549](https://github.com/isogeny-labs/cryptanalysis/pull/1549), [#1547](https://github.com/isogeny-labs/cryptanalysis/pull/1547) and their parents support the [#3877](https://github.com/isogeny-labs/crypto/pull/3877) program; they are not independent attack mechanisms. CT138 proposes 3,840 attempted bounded source/target cold solves with manual budget admission, not an already executed performance result. CT137's recorded artifact-hydration failure is not a failed arithmetic compile. Price original-prime native primitives and strongest source-side references after custody gates. [pcc:unverified]

**Auxiliary and N83.** [#4305](https://github.com/isogeny-labs/crypto/pull/4305) has a buildable auxiliary setup prototype but expressly has not run its 14,754-preimage qualification due to missing existing checkpoint upload access. [#4295](https://github.com/isogeny-labs/crypto/pull/4295) reports exact analytic query ceilings from retained N83 objects, not observed smooth relation throughput. [#4284](https://github.com/isogeny-labs/crypto/pull/4284) adds a native genus-3 public-target solver; its old 64-target wall ratio comes from the historical Python arm, not native end-to-end timing. [pcc:unverified]

**Negative and partial evidence.** [#3309](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3309) reports zero recovered candidate roots in its 1,760-cell bounded lattice trial, and a separate working degree-1/2 root control. [#4239](https://github.com/isogeny-labs/crypto/pull/4239) retains a falsified small-field density prediction. Neither negative outcome is erased by a passing software test. [pcc:unverified]

## Source and validation limitations

- **Sources checked (KB): unavailable.** The dedicated research-KB MCP actions are not loaded in this connector runtime; no catalog search, theorem-index digest or recent-sources receipt was possible. The project AGENTS.md and primary PR descriptions, pinned Lean theorem files and latest current repository session logs were used instead. This is explicitly not a novelty clearance. [pcc:unverified]
- No cloud billing action, test job, pod/worker lifecycle action or infrastructure mutation was made by this survey. Existing source reports of launches, S3 records or completed pods were not converted into current worker-liveness claims. [pcc:unverified]
- The sweep PR already has a maintainer request to Copilot to merge current main while protecting the immutable main PCC ledger prefix and all session log entries. The request was inspected. This document and the root leads.log are **documentation-only additions**; no PCC ledger or conflicted session file was rewritten. Resolve the existing conflict via the authorized merge workflow and re-run validation/review before any merge. The rejected auto-merge `squash` mode was configuration error, not a permission denial. [pcc:unverified]
- The research claims remain producer reports unless a disjoint independent verifier, matched baseline, registered holdout and signed PCC standing are explicitly established. No leaderboard or benchmark promotion is made. [pcc:unverified]
