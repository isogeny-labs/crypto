# Prime31 independent-slot mask continuation — 2026-11-18

## 10:27–10:33 UTC — fixed-curve mask-only falsifier completed [pcc:unverified]

### What was done (commands, files, features)
- Read AGENTS.md, docs/ACTIVE.md, 2026-11-18 shared and Prime31-specific logs, and prior lead work #4714/#4678/#4692/#4698/#4711 before selecting the experiment. Shared research-KB tools were unavailable; bounded GitHub source search and accessible published sources were checked.
- Created branch automation/prime31-independent-slot-mask-20261118, added ACTIVE task, opened draft PR #4716, preregistered exact mask/public-target protocol before any source run.
- Implemented rust example prime31_masked_slot_parity.rs importing original pinned crate, and separately coded Python BigInt verifier, pinned source/workflow.
- Source-specific Actions [#44286173248](https://github.com/isogeny-labs/crypto/actions/runs/44286173248): 40/40 original Rust tests, exhaustive B4 mask test, full B8/B16/B31 candidate, and independent Python B8/B16 24-target parity and three planted witness identities all PASS. B31 full parity was not independently enumerated. [pcc:unverified]
- New algebraic proof: for arbitrary eight positive slot subsets of the **same** B-point base, both exact positive-sum membership support and group-algebra parity support are <= C(B+7,8). At B31 this is 48,903,492 / 2,147,446,831 = 2.27728535% maximum uniform target coverage.
- Outcome on same 12 original Prime31 public targets: 0/12 odd for B8, B16, and B31. B31 132,176 setup +177,888 query original EC additions. Raw transcript retained in Actions artifact #14418798701, sha256 378d0a7afed84eddad6b9c923f43ef5323d1db727fec53c946cf5cabebb4f913.

### Decisions made (and why)
- Predeclared >=4/12 odd B31 acceptance gate failed. Retire independent binary-masked subset rearrangements of shared B31 from the next nonenumerative extractor queue; their 2.28% maximum membership coverage cannot repair the yield issue, even with a perfect oracle.
- Keep this as a scientifically useful negative control. Despite 41.66x fewer *stage* additions than prior weighted B31 #4714, zero natural relations means no solver cost advantage. Do not present a complete log recovery claim.
- Prioritize truly distinct public point maps/independently hashed images with larger effective support, and reuse existing RR9 #4692 rather than duplicate it.

### Open threads (unfinished, pick up next time)
- Independent signed PCC approval.
- Existing nonenumerative RR9/F4 or tensor extractor must yield natural original-curve relations and full-rank factor-base logs; run the complete 1,024-blinded-target Rust benchmark with matched IC, shared BSGS, rho charges.
- Shared CI artifact-store failure is outside this task.

### Errors (what went wrong, how it was handled)
- The frozen experiment ran as specified and passed without code retries. The >=4/12 natural odd-support result gate failed scientifically, 0/12 in every rung; retained without changing seed or masking.
- Shared workflow-parse job failed pulling existing external artifacts; source-specific Rust job and independent replay both passed.
