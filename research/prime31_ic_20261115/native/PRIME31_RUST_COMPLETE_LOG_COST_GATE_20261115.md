# EXP-R31-O: Complete native Rust prime31 log and generic-cost comparator

Date: 2026-11-15. Evidence **[pcc:unverified]** producer GitHub Actions, not independent replay. This is the same ordinary cofactor-one curve `p=2,147,483,647; a=847,514,846; b=2,087,444,724; n=2,147,446,831; G=(625037715,2100627251)`, never a deliberately weak curve.

## Complete algorithm and independently replayed target equation

The checked-in Rust `examples/prime31_s3_root_index.rs` exposes `--solve-one`: it receives only the public target point `Q=(1477799919,824532935)`, constructed by the known-answer *caller* as `[1038579171]G`. The source scalar is **never passed to the algebraic discovery oracle or solver**. The solver constructs the canonical B31 Dickson factor base and the algebraic split-S3 x-support index cold, generates public mixed-walk points `R=A G+B Q` with tracked `A,B`, then obtains true signed four-point equations `sum_i c_i P_i=R` using coordinate-only Semaev S3 membership with **no ordinates and no EC sums in discovery**. Group arithmetic checks each candidate identity AFTER discovery, not before. Incremental Gaussian elimination modulo prime n solves `sum_i c_i log_G(P_i)-B log_G(Q)=A`. At rank 32, code independently checks every factor-base logarithm and the recovered target logarithm using Rust EC scalar multiplication.

This is **a complete factor-base relation-collection plus linear algebra solver for the witnessed target**. But the algebraic coordinate-root index enumerates all two-point sums/differences; it is mathematically **generic meet-in-the-middle equivalent**, NOT a new non-generic / subgeneric prime-field attack. This explicit classification prevents claiming the original top-level goal has been met. A complete result on one target also does not prove lower cold cost than optimized rho.

## First source-specific CI: completed but overcharged measurement

[GitHub Actions #43775662865](https://github.com/isogeny-labs/crypto/actions/runs/43775662865), job `133001256603`, source `83aba43bf7c5a2bca861de3d51ae54b49795559c`, pinned Rust 1.100.0 Ubuntu 24.04 release, passed **40/40** reused-library tests, **3/3** S3/m4 and **3/3** new index/LA tests, the 16,384 target panel, and `--solve-one` verification:

- Full 31 factor-base logs plus public target logarithm internally recovered and verified, `verified_complete_log=1038579171`, `complete_ECDLP_recovery=true`.
- `coefficient_rank=32`, `independent_equation_rows=32`, `algebraic_nonempty_targets=86`, `verified_relation_rows=86`, `dependent_rows=54`, `visited_distinct_targets=249987`.
- `full_cold_wall_ns=136479297327` (136.479 s), `algebraic_discovery_ns=136212043740` (136.212 s), `cold_setup_ns=799343`, `step_setup_ns=1698845`, `linear_algebra_ns=394100`. Target-generation and verification are separately charged in the source output; fixed base construction is included in cold wall.

**Detected measurement defect immediately after checking logs**: when rank 32 and full independent replay succeeded, an unlabeled `break` exited only the inner signed-row loop, not the outer target-collection loop. Thus the 136.479s measurement continued scanning to the entire 250,000 cap **after a verified solution already existed**. It is a correct upper-cost execution record but **not an actual time-to-first-complete-log benchmark**. No speedup or competitiveness inference may be drawn from it. Source fix `9996fa009442b881eb915853d7392842d8a06a4f` introduces `'walk: for turn ...` and `break 'walk;` after the full replay. Record final source-specific measured new cold wall and stop count ONLY after the new GitHub Actions CI passes.

## Matched generic benchmark predeclared

New Rust flag `--bsgs-one` solves exactly the **same public Q** with full cold HashMap baby-step/giant-step using the existing Rust `Arithmetic` over the same curve, checks the discrete log and reports setup, baby/giant intervals, memory entries and full cold wall. This is **plain BSGS**; it is *not* the strongest optimized negation-map rho. It is a necessary generic sanity control, not an acceptance-quality final optimized-rho comparison. Side-by-side same-runner evidence needs to show the true cold ratio; do not extrapolate ratios from separated hosts. The source branch commits the two implementations and single Rust CI workflow before the paired result; no Python or C++ implementation.

## Research assessment and next non-generic experiment

The S3 pair-root index is an engineering reference, not novel IC: its degree-961 support is just finitely many algebraic x-coordinates of pair sums, equivalent to a generic table. The 16,384 scalar-fixture panel produced exactly 7 signed four-point relations of coefficient rank seven. To be truly non-generic the attack must extract relations from target-dependent algebraic S4/S3/F4 or certified-quotient identities **without enumerating enough finite pair/sum support to reproduce MITM**.

Previous lead files and PRs checked: #4188 m4 algebraic factorized S4, #4228 certified quotient/subresultants, #4349 real native worker (bounded F4 incomplete), #4275 dense-vs-tower F4, #4077 complete m8 cost gate, #4338 no-hidden-label selector, #3877 chained transfer, cryptanalysis #1536 auxiliary IC, PKM 2016 and APS 2018 published index calculus. Repository research-KB was unavailable. The next executable threshold is full **m8 target-dependent** extraction on naturally sampled public points with rank ≥ B+1 on 12 independent target scalars, full cold time against optimized negation rho, independent PCC. Distinguish cap from actual no-relation outcomes.


## Corrected first-complete-log CI (supersedes overcharged wall)

Exact-source [Actions #43776127389](https://github.com/isogeny-labs/crypto/actions/runs/43776127389), job `133002722026`, commit `9996fa009442b881eb915853d7392842d8a06a4f`, Rust 1.100.0 release **SUCCESS**; 40/40 reused-library and 3/3+3/3 focused Rust tests passed. After fixing the outer-walk break, same held-out public `Q` had all 31 factor-base logs and unknown `log_G(Q)=1038579171` independently checked at **rank 32** with **32/32 verified independent equations**, **84,554 distinct targets**, only 3 duplicate public targets, first relation at turn 2011, and **0 dependent equations**. Measured `full_cold_wall_ns=46061407746` = **46.061407746 seconds**; target-dependent algebraic discovery `45998997867ns` (45.998998s), cold setup `808718ns`, public-walk target generation `26757428ns`, postdiscovery signed verification `466675ns`, modular linear algebra `77206ns`, step setup `1719307ns`. All timing categories are source-bound producer measurements; no optimized rho yet. This supersedes the prior overcharged 136.479s execution, which scanned the rest of the cap after recovering the log. `matched_optimized_rho=false` and `new_non_generic_attack=false` remain deliberately explicit in the emitted JSON.



## Same-source **measured** Rust cold BSGS comparison: decisive negative

The first **actual same-host** Rust-only paired execution completed [GitHub Actions #43776311207](https://github.com/isogeny-labs/crypto/actions/runs/43776311207), job `133003280567`, exact source `ad8349cc9be584914d223082b7890d9309c2b902`, pinned Rust 1.100.0 Ubuntu 24.04 single thread, **PASS** (40/40 library, 3/3 S3, 3/3 index/LA tests and both CLI tools). Both solvers recovered and verified the **identical public Q** and independently recomputed `log_G(Q)=1038579171`. Q's fixture scalar was not supplied to either solver.

| Fully cold Rust algorithm on exact same Q | Measured full wall | Work (source JSON) |
| --- | ---: | --- |
| S3 coordinate-root index, factor-base IC-style relations + 32-variable modular LA | **46.200742075 s** | 84,554 distinct public mixed-walk targets, 32 independent verified equations, rank32, 32 scalar replays |
| Plain HashMap BSGS, full setup, baby/giant, scalar replay | **0.023613920 s** | 46,341 baby table entries, 22,412 giant-step probes, 68,880 group additions including curve certification |

Thus **plain generic BSGS is 1,956.50× faster** than the complete coordinate-root-index relation/LA solver on this Q in this one paired CI run. The BSGS stage breakdown is baby `16009812ns`, giant/replay `7133545ns`. Root-index breakdown is algebraic target discovery `46111819788ns`, base setup `802046ns`, public-walk `26807705ns`, verification `489279ns`, LA `133391ns`. Full cold wall for both includes the fixed validation steps each function calls, but excludes the same external fixture Q construction. These are same-host sequential process runs, not randomized repeated medians, not an independently reproduced PCC, and **BSGS is not the optimized negation-map rho** requested for final validation. Since the complete algebraic-root-index algorithm is generic MITM-equivalent, this is a clean **negative result**, not evidence of an exploitable prime-field elliptic-curve weakness.

**Decision:** stop optimizing this S3 root-index *as an attack candidate* at B31. Retain it as an independently tested exact oracle and benchmark control, and direct further cryptanalytic work to target-dependent non-enumerative m8 quotient/F4/subresultant circuits in #4349/#4228. Any candidate cheaper than rho must produce rank32 and full target scalar recovery at lower **whole-call cost** without hidden generic point-sum table or amortization. Repeat over 12 held-out synthetic unknown-scalar Qs and compare against optimized Rust negation-map rho before making any accepted fastest or security claim.
