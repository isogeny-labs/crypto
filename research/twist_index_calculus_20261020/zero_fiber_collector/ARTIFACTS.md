# Raw evidence and reproduction

The completed `panel-v2` contains all 42 polynomial collectors and separate
auditors. Native summary: `results/panel-v2/native-summary.json`; its full
SHA-256 is in `native-summary.sha256`. The actual run starts at
2026-10-24T20:03:47Z and terminates successfully at 20:39:43Z. Counts are
correctness/counter diagnostics; elapsed values are not admitted timing.

## Complete raw archive: verified and published

The canonical candidate is 105,608,944 bytes, SHA-256
`86a7f68e331a24c4bb1048aab3932ce9b30901c7f64def2cae1f617f93ce8bb8`.
Its canonical name is
`store:evidence/twist-index-calculus/20261024-zero-fiber-runs.tar.xz`.
The [publication receipt](results/publication/store-publication-v2.json)
reports `pass`: native store verification and the remote whole-object size
and SHA-256 checksum match; every unrelated index entry is unchanged. The
[integrity receipt](results/archive-verification-v2.json) records a fresh
extraction, all 1,112 native file comparisons and a byte-identical repack.
That earlier integrity receipt retains its pre-publication pending field;
publication is certified by the separate successful v2 receipt.

`pack-verify.sh` selects these ten terminal raw directories explicitly:

- `native-completion-20261024T1934Z`
- `native-gate-v2`, `native-gate-v3`, `native-gate-v4`, `native-gate-v5`
- `native-gate-v6`, `native-gate-v7`
- `native-gate-journal-20261024T1940Z`
- `panel-v1` (stopped before any native collector; nested-reservation receipts retained)
- `panel-v2` (all 42 collectors and audits complete)

The archive also includes the exact 38-file compiled-source snapshot rebuilt
from committed producer `57cde8bee873b43d50cf2dc4e1e74a6962604f1a`, the two
mixed oracle reference files and original summary for each of 21 cases, and
the unchanged independent point-replay summary. Source checks must pass.
Inside manifests bind all payload bytes. Fresh extraction, every native
file comparison and byte-identical deterministic repacking are mandatory.
The first repack failed because umask changed extracted permission modes.
That failure and its successful permission-preserving repair remain in
`results/pack-v1/` and `results/pack-permission-repair-v1/`. A later check
found epoch-zero tar timestamps, so `pack-with-study-date.sh` reused the
verified payload with the study date and normalized modes required by
AGENTS.md section 13a. The first local archive is retained at SHA-256
`8418084d7e25f6076f69e17d46a20a3ede3f67df73699eb4c7fc4b0df0ef74bd`;
it was not uploaded. No collector or old oracle was repeated.

Pull with `ecbench store pull
evidence/twist-index-calculus/20261024-zero-fiber-runs.tar.xz --out
runs.tar.xz` (one shell command). Then check and extract into a fresh directory:

```sh
sha256sum -c runs.tar.xz.sha256
mkdir extracted
tar -xJpf runs.tar.xz -C extracted
(
  cd extracted
  sha256sum -c SHA256SUMS
)
```

Deterministic repacking uses exactly:

```sh
tar --format=gnu --sort=name --mtime='2026-10-24 00:00:00Z' \
  --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' \
  --hard-dereference -C extracted -cf - . | xz -1 -T1 > repacked.tar.xz
cmp runs.tar.xz repacked.tar.xz
```

Run the repack from outside `extracted`; it must contain only the archive's
extracted members. The public input slices and prior reference sets are
included; a scalar/point census is not needed to check recorded bytes.

## Native test fixture dependency

Six older image-crate unit tests load the original frozen NDJSON at runtime.
The new 13-key public inputs do not replace that legacy fixture. On a sparse
checkout, materialize the tracked file from the producer commit:

```sh
git restore --source=57cde8bee873b43d50cf2dc4e1e74a6962604f1a \
  --worktree --ignore-skip-worktree-bits -- \
  research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson
```

Its SHA-256 is
`d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c`
and its size is 3,009,728 bytes. It is already tracked and is not duplicated
in this new archive. Reproducing the complete 84-test suite therefore needs
the source checkout and that fixture. The public-only collector does not
read it; archived candidate inputs remain the sealed 13-key JSON files.

## Existing dependencies remain unchanged

- `store:evidence/twist-index-calculus/20261023-image-base-runs.tar.xz`:
  SHA-256 `1e0a2139c9a32571a9476e4b9579f73da91e93e4a8cc7c2143d2bf4536c36038`;
  28,241,048 bytes; complete 189+21 controls and admitted old cold rounds.
- `store:evidence/twist-index-calculus/20261024-joint-image-runs.tar.xz`:
  SHA-256 `54b31b3d73602d7758dad263939bdfdcbd8c305df2299fe7beb498029090b729`;
  1,153,392,456 bytes; all 43 joint-study raw directories and independent
  scalar/point censuses. No BSGS or those censuses were repeated.

Reports and benchmark copies are rendered only from the recorded native
summary. Both PDFs, four pages total, were actually inspected before
committing; exact hashes and byte-identical benchmark-copy checks are in
`results/pdf-inspection-v1.json`.
Benchmark copies remain unlisted: target descent, a complete price and
VALIDATION/rho/GAE obligations are still open.
