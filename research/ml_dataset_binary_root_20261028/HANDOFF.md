# Standalone binary root module handoff

## Completed integrated validation, 2026-10-28 15:31 UTC

The integrated runtime `d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83` now passes
the full library suite (5183 passed, 0 failed, 123 ignored), native root9,
dataset158, CLI8 and strict integrated Clippy. The independent consumer's
51 retained software fixture cells include30 checked completed root sets.
The [final validation record](VALIDATION.md) gives the precise source, scope,
limitations, binary identities, retained failures and freshly verified S3
archives. All pending-test statements below are historical. Hosted delivery,
actual cloud publication and broad corpus coverage remain separate obligations.

No new campaign worker or experiment retry was launched. The original total
USD200 ceiling and existing approvals, owners and deadlines are unchanged.

## Integration update, 2026-10-28

The original standalone snapshot below is retained as history. The parent
subsequently authorized module registration, exclusive centralized Cargo and
dataset integration under renewed T-743 scopes. A reported Git index lock was
already absent when reconciled; no lock was removed. Registration was committed
as `aea52933390a63e38cdebe7c81d1baff1b059a45`.

All nine native tests passed and strict release-library Clippy with `-D warnings`
passed using Rust 1.98.0. Logs are retained at
`/home/ec2-user/ml-dataset-resume-20261028/binary-root-validation/native-first.log`
and `clippy-first.log`. The integration design was committed first as
`8f6d423ef21`; the adapter now has typed exact-instance/domain/configuration
bindings, aggregate work/event/byte replay, live intermediate/call artifacts,
outcome checks and restricted training-export references. Native dataset tests
are being compiled; these changes are not yet a passing integration gate.

Rust 1.98.0 lacks the rustfmt component on this host. The attempted scoped
format command failed before changing files; installed default rustfmt then
completed successfully. This tool setup failure is retained separately from
native correctness results. No scientific cloud job or experiment ran.

Remaining: finish dataset tests, address any actual failures, strict integrated
lint, exact-source evidence persistence and parent full-library/consumer gate.
The companion consumer agent is independently implementing root-set replay.

Recorded 2026-10-28 at 14:08 UTC. Implementation is reviewable; native
compilation, tests, strict lint, dataset integration and publication are pending.

## What was done (commands, files, features)

- Created `codex/ml-binary-root-trace-20261028` in
  `/home/ec2-user/crypto-ml-binary-root-trace-20261028`, based exactly on
  `d71ec2ef5ab9b7f2a038f61c941af10297a775ad`.
- Committed the design before implementation as
  `a90fca0a63c9d1365ce69bb2029446ef8a291fb9`. Conductor T-743 reserved only
  `src/cryptanalysis/binary_root_trace.rs` and this research directory.
- Added the standalone module with strict typed field/modulus/coefficient
  binding, native bounded quadratic and Artin--Schreier operations, odd-degree
  half-trace, even-degree linear-system states and separate portable root-set
  verification. No legacy API or shared module-registration file changed.
- Added nine native tests covering exhaustive quadratics over GF(8)/GF(16),
  constructed roots and trace-one controls at all 24 requested field degrees,
  degenerate branches, legacy root agreement, inverse primitive counts,
  every work/event/sink boundary of a representative even-degree solve,
  input/record/certificate/event tampering, and the incomplete execution/cost
  claim boundary.
- Scoped Rustfmt and whitespace checks passed; Rustfmt parsed the complete
  module. Commands were
  `rustfmt +stable --edition 2021 --config skip_children=true src/cryptanalysis/binary_root_trace.rs`
  and `git diff --check`. The precommit source SHA-256 is
  `d77bfc85556b51d938ffa8cc82a402f7e0b3cbaf4c05b81707f5154e5a01ad0e`.
- No Cargo command, root build, experiment, cloud job, public PR, push or
  historical-artifact rewrite was performed by this task.

## Decisions made (and why)

The public entrypoints are:

```rust
FieldSpec { degree: u32, modulus: u64 }
RootLimits { work: u64, events: u64 }
solve(&[u64], FieldSpec, RootLimits,
      &mut dyn FnMut(&RootEvent) -> Result<(), String>) -> Result<RootReport, String>
verify(&RootReport) -> Result<(), String>
```

`modulus` is the full monic bitmask, including bit `degree`. The coefficient
slice is constant-first; reports trim trailing zero coefficients. Reject
noncanonical input words rather than silently masking their high bits. Degree
is restricted to 3..61 and polynomial degree to two. Hard limits are 1,000,000
selected work units and 100,000 attempted events.

`RootCounters::work()` adds field add/mul/square calls, matrix pivot bit tests
and individual word XORs. Inverse calls remain informational. This mixed unit
is a resource admission bound, not an equivalent field-operation cost model,
CPU-instruction count or certified total. The core excludes coefficient setup,
group lifts, validation, serialization, replay and loop/control overhead.

`RootStop` distinguishes Complete, AllFieldElements, WorkLimit, EventLimit,
SinkFailure and a fail-closed LinearSystemFailure. Reports expose acknowledged
events plus at most one unacknowledged attempted event, whose executed work is
still charged. The sink is not invoked again after failure. A failed terminal
event leaves the report incomplete and its returned root vector empty.

The independent checker uses portable carryless multiplication/reduction,
simple exponentiation, and polynomial Euclidean arithmetic. It verifies the
monic gcd(f, X^(2^n)+X) equals the product of distinct reported linear factors;
it does not use native field multiplication, native inverse, the producer root
finder, or root enumeration. Zero-polynomial semantics are explicit.

Supplied event identities and recorded tallies are checked, but omitted
primitive events may leave a valid root-set certificate. Both
`execution_trace_complete_verified` and `total_operation_count_verified`
therefore remain false and are enforced by verification. The omitted-events
regression makes this limitation explicit.

## Open threads (unfinished, pick up next time)

1. Parent adds `pub mod binary_root_trace;`, then runs the nine native tests and
   strict lint. Native compilation and execution have not yet been attempted.
2. Integrate the existing binary/Koblitz `root`, m=2 adapter, preserving both
   twists and exact public domains. Charge S3 coefficient setup separately
   (two adds, two squares, one multiplication), original-root evaluations and
   group-lift additions; enforce aggregate work/events/bytes across calls.
3. Bind certificates to public field/curve/twist/target/configuration/domain,
   canonical query ordering and stop reasons. Check complete point lifts
   separately from root sets, and require unique artifact/outcome bindings.
4. Use the existing live sink for native events and complete state/call payloads.
   Prefix persistence is not a resumable solver-memory checkpoint. Keep these
   artifacts out of static model inputs; update independent consumer support.
5. Run the required integrated release library suite before publication and
   persist source, retained errors, logs and receipts to canonical S3/Git.
   Parent coordinates the shared session-log entry and final delivery.

This source does not deliver a completed cloud dataset, higher-m elimination,
multiplicity certificate, recovered scalar, performance comparison, or
first/last-fall-degree theorem. Existing campaign budget, approvals, external
jobs and deadlines are unchanged.

## Errors (what went wrong, how it was handled)

An initial test-append patch failed context verification because the new file
was missing its final closing brace. No partial test patch was applied. The
brace and tests were then added together and scoped Rustfmt parsed the result.
This is a source-edit correction, not a passing native build or test result.
