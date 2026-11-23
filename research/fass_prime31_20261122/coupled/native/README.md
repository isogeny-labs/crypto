# Native coupled S3 correctness qualification

This directory contains two isolated Rust crates for the bounded checks in
[`../PROTOCOL.md`](../PROTOCOL.md). They include the unchanged published
repository modules at base `e68f631c492c15bb54d79b96714e56ff72cf9f9d` by path.
This is an exact-module development qualifier; it is distinct from full-repository
Cargo validation. No runtime or solver-cost conclusion is attached to these
checks. [pcc:unverified]

## Coefficient-rank qualifier

`main.rs` includes `src/cryptanalysis/semaev_prime_poly.rs`, using its `Fp`,
`Curve`, S3 evaluation, and formal Sylvester-resultant implementation. The
standalone dependency versions are `rand = 0.8.8` and `serde = 1.0.229`, pinned
to the original repository lock versions. Its direct product over the selector
roots is a correctness oracle, not a candidate performance solver.

The constructed array stores the coefficients of

\[
H(x,t)=\prod_{z^d=1}S_3(x,z,t)\pmod{x^d-1}
\]

with exactly `d` rows and `2*d+1` columns. Only powers of `x` are reduced
cyclically. Native elimination computes its row rank and the determinant of
the first `d` columns. The code also checks scalar product evaluations, small
formal Sylvester resultants including degree-drop cases, the singular
specialization's coefficient and determinant formulas, the highest two
coefficient columns, and the explicit nonsingular rank-one exception.

The frozen test grid uses both `A=B=0` as a singular algebraic control and an
nonsingular fixture:

| Field | Nonsingular coefficients | Selector orders |
| --- | --- | --- |
| `p=97` | `A=2, B=3` | `1,2,3,4,6,8,12,16` |
| `p=2147483647` | `A=847514846, B=2087444724` | `1,2,3,6,7,9,14,18,21,31,42,63` |

The second curve is the existing project fixture. The code checks
nonsingularity but does not recertify its group order. The explicit exceptional
case is `p=97, A=96, B=0, d=2`, for which it checks `H=16*t^2` and rank one.

### Frozen producer receipts

[`../evidence/attempt1/`](../evidence/attempt1/) preserves the initial build
failure: the included source module derives `serde::Serialize`, but the first
isolated manifest omitted that dependency. The original failed manifest and
lock file are retained. Adding the exact original-lock `serde` dependency fixed
that build issue without editing the included module. [pcc:unverified]

[`../evidence/attempt2/`](../evidence/attempt2/) holds compiler and source
hashes, immutable executable hashes captured before execution, separate stdout
and stderr, and exit statuses. The qualifier reported 40 cells, 2,690 scalar
product checks, 270 Sylvester checks, 269 singular coefficient columns, 20
singular determinant checks, 40 leading-coefficient checks, and one explicit
exception check. All reported success. [pcc:unverified]

The 20 nonsingular-fixture cells had row rank `d` and nonzero first minors. These
finite results do not assert full rank for every nonsingular curve. In
particular, the separately checked rank-one exception is retained. Nine native
unit tests passed: six from the unchanged included module and three specific
to this qualifier. [pcc:unverified]

The copied test and qualifier executables were each invoked with a 60-second
timeout and a virtual-memory ceiling of `2097152` KiB. Raw Cargo and test-runner
output is retained as emitted; any elapsed-time text in those tool logs is
incidental and is not a benchmark result.

### Reproduce in a fresh receipt directory

From this directory, build with:

```sh
timeout 60s cargo build --release --offline --locked
```

Before running, record `rustc --version --verbose`, the source/lock hashes, and
the built executable's SHA-256 in a new pre-call manifest. Copy the executable
to a new immutable path, then run that copy with separate stdout, stderr, and
exit-status files:

```sh
timeout 60s sh -c 'ulimit -v 2097152; exec target/release/fass-coupled-rank-qualifier'
```

The example command uses Cargo's ordinary output path; the stored producer
receipts instead name the copied immutable binary. The unit tests can be built
with `cargo test --release --offline --locked --no-run`; hash and copy the
resulting test executable before invoking it with `--test-threads=1` under the
same limits. Preserve failed calls and use new receipt paths on every replay.

## Fixed-d identity certificate

[`certificate/`](certificate/) is a separate crate so the completed qualifier
source, manifest, lock, and receipts remain unchanged. It includes the exact
published `src/cryptanalysis/identity_certificate.rs` and `src/hash/sha256.rs`.
Its dependency versions are pinned to `serde=1.0.229`, `serde_json=1.0.151`,
and `hex=0.4.3`; it uses the repository's own SHA-256 implementation.

For the single fixed order `d=6`, put

\[
\alpha=(1-T)^2,\qquad \beta=-2T(1+T),\qquad \gamma=T^2,
\]

and build the expression tree

\[
U_0=2,\quad U_1=-\beta,\quad
U_j=-\beta U_{j-1}-\alpha\gamma U_{j-2}.
\]

The certificate compares `alpha^6+gamma^6-U_6` with

\[
(T-1)^{12}+T^{12}-2T^6\sum_{j=0}^{6}\binom{12}{2j}T^j.
\]

Both sides have syntactic degree 12. The original `issue` and
`check_with_extra` APIs reported equality at 32 issuer points and 32 additional
producer replay points over `p=2^61-1`. The producer also checked that adding
one to the right side prevents issuance, and that changing the sealed value
digest is rejected. [pcc:unverified]

The emitted record is
[`../evidence/certificate-attempt1/cusp-d6.identity.json`](../evidence/certificate-attempt1/cusp-d6.identity.json),
with identity `IDC1h099b8aa03eee75f6`. It uses the repository's existing
`identity.certificate/v1` schema and has `proof: null`. Its scope is the fixed
`d=6` univariate identity over this one field. It is not a formal proof of that
identity, an all-orders identity certificate, a generic-rank proof, or a signed
independent verdict. [pcc:unverified]

From `certificate/`, build and issue a new record with:

```sh
timeout 60s cargo build --release --offline --locked
timeout 60s sh -c 'ulimit -v 2097152; exec target/release/fass-coupled-cusp-certificate issue NEW_CERTIFICATE_PATH'
```

`issue` refuses to overwrite an existing output file. To replay a saved record
with an independently chosen extra-stream seed, record a new pre-call manifest
and use:

```sh
timeout 60s sh -c 'ulimit -v 2097152; exec target/release/fass-coupled-cusp-certificate check CERTIFICATE_PATH DECIMAL_CHECKER_SEED'
```

The checker requires the record to contain exactly the fixed statement this
crate constructs, then delegates validation and evaluation to the unchanged
published checker. Keep an independent replay's receipt separate from the
producer's record; do not alter the certificate's authorship field afterward.

## Provenance and limits

The parent's protocol records the prior-work review and unavailable KB access.
The native code implements that frozen question, without importing an
uncommitted sibling solver. The generic separation-rank argument belongs to the
parent's written derivation. The finite qualifier and fixed-d certificate do
not extend their own checks to `d=1302`, other curves, or later quotient
compression. They report no end-to-end ECDLP result. [pcc:unverified]
