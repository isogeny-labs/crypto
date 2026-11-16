# Root validation log artifact readback

At commit `6c42919c14c0`, GitHub artifact automation replaced the tracked
`lib-test-current.log.gz` with the [content pointer](.artifacts.json). On
2026-11-15 UTC, a read-only S3 HEAD under the `ecc-challenges-new` profile
returned **120,746 bytes** at:

`s3://setec-astronomy-artifacts/store/sha256/0c/0c885577d55a9f387d56dc11b0cc0838780d0de622c9e2e6d7c5dfb9fd2fa2cb`

A fresh GET to `/private/tmp/n83-root-lib-readback-c01b1b6d.log.gz` returned
120,746 bytes, SHA-256
`0c885577d55a9f387d56dc11b0cc0838780d0de622c9e2e6d7c5dfb9fd2fa2cb`,
and passed `gzip -t`. Retrieve the unchanged gzip bytes with:

```sh
AWS_PROFILE=ecc-challenges-new aws s3 cp \
  s3://setec-astronomy-artifacts/store/sha256/0c/0c885577d55a9f387d56dc11b0cc0838780d0de622c9e2e6d7c5dfb9fd2fa2cb \
  ./lib-test-current.log.gz
shasum -a 256 ./lib-test-current.log.gz
```

The tracked [root validation receipt](ROOT_VALIDATION_20261115.md), [failure names](lib-test-current-failures.txt),
and isolated-rerun log remain beside this pointer. This readback verifies
artifact custody; it does not change the failed root validation result.
