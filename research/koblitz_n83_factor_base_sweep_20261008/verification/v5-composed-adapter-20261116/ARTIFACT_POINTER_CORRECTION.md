# V5 admission artifact pointer correction

GitHub artifact automation at `ce4cf7581c418c7d979031a02448d8252207c95f`
moved three files from Git to the write-once artifact store. The PNG links
in [the first admission](RESULT.md), [the second admission](attempt-2/REPORT.md),
and [the full-library report](FULL_LIB_RESULT.md), and the latter's log link,
therefore no longer resolve inside a checkout. This versioned note gives
their retrieval paths without changing those frozen reports. The DOT, SVG,
PDF, and compact validation receipts remain tracked in Git.

| Original file | Tracked manifest | Bytes | SHA-256 |
|---|---|---:|---|
| `binding-flow.png` | [root manifest](.artifacts.json) | 94,524 | `9af16c6202718eede21759bf1be3abb5003efcef6b79535f80ea2cf40e838c1d` |
| `attempt-2/binding-flow.png` | [attempt-2 manifest](attempt-2/.artifacts.json) | 94,524 | `9af16c6202718eede21759bf1be3abb5003efcef6b79535f80ea2cf40e838c1d` |
| `full-lib-flow.png` | [root manifest](.artifacts.json) | 91,225 | `bf265a02bff2afce4dcc3d24ffcf3a8297dd618705f54cb251e5e8f2de6ff21c` |
| `full-lib.log` | [root manifest](.artifacts.json) | 259,693 | `66e7ffcb926ba1310749e459a241694e576fb2e20acb0eec3ce999ea1e1e0106` |

On 2026-11-16, read-only S3 `GetObject` calls through profile
`ecc-challenges-new` returned the three distinct objects with these exact
byte counts and SHA-256 values. The two binding diagrams have identical
bytes and share one store object. Retrieve each object with the store key
`s3://setec-astronomy-artifacts/store/sha256/XX/SHA256`, where `XX` is
the first two hex characters of its listed SHA-256. For example:

```sh
AWS_PROFILE=ecc-challenges-new aws s3 cp \
  s3://setec-astronomy-artifacts/store/sha256/66/66e7ffcb926ba1310749e459a241694e576fb2e20acb0eec3ce999ea1e1e0106 \
  ./full-lib.log
shasum -a 256 ./full-lib.log
```

The log readback verifies custody of a **censored** full-library run. It
does not supply a terminal test-suite verdict or a relation/total-runtime
measurement.
