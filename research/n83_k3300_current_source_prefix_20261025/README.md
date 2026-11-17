# K3300 current-source base prefix

The first 3,300 signed-Frobenius orbits of the current-source K3596 base
reconstruct the historical K3300 wide3 base exactly: 547,800 usable points,
3,300 folded columns, and matching ordered-dump SHA-256, point-set SHA-256,
and BLAKE3 source-base hash. `prefix_receipt.json` records the three values
and the source digests. The verifier also seals a current-source K3300 wide3
candidate manifest by carrying over the selected algorithm flags and updating
every executed component source digest to the checked current source. Its
execution status is `UNMEASURED_CURRENT_SOURCE`; the historical timing is a
prior measurement rather than a current-source K3300 timing.

Replay against the archived 48 MB current-source K3596 dump with:

```sh
python3 \
  research/n83_k3300_current_source_prefix_20261025/verify_prefix.py \
  /Volumes/data/cryptanalysis/n83_k3596_current_source_base_20261021_1208/base_dump.jsonl \
  --check-receipt
```

The verifier requires Python `blake3==1.0.8`; any installation of that
package suffices. It checks the parent dump against the current-source K3596
candidate manifest before truncating. A current-source K3300 wide3 execution
on the frozen single-target workload is the next comparison; the base
identity alone does not select an online winner.
