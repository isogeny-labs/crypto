# Packed sessions (tier S)

The five small-field sessions (`s1a`–`s1d` at ℓ = 8209, `s2` at ℓ = 131101) hold
73,728 + 12,288 records and 420 MB raw. They are committed here as gzipped tarballs
(`tar czf <name>.tar.gz -C sessions <name>`), byte-for-byte the session directories
ecbench wrote, so that the CI session discovery (which keys on `session.json`) does
not try to re-audit 86,016 records on every pull request. Nothing inside was edited.

To restore and re-audit:

    cd research/conductor_gap_rho_large_sample_20261018
    for f in sessions_packed/*.tar.gz; do tar xzf "$f" -C sessions; done
    ./target/release/ecbench verify --dir sessions/s1a-ell8209-q29 --replay-all

The receipts in `../audits/{s1a,s1b,s1c,s1d,s2}-*.audit.json` record the sha256 of
every file in each session (`files`), the full-replay result (every run
reproduced with identical counts) and `ok: true`. `SHA256SUMS` covers the tarballs.
