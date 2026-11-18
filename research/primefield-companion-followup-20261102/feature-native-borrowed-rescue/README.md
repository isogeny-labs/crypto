# Borrowed interval ownership

T-1881 replaces the preceding Rc engine owner with an inline owned-or-borrowed enum and propagates lifetimes through coefficient contexts. It retains the shared rescue and its arithmetic controls. See DERIVATION.md for ownership and lifetime arguments and RESULTS.md for failures, layout and preserved results. PCC-20261107-2a715d is an unsigned producer attestation. [pcc:unverified]

Replay from this directory into fresh outputs:

```sh
python3 validate-recovery.py /tmp/borrowed-rescue-replay
python3 validate-lifetime.py /tmp/borrowed-rescue-lifetime
```

The lifetime validator verifies current sources against the committed validation02 dependency hashes, compiles the actual native source with escape_probe enabled, and accepts only the intended E0515 diagnostic referencing owner. It is a compile-only ownership fixture, not a valid arithmetic execution. Normal native validation compiles without that deliberately invalid cfg branch. Each validator retains command, compiler version, stdout/stderr and receipt; native execution is limited to 60 seconds.

validation01 failed compilation with E0106 in the inherited reciprocal-control field. Its source hashes and the changed generator/control source are preserved with validation01- prefixes. validation02 uses an explicit owned 'static context in that fixture and passes. lifetime01 rejects the attempted escaping borrow as intended. Replaying the failed attempt requires a separate copy with the saved control source; do not overwrite the published successful source.

All arithmetic remains Rust; Python orchestrates and renders reports. The engine ownership constructor has no Rc allocation or reference-count operation, but vectors inside the engine still allocate. Layout sizes are compiler/target observations, not peak memory or timings. No cloud or timed workload; original 4563-case cohort unchanged. Historical full-Cargo fixture/deadline and ledger conflict remain as recorded.
