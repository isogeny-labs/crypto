# Attempt-2 diagram artifact pointer correction

GitHub artifact automation at commit `5eff7c59a6cb506aacf86faf1dde6adb6876e15b`
removed `admission.png` from Git and added the tracked
[artifact manifest](.artifacts.json). The PNG link in the frozen
[attempt-2 report](RESULT.md) consequently points to a file no longer in
the checkout. The [source diagram](admission.dot), [SVG](admission.svg),
and PDF remain in Git. This versioned note corrects the PNG retrieval
path without changing the original report.

The manifest specifies 80,256 bytes and SHA-256
`e592ace71c0537a64114e74654e9712c4d935541662ac15ab3569fd7b09e1598`.
On 2026-11-16, a read-only S3 `HeadObject` returned 80,256 bytes, and a
fresh `GetObject` through AWS profile `ecc-challenges-new` returned bytes
with that exact SHA-256. Retrieve and verify the PNG with:

```sh
AWS_PROFILE=ecc-challenges-new aws s3 cp \
  s3://setec-astronomy-artifacts/store/sha256/e5/e592ace71c0537a64114e74654e9712c4d935541662ac15ab3569fd7b09e1598 \
  ./admission.png
test "$(wc -c < ./admission.png | tr -d ' ')" = 80256
test "$(shasum -a 256 ./admission.png | cut -d ' ' -f 1)" = \
  e592ace71c0537a64114e74654e9712c4d935541662ac15ab3569fd7b09e1598
```

This is artifact custody verification; it does not change the attempt-2
admission result or complete the Linux full-library validation gate.
