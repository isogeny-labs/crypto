# 2026-11-17 — Prime31 Dickson31 complete rank-31 / 1024-Q control

## What was done (commands, files, features)

- Read `AGENTS.md`, `docs/ACTIVE.md`, source-specific #4618 and parent #4615, portfolio #4594, competing separate lanes #4596 and #4604 and previous #4532 full BSGS frontier. Consulted current native Rust source, PKM 2016 and APS 2018 literature records; shared research-KB tools and local CLI were unavailable in this connector-only runtime (bounded source audit, not novelty clearance).
- Registered the work first in `docs/ACTIVE.md`, opened draft PR #4623 stacked on #4618 and froze `DICKSON31_COMPLETE_RANK_1024_PROTOCOL_20261117.md` before experiment code.
- Reused existing Rust `Arithmetic`, `Base::build(PRIME31,6)`, `FourSet::build`, 512-way `recover_batched`, and implemented a 31-column modular-n linear algebra class, known-source relation collection, 16 additional source controls, 1,024 target calls (caller-only k never supplied to relation collector), nine adaptive shifts, separately charged total BSGS fallback, scalar-point verification and exact JSON.
- First exact-source Rust 1.100 GitHub Actions run [44189781339](https://github.com/isogeny-labs/crypto/actions/runs/44189781339) **passed** 40/40 library and 1/1 new focused test. New p=2147483647, n=2147446831, 31 Dickson base: rank 31 from 31 rows, 1,024/1,024 complete unknown scalar recoveries, 0 fallback, 76 exhausted target shift misses, 1,100 membership attempts; charged 10,028.811863ms including setup and verification. The original 4A support remains explicitly generic-equivalent. No benchmark-frontier claim. Raw artifact [14376595656](https://github.com/isogeny-labs/crypto/actions/runs/44189781339/artifacts/14376595656) is a producer diagnostic (not independent PCC).
- Independently reimplemented curve arithmetic + Dickson depth-six factor base from the historical #4615 98-case JSON in JavaScript BigInt, not using the new solver. Reconstructed seed 3 and 31 rational points, verified 98 caller-Q points and 98 signed eight-point relations, 11 recorded exhaustive negatives, modular rank 31 from the first 31 successful equations, and all 31 independently recovered factor-base discrete logs against original curve arithmetic. Committed `DICKSON31_INDEPENDENT_98Q_RANK_AUDIT_20261117.json`. This independent historical audit is not a new unknown-log algorithm or PCC signature.
- Added 31 raw basis-row and 1,024 per-target (Q, recovered k, shifts, signed word, failures) transcript fields for independent verification of the *new* 1,024 case run. Exact-source workflow [44190175878](https://github.com/isogeny-labs/crypto/actions/runs/44190175878) then passed 40 lib tests but failed compiling the larger JSON macro at the default `recursion_limit`; no corrupted measurement is claimed. Raised only macro recursion limit to 512 in commit `ce045bc`; re-run pending at the time of this log entry.

## Decisions made (and why)

- Keep correctness/cost control rigorously separate from *novel* target-conditioned nonenumerative RR9/S9/K961-S5 extraction. The 4A enumerator is still generic-equivalent even though factor-base scalar recovery is a complete index-calculus-style pipeline.
- Historical 170.113ms shared-precompute BSGS / 2221.474ms mixed IC is from different code / previous CI. Do not claim an exact matched ratio for this new Dickson31 10.029s case, nor a generic frontier win. #4596 owns the matched 1024Q comparator.
- All targets, 31 group source logs, all relation failures, verification, setup and fallback must be charged. Additional raw transcript instrumentation changes wall work and must be measured anew; preserve first successful timing as prior producer run.
- No paid compute, no merge, no leaderboard or signed-PCC promotion.

## Open threads (unfinished, pick up next time)

- Read source-specific rerun after `ce045bc`. On success independently replay all newly logged 1024 original-curve target witnesses and recovered scalars, commit durable raw JSON, validation summary and experiment results; on another failure preserve error and repair without protocol retuning. Then mark PR #4623 ready when appropriate and retain unmerged parent stack.
- Next *non-generic* experiment is public-residual-conditioned S9/K961-S5 polynomial extraction with original-ideal certificates and no 4A enumeration, starting B4/B8 positives and negatives, then real Dickson31 B31. Full unknown-log, cost, strongest generic BSGS/rho and mature published IC comparisons before claims.

## Errors (what went wrong, how it was handled)

- New 1,024-case transcript increased the `json!` macro expansion past rustc default recursion limit. Run [44190175878](https://github.com/isogeny-labs/crypto/actions/runs/44190175878) failed at compile stage; fixed macro recursion limit only and retained failed run in evidence. First summary-only run had succeeded and is a separate measurement.
