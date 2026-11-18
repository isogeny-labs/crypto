# 2026-11-18 Prime31 paired-complement experiment — session log

## 02:22 UTC — Task intake, repo review and prospective protocol

### What was done
- Read repository AGENTS.md, docs/ACTIVE.md, latest available 2026-11-18 session log, existing native Rust original source, source-specific PR #4635 full-cost results, and prior RR9/IC PR metadata (#4604, #4596, #4623, #4626, #4629, #4638, #4646).
- Reviewed PKM (2016) and APS (2018) prime-field index-calculus publications; inspected original Rust 512-lane batch complement and first-x 256-Q harness.
- Created the task branch stacked on unmerged #4635, committed the initial ACTIVE task line before code, opened draft PR #4647 and froze protocol independently of candidate measurements.

### Decisions made
- Test negation-quotient symmetry of the existing S=4A table: one representative per ± pair, both R±P tested using the same batch-inverted x denominator.
- This is explicitly a generic-equivalent engineering control. Use unchanged curve, factor base, blinded corpus and exact signed relation recovery.
- Live research-KB unavailable in this runtime; bounded GitHub and primary literature review used.

### Open threads
- Add paired complement Rust implementation and matched first-shift 256-Q component test; compare to original.
- If successful, integrate into complete blinded logarithm harness and measure all costs with preserved source.
- Capture exact-source Actions logs and raw evidence, including negative/exceptional controls; independent signed PCC separately.

### Errors
- No local Rust toolchain or GitHub network from isolated container; tests must run via exact-source GitHub Actions.

## 02:32 UTC — First pinned CI compile gate

### What was done
- Added Rust paired-complement test to the existing original native example and exact-source Actions workflow; pinned library regression passed 40/40.
- Checked candidate arithmetic independently against original affine formulas on 500 deterministic randomly chosen nonexceptional P31 scalar-multiple pairs.
- [CI #44223919906](https://github.com/isogeny-labs/crypto/actions/runs/44223919906) produced a Rust compile error in the new **corner-case test only**, not in original library: `find_map(|r| r.point)` yielded a bare `(u64,u64)` whereas `Point` expects `Option<(u64,u64)>`. No benchmark executed.

### Decisions made
- Restore the typed `Point` in the test with `map(|r|r.point).find(|p|p.is_some())`; do not change candidate group arithmetic, curve, test corpus or timings.

### Open threads
- Rerun exact-source component CI and preserve raw JSON; only then integrate complete cost.

### Errors
- Rust E0308 at line ~2004 (corner input type mismatch), to be corrected in next commit.

## 02:34 UTC — Exact 256-Q paired-complement component passes in Rust

### What was done
- [Source-specific run #44224128962](https://github.com/isogeny-labs/crypto/actions/runs/44224128962): Rust example compiles, 40/40 library tests pass and the 256-Q paired-complement test passes.
- Source test reports same first-shift hit/miss mask (244 successes, 12 exhaustively rejected), 309443 quotient representatives from 618885 original four-sums, four corner controls and 248 total signed witness replays.
- In the same interleaved AB/BA test, measured baseline query 1,899,587,307 ns vs paired 1,375,302,415 ns. Extra quotient setup 69,422,344 ns. Original diagnostic multiplications 192,014,281 versus paired 129,033,251; batch inversion windows 61,292 vs 27,645.
- Original source test passed but workflow's trailing grep gate failed because the quoted shell grep patterns accidentally contained literal backslashes. No test assertion failed.

### Decisions made
- Fix the YAML grep pattern only (no algorithm, corpus, or outcome change), preserve exact raw result, then integrate paired candidate into the *complete* unknown-log rank/256-Q harness.
- First component improvement is not a complete ECDLP speed comparison and not an algebraic RR9 attack.

### Open threads
- Repeat successful workflow gate and full rank/relation/256-target matched comparison.

### Errors
- Initial corner-test Option type failure fixed. New CI false-negative was a grep quoting error; to be corrected in workflow.
