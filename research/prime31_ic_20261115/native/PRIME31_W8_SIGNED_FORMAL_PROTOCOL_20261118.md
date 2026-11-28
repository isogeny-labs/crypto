# Prime31 weighted-eight signed formal-support audit — frozen protocol

Date: 2026-11-18 UTC. Extends existing Prime31 exact formal-support lane PR #4722; prospective comparison with weighted RR9 chart lane PR #4728.

## Exact experiment
On the **unchanged** ordinary cofactor-one Prime31 curve `p=2147483647, a=847514846, b=2087444724, n=2147446831` and the canonical first-x factor base, use eight signed scalar weights `(1,3,5,7,11,13,17,19)`. Count the number `F_B^{±}(w)` of distinct *integer coefficient vectors* `c=(c_0,...,c_{B-1})` obtainable as `c_i=Σ_{j:index_j=i} ε_j w_j`, with each `index_j in {0,...,B-1}` and each `ε_j in {-1,+1}`.

This is a formal support bound. Different vectors can still map to the same curve point; no hidden scalar, known logarithm, generic group-sum table, or ECDLP relation oracle enters the count.

## Independent finite algorithms
A. Native Rust: for all `2^8` sign choices enumerate all `Bell(8)=4140` restricted-growth partitions, compute each block's *signed* coefficient; discard zero blocks; canonically sort nonzero coefficients and keep the minimal number `h` of blocks realizing this signature. For every pattern with `h<=B`, `k=number_of_nonzero_blocks`, add `B!/(B-k)! / Π_v multiplicity(v)!`. Include the all-zero coefficient vector where attainable. Use `u128`; assert original Prime31 `check_curve` and its actual first-x point basis.

B. Independent Python: enumerate set partitions by growing subsets/bit masks (not restricted-growth strings), apply signs and independently recompute the same signature-minimum map and falling-factorial sum. Independently brute all `2^8 B^8` signed indexed words at `B=2` and compare exact coefficient vectors.

## Frozen tests/acceptance
- Partition count exactly `4140` per sign assignment and `1,059,840` across all 256 signs.
- Verify `B=2,3,4,8,12,16,31`; assert `F_B^{±}<=256 B^8`, monotonic `F_B^{±}`, and Python/Rust equality per B and support-size stratum.
- `B=2`: exact signed word enumeration must match. Include B1 as a cancellation sanity control.
- For **uniform independent shifted public residuals**, report only the rigorous `Pr[representable] <= min(1,F_B^{±}/n)` bound and corresponding expectation *lower* bounds for successful membership hits. Do not apply to arbitrary adaptive nonuniform targets or mistake formal count for actual EC coverage.
- Report the unsigned comparator `F_B^{+}` for W8 separately: positive-only support must **not** be used as an upper bound for signed RR9.
- Compare with #4728's B4/B8 natural-target acceptance protocol. If the formal bound predicts rare hits, require deliberately *planted, sealed* positive charts and independently enumerated grader-only controls before interpreting missed unplanted tests as an extractor failure.

## Reproducibility and status gates
Reuse the original `prime31-algebraic-ic` Rust crate, with pinned baseline source `1df714645aa683cf975bc76fcacb9842cb4780fd` in GitHub Actions and Rust 1.100.0. Run library tests, focused example tests, original-curve source record, and Python replay; archive JSON. Publish raw run links/hashes when actually observed. Benchmark arithmetic cost is **not** part of this finite counting protocol. Signed PCC independent validation remains separate.

Prior explored: #4714 (complete generic 3+3 M6 loses matched shared BSGS), #4716 (mask-only support obstruction), #4722 (positive M6 formal quotient), #4728 (signed W8 RR9 certificate but unknown-chart extraction open), #4692 (RR9 F4 lane), #4698 (independently unverified subexponential claim). Literature: Semaev (2004), Yokoyama et al. (2020), APS prime-field variant (2018), and the Bochum 2026 group-network proposal. Research KB not available through the present connector; this is a bounded primary-repo/literature audit, not a novelty claim.

**Scientific success here** is a rigorously checked natural-hit ceiling and a bounded experiment design, **not** a cheaper complete non-generic solver.
