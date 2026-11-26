# Full point-count replay output: local, not remotely archived

- File: places.tsv
- Bytes:112897
- SHA-256:8d0b9d9937889ff2901f56c07c1c5ec8c3d2dbd6859ee76a4acbabd2ce3700a0
- Rows:1310 plus header (five traces, extension degrees1–262).
- Remote status: upload attempted; blocked by missing AWS credentials.
- Intended immutable object: s3://setec-astronomy-artifacts/store/sha256/8d/8d0b9d9937889ff2901f56c07c1c5ec8c3d2dbd6859ee76a4acbabd2ce3700a0

The URI names the intended object, not an uploaded/verified artifact.
No .artifacts.json manifest is fabricated. Full raw data remains gitignored
in this execution workspace. The finite first19-degree certificate is a
mathematical certificate for the analytic tail argument, not a sharding
of the larger file to evade storage policy. The full source reproduces
all262 degrees deterministically:

```sh
g++ -std=c++17 -Wall -Wextra -Werror -O2 screen.cpp -lgmpxx -lgmp -o /tmp/ecc2k130-screen
/tmp/ecc2k130-screen places.tsv field.gp
sha256sum places.tsv
```

After an authorized AWS credential is available, the existing artifact
tool can publish and verify the full bytes. That obligation remains open.
