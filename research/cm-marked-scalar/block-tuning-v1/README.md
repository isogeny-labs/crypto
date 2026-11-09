# Complete block-aware parameter census

CM-BLOCK-PLAN-CENSUS-V1 completed its single27090-row untimed census. Independent GMP suffix-DP replay verifies every split, digit expansion, prefix/table inventory, single-column schedule, bounded-block minimum and transition count, plus80 outer minimum sets. All400 selected original-mu point/counter cases pass. Four fault fixtures reject at their intended checks even after raw checksums are updated. Fresh extraction/replay and deterministic repacking are byte-identical. No scientific scan was repeated.

| Curve | V3 fixed-default block D+A | Tuned block relation | Tuned same-bank single relation | Tuned block binary | Prior optimized direct |
| --- | ---: | ---: | ---: | ---: | ---: |
| P-192 | 231 | 230 | 231 | 229 | 228 |
| P-224 | 265 | 265 | 266 | 260 | 260 |
| P-256 | 303 | 303 | 303 | 300 | 300 |
| P-384 | 452 | 451 | 451 | 450 | 449 |
| P-521 | 612 | 611 | 612 | 606 | 606 |

The relation minimum still exceeds both the matched block-binary and prior direct/binary references on every curve. All three declared sensitivity weights retain losses. These are exact group-call counts within the frozen family; no field, allocator or CPU-time price is inferred. V3 values are historical fixed-default context, not this census's decisive comparison. The stronger direct/binary donor is the already published residual-v1/plans.json; no donor data collection was repeated.

Primary selected relation (radix,offset,width;D,A,N,T;physical bank;normalized entries;planner transitions): P-192(13,1,4;190,40,18,6;10;4;676), P-224(11,3,5;220,45,17,10;14;8;829), P-256(11,1,5;251,52,22,10;14;8;928), P-384(19,-2,6;379,72,26,18;23;16;1553), P-521(7,-1,6;520,91,33,18;21;16;2139). Every full vector and all20curve/weight records remain in plans.json. The four sensitivity weights select among the emitted programs whose internal DP still optimizes D+A,N,D,steps; they are not globally weighted-optimal addition-chain claims.

## Scope and full-cost dependency

The domain is exactly five curves x(certified ell plus binary1,2,4,8,16,32) x129offsets(-64..64) x6widths(2..7). Each row pairs the original single-column bank schedule and the bounded16-column folded program. The original mu, subgroup, point-output target and complete prefix/table work remain unchanged. The independent suffix DP enumerates all16-column possibilities without the producer's pruning; small exhaustive qualification passes90cases. Producer counters match630 paired actual point/counter boundary cases before data. No point or curve is selected after seeing the census.

This is offline parameter selection, not free per-call planning. Search cost remains attached to the artifact. A later first-use or e2e comparison cannot omit generation, input-dependent planning, allocation, bank normalization or destruction. No native point-action clock ran here and the prior V3 admission refusal remains unchanged. The original e2e objective remains active; this census rejects the declared group-count prediction for this finite family and does not decide runtime or all possible constructions.

## Source, protocol and resources

Source/qualification850a36dd03da, frozen-record24b03006a877 and guarded launch b700cb0bb8fa were verified hosted and PR3674 updated before the first row. Canonical body digest ae0ddaa50b9d411171f3464998c50df18fec2836dfb862ff82e1678eb0fbb2d2 binds eight inputs, nine sources and producer/replay binaries; the launch additionally verifies and records all three qualified executables. The one scan and all controlled replays/point checks stayed within the declared120seconds/2GiB/thread1 invocations. The native backend receipt accounts for17 invocations, including the failed first build, under the20-invocation cap. No cloud job/spending, role/cap/budget change or generator/controller restart.

## Custody and replay

runs.tar.xz is110312bytes, SHA25611f5de7252cc1a9d2f26dd878b8eb8c5b93780e843e9ee79de9415bb71c559c8. The local raw runs tree is ignored. Verify `sha256sum -c runs.tar.xz.sha256`, extract into a fresh directory, then verify runs-manifest.sha256 against that copy. A fresh native --replay output is byte-identical to plans.json; the repacked archive is byte-identical too. This replays analysis, not the scientific scan.

From the repository root, after building the exact published native replay source:

```
/tmp/cm-marked-scalar-build/block-tune-replay --replay research/cm-marked-scalar/fixtures-v2 research/cm-marked-scalar/protocol/block-plan-census-v1/protocol.json research/cm-marked-scalar/block-tuning-v1/runs/census
```

The replay intentionally binds its exact binary and input/source hashes; a different build needs a separately recorded replay binding, not an edited old frozen protocol. The first replay compilation failed on mixed-type auto declarations; original bytes/logs remain in qualification/block-tune-build-v1. Final v2 and native fault-tool builds are warning-free. No root Cargo suite is claimed, and no Rust implementation changed. KB unavailable as previously recorded; existing block/residual contracts and cited primary precomputation background checked. No novelty, global-chain optimality or expanded quotient claim.
