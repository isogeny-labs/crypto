# Publication validation and limits

Public PR: [crypto #5115](https://github.com/isogeny-labs/crypto/pull/5115).
Source for official helpers and existing ledger: `ceba74bb64854027c4dd16f00428bf676383bfdb`.

The administrative claim `PCC-20261124-afb0db` binds only the immutable [publication preflight record](manual-m31-publication-preflight.json). It is an unsigned execution/data attestation. The official verifier reports `not_covered` (exit 2), and standing reports `unverified` (exit 2). It does not cover the report's scientific conclusions, authenticate a new benchmark or provide independent validation. All written interpretations retain their evidence limits. [pcc:unverified]

The official ledger verifier accepted the exact public ledger prefix and the one appended claim registration: 1,849 existing entries, 1,850 after append. The original 1,022,465 bytes remain identical. This is a chain/prefix check, not a complete verification of all historical claim payloads. [pcc:unverified]

The exact retained ten-query input was hashed locally: 165,511 bytes; SHA-256 `e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab`; Git blob `7fd27b9888058399d8814cef373d373243e6a1e5`. Both matched the source protocol. No group-law or matrix-rank replay was inferred from hashing. [pcc:unverified]

The initial prose lint found four missing scope tags. The publication copies carry those tags; the final official lint result is recorded separately. No full cryptographic build, Lean build or new benchmark was run. The metadata-only partial-source benchrec attempt lacked the repository's 403 record files and generated progress artifacts. Its exit 1 is an incomplete-local-input result, not a full-repository baseline failure or a pass. The branch must follow its normal hosted checks and review gates. [pcc:unverified]

The root-log refresh is unfinished for a concrete policy reason: official `belongs_in_store` selects the existing 73,956-byte `leads.log`, because `.log` is a data suffix over 65,536 bytes and the configuration has no matching keep rule. Its Git blob remains `862858fb9af88bfb7263c41d403cfe782eda4cfe`; SHA-256 `96193af9735727ab958a2d42eeb33725b4852b2e67dcab862f2fc242775e90b7`. The file is untouched. No policy bypass, exception label, truncation or speculative artifact-store action was used. PDFs match the existing `*.pdf` keep rule.

Sources checked (KB): connector unavailable; documented wrapper exit 127, no executable. Conductor was absent; the work used a bounded source/metadata scope with existing research ownership preserved. These are retrieval and coordination limitations, not assertions that prior research or active jobs are absent.

## Final staged-source checks

The pinned official PCC prose linter returned exit 0 with zero findings for the five new report Markdown files and the appended journal entry. The PR body, supplied through standard input, also returned exit 0 with zero findings. This validation note and the algebra example's added `f(r)!=0` requirement each received a focused follow-up lint; both returned exit 0. The original journal blob `80e16960c96bf88005a812bbbf4d9537391e36ad` remains an exact 39,104-byte prefix; the new entry adds 2,348 bytes. Historical prose was not edited or included in the new-entry lint.

The official artifact selector, loaded from the pinned public helper and unchanged policy, selected none of the staged paths recorded in [manual-m31-final-checks.json](manual-m31-final-checks.json). The record contains the checked path sizes and hashes, source identities, commands and limits. It also checks that the staged administrative claim, its proof artifact and its appended ledger exactly match the prepared versions. This is a path-policy check, not the complete hosted artifact workflow. PDF rendering and visual inspection are a separate publication step.

These final checks cover documentation syntax, source custody and publication policy. They do not supply a signed mathematical verdict, native replay, complete benchmark-record gate or hosted CI result. [pcc:unverified]

Impact score: 2/10 — The checks caught a real publication-policy blocker and tightened the claim scope. They do not validate a new cryptanalytic result. [pcc:unverified]
