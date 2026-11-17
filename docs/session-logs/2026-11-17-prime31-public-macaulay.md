# Prime31 public RR9 Macaulay gate — 2026-11-17 (UTC)

## 2026-11-17 12:22–12:27 UTC — Register and implement public-coefficient negative certificate [pcc:unverified]

### What was done (commands, files, features)
- Read current `AGENTS.md`, `docs/ACTIVE.md`, latest 2026-11-17 session journal, RR9 source and results from PR #4547 and ancestors; reviewed mixed-base #4513, cached BSGS #4532, F4 #4373, autoresearcher #3301/#3305.
- KB MCP not loaded here; repository wrapper cannot be run in this connector runtime. Checked GitHub source/PRs and Amadori–Pintore–Sala 2018 DOI 10.1016/j.ffa.2018.01.009. No novelty claim.
- Created branch `research/prime31-rr9-macaulay-public-20261117` based on #4547, registered active item in `docs/ACTIVE.md` and opened draft PR #4548 before source code. Froze `RR9_PUBLIC_MACAULAY_PROTOCOL_20261117.md` at Git commit `6ca7ab2d`.
- Added `native/examples/prime31_rr9_public_macaulay.rs` at `ad957457`. On real ordinary prime31, it forms all eight quadratic coefficient equations of `D_R V^2-(x-r)A^2-2sAV-H_B=0` for seven variables, and a degree-5 Macaulay matrix (792 monomial columns, 960 potential input multiples). It performs public-only row elimination and, if a unit is found, reconstructs and replays the explicit degree-5 polynomial identity `1=sum c_{ij}m_j q_i`.
- Independent test-only original-curve group census chooses a genuine B8 signed-sum positive and two outside all 256 signed sums. It does not pass signs/group lookup into the algebraic candidate; tests reject exceptional x and infinity residuals.
- Added pinned-source Rust 1.100 workflow `.github/workflows/prime31-rr9-public-macaulay.yml` at `5db662f6`, running the existing original Rust arithmetic tests, new example tests and raw JSON archive. Workflow #44083153755 was initiated; no verdict yet at this journal timestamp.

### Decisions made (and why)
- Limited to **negative-only B8 unit certificates**, rather than claiming a positive extractor or complete unknown-Q DLP: rank deficiency is inconclusive, and a planted positive has no relation-yield significance.
- Preserve [pcc:unverified] and do not promote benchmarks or assert an advantage. Historical #4532 1024-target cached BSGS took 170.113ms versus mixed IC 2221.474ms in a producer run; this is not a same-source measurement of the new candidate.
- Stack on #4547 until parents merge; keep draft until source CI and record review are complete; no paid EKS work or new full DLP claims.

### Open threads (unfinished, pick up next time)
- Inspect #44083153755 exact-source Rust CI and raw evidence; resolve any compile/mathematical errors without altering registered degree 5 or positive/negative definitions.
- Report measured rank/unit certificates, actual wall and field-operations for all cases, including any inconclusive negative. Run independent certificate replay if tool support allows; remain unsigned until PCC.
- Gate a subsequent public-only B31 coefficient extraction attempt on positive solution recovery (not merely negative elimination), complete ECDLP unknown Q and matched shared BSGS costs.

### Errors (what went wrong, how it was handled)
- No implementation or mathematical failure observed before CI verdict. Native Rust compiler is unavailable in this connector runtime; GitHub Actions is the source-specific compilation and runtime environment.
