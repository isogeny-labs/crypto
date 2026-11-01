# Validation and gate status

Native source freeze: `a0cd24e1c7f4381e9ce59cfce760eb5eb8cea3c5`.
Unchanged library baseline: `a679b78eea8687d9f3bbe04bb42b166b512cfeae`.
The native source files and lockfile are pinned in
[source-files.sha256](source-files.sha256); run its check from the repository
root. The final branch adds reports, workflow and problem metadata after
that source freeze, without changing its native arithmetic.

| Command/check | Outcome |
| --- | --- |
| `cargo test --release --locked --manifest-path research/quotient_calibration_20261025/native/Cargo.toml` | 4 passed, 0 failed. Includes full frozen source replay, altered-reference rejection, dependent-frame rejection and small-field inverse/order controls. |
| `cargo clippy --release --locked --manifest-path research/quotient_calibration_20261025/native/Cargo.toml --all-targets -- -D warnings` | Pass after four initial style diagnostics were fixed. |
| `cargo fmt --manifest-path research/quotient_calibration_20261025/native/Cargo.toml -- --check` | Pass. Initial tool absence was resolved by installing rustfmt. |
| Native command-line replay | Exit 0; recorded in native-preflight.json. A second fresh command-line output matches it byte for byte, as the focused CI workflow requires. |
| `bash research/quotient_calibration_20261025/verify-upstream.sh NEW_DIRECTORY` | Exit 0; four fresh downloads match all four historical hashes. Downloaded bytes also match the GitHub connector fetches. |
| `actionlint .github/workflows/quotient-calibration-preflight.yml` | Pass with Actionlint 1.7.7. |
| `bash -n research/quotient_calibration_20261025/verify-upstream.sh` | Pass. |
| `python3 -m unittest tools/artifacts/test_artifacts.py` | 12 metadata/tool tests pass; no Python research arithmetic is executed. |
| Report PDF | Typst 0.15.0; one page, 54,485 bytes. Rendered with pdftoppm and visually inspected; text also inspected with pdftotext. |
| Local report links | 23 local Markdown links resolve. |
| `cargo test --release --lib` | **Not green:** 4,634 passed, 2 failed, 119 ignored; exit 101. See the exact failures below. |

## Full-library failures and follow-up

1. `cryptanalysis::ecbench::bounds::tests::committed_version_1_bounds_re_derive_under_their_recorded_version`
   initially lacked the tracked
   `research/ecbench_bounds_challenges_20261006/sessions/prime-toy-reference-e1/session.json`
   outside the sparse profile. Materializing that study and rerunning the
   exact test passes: 1 passed, 0 failed, 4,754 filtered out. This focused
   pass does not rewrite the original full-suite result.
2. `cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity`
   fails at `compare.rs:2433` on
   `zero.legacy_comparison_id.is_some()`. An exact focused rerun also exits
   101. The implementation retains a legacy ID only for `weight.is_none()`,
   while the test expects it for `Some(0.0)`. The intended contract is open.
   See [the named problem](../../../../problems/ecbench-weighted-zero-legacy-id-contract.md).

The original root attempts also could not resolve the tracked
`isogeny_algos` path package outside the sparse profile. That directory was
materialized before the completed full-suite run. The library's final long
Barrett parameter test completed successfully; no root test was skipped,
disabled or interrupted to shorten the gate.

`git diff a679b78e -- src Cargo.toml Cargo.lock tests isogeny_algos tools/volcano-cli`
is empty. This PR does not repair or conceal the independent baseline failure.
The passing native replay must not be described as a green whole-library
suite or a completed target experiment. Hosted final-head CI/review is a
separate gate recorded on the PR.

## Raw validation records

The new raw log tree was archived locally as `runs.tar.xz`, following
AGENTS.md section 13a. It is 67,540 bytes, exceeding the 64 KiB data-file
threshold. It is **not committed or published**. The attempted artifact-store
upload was rejected by automatic approval review: internal research/test
logs require explicit authorization for this payload and S3 destination.
No workaround or second upload attempt was made. The environment also has
no configured AWS identity or installed AWS CLI. The
[publication record](ARCHIVE_PUBLICATION.json),
[archive hash](runs.tar.xz.sha256), contents list and
[named blocker](../../../../problems/quotient-calibration-validation-archive-publication.md)
make that missing custody explicit. The raw archive and extracted logs are
retained in this execution workspace, not a durable external location.

After authorized publication and retrieval, check the bytes in place:

```sh
cd research/quotient_calibration_20261025/reports/run-20261025T1845Z
sha256sum --check runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Members include `runs/root-full-library.log`,
`runs/baseline-weighted-comparison.log`,
`runs/bounds-after-materialization.log`, all native test/lint/replay logs,
the initial failed setup attempts, the metadata test log and hash checks.
No claim/verifier resolves these logs as loose repository files; the small
structured native receipt and source hash manifest stay loose.

Repack deterministically with:

```sh
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/quotient-calibration-runs.tar.xz
cmp /tmp/quotient-calibration-runs.tar.xz runs.tar.xz
```

Conductor was not installed/configured here; the attempted check returned
`command not found`, the repository's documented fail-open setup case.
No Conductor reservation or independent-host target verification is claimed.
