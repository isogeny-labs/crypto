# Fixed n83 top-form regularity certificates

Run `n83-top-quotient-20261028-r1` completed 2026-10-28 05:24:08–05:25:18 UTC.
All 48 saved input systems received a certificate and fresh-process replay.
All source hashes match the preceding fixed-base FFD/LFD batch. No point base,
curve, target, generator list or input polynomial was changed.

| m | Koblitz and binary, both targets | Prime, both targets |
|---|---|---|
|2|28 ≤ dreg ≤ 55|29 ≤ dreg ≤ 55|
|3|82 ≤ dreg ≤ 165|positive-dimensional top quotient|
|4|109 ≤ dreg ≤ 275|positive-dimensional top quotient|
|5|136 ≤ dreg ≤ 385|positive-dimensional top quotient|
|6|163 ≤ dreg ≤ 495|positive-dimensional top quotient|
|7|190 ≤ dreg ≤ 605|positive-dimensional top quotient|
|8|217 ≤ dreg ≤ 715|positive-dimensional top quotient|
|9|244 ≤ dreg ≤ 825|positive-dimensional top quotient|

These are certified bounds, not exact regularity values. For prime m3–9 there
is no finite first degree where the original top-form graded quotient vanishes;
this is neither a claim of infinite Castelnuovo–Mumford regularity nor a claim
about the affine ideal's dimension. The prior timeout records remain unchanged.

The earlier four binary/Koblitz m2 systems have FFD=LFD=2. Their original top-form
regularity is now at least28. This supplies an explicit separation of those
quantities for the saved presentations, with producer replay and independent
scientific review still pending. It does not settle a universal FFD/LFD conjecture,
prove cryptographic complexity, or select an elite factor base.

## Data and file structure

- [evidence/summary.json](evidence/summary.json): all 48 intervals/statuses and
  source identities. Null bounds for positive dimension are not timeouts.
- [DESIGN.md](DESIGN.md): precise quotient-map arguments and metric convention.
- [FORMAT.json](FORMAT.json): certificate fields, zero-based monomial encoding,
  top-term hashes, failed projections, original-source archive and log formats.
- [validation/input-identity-join.json](validation/input-identity-join.json):
  hash match to every preceding FFD/LFD input.
- [experiment.export.json](experiment.export.json): validated48-case export.
- [protocol.json](protocol.json): frozen before data, SHA256
  `4337e3059205595ab6839558e65b162f68077b30c025bff3175d39203adec660`.
- [.artifacts.json](.artifacts.json): complete raw archive pointer. All60
  manifest-listed files are retained, including the executable and failed
  candidate projections. Archive:1,945,996 bytes; SHA256
  `54e1e9a4fbb249a0e016e14583ebfed69443b375fff4061785330b5db59601a6`.
- [PDF report](reports/run-20261028T0524Z/report.pdf) and
  [quantitative certificate grid](reports/run-20261028T0524Z/bounds-grid.svg).

Raw prefix:
`s3://setec-astronomy-artifacts/runs/n83-top-quotient-20261028-r1/local/`.
The complete archive was downloaded and hash-checked; its manifest matches the
sealed local run. Source snapshot at the run prefix's `frozen-source.tar.xz`,
SHA256 `5df1761c8375ce0de0bbcc3b152e5d058485507823f0bff9100ee059605286b9`.

Hydrate from repository root with:

```sh
python3 tools/artifacts/artifacts.py pull research/n83_top_quotient_20261028
```

This is a native exact-computation batch on the recorded existing-host fallback
following the EKS shared-bucket failure. No performance comparison is claimed.
Five native controls pass; all48 source/certificate replays pass; export validation
and remote archive verification pass. The unchanged root library's preceding
suite passed4966 tests with122 ignored and no failures. This standalone crate
does not alter library behavior.

## Next required evidence

The elite-base goal remains active. Candidate bases must have matched size and
held-out targets, with a fixed preprocessing policy. Preserve both original and
transformed presentations: adding valid derived equations can change the top
ideal without improving the factor base. Direct solving-degree and LFD evidence
must remain distinct from these original top-form regularity bounds.
