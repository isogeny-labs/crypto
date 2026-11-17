# Main refresh: combined native checks

2026-10-30. Integration of main
`178c03f46d9347eb92da61a9307eff3d396e3e4f` into the finite-size branch,
whose prior uploaded head was `e2c36e34c404907821a1e013d4b4dee49f09a0c3`.
Main independently added square-root setup/reuse and three-mod-four root
verification, plus strict-JSON/reader/point work. These changes are preserved.

The original implementation receipt, 55-test log and source manifest in the
parent folder are immutable evidence of the earlier e2c36e34 snapshot. They
are not claimed to bind this refreshed native tree. This additive receipt
supplies the new source manifest and raw integration checks.

- **68 crypto native tests pass:** 44 library, 5 runner, 3 panel, 9 finite/CLI,
  3 strict-JSON and 4 width. `tests.txt.gz` retains the full transcript.
- Strict all-target clippy and Rust1.78 all-targets exit zero (`clippy.txt`,
  `msrv.txt`); the pre-existing parent-manifest check-cfg warning remains.
- Formatting and byte identity of the shared additions pass. The actual
  refreshed tree matches `native-sources.sha256`, checked from the crypto
  repository root; `source-check.txt` retains the readback.
- The separate cryptanalysis branch has **81 passing native tests** and no
  incoming native change; its two-file Lean/tooling main refresh leaves that
  validated source/lock unchanged. It is published as
  [cryptanalysis #1316](https://github.com/isogeny-labs/cryptanalysis/pull/1316).
- The only merge conflict remains the append-only shared session log. Both
  bodies were preserved. No frozen research record was selected or replaced.

Commands are the same release, strict-clippy, Rust1.78 and formatting commands
in the parent validation record. Source paths, test counts and successful
outcomes here supersede its *current integration* metadata without rewriting
its earlier evidence. `raw-transcripts.sha256` binds the uncompressed test
bytes; deterministic gzip preserves its final blank line.

The diagram and implementation contract are unchanged. The [current refresh
PDF](report.pdf) includes that diagram and current integration scope. The
full source/pin/plan/witness/partial/binary correctness controls pass on the
combined code. No profile or scientific A/B panel ran, and test durations
are not performance measurements. Item3's smaller-packet proof, changed-base
coverage, native authenticated execution/admission, both challenge scalars
and independent audit remain open.

One readback command initially ran from the repository root against a study-
relative artifact manifest and reported missing paths. Reran it from the
study directory: all original artifact bytes pass. No evidence was edited
to correct that working-directory error. Canonical backlog validation and
all required/applicable PR checks/reviews remain separate pending gates.
