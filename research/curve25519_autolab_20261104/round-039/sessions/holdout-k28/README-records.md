# Holdout records, packed

`records.jsonl` (48,160,631 bytes, five arms x 1152 records) is over the repository's 10 MiB file limit, so it is committed as `records.jsonl.xz` (`xz -6 -T1`), not loose; `.gitignore` here keeps the extracted file out of Git.

Extract and re-check:

```sh
cd research/curve25519_autolab_20261104/round-039/sessions/holdout-k28
sha256sum -c records.jsonl.xz.sha256          # the archive
xz -dk records.jsonl.xz
sha256sum -c records.jsonl.sha256             # the extracted file; equals records_sha256 in session.json
```

`session.json`, `spec.json`, `plan.json` and `comparisons/` stay loose. `ecbench verify`, `ecbench compare` and `recompute/paired_ratio.py` read the extracted `records.jsonl`. Raw data of this round was not committed as loose files for this one file only; the bytes are unchanged (hash above).
