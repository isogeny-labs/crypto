# Prime31 RR9 public B8 chart extraction — frozen result and next gate

Date: **2026-11-17 UTC**. Source: [merged PR #4556](https://github.com/isogeny-labs/crypto/pull/4556). Evidence PR #4558. Scope **producer-verified [pcc:unverified]** only; no independent signed PCC or full-size non-generic ECDLP solution.

## Mathematical and implementation result

The original RR9 chart is F(X,Y)=(X-r)A(X)+(Y+s)V(X), with deg A<=3 and monic V=X^3+v2 X^2+v1 X+v0. For the frozen **distinct B8** abscissas x_i not equal to r and ordinates y_i !=0, a chosen sign epsilon_i gives the linear fibre constraint

    A(x_i)+c_i V(x_i)=0,   c_i=(s+epsilon_i*y_i)/(x_i-r).

Let w_i = (prod_{j != i}(x_i-x_j))^(-1). The eight signed values -c_i V(x_i) interpolate a cubic A iff

    sum_i w_i x_i^k c_i V(x_i)=0, k=0,1,2,3.

The forward implication is the leading-coefficient Lagrange identity applied to X^k A(X) (degree <=6). The converse follows because these four weighted-Vandermonde rows are independent and their kernel is the four-dimensional evaluation image of cubics on eight distinct nodes. Thus *for each sign mask*, a 4-by-3 affine linear system determines whether a monic-V chart exists. RREF handles possible deficient rank; Newton interpolation reconstructs A from four nodes, then all eight constraints and the full RR9 norm quotient M(X)=H_B(X) are checked. The original curve-point group sum separately replays the positive relation.

The new [native Rust child module](../native/examples/prime31_rr9_barycentric_core.rs) reuses existing `Arithmetic`, `Point`, `PRIME31`, factor-polynomial, finite-field and group-replay routines in `prime31_rr9_public_macaulay.rs`. The [new example](../native/examples/prime31_rr9_barycentric_public.rs) pins the full positive/negative panel. **The extractor enumerates all 256 public sign masks.** It is an exact B8 chart oracle, but equivalent to a generic signed-support search, not a novel algebraic advantage. Exceptional x_i=r, y_i=0, non-monic V and B31 repeated leaves remain excluded.

## Actual p=2^31-1 native Rust result

Original ordinary cofactor-one curve: p=2147483647, a=847514846, b=2087444724, subgroup order n=2147446831. B8 x_i=[0,1,2,3,5,6,8,9].

| Public R (frozen) | Test-only classification | B8 charts | Public candidate wall |
| --- | --- | ---: | ---: |
| (1911579686,548820096) | positive | **1** | **0.140161 ms** |
| (625037715,2100627251) | outside support | **0** | **0.135655 ms** |
| (1923055775,307731458) | outside support | **0** | **0.135685 ms** |

Every candidate enumerated all 256 signs, with no supplied masks or scalar. Recovered positive (a0,a1,a2,a3,v0,v1,v2) is (455835358,518652820,754716693,932383545,62510441,1884059825,1142049404), matching the existing #4550 test-only witness; original-curve group sum passed independently of chart interpolation.

**[Exact-source Rust 1.100 CI PASS #44114922651](https://github.com/isogeny-labs/crypto/actions/runs/44114922651)** (40/40 native lib tests, 2/2 example tests, release JSON assertions, bounded 1 GiB virtual memory). Original native library pinned to 1df714645aa683cf975bc76fcacb9842cb4780fd. Raw [artifact #14341594010](https://github.com/isogeny-labs/crypto/actions/runs/44114922651/artifacts/14341594010) stores all source-specific timings and coefficients. ZIP SHA256 `c8e9034540686b598153b702e0a133a2ef09152ed7741d33f5c14a1a744c18ff`, independently recomputed from downloaded 726-byte ZIP, matches GitHub's recorded digest. The single-run millisecond values do not carry isolated-host confidence intervals.

## Cost and prior-lead correction

The earlier **complete 1024-unknown-log prime31** measurement [#4532](https://github.com/isogeny-labs/crypto/pull/4532), a *different* experiment, had mixed-base IC 2,221.474 ms versus shared 2^20-baby generic BSGS 170.113 ms (103.840 ms setup, 66.274 ms online). B8 `0.14 ms` is **only a membership/chart-extraction component**, not a DLP solve, and cannot be compared to a complete-solver time.

B31's squarefree eight-signed supports comprise C(31,8)*2^8 = **2,019,513,600** formal possibilities, or C(31,8)=**7,888,725** separate B8 coordinate subsets. Naively multiplying the B8 chart work by that subset count suggests ~18 minutes per B31 public residual on the measured positive-run rate; this is an *unmeasured illustrative extrapolation*, not a complexity lower bound or a B31 benchmark. The generic support-enumeration barrier is why this is a correctness oracle rather than a promising standalone faster ECDLP solver.

Prior repository leads audited: #4539 RR9 direct polynomial, #4541 radical, #4545 signed fibres, #4546 exceptional point, #4547 supplied-anchor generic equivalence, #4548/#4549 Macaulay D5/D6 nulls, #4550 D9 rank gate, #4513 complete IC, #4532 cached BSGS; crypto-autoresearcher #3301 (lead atlas), #3305 (Semaev alternatives and errata); cryptanalysis #1536 (matched base controls). Literature context: Petit–Kosters–Messeng PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala FFA 2018 DOI 10.1016/j.ffa.2018.01.009; Kudo et al. CANS 2018 DOI 10.1007/978-3-030-00434-7_19. The barycentric interpolation identity is standard; hybrid exhaustive search has published precedent. Shared research-KB MCP/CLI unavailable in this connector runtime; bounded GitHub and primary-publication review used instead, not an exhaustive survey.

## Proposed experimental queue, in priority order

1. **Matched B8 generic baseline:** Rust 4+4 meet-in-middle with two 16-entry signed half-sum tables, setup and online costs separated, identical three R; verify equal chart membership. Expect generic to be competitive; do not promote sign enumeration as non-generic.
2. **Seven-of-eight quotient:** with the existing Rust sparse F4/quotient code, measure the actual dimension and any infinity/saturation components for seven public coefficient quadrics, then test multiplication by the eighth. The often-quoted dimension 128 is only a generic Bézout upper bound, not measured on this instance. Require public-R-only positive chart and replayable negative absence certificate, without masks.
3. **True B31 algebraic gate:** extend to all 31 factor-base points, repeated leaves, exceptional coordinates and normalization charts; **do not** enumerate 2.02 billion supports.
4. **Full unknown logarithm:** complete blinded Q recovery and index calculus factor-base logs using original Rust, all failed relation attempts, setup, linear algebra, verification, memory and code compilation charged; match same-host cached BSGS/tuned rho and published index-calculus frontier. Obtain independent signed PCC before speed/leaderboard claims.

**Status:** a previously unresolved *B8 public-only chart recovery* task was completed, benchmarked and merged in #4556. It does not establish cheaper complete prime-field ECDLP. The genuine nonenumerative B31 extractor and full solver measurement remain open.
