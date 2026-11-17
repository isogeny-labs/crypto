# N83 K4300/K4405 current-source base prefixes

The first 4,300 and 4,405 signed-Frobenius orbits of the independently
replayed current-source K5500 dump match the historical K4300 and K4405
bases exactly. The verifier reconstructs each base hash, ordered dump hash,
and point-set hash from the 913,000-point parent dump, then checks the exact
historical candidate manifests. The current-source candidate IDs are:

| Orbits | Usable points | Current-source candidate ID |
| ---: | ---: | --- |
| 4,300 | 713,800 | `IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h067ce0b0fa30` |
| 4,405 | 731,230 | `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h76ff6ddee28c` |

The manifests preserve the historical fixed five-byte, state-ID30, wide3
Bloom index policy and update the producer source commit and digests to the
current source. The identities are candidate controls for the fresh N83
cross-family bridge. Both current-source candidates remain unmeasured in
that bridge. Their prior 192-point online screen did not choose a unique
large-base winner.

Replay against the archived 71 MB parent dump with Python and `blake3==1.0.8`:

```sh
UV_CACHE_DIR=/Volumes/SSD990-2/tmp/uv-n83prefix uv run --no-project --with blake3==1.0.8 \
  python research/n83_k4300_k4405_current_source_prefix_20261025/verify_prefix.py \
  /Volumes/data/cryptanalysis/n83_k5500_current_source_base_20261021_1900/base_dump.jsonl \
  --check-receipts
```

The versioned receipts pin the parent dump SHA-256, source SHA-256, exact
prefix sizes, and all three base identities. A bridge run must still freeze
its hosts and caps, compare the same fresh public points, and independently
verify IC and rho intervals before selecting a factor base.
