#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= TR-04: checkpoint failure before primary execution

Run 40973940936, attempt 1; dispatched 2026-10-29 23:33:25 UTC;
completed with failure at 23:41:34 UTC. Exact source and workflow:
#text(size: 8pt)[85fc870c96a3ea707afe11e06cf884eba6f957ce].

*Zero primary field attempts. Zero completed research rows. All 27 expected
rows missing. This is an infrastructure failure, not a null scientific result.*

#image("execution.svg", width: 100%)

== Recorded execution and evidence

Preflight job 123797235834 passed source/schema checks, 40 native tests,
strict Clippy, formatting, frozen-protocol and analysis-source identity,
and presence of existing artifact credential bindings. The unmodified
checkpoint gate returned 1: no AWS profile ecc-challenges-new, no upload
token and no token file. Nothing was checkpointed. The field matrix and
remote aggregate were skipped. No research worker, primary binary hash,
relation record, control receipt or independent primary replay exists.

GitHub artifact 12941208365 contains all 15 original preflight files.
The downloaded ZIP is 19,654 bytes and matches GitHub's SHA-256:
#text(size: 7pt)[3005f6c178558119459e3553f28bc7fa842ec7251123fcd6d432c35fb66f0570].
This verifies a GitHub download, not artifact-store delivery/readback.
Raw ZIP, job log, metadata and analysis command/exit receipts remain
preserved in evidence.tar.xz, with hashes in receipt.json.

== Reporting and the separate PR fix

The frozen native analyzer read only three absent field paths. Summarize
and report returned 1; the canonical export has zero actual rows and lists
all 27 missing IDs. Its flow planned=0 counts actual exported rows, while
the protocol still requires 27. No missing attempt was invented. Both native
PDF pages were inspected; their editable drawing commands are retained.

Reviewer comment 4419135390 also found a distinct aggregation bug:
#raw("tr04-p*") matches the preflight artifact. The repaired pattern requires a
digit after p. Actual Minimatch 9.0.3 regression and Actionlint/ShellCheck
passed; scientific code and frozen protocol bytes are unchanged. This bug
did not cause the observed checkpoint failure. No new job was dispatched.

== Scope and open requirements

All four requested scientific parts remain implemented but unexecuted.
No repeated run, infrastructure change or checkpoint bypass was performed.
The earlier 5,389-pass root run validates exact source 445d84bf7 only;
it does not substitute for this primary census or validate later main.
Generic decomposition (bounded by sqrt(r); cannot beat rho); primitive
counter independence remains unset. No relation-yield, timing/GAE, scalar
recovery, end-to-end DLP, security or universal impossibility claim follows.

Sources: README.md, receipt.json, native-report/export.json; GitHub run and
artifact IDs above; actions/download-artifact v4 src/download-artifact.ts
blob 5cc6b1704fb5ef0d4487ed63d910d4f8a9f2fbd2 uses Minimatch matching.
