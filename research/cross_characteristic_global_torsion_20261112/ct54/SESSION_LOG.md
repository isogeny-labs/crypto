# CT54 session handoff — 2026-11-12

## 2026-11-12T23:50Z — Fixed-vector source-prime selectivity

### What was done (commands, files, features)
- Read root PR #3877 (concurrent native degree-196 Richelot construction), CT51–53 PR #4015, cross-characteristic PR #4012, root AGENTS.md, latest default-branch 2026-11-12 session-log excerpts, and existing CT52–53 research notes. Avoided native Jacobian and cryptanalysis replay files.
- Shared research-KB MCP and checkout wrapper unavailable; fallback exact-code GitHub search in crypto / cryptanalysis / crypto-autoresearcher returned no match. Read Jones–Rouse §5 page 18 (primary prior-art count) and Zywina's mod-ell image context.
- Produced the CT54 group-theoretic proof, two stand-alone independent verifiers and bounded Actions workflow: ct54_primes.py, ct54_independent.js, CT54_FROBENIUS_SOURCE_FILTER.md and CT54 cross-characteristic workflow.
- Local runs: Python normal/optimized and Node normal/no-opt, all pass; per-language output identical by cmp. Tested exact GL2 counts ell=2,3,5,7,11 and rational 11a1 samples: 665 eligible primes p<=5000, 294/665 divisible by 3, 665/665 divisible by 5. A second independently coded (x,y) point counter agreed on p<=257, 21/51 and 51/51. Tested an orientation-versus-existence mutant conflation, rejected. No paid cluster compute used.
- This handoff was written inside the isolated research directory because the feature branch predates the shared main branch's concurrently edited docs/session-logs/2026-11-12.md; do not add a stale copy of that file and overwrite other workers.

### Decisions made (and why)
- Retain a conditional *conjugacy-invariant* density theorem (there exists some nonzero rational ell-torsion point) and report the labeled-vector stabilizer fraction only as a group proportion, not as a canonical base-prime Chebotarev density. Prevents a mathematical orientation error.
- Focus low-image/adaptive global models, not a generic full-image fixed-global curve or same-characteristic Richelot replay.
- Keep scientific claim [pcc:unverified] until an independently signed proof receipt. Run no paid EKS jobs because these are bounded local finite-field controls and EKS budget status is not approved/verified in this run.

### Open threads (unfinished)
- Exact-head GitHub Actions completion and scientific PCC verification.
- Uniform, encoded-input global torsion lift; compatible algebraic conjugate at source/target primes; full-degree, height and cold solving cost. Check research-KB and autoresearcher lead atlas when available.
- The shared main-branch root session log was not edited because it is concurrently changing; reconcile the CT54 handoff at merge.

### Errors (what went wrong, how it was handled)
- Research-KB MCP/conductor/gh are not installed in this runtime; used repository connector and local stdlib.
- A first connector attempt to write JavaScript failed in the JS call constructor (unescaped interpolation); no file was written. The corrected connector call committed the intended JavaScript verifier. All local controls were rerun before publication.
