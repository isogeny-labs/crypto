# Result publication and verified local handoff

Result commit `f85fa363268175173df6b780afbad40686126b61` contains the outcome,
report, controls and compact records. Review
[08](reviews/COHORT-PUBLICATION-REVIEW-08.json) retains its concrete missing-lockfile
finding: one of the 386 result-manifest paths was absent from that commit.
The [correction](LOCKFILE-PUBLICATION-ADDENDUM.md) at
`89c42acb6ec5d3169351e4eab537576f2122732f` adds the exact Cargo.lock and disclosure
only. Review [09](reviews/COHORT-PUBLICATION-REVIEW-09.json) checks all 386
unchanged manifest entries and all 419 scoped Git objects against that corrected
commit, with no new discrepancy. Original 08 and all earlier reviews remain
unchanged. [pcc:unverified]

The local capsule `/tmp/ecdlp-cpu-cohort-20261123.tar.gz` is **22,620,814 bytes**,
SHA256 `427298202c50b757c1a65fe7bddbbbe792570b98776c35f1e0c4a1fe1b069202`.
Fresh native tar extraction and GNU sha256sum verify all **1,335 payload files**,
plus the separately checked payload manifest. A separate metadata check confirms
all regular-file permission bits and zero symlinks. Payload-manifest SHA256 is
`df5d135ba197bc250cb2e35d771559502dd61eba1e3c5a8d96b5916901e64d4b`.
See [HANDOFF.json](HANDOFF.json). These are archival checks, separate from scientific
execution attestation. [pcc:unverified]

The capsule includes the corrected 419-file source/record snapshot, all 24 full
certificates, four export versions, frozen/copied binaries, raw phase/audit and
failure records, the 108-input pre-run checkpoint, reviews 05–09 and a resume
note. Generated Cargo target cache is excluded and remains intact at the
original local root; the installed Rust/Cargo toolchain/cache is separate.
Earlier baseline/precision/history capsules and external cache archives remain.

The capsule binds source commit 89c42acb. This later publication addendum and
handoff receipt are outside the capsule, avoiding a circular archive hash.
No solver, EC audit or mathematical proof was rerun for packaging.

Full exports/certificates and qualified binaries remain local under the user's
no-S3 instruction. The remote PR supplies compact records, exact sources,
reviews and receipts, with the full remote replay payload explicitly absent.
Unsigned `PCC-20261123-e8d860` remains unverified; this publication promotes no
signed execution standing or performance leaderboard entry. [pcc:unverified]

The broad Git archive attempt stalled on partial-clone lazy retrieval and its
owned process group was stopped with exit 143. After terminal stop, the scoped
snapshot checked local content against the fetched Git blob identities,
materialized those exact objects locally and read them back, then checked SHA256.
The 09 reviewer separately checked the actual committed objects. Original data
was unaffected. A redundant copy of already frozen read-only review09 files was
refused; their existing staged copies compared equal, so no replacement occurred.
[pcc:unverified]

The result/PR prose and new journal paragraphs pass their scoped lint checks.
An initial full-journal lint included inherited findings; existing journal
records were preserved. Full-root/MSRV/CI and signed independent execution
remain distinct open checks. Canonical performance charts stay unchanged for
this L0 descriptive recovery cohort. The lane remains active for a prospective,
source-reconciled matched candidate/baseline comparison and the separate original
84-case/source-history/total-cost obligations. [pcc:unverified]

Primary implementation documentation consulted for the point audit:
[OpenSSL 3.5 EC_POINT operations](https://docs.openssl.org/3.5/man3/EC_POINT_add/).
The frozen primary Rust analyzer remains the source of the run plot.
