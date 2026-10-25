# Raw-session archive

`runs.tar.xz` contains the complete `sessions/` tree, including all 68 sealed
sessions, audit receipts and native wrapper logs. `session-tree.sha256` pins
every extracted file. No session file was changed during packaging.

The archive uses the study start date and normalized tar metadata, with
single-threaded xz level 9. It is 10,082,176 bytes, below the repository's
10 MiB blob threshold. This is the new raw-run-tree archive required by
AGENTS.md section 13a; source documents and analysis CSVs remain plain text.

```sh
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - sessions \
  | xz -9 -T1 > runs.tar.xz
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c session-tree.sha256
```

Readback was compared byte-for-byte with the live sealed tree. Repacking the
extracted tree with the command above reproduces the archive bytes exactly.
An initial level-6 packaging was 10,490,912 bytes (SHA-256
`2c7d3018d69bbbba75907f4c504585cb7efbfcab15f0f2108aa3531b221e5a8c`),
so level 9 was used for this raw archive. Both packagings encode the same
unchanged tree; the level-6 packaging is reproducible from the extracted files.
