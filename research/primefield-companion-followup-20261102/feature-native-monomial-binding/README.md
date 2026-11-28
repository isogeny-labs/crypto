# Progress-runner source binding

The successor draft names the published progress-enabled runner at
`32d5ed1aaf13a27d2c48b4d89cab1cd6859ef403`. All arms share its binary.
`launch.py` checks the pinned dependency closure and rejects an admission whose
source commit differs from the instrument. It also rejects the older runner,
a changed/unfrozen protocol, expired or naive deadlines, and non-integer or
expanded attempt/timeout envelopes before creating output or spawning a process.
It delegates bounded capture to the preserved wrapper after these checks.

Six local software test methods pass, including exact build-command comparison
with the saved release receipt and hashing the published source closure.
The preserved protocol validator reports zero findings for the new draft.
Claim `PCC-20261107-41adaf` is unsigned producer evidence. [pcc:unverified]

The binding is a producer-authored provenance record, not an attestation or an
execution authorization. External admission must still enforce resources,
remaining budget, ownership and attempt accounting. Binary hashing is a prelaunch
check; it does not provide an immutable executable handle against concurrent
replacement. Run from an immutable admitted image. Progress output adds harness
work outside counters; no claim of wall-time neutrality is made.

The protocol is still **draft**. Preflight, authoritative budget/capacity
admission, exact launch-image publication, and canonical export handling of
unstarted cells remain open. No comparison or cloud job was run. The existing
failure-flow adapter preserves observed starts without fabricating attempts.
The original 4563-case cohort is unchanged.

Replay software checks from repository root:

```sh
python3 -m unittest discover -s research/primefield-companion-followup-20261102/feature-native-monomial-binding -v
python3 research/primefield-companion-followup-20261102/tools/experiment_protocol.py validate research/primefield-companion-followup-20261102/feature-native-monomial-binding/comparison-protocol.draft.json
```

Validation requires jsonschema with Draft202012Validator. The research venv was
used for protocol validation; no Rust build or cryptographic computation is
needed for these checks. Launch interface, only after external admission:

```text
python3 launch.py --runner <immutable-runner> --protocol <frozen-protocol> --admission <operator-receipt> --out <new-attempt-directory>
```

Sources checked: preserved capture wrapper, progress producer/source lock/release
receipt, and bundled protocol schema/tool. KB unavailable: no MCP tools loaded;
`scripts/research-kb status` reported no executable. This is a provenance defect
fix, with no new cryptanalytic mechanism or novelty claim. Historical artifacts
were left intact.
