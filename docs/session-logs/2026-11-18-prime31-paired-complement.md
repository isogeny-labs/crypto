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

## 02:35–02:40 UTC — Complete 256-Q source-specific Rust and separate arithmetic replay

### What was done
- Integrated optional paired relation lookup into existing \`complete_b31_1024_tests\` source module, preserving default Dickson/first-x test behavior; added \`PRIME31_B31_NEGATION_COMPARE=1\` AB/BA paired-versus-original option and source-specific Actions checks.
- Frozen exact-source [Actions #44224597718](https://github.com/isogeny-labs/crypto/actions/runs/44224597718), source \`b49311293f2617b0187b3a729645850909e31530\`: PASS, original Rust lib 40/40; 256-Q component 1/1; 256 complete unknown-log AB 1/1 and BA 1/1; success gates all pass.
- AB fully charged: original 3,117.049332 ms versus paired 2,461.077553 ms (1.26654×), both 256/256 target logs, rank 31/31, 16/16 extra relations, first shift 244/256 and zero BSGS fallback.
- BA order reversed: original 3,117.006964 ms versus paired 2,484.184035 ms (1.25474×). All matching public target Q and recovered k unchanged.
- Component on final run: 2,161.949278 ms original versus 1,584.961158 ms paired, extra canonical preparation 89.824484 ms; 244 positives, 12 exhaustive negatives, 309,443 representatives and four corner controls.
- A separately authored BigInt short-Weierstrass checker, without candidate Rust arithmetic, replayed all 62 AB factor-base logs and base points, 62 modular relation rows, 512 AB target scalar products, 512 signed eight-point witnesses and 512 corresponding AB/BA public Q and recovered k, all passed.
- Committed full raw JSON with source/artifact metadata, report, lead audit and nonduplicate next experiments. GitHub artifact ID 14391903238, recorded ZIP digest sha256:b5110f3979f88d08ea95529a05f64db6c544f95deeb25085dff9bc9b226ecd6b.

### Decisions made
- Report specifically a 20.3–21.0% *complete generic-equivalent control* gain; this does not prove a novel non-generic prime-field attack or beat cached BSGS.
- Do not merge while stacked on unmerged #4635; independent signed PCC and host-calibrated performance review remain outstanding.
- Next hard mathematical gate is sign-free public-residual RR9/S9 or K961/S5 using existing Rust polynomial, quotient and rootfinding infrastructure, coordinated with #4604/#4596.

### Open threads
- Independent signed PCC validation of the raw evidence; repeated new-host ABBA.
- Sign-independent algebraic extraction and complete 1,024-Q unknown-log cost comparison to matched BSGS, optimized rho and published index calculus.

### Errors
- Initial Option typing compiler defect fixed; later source-specific grep quoting false-negative fixed; final exact-source CI all green. No reported failed original-curve relation/target checks.
