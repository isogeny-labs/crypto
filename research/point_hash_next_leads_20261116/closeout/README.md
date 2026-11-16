# Planning PR4382 terminal receipt

Observed 2026-11-16 UTC. Producer Git/API/document evidence only; this is
not signed scientific, execution or performance standing. [pcc:unverified]

PR4382 is merged. The actual merge is
`69674b9276a1c1a35c00550f0d264f80b80ab118`, with ordered parents
`33d7a50f0597596c47828b68ad8b3fe2adbd8123` and
`403f5770600a6ade4adb5e984d8213670831632e`. Its root tree
`4fc58d1447b99354885e5bf9850ecc2dfb85ddf2` equals the final head's tree.
It is reachable from observed main
`222394077d1700e5e3124a385673bad4143875ef`. This is the actual merge,
not synthetic CI checkout `c1663e142ac3070a377edbe7449cbc4b88e923ee`.

The final head is an ordinary merge of uploaded planning head
`d3c38247a2a4fab01116f51279207a3111614894` and the main parent above.
The complete original study tree `fdd73fed36eb6f020ea2398c6bf8d166e0894503`
is identical in the uploaded head, final head, actual merge and observed main.
The report, PDF, rendering source, source manifest and original checks are
unchanged. No native source is changed by the 17-file PR diff against its
actual main parent. No reviews, requested reviewers or inline threads were
returned at the terminal inspection. [pcc:unverified]

The refreshed UTC journal preserves every full normalized H1/H2 block from
both parents, including repeated-block multiplicity and each source order.
Only conflict-marker lines and outer block newlines are normalized. There
are 15 source blocks in the uploaded-head journal, 32 in the main-parent
journal, and 33 in the result; neither source has a missing instance. Raw
Git blobs and complete source journals are retained in the receipt archive.
An initial parser attached H1 banners to preceding entries and reported
two false mismatches caused by boundary blank lines; both that attempt and
the corrected parser/result are retained, without altering any journal.

## CI qualification remains partial

The final-head API snapshot contains 58 check runs: 45 successful, 11
skipped and two failed. The merge occurred at 02:58:46Z, while several
of those runs were still pending. Neither merge status nor a successful
check with the same name establishes that all checks passed. The episode
records `ci_green_at_merge: false`. [pcc:unverified]

- [Artifact job 133062770306](https://github.com/isogeny-labs/crypto/actions/runs/43795338320/job/133062770306):
  tool tests fail with missing `wire_workflows.cover`,
  `evidence_store_files` and `reads_validation_gate` functions, plus a
  test expecting a rejection that returns success. Later artifact steps
  were skipped, so their hosted qualification is not established.
- [Validation job 133062771022](https://github.com/isogeny-labs/crypto/actions/runs/43795338594/job/133062771022):
  committed-tree tests fail on the historical aimed-gather registry digest,
  packed-filter tolerance exceeding the gate cap, and pre-cap report inventory.
  Subsequent validation steps were skipped.

The failing tooling, workflows and named historical evidence paths are
byte-identical between the main parent and final PR head. This identifies
them as inherited inputs, not a passing baseline reproduction. No unrelated
checker, record, tolerance, policy, workflow or branch protection was changed
here, and no failed job was relabeled as passed. The merge actor in the
timeline is the connected owner account; the timeline alone cannot identify
which client performed the merge. The full API snapshots and decoded failure
logs are retained without omitting their failure bodies.

## Autonomy and replay

Episode `E-03c8ebff3d50` records a merged but partially qualified planning
deliverable. Its start uses the first retained clock receipt at 01:54:57Z;
the exact initial prompt timestamp is unavailable, so this is a left-censored
start, not a trustworthy duration measurement. There was no later human
instruction within this planning episode; environment/automation messages
are not invented human interventions. Recovery entries describe only the
three corrected document/preflight incidents. The unresolved CI failures
are recorded separately, without pretending SELF or HUMAN recovery.
The fourteen-day revert check is pending, not recorded as false.

`terminal-verification.json` gives exact identities and journal results.
`terminal-receipt.tar.xz` preserves the raw checks/status/timeline, complete
decoded failure responses, source journals, initial parser failure and final
Git/document verifier. It is metadata, not a scientific run archive.

To replay into a new scratch directory, extract the archive, then run its
`verify_terminal.py --repo /path/to/crypto --output /new/scratch/output`.
Use a fresh output path; do not overwrite this receipt. The script pins the
observed main snapshot and checks original Git objects, rather than pretending
a later main tree is the original source. SHA256SUMS binds these closeout files.

Tracking PR4382 ends here. Implementation and all five experiments remain
unperformed by this tracker; the original scientific and execution gates
remain open. Native integration and ECBench are separate workstreams.
