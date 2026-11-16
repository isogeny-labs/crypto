# Frozen actual-field construction, version 1

This archive is a known-witness construction check, not a solver run.
The pre/post records explicitly say `solver_invoked=false`.

Public input, parameters, known points, arithmetic record, identity
certificates and PCC-pinned pre/post records remain loose. Nine full systems
and native scripts (original target plus its eight raw cofactor sections)
are preserved byte-for-byte in the deterministic `systems.tar.xz`.
The original files are ignored after extraction; no frozen bytes were edited.

```sh
sha256sum -c systems.tar.xz.sha256
tar -xf systems.tar.xz
sha256sum -c systems-files.sha256
```

Repack the extracted tree with the same metadata to reproduce its bytes:

```sh
tar --sort=name --mtime='2026-11-15 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu \
  -cf - system.json input.sing prime-sections | xz -6 -T1 > /tmp/systems-repacked.tar.xz
cmp systems.tar.xz /tmp/systems-repacked.tar.xz
```

The version-one parameter record implicitly used shift=(1,0). Serializer
version two records the shift explicitly; that change did not rewrite these
artifacts. Claims PCC-20261115-02fa5a and PCC-20261115-779d94 remain producer
observations with unverified standing. [pcc:unverified]
