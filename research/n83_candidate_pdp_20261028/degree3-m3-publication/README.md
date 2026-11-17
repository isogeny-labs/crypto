# m3 publication support

`support.tar.xz` preserves development, publication, validation and report-inspection artifacts separately from the sealed production archive. `FORMAT.json` identifies the archive and its S3 object. The internal `FORMAT.json` describes the files, and `manifest.sha256` binds every file in this snapshot. Failed attempts and rejected controls are retained.

Hydrate from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/n83_candidate_pdp_20261028/degree3-m3-publication
```

Check `support.tar.xz.sha256`, extract into a fresh directory, then run `sha256sum -c manifest.sha256` there. Production rows remain in `../degree3-m3-archives/`; this is supporting evidence, not a second production run. Final publication/checkpoint receipts are stored separately at the prefix in `FORMAT.json`.
