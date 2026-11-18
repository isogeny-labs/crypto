# Session log — Prime31 radical RR9 support, 2026-11-17 UTC

## 2026-11-17T08:20Z–08:30Z — research, exact reduction, native source and initial CI

### What was done (commands, files, features)
- Read repo root AGENTS.md and current docs/ACTIVE.md; inspected shared 2026-11-17 session log, latest RR9 task-specific log, #4539, #4532 and #4466, prior native Rust example and pinned workflow. Reviewed prior negative leads #4373/#4484/#4486, #4228/#4349, #4513, crypto-autoresearcher #3301/#3305 and primary APS 2018 / McGuire–Mueller 2017. Research-KB MCP and local repository wrapper unavailable in this runtime; connected GitHub fallback. [pcc:unverified]
- Created task branch `research/prime31-rr9-radical-support-20261117` from PR #4539 head `857f4d830736992288587e3db2cdf277ff05aa6e`, registered task before research in docs/ACTIVE.md (`e1248c2`), opened draft stacked PR #4541, then linked the PR in task line (`2fd5d2e`). Parent #4539 remains open; no duplicate parallel PR on main.
- Prospectively froze `RR9_RADICAL_SUPPORT_PROTOCOL_20261117.md` (`d46ec2c`) before source changes. Derived exact `M|H^8 iff M/gcd(M,M') | H` for p>8, H squarefree, M degree8, all repeated multiplicities; squarefree stratum simplifies to `H mod M =0`.
- Reused original Rust native example; added polynomial Euclidean quotient/remainder, monic gcd, radical support oracle, exact on-curve cross-check assertions (64 positive, 64 coefficient-mutation negative, 6 repeated positive, 6 repeated negative) and polynomial-only multiplicity-partition tests at commit `02e76e640aaf7407280de86ce81ba5f60a115347`. No new solver branch, no Python ECDLP code.
- Derived exact signed-support counts: B4 total 6435, squarefree 0; B8 total 490314, squarefree 256; B31 total 8361453672, squarefree 2019513600. Therefore at least 127933231 of the 2147446831 prime31 residuals cannot be represented by any squarefree 8-leaf support over B31, even under ideal no-collision mapping. Computed via exact binomial integers; this is a necessary coverage bound, not a heuristic expected-yield claim. [pcc:unverified]
- Committed rigorous reduction, structural limit, prior-lead triage and next B8/B31 nonenumerative extraction plan in `RR9_RADICAL_SUPPORT_RESULT_20261117.md` (`3f10107`). Exact-source Rust CI run #44046676997 was in progress when noted; no passed test or new solver speed result claimed.

### Decisions made (and why)
- Split squarefree and repeated-root branches rather than silently discard diagonal F: B31 alone has a rigorous ≥5.96% squarefree coverage deficiency; a complete solver must handle repeats or target randomization and count costs.
- Test B8 first: M=H8 on squarefree branch reduces to eight quadratic coefficient constraints in seven chart variables, avoiding x-support enumeration during coefficient solving. A test-only exhaustive 256-sign oracle may verify entire fibres. B4 squarefree branch is necessarily empty.
- Prior #4532's 1,315ms IC setup/906ms target work versus 170ms BSGS complete batch makes supplied-certificate optimization insufficient to show a full attack; do not assert solver speed.
- No paid compute. Parent PR unmerged so stacked PR #4541 cannot enable main auto-merge yet.

### Open threads (unfinished, pick up next time)
- Inspect exact-source CI #44046676997 on original Rust 1.100 package, fix any Rust compile/test failures and retain failed evidence. Record CI status, test count and immutable source SHA.
- On passing tests, finish report/PR metadata, mark PR ready for review; retarget main and enable auto-merge only after parent #4539 merges (if permitted). Signed independent PCC is still absent.
- Next scientific experiment: public-only B8 coefficient solving and exact signed-fibre equivalence; B31 unknown-factor quotient/F4 plus repeated-x strata; then complete cold unknown-scalar B31 solves against matched cached BSGS and tuned rho.

### Errors (what went wrong, how it was handled)
- No local cargo/rustc or direct GitHub network in the container runtime. Used the authenticated GitHub connector to inspect and commit Rust source; existing pinned GitHub Actions supplies compilation and execution.
- Research-KB/Conductor tools not exposed; fallback to checked GitHub sources documented; no novelty inference from unavailable search.

## 2026-11-17T08:29Z — exact-source Rust checks successful

### What was done (commands, files, features)
- [Actions #44046676997](https://github.com/isogeny-labs/crypto/actions/runs/44046676997) completed with conclusion `success` on exact Rust source `02e76e640aaf7407280de86ce81ba5f60a115347`, job `133913970897`. Read job logs: pinned original Rust 1.100 ran **40/40 library** and **6/6 RR9 example tests**. The example's 128 distinct prime31 positive/negative determinant records passed; 70 radical-support positives and 70 corresponding negatives were exercised in unchanged on-curve panel assertions (including repeated F). Source-specific CI, [pcc:unverified].
- Retrieved uploaded raw artifact id `14312111559` (13,662 bytes) and committed CI evidence/outcome to `RR9_RADICAL_SUPPORT_RESULT_20261117.md` (`81daf8a`). No new full ECDLP benchmark or signed independent verdict.

### Decisions made (and why)
- The exact radical characterization and source-specific test gate are complete at producer-validation scope. The coefficient extractor and fully charged batch speed comparison are separate open research objectives.

### Open threads (unfinished, pick up next time)
- B8 public-only coefficient root solving and B31 repeated-root branches, with exhaustive independent solution-fibre validation. Independent signed PCC still required for verified standing.
- Retarget stacked #4541 to main after #4539 merges; only then consider enabling auto-merge.

### Errors (what went wrong, how it was handled)
- No Rust build/test or original-curve numerical failure in exact-source CI. No local cargo; source-specific Actions supplied compiler and runner.
