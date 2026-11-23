# Accepted prospective run record

The main run uses **[protocol.frozen.json](protocol.frozen.json)**, canonical
SHA256 `5bcd31006a76336912ac3687a371d4bc3f291feeb1cb4058d07e0f2f0645d625`.
It was frozen at 14:30:46 UTC on 23 November 2026 against published source/control
commit `b4997f9ef62a8d7c7fcfd19c85e49eb5333246a3`, before main data. Earlier
`protocol.draft.json` and `protocol.json` are preserved pre-freeze snapshots.
[pcc:unverified]

The exact 295-entry main execution manifest and zero-call pre-call declaration
are in `checks/main-execution-before.sha256` and `checks/main-pre-call.json`.
They bind source, copied qualifiers, host/runtime receipts, config, frozen
protocol, worker wrapper, outer guard, supervisor and finalizer. The exact
supervisor argv is in `checks/main-command.json`. One attempt starts only when
the main supervisor marker is created in the declared data root. [pcc:unverified]

Raw output root: `/tmp/ecdlp-cpu-comparison-20261123/main`. All control fixtures,
synthetic measured-summary records and warmup-only preflight stay outside that
root. They are never main observations. The nominal budget and actual enforced
phase limits are stated in MAIN-PROTOCOL.md; no hard overall watchdog is claimed.

Read-only reviews 10–13 remain distinct historical scopes. Later fresh source
custody receipts do not rewrite review 12's original V2 pre-unit limitation.
The main snapshot requires publication/readback and a local resume checkpoint
before launch. S3 stays excluded; full payloads stay local with exact receipts.
