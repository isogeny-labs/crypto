# Structured IC post-merge software and custody audit

2026-11-05. Producer statement PCC-20261105-e660a7, standing unverified.
The existing public known-answer controls report three passing CLI tests in
each consumer and 33 passing legacy suite library tests. Scoped tensor Clippy,
locked transitive metadata and the source/custody checks also report passing.
These are software observations from this session. [pcc:unverified]

See [report.pdf](report.pdf), [audit.svg](audit.svg), [audit.json](audit.json)
and [custody.json](custody.json). Exact commands, working directories, timestamps,
exit codes and raw-output digests are in `checks/*.json`; raw outputs are retained
losslessly as `checks/*.txt.gz`. Durations in native test output are diagnostics,
with no performance comparison or isolation claim. [pcc:unverified]

| Original requirement | Audit observation | Evidence |
|---|---|---|
| Three native consumer entry points | 3 CLI controls pass in crypto, bindings and suite | `checks/{crypto,bindings,suite}-cli.json` |
| Older suite API | 33 existing library controls pass; original exports remain byte-identical after removing the additive structured aliases | `checks/suite-library.json`, `custody.json` |
| Scoped six-file parity | Both current main copies match the original published crypto source | `custody.json` |
| Frozen original reports | 26 original paths in each implementation report are unchanged | `custody.json` |
| Exact archive and KN record | All 14 receipt hashes match archive commit b7955c3d13834c076b7b345c44b4912bfdb03d4b; 13 live paths also match | `custody.json` |
| Historical journal custody | The original journal blob matches at the reachable archive commit; its live journal contains later entries | `custody.json` |
| Original claims and ledger history | Original claim bodies are unchanged; both ledgers retain their merged-main prefixes and valid chains | `checks/{crypto,ca}-ledger.json` |
| Claim standing | PCC-20261104-458dda and PCC-20261104-f011a3 remain unverified, with no signed verdicts | `checks/{crypto,ca}-standing.json` |
| Genus-2 tool lock repair | Full locked metadata resolves with the same retained graph digest | `checks/genus2-lock.json` |

All table entries describe producer checks, not independent signed execution
evidence. The archive comparison uses the receipt's source commit, including
its historical journal bytes; it does not substitute the current mutable journal
for that archived blob. [pcc:unverified]

## CI findings and attribution

Historical post-merge crypto jobs failed on registry-name references and a GitHub
HTTP 500 during artifact/Git transfer. Autoresearcher's historical benchmark-record
job failed on an absent source-control PDF. The naming inputs and missing-record
paths were unchanged by these integration PRs. The pinned current-main release
and autoresearcher record jobs report success. [pcc:unverified]

The pinned crypto main still has failing whole-repository Clippy, ephemeral-CLI
and naming jobs. Their logged source locations are outside the structured
consumer change; scoped tensor Clippy reports success. These baseline defects
remain visible in the captured evidence. No validator, test, fixture or unrelated
benchmark data was changed. Full check snapshots and original failed logs are
retained in `ci-snapshot.json.gz` and `checks/*-ci.txt.gz`. [pcc:unverified]

## Reproduction and scope

`audit.json` pins the three main revisions. The custody script reads those exact
Git objects, the source commits in the intake manifest, the archive receipt and
both original claims. `run_control.py` records each command and refuses to
overwrite an existing control. For fresh reproduction output, set
`STRUCTURED_IC_AUDIT_OUTPUT` to an empty directory and use the exact commands and
revisions listed in the control records. Installed Python dependencies for the
optional visual/PDF renderer are recorded in `environment.json`.

Sources reviewed: the [consumer contract](../../docs/structured-prime-ic.md),
the [original integration report](../structured_ic_integration_20261104/README.md),
cryptanalysis's `docs/ic/structured-prime-ic-20261104/README.md`, autoresearcher's
intake/receipt and KN-OPEN-08ba7e, and the
[PCC contract](../../docs/SABOTAGE_RESISTANCE.md). These define the existing schema,
signed-tuple semantics, failure behavior, provenance limits and statement
standing. This audit imports no additional algorithm or complexity theorem.

The consumer's receipts and replay retain producer-assertion standing. The open
full constrained-summation-message requirement remains in KN-OPEN-08ba7e. An
independently signed execution verdict requires the repository's trusted verifier
chain; this session did not produce one. [pcc:unverified]
