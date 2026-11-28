# Historical-route release build and CLI help validation

One user-authorized local attempt passed: `cargo +1.98 build --release --bin crypto --locked --offline` then `crypto dataset --help`. Cargo reported11m27s; the runner retained one20-minute deadline, two build jobs, no retries and zero solver/cloud executions. All1467 frozen inputs matched source3517ae554787 before and after; checkout HEAD was106eff003217. These exact commands establish release linkage and dataset CLI registration, not native solver or public export acceptance.

The53,654,832-byte executable SHA-256 is a49e6b0bd071091c49a05fac37ed6d715c981831bb9546d18fd3a473539bd3c1. Its local path is /var/tmp/crypto-ml-search-recovery-target-20261107/release/crypto. The complete binary and evidence are archived via S3_RECEIPT.json:15files,18,053,776 compressed bytes, freshly downloaded with byte equality and all member hashes checked. Verify the archive SHA then payload/CONTENTS.json before using restored files. The archived payload/crypto may need `chmod +x` after verification; no restored execution was performed.

The build emitted198 warnings. The retained source-only review explains the ignored revalidation Result warning; it is not an executed negative control. The newer registry/supervisor/public-export patches remain unapplied and excluded from this build. No scientific production budget or deadline was extended.
