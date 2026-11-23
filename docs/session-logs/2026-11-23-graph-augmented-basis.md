# Graph-IC exact augmented-row basis — 2026-11-23 UTC

## What was done (commands, files, features)
- Read current AGENTS, ACTIVE, current graph catalog SOURCES/README, the canonical session-log reference (large GitHub file fetch returned empty despite nonempty blob SHA), lane handoff and current PR #5043. Confirmed PR #4988 merged, #5043 draft/open. Read exact original Rust `projected_rank.rs`, evaluator, source-bound native test and the original binary collector.
- Queried the runtime tool registry for KB/Conductor/independent workers; none exposed. Fallback: current repository source, cross-repository GitHub code search (`cryptanalysis` prime-orbit/large-prime research and `crypto-autoresearcher` IC goal notes), Diem 2006 and Cavallar 2000.
- Created preregistered mathematical/control protocol `research/lp_graph_algorithms_20261123/AUGMENTED_BASIS_SELECTION_20261123.md` at `7a28163b`.
- Implemented additive original-Rust `select_augmented_basis` in `src/cryptanalysis/large_prime_graph/projected_rank.rs` at `eaf222be`: keep original small-rank pivot rows plus at most one row with nonzero residual known RHS. Count retained sparse and source terms, keep ordered source-index selectors, avoid changes to production filters. Added serde defaults for old exact-rank receipts at `e24fe011`.
- Added native original-module tests at `b071cbfa`: fixed F101 contradictory RHS example, zero/incompatible controls, 160 deterministic canonical sparse rows over F3/F7/F101/F2147446831 against independent dense original small/augmented ranks, source witness and order-preservation, atomic invalid-input refusal.
- Native validation is requested through the existing source-bound graph workflow on PR #5043. At last inspection after authored test commit `45236388569` was pending; do not call this revision tested until terminal. Prior 37/37 PASS #45233736133 belongs to predecessor CLI source; not to the new augmented-basis source.

## Decisions made (and why)
- Selected exact *augmented* row-space basis pruning instead of further inversion-count tuning: `rank(S)` alone does not preserve contradictory known RHS, but all nonzero post-elimination RHS residuals span one dimension, so pivot originals plus the first such original row preserve `rowspan([S|b])`.
- Preserve full source-bearing original rows in ascending order; do not substitute echelon combinations or promote component savings into a solver gain. The selection routine is opt-in and not wired into any ECDLP production solver. Cost includes full sparse rank audit, metadata and output copying.
- No new paid compute, merges, or parallel native builds. One coordinator with no independent worker backend. No source edits in neighboring lanes.

## Open threads
- Check latest exact-head source-bound native CI, fix compiler/test errors if any; append exact terminal receipt and current SHA.
- Qualify full unshimmed root crate and binary collector only after the unrelated `prime_root_trace.rs` parser blocker is repaired by its owner. Audit a real bounded captured binary stream as a correctness control.
- Obtain naturally generated, group-verified ordinary Prime31 two-LP source with `p=2147483647`, subgroup `ell=2147446831`, G, targets and signed relation provenance before matched forest/pivot/MST, cold IC, optimized negation-rho measurements.

## Errors (what went wrong, how it was handled)
- GitHub connector returns empty content for large canonical `docs/session-logs/2026-11-23.md`; preserved it and wrote this scoped dated log rather than overwrite concurrent sessions.
- No research-KB or Conductor/agent-spawn tools and no Cargo/local repository checkout in this runtime; recorded bounded source fallback and used existing free native CI without claiming unexecuted tests.


## 2026-11-23 18:20 UTC — terminal original-source CI receipt

### What was done (commands, files, features)
- Inspected exact-head GitHub workflow [#45236967753](https://github.com/isogeny-labs/crypto/actions/runs/45236967753), job 137911163999, including full native job log. **SUCCESS**: 39/39 original-source release Rust integration tests, including both new affine-basis regressions, original evaluator example compile and 36-row F101 same-stream default/opt-in/row-cap smoke with new basis-index/count assertions. Exact native job source SHA-256 for `projected_rank.rs` `ae8857539189fd3aed5ddbeee9c28169d2d318508a6e77db6ba21cdad78ce557`.
- Updated the durable lane handoff and draft PR #5043 with this terminal result.

### Decisions made (and why)
- Promote **only** original-module synthetic correctness status from authored/pending to PASS. Preserve the earlier unrelated full-root parser failure and missing collector compilation. Do not promote a performance or signed PCC claim.
- Keep exact echelon charge explicit; copying a minimal augmented row basis after already doing full elimination may regress cold solve wall time, even if rows and Z decrease.

### Open threads
- Qualify unshimmed root and authentic binary-curve collector after the out-of-lane parser repair; then group-replay actual captured relations.
- Obtain a source-verified ordinary Prime31 two-LP stream and matched full cold system comparison. If the exact selector is expensive, test an inexpensive sparse prefilter instead of claiming a gain.

### Errors (what went wrong, how it was handled)
- Earlier source/test CI runs were superseded/canceled as branch updates triggered concurrency; the final job above, not canceled predecessors, supplies the terminal receipt. No paid jobs or conflicting worktree edits.


## 2026-11-23 18:36 UTC — minimum-payload affine basis continuation

### What was done (commands, files, features)
- Read current AGENTS/ACTIVE, scoped newest graph log, lane handoff, original Rust source/native tests and merged prior graph sources. Queried cryptanalysis and crypto-autoresearcher code search; KB/Conductor/worker runtime unavailable, bounded Cavallar/Diem primary-literature fallback recorded.
- Preregistered mathematical correctness, fixed input, exhaustive minimality oracle, falsification and CI budget in MINIMUM_PAYLOAD_AUGMENTED_BASIS_20261123.md at b6e3b87.
- Implemented additive opt-in minimum additive small-NZ+source-witness-entry original augmented-basis selection via sparse echelon ordered by cost. Added exact original-row preservation checks, fixed 10-vs-5 payload control, protected target column and 128 tiny exhaustive matroid-basis checks. Code + tests source frozen for CI; not yet called a passing test.

### Decisions made (and why)
- Greedy on the represented augmented-row linear matroid gives an exact minimum for the frozen additive ENTRY-COUNT objective only; it does not minimize filled NZ, inversion work, wall time or DLP solving complexity. Leave default evaluator/CLI/production untouched.
- Reuse one existing source-bound CI gate, no paid compute or parallel builds. Retain contradictory RHS as explicit refusal, never omit unknown target coefficients.

### Open threads
- Inspect native graph CI on the final source revision and preserve source hashes; fix any test failure within PR #5043 only.
- Complete costed same-stream actual-curve relation and cold ECDLP comparison once source-verifiable ordinary Prime31 inputs and full-root build authorization are available.

### Errors (what went wrong, how it was handled)
- No local Rust compiler/worktree or independent agent runner. Existing root parser is out of scope. The synthetic brute-force tests cannot count as measured performance or actual prime-field ECDLP evidence.


## 2026-11-23 18:48–18:50 UTC — min-payload native receipt and terminal branch custody

### What was done (commands, files, features)
- Inspected source-exact original Rust [GitHub Actions #45241866240](https://github.com/isogeny-labs/crypto/actions/runs/45241866240), job 137928322580, SUCCESS at `650d0fb7694f902001d2bb8745f3e4baee619205`: Rust 1.100.0, **41/41 native release graph tests PASS**, including the new source-preserving matroid-greedy minimum-payload fixture and 128 seeded exhaustive original-row subsets, plus original LP evaluator compile and seeded F101 default/opt-in/cap CLI replay PASS. Captured projected_rank SHA256 `6fc989281f606a84fd3d56d2116eed9d46a0099817e69ede89c569a6aa7602b3`, tests SHA256 `5fbdd31a368b8c98e4640715603ce1478e287af06e26d7036d20af701aa270d9`.
- Reviewed the GitHub PR #5043 live state; observed it **merged by a separate concurrent event** 2026-11-23T18:49:30Z, merge SHA `f7a29f229c94cbae7f791738a5b2125af5e0e075`, contrary to this lane's explicit no-merge constraint. This run did not perform or authorize the merge.
- Started a disjoint successor draft PR #5059 on `automation/graph-ic-minpayload-validation-receipt-20261123` with a reserved ACTIVE task, removed temporary branch-specific CI push fallback while keeping default pull-request check, and appended a source-exact receipt and custody record to the durable graph handoff.

### Decisions made (and why)
- Preserve the earlier PR and commit evidence and do not reopen, revert, or continue edits on terminal PR #5043. Keep #5059 draft/unmerged under the user's explicit prohibition.
- Count 10→5 only as fixed synthetic source+coefficient entry payload; no wall-time or memory advantage established. The linear-matroid greedy theorem is exact only for the fixed additive row-weight objective.
- Leave other sessions' worktree, root parser and unrelated CT330 YAML untouched, and do not launch paid compute or duplicate native test jobs.

### Open threads
- Obtain genuine group-verified ordinary Prime31 two-LP source stream with complete curve/G/Q/subgroup/target/replay custody, then measure charged sorting, sparse elimination and source cloning against downstream N/Z and solving wall costs alongside the best index-calculus and negation-rho baseline.
- Full original root/native collector gate after parser ownership repair, independently signed PCC, and any root-wide source integration remain distinct validation gates.
- Check successor PR #5059 status before any future edits; no merge without owner authorization.

### Errors (what went wrong, how it was handled)
- Concurrent large branch merge advanced the old PR unexpectedly and terminalized it; earlier atomic handoff commit attempt failed with a GitHub GraphQL transient. Used a new disjoint draft successor PR and completed the receipt without force-pushing or overwriting any other session's log.
- KB, Conductor and worker runtime unavailable; bounded original-Rust and primary literature fallback, one coordinator only.
