# Note on Amendment 1 (written after the runs; not part of the pre-run hash)

Two things `AMENDMENT-1.md` did not say.

1. **`run.sh` also changed, in the same edit, before panels B-E were run.** The registered `run.sh` (hash in `REGISTRATION.sha256`) ran all panels. The committed `run.sh` adds a `PANELS` switch (default `BCDE`, so panel A is not rerun and not overwritten) and runs the slowest cell, planted `s = 8`, last. The panel cells, draw counts, `cmax` values and seeds are unchanged. The committed `run.sh` is the one panels B-E ran with; its hash is in `REGISTRATION-AMENDMENT-1.sha256`'s companion line below, recorded after the fact.
2. **`REGISTRATION.sha256` therefore reports `FAILED` for `digit_fall.rs` and `run.sh`.** That is expected: both were amended, and the amendment records why. `PROTOCOL.md` is unchanged and its hash still verifies. The original source of `digit_fall.rs` is not kept; the diff is the `plant(...)` function described in `AMENDMENT-1.md`, and panel A's `s = 3` cell was reproduced line for line by the amended binary.
