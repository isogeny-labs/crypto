# Publication checks and qualification boundaries

This file records administrative checks separately from the frozen native
attempt. None of these checks is a new finite-field experiment or solver
comparison. The proof and numerical observations retain the unsigned
standing stated in the report. [pcc:unverified]

## Observed local checks

| Check | Outcome | Interpretation |
| --- | --- | --- |
| PCC verification against main's pinned keyring | Exit 2, `not_covered` | All four subject hashes match; unsigned attestations and a statement claim cannot establish a signed verified result. |
| Ledger verification against base ea922ca25fa85005c6f45ec5b53a7d91e1b293e2 | Exit 0; 1,859 entries, chain intact | The base ledger remains an exact prefix. Existing legacy missing-type warnings are retained in the output. |
| Report, PR body, source review, receipt review and proof-status prose lint | Exit 0 | No findings in the listed files. |
| Exact PR-body stdin lint | Exit 0 | Same invocation used by the strict PR-body workflow step. |
| Optional full mathematical-note prose lint | Exit 1; four paragraph-tag findings | Advisory observation described below; no hash-bound file was rewritten. |
| Artifact-policy check on staged tree a2eedc53b20e3f6dba8bbe83daf6d1995afa9b85 | Exit 0 | The report, PDF, source and retained native records obey the existing artifact policy. The policy explicitly keeps PDFs. |
| PDF visual inspection | All six pages inspected | Diagram border, both rank-plot legends, tables, equations, commands, links and page boundaries checked after final rendering. |

The exact administrative commands, exit codes and output are in
`evidence/publication-checks/`. The staged-tree check predates this metadata
file and the final manifest; the final committed tree is checked again
before publication. It is not presented as a remote CI result.

## Preserved prose findings

The optional check of `MATHEMATICS.md` reports missing paragraph-local PCC
tags beside “improvement” at line 13, “proves” at lines 307 and 416, and
“proved” at line 536. The note already labels its overall standing as
unverified. Its bound SHA-256 remains
`03ee4c984991945986ffa155f596aa2ebc0f7eaefc3f89e50d922878708e2c4b`.
[pcc:unverified]

The existing `pcc-gate.yml` makes PR-body lint strict and report/session-log
lint advisory. It does not scan `MATHEMATICS.md` in that advisory file list.
These notices are therefore recorded without rewriting the claim-bound
artifact, its claim, its ledger entry, or a workflow. The separate status
map and report state the actual qualification explicitly. [pcc:unverified]

## History and remaining checks

Build and checker producer commits in the receipts are local commit
identifiers. Publication through the GitHub connector can give the resulting
repository tree a different commit identifier; the source hashes, frozen
closure and raw receipts are the reproducibility anchors. No producer
identifier is replaced with a later publication identifier.

The previously published written checkpoint at remote head
`e6292e3887a6cb3081f8b5ded3e50a97d76d2879` is retained byte-for-byte,
including its full journal block. Its statement that native tooling was
unavailable describes that earlier invocation. The current report describes
the subsequent frozen native build and checks in this workspace.

The standalone binary is the qualified compilation scope. The root crate
was not rebuilt, cargo-fmt was unavailable, and the aggregate benchrec scan
retained historical missing-artifact/open-record findings. No formatter,
root-crate, aggregate benchmark or remote CI pass is asserted here.

The M61-only v1 identity-certificate route was not used for the M31 work.
Lean formalization remains open with the concrete status in `PROOF_STATUS.md`.
Required CI and external review must be observed on the actual published
head before any merge; this local record does not replace those gates.

The first final-packaging command reported one missing paragraph tag in
this unbound administrative file and one extra blank line at the end of
`PROOF_STATUS.md`. The shell continued to make a local commit after those
diagnostics. Both presentation issues were corrected in a follow-up local
commit before publication; the original mathematical note, native sources,
claim and scientific receipts were not changed. The subsequent packaging
check and its explicit outcomes are retained separately.
