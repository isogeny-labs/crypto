# Higher-m prime terminal raw data

Hydrate with `python3 tools/artifacts/artifacts.py pull research/n83_candidate_pdp_20261028/prime-terminal-archives`.
Check runs.tar.gz.sha256;extract into a fresh directory ROOT. Verify:

```sh
(cd ROOT && sha256sum -c manifest.sha256)
ROOT/instrument replay ROOT ROOT/results
(cd ROOT && sha256sum -c manifest.sha256)
```

Use2GiB virtual memory and300seconds for replay. It writes no dataset files.
The deterministic GNU tar archive has sorted names,mtime2026-10-30UTC,uid/gid0,
and gzip compression. It contains42 original inputs/sources,all126 sparse
factors,exact circuits,certificates,controls,logs andterminalrecords. Read
PRIME_TERMINAL_FORMAT.json for byte layout and variable mapping. The manifest
inventories the entire raw tree. No parent dataset is needed for native replay.
Support retains failedEKS R1/R2,allsource/image/protocol versions anddevelopment
errors;later publication receipts are archived separately aftersupportsealing.
