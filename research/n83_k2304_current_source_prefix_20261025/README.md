# K2304 current-source base prefix

The first 2,304 signed-Frobenius orbits of the independently replayed
current-source K3596 dump reconstruct the historical K2304 base exactly:
382,464 distinct subgroup-usable points and 2,304 folded columns. The
ordered dump SHA-256, point-set SHA-256, and source-base BLAKE3 hash all
match the historical candidate manifest. `prefix_receipt.json` binds these
values to the parent dump and the current source. The source-updated candidate
ID is
`IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h06c953ec903b`.

This identity keeps K2304 available as a fixed contender in the fresh
cross-family bridge. Its earlier online times were close to K3300 on the
two individual cloud strata, while the earlier combined comparison used
different Rust compiler versions. The current-source K2304 candidate has no
timing row under the bridge protocol yet.

Replay the prefix and candidate manifest against the archived 46 MB parent
dump using Python with `blake3==1.0.8`:

```sh
python3 research/n83_k2304_current_source_prefix_20261025/verify_prefix.py \
  /Volumes/data/cryptanalysis/n83_k3596_current_source_base_20261021_1208/base_dump.jsonl \
  --check-receipt
```

The candidate preserves the historical K2304 root11 dynamic index policy
and updates only the executed source commit and source digests. A future
bridge run must compare it on the same fresh public points, hosts, resource
envelope, and independently verified IC/rho intervals as the selected
nearby and wide arms.
