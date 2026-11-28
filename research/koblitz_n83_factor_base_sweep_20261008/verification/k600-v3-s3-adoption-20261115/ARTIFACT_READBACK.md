# Archived validation log readback

GitHub artifact automation at `796d7af89404` moved `lib-test.log.gz` from this directory. The tracked [artifact pointer](.artifacts.json) binds 120,555 bytes to SHA-256 `f1cde7941dd6d5864e221a89a20d27cc04b1637fd06ae92d743dfbb41787a981`. A read-only S3 HEAD and fresh GET on 2026-11-15 returned that exact length and SHA-256 under AWS profile `ecc-challenges-new` (account `104186811093`). Retrieve the unmodified gzip bytes with:

```sh
AWS_PROFILE=ecc-challenges-new aws s3 cp \
  s3://setec-astronomy-artifacts/store/sha256/f1/f1cde7941dd6d5864e221a89a20d27cc04b1637fd06ae92d743dfbb41787a981 \
  ./lib-test.log.gz
shasum -a 256 ./lib-test.log.gz
```

Expected SHA-256: `f1cde7941dd6d5864e221a89a20d27cc04b1637fd06ae92d743dfbb41787a981`. The sorted [failure names](lib-failures.txt), [exit receipt](lib-test.exit), and report remain tracked in Git. This readback is artifact custody, not an independent root-test rerun.
