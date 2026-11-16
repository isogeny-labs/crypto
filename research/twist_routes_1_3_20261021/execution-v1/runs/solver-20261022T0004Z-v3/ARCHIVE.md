# Raw-case archive

The complete unmodified inputs, native bases and CAS scripts/stdout/stderr for solver-20261022T0004Z-v3 are stored in the ordered split archive. Top-level JSON remains readable. The unpacked cases/ and joined cases.tar.xz are ignored local materializations.

```sh
sha256sum -c PARTS.sha256
cat cases.tar.xz.part-* > cases.tar.xz
sha256sum cases.tar.xz
tar -xJf cases.tar.xz
```

The joined SHA-256 must equal `c4a37f3c0c5cdba2b704a0e339a3713f4289122db53aa7e3d78735b07fd98e58` in ARCHIVE.json. Every part and the joined bytes were verified locally. Run historical source/protocol versions for replay; do not infer performance from the invalid or resource-limited attempts. Tar metadata is normalized without changing member contents.
