# Exact replay of the recorded SEA coverage proxy

All 21 values match the previous native feature records. Start with the
[report](REPORT.md), [table](results/coverage.tsv) and [summary](results/summary.json).
The three fixed arithmetic controls and both fresh-process arms passed;
A/A outputs are byte-identical. This verifies the exact metric and supplies
counterexamples to uniform larger/smaller-bound preferences. It adds no
p-value, unusualness, SEA cost, historical-selector or vulnerability claim.

[PROTOCOL.md](PROTOCOL.md) fixes the already visible cohort and exact proxy.
[replay.cpp](replay.cpp) uses C++17/PARI. The source metric includes ramified
primes, omits 2 and ignores Atkin information. The shared log's exploratory
Python association is not a result here. The incomplete P-521 n+1
factorization and broader criterion search remain separate.

Compile with `-std=c++17 -O2 -Wall -Wextra -Werror`, the existing PARI
include/library paths, `../native` as an include path and `-lpari -lcrypto`.
The native CLI is `replay historical.tsv recorded_feature_dir fresh_output_dir`;
`replay --self-test` checks the public fixed integer fixtures. Run only after
the required S3 checkpoint gate. [run.sh](run.sh) records every native status
and can use `NIST_CHECKPOINT_WRAPPER` for an authenticated child. Set fresh
`NIST_SEA_OUT`, the compiled `NIST_SEA_BIN` and `NIST_SEA_FEATURES` explicitly
when reproducing on another host.

Hydrate the managed archive/PDF/PNG with the repository artifact tool, then
unpack `evidence/replay-001.tar.xz` here. It contains
`runs/replay-001/manifest.json`, copied exact inputs/features, source,
native binary, all output/status files and both arms. Use the copied
`source-snapshot/historical.tsv` and `input-features/` to replay without
hydrating the parent's larger raw archives. Do not reuse preserved process
IDs or the historical operational pause wrapper.

The [raw manifest](evidence/manifest.json) and
[delivery receipt](evidence/replay-001-delivery.json) verify a full S3 GET,
a byte-equal internal manifest and all 45 listed file hashes. The
[rendered receipt](evidence/rendered-delivery.json) binds the inspected
one-page PDF and PNG after full downloads. SVG and document-rendering source
are tracked; Python only plots recorded values and typesets documents.
