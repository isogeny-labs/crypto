# Current-source K992, K1024 and K3300 manifest checks

Both small-base candidates completed one full N83 IC process on a public
point already used by the earlier K992 panel. Their base dumps match the
exact candidate manifests, their root table allocations match the pinned
layouts, both relation systems reached full rank, and the recovered scalars
match the independent answer sidecar and scalar multiplication.

K3300 wide3 also completed a full process on that previously used point.
Its current-source ordered base dump and base hash match the historical
K3300 manifest, while independent replay checked all 547,800 base points,
full rank, the four-point target relation, scalar and five target phases.

| Base | Candidate ID | Full process wall | Rank | Archive SHA-256 |
| --- | --- | ---: | ---: | --- |
| K992 | `IC1N83Ckb1a1fb164672PDP4rootRCguidedLAgaussTDdirectISO0ha6a31a64b52b` | 81.741 s | 992 | `79863413acfe406585bc6e2337ae16a46e07f5b6c806633f7651d7c83d3b43ab` |
| K1024 | `IC1N83Ckb1a1fb169984PDP4rootRCguidedLAgaussTDdirectISO0h644bda8bfc36` | 66.920 s | 1,024 | `6c426a5ea2e3ade1af45801d13959458e69b7f67faae2565e406070fb6c06cec` |
| K3300 wide3 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` | 636.799 s | 3,300 | `4bfe2cbe3592fe4261abf07f4e92184c2db456ac2ebdaf4befe79a087fb6bfd3` |

The producer source SHA-256 is
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
The executed release binary SHA-256 is
`97a93d876f8e711253f84331866194472c9624260aeabc71f2460879284c6a4b`.
All three pilots use the same already measured point and local macOS host; the
fresh 16-point Tier 1 holdout in [PROTOCOL.md](PROTOCOL.md) is the timing
selection experiment.

The archives [K992](pilots/k992.tar.gz), [K1024](pilots/k1024.tar.gz) and
[K3300](pilots/k3300.tar.gz)
retain launches, exact flags, public input, the full ordered base dump,
target result, stdout, stderr, producer receipt and independent verification.
After extraction, replay each with:

```sh
python3 -m research.n83_final_extended_20261018.verify_pilot /path/to/extracted/k992 --k 992
python3 -m research.n83_final_extended_20261018.verify_pilot /path/to/extracted/k1024 --k 1024
python3 -m research.n83_final_extended_20261018.verify_mid /path/to/extracted/n83-k3300-current-pilot-20261018
```

The K992 and K1024 freshly extracted archives returned `PASS` on all 12 verification
checks. The manifest preparation script also reconstructs the already
verified current-source K1107 manifest byte for byte before writing either
new candidate manifest.

The freshly extracted K3300 archive returned `PASS` on all 14 independent
checks. Its manifest SHA-256 is
`8af182db3c194e5036dc534781b9ce80cc1c6dee2ece8b10dc31442da9593c61`.
The current-source K3300 manifest is derived from the historical ordered
base and the current K3450 wide3 policy by
[`prepare_mid_manifest.py`](prepare_mid_manifest.py).
