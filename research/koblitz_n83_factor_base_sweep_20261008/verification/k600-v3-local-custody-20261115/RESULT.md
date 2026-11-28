# K600 v3 local-store construction result — 2026-11-15 UTC

**Scope:** one primary `a=0` K600 factor-base construction and local
content-addressed custody control under the committed [protocol](PROTOCOL.md).
This is a construction/custody result, not an IC relation or total-runtime
comparison. The producer source was clean commit
`498efdc53e821edb24fcebf0826ff94549ce266b`; the protocol was committed
at that same revision before this attempt.

| Check | Frozen acceptance | Observed | Status |
|---|---|---|---|
| Producer and independent importer | exit 0; v3 receipts `PASS` | both exit files `0`; manifest completed; replay, storage, import and independent import `PASS` | verified local control |
| Primary subgroup and signed orbits | 99,600 points; 600 columns | 99,600 points and 600 representatives replayed; independent import checked 99,600 points and 600 columns | verified local control |
| Exact-set control | historical K600 point-set BLAKE3 `ac664f5f…ee21` | `ac664f5f0c7122c52a08c0a288b543efb709111a99dc2a5cb2bef025ffffee21` | matches |
| Object integrity | producer, fresh pull, separate import and direct file hash agree | 3,025,798 bytes; SHA-256 `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf` throughout | matches |
| Historical compressed-object identity | comparison only, not required for v3 acceptance | historical K600 protocol records the same SHA-256 and 3,025,798-byte length | matching hash and length |
| S3 publication | separate authorization required | temporary `file://` remote only; tracked `store/index.json` unchanged | pending |
| Natural relations and cold IC cost | separate frozen searches | no relation query, rank, logs, descent or matched rho in this control | not measured |

## Execution and raw evidence

The clean macOS arm64 Darwin 27.2.0 host had 9,723,520 KiB free on the run
filesystem at admission and one-minute load 10.54. The touched release
example passed 26/26 before launch ([raw log](example-test.log)). The
`attempt.json` file was created at 2026-11-15T12:09:43Z and the producer
exit file was written at 12:09:46Z. Those are filesystem timestamps, not
independent stopwatch boundaries. The separate importer exit file was
written at 12:10:37Z. Both commands used Rust 1.99.1 and the same source
checkout. The producer had a 600-second internal construction watchdog and
a 900-second outer `gtimeout`; the independent check had a 300-second outer
cap. Neither cap fired.

The producer invocation set `CRYPTO_STORE_REMOTE` to
`file:///Volumes/SSD990-2/tmp/n83-k600-v3-local-custody-20261115/remote`,
`CRYPTO_STORE_INDEX` to the sibling temporary `index.json`, and
`CRYPTO_STORE_CACHE` to the sibling temporary `cache`. It ran:

```text
gtimeout -s TERM 900s cargo run --release --example koblitz_n83_factor_base_export --locked --offline -- primary-store-v3-construct-publish <absolute attempt-1> public_x_hash 600 2026100801 600 file:///Volumes/SSD990-2/tmp/n83-k600-v3-local-custody-20261115/remote
```

The independent command used a new empty `independent-cache` and ran
`primary-store-v3-check <absolute attempt-1> independent-import.json` under
`gtimeout -s TERM 300s`. The exact command arguments, source and remote are
bound by [attempt](attempt-1/attempt.json),
[manifest](attempt-1/manifest.json), [local index](local-store-index.json),
[replay](attempt-1/replay.json), [storage](attempt-1/storage.json),
[first import](attempt-1/import.json), and
[independent import](independent-import.json). Raw process records are
[producer stdout](producer.stdout.log), [producer stderr](producer.stderr.log),
[producer exit](producer.exit.txt), [import stdout](independent.stdout.log),
[import stderr](independent.stderr.log), and
[import exit](independent.exit.txt). The retained
[compressed object](attempt-1/objects/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf.jsonl.gz)
is durable in this PR; the temporary remote and cache are not the durable
source of truth.

## L0 timing and claim boundary

The one producer receipt reports construction **1,102.783 ms**, generic
replay **753.807 ms**, local file-store publish **36.091 ms**, fresh pull
**22.700 ms**, and total in-process wall **1,963.477 ms**. These are
single-run, shared-host L0 diagnostics. They are not isolated, do not include
a relation search, and are not ratios to an IC or rho baseline. The compiler
emitted an inherited `rust-objcopy`/`libLLVM.dylib` stripping warning while
the command still exited 0; the raw stderr is retained.

The committed object is a candidate input for a later S3 publication and
full-cost trial. The user must specify the exact astronomy S3 destination
and authorized writer before a new remote write. The separately frozen K600
m6 positive/natural search still needs its own host, input and budget gates.
`selected_best_total_runtime` remains null. No canonical runtime graph or
leaderboard entry changes from this construction control.
