# Run 20261024T2258Z: independent factor-base / graph replication

This run executed all five frozen arms on 1,260 independently derived public
known-answer targets: 6,300 candidate solves and 6,300 paired rho solves. All
answers verified. The native validator independently recomputed every target
derivation, 12,600 source scalar checks and every aggregate price.

The complete methods, values and limits are in [the study report](../../REPORT.md),
its [PDF](../../REPORT.pdf), the [graph](../../factor-base-replication.svg), and
the [data dictionary](../../DATA_DICTIONARY.md). Raw files are listed in
`../../results/raw/.artifacts.json`; hydrate and verify them with:

```text
python3 tools/artifacts/artifacts.py pull research/factor_base_replication_20261024/results/raw
python3 tools/artifacts/artifacts.py verify research/factor_base_replication_20261024/results/raw --remote
```

The run used a shared 32-vCPU host. Process elapsed times are diagnostic and do
not establish a wall-time comparison or isolation tier.
