# Fixed-base n83 first/last-fall data

Run `n83-fixed-fall-catalog-20261028-r1`, 2026-10-28 05:00:50–05:04:37 UTC.
All 48 saved fixed-ell27 systems were processed. All 48 first-fall certificates,
32 Boolean degree-two closures and 80 fresh-process replays passed. All48 source
systems have structural catalogs. No timeout or failed replay occurred.

| Family and m | First fall | Last fall | Coverage |
|---|---|---|---|
| Koblitz/binary m2 | 2 | 2, restricted global certificate | planted and natural, four systems |
| Koblitz/binary m3–9 | 2 | unresolved; lower bound2 | planted and natural,28 systems |
| Prime m2–9 | at least5; exact value unresolved | unresolved | planted and natural,16 systems |

For each planted m2 binary/Koblitz case, V2 has53 independent linear constraints
on54 variables; both remaining assignments satisfy every original equation.
The natural m2 cases have1 in V2. Larger m cases have one independent linear
constraint in V2. The certificate argument and limitations are in [DESIGN.md](DESIGN.md).
These are exact input-specific producer certificates with fresh-process replay;
independent scientific/formal review is pending. No finite top-form regularity
was measured in the separate preceding fixed-base sweep. No elite-base or
regularity-improvement claim follows from this batch.

## Files and access

* [evidence/summary.json](evidence/summary.json): all48 cases, separate FFD,
  LFD, closure and regularity fields. Missing values remain null.
* [evidence/catalog-index.json](evidence/catalog-index.json): source hash/size,
  variable blocks, equation-index filenames, Boolean degree and term metadata.
* [FORMAT.json](FORMAT.json): binary header offsets, record widths, monomial
  column mapping, JSONL fields and original-source byte-offset conventions.
* [experiment.export.json](experiment.export.json): validated experiment export.
  Its unit is one original-input case (48), containing80 native subcomputations
  and80 replays. Every subcomputation has a separate raw status/replay receipt.
* [protocol.json](protocol.json): frozen before data; source commit
  `390f82a33ad189491bb9a5dc7a458c39e57591ac`, protocol hash
  `ee0a5693e8302d1726012065d9bb59ef8bbf21e332c3c29b0db8612bab1d346c`.
* [.artifacts.json](.artifacts.json): content-addressed raw archive pointer.
  `runs.tar.xz`:2,798,824 bytes, SHA256
  `5d196259efd5bcffc0318932b990abdaf156a24e7f6bb7ca8613c3fa0ad91469`.
* [reports/run-20261028T0500Z/report.pdf](reports/run-20261028T0500Z/report.pdf):
  descriptive report and quantitative closure grid.

Raw S3 prefix:
`s3://setec-astronomy-artifacts/runs/n83-fixed-fall-catalog-20261028-r1/local/`.
It contains every operation, immutable coefficient row, certificate, source
catalog, executable, status, replay receipt and development log. The complete
archive was downloaded again and hash-checked; its manifest matches the local
sealed run. Original systems remain in the preceding fixed-degree archive;
point shards remain in `../n83_fixed_factor_base_20261028/data/`.

Hydrate the compact archive from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/n83_fixed_fall_catalog_20261028
```

Exact frozen source archive:
`s3://setec-astronomy-artifacts/runs/n83-fixed-fall-catalog-20261028-r1/frozen-source.tar.xz`,
SHA256 `170a01ed0f86d0d69040718c7e9d36d52e318ed0fdf5d1ecffcb92b09e223770`.
The previous incomplete compression stream is separately retained under
`retained-prior-partials/` with `complete:false`, not substituted for a complete
archive. Invalid export/format drafts and their errors are also preserved.

## Remaining goal

The fixed baselines are ready for matched candidate-base design and held-out
regularity measurements. This batch does not choose an elite base or settle a
universal FFD/LFD conjecture. Preserve all three curves and m2..9, full point
manifests, candidate encodings, all failures and every old run during future work.
