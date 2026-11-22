# Prime31 run 52 — exact signed x-only cosets and source-linked F4 factor-base extraction

**Claim classification:** exact algebraic identity + independently executed archival replay; Rust F4 extraction candidate checked in but not compiled/benchmarked yet. Not a public-only multi-leaf F4 root, complete index-calculus solver, or measured ECDLP advantage.

## Proven identity (arbitrary odd-characteristic field)

For E: y²=f(x)=x³+Ax+B and any public nonzero c, the condition y⁷=c is **exactly** equivalent to

```
f(x)^7 - c^2 = 0
y = c^(-1) f(x)^4.
```

Proof: y²=f implies f⁷=y¹⁴=(y⁷)²=c² and f⁴/c=y⁸/y⁷=y. Conversely if f⁷=c² and y=f⁴/c, then y²=f⁸/c²=f and y⁷=f²⁸/c⁷=(f⁷)⁴/c⁷=c. c≠0 implies f≠0 and the lift is unique: the opposite y has seventh power -c, not c. No square-root ambiguity, exceptional zero-y root or discrete-log oracle.

On the unchanged prime-order E(F_(2³¹−1)) from archived runs 45–50, each of eight fixed public y⁷ cosets has 21 points. Each monic `f(X)^7-c²` has degree **21** and 21 distinct archived Fp roots. Direct coefficient multiplication independently verified eight exact identities

```
f(X)^7-c_i² = product_{j=1}^{21} (X-x_{ij})  (mod 2147483647).
```

This proves **complete splitting with exactly those 21 roots**, even over the algebraic closure. The eight lists have 168 distinct x coordinates (no ±c partner overlap). Each polynomial contains **21 nonzero coefficients**. A signed factor-base point is recovered from its x root by a single known-constant inverse and a degree-12 polynomial evaluation, not Tonelli–Shanks.

## Replayed measurements and evidence

Local independent Python bigint-field arithmetic checked the archived run50 source fixtures, without reusing its C++ addition code: **168/168 correct signed lifts**, **168/168 wrong-sign rejections**, **8/8 full coefficient factorizations**, and **4,090/4,090** saved hash-to-curve seven-leaf witnesses independently resummed using *only the recovered x roots and their algebraic signed lifts*. Observed exceptional addition denominators: **0**. The six archived seven-leaf misses were not converted into positive roots or certified impossible by this verifier.

Frozen inputs from the user-provided run50 archive, not yet present on the PR: `records/eight_leaves.tsv` SHA256 `127fe818a2712ad942af86db78bfba8e87cc5bbac432c9d00c790f84ef1b0472`; `experiments/prime31_run50/records/adaptive4096.csv` SHA256 `07dfd381c1d069f4610e907ca3e7f9922ebb599ed473a18dca8e010652bb6e6a`. The independent verifier and exact JSON result are available in the run52 archive outside the repo. This is arithmetic/certificate verification **not unknown-index F4 extraction**.

## New native Rust test (pending build and CI)

`examples/prime31_coset_xlift_f4.rs` uses **the existing** `crypto_lib::cryptanalysis::f4_fp::{solve, F4Options, Verdict}` on eight public univariate polynomials, p=2³¹−1, an explicit 10-second deadline per instance, a degree-24 bound, and no group-sum/BSGS/witness-index inputs. The only archived point data are eight anchor points used as post-solve checks. The candidate recovers 21 roots per coset, validates each unique y lift, cross-checks the exact polynomial coefficients, and reports F4 field multiplications, wall time, 168-root coverage. The PR workflow now runs it after the existing field-width smoke gate.

**Not yet qualified:** local container has no rustc/cargo; the available GitHub connector returned no workflow runs or status at report time. Hence there is no measured F4 root-finding time, no Rust test PASS, and no 7-/8-leaf blind extraction. Do not promote these fixtures to solved ECDLP measurements.

## Next algebraic proposal, not implemented or timed

Use the explicit signed polynomial map to eliminate seven leaf y variables, their power auxiliaries, and all leaf sign bits from the **run50 seven-coset oriented** group-addition system. One possible nonexceptional affine chart has **29 variables**, **31 equations**, maximum degree **21**:

- Seven x leaves: `f(x_i)^7=c_i²` (7 equations).
- Five intermediate (x,y) pairs, six slopes: 6×3 oriented addition equations (18 equations), substituting y_i=f(x_i)^4/c_i at leaves.
- Six denominator-product/inverse guard variables (6 quadratic constraints), ruling out division by zero.

An independently implemented Python evaluator then checked **all 31 specialized equations on 4,090/4,090 archived witnesses** and rejected **4,090/4,090 poisoned-target coordinate mutants**, with zero exceptional denominators in that cohort. This validates the proposed chart on known witnesses but is not coefficient-level Rust parity or a blind polynomial solve. The independent checker is available in the run52 replay archive.\n\nThis reduces run50's 64-variable quadratic circuit, but raises degree from 2 to 21. **There is no reason to presume this will outperform the existing 14-variable degree-8 transformed-slot Horner chart** in PR #4812 or the 4-variable triangular reduction in PR #4822. An admission test must measure actual F4 rows/columns/degree and failed-query cost, not variable count alone.

## Prior work, overlap and rank ordering

- #4692 owns the main public-only Prime31 selector/RR9 and F4 trials; do not duplicate its frozen B8 corpus. #4812 owns a different weighted-slot F3/Horner sign-lift system; #4822, #4816 own the 4-variable sparse Wiedemann/triangular RR9 experiments. #4809 owns F3 pair-root extraction. #4819 developed batch inversion, subproduct-tree and full prime-field IC rootfinding speedups.
- #4793 documented matched same-runner W8 and historical faster 2²⁰ shared-BSGS: **170.113 ms for 1,024 Prime31 targets**, including setup, on the historical runner. An algorithm that merely beats the older, slower 585-ms/4,096 C++ BSGS control is not a confirmed new frontier result. Current-host matched Rust BSGS and rho must be rerun.
- Published analysis of naive Semaev prime-field PDP Gröbner costs (J. Math. Cryptol. DOI 10.1515/jmc-2019-0029) flags a structural obstacle; the present polynomial identities do not circumvent that lower-bound analysis.

## Priority next tests and acceptance gates

1. Observe CI PASS/failure for the **actual** 31-bit Rust univariate `solve` on all eight cosets. Record each call's wall, Fp row multiplications, failure/timeout; no fallback enumerating all p values.
2. On 16 fresh public hash-to-curve Q, run #4692/#4812/#4822's **existing 14-variable and 4-variable exact source-specific** blind extractor candidate with no leaked witness indices or scalar. Explicitly include negative attempts, finite charts, budgets and peak RSS.
3. If an extractor produces unplanted relations, feed it to rank-complete factor-base log recovery. Measure relation collection, linear algebra, individual logs and fallback. Compare cold and amortized 1/32/128/1024/4096 against same-runner fastest shared BSGS, negation rho, and prime-field IC frontier; independently verify [d]G=Q.
4. Do **not** claim a cheaper complete solver until the full matched cost (including negative probes and unavailable charts) wins with source-bound replay.

**Result:** exact sign-eliminated factor-base equation validated at p31; complete prime-field ECDLP speedup remains unestablished.
