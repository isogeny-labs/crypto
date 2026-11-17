# Session log — prime31 seven-of-eight RR9 quotient split — 2026-11-17

## 17:23–17:35 UTC — exact split quotient and source-specific Rust benchmark

### What was done (commands, files, features)
- Read current `AGENTS.md`, `docs/ACTIVE.md`, today's shared session log, original native `research/prime31_ic_20261115/native/lib.rs` and existing B31 four-plus-four oracle. Reviewed prior #4545/#4547/#4548/#4549/#4550/#4556/#4558/#4575, crypto-autoresearcher #3301/#3305, and cryptanalysis #1536, plus named published prime-field index-calculus and hybrid papers. Knowledge-base MCP and CLI are not exposed in this connector-only runtime: **KB unavailable**; all novelty claims are withheld.
- Created branch `automation/prime31-rr9-seven-quotient-split-20261117`, separately committed its task line to `docs/ACTIVE.md` (commit `d7178b7`), opened draft [PR #4581](https://github.com/isogeny-labs/crypto/pull/4581), and froze a prospective three-residual, 128-branch protocol before writing code.
- Committed **native Rust** example `research/prime31_ic_20261115/native/examples/prime31_rr9_seven_quotient_split.rs` reusing the exact original prime31 `Arithmetic` (immutable native source `1df7146`) and a pinned Rust 1.100 GitHub workflow. Each of 128 signed 7x7 systems for each of 3 frozen public residuals is independently solved and checked against the **unfactored** polynomial identity and eighth selector; original group-law replay checks the positive.
- Both [pull-request CI #44138009239](https://github.com/isogeny-labs/crypto/actions/runs/44138009239) and [push CI #44137999498](https://github.com/isogeny-labs/crypto/actions/runs/44137999498) **PASS** (40/40 native library tests, 1/1 new example test, release jq receipt assertion). [Artifact #14353560993](https://github.com/isogeny-labs/crypto/actions/runs/44138009239/artifacts/14353560993) downloaded and ZIP rehashed; SHA-256 `6b1aa75e052fa93d6ea5c51c156a3cef5cac02c28f59ab8500cdcae0d12fe673` matches GitHub digest. Raw 1,737-byte JSON content's Git blob `644dfff371fe2311963e20be7d438c2382497155` matches the committed `RR9_SEVEN_QUOTIENT_SPLIT_RAW_20261117.json`. Full proof, report and next experiment queue committed to `RR9_SEVEN_QUOTIENT_SPLIT_RESULT_20261117.md`.

### Decisions made (and why)
- All three frozen residuals give 128/128 full-rank 7x7 systems and 128 distinct solutions, 896/896 unfactored seven-root norm checks each, zero double-sign ambiguity, and eighth-selector counts [1,0,0]; the one positive passes independent original group addition. Therefore each **seven-equation** affine quotient is exactly `Fp^128` for the tested nonexceptional B8 inputs. The proposed seven-of-eight quotient **encodes** signed branches and is retired as an independent *nonenumerative speedup* lead; this is not a general lower bound.
- Diagnostic enumerative Rust phase costs 0.443992 / 0.437080 / 0.419025 ms on the three instances; producer [pcc:unverified], not optimized sign-free F4, complete ECDLP, or a calibrated hardware comparison. No leaderboard or benchmark promotion.
- Preserve prior historical 1024-log producer comparison in PR #4532: shared BSGS 170.113ms vs complete mixed-base index calculus 2221.474ms; not rerun or claimed defeated.

### Open threads
- Build actual **public-R-only sign-independent full-eight-equation** sparse D9 rank/quotient extractor using existing Rust; replay positive coefficient witnesses and negative unit certificates without choosing sign masks. D9 dimensions: 11,440 columns and 27,456 generator rows, so avoid blind >1-GiB dense matrices.
- Extend to natural unplanted B31 residuals and all-multiplicity `M/gcd(M,M') | H_B` with exceptional `x(R)=x_i`; then complete blinded prime31 unknown logarithms with failed attempts/setup/memory fully charged against shared BSGS, optimized rho, and external IC.
- Independent signed PCC is still absent; seek unrelated implementation review and source replay.

### Errors
- No Rust compilation/test errors in the two final-source CI runs. The research-KB tools and EKS/Conductor controls were unavailable in this connector-only runtime, so source review used live GitHub and the work used bounded GitHub Actions Rust CI instead of paid cloud compute. No secret or infrastructure changes.
