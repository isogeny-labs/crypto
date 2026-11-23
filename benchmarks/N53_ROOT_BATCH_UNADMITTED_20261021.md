# N53 root-batching evidence awaiting validation

The row below is preserved verbatim from PR #1740 head
`496394085d5148c6bd3eec3e32cddf126c2c8d84`. It lacks the required
`VALIDATION.json` artifact set and is excluded from the admitted benchmark
index until independent replay, headline validation and the matched reference
are supplied. The original measurements, reports and figures remain in their
existing directories; this disposition makes no new performance claim.

| date | entry | measured | headline (as reported) | host | study |
|:--|:--|:--|:--|:--|:--|
| 2026-10-12 | [`20261012-n53-s3-root-batch`](20261012-n53-s3-root-batch/) | fixed K220 N53 one-target S3 root batching, nine same-Q/rank-seed pairs and matched strong rho | Batch/control median cold 0.738 [0.714, 0.750], target-online 0.612 [0.569, 0.903]; rank inversions 763,167/80,790,012; batch/rho cold 52.586 [33.179, 64.668]; all 27 cells completed and 9 pairs replayed. Shared-host L0 diagnostics, common-S unset | Apple M4 Pro, macOS 26.6, shared L0 | [study and run reports](../research/notes/ecc2k130/n53_s3_root_batch_20261012/RESULT.md) |
