# Frozen involution identity certificates

16 November 2026. Issue two `identity.certificate/v1` evaluated-tier
records for the exact polynomial numerator identities underlying the
F5 involutions in `RESULT.md`. Both statements are identities over the
integers. The first has degree three; the second has degree nineteen.
The issuer uses a verbatim snapshot of the repository's
`src/cryptanalysis/identity_certificate.rs` module in
`idc_issuer/src/identity_certificate.rs`, with a standard SHA-256 adapter.
The snapshot digest must match the tracked repository module at the
freeze commit. The issuer checks 32 statement-derived points and 32
independent extra points. The records have `proof: null`, so their status
is evaluated; the native coefficient check and direct algebra give the
exact F5 specialization separately.

Freeze the issuer, dependency lockfile and this protocol in a commit
before producing study-local certificate JSON. Retain issuer stdout,
source digest, and any failed issuance. Reproduce with

`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/ghs_idc_target cargo run --release --locked --manifest-path idc_issuer/Cargo.toml -- e2`

and the same command with `e24`.
