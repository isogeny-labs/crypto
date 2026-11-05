# Source-equivalent registry binding, 2026-11-05

Successor to `../registry-binding-20261101/`, after restoration of 1,918
existing native catalog registrations (20,848 → 22,766 registry rows).
The new current registry SHA-256 is
`6dcbd903602eca7a6973136e86bff1f1e3a595a0ffa3dedc6374d4f1ed0a5ef3`.
These are producer observations with unverified standing. [pcc:unverified]

The existing Rust checker retains every assertion and input hash. Only its
receipt date/predecessor literals and formatting differ. The original
registry at `ddaba2819f4ee96139c4145fbed905db1c1db46e` remains pinned to
`53fe929b9b971fa2c74ee29fcd5cea7fca969d411ae28e8cbf413c9550f3032e`,
and the original report to
`505327cf69bf6b6554244ee1214c71c87ee1331ba73dc74e1c36ebaa6388ee57`.
The complete unique target record is equal; all 211 other raw pins match
(210 file-byte hashes and one existing artifact-manifest hash). Every
inherited scientific field remains unchanged. [pcc:unverified]

`validator-report.json` and `source-equivalence.json` are actual checker
outputs, built using Rust 1.99.1, serde_json 1.0.151 and sha2 0.10.9.
The invocation takes six arguments: original registry, current registry,
original `../validator-report.json`, the parent benchmark directory, new
validator-report path and new source-equivalence path. [pcc:unverified]

The mutable `../VALIDATION.json` pointer now names this successor. Its
previous complete bytes are retained as `previous-VALIDATION.json`; both
historical registry-binding directories, original reports, raw sessions,
protocols, replay certificate, rho reference, measurements, plots and PDFs
are unchanged. No benchmark, scalar-recovery, timing, storage operation or
new scientific experiment ran. A manifest pin is not a new remote-byte
custody check. Actual current-head validation CI remains required. [pcc:unverified]
