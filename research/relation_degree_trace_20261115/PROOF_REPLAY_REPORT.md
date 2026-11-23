# Exact replay component: qualified, not timed

The new Rust replay module emits acyclic membership identities, containment
witnesses for every original input, and a strict standard representation for
every S-pair. Its consumer enforces canonical shapes, complete coverage,
monicity/reducedness, acyclicity and budgets. A separately executed Singular
program checks all arithmetic identities, binds leading monomials to dp order
and checks the S-pair multiplier bounds, without a new basis solve. The
implementation is producer code, not a signed independent verifier [pcc:unverified].

Buchberger's criterion needs the strict monomial bound: the trivial defining
S-pair identity is true for arbitrary polynomial sets and proves no basis
property. The qualifier includes that trap and a non-GB forged-zero witness.
All original inputs must separately have membership in the final basis; checking
only basis membership or only S-pairs is insufficient. [pcc:unverified]

Source35bdf5b70 immutable checks passed39 default/41 feature-counter release
tests and strict clippy. Attempt02 consumer/CAS qualification failed because
the copied Singular could not load standard.lib. Its strict stderr rule
withheld acceptance, even where an expected marker printed. Preserve the
failed outputs; do not relabel them as clean arithmetic execution (PCC-20261116-2c4435) [pcc:unverified].

After an explicit SINGULARPATH repair, qualifier40c111744 completed both valid
compact/expanded cases and all ten tamper cases. All executable, runtime and
standard.lib hashes match before/after. Positives have clean stderr; the
changed-coefficient and forged-zero cases are rejected by mature arithmetic.
Other malformed/coverage/nonstandard cases are rejected before launching it.
No unchanged passing core suite was rerun. This is unsigned correctness
qualification, not execution verification or performance data [pcc:unverified].

Read replay-development/{build,pre-call,post-call}-02/03.json, exact test logs,
qualification summaries and raw archives. CLI: certificate-replay produce
INPUT compact|expanded NEW_WITNESS; certificate-replay script INPUT WITNESS
NEW_SCRIPT. Consumer acceptance requires exit0, empty stderr and stdout exactly
CERTIFICATE_REPLAY_OK. Characteristic above2^31-1 is explicitly unsupported by
this CAS exporter, without changing wide-field Rust constructor support.

All prior full-cost clocks/records/protocols remain frozen. No15x improvement
is claimed for replay. A future matched experiment must pay witness creation,
serialization, process launches and consumer checks, compare identical complete
answers against strongest supported controls and retain failures. It must
state the changed validation algorithm, rather than silently drop reference
costs from an old ratio. [pcc:unverified]

Sources checked (KB): unavailable MCP/wrapper; fallback shared provenance code,
Singular4.4.1 manual lift/liftstd/interred/reduce/division and polynomial comparison
semantics, plus prior Isabelle2025-2 groebner.ML review. reduce presupposes a
standard basis and is not used circularly in this checker. No novelty claim.
