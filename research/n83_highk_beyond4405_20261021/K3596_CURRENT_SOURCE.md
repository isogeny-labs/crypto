# K3596 root4 base on the current source

The current fixed31 comparison source reproduced the archived K3596 root4
ordered factor base exactly. Its 3,596 signed-Frobenius columns contain 596,936
distinct usable subgroup points. The current dump SHA-256 is
`1f3b323d6af8f16dbce51b4d7a3db15a27e68d3f7b72c604cdf147d6071b1473`,
the same as the committed historical pilot dump. Its BLAKE3 base hash is
`9bdadb868c18ccaad3446a62316356e3fe21ecc76512c0c27ccc1fa24d661bc1`.
An independent N83 group implementation replayed every representative,
subgroup point, signed Frobenius label and the canonical set SHA-256
`c86bd1ed9c983397bbe9641995ef99e13ec4daedea2d469b55f8836ced9cd777`.

The base-only child used source SHA-256
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`
and binary SHA-256
`07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a`.
It used the earlier public cold-factor-base fixture, a 300-second cap and
`KIC_BASE_BUILD_ONLY=1`; no root table was allocated. The local stage and
arithmetic replay returned `PROCESS_EXITED` and
`PASS_INDEPENDENT_BASE_REPLAY`, respectively. The raw current dump is at
`/Volumes/data/cryptanalysis/n83_k3596_current_source_base_20261021_1208/base_dump.jsonl`.
Its content is byte-identical to the versioned
`research/n83_root4_frontier_current_20261018/pilots/k3596.tar.gz` member
`pilot_work/base_dump.jsonl`. The stage receipt, independent replay result,
stdout and their hashes are in `k3596_current_source_base/`.

Reproduce the base-only run with:

```sh
python3 -m research.n83_highk_beyond4405_20261021.run_k3596_base_stage \
  --binary /absolute/path/to/current-source/koblitz_orbit_dlp_fast \
  --output /absolute/path/to/new-output
python3 -m research.n83_highk_beyond4405_20261021.verify_k3596_base_stage \
  --base-stage /absolute/path/to/new-output
```

This establishes the current-source factor-base identity for a K3596 root4
control. A current-source full index, rank and one-target run needs its own
resource and timing receipts before K3596 can enter the new high-K comparison.
The historical K3596 Tier 1 online measurements remain associated with their
earlier source and candidate ID.
