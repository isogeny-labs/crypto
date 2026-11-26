# Preserve both histories while unblocking M31 native validation

This metadata-only resolution merges main `81f0308c75204f3a1c0ec52fdf78396eee0898ed` into the M31 implementation at `8233cbb8fc844b7321616923f52b8d4805e95e52`. The native converter, workflow, frozen protocol, original claim and scientific evidence are unchanged. No PR merge, paid compute, scientific verification or signature is performed. [pcc:unverified]

The only conflicts were concurrent insertions in `docs/ACTIVE.md` and competing append-only ledger envelopes. `ACTIVE.md` retains every byte of main with the unchanged M31 task inserted. The ledger retains all 1,845 main entries as an exact byte prefix. The branch's original 404-byte envelope is archived verbatim in `branch-envelope.jsonl`; its unchanged claim body is registered after main as sequence 1,845. The candidate has 1,846 entries. `registration-map.json` records both envelope hashes, claim identity, exact input revisions and validation checks.

The executable `reconcile_metadata.sh` reproduces the candidate files using Git, jq and SHA-256, following the unchanged PCC metadata contract. Native chain validation, the main-prefix comparison, claim/body identity and the ACTIVE preservation comparison passed. This creates a new registration envelope, not a new scientific claim or a change in its unverified standing.

The repository's Copilot-first conflict request received no response during a wait exceeding 53 minutes. The attempted browser connection stalled and was stopped before any manual workflow action. The coordinator therefore took the documented conflict-resolution fallback and reviewed the candidates. Native M31 replay remains pending until the existing CI job actually executes.
