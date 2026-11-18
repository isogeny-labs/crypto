# 2026-11-18 Prime31 W8 superincreasing signed-weight screen — PR #4770

## 2026-11-18T23:25Z — mathematics, prior art, frozen experiment and source-specific launch

### What was done (commands, files, features)
- Read root AGENTS.md; read current #4728 RR9 and four-window rank results, #4692 draft public-only extractor, #4722 exact W8 formal count, #4714 historic weighted M6 and matched control, plus same-date shared session log and active queue.
- Reviewed crypto-autoresearcher `experiments/semaev-alternatives-20261105/SUMMARY.md` (SA-04 RR norm, SA-18 determinant nulls, SA-19 Edwards caveat) and PKM 2016 / APS 2018 prime-field IC publications. Research-KB MCP/CLI unavailable; this is an explicitly bounded GitHub/public-source audit, not novelty clearance.
- Proved exact signed **formal-vector injectivity** of weights w_j=3^j using the largest-different-slot dominance argument. Eight-slot B8 integer vector count is 4,294,967,296 versus the exact old W8 3,240,125,537 from #4722; group cardinality 2,147,446,831 means the formal count ratio alone proves no coverage gain.
- Ran independent local Python BigInt affine EC preflight on unchanged p=2^31-1 ordinary cofactor-one original curve. In all four B8 windows, both four-slot halves had 65,536 distinct ternary EC sums; old W8 had 64,737 distinct half sums. On the first 256 prior-archive public Q at window 0 only, old generic grader had 201/256 hits versus ternary 221/256; paired discordance 28 old-only, 48 ternary-only; exploratory McNemar exact two-sided p=0.0286267. This is **development-only**, not confirmatory source-specific Rust evidence.
- Created a fresh stacked draft PR #4770 from exact #4728 head. Registered task in docs/ACTIVE.md and committed native Rust paired implementation reusing pinned original Prime31 Arithmetic, independent Python **verification only**, frozen protocol and GitHub Actions workflow. The workflow runs 40 original Rust library regressions, three new Rust tests, 1024-log OLD/TERNARY/TERNARY/OLD arm order on identical public targets, and independently replays every accepted row, base log and recovered target.
- CI requested from source commit `8e0722378b993e8d2f146e751b722a6ea61ced64`, run [#44421016999](https://github.com/isogeny-labs/crypto/actions/runs/44421016999). At this log entry the run was in progress; no Rust result was claimed yet.

### Decisions made (and why)
- Retain generic 4+4 EC group-sum code strictly as **grader/complete control**. It does not satisfy the research goal of non-generic unknown-index relation extraction. #4692 owns that step.
- Use independent per-target and per-attempt pseudorandom derivations so old and ternary arms receive exactly the same public Q even when relation-hit counts differ. Charge setup, learning, online shifts, verification and all group/field operations.
- Do not extrapolate 32.56% larger formal support to 32.56% faster relation extraction or solver runtime: the actual EC image is a many-to-one map. Do not infer same-host BSGS/rho speedup from historical different-runner measurements.

### Open threads (unfinished, pick up next time)
- Read exact-source run #44421016999 and jobs; fix any Rust compilation or replay failures and preserve failure evidence.
- If the four-arm source-bound replay passes, commit a durable result with both wall-time orderings, total EC/field counts, natural online queries, setup/training, exact source SHA and artifact link, and mark this control complete.
- Pursue #4692 public-residual-only B8 RR9/F4 unknown-index extraction, then reuse #4728's 31-rank/1024-target path and a matched same-runner optimized index-calculus/BSGS/rho frontier. Ternary weights can be promoted as candidate only after algebraic extraction cost passes.

### Errors (what went wrong, how it was handled)
- Local runtime lacks cargo/rustc and cannot clone the repository. Only source-specific GitHub Actions may establish Rust execution; Python preflight is labelled development-only, not a Rust solver or signed independent PCC verification.
