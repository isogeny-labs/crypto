# Frozen-design pilot for M31 RR5 sector relation yield

**Status (28 November): conditional protocol, not a queued run.** [Merged #5208](https://github.com/isogeny-labs/crypto/pull/5208) compared these five sectors with coupled, sector-4, Graeffe and deflated RR5 on identical B677 inputs. None completed extraction or emitted a relation in 148 capped calls, including planted targets [pcc:unverified]. Do not repeat this same-formulation B677 pilot without a new smaller selector or solver premise that first completes a frozen smaller instance. The exact tuple counts below remain a combinatorial budget, not an F4 feasibility signal.

Prepared 2026-11-25 from public PR #5119 (sector constructor), #5120 (streaming query pipeline), #5184 (generic B677 cost reference), and the main active goal. This is a design and exact combinatorial calculation, not an executed solver panel. All quoted PR measurements are [pcc:unverified].

## Immutable public instance and admitted chart

Use p=2,147,483,647, E: y²=x³+847514846x+2087444724, prime group order 2,147,446,831, G=(625037715,2100627251), the exact 509 Dickson plus 168 Y7 public points and their frozen CSV/hash from #5119, and public Q=(976979610,1228979761) from #5184. The solver is given Q coordinates and no scalar. An independent verifier may use the published fixed check scalar only after solver output to check dG=Q. Reject identity, target-in-base, repeated-root, zero-y and denominator-exceptional cases as `UnsupportedChart` until a separate chart is implemented; do not count them as completed misses.

Keep one source/binary/protocol manifest and exact roster across all arms. The published #5119 sector proof gives five disjoint k cases (k Dickson, 4-k Y7) on the supported distinct-root chart, but does not supply solver performance or total cold cost. The #5120 visitor/adapter preserves stopped, complete, error, and unsupported states separately. Current PR #5120 reports a divergent PCC prefix against main; publication requires reconciliation before a branch is advanced.

## Exact sector budget

For 509 D points and 168 Y points, the number of distinct unordered four-point tuples in sector k is C(509,k) C(168,4-k). For a uniform public R in the order-n group, the expectation of signed tuple matches is exactly 16 times this count divided by n, by linearity of expectation. Sign and tuple dependencies make this **neither** an independence model nor a prediction of useful rank. The [calculation](rr5_sector_budget.py) gives:

| k D | k Y | Tuples | Fraction | Expected signed matches per uniform R |
|---:|---:|---:|---:|---:|
| 0 | 4 | 32,018,910 | 0.00369 | 0.239 |
| 1 | 3 | 395,093,944 | 0.04554 | 2.944 |
| 2 | 2 | 1,813,624,008 | 0.20905 | 13.513 |
| 3 | 1 | 3,670,688,112 | 0.42312 | 27.349 |
| 4 | 0 | 2,763,940,751 | 0.31860 | 20.593 |
| **All** | | **8,675,365,725** | **1** | **64.638** |

Sectors 3 and 4 hold 74.17% of tuples. A pilot order of 3, 4, 2, 1, 0 maximizes expected *raw* witness opportunity if sector solver cost is equal. Since k=4 touches no Y column, rank and target projection may favor a mixed sector despite lower tuple count. Record useful rank gained per sector and overlap; do not choose order after looking at target outcomes. The ratio 677/64.638=10.47 is a deliberately optimistic relation-count reference, requiring at least 11 uniform queries if each yielded its expectation and every row contributed new rank; it is not a valid stopping or complexity bound.

## Executable comparison when tracked on a draft branch

1. Freeze 3 eligible ordinary M31 curves (including the fixed B677 curve), public target coordinates and blind scalar custody, exact factor-base build, query generator and 64 logical `(a,b)` query labels per target, and the five-sector order above. Freeze limits and a same-host A/A control before solver calls. Use a hard process watchdog outside cooperative visitor deadlines and retain all censored rows.
2. For each query, form R=[a]G+[b]Q from public coordinates. Run each sector through the current original-Rust RR5 constructor/F4/extractor. Emit complete source/solver status, elapsed CPU/wall, peak RSS, field operation and F4 counts, all signed lifted candidates, rejected candidates, duplicates, and every claimed row's group identity. A second implementation replays rows and modular rank, with no producer scalar access.
3. Feed accepted rows to the same exact large-prime/factor-base elimination and stop only at a pinned target coefficient and independent dG=Q. Retain sector-specific raw count, unique count, marginal augmented rank, target-projection gain, and cold work per rank. Record setup, all five sectors attempted, failed/censored calls, graph work, elimination, and recovery.
4. Reference arms: the strongest current admitted IC implementation (the 25 November #5184 sampled-13 arm is a historical reference; #5322 reports a newer graph-control implementation) and an equal-engineering rho with negation map. Use the same target and resources, independent seeds, interleaved cold A/B blocks, A/A and positive controls. No speed claim from the shared development VM or a single curve.
5. Primary decision: complete cold single-target cost at fixed success criterion versus the strongest admitted IC baseline. Secondary decisions: native F4 work and useful rank/second by sector. If the RR5 path remains censored, report the exact chart/work/time boundary and do not impute a zero-relation result.

## Immediate implementation gate

The public #5119 code is construction-qualified only, while #5120 provides a streaming adapter but explicitly marks `phase_accounting_complete=false`. Before scientific execution, wire per-call owned accounting across construction, F4, roots, lift, rank and target check, and independently prove the resulting sum equals the outer cold interval on a small fixture. That is the narrowest actionable code task. A draft PR and session-log entry are mandatory under `crypto/AGENTS.md`; this workspace currently has an invalid GitHub token, so this plan remains outside the repository.

A [source audit of the current streaming code](RR5_ACCOUNTING_SOURCE_AUDIT.md) found opt-in F4 visitor timings already implemented but disabled in the extractor. Its callback includes signed lifting and adapter row work; these timings must be treated as nested, while pipeline setup and base construction need separate outer timing. The source audit specifies the smallest accounting patch and status controls.
