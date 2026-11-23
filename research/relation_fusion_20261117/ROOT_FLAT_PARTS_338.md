# Reassemble the complete root evidence packet

The complete deterministic archive337 exceeds the repository’s10MiB blob limit. Two ordered byte shards preserve every payload; no file is excluded. Receipt338 records each shard and assembled archive SHA-256. The persistent checkpoint/S3 waiver permits local Git evidence publication. [pcc:unverified]

Run from this directory, assembling outside the checkout:

```sh
cat root-flat-recovery-337.tar.xz.part-00 \
  root-flat-recovery-337.tar.xz.part-01 > /tmp/root-flat-recovery-337.tar.xz
(cd /tmp && sha256sum -c "$OLDPWD/root-flat-recovery-337.tar.xz.sha256")
mkdir /tmp/root-flat-recovery-337
tar -xJf /tmp/root-flat-recovery-337.tar.xz -C /tmp/root-flat-recovery-337
```

Then follow ROOT_FLAT_REPLAY_336.md from its extraction step onward (the archive embeds the pre-sharding copy of that unchanged replay guide). ROOT_FLAT_EXTRACTION_340.json records a fresh2087-file byte check, complete36-proof replay and reexecution of all98 cache correctness calls. It compares every raw field except diagnostic `_ns` clocks. No timing inference, signed execution or full-root unit pass is asserted. [pcc:unverified]

Initial extraction339 completed every byte/index check, then stopped before math because two auditor Cargo files were stored under `study/`, not `source/`. Adapter340 binds those exact bytes from their indexed location. To rebuild the study crate, copy `study/Cargo.toml` and `study/Cargo.lock` into `source/research/relation_fusion_20261117/`. No archive or original pin was changed.
