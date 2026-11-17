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

## 19:18–19:27 UTC — certified negative D9 ideals with standalone unit-witness replay

### What was done
- Initial registered GitHub Actions run #44156703850 (source fe7f58c) passed 40 original native tests and focused sparse operator control. Both public negatives produced rank gate BM complexity N=11440, with nonzero terminal coefficients 804920363 and 948224476.
- After seeing rank success, amended the preregistration **before** writing a constructive certificate extension. From BM characteristic-polynomial inverse applied to constant e0, computed w=D M x and asserted the exact finite-field polynomial identity M^T w=e0.
- Exact-source run #44157634359 passed. Retrieved both full 27,456-coefficient unit witnesses; independently checked ZIP SHA256 1bcb1c9653f59b6c40aa31f2c511b208ab74808a4f87a33bb158acb00ade563f. Reconstructed each public polynomial independently in a separate modular arithmetic program and verified exactly constant 1 and all 11,439 nonconstant monomial coefficients zero. Separately enumerated all 256 signed B8 original-curve sums to confirm the two public negative fixtures are absent and the known-positive fixture present (validation only, not candidate input).
- Added independent **Rust** unit certificate verifier prime31_rr9_d9_unit_verify.rs, which reconstructs the eight RR9 quadrics independently of the Krylov candidate and checks the full polynomial identity. CI #44157953813 succeeded: 40 native library tests, two candidate example tests, one independent verifier example test, both full negative rank+witness runs and their independent replay. Downloaded full vectors; ZIP SHA256 7072196fa51a60b7b0559c3caa88ac7e0491938e8c9749d5f062c0a6133fbbe4 matched GitHub.
- Published permanent report RR9_D9_BLACKBOX_RESULT_20261117.md and concise JSON evidence index RR9_D9_BLACKBOX_SUMMARY_20261117.json. Full coefficient witnesses remain in GitHub Actions 30-day-retention artifacts. Updated workflow to additionally archive the independent replay JSON without changing original algorithm or frozen parameters.

### Results, limits, and future
- D9 M has 27456 rows, 11440 columns and 240240 nonzeros. Both R0 and R1 have explicit modular Bézout unit certificates. BM rank work used 22880 Krylov steps; inverse construction 11439 further Gram applications. Producer full wall ~27.142s and 27.190s respectively.
- This is **not** a complete ECDLP solver or a speed advantage: zero new blinded unknown-target logs, no sign-independent positive chart extraction, no B31 completeness. [pcc:unverified] — independent standalone arithmetic replay completed, external signed PCC validation still missing.
- Global 'Workflow files parse' check failed at upstream external artifact-store pull before parser execution, unrelated to new pinned Rust source-specific passing workflow.
- Next: public-only positive coefficient extraction in original Rust sparse F4/quotient; B31 Dickson H31/K961/S5 and repeated-root/exceptional cases; fully charged unknown-log test vs shared-advice BSGS, optimized rho and published prime IC. Separate owner has concurrent full 1024-Q batched IC comparison #4596; avoid duplication.
