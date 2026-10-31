# Independently replayed RunPod root4 frontier

Two distinct concurrent RunPod CPU Pods measured the three exact current
source N83 factor bases on 16 frozen public points, eight per Pod. All 48 IC
and 48 same-point rho pairs, their calibration rows, factor-base points,
four-point relations and recovered scalars passed local independent replay.
Each Pod passed the Tier 1 resource and per-pair 10% calibration gates.
The authenticated provider snapshot and raw archives are bound in
[`validated.json`](validated.json); the preregistered exact 16-point selector
is in [`selection.json`](selection.json).

| Candidate | Usable points | Geometric mean online | Mean cold process |
| --- | ---: | ---: | ---: |
| K3300 wide3 | 547,800 | 8.570 ms | 356.400 s |
| K3500 root4 | 581,000 | 4.542 ms | 407.059 s |
| K3596 root4 | 596,936 | 5.902 ms | 441.269 s |

The cold selector chose K3300 wide3 among these three; its two comparisons
each have exact paired sign-flip `p=0.000030517578125`, below `0.05/3`.
The online selector is `BOUNDED_UNKNOWN`: K3500 has the smallest descriptive
geometric mean, while its paired tests against K3300 and K3596 have `p=0.1162`
and `p=0.4815`. The broader six-arm cold winner remains K1107; this three-arm
reserve does not replace that comparison. The complete online winner awaits
the wider fresh holdout and bridge.

The original raw tarball SHA-256 values are
`d06daffcbdb35d4ba3fc74c94d6eb3c4fe206d5b428f53a3673fce9f46866aca`
and `0abc06f1353ebabb79e5009cd7d9021bd8f37132a721017d3734741999ff54bd`.
`pack_complete.py` checked every original archive member against the independent
replay, then stored all 2,768 file contents with the three distinct base
dumps deduplicated. `raw_dedup.tar.gz` is 47,046,111 bytes with SHA-256
`b862dbcfaed48096d5609546deed76247fb284327c7153d016b1704dc23147bc`.
Its machine-readable file map is `raw_dedup_manifest.json`; a fresh restore
with `verify_complete.py` passed all 2,768 hashes. The prior partial archive,
on-Pod build/support receipts and first-Pod termination record remain in
[`../runpod_20261018_partial`](../runpod_20261018_partial).

Restore from the repository root with:

```sh
python3 research/n83_root4_frontier_current_20261018/results/runpod_20261018_complete/verify_complete.py /tmp/n83-root4-restored
```

The live Pods were terminated by 2026-10-19 02:28:50 UTC; the saved
`termination_state.json` records the watchdog deadline and provider IDs.
