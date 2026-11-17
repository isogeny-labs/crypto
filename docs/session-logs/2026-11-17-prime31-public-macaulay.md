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

## 2026-11-17 ~12:36Z — Complete bounded degree-5 public RR9 test [pcc:unverified]

### What was done (commands, files, features)
- Exact-source pinned Rust Actions run #44083153755 (and prior-sha retries) compiled all **40/40 original library tests** but rejected new example with E0308: the test `Point` optional was unwrapped into a tuple in `fixtures`. Corrected only the Rust return-wrapper at source commit `493d5ea7a97e1e1e1182b446c8a6c3a61798efd9`; prospective degree, positive/negative fixtures and all algebraic contracts remained unchanged.
- [Final-source GitHub Actions #44084070161](https://github.com/isogeny-labs/crypto/actions/runs/44084070161) **SUCCESS**: 40/40 inherited original native library and 1/1 new example test, then source program JSON and workflow assertions passed on `493d5ea7`. Exact artifact ZIP #14328996551, digest SHA256 `36d587f81ea7a91b6f28d4620bd19918857c07ca807f0a84fbca442d829088e9`; locally downloaded ZIP hash matched provider hash and parsed the complete 2862-byte raw JSON.
- Positive R=(1911579686,548820096): degree5 960 input multiples, 792 columns, rank736, 56 residual dimensions, 22,557,136 elimination multiplications, 37.414ms, no unit certificate and no unique positive coefficients.
- Two public residuals excluded from all 256 signed B8 sums by separate test oracle: G=(625037715,2100627251) and (1923055775,307731458), both 792-column rank736 with no unit/positive coefficient; 35.174ms and 35.081ms. Thus both negatives are **inconclusive** at degree5 (not incorrect positives), and the true positive also cannot be extracted at degree5.
- Committed complete result and prior-lead audit at `research/prime31_ic_20261115/rr9/RR9_PUBLIC_MACAULAY_RESULT_20261117.md` on #4548. Existing source unchanged since final successful CI. All outcomes producer [pcc:unverified], no signed independent PCC.

### Decisions made (and why)
- Negative bounded experiment complete. Preserve unhelpful degree5 limit and exact raw evidence; do not extend degree post-data, inflate claim into a non-generic extractor or full unknown-target solver, or enter leaderboard.
- Historical #4532 cached 1024Q BSGS 170.113ms remains applicable required comparator but not same-runner remeasured. A full solver could not afford this 35ms per failed coefficient attempt under that cohort; no speedup ratio asserted.
- Keep PR stacked on #4547; make this bounded finished task ready for review without enabling invalid stacked auto-merge.

### Open threads (unfinished, pick up next time)
- Source-independent Rust arithmetic/syzygy audit of degree5 rank736/56 residual dimensions; prospective sparse F4 degree6/7 with fixed cost/memory budgets; then public positive coefficients and blinded complete B31 ECDLP, all setup/failures charged.
- Signed independent PCC and strongest generic/external IC matched full-cost comparisons remain undone.

### Errors (what went wrong, how it was handled)
- Multiple first-version exact-source GitHub workflow runs failed compilation with E0308; retained failed run links, corrected option wrapper, final exact-source PASS.
- The test's three source-checked residuals produced **no** degree5 unit or coefficient extraction, a scientific inconclusive result not a technical crash. Both negative targets remained unrefuted.
