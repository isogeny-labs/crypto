# 2026-11-19 — Prime31 block-scale W8 autosweep

## 2026-11-19T14:21Z — Task selection and source audit
### What was done (commands, files, features)
- Read `AGENTS.md`, `docs/ACTIVE.md`, newest relevant 2026-11-18 session-log entries and existing Prime31 original-Rust sources.
- GitHub connector queried current open PRs; read #4692 (active public-only RR9/F4 owner), #4807 (W8 full fibres), #4805, #4803, #4779, #4801, #4793, #4532, #4774, #4770, #4728 and current autolab #4799. Checked literature via public search (PKM 2016, APS 2018 and Hu 2024) and cross-repo GitHub keyword searches for existing blockscale experiments. Shared research-KB MCP and wrapper not connected in this runtime.
- Created branch `automation/prime31-blockscale-sweep-20261119`, tracked task in `docs/ACTIVE.md`, opened **draft PR #4808** prior to the experiment.

### Decisions made (and why)
- Do not duplicate #4692's nonenumerative extractor. Isolate a previously unmeasured design choice: positionally weighted 4+4 W8 relation `R=U+[2^k]V`, varying `k=4..8` on the original curve, same 31-point base and four windows.
- Treat any generic table grader only as a relation-yield control, never as a cheaper non-generic ECDLP solver.
- Freeze new seed, sample, mathematics and output schema before running.

### Open threads (unfinished, pick up next time)
- Original-source CI compile, exact 5-arm measurements, independent signed-curve replay, true RR9/F4 extractor in #4692.

### Errors (what went wrong, how it was handled)
- Original shared dated `2026-11-19.md` absent on main when this research began; use this scoped session log to avoid concurrent-write conflicts. GitHub keyword research/Kb coverage is bounded.

## 2026-11-19T14:26Z — Mathematical result and implementation committed
### What was done (commands, files, features)
- Froze `research/prime31_ic_20261115/rr9/W8_BLOCK_SCALE_PROTOCOL_20261119.md` before implementing or running the study.
- Derived the exact shared-four-slot convolution, `m_c(R)=Σ_U l(U)l(c⁻¹(R-U))`, and group autocorrelation identity `Σ_R m_c(R)^2=Σ_Δ C(Δ)C(c⁻¹Δ)`. Proved `m_{-c}=m_c` pointwise for signed supports and population multiplicity equivalence of `c` and `c⁻¹`.
- Committed `native/examples/prime31_w8_blockscale.rs` reusing the pinned original Rust Prime31 arithmetic; it is a 5-arm full-multiplicity generic control with shared prefix sumset, stored entire 512 public-point corpus, operation counters and 20 signed witness certificates.
- Committed independent `native/verify_prime31_w8_blockscale.py` only for affine validation of Rust outputs and all summary counts; Python does not perform relation grading or ECDLP.
- Committed source-specific `.github/workflows/prime31-w8-blockscale.yml` and mathematical/prior-lead report `rr9/W8_BLOCK_SCALE_RESULT_20261119.md`.
- GitHub Actions triggered with head `082177565c87fac47bbd0c61ba032eb4c5274cf8`: [PR #4808 workflow](https://github.com/isogeny-labs/crypto/actions/runs/44543805506) and [push workflow](https://github.com/isogeny-labs/crypto/actions/runs/44543792457), both were executing pinned source checkout when initially inspected.

### Decisions made (and why)
- Run 128 held-out public targets/window, 4 windows, 5 arms (2,560 exact multiplicities), selected k4 primary; separate setup/probe costs, no positive-query early exit.
- Keep PR draft until results and CI can be validated. Do not claim benchmark success until artifacts exist.

### Open threads (unfinished, pick up next time)
- Check CI actual outcome; if compiled, inspect artifact and independently validated 20/20 witness identities and full count distribution. Add frozen results + cutoff criterion and update #4692. If compile fails, fix and keep exact protocol immutable.
- Integrate only genuine non-generic F4 extractor in #4692 into 31-rank/1024-target Rust solver, then same-runner compare to optimized shared BSGS and rho.

### Errors (what went wrong, how it was handled)
- Found two nested mutable borrow expressions while reviewing Rust code; corrected them in commit `995b373b` before workflow launch.
- No local Rust compiler in automation container; all Rust compile and measurement claims require source-specific Actions evidence.

## 2026-11-19T14:32Z — Exact-source original Rust CI PASS, raw and independent replay inspected
### What was done (commands, files, features)
- GitHub Actions **push #44543792457** and **PR #44543805506** both passed. Original pinned Rust 1.100.0 library tests 40/40, focused tests 4/4, 5-arm full data and independent Python affine replay of 20/20 signed witnesses; 512 Q point reconstruction; all 2,560 multiplicity row summaries verified.
- Downloaded and inspected raw [GitHub artifact #14539671492](https://github.com/isogeny-labs/crypto/actions/runs/44543792457/artifacts/14539671492) with original full JSON, independent checker result and hashes. Original full JSON sha256 `681efda3b729f103cea62e82072989ef6032bf3ea9cfeaedc1f339b79c69431b`.
- Observed public target relation hits: k4 **452/512**, k5 **440**, k6 **440**, k7 **444**, k8 **448**. Prespecified k4 versus k5 paired exclusives 64 vs 52; exact two-sided binomial test p~0.307. W1 shared four-slot EC support 65496; others 65536, invariant under all k scalings. Raw witness totals: 1052, 1063, 1026, 1027, 979.
- Added source-linked machine summary `rr9/W8_BLOCK_SCALE_CI_SUMMARY_20261119.json`, report update `rr9/W8_BLOCK_SCALE_RESULT_20261119.md`, marked scoped task complete in `docs/ACTIVE.md`.

### Decisions made (and why)
- k=4 is the conservative candidate for #4692: highest observed yield with shortest x-only doubling-chain constraints. The observed paired yield difference is not significant on this one-curve cohort, and no higher-k advantage justifies extra chain depth.
- Complete public-only nonenumerative index extraction, rank31/1024 unknown ECDLP solver, and matched faster-than-optimized-BSGS result are still unproved. This is a generic-only grader, not a real algorithmic solver speedup.

### Open threads (unfinished, pick up next time)
- Owner lane #4692: public-R-only B8 F4/remainder-tower with 46+/18− corpus and no generic half-table; same-runner fully charged optimized BSGS/rho vs final rank31/1024 solver.
- Signed independent PCC or third-party replay beyond GitHub CI has not been completed.

### Errors (what went wrong, how it was handled)
- CI PASS; no Rust runtime or independent affine replay failures. Historical baseline #4532 is on a different runner and must never be used as a paired claim.

## 2026-11-19T14:35Z — Handoff, PR readiness and merge-method blocker
### What was done (commands, files, features)
- Posted exact autocorrelation identity, full hit table and no-generic-table next gate to the existing nonenumerative RR9/F4 owner [PR #4692](https://github.com/isogeny-labs/crypto/pull/4692#issuecomment-7012162907).
- Updated PR #4808 body with frozen original-run artifact, formulas, 40+4 Rust tests, 20 signed affine certificates, 512×5 observed outcomes, source links, limits and prior-lead audit.
- Verified the remote PR has exactly eight expected changed files. Marked #4808 **ready for review** after source-specific checks passed.

### Decisions made (and why)
- This research lane is complete as an original-curve generic-control experiment, not a solution to the full nongeneric prime-field ECDLP target. #4692 continues to own unknown-index extraction.
- Existing #4532 is the proper optimized shared-BSGS sensitivity frontier and must be rerun same-runner with any full solver candidate.

### Open threads (unfinished, pick up next time)
- Independent signed PCC remains pending. Final algorithmic gates are #4692 public-only F4 B8 46+/18−, rank31/1024 full solve and strongest BSGS/rho matched comparisons.

### Errors (what went wrong, how it was handled)
- Attempt to enable repo default auto-merge via GitHub failed: **'Merge method squash merging is not allowed on this repository'**. PR remains open ready for manual merge-method decision; no merge performed.
