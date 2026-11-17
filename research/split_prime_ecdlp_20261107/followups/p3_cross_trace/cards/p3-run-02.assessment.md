# Assessment of CARD-p3-cross-trace-20261107-2

card_id: CARD-p3-cross-trace-20261107-2
card_sha256: 2ef981a0cc1db9393611cfe12034720f4d70519b751c47f6ed2550a4c4e9503e (matches receipt.card_sha256)
receipt_sha256: d9c84a4311bad5d95acf79b6361483bd50ff95bde4d740ed6d9f80097eb49641 (matches)
relay_verbatim: not_checkable (see reasons 9 and 10)
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed

## Reasons (each rests on a file or output)

1. Receipt status completed, commands_total 16; receipt commands list has 16 entries; for every index the receipt argv equals the card argv exactly (programmatic comparison), attempts 1, timed_out false, exit_code 0 (cards/p3-run-02.receipt.json).
2. All 17 binds (PROTOCOL.md, followups/README.md, p3census README/Cargo.toml/Cargo.lock/10 src files, and the extra prebuilt-binary bind at the scratchpad path, 7cefdb9f...) re-hash to the card's sha256 now. binary-sha256.stdout prints 7cefdb9fbc817ede7241a3b606a162ab01002de275cfa1144f26707c7e63bd4a, equal to the bind. The build step ran 0.059 s ("Finished release in 0.01s"): it reused the prebuilt binary, not a fresh compile.
3. Every command's stdout and stderr file exists and its sha256 equals the receipt's stdout_sha256/stderr_sha256 (0 mismatches). All 12 declared output files exist and re-hash equal to the receipt output digests (0 mismatches).
4. Seeds, primes, counts match PROTOCOL.md Addendum A table: p24 P=16777213 SC/SA/SB/SZ=11787384277891755733/11424068243091161927/16739699506839864087/9456636048090206450, NP=502; p28 P=268435399 seeds 9196315568214002056/6471924370547007479/13467469337155996127/14287345612908892213, NP=2007; targets 256, count 2000, perms 1000, alpha 0.05, planted 0.02, family-size 16.
5. Covariates: p24 argv has z_691,logL1_200000,psi3_roots,psi5_roots; p28 argv has z_691,logL1_200000,psi3_roots (psi5_roots omitted, as Addendum A requires). The string psi5 does not occur in runs/run-20261107T2220Z/p28/analysis.json or p28-analyze.stdout/stderr.
6. Selftest (selftest.stdout) ends "ALL SELFTESTS PASSED".
7. Write scope: git status shows new paths only under cards/ (p3-run-02.yaml, .receipt.json, .launch.log, and p3-run-01.yaml pre-existing), runs/ (run-20261107T2220Z and the card-1 directory), and an analysis/ directory (untracked, p3_cross_trace/analysis/, not written by the card; created by the main session, outside the card's write_scope but not by the card's commands). Other dirty paths (benchmarks/records/*, p1_rr_dickson, p2_twist_coset) were already dirty before the run (receipt.dirty_before). benchmarks/records/20261107T2144Z-split-prime-p3-cross-trace.json is the BENCHREC record the main session named at launch.
8. Card 1 directory runs/run-20261107T214600Z/ is untouched since it was killed: newest mtime in the tree is p28/rhoA.jsonl at 2026-11-07 22:13:53 (a partial 39,985-byte file); directory mtimes 22:08:32; card 2's directory was created 22:2x and all card-2 writes are 22:22 onward. Card 1 contains no p28 rhoB/yield/analyze files and no p28-rho-*, p28-yield, p28-analyze logs. Comparison facts only: card-1 p24 curves, cov, rhoA, rhoB, analysis.json and p28 curves, cov are byte-identical to card 2's; card-1 p24 yield.jsonl differs only in the per-line "secs" timing field (identical with that field removed).
9. Harness deviation, declared by the main session and not by the executor: card 2 was launched detached with `setsid nohup env BENCHREC=... python3 tools/task_card.py run <card> > p3-run-02.launch.log 2>&1 &` instead of relay to a live executor-mechanical, because card 1 died at the executor shell's 30-minute background limit. The commands that ran are the tool's, from the card, per the receipt (reason 1). I classify the execution as executed_as_specified, with this launch route recorded; the main session should decide whether it also wants it noted in the research record.
10. `tools/task_card.py check-relay <card> --report p3-run-02.launch.log` prints CARD FAIL (lint: "names repository file ...runs/run-20261107T2220Z/... that binds does not pin") for all 30 generated run files. This arises because the run outputs now exist on disk (lint treats existing repository files in argv as unpinned); the card was presumably linted before they existed, but I did not re-lint at that time and did not verify it. No executor text file was supplied (the tool ran detached; the launch log is the tool's own printed output), so relay_verbatim is not checkable; the launch log's per-command digests were not compared line by line to an executor text.
11. Commit: receipt.commit and launch log commit are 33f47d761a303df62b468ee182d3606f0233fee7; repository HEAD now is 1865166f34838a5ea77f2b73310bc820b12585d1 (later commits exist). The card has no `commit:` field.
12. Timing recorded in receipt: p24 rho A 678.70 s, rho B 679.34 s; run completed by 2026-11-08 01:41 UTC.

## Not assessed

Analysis outputs (analysis.json decisions, tests, controls, degenerate_covariates), rho/yield statistics, and any interpretation against PROTOCOL.md acceptance criteria. Those belong to the reasoning tier.
