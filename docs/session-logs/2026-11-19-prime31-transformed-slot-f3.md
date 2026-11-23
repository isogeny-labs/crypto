# 2026-11-19 — Prime31 transformed-slot S3 public-target chart (continuation of #4812)

## What was done (commands, files, features)

- Re-read AGENTS.md, existing #4812 Rust and report, #4692 / #4822 F4 owner plans, #4816 fixed-unweighted negative result, #4728 complete 1,024-log generic grader, merged #4532 shared BSGS. Checked primary literature on Semaev summation polynomials and known first-fall-degree / naive-IC complexity caveats and bounded GitHub searches in crypto, cryptanalysis, crypto-autoresearcher. Shared research-KB MCP was not present; no novelty-by-absence claim.
- Reused existing #4812 lane. Registered the experiment in docs/ACTIVE.md commit `5106b0c0`, and **froze** the [protocol](../research/prime31_ic_20261115/rr9/TRANSFORMED_SLOT_F3_PROTOCOL_20261119.md) at `c745935b` before source and measurements.
- Derived a target-coupled direct S3 system with 14 unknowns, 15 equations, maximum degree 8, replacing the same-size rationally eliminated degree-10 Horner system by precomputed transformed slot roots. Derived a 7-variable 8-equation K_i product chart, bi-degree (16,16), ≤289 terms each. Proved finite sign-lift by backward rational-sum induction; no claim of full projective charts or F4 runtime advantage.
- Independently ran local Python BigInt affine arithmetic: all four unchanged Prime31 windows, fixed SplitMix seed `0x54524e5333463301`, 512 planted public Q, 4096 valid H roots, 3584 F3 and 3584 K identities, 512 poisoned-target rejections and 512 recoveries using only x-chain and public transformed factors; no exceptional partial sum observed. Computed exact K coefficient support 26×289, 1×288, 1×285, and direct system input 162/163/163/163 terms. These are only planted controls, not public-only extraction.
- Checked in original-crate Rust example `research/prime31_ic_20261115/native/examples/prime31_transformed_slot_f3.rs`, independently authored Python full-cohort verifier, report and source-specific GitHub Actions gate. Pin original crate `1df714645aa683cf975bc76fcacb9842cb4780fd`; add independent expanded K coefficient replay and sparse checkout (branch commits `a8f22d4b`, `59e39756`, `074fc889`, `4600ef08`, `9cd3f167`).

## Decisions made (and why)

- Maintain existing #4812 algebraic-chart lane, without creating a second F4 engine or claiming generic relation grading is a non-generic index-calculus solver.
- Exact comparator on original Prime31 remains fully charged 1,024-log same-runner against best index calculus, shared BSGS (historical 170.113ms/1,024) and optimized rho. 1,312 attempts imply <129.66 us per attempt as only an **optimistic necessary** extractor budget.
- Never conflate 7-variable count with faster solving; each K_i is up to degree 16 when univariate and naive staged branching can reach 16^7 partial assignments.

## Open threads (unfinished, pick up next time)

- Inspect current GitHub Actions run for the **latest source SHA** after sparse checkout; verify 40 original Rust library tests, focused Rust test, 512-case producer, independently replayed entire cohort and artifact hashes. Repair failures, freeze exact receipts in the report, tick ACTIVE line only after successful source verification.
- Hand the 14-variable degree-8 target-coupled chart plus 7-variable sparse K alternative to [#4692](https://github.com/isogeny-labs/crypto/pull/4692) as additional F4 ablation candidates, with previously frozen B8 46-positive/18-negative public-only corpus and zero generic table calls. Avoid changing live #4692 code owned by another concurrent session.
- Add projective/exceptional chart tests and independent signed PCC; only after public-only extraction passes, integrate with #4728 rank31/1024Q pipeline.

## Errors (what went wrong, how it was handled)

- Local runtime lacks cargo/rustc and external GitHub git connectivity. All Rust changes committed via authenticated GitHub connector; native execution must be observed through GitHub Actions, not inferred from Python.
- Corrected an independently authored JSON checker tuple-versus-list serialization mismatch at commit `4a947654`; no count was fabricated. Fixed report support-count wording at `038b3538`.
- Initial source-specific CI jobs were slow during monorepo checkout; updated workflow with sparse checkout at `9cd3f167`, pending validation.


## 2026-11-19 22:39 UTC — exact-source Rust and Python CI closeout

### What was done (commands, files, features)
- Source-bound GitHub Actions [#44630431346](https://github.com/isogeny-labs/crypto/actions/runs/44630431346) on workflow head `5a0942768fd81be66cb0bb4db38f04d7200fb8a6`: original pinned Rust library **40/40**, focused S3 test **1/1**, frozen 512-case release producer strict `jq` PASS (512 finite, 4096 H, 3584 F3, 3584 K, 512 poisoned-target rejects and sign lifts), separate Python full 512-case replay PASS.
- Evidence [artifact #14580424253](https://github.com/isogeny-labs/crypto/actions/runs/44630431346/artifacts/14580424253), original JSON SHA256 `8efc9f1a2d5d98fe3f5d2b46f844ae6a81150e80b702361017e80d029d544cdb`, independent JSON `be00dd6de9dcb67914982d7cdbc39f0986e5477fb9c55e3b0cac7e893c32523a`. Updated research report with exact failed precursor and successful evidence, checked off isolated #4812 experiment in docs/ACTIVE.md.

### Decisions made (and why)
- Successful correctness gate; retain **no public-only unknown-index extraction** and **no complete ECDLP cost advantage** status. Transfer the three representation candidates to existing #4692 for a paired F4 measurement rather than creating a duplicate solver engine. Maintain independent signed PCC outstanding.

### Open threads (unfinished, pick up next time)
- Frozen 46-positive/18-negative public-only B8 F4 extraction, exceptional projective charts, signed PCC, and full rank31/1024-Q end-to-end matched-cost benchmark if extractor passes.

### Errors (what went wrong, how it was handled)
- Before final PASS, sparse original-source checkout omitted `src/cryptanalysis/f4_fp_tower.rs` and `sig_fp_tower.rs` causing baseline compilation to fail. Added both in sparse checkout at `5a094276`, reran same frozen controls. The earlier failed run is retained as diagnostic evidence.
