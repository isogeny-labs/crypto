# Artifact publication standing

The native run archive is164272bytes, SHA256
8f7b6024a389f032ed7d5342d7b1e2422ea002a23673accda283de154332916a.
It contains the complete original raw JSONL, not a sample. Its64KiB data
threshold selects it under the actual repository policy. No exemption,
alternate extension or split is used. [pcc:unverified]

Publication is initially pending the maintained trusted artifact-autofix
workflow on PR4984. That workflow reads data bytes only from the candidate,
executes the trusted base artifact tool, uploads and replays full remote
bytes, and commits a .artifacts.json pointer. Require its actual subsequent
receipt with the above digest and byte count before calling upload complete.
Do not invent a receipt, expose credentials, delete remote objects or remove
the inspected local archive before a byte-consistent adoption is possible.

The source/delta and retained development archives are ordinary complete
archives too; their selection is determined by the same actual policy.
The report PDF follows the repository's existing PDF keep rule.

The complete retained first-layout archive is also policy-selected:
reports/section-coeff-v1/first-layout.tar.xz,201084bytes, SHA256
88925b1a3c7dcf40cbe81840122ed0d1168263a77c39e931872a0962f1667335.
It retains the initial source/log/PDF with two overfull boxes. The final PDF
was inspected after correction. Require the same genuine trusted upload and
remote replay receipt for this archive too. [pcc:unverified]

The already source/claim-pinned SOURCES.md has one prose-lint advisory on its
negative full-proof caveat. Its original bytes remain unchanged. SOURCES_V2.md
adds the missing unsigned tag; this is a citation-only current status, not a
new native input or a claim that whole-prose lint passed. No policy exemption
or frozen-subject edit is used to conceal the diagnostic. [pcc:unverified]

## Terminal PR update,2026-11-23

PR4984 merged16:33:36Z with both archive bytes still tracked, not store
receipts. Its actual final-head trusted migrate137843787991 failed after merge
on HeadObject HTTP403 for both expected hashes. No trusted upload or absent-key
diagnosis is inferred. Full decoded log and actual merge custody readbacks are
in ../pr-closure-4984-20261123; PR5022 owns publication follow-through. Backend
restoration or an owner-approved supported store is required; no unchanged
manual rerun or policy workaround is used. [pcc:unverified]

The complete failed-job log gzip is also selected by the actual data policy:
../pr-closure-4984-20261123/migrate-137843787991.decoded.log.gz,82727bytes.
MIGRATE_LOG.json pins its hash and complete628339-byte decoded UTF8 stream.
This is not a raw-server-byte or signed execution receipt. Keep all inspected
bytes until genuine trusted hash/size receipts permit pointer adoption using
the maintained helpers. [pcc:unverified]
