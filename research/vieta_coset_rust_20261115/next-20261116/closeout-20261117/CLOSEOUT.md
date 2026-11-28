# PR4410 publication closeout

PR4410 merged on 2026-11-16 at23:16:15UTC as
`2b15288e325bcfea86175601c1321d1520c9d6d2`, from head
`0396799e4f45413141775061121d9253c525a1d4`. The API reports merged_by=aburan28.
This follow-up observed the completed merge; it did not invoke a merge command.

CI was **not green at merge**. The PCC gate failed at22:02:08UTC and artifacts
failed at22:08:12UTC, both before the merge. Size passed at22:07:11UTC. The
later snapshot contains15 failed,1 cancelled,11 successful and8 skipped jobs.
These results are retained literally; neither the merge nor a local pass
changes a failed remote check into a pass. The reason GitHub accepted the
merge is not established by these observations [pcc:unverified].

Selected log causes:

- PCC: missing claim_type diagnostics; the earlier147-error main/head
  reproduction remains in the immutable reconciliation receipt.
- Validation: shared artifact HeadObject403 download failures.
- Artifacts: `wire_workflows.py` reaches `read_manifest` and fails JSON parsing
  with Extra data at line10,column1,char419. The shown traceback does not name
  the malformed manifest, so this receipt makes no file-level attribution.
- Rustfmt scope setup: `ci-scope.mjs` cannot read
  `HEAD:examples/pdp_lin_cdcl.rs`; formatting does not start in that job.

The status snapshot includes the actual job URLs, effective required-check
rules, empty review/thread lists and the three independently retrieved job
completion times. Selected public log excerpts are retained separately.
No credential workaround, gate/policy edit, historical claim/record rewrite
or new experiment was performed by this closeout [pcc:unverified].

At recording base2043bee6ae, the scientific source, all study files and the
five selected scientific claim files equal the published head byte-for-byte.
All prior measurement and calibration-custody limitations remain. Data and
the hand last-fall-degree argument still await independent signed standing.

The append-only autonomy entry records a **partial publication-window** outcome
with ci_green_at_merge=false. Its start is the known PR creation time, not an
invented first-prompt timestamp. Original prompt/intervention timestamps and
full-session costs are unavailable. Two visible later human-message hashes
and rule labels are retained without fabricating timestamps; no zero-human-
intervention or unsupervised-success assertion follows. Incident coverage is
limited to three known self-recovered metadata/preview incidents. Unresolved
infrastructure incidents remain described above. The matching lesson outcome
also remains partial [pcc:unverified].

Automation2624 is disabled, with a successful service response. No replacement
study polling is scheduled. Revert review is due2026-11-30T23:16:15UTC and has
not been performed. This follow-up does not reopen PR4410 or rerun benchmarks.
