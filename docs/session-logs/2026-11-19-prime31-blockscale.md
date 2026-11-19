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
