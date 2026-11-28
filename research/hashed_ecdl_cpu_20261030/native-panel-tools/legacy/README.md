# Historical source only

The `*.py.txt` files preserve the previous Python test, export and transfer
helpers byte-for-byte as archival text. They are retained to show the provenance
of historical validation and round18 packaging, not as execution paths. The
text suffix prevents ordinary Python test discovery from collecting them.
Do not run these snapshots, including commands in README.pre-native.md.

Use the parent directory's Rust crate for active tests and export. The frozen
native analyzer, protocols, original certificates and result records have not
been changed. Round19's one admitted attempt expired unscheduled; no new
measurement was produced.

The original filenames are obtained by removing the final `.txt`. Each file
was compared byte-for-byte against commit
`93e255e3a2795070451a09c5568588d44a5f24f5` before renaming; no historical
source content was edited. Verify custody from this directory with:

```sh
sha256sum -c source-snapshots.sha256
```

The active test implementation is `../tests/panel.rs`, and the active export
implementation is `../src/lib.rs`. Their documented Rust workflow and native
analyzer subprocess are unchanged. T-1025 owns this archival clarification in
response to the review finding on legacy/round18/test_analyze.py. That historical
file is not the correctness-test execution path.
