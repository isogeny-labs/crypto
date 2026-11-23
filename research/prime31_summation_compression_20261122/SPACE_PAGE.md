# Prime-field summation-polynomial research

Checkpoint: November 22, 2026 (America/Los_Angeles). Objective remains open: establish a novel, correctly accounted relation-solving gain on growing prime fields, using p=2^31-1 as the initial test case.

## Current research state

[PR #4928](https://github.com/isogeny-labs/crypto/pull/4928) preserves the Dickson/S3 component, raw evidence, mathematical arguments and continuation state. [PR #4924](https://github.com/isogeny-labs/crypto/pull/4924) preserves the existing symmetric/Newton/monic-pair work. Both are comparison lanes; neither establishes the requested complete asymptotic result. [pcc:unverified]

The exact ordinary source curve is y²=x³+847514846x+2087444724 over F_2147483647, with recorded prime group order 2147446831 and generator (625037715,2100627251). These are inherited source parameters, not a new point-count certificate.

## Implemented adaptation

For target R=(r,s), s nonzero, write S3(X,Y,r)=dY²-tY+u and retain a full composed fibre F_k(Y)=D_(2^k)(Y)-c. Initialize (Q,T,U)=(d,t,u), reduce modulo the chosen outer membership polynomial, and iterate:

```math
\begin{aligned}
Q'&=Q^2,\\
T'&=T^2-2UQ-4Q^2,\\
U'&=U^2-2T^2+4UQ+4Q^2.
\end{aligned}
```

The final resultant is U_k-cT_k+c²Q_k. The recurrence has no inverse of X-r and retains the diagonal degree drop. Rational lifting and an oriented line formula recover signed original-curve relations. See the complete assumptions and derivation in the PR report. [pcc:unverified]

## Outcome of the first comparison

At N=64 and actual lifted base B=31, deterministic development counters give:

| Kernel | Counted field multiplications |
| --- | ---: |
| Dickson recurrence | 17,923 |
| Sparse full-fibre product | 8,816 |

The recurrence loses this cell. Counts exclude setup, root extraction, validation and complete ECDLP. Development receipts and exact dependency qualifications are retained. No timing or full-solver claim follows from these counts. [pcc:unverified]

The later [original-lock hosted CI run](https://github.com/isogeny-labs/crypto/actions/runs/45089569480) passed 41 native library tests and all 5 component tests at repository source commit 109fc024e33f122fd678d2c93a18b2c0438a8668. Its deterministic diagnostic matches the local output byte-for-byte. Claim PCC-20261123-526846 remains unverified: this is producer CI evidence, not a signed independent execution proof. [pcc:unverified]

## Two subsequent mathematical steps

For a nonzero odd-order target R, the full-curve relation between D_N(x(P)) and D_N(x(R-P)) has exact bidegree (2N,2N). This rules out bounded-degree scalar compression of that full relation; it does not exclude compact circuits, auxiliary states or finite-fibre algorithms. [pcc:unverified]

With an exact subtree-emptiness oracle, the Dickson tree needs O(1+z[1+log(N/z)]) node tests for z projected answers, and one test when z=0. Standard norm recursion still visits all N leaves on an empty query. Explicit dimension-charged empty blocks cost Omega(N-z). The unresolved operation is therefore exact sublinear empty-root certification with target-dependent setup and every unsuccessful branch included. [pcc:unverified]

## Continuation and evidence

The existing hourly task continues from [STATE.json](https://github.com/isogeny-labs/crypto/blob/lane/prime31-dickson-quotient-gate/research/prime31_summation_compression_20261122/STATE.json) and [CONTINUATION.md](https://github.com/isogeny-labs/crypto/blob/lane/prime31-dickson-quotient-gate/research/prime31_summation_compression_20261122/CONTINUATION.md). The full [illustrated PDF](https://github.com/isogeny-labs/crypto/blob/lane/prime31-dickson-quotient-gate/research/prime31_summation_compression_20261122/REPORT.pdf), [mathematical report](https://github.com/isogeny-labs/crypto/blob/lane/prime31-dickson-quotient-gate/research/prime31_summation_compression_20261122/REPORT.md), raw receipts and prospective matched-cost protocol are in the PR.

All current PCC claims remain unverified. Four native identity certificates have proof:null; no Lean-kernel certificate or signed independent execution verdict is asserted. CI status and source provenance must be read from the current state before quoting test outcomes.

## Publication instruction

This file is the prepared Page content. It is not evidence of a completed Space write. Reconcile the single uncertain create request for the exact Space name Prime Field Summation Polynomial Research, read that Space's root, then create or update one research Page using a retained idempotency key. Record returned IDs and readback before labeling synchronization complete.
