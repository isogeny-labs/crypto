# Current-source K992 and K1024 manifest checks

Both small-base candidates completed one full N83 IC process on a public
point already used by the earlier K992 panel. Their base dumps match the
exact candidate manifests, their root table allocations match the pinned
layouts, both relation systems reached full rank, and the recovered scalars
match the independent answer sidecar and scalar multiplication.

| Base | Candidate ID | Full process wall | Rank | Archive SHA-256 |
| --- | --- | ---: | ---: | --- |
| K992 | `IC1N83Ckb1a1fb164672PDP4rootRCguidedLAgaussTDdirectISO0ha6a31a64b52b` | 81.741 s | 992 | `79863413acfe406585bc6e2337ae16a46e07f5b6c806633f7651d7c83d3b43ab` |
| K1024 | `IC1N83Ckb1a1fb169984PDP4rootRCguidedLAgaussTDdirectISO0h644bda8bfc36` | 66.920 s | 1,024 | `6c426a5ea2e3ade1af45801d13959458e69b7f67faae2565e406070fb6c06cec` |

The producer source SHA-256 is
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
The executed release binary SHA-256 is
`97a93d876f8e711253f84331866194472c9624260aeabc71f2460879284c6a4b`.
Both pilots use the same already measured point and local macOS host; the
fresh 16-point Tier 1 holdout in [PROTOCOL.md](PROTOCOL.md) is the timing
selection experiment.

The archives [K992](pilots/k992.tar.gz) and [K1024](pilots/k1024.tar.gz)
retain launches, exact flags, public input, the full ordered base dump,
target result, stdout, stderr, producer receipt and independent verification.
After extraction, replay each with:

```sh
python3 -m research.n83_final_extended_20261018.verify_pilot /path/to/extracted/k992 --k 992
python3 -m research.n83_final_extended_20261018.verify_pilot /path/to/extracted/k1024 --k 1024
```

Both freshly extracted archives returned `PASS` on all 12 verification
checks. The manifest preparation script also reconstructs the already
verified current-source K1107 manifest byte for byte before writing either
new candidate manifest.
