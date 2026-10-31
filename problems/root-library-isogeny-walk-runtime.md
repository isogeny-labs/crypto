# Parent-library validation does not finish its legacy isogeny-walk case

- **Status:** open
- **Kind:** incomplete execution; termination cause unknown
- **Affected work:** [primality follow-up validation](../research/hashed_ecdl_primality_fast_paths_20261030/README.md)
- **Next action owner:** unassigned root-library maintainer
- **Last updated:** 2026-10-30

## Problem

A full parent-library pass cannot be claimed because one unchanged legacy test
did not return before its cargo/test processes were killed with SIGKILL.

The case is
`cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
It calls the existing `run_end_to_end(101,1,8,&[3,5,7],3,1.3,1.25,Some(64))`.
This is separate from the primality package and published ECCp-131 network
construction control. Neither the case nor root-library source was edited
for this follow-up.

## Evidence and uncertainty

- **Observed:** `cargo test --locked --release --lib` at `95cf0a81` compiled
  and started 5,567 default-scope cases. Eighteen cases initially failed because
  runtime fixtures were outside the sparse checkout; every exact-name replay
  passes after materialization against the same executable.
- **Observed:** comparing the test inventory with recorded results leaves
  only the named legacy case unreturned. The 123 default ignored cases were
  neither enabled nor reclassified. The final log has no full-suite result.
- **Observed:** `/proc` reported state Z and encoded exit status 9 for cargo
  PID 12024 and executable PID 13556. No validation process remains running.
  Executable SHA256:
  `b1d584c5a85a4407e20731acc2978bca4fb8f93e06f6d1d3e5196ead5d328fe0`.
- **Unknown:** the cause of SIGKILL and the exact active runtime are not
  established. The observed process lifetime includes time after termination.
- **Not established:** a mathematical obstruction, a failure of the new prime
  proofs, or a green parent suite. The earlier successful receipt at
  `research/prime_tensor_variants_20261030/receipts/ROOT-VALIDATION.md` concerns
  another pinned source and does not discharge this run.

An additional isolated replay on the identical pinned source/executable used
an explicit 180-second development deadline. GNU timeout returned 124 after
terminating the case; no test result was produced. This establishes only
non-completion under that declared policy. It neither explains the original
SIGKILL nor establishes an algorithmic obstruction. The small complete replay
record is committed separately in the study's `parent-validation` directory.

Raw parent output, inventory, fixture reruns and process observation are
preserved locally; the full archive's upload failed for missing AWS credentials.
The [archive status](../research/hashed_ecdl_primality_fast_paths_20261030/parent-validation/README.md)
does not claim a verified remote copy. Native-package tests and construction
validation are separate evidence, not replacements for this check.

## What would unblock it

- [ ] Complete the exact named case with exit 0 under a declared development
  resource policy and retain source/binary/input identities and full output.
- [ ] Complete the full required parent scope with all fixtures present,
  retaining default ignores and every failure; do not weaken or skip this case.

That would close this execution gap, not the independent mathematical audit
or either challenge-solve requirement.

## Next discriminating step

Replay the single case on pinned source with the restored fixtures and an
explicitly recorded deadline before another full run. If it completes, collect
the full scope. If it again fails to return, profile the legacy route and fix
that baseline in a separate change without altering its acceptance condition.
An infrastructure failure must be distinguished from algorithm runtime.
No new scientific job or spending cap is authorized by this record.

## Resolution and history

Unresolved. Recording this problem does not make the parent validation green.
