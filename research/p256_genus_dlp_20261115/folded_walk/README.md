# P-256 mixed-image folded walk

The destination solver now has independent increments, exact representative
coefficient updates, distinguished points and bounded short-cycle escapes.
Public toy solves and actual-field traces are producer checks only.
PCC-20261115-30dcbf is unsigned. [pcc:unverified]

- [Report and requirements](REPORT.md), [PDF](REPORT.pdf), [diagram](walk.svg).
- [Frozen correctness scope and RunPod amendment](PROTOCOL.md).
- [Raw toy cases](results/toy.json), [actual-field trace](results/trace.json),
  [producer checks](results/local-checks.txt), [validation limits](VALIDATION.md).
- [RunPod job and access dependency](runpod/README.md).
- [Prior mixed-image construction](../transfer_search/REPORT.md).

```sh
bash research/p256_genus_dlp_20261115/folded_walk/check.sh
```

No cloud allocation, timed comparison or net-speedup result exists here.
[pcc:unverified]
