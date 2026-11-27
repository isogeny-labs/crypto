# Root repair and Prime31 original-Rust evidence recheck — 2026-11-24

[pcc:unverified] Read-only audit of completed external CI, not a new trial or a new theorem. The prior source-level reconstruction audit remains historical. No new randomized experiment, native compilation, cloud job, PCC claim, or worker execution by this invocation.

## Exact identity and result

- Main-build repair PR #5078 merged at 2026-11-24T15:48:21Z, actual merge ee44af42892026802dd55bfe96d383faea697926.
- GitHub Actions checked out synthetic merge a2df68ded11dec65b5cbaa9f8fe56a40ca5f1b15. Both Git commit objects have identical tree b1104b42d432d526083d25526c5f03ec47da2f24 and ordered parents ff7207be25f57bf27c225ce5fb09bd808730f606 and f4a5570be83710b7b4580076497abd72caeeb46a. This is exact tree identity, not merely a matching PR title or head.
- [Rust all targets compile](https://github.com/isogeny-labs/crypto/actions/runs/45427881542/job/138565812548): both workspace all-targets and wide-vars checks passed; crypto-analysis-core library tests 10/10 passed. The manual native full-library job was skipped, not passed.
- [Prime31 symmetric Newton and S3 F4 original-Rust gate](https://github.com/isogeny-labs/crypto/actions/runs/45427767861/job/138564343636): three original-crate test commands passed 2/2, 2/2 and 3/3; nine other scientific/control command steps passed, including the non-table m=2 x-only oracle through B677 and the planted six-variable four-point F4 example. Rust 1.100.0.
- [ecbench smoke](https://github.com/isogeny-labs/crypto/actions/runs/45427881976/job/138565342763): ecbench built; committed smoke spec validation failed, so fresh sessions and replay were skipped. Do not promote this to full correctness.
- The m2 example, guard example, and lockfile have identical blobs on the tested tree and #5131 parent: 246953d6b33af884f86578e080a7fa6cb06121b8, 7f991783bd99d4f95ec4df1a94b50e79be693b51, afc9511377b3549c26a44d8e285e865e2d2041b4. This does not prove whole-branch equivalence or execution of the guard.

```text
Historical malformed module -> repaired root -> exact merged tree compile PASS
                                             -> m2 original Rust x-only checks PASS
                                             -> full library manual SKIPPED
                                             -> smoke spec validation FAIL
                                             -> dedicated Dickson m4 query still OPEN
```

## Readback of already-executed component diagnostics

At each B, eight fixed x-coordinate targets: three hits with one x-pair each and five misses. These are m=2, not the task's four-leaf Dickson extraction. The source's base starts with GX and X2G then scans rational x coordinates; it is not the inherited B31 Dickson fibre.

| B | Newton sum of medians ms | Tree sum of medians ms | Newton/tree |
|---:|---:|---:|---:|
| 64 | 2.182969 | 2.396619 | 0.9109 |
| 128 | 7.950235 | 7.966029 | 0.9980 |
| 256 | 33.271762 | 27.813983 | 1.1962 |
| 677 | 232.294416 | 176.487517 | 1.3162 |

A separate cached-Newton panel at B677 reports 221.457411 ms including membership-polynomial setup vs tree 176.562488 ms and uncached Newton 230.392485 ms. Base construction is outside the timed loop. These are observed component diagnostics, not newly measured values or a qualified speedup claim. The m4 planted B4 F4 symmetric degree-B arm reduces reached degree 7 to 6 and columns 742 to 326 but reports 2,182,404 field operations vs 1,036,961 ordered; no complete signed recovery or natural-target rank follows from reduced degree or columns.

## Reproducibility audit

Source at checked tree, examples/prime31_monic_s3_pair_oracle.rs lines 320–471, uses a fixed u64 LCG initial state 0xC31B_A7ED_2026_1122, wrapping multiplier 6364136223846793005 and increment 1442695040888963407, candidate x = state mod p with rejection. This is one reused deterministic control stream, not independent replicate seeds. Two positive targets are planted for grading; GX and X10G are fixed controls. Five timings per target use alternating two-arm or rotating three-arm order and exact x-pair-set comparison to a reference. The source computes that reference before timing in the comparison loops, regardless of the printed postsolve_reference label. No solver input of witness indices is established by these reads. The unqualified comparison lacks an observed prospective seed manifest, independent replicate panel, shared corpus hashes, held-out natural targets, A/A and per-repeat raw timings/paired uncertainty in the inspected logs. The exact source uses deterministic root splitting with a finite cap; no new random draws were made for this audit. No historical seed schedule was invented or modified. Label as fixed-panel correctness and diagnostic timing only. [pcc:unverified]

## Next action and ownership

#4977 and #4979 merged; #5078 merged; #5128 remains open draft and owns current-root relation-pipeline qualification (source 9f3aec6264e7a91f81ff8ea81be8bc83ec458f53). Its listed all-targets workflow was skipped; do not duplicate active qualification. Reconcile #5131 with the repaired root through the existing coordinator; once admitted, freeze the exact dedicated original-root source and run the existing rational-chain correctness gates and then the matched Dickson four-leaf miss/sign/duplicate/rank controls. The compiled root at the merge is evidence, not authorization to launch experiments. No new field, curve, source test, paid compute, or Space mutation. No native worker dispatch, KB or Conductor was exposed; coordinator parallelized independent source and CI reads. Main and branch session journals remain divergent; append-only branch entry, preserve both in future integration.

Impact score: 4/10 — exact-tree CI evidence removes the historical compile blocker at a specific landed root and validates adjacent original-Rust m2 controls. No dedicated four-leaf extractor, independent validation, or qualified performance advantage is established.
