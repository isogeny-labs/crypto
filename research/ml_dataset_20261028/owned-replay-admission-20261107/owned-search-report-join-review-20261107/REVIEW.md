# Independent report-join review

Scope: report_join.py plus check_capture.py delta against the frozen semantic-replay packet. Hashes of reviewed inputs are in REVIEWED_HASHES.json. No repository, frozen packet, build or cloud mutation occurred. One small adversarial capture was generated in a temporary directory solely to inspect join behavior; its receipt and reproducer are retained here.

## Finding: joined snapshot head can precede actual committed head

The report joins `capture_head_event_count` and `capture_head_sha256` to the final snapshot's last-chunk acknowledgement, but does not require that acknowledgement to be the full committed capture head. Appending a valid unrelated committed event after the final snapshot leaves the old report accepted as `verified_selected_rows` with `report_capture_join_verified=true`. The probe records acknowledgement event 12 versus committed head event 13.

This is weaker than the actual Rust producer report precondition: OwnedAttempt::report_inner checks `self.sink.committed_event_position() == ack` before returning the report. The row identities themselves remain verified relative captured equations; the finding concerns terminal-head joining, not polynomial correctness.

Recommended correction: require the joined acknowledgement's event_count equals the fully verified capture event_count. Given the checked hash chain and position map, that guarantees it is the same committed head. If historical report joining is intentionally desired instead, make that weaker temporal scope explicit and avoid claiming a current terminal-head match. Add a valid trailing committed-event negative control, and a multi-snapshot stale-report control.

## Checks supporting the remaining scope

- Callback change is only the addition of verified chunk count and a copy of the last chunk's checked acknowledgement. Frozen donor integrity verifies all 17 manifest files; there are no mismatches.
- Snapshots use capture insertion order; per-pattern contiguous snapshot indices and publication boundaries are checked. Requiring every snapshot's pattern to equal the trusted report pattern means the last returned snapshot is the latest snapshot for that pattern.
- Report pattern/count/byte fields reject Boolean-as-integer substitutions. Source byte arrays must have exactly 32 valid bytes and match the four caller-provided source hashes; the capture verifier independently checks these against encoded identities and derives owner binding from attempt/public/factor inputs.
- Scope and false/null gates prevent promotion to solver completion, original descent, S3 custody or degree labels. Unchecked producer counters and signed-lift classifications are explicitly not exported as verified.
- Replay operates only on the final snapshot. An earlier successful row cannot replace an inconclusive or mismatching final snapshot.
- Caller-supplied source identities and an immutable input directory remain dependencies. This is not native cell admission or a trusted report-file/manifest join; the result explicitly says cell_identity_admitted=false.
- The review did not repeat the 51-test donor/new suite. The additional adversarial probe passed through the real proposed capture/reader/replay/report functions and demonstrated the head gap above.
