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

## Guarded K64 search diagnostic, 2026-10-26

At source commit `4445c40b49e1b9a66a61722a1a5b149af3052fbe`, a
source-matched static Linux worker loaded the frozen primary `a=0`
`public_x_hash`, seed `2026100801`, 64-orbit base and public target fixture.
The selected base has 10,624 points and 65 matrix columns. The zero-trial
Karatsuba preflight passed the source, input, replay and 4 GiB/zero-swap
guards. The selected compressed object was downloaded from its manifest's S3
URI into an external replay directory before launch.

Two one-query arms used the same worker binary and source, 150,000-variable
and 3,000,000-domain-clause limits, a 10,000-conflict cap, one CPU, 4 GiB
memory, zero swap and a 300-second outer wall cap. Karatsuba ended at
`UNKNOWN_wall_cap` after 301.1 seconds and emitted no final worker summary.
Its progress log reached `RelationCollectionStarted`, then contains no
completed SAT attempt. Thus the wall cap gives no conflict count for that arm.
Schoolbook emitted `UNKNOWN_solver_cap` after 10,000 conflicts, with zero
models, relations and rank increase, in 163.7 seconds of outer process time.
Its outer status is `PRODUCER_FAILURE_worker_exit`: Docker returned exit 125
while waiting for the finished worker because its containerd metadata store
had become read-only. The container was subsequently reported `dead`, but
`docker rm -f` failed with the same metadata error. The schoolbook worker
summary is retained as a diagnostic; it does not pass the outer receipt gate.

These source-matched local runs establish preflight compatibility and an
inconclusive bounded search comparison. The next operational step is to
repair Docker's metadata store without disrupting other containers, then
rerun a short matched pair with a lower conflict cap. In-attempt SAT
checkpoints would let a wall-capped arm retain its conflict count. A Tier 1
paired one-target comparison is still required for
factor-base ranking. The exact configs, outer receipts, build identity and
worker summary are in
[`verification/karatsuba-k64-guarded-20261026/`](verification/karatsuba-k64-guarded-20261026/).

## One-level split candidate

The opt-in `ShallowKaratsuba` kernel splits an 83-bit polynomial after bit
41, with a 42-bit low part and 41-bit high part. It computes the low, high
and mixed products once, then routes their AND gates directly through the
pinned field reduction to output XOR rows. For one field product this uses
`42² + 41² + 42² = 5,209` AND gates, compared with `83² = 6,889` for
schoolbook. The pre-domain five-summand N83 model has **62,508 AND gates,
187,607 CNF clauses and 65,567 SAT variables**. Schoolbook has 82,668,
248,087 and 84,743 respectively; recursive Karatsuba has 22,152, 66,539
and 95,039. The shallow kernel introduces fewer intermediate XOR wires than
recursive Karatsuba, though SAT conflict work remains to be measured.

The kernel passes independent field-multiplication controls at degrees 7,
9 and the pinned degree-83 basis, including operands above bit 64. Its
distinct full IC strategy passes the checked-scalar, relation-rank and
recovery control on a degree-7 curve. The guarded worker and supervisor
bind it as `chain-s3-shallow-karatsuba` and `product_kernel:
shallow_karatsuba`; existing kernel names and receipts remain distinct.
The next empirical gate is a source-matched retained K64 comparison under
the same trial, conflict, memory and wall caps after Docker's metadata store
is repaired.

The same shallow product circuit is also available with the exact
signed-Frobenius representative-and-phase domain, both unordered and with
the first pair ordered. These guarded strategies are recorded separately as
`orbit-chain-s3-shallow-karatsuba` and
`ordered-orbit-chain-s3-shallow-karatsuba`. The domain replay, full
original-curve witness check, and matrix-rank logic are shared with the
schoolbook orbit solver. Degree-7 controls exercise checked five-summand
target recovery through both new strategies. The pre-existing orbit-domain
control covers five- and six-summand schoolbook decomposition; shallow
six-summand solving remains a separate measurement.
This admits a source-matched K1,182/K2,048 comparison after the small-base
capacity gate, without forcing those larger bases into a coordinate trie.

## Matched high-K circuit result, 2026-10-26

A source-frozen, eight-arm local VM panel paired schoolbook and shallow
Karatsuba on the exact retained K=1,182 and K=2,048 primary-curve bases,
first through x86-64 emulation and then with a native ARM64 worker. In the
native panel, schoolbook completed 5,000 conflicts in 173.087 seconds at
K=1,182 and 154.245 seconds at K=2,048. Shallow completed that cap in
208.757 seconds at K=1,182 and reached the 280-second outer wall cap at
K=2,048. The emulated panel had the same terminal statuses. Every
completed arm returned zero models, relations and rank increase. Thus the
smaller shallow circuit did not improve this capped SAT stage, and base
selection remains open pending natural relation yield and complete
single-target accounting. The exact inputs, all statuses, native build
identity and audit are in
[`verification/localvm-shallow-orbit-20261026/`](verification/localvm-shallow-orbit-20261026/).
