# Direct sign handling: three-arm fixed comparison

2026-10-25. Automatic continuation of the existing user objective, same PR2381
and author. Preserve the single scheduled automation; this is not a new writer.
CI was checked at18:04UTC on cc23bec37424c50d048e6ea4bdc3bfacd3f0f2e4; do not
poll that same head ahead of its cadence. Check PR stop/head state before push.

## Hypothesis, reference and population

Distinguish useful signed-point reuse from avoiding the frozen binary ±1 loop's
identity addition and unused final doubling. Keep THREE arms: binary (frozen
signed-cache reference), cached (frozen signed-cache candidate), direct (P or -P
computed per operand without a scalar loop or signed cache). Same40 setups/400
separate nonidentity targets per arm: all10 F43 size2 policies,4 direct/transported
roles and10 source targets. Total120 setups/1200 verified recoveries. Reference
is cc23bec37424c50d048e6ea4bdc3bfacd3f0f2e4. Exact source cases:ten reduced,
ten adaptive,ten signed-cache records. Preserve every ordered lift (including
after a first successful lift), root, coefficient row/rank, complete certificate,
degree/expanded witness/promotion, base log, failed shift and source scalar.

Keep registered main icv1-fp6-t0-74779950 P=(31,18), twist
icv1-fp6-t0-c6a8a47c U=(24,26), F43/r11 and actual phi=(41x,11y). Exceptional
base-field-isomorphic fixture only; U has no EC1 registration. Recovery receives
Q alone, with planted labels only generating/checking public synthetic points
outside it. No relation, target or scalar table replaces the solver.

## Exact algorithm and new counters

Binary and cached arms retain ALL36 old request definitions and exact values.
Direct uses base[i] when sign+1 and exactly one counted curve negation when−1,
then performs the same counted actual sum/comparison. No scalar call, final
double, or cache creation/validation. Cached setup/reads remain exactly as frozen.
Root membership setup, row solver, matrices, witnesses and linear/recovery phases
stay unchanged. Preserve factor-base column order and separately sorted query roots.

Add TWO common request counters, sign_operand_reads and sign_dispatches, each
incremented once per tested operand (two per complete sign test) in ALL arms.
They describe that common interface even in the binary arm. cache_reads is an
older nested counter, not an extra equal-cost event to be added to operand reads.
No statement that these new counters were previously zero: drop them when
comparing old records, retaining every old component. Add sign_trace_sha256 per
query: hash the canonical ordered list of EVERY tested tuple (i,j,si,sj,A,B,sum,
accepted), with the sum reused from the actual test. This audit-only record
binds rejected tests too. Hashing/trace construction remain explicitly unpriced;
the mathematical query/recovery receives no answer from the hash.

Finite ledger predictions to check independently: direct additions=binary−4L;
direct iterations=binary−2L; direct negations=binary. Cache additions=direct+2
in setup/cold and equal online/verification; cache negations=direct−L+2 in
setup/cold, direct−L online, with L that phase's tested signed_lifts. Direct
and cached scalar iterations match. All other old components match their stated
frozen arms. New common reads/dispatches=2L in every arm. Emit all setup/online/
verification/cold components; never replace separate targets by a batch average.

## Verification, interpretation and controls

Independent Rust field/group/polynomial arithmetic reconstructs all3 arms,
complete signed operand traces, requests and scalar answers. Compare binary and
cached records to frozen signed-cache cases after dropping only new counters/
trace field. Require identical complete mathematical behavior/trace across all
arms and run the additional old batch-RREF/witness audit of every distinct solver
certificate. Hash checks never replace semantic checks.

The existing SignedCache.signed_eq_zsmul and ordered_lifts_eq, validated cache
lemmas and PhaseRecovery annihilator-aware recovery already state the required
structural implications. Direct is exactly the existing signedPoint branch;
reuse their unchanged proofs/source bindings faithfully. Run the unmodified
scoped proof gate; no new polynomial identity or universal native-counter proof
is claimed, and all16 existing identity records remain unchanged.

Rehashed controls: false direct operand-trace hash, false direct demand, wrong
on-curve cached point, omitted direct arm, missing direct failed query, false
direct scalar, false source. Each rejects101/no receipt. Retain exact corrupted
payloads/logs. Success is all120 setups/1200 scalar checks, unchanged ordered
solutions/certificates and exact checked request deltas, with losses retained.
A mismatch rejects the implementation; repair without dropping population or
record a null and choose another feasible lead.

This is an algorithm-request/reference-quality experiment. A component loss
refutes only that component comparison, not the wider twist/IC objective.
Requests are differently weighted and may be nested; compiler may remove unused
binary arithmetic. Cache checks/storage/reads, fixed tables, hashing/trace work,
allocation/sorting/traversal, internal exponentiation and audit remain unpriced.
No ranking of calibrated total work, timing/speedup, GAE/S/rho, intrinsic
regularity or fastest-external claim. Full calibration/ecbench/matched references
and benchmark gates remain. No registry/catalog/benchmark/leaderboard changes.

## Execution and delivery

Commit this protocol and compiled new native sources before first execution.
New versioned sources/crate/report/frozen output only; no prior evidence/source
or workflow rewritten. Default nativecap600s; focused iteration envelope3600s;
existing local resources/two build workers, no paid provisioning/upload/credential
reuse. Fresh output directories; preserve exact failures and required report,
graph/diagram/inspected PDF. Fresh root release/locked library suite,22 backend
and4 workflow metadata tests, naming/card/artifact/size/source/archive checks.
Actual Conductor check still exits127, no reservation or replacement coordinator.
Missing gawk still blocks profiling, not this independent request comparison.
Keep PR open for explicit CI/review/integration/scientific holds. The next
mechanism search returns to non-isomorphic scalar-preserving transfer and actual
useful-yield/solver-degree comparisons under the lead atlas and identity rules.
