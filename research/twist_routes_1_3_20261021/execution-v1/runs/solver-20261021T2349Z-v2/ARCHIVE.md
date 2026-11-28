# Raw-case archive

The complete unmodified inputs, native bases and CAS scripts/stdout/stderr for solver-20261021T2349Z-v2 are stored in the ordered split archive. Top-level JSON remains readable. The unpacked cases/ and joined cases.tar.xz are ignored local materializations.

```sh
sha256sum -c PARTS.sha256
cat cases.tar.xz.part-* > cases.tar.xz
sha256sum cases.tar.xz
tar -xJf cases.tar.xz
```

The joined SHA-256 must equal `ab089b61b1151e1b3d686bcdfa721903b7fe2940e466071bcd5889a94882f7bb` in ARCHIVE.json. Every part and the joined bytes were verified locally. Run historical source/protocol versions for replay; do not infer performance from the invalid or resource-limited attempts. Tar metadata is normalized without changing member contents.
