# R91 bidirectional rank-32 RR transport

The single frozen R91 attempt checked all 32 normalized R50 RR generators
through each rational diamond direction. Its exact native gate passed 64
column identities and 64 corrupted-coordinate controls. A fresh S3
readback supplied the independent, ideal-free F2(u) replay, which checked
both full matrices, all 64 columns and 2,048 polynomial module
coordinates. The chart shows the first pole-clearing power for each
generator: both directions have the same 32-value sequence, ranging
from 44 to 61 with one generator at 61.

Source revision `7ff259920`, seed 91, one Singular/FLINT thread, 4 GiB
virtual memory and 1200-second cap. The 18 raw files passed fresh-readback
hash verification. The first two replay probes in F2[u] failed because
the exported power vectors contain rational coefficients; their logs and
the corrected F2(u) replay are retained. The Lean bound proof covers the
integer inequalities under explicit assumptions, while the CAS checks
establish the matrix identities.

This completes localized module transport on the registered diamond
charts. The next geometric task is the actual boundary normalization and
proper map, followed by a nonzero action on the modular factor. The
matching forward/reverse pole sequences warrant a separate automorphism
check. Evidence: `../CM.md`, `../results.json`, `../REPLAY-ERRATUM.md`,
`../replay-cert91.log`, `power-histogram.csv`, and the raw archive receipt.
