# Fixed hash-domain vectors

These literal design vectors are not generated dataset cells or complete partial snapshots. They have synthetic digest values and no authoritative context files. Only their exact bytes, lengths and SHA256 were calculated while writing this design; no consumer, producer, test executable, solver or hardware collector was run.

| File | Bytes | SHA256 | Meaning |
| --- | ---: | --- | --- |
| binding-canonical.bin | 619 | cf6eb83dd75443e2077e3fac761650963f380209f683b70e565b9e6fb76f8b67 | Fixed-order compact AttemptBinding JSON, NO trailing LF |
| capture-genesis.bin | 102 | ecaa553fa5789d49cb42686de0cc59ec8db26a5af6d76140b5ac89a61abf564a | Domain line, binding digest line, LF after each |
| event-1.jsonl | 273 | c38840a0a8188d791f3d2689d20ab452b8577092f860ebd1e08139c79d83bfca | Sequence1 checkpoint record INCLUDING trailing LF |
| event-2.jsonl | 444 | 9f86063dc854838c04e5a58d7b3eecd31da8d55be614374602e2c4f8f67c23a4 | Sequence2 payload-commit record, predecessor event1, INCLUDING LF |
| empty-payload.bin | 0 | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | The zero-length payload named in event2 |

Expected genesis cursor:

```json
{"byte_end":0,"event_count":0,"last_event_sha256":"ecaa553fa5789d49cb42686de0cc59ec8db26a5af6d76140b5ac89a61abf564a","next_chunk":0,"payload_count":0,"payload_bytes":0}
```

For a first segment `events/000000000000.jsonl` containing exactly event1, expected range is `[0,273)`, first/last sequence1, preceding genesis digest and last event1 digest. Expected cursor after this segment:

```json
{"byte_end":273,"event_count":1,"last_event_sha256":"c38840a0a8188d791f3d2689d20ab452b8577092f860ebd1e08139c79d83bfca","next_chunk":1,"payload_count":0,"payload_bytes":0}
```

For a second segment `events/000000000001.jsonl` containing exactly event2, range is `[273,717)`, first/last sequence2, preceding event1 digest and last event2 digest. The delta must additionally contain the exact artifact `payloads/empty.bin`, role `test-payload`, encoding `raw`, length0 and empty-file digest. Expected cursor:

```json
{"byte_end":717,"event_count":2,"last_event_sha256":"9f86063dc854838c04e5a58d7b3eecd31da8d55be614374602e2c4f8f67c23a4","next_chunk":2,"payload_count":1,"payload_bytes":0}
```

The same two lines concatenated into one segment would have `next_chunk=1`, not2; event/byte/payload totals stay the same. Segment count is a distinct committed field. The implementation acceptance fixture must independently hash that concatenation if it tests this alternative.

Required negative uses: sorting binding keys produces the wrong canonical binding digest; adding an LF to binding changes its digest; stripping the event LF is invalid even if outer object hashes are refreshed; whitespace inside a valid event changes the event-chain digest; sequence2 without its same-delta empty payload artifact rejects; event2 with genesis as its predecessor rejects. Correctly rehashing an intentionally changed valid raw event chain can remain structurally valid and must not be confused with an authenticity check.

These vectors complement, rather than replace, the complete context/lineage fixtures and rehashed negative matrix in ACCEPTANCE.md. Frozen protocol canonicalization continues to use the existing checked-consumer vectors and is not redefined here.
