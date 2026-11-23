# Rejected execution-path provenance: attempt-01

Attempt-01 is retained unchanged as historical evidence. It used Python for
source-byte extraction, native build/run orchestration, and result
summarization. All field and curve arithmetic ran in Rust, but the Python
orchestration and summarization conflict with the repository's current
instruction. The parent rejected that execution path before publication.

Attempt-02 prospectively replaces the accepted path with a thin shell
runner, source/custody-only Node I/O, and direct compact Rust output. It
retains the same unchanged production selector and arithmetic. Attempt-01
has not been deleted, relabeled, overwritten, or silently substituted.

| Retained original item | SHA-256 |
|---|---|
| `PROTOCOL.md` | `56f0e45707a30d2b9ac8989fa4899fcf9f57b934fdb0efffbd544f7b06ac4166` |
| `extract_sources.py` | `38d5bddda10bc530ad660e22999b96b13b208346955f9c9e93967633706b97f6` |
| `qualify.py` | `18e3b6c9c5057fd30b64d313e9c660da7e7c1bf7ea46bfde1b600d11d0e799df` |
| `qualifier.rs` | `e208a3532b1389da162b4a6f347ee9d0450a67f0f8536a14caf2b7421c5776a3` |
| `production.rs` | `c6dff02843fceab1cf221465056598e6cd94d3d29a350b72eed85bee4c02c56c` |
| `evidence/attempt-01/qualifier.stdout` | `ca100e1479a9e9513d1e41d6648dc3fdda240209d20f0dfc9de538472c371b37` |
| `evidence/attempt-01/summary.json` | `7cf1aa03b183d3d109ba5e72bb98f63af06ecd3c67c1dbc6417f17a80bb4a8ec` |

The original raw stdout is **97,937 bytes**, above the 64 KiB log-publication
limit. It is retained locally under its original name and must not be checked
into the repository as a raw log or disguised by renaming it. Durable
object-store archival and its repository descriptor have not been completed
by this subtask; that custody obligation remains for the parent if required.
The rejected Python harnesses are not part of the accepted execution path.

The accepted run emits 3,829 bytes directly from Rust, omitting root arrays
prospectively while still evaluating every predicate on every root. This is
an output-design change made before attempt-02, not truncation of a log.

Both attempts remain same-team observations, `[pcc:unverified]`. No signed
independent validation or performance claim follows from either record.
