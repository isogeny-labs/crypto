# Prime31 RR9 D9 sparse public-rank experiment — 2026-11-17

## 19:05–19:15 UTC — prior-lead audit, protocol, original Rust implementation, CI

### What was done (commands, files, features)
- Read AGENTS.md, docs/ACTIVE.md, prior prime31 PRs #4548/#4549/#4550/#4556/#4575/#4581, B31 signed-eight oracle, concurrent PR #4596, native Rust prime31 library/public RR9 coefficient code and recent logs. Research-KB MCP/CLI unavailable in connector runtime; used GitHub code/PR searches and primary literature (no novelty absence claim).
- Literature checked: Amadori–Pintore–Sala 2018 DOI 10.1016/j.ffa.2018.01.009; Dumas–Kaltofen–Thomé 2015 arXiv 1507.01083; Eberly 2016 arXiv 1607.04514; Dumas–Kaltofen 2014 arXiv 1401.4567. Wiedemann and BM are established techniques.
- Created task branch automation/prime31-rr9-d9-wiedemann-20261117, tracking line in ACTIVE alone, draft PR #4604, committed PR number.
- Froze protocol before implementation: research/prime31_ic_20261115/rr9/RR9_D9_BLACKBOX_PROTOCOL_20261117.md, using actual p=2147483647, n=2147446831, G, original B8 basis and two original frozen off-support negative residuals.
- Implemented original-Rust sparse matrix-free 27456x11440 Macaulay M from original public RR9 quadratic system, A=M^T D M with nonzero diagonal; 2N Wiedemann scalar Krylov moments and BM rank criterion; no signed supports or known scalar enter candidate.
- Test-only D3 sparse-vs-dense equivalence, invertible/singular 2x2 toy, original source #4550 positive D9 null-vector check, two D9 smoke applications. Added source-specific pinned Rust 1.100 GitHub workflow and always-upload raw artifacts with 1500s caps.
- GitHub Actions compilation/benchmark is required. No new D9 rank or complete DLP claim before exact-source results. [pcc:unverified]

### Decisions made (and why)
- Avoid duplication of separate complete 1024-Q same-cohort Rust IC/BSGS benchmark already running in #4596.
- Do not repeat enumerative B8 signed fibres (#4575) or 7-quotient Fp^128 (#4581); D5/D6 dense ranks #4548/#4549 have generic Hilbert function. #4550 established D9 as exact potential rank separator.
- Important finite-field caveat: singular M^T D M does not imply singular M because of isotropic subspaces; only invertible A certifies full rank of M. BM scalar degree N with nonzero last coefficient is sufficient, other outcomes inconclusive.
- This is a proof/certificate component, not a new complete non-generic ECDLP solver. Do not rank a component timing against a full BSGS solve.

### Open threads
- Check PR #4604 source-specific CI, repair only build defects and preserve frozen inputs/seeds. Verify negative case JSON before claiming rank certificates.
- Continue sign-free public positive RR9 extraction via original sparse F4/quotient using B31 repeated-root/exceptional controls, then full unknown-target IC cost against BSGS, rho, external IC.
- Independent PCC signed standing, full B31 complete solver performance and mature external comparator pending.

### Errors
- No local Rust compiler or research-KB MCP/CLI available in this connected runtime; GitHub Actions exact-source CI used. No claim inferred from pending test.
