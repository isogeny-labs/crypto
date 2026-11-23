# Main result publication and local handoff

Published result commit: `99a6c9fa27b2d1c70013b123cad6eb8a468a9048`, tree
`b927d5dc7cbda7e16a7af0772c69f8c9013b2d72`, in PR #4996. Fetched-tree
readback matches all 157 published file identities and all 144 result-manifest
entries, including the ignored exporter Cargo.lock. The result/report,
original failed protocol check, full-range CSV and reviews stay unchanged.
[pcc:unverified]

Local archive: `/tmp/ecdlp-cpu-main-20261123.tar.gz`, **51,187,102 bytes**,
SHA256 `710a6873cc30129470902817339472cc8761f29ed0f350b6594d2ab8d813ee7b`.
The later fresh extraction with `tar --same-permissions` passes native SHA-256
checks for all 9,391 payload files plus its additional manifest. All 9,392 file
modes match; zero symlinks. It includes the complete main raw data, separate
preflight/synthetic/infrastructure records, qualified executables, study source
and the exact pre-run checkpoint archive. Six generated target caches remain
separately retained. S3 was not used. [pcc:unverified]

Restore into a new directory using `tar --same-permissions`; run
`sha256sum -c PAYLOAD.sha256` there. Read `RESUME.md` and the nested pre-run
input mapping before restoring anything. External tool/library bytes are
inspection material and must not automatically replace system paths. The main
supervisor marker identifies a completed single attempt; it does not authorize
rerunning or resuming that attempt.

The first default extraction passed all payload hashes but its umask removed
group permissions on eight files. Its directory and mode-failure record are
retained. A shell without fail-fast handling then continued to register
`PCC-20261123-904472` prematurely. **That statement is ineligible here as
evidence of successful mode validation.** It remains unchanged in the append-only
ledger. `PCC-20261123-a97e27` explicitly corrects it after the separate passing
fresh extraction; the archive bytes never changed. [pcc:unverified]

This handoff file, the corrective claim and readback receipts were created
after the captured archive and sit outside it. Checksums establish preserved
content and modes; they do not authenticate scientific execution. Original
protocol-bound canonical acceptance remains **failed with 324 seed-table
findings**. The narrower schema/flow/file-only check and future unadopted draft
do not change that gate. [pcc:unverified]

The report is [three pages](reports/main-20261123T1444Z/report.pdf); the
[native outcome](MAIN-OUTCOME.md) retains all 12 target comparisons and A/A
variation. [Export erratum](EXPORT-ERRATA.md) records the exact binding failure.
Signed standing, complete counted cost, the source/history proof bridge and
the original 84-case panel remain open. No experiment is running; the classical
research goal remains active.
