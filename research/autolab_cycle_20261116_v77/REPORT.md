# C77: exact Mersenne modular power kernel and five-field cold screen

Hypothesis: P521 dense modular exponents can retain k-1 squarings while reducing multiplication count from a dense binary chain to O(log k). The exact recurrence uses no inverse and works with zero/nonunit bases and repeated quotient polynomials. Other exponents use the frozen generic function. The copied solver otherwise preserves every conditional shift, common intermediate, residual check and unresolved branch.

Independent oracle evidence: 2,232 repeated-root Taylor/Hasse remainder cases, including full520/521-bit exponents;240 arbitrary small root polynomials checked by literal field enumeration;32 original S3 systems and3,168 literal shift/auxiliary checks. Every coefficient and root set matched. For exponent2^521-1 the kernel used520 squarings and11 multiplications; for2^520-1 it used519 and10. These are arithmetic counts, not47x runtime gains.

The frozen cold panel retained all five exposed NIST inputs, three fresh-process repetitions and alternating method order. All30 runs completed with identical roots, shift candidates and degree12 polynomials per field. Startup, parsing, full solve, original verification and final stdout are included in supervised child lifetime. Parent comparison bookkeeping is outside per-child times and inside experiment lifetime.

| Field | Generic median s | Chain median s | Generic/chain | Generic span | Chain span |
| --- | ---: | ---: | ---: | ---: | ---: |
| P192 | 0.093001 | 0.091545 | 1.0159 | 2.30% | 1.58% |
| P224 | 0.101451 | 0.103161 | 0.9834 | 35.89% | 4.12% |
| P256 | 0.111260 | 0.101717 | 1.0938 | 8.67% | 4.61% |
| P384 | 0.195629 | 0.218455 | 0.8955 | 7.80% | 32.31% |
| P521 | 0.413532 | 0.219520 | 1.8838 | 30.11% | 15.77% |

The P521 median gain1.8838x supports the preregistered>=1.3x development prediction. Its repeat spans30.11% and15.77% fail repeatability requirements. P384 chain/generic median slowdown is about1.1167x and chain span32.31%; this loss is retained, not dismissed as a successful fallback result. No noisy-cohort or field selection is used to claim qualification. The candidate remains experimental, not integrated into the generic fast path.

This compares a new backend with its own frozen exact reference. It is not a genuine F4/F5/F6 comparison, a twenty-hard-instance panel or a full independent-source solver result. Fastest reference medians are below the hardness floor. No2x/50x release claim follows.

Memory was recorded for every child via wait4, post-exec sampling and child lifetime peak; all raw stdout/stderr and operation counters are preserved. Setup and finite-field point sources are frozen exposed training, not unopened hard holdouts. Classical addition chains do not establish algorithmic novelty.

Next: field-entry routing should bypass the candidate altogether for non-Mersenne primes; prove that route identity and measure its A/A overhead. For P521, use a separately frozen multi-instance throughput cohort rather than silently retrying this noisy execution. Sparse signed chains for other NIST primes need explicit unit checks and zero-root fallback. Binary semilinear trace jumps remain another active, independently testable design.

Two frozen90-second,4-GiB,one-core experiments passed through resume/run-ready. Scientific goal active; the complete original independent-source search and full performance gate remain open.
