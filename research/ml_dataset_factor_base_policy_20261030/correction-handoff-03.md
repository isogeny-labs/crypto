# Proposal03: targeted post-execution corrections

Immutable parent proposal02 is unchanged. This revision changes only export.rs
software fixtures, one mod.rs semantic test, and visibility of the existing
protocol_canonical_bytes function (`pub fn` for the external acceptance example).
No runtime predicate, policy constraint, solver algorithm, or canonicalization
implementation is weakened or replaced. Donor composition remains separate.

The parent-run gated01 suite compiled, then reported345 passed/4 failed/1 ignored
at17:51:49Z. Its full retained log is
/home/ec2-user/ml-dataset-work-20261028/proposal-dataset-01.log. These are inherited
input failures, not tests executed during this revision. Original log and prior
source snapshots remain unchanged. Failure excerpt retained beside this note.

Three failures were the shared exporter helper's axes `twists:false`, rejected
by DatasetPlan validation. The helper now requires paired twists and computes
the policy-major block from axes.cell_count() (asserted2). Explicit legacy
normalization uses policy_index0/cell0; signed censor/unsealed controls use
policy_index1/cell2. Tests assert policy identity, origin, axes index and twist;
manifest index comes from BoundCase. A boolean-only repair would silently select
the wrong policy, so it was not used.

The fourth failure asserted a chain-specific error after transplanting valid m4
algebra into an m3 cell. New BoundCase factor-base admission runs earlier and
rejects its public m4 context against frozen m3. The retained log did not print
that returned error, so this exact diagnosis is source-derived until rerun.
The revised test independently verifies public curve, factor base, reconstructed
m4 equations and m4 group certificate, then asserts frozen configm3/publicm4.
It compares the exact early occurrence mismatch against central replay. This
proves rejection is binding-related, not an invalid polynomial or group witness;
it does not accept an arbitrary error or skip the semantic test.

Nativefixture02 requires external access to the repository's existing protocol
canonicalizer for float values such as alpha0.05. Its author requested the exact
`pub fn protocol_canonical_bytes(value: &Value) -> Result<Vec<u8>, String>` hunk;
no new canonicalizer or implementation change is introduced.

combined-proposal.patch applies against exact655b690ef56d6869023a41326a127bf319cf4342.
correction-from02.patch applies against enabled proposal02. Both were replayed
with exact context into separate scratch copies, and all16 resulting source
files matched proposed/. Rustfmt --check passed. No Cargo, tests, native calls,
cloud operations, live edits, or Git mutation occurred. Parent should freeze and
run03 independently before adding the larger donor composition, to localize any
remaining failures. All integration/testing limits in proposal02 still apply.
