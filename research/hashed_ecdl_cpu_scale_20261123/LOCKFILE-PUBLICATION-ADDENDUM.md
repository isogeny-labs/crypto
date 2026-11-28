# Standalone adapter lockfile publication correction

Read-only review of result commit
`f85fa363268175173df6b780afbad40686126b61` found that
`SHA256SUMS-results-1` lists 386 paths but only 385 exist in that commit.
The missing path is `independent/export-lossless/Cargo.lock`, which existed
locally and was used by the recorded locked final build. Root Git ignore rules
excluded this new lockfile from the publication selection. [pcc:unverified]

The corrective successor adds its exact bytes. The 386-entry result manifest,
all prior exports, sources, reports, measurements and reviews remain unchanged.
No build, solver or point audit is rerun for this publication-only correction.

The original working-tree checksum check established local consistency but did
not prove every manifest path existed in Git. The successor check must read
every listed path from the actual Git commit and compare its SHA256. Review 08
retains the original commit's gap; any acceptance of the correction is a new
publication addendum. [pcc:unverified]
