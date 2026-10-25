# Review packet artifact retrieval

Raw records, PDF reports, and rendered PNG graphs selected by the repository
artifact policy are stored by SHA-256. Each affected directory keeps a tracked
`.artifacts.json` manifest and an ignore rule. Markdown reports, SVG graphs,
protocols, data dictionaries, validators, and checksum lists remain in Git.
The moved bytes are unchanged; the adjacent manifest records their exact hash
and length.

From the repository root, hydrate the packet you want to inspect:

```sh
python3 tools/artifacts/artifacts.py pull research/three_leads_e2e_20261018
python3 tools/artifacts/artifacts.py pull research/five_leads_e2e_20261019
python3 tools/artifacts/artifacts.py pull research/four_followups_20261021
python3 tools/artifacts/artifacts.py pull research/four_followups_review_20261022
python3 tools/artifacts/artifacts.py pull research/projective_law_followup_20261022
python3 tools/artifacts/artifacts.py pull research/kummer_differential_collector_20261022
python3 tools/artifacts/artifacts.py pull research/pencil_sparse_20261022
python3 tools/artifacts/artifacts.py pull research/factor_base_replication_20261024
python3 tools/artifacts/artifacts.py pull research/factor_base_heldout_20261025
python3 tools/artifacts/artifacts.py pull research/factor_base_heldout_replay_20261025
python3 tools/artifacts/artifacts.py pull research/factor_base_heldout_wall_20261025
```

The corresponding benchmark mirrors can be hydrated by passing their
`benchmarks/<study>` directory to the same `pull` command. A single file may
also be named instead of a directory. After hydration, run
`python3 tools/artifacts/artifacts.py verify <same-path> --remote`; the study's
`SHA256SUMS` checks the original bytes at their original relative paths.

The [four-lead status](NEXT_STEPS.md) and each study `REPORT.md` identify
which raw table, graph, and PDF support a stated result. Pulling a packet does
not rerun an experiment or alter its price rows. See
[artifact-store policy](../../docs/artifacts.md) for the manifest format and
transport requirements.
