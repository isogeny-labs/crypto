# Ordinary design checks, not scientific validation

This directory records canonical draft-schema, PCC ledger/prose/standing, source-binding and document-rendering observations. None executes the proposed field/group/solver/auditor, a corruption path, a Lean elaboration or an IDC1 check. The one new PCC record is a `statement`, with standing UNVERIFIED/exit2 and no signed verdict. [pcc:unverified]

Nine exact failed metadata/document sources/logs are retained in deterministic `attempts.tar.xz`, with `ATTEMPTS.sha256` and an archive receipt. Extract into a fresh directory with `tar -xJf attempts.tar.xz -C <fresh-directory>`, then run `sha256sum -c <absolute-path-to-ATTEMPTS.sha256>` from that directory. All payload bytes and deterministic repack were compared before the duplicate loose copies were removed. One raw TeX log has trailing whitespace; the archive preserves it without changing raw evidence or weakening the whitespace check. [pcc:unverified]

The canonical checker and schemas were read byte-for-byte from crypto-autoresearcher main `b9c6c32a70d273a22355019b59ed897d6ad733eb`, outside this repository. Its first run rejected15 schema findings (enum/type values); the second rejected an absent absolute comparison threshold. Both exact draft sources/logs remain here. The corrected draft passes the unmodified checker with `freeze.status=draft`; this does not fill missing source, analysis, binary or runtime hashes or authorize execution. [pcc:unverified]

PCC prose lint was first invoked on a directory and raised IsADirectoryError; the file-list invocation then found three untagged words. Exact logs remain; tags/wording and invocation were corrected without changing any verification policy. The final prose and append-only95-entry ledger checks are ordinary producer observations. [pcc:unverified]

The isolated no-checkout worktree initially had an empty index; ordinary sparse `read-tree -mu HEAD` populated ONLY this new worktree. No old merged worktree, branch or evidence was advanced. Conductor exits127, with no applicable connected coordinator; no shared lease or replacement is claimed.

The current atlas match first lacked its imported metadata helper. The unchanged current helper and its YAML loader were hydrated outside the repository and the match completed. Its output records existing mechanism families; no lead disposition or novelty promotion was written.

The initial report had a small footer overrun; its exact source/log remain. The initial PDF is preserved in public scratch `/workspace/twist-chord-preflight-20261105/report-first.pdf`, not uploaded or claimed as a self-contained archived scientific record. The final two-page compact PDF was rendered and every page inspected. PDF compaction changes text ligature encoding; page text is equal after explicit Unicode NFKC and whitespace normalization. This is a document check, not proof of its formulas. [pcc:unverified]

No library, benchmark, numerical graph, registry, workflow, verifier, old source/result/identity or merged PR changes. No scientific run/proof/checkpoint/upload/paid job or timing/cost claim. The file and command receipts below are producer observations; no signed independent execution validation is asserted. [pcc:unverified]
