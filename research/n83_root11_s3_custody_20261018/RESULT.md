# N83 root11 factor-base S3 custody

**Result: `PASS_TWO_FRESH_S3_ROUNDTRIPS`.** The two exact, point-defined
factor-base dumps from [provider run 39061060775](https://github.com/aburan28/crypto/actions/runs/39061060775)
are now stored under the requested S3 prefix. The [frozen plan](plan.json)
binds provider source commit `9b9a0f3842de6cf921e38573cb99fcaed2a4683c`,
four artifact IDs and ZIP hashes, both candidate IDs and source-manifest
hashes, raw base hashes, and the destination. Its SHA-256 is
`1b434bd6682167e9b034fd2191da7bddc9a878c575eb9d5ea7b76bd70131819b`.

| Role | Orbit columns | Points | Raw SHA-256 | Compressed BLAKE3 object key | Compressed bytes |
| --- | ---: | ---: | --- | --- | ---: |
| K2816/root11 | 2,816 | 467,456 | `e3b9a30d57ee61a31ae5b80d585ba6aee55475654062ad8dda17693a38adc5cb` | `a4a048d9f26051ed4de64f0684bfdaf1471b2548467eb990bbbd2c1c1f690b93.jsonl.gz` | 11,166,723 |
| K3072/root11 | 3,072 | 509,952 | `a58965693e46b7af7e8de7852649c101d51f70854473459c99b5e8a8c6692874` | `17b4ab1d1c5dbcd5a673deecd3d31bf987bd74d207d194c2518a6d8c5ad696f8.jsonl.gz` | 12,182,011 |

The complete object addresses are recorded in [build.json](receipts/build.json)
and [audit.json](receipts/audit.json). For each arm, the builder checked the
SHA-256 of all four provider ZIPs, all 32 repeated byte copies of the same
base dump, and the decoded point array. It used deterministic gzip and
content-addressed each compressed object by BLAKE3. Both successful PUTs
used `If-None-Match: *`, so an existing object could not be overwritten.
Fresh GETs matched compressed SHA-256 and BLAKE3,
decompressed raw SHA-256, sizes, ETags, content type, and S3 metadata. The
small [PUT/GET receipts](receipts/) preserve the returned S3 fields.
The [fresh prefix inventory](receipts/inventory.json) found exactly these
two keys with the expected sizes and ETags.

The [mutation control](receipts/mutation-control.json) flipped one byte near
the end of a copy of the fresh K2816 GET. The independent verifier exited
nonzero with `ValueError: compressed S3 object differs: k2816root11` and
produced no passing audit. The unmodified downloads replayed successfully.

These are bases for the degree-83 **a=1 diagnostic curve with a 53-bit
subgroup**. The primary K600 SAT sweep concerns an a=0 N83 subgroup, and
cross-curve runtime comparisons are invalid. This custody experiment did not
measure relation yield or total IC runtime. It leaves
`selected_best_total_runtime: null`; a complete cold and online comparison
with matched rho, rank, linear algebra, individual-log work, and verification
is still needed for selection.

Validation of this evidence change: `py_compile` passed for the builder and
verifier under Python 3.13 with [pinned `blake3==1.0.8`](requirements.txt);
the full N83 Python discovery suite passed 63/63 tests, and the previously
touched Rust worker example passed 6/6. The full serial
`cargo test --release --lib -- --test-threads=1` passed **4,085 tests, with
117 ignored and zero failed**. Its exact raw output, SHA-256, command,
environment, and 3,882.17-second test wall are retained in the
[serial-release receipt and log](validation/serial-release.json).

The validation history also retains two earlier resource-limited attempts.
The unrestricted local run reported 4,072 passed, 117 ignored, and 13 failed:
four long `jv_cover` cases reached bounded-work limits, and nine
`pollard_collab` socket tests returned `PermissionDenied` in the sandbox.
Their exact names are in [initial-release.json](validation/initial-release.json).
With four test threads and socket access, 4,084 passed, 117 were ignored, and
only the replayed-F4 case failed after reaching 16 of its required 30 usable
cases. Its [complete raw log](validation/bounded-release.json) is retained.
That case then [passed alone in 69.24 seconds](validation/replayed-f4-isolated.json)
and passed again inside the full serial suite. No Rust source changed in this
custody round; the Python, example, and mutation logs are also in
[validation](validation/).
