# Cross-characteristic torsion positive control — 2026-11-12, ~18:45 UTC

## What was done (commands, files, features)

- Read root PR #3877 at head 30fa845af46c70b05fb3665cc12d787e0d793daf; read cryptanalysis CT50 PR #1514, crypto/cryptanalysis AGENTS.md, crypto docs/session-logs/2026-11-12.md and the existing second-prime lift problem.
- GitHub repository search across crypto/cryptanalysis/crypto-autoresearcher checked `cross-characteristic`, `11a1`, `global torsion`, `xedni` before selecting this lead. KB MCP unavailable; fallback documented in README. No concurrent native CT files touched.
- Checked the 11a1 model against LMFDB 11.a2; read background on Xedni and CM torsion fields. This test is an established specialization principle, not novel mathematics.
- Independently computed Cremona 11a1 rational 5-torsion, full char-3 and char-2 group tables and exact pointwise bridge, four source primes, all 25 source/target group-law pairs per prime and all five toy DLPs per prime; explicit wrong-map mutants rejected.
- Executed Python locally with and without -O, and independently written JS with and without --no-opt. The committed code is subject to exact-head CI; the local tests are producer-run, **[pcc:unverified]**.
- No EKS Spot, cloud cost, leaderboard, rho speedup, merge override or native branch mutation.

## Decisions made (and why)

- Use a fixed five-point positive control to distinguish existence of an **abstract group transfer** from existence of an algebraic field-changing morphism.
- Charge the complete O(N) precomputed lift table; do not describe the example as an attack.
- Put isolated work on a fresh feature branch to avoid rewriting the concurrent native #3877 and CT45–50 branches.

## Open threads (unfinished, pick up next time)

- Check exact-head CI/PCC after opening this PR; obtain independent signed validation before claiming verified.
- The scalable obstruction is the number-field degree and construction cost for simultaneous global N-torsion lifts of arbitrary encoded ECDLP instances, especially g>=2.
- Integrate with root #3877 only by linking; keep the native 38-test geometric7 and full Cantor tasks in their existing lanes.
- Shared KB, research-kb index, encrypted atlas, EKS credentials and conductor scope reservation were unavailable from this invocation; no permission bypass.

## Errors (what went wrong, how it was handled)

- The crypto-autoresearcher AGENTS.md fetch returned a decoding failure. Followed crypto AGENTS.md and cryptanalysis AGENTS.md plus lab cross-repo contract pointers instead.
- No relevant research-KB tool was exposed. GitHub fallback does not establish novelty or coverage of encrypted sources.
