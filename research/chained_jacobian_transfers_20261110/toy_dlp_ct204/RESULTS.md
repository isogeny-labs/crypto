# CT204 — public toy ECDLP via an actual odd-prime Jacobian transfer

Date: 2026-11-18. Branch `research/ct204-odd-cover-toy-dlp-20261118`, draft [PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on [#4697](https://github.com/isogeny-labs/crypto/pull/4697). **[pcc:unverified]**

## Narrow advance, not a solver-speedup result

A Rust integration test now contains a **complete discrete-logarithm recovery algorithm**, not merely a transfer-composition certificate. It uses the pre-existing `PrimeQuadraticCoverP` and `BalancedDivisorP` to pull a public elliptic target into the Jacobian of the genuine genus-two cover, solves the new DLP using a generic BSGS, checks its Jacobian answer, norms the target back, and independently verifies `[k]G=Q`. The same BSGS source is applied to the direct elliptic group for an operation-count comparator. A second test rejects a deliberately wrong scalar on both groups.

**Execution standing:** code is committed but has NOT been compiled or executed in this automation workspace. The current draft PR has no exact-head test verdict. Do not quote the source test's anticipated `43/43` printout as experimental success. No original 695/701 corpus, EKS, S3 or budget permission is implied.

## Public fixed instance, independent arithmetic check

- Source `E / F_31: y^2 = x^3 + 3`, with `G=(1,2)`.
- Separate integer-field enumeration (outside the Rust implementation) found **42 finite affine points plus infinity**, hence `#E(F_31)=43`. Repeated group addition gave the exact order of `G` as **43**, prime.
- Cover `C: v^2=u^6+3` with `pi(u,v)=(u^2,v)`. Modulus is odd, `F(0)=3 != 0`, the target discriminant `4a^3+27b^2 = 26 mod 31 != 0`, and `C` has squarefree sextic in this characteristic.
- The already implemented identity `pi_* pi^*=[2]` has `2^{-1} = 22 mod 43`. Thus pullback is injective on the tested 43-order source subgroup **conditional on the checked map identities**. It does not establish an easier DLP.

These toy parameters exercise arithmetic and recovery only; the tiny subgroup is explicitly not a security-representative or intentionally advantaged production benchmark.

## Committed test and exact expected counters

File: [`tests/prime_cover_toy_complete_dlp.rs`](../../../../tests/prime_cover_toy_complete_dlp.rs).

For all 43 public targets `Q=[k]G`, including infinity, the test obtains `k` from **only** `G,Q,43` inside each solver. It asserts target reconstruction, source-Jacobian reconstruction, the norm result `pi_*pi^*(Q)=[2]Q`, and cross-arm agreement. For every target the generator and target are re-pulled, so there is no shared precomputed Jacobian generator across test targets.

Both groups use the identical intentionally simple BSGS, with `m=ceil(sqrt(43))=7`. Each target costs `(m-1)+m+floor(k/m)=13+floor(k/7)` group additions inside this BSGS; the analytically expected 43-target total is **670 elliptic additions and 670 Jacobian additions**, with 86 pullbacks, 43 pushforwards, and separate verification/scalar-multiplication work. Equal **counts** do not mean equal runtime or `S` cost. The linear baby-vector membership check is unsuitable as an optimized scalable BSGS comparator.

## Reproduce when approved and toolchain available

```sh
git checkout research/ct204-odd-cover-toy-dlp-20261118
cargo test --locked --release --test prime_cover_toy_complete_dlp -- --nocapture
```

This is a bounded correctness check, **not** the preregistered same-host cold performance experiment. The command has not run here because the current runtime has no Rust compiler or network access to the repository. Its invocation must remain subject to normal lab CI/budget gates.

## Remaining decisive gates

1. Obtain an actual clean exact-head Rust compilation/test receipt, and independently replay accepted and wrong-answer controls; if the balanced arithmetic fails on F31, investigate and fix before any performance claim.
2. Replace toy BSGS with the lab's strongest complete destination IC implementation, and compare each native cold single-target source/transfer/solver/recovery/verification total to matched optimized rho, BSGS and existing IC with *all* costs and memory.
3. Restore authenticated CT125/CT192 695/701-record model custody (current S3 `HeadObject` HTTP 403), then price `C169`, both `C13` hops, `N2_actual`, and `D` on the same host. CT198's norm-point work cancels in direct-vs-retained comparison and must not be double-counted.
4. Preserve blind target provenance, baseline/candidate identical verified outputs, bounded uncertainty and an independently signed PCC verdict. A complete-solve win is not asserted without these.

## Prior work and concurrency

Reviewed merged [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877), [#4215](https://github.com/isogeny-labs/crypto/pull/4215), producer implementation [#4697](https://github.com/isogeny-labs/crypto/pull/4697), mirror [cryptanalysis #1770](https://github.com/isogeny-labs/cryptanalysis/pull/1770), [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), cold evidence gate [#1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766), and earlier [GPU IC #4178](https://github.com/isogeny-labs/crypto/pull/4178). This patch does **not** edit the concurrent transfer producers, original fixtures, or model hydration. Shared research-KB could not be queried through this runtime; no originality/absence claim is made.

**Standing: [pcc:unverified] — new source harness, independent tiny-field arithmetic check, no Rust execution, no measured end-to-end advantage, no protected merge, no paid compute.**
