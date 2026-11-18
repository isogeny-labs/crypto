# Prime31 Rust APS S3 x-fibre square-root precomputation — actual 24-Q fresh confirmation

**2026-11-16 UTC. Status:** Rust release GitHub Actions development execution completed on exact source `ca03804400d998efe17bb904aa8f757e92a6ff8f`, [run 43840938217](https://github.com/isogeny-labs/crypto/actions/runs/43840938217), job `133209109179`. All focused Rust tests and the CLI execution passed. Original unedited CLI JSON, all 72 complete scalar transcripts, source pin and full failed-round accounting: [S3_XFIBRE_MATCHED_CI_20261116.json](S3_XFIBRE_MATCHED_CI_20261116.json). Producer development result **[pcc:unverified]**, without independent PCC, EKS isolation, a mature external IC comparator, or a new security/complexity claim.

## Predeclared experiment and exact cryptographic fixture

Protocol [S3_XFIBRE_PRECOMPUTE_PROTOCOL_20261116.md](S3_XFIBRE_PRECOMPUTE_PROTOCOL_20261116.md) was committed at `735b7dc8` before source commit `e75a3f79` and before observed CI. It fixed a new caller-side 24-public-Q seed, the prior-best `[384,384,16384]` uneven APS point-list profile, six-round bound, alternating `legacy,xfibre,xfibre` / `xfibre,xfibre,legacy` order, independent scalar checks and the ≥20% complete wall saving, ≤5% candidate A/A drift, ≥18/24 paired-win exploratory gates.

Existing ordinary full-prime-order curve (no weak cofactor/smooth-order shortcut):

    p = 2147483647 = 2^31-1
    E: y²=x³+847514846*x+2087444724
    #E(Fp)=n=2147446831 (prime), cofactor=1
    G=(625037715,2100627251)

Each trial independently constructs source public points `R_i=a_iG+b_iQ` using the inherited Rust additive-walk generator. The scalar k for a generated `Q=[k]G` is retained *only by the caller-side verifier*. Neither arm receives k, precomputed pair-sum indices or candidate roots. Both arms solve *the same* one-relation APS-style S3 problem and independently replay the recovered scalar against Q.

## Mathematical identity and changed Rust code

For `S3(X,Y,Z)=a Z²+b Z+c`, with

    a=(X−Y)²
    b=−2((X+Y)(XY+A)+2B)
    c=(XY−A)²−4B(X+Y)

the exact polynomial identity is

    b²−4ac = 16(X³+AX+B)(Y³+AY+B).

Therefore computing arbitrary rational square roots `r_X²=X³+AX+B`, `r_Y²=Y³+AY+B` **once for each first/second-list abscissa** gives a square root `4*r_X*r_Y` of the pair discriminant for every pair. Candidate roots `Z=(-b±4*r_X*r_Y)/(2*a)` are the *same two* `S3` rational roots as the incumbent pairwise exponentiation, and reuse the incumbent row-batched denominator inversion. The diagonal `X=Y` linear S3/identity exception is handled by the unchanged code.

The source `native/examples/prime31_aps_one_relation.rs` now exposes `algebraic_mode_precomp::<true,true>` and an opt-in `--xfibre-confirm` panel. Its old `algebraic_mode` routes unchanged to `algebraic_mode_precomp::<FAST,false>`. The new discovery computes y-fibre square roots **from x and finite-field operations**; it never accesses stored source point ordinates or uses elliptic-curve additions in candidate discovery. An independent new Rust unit test verifies the discriminant factorization, old/new rational root sets and diagonal doubling on 53 public points and 2,756 off-diagonal ordered pairs.

**Classification:** this is a faster implementation of the published APS single-relation *index-calculus methodology*, but because the inputs are finite explicit public-point lists and every S3 pair root can be translated to EC signed sums, the algorithm is still **generic meet-in-the-middle equivalent**. This is an engineering result, not a novel non-generic ECDLP attack.

## Actual completed same-source Rust measurements

| Charged metric, 24 fresh Q | Incumbent per-pair S3 sqrt | New x-fibre | New x-fibre repeat |
|---|---:|---:|---:|
| Complete verified discrete logs | 24/24 | **24/24** | 24/24 |
| Cold in-process solver total (ms) | 461.210284 | **361.223183** | 359.105247 |
| Charged public-labelled source generation (ms) | 232.296561 | 233.989483 | 233.146878 |
| Charged S3 query/verification phase (ms) | 219.336433 | **117.618925** | 116.356176 |
| Actual finite-field square-root exponentiations | 1,297,536 | **19,968** | 19,968 |
| Actual S3 pair quadratics traversed | 1,297,536 | 1,297,536 | 1,297,536 |
| Fully paid attempts including misses | 26 | 26 | 26 |

- Full cold candidate/incumbent ratio **0.783207**, or **1.277× faster** in the paired observed panel; a **21.68% complete-solver wall reduction**.
- Candidate wins **23/24** paired public targets. Candidate A/A aggregate drift **0.5863%**.
- S3 sqrt exponentiations fall exactly **65×** because 26 trial rounds each precompute 384+384 =768, instead of one per pair trial. The measured root-search phase is **1.865× faster**. This changes *arithmetic*, not pair-query count, source distribution, relation yield or asymptotic cost.
- All 72 recoveries return identical target logarithms across arms; no altered signed-relation semantics, no candidate oracle fallback, no silent omitted unsuccessful rounds. The preregistered exploratory development gate emitted `passes_preregistered_development_gate=true`. It is not an independently signed scientific performance verdict.
- The original #4391 earlier [384/384/16384 fresh study](APS_UNEQUAL_PROFILE_384_FRESH_REPORT_20261116.md) reported 278.384ms for **different Q seeds and source/runner**. **Do not divide cross-run timings** to claim that the new 361.223ms is slower or faster than that different sample. Only the 461.210ms versus 361.223ms within-run comparison is a defensible measured ratio here. Similarly, no comparison against fastest mature external PKM/APS has run.

## Prior-lead assessment and next concrete experiments

The lead audit was performed before preregistration. Published APS 2018 (DOI 10.1016/j.ffa.2018.01.009) establishes single relation; PKM 2016 (DOI 10.1007/978-3-662-49387-8_1) establishes compositional rational-map factor bases; Semaev ePrint 2004/031 establishes summation polynomials. #4383 equal-size T1024 was rejected by fresh confirm, #4188 S3 root index was generic-equivalent and cold-slow, #4373 chained m8 and #4391 anchored m6 F4 reached only censored matrices; the #4391 exhaustive B4/B8 symmetric f validation achieved 26,400/26,400 decisions but no target-conditioned f recovery. #4228/#4349 selected-root/compact-certificate machinery lacks a complete B31 m8 root solver and has different control denominators. Shared KB was not loaded and its wrapper couldn't run in this connector environment; GitHub primary sources and publisher records are a bounded fallback, not an exhaustive novelty search.

1. **Non-generic m8 selected-root experiment (highest scientific priority):** use the already-tested symmetric `f(T)|H(T)^8` representation and existing Rust symmetrized Semaev / certified quotient machinery. Build a *target-dependent* polynomial/circuit in elementary-symmetric coefficients (try block sizes 3+3+2 first), returning original-ideal-certified rational roots without inspecting the `C(38,8)=48,903,492` candidate-f universe, a generic pair-sum support index, or precomputed scalar labels. Small B4/B8 positivity, infinity and multiplicity checks must match exhaustive ground truth before B31. Existing dense F4 censor is `undetermined`, never `UNSAT`.
2. **Algebraic cost frontier:** benchmark actual *completed* same-ideal original-root extraction against PKM-style rational-map IC, APS-style one-relation and native F4/quotient/certified workers, then independently against a mature msolve/Singular/Magma control where supported. Stage-only clocks or unsupported 31-bit native controls cannot be used as a speed ratio.
3. **Complete solver and independent validation:** once a nonenumerative m8 oracle yields natural signed relations, recover 12 new unknown prime31 logs, charge every target-conditioned query, preprocessing, failed/censored attempts, full-point verification and rank or one-relation coefficient inversion; then isolate repeated executions with A/A, obtain independent PCC, and compare optimized negation rho *separately* as full security control.
4. **X-fibre engineering follow-up:** optionally test x-only root caching across step orbits and alternative batch-inversion layout on a **new** preregistered panel. The current speed result is not an invitation to tune against its own fresh holdouts.

## Reproduction

    cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation
    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --xfibre-confirm

The actual source-bound GitHub Actions successful run is [43840938217](https://github.com/isogeny-labs/crypto/actions/runs/43840938217). No Python/C++ solver or paid EKS job was used. No leaderboard/PCC promotion is claimed.
