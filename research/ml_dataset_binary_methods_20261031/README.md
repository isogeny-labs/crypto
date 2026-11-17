# Ordinary binary n23 paired-method dataset campaign

`protocol.json` freezes a 48-cell native population: one curve seed, two
generators, two keys, both twists, and SAT, Boolean F4, Boolean F5, finite
selector Weil descent, double-large-prime, and BSGS at `m=2`. Methods of the
same twist share public instances. The protocol and `plan.json` were committed
before generation at `6e536e075e27614c4fb84e7a05533a5635ada52e`.

`campaign.py check` admits only the pinned clean native binary, exact committed
protocol and plan bytes, the second local XFS filesystem, and accessible S3
credentials. `campaign.py run --max-new-cells N` starts at most N new cells,
one at a time. Each has a 30-second inner solver cap, 90-second outer native
runner cap, 2 GiB memory limit, 64 MiB trace cap, a 12-hour campaign ceiling,
and an 8 GiB free-space floor before a new attempt. It preserves a one-attempt
marker, command receipts, native verify/report/strict export outputs, restricted
archive, content-addressed S3 PUT, and fresh GET/hash receipts. A failed stage
stops the wave; rerun resumes only finalization and custody for an attempted
cell. The external Conductor lease must be active for any write.

The exact native status is retained separately from the administrative stage
status. In particular, a `partial`, `budget_exhausted`, or `unsupported` native
outcome can still have a verified sealed artifact and S3 custody. A scalar
label requires independent native recovery plus group replay. These cells do
not support method speed comparisons from shared-host clocks, an UNSAT proof,
or a full Weil solver claim.

After all 48 cells have terminal custody checkpoints,
`make_experiment_export.py` projects their receipts into
`experiment.export/v1` and runs the pinned official checker. Its 48-row
denominator includes all failures, timeouts, and unsupported outcomes.
