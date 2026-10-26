# Karatsuba circuit for the primary N83 chained-S3 solver

The opt-in `ChainProductKernel::Karatsuba` circuit reduces the pre-domain
five-summand N83 S3 model from **82,668 to 22,152 two-input AND gates** and
from **248,087 to 66,539 CNF clauses**. It uses 95,039 SAT variables instead
of 84,743 and adds native XOR rows. These are exact source-construction
counts for the all-affine identity pattern, not solver-time measurements.
The schoolbook circuit remains the default and retains its prior source and
receipt identity.

The field is the primary `a=0` curve's polynomial basis
`z^83+z^45+z^2+z+1`. The circuit recursively computes three half-size
polynomial products and combines their coefficient wires over F2. Padding
to a power of two is internal; output reduction uses only coefficients
through degree `2*83-2`. The SAT solver receives native XOR constraints for
each new linear wire and the same three-clause encoding for each AND gate.
The selected kernel is explicit in `ChainedS3Encoding::build_with_kernel`.
The caller's variable cap is checked before the SAT solver is constructed.
The generic IC driver exposes a separate `ChainedS3Karatsuba` strategy, and
the guarded primary worker names it `chain-s3-karatsuba`. The outer search
supervisor selects the corresponding worker command with `--karatsuba` and
keeps the source, base, target, caps and product kernel in its frozen config.

A shared-host degree-7 checked-scalar control completed in 93.55 seconds for
the Karatsuba strategy and 2.41 seconds for schoolbook in consecutive test
invocations. Other Rust jobs were active, and these correctness-test times
have no Tier 1 isolation receipts; they are a diagnostic reason to measure
SAT conflict work before promoting the smaller circuit. A screened variant
that moved intermediate XOR gates to CNF instead built 545,531 clauses and
143,975 variables for the same N83 pre-domain model, so it was dropped before
retained-base search.

Focused tests compare both kernels against independent field multiplication
for degree 7 and 9 inputs, including a deliberately wrong product bit, and
check the Karatsuba circuit in the pinned degree-83 basis with inputs above
bit 64. Both kernels agree with the original S3 polynomial on positive and
negative degree-7 assignments. An N83 five-summand construction test checks
the counts above and rejects a cap one variable below the required count.
Run them with:

```sh
CARGO_TARGET_DIR=/absolute/internal/target cargo test --lib binary_semaev_chain_sat::tests::
```

The next gate is a matched, source-frozen comparison on one replayed primary
factor base and the same public point: build time and peak memory, SAT
conflicts and models under the same cap, verified natural relations and
incremental rank. The new XOR rows can offset the smaller AND/CNF circuit,
so solver cost and end-to-end factor-base selection remain open until that
gate and a fully charged single-target run complete.
