# Restore the tested dataset validation source

The standalone validation commit is
`1d858fb9e4b8035b38f93db4423880627b09211f`. It contains the exact 3,963-file
source closure from snapshot 09. The package records the original base commit,
overlay digest, file hashes, modes, and complete path list. This commit identifies
the source used for local CLI validation; upstream integration remains a
separate delivery step.

The package is stored at:

```
s3://setec-astronomy-artifacts/store/sha256/39/39cf38c008b681cafd4a3c2a38d84c64455c6d3bb05f55e0ae749093fd060d8f
```

Archive SHA-256:
`39cf38c008b681cafd4a3c2a38d84c64455c6d3bb05f55e0ae749093fd060d8f`
(36,590,013 bytes). A fresh S3 GET matched the original archive byte for byte;
see `clean-source-package-01-readback.json`. Source preparation and bundle
verification are recorded in `clean-source-package-01.receipt.json`.

Use a new directory and the configured artifact-store credentials:

```bash
mkdir dataset-source-restore
cd dataset-source-restore
aws s3 cp s3://setec-astronomy-artifacts/store/sha256/39/39cf38c008b681cafd4a3c2a38d84c64455c6d3bb05f55e0ae749093fd060d8f package.tar.gz
printf '%s  %s\n' 39cf38c008b681cafd4a3c2a38d84c64455c6d3bb05f55e0ae749093fd060d8f package.tar.gz | sha256sum -c -
tar -xzf package.tar.gz
sha256sum -c package.sha256
git init source
git -C source bundle verify ../source.bundle
git -C source fetch ../source.bundle HEAD
git -C source checkout --detach FETCH_HEAD
test "$(git -C source rev-parse HEAD)" = 1d858fb9e4b8035b38f93db4423880627b09211f
(cd source; sha256sum -c ../source.sha256)
git -C source ls-files -z | sort -z | cmp final-paths.bin -
test -z "$(git -C source status --porcelain --untracked-files=all)"
```

The original preparation checked POSIX file modes too. On a restored Unix
checkout, check `source-modes.tsv` before using it as a build source:

```bash
while IFS=$'\t' read -r path expected; do
  test -f "source/$path" && test ! -L "source/$path" || exit 1
  test "$(stat -c %a -- "source/$path")" = "$expected" || exit 1
done < source-modes.tsv
```

The frozen build command compiles `crypto` and `dataset_startup_cli_fixture`
with Rust 1.98 and embeds this actual Git commit. The guarded build helper is
`run-clean-build.sh`; its contract is `clean-build-01-contract.md`. Check the
build's terminal receipt and retained executable hashes before using its binary.
Source-package success alone does not establish a successful build or CLI run.

The next executable validation is the bounded native CLI acceptance workflow,
after its actual test, lint, schema, and runtime-preflight receipts are retained.
That workflow checks fixed local software fixtures and copied negative controls;
it does not consume a cloud generation budget.
