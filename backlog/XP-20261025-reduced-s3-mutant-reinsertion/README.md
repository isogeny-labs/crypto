# Adaptive row reinsertion on reduced twist S3 systems

Draft next iteration for PR2381. No run, lease, funding or Cairn publication
is claimed. Implement the runner and independent verifier, then commit the
exact protocol before marking this card ready. Its `run.argv` names that
future runner, not an existing executable.

The [executed reduced-S3 study](../../research/quadratic_twist_evidence_20261021/reports/reduced-s3-20261025/README.md)
lowers the F43 input-leading cutoff from 3 to 2 on both arms but has main/twist
affine closure 4/2. This makes a precise adaptive solver-degree certificate
the next checkable step. It advances the lead atlas's closure-degree workstream;
it does not claim to have invented MutantXL.

Use all 98 size2 cases from `native/reduced-s3-20261025`, every nonzero target,
both transported-base controls and the 188 frozen comparisons. Compare the
original/reduced-generator reference profiles with an adaptive loop that
records newly found lower-degree rows, reintroduces admissible multiples, and
reaches a declared fixed point before increasing matrix degree. Pin ordering
and row-selection rules before execution.

Every added row must retain an independently checked combination witness in
the original inputs. Report the largest degree actually processed separately
from the degree of an expanded original-input witness. Reinsertion must not
hide an earlier higher-degree computation or relabel an expanded certificate
as a lower original-input bound. Retain actual solutions, coefficient ranks
and scalar-preserving transport. Include false-combination, false-degree,
missing-target and source/hash controls.

Acceptance is a replayed operational degree bound for the named finite solver
on actual inputs, with explicit exceptions. Ties/losses select the next
encoding; they do not end the investigation. No F4/F5 timing, intrinsic-
regularity identity, stationary yield or complete-work advantage is presumed.

Default native commands are capped at 600 seconds with a 3600-second envelope
for this single iteration, two CPU cores and 4GiB RAM/disk for native artifacts.
These are limits, not measured estimates. Estimates remain null until code
exists. Use existing local resources only; no paid hardware or upload request.
A future performance protocol must include all applicable baseline, reference,
timing and visual gates. No funding/reservation is created by this draft.

Next action: implement both native paths, freeze the protocol with commit/blob
pins, fill the independent checker command and validate again before marking
ready. Actual coordination/review/merge gates remain; no replacement coordinator.
