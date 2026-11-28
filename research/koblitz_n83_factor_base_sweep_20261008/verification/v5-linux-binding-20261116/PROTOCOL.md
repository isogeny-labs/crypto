# N83 V5 retained-K600 Linux binding admission

Freeze this protocol and `run-binding.sh` before execution. This is a
**read-only input and no-work adapter check**, not a relation, rank,
target, rho or total-runtime measurement.

## Exact inputs and hypothesis

- Curve `icv1-f2m83-tm6151469093347-debefd74`, primary `a=0`, 600
  signed-Frobenius columns, `public_x_hash`, seed `2026100801`; V5
  ordinal `26960859236` with ordered first pair.
- Linux ARM64 exporter from the [successful build](../v5-linux-exporter-build-20261116/attempt-5/RESULT.md):
  source `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`, binary SHA-256
  `87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`,
  pinned image `sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`.
- Read-only AWS profile `ecc-challenges-new`, expected account
  `104186811093`, exact bucket `setec-astronomy-artifacts` and key
  `store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`.
  Require a fresh 3,025,798-byte GET with SHA-256
  `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`.
  The retained v3 manifest SHA-256 is
  `d2095084767804af1525665307c9ad1c9eb8dbee2fbdb5adcac9f849e765d119`.

The hypothesis is that the built Linux binary accepts the exact
ordinal and manifest, and rejects both an adjacent ordinal and a
changed manifest digest. An S3 HEAD response alone is not a SHA-256
check. Stop on identity, byte count, digest, source, binary, mount,
control, cgroup, timeout or cleanup failure.

## Execution and accounting

The host GET has a 90-second outer cap. The Docker admission has a
120-second outer cap, 2 GiB hard memory, zero swap, one CPU, 128 PIDs,
no network, read-only root/source/input/binary, and a writable
write-once output mount. `RAYON_NUM_THREADS=1`. Preserve AWS identity,
HEAD/GET metadata, hashes, raw outputs, negative-control exits, Docker
inspection, final receipt, and cleanup status. Record elapsed
intervals but do not use them for factor-base ranking. No S3 PUT,
relation query or solver call is authorized in this gate. The chosen
minimum total IC runtime remains unset.
