# Spike38 fixed-factor-base first-fall results

Run: `spike38`, attempt 01, completed 2026-11-05. The 48 planted/natural systems across m=2..9 used the same complete factor base per curve family. The candidate x-selector domain has 2^27 (134,217,728) x-coordinates per family; the persisted point sets contain 134,217,351 Koblitz points, 134,215,721 binary points, and 134,224,756 prime points. The indexed compressed shards total 4,576,000,923 bytes across 384 files. The selector domain and point set for each family are reused for every m in 2..9.

## Outcomes

| m | Koblitz (2 cases) | Binary (2 cases) | Prime (2 cases) |
|---:|---|---|---|
| 2 | 2/2 exact first fall at degree 2 | 2/2 exact first fall at degree 2 | 2/2 certified no fall through degree 4 (lower bound 5) |
| 3 | 2/2 exact first fall at degree 2 | 2/2 exact first fall at degree 2 | 2/2 certified no fall through degree 4 (lower bound 5) |
| 4 | 2/2 exact first fall at degree 2 | 2/2 exact first fall at degree 2 | 2/2 certified no fall through degree 4 (lower bound 5) |
| 5 | 2/2 exact first fall at degree 2 | 2/2 exact first fall at degree 2 | 2/2 certified no fall through degree 4 (lower bound 5) |
| 6 | 2/2 exact first fall at degree 2 | 2/2 exact first fall at degree 2 | 2/2 certified no fall through degree 4 (lower bound 5) |
| 7 | 2/2 exact first fall at degree 2 | 2/2 exact first fall at degree 2 | 2/2 certified no fall through degree 4 (lower bound 5) |
| 8 | 2/2 exact first fall at degree 2 | 2/2 exact first fall at degree 2 | 2/2 certified no fall through degree 4 (lower bound 5) |
| 9 | 2/2 exact first fall at degree 2 | 2/2 exact first fall at degree 2 | 2/2 certified no fall through degree 4 (lower bound 5) |


Across the cohort, 32/32 Koblitz and binary systems have exact first fall at degree 2. All 16 prime systems have a certificate excluding a fall through degree 4, hence a first-fall lower bound of 5. Planted and natural cases have the same result in each family/m cell.

## Scope and replay

All 48 certificates replayed in a fresh process; all 48 S3 certificate readbacks matched their local SHA-256 and byte counts. The complete S3 attempt prefix was restored independently: 392 objects totaling 3,318,856 bytes, covering the indexed result mirror, direct per-case start/terminal/certificate records, pre-run metadata, and all 48 rolling checkpoints. The 198-file result mirror totals 992,042 bytes and its index SHA-256 is `{h(res/'S3_PERSISTENCE_INDEX.json')}`. The index and per-file meanings are in `results/FILE-LAYOUT.json`; the capsule file map is `../FILE-LAYOUT.md`.

These are first-fall certificates for the measured Boolean polynomial systems. The pinned executable does not implement the LFD closure step: `last_fall_degree`, `last_fall_lower_bound`, and closure rank remain null. This run therefore does not determine LFD or degree of regularity. Wall times are recorded for custody only and are marked invalid for comparison (`wall_valid=false`). The certificate replay is a separate process using the pinned executable; it is not an independent research review.
