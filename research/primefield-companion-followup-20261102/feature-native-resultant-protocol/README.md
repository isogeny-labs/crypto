# Canonical resultant protocol and export integration

The prospective48-case/240-row panel now has a canonical experiment.protocol/v1 draft, an experiment.export/v1 success adapter and a bounded in-job capture wrapper. The real protocol remains draft: production binary/image, admitted preflight, publication/freeze and authoritative funds/capacity evidence are missing. No native panel or cloud job was launched.

## Canonical authority

Local protocol tools were absent (including Git-tracked paths) in both checked repositories. Retrieved the canonical tool, two schemas and normative document from isogeny-labs/crypto-autoresearcher at commit41bfaed260093c69da299cbad936205530909b6f. authority.json records Git blob IDs, byte counts and SHA256; authority/ contains exact unmodified upstream bytes. common.py verifies them before loading. The provenance is https://github.com/isogeny-labs/crypto-autoresearcher/tree/41bfaed260093c69da299cbad936205530909b6f . This is a pinned validation dependency, not a replacement schema. No canonical file is edited.

protocol.draft.json passes the pinned schema and semantic validator. It binds inputs, metrics, all directly compiled native modules, the analyzer, capture/adapter/common code and counter definitions. Five arms share a source revision; production binary_sha256 deliberately remains null. Preflight runtime remains null. make_draft.py deterministically regenerates this draft; it does not freeze anything. The detailed rotation rule accompanies the schema's sequential ordering enum.

## Capture and admission boundary

capture.py is an in-job wrapper, not a job submitter, resource enforcer or authorization service. By default it checks only. --execute requires a correctly hashed, filed-before-data frozen protocol, unchanged source/data/tool bindings, a matching binary, and externally supplied authentic admission evidence. It checks the one-attempt120s/1CPU/1024MiB/concurrency1/USD1 envelope, sufficient finite remaining funds, deadline, job/pod/image identifiers and publication/quota verification fields. Admission must name one absolute attempt directory; creation is exclusive, and renaming the directory requires a different admission record. It never raises permissions or resource caps.

Admission assertions and evidence hashes are not signatures or live queries. The authorized external operator must supply and verify them; this wrapper cannot turn self-authored JSON into cloud authorization. No real admission file was created. CPU/RAM limits must be enforced by the admitted Job/namespace, not this Python process. Successful exit alone is not a valid comparison; the strict analyzer must also accept every row/control.

The direct child gets at most120s, clipped to the remaining admitted deadline, and no retry. Expiry during checks creates a timeout receipt without starting a child. Capture writes an external receipt with actual exit/status, raw/log hashes, binary/input/metric/protocol pins and available host facts; timeout, nonzero exit and launch OSError retain failure receipts. Host collection uses best-effort file/syscall reads. Host reclamation or forced process termination can still prevent a receipt; missing receipts invalidate the run and do not authorize a retry.

## Export semantics

export.py accepts only a complete successful receipt and the strict previous analyzer's240 valid rows. It writes comparison.json and a canonical export with separate counters, no valid wall clock and verification.status=no_claim (producer oracle, not independent review). Source dirty status is unknown, not asserted clean. It records raw/receipt/input/metric/protocol/analysis file pins and actual available host fields. Canonical schema, protocol binding and file verification must pass. Output directory is reserved exclusively; a validation failure may leave an empty directory and is never overwritten.

Failed/partial attempts remain raw rows plus failure receipts. They do not produce a successful canonical export or fabricated unexecuted cells. The canonical schema's attempts>=1 and complete planned-cell check cannot honestly describe never-executed cells as attempts; a separate failure-flow convention or canonical schema extension remains an integration obligation before execution. No failure is dropped or counted as a success.

## Software validation

validation-02 runs the canonical draft validator and six synthetic test methods. Tests cover real-draft launch refusal, missing-preflight freeze rejection,18 admission mutations, complete synthetic240-row export with upstream check-export --verify-files, corrupted receipt/incomplete-panel refusal, output overwrite, check-only no-child behavior, and mocked nonzero/timeout/launch-error capture. Mocked frozen protocols and admission records exist only in temporary test directories; they are not research freezes or authorization. No native arithmetic panel is invoked. [pcc:unverified]

Replay: use a Python environment with jsonschema Draft202012Validator, e.g. the recorded venv, and run `python validate.py NEW_OUTPUT_DIRECTORY`. Each command limit90s, sequential, one invocation; source/tool/log hashes are retained. Raw development issues: initial local tool lookup/restore failed because paths were not tracked; canonical order enum rejected free text; export analysis-file metadata needed bytes/kind; mocked subprocess exposed platform.processor's fallback. Corrections are in this successor, with preceding evidence unchanged.

Sources checked: pinned normative protocol document/tool/schemas; previous panel/analyzer and prior admission-reconciliation receipt. KB unavailable: no MCP tools and wrapper executable missing. This is software integration, not a new arithmetic result or independent evidence review. Next: honest failure-flow export semantics, production build/preflight admission, then freeze/publication before any panel data. Original4563-case cohort unchanged.

Evidence: PCC-20261107-c205f0, unsigned producer attestations. validation-01 is preserved; validation-02 covers the final deadline-hardened source.
