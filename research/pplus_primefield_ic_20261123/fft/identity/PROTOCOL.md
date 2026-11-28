# Frozen protocol: three line-FFT algebra identities

Date: 2026-11-23. Parent: crypto PR #5049.
Standing: producer-only local qualification, [pcc:unverified].

This separate certificate-only work does not repeat the native FFT qualifier.
It uses the unchanged production `identity.certificate/v1` implementation to
record fixed finite evaluations over its field `F_(2^61-1)`.

## Statements

| Key | Equality | Ordered variables | Syntactic degree bound |
| --- | --- | --- | --- |
| butterfly_sum | `(E+X*O)+(E-X*O)=2*E` | `E,O,X` | 2 |
| butterfly_difference | `(E+X*O)-(E-X*O)=2*X*O` | `E,O,X` | 2 |
| cleared_trace_doubling | `(Z^2+1)^2-2*Z^2=Z^4+1` | `Z` | 4 |

The last identity gives `(z+z^-1)^2-2=z^2+z^-2` after division by `z^2`
when `z` is nonzero. The certificate itself records only the cleared polynomial
statement. The first two identities give butterfly inversion only after
dividing by 2 and by `2X` in a field where those denominators are nonzero.
The certificates do not establish those conditions, root counts, field splitting,
FFT correctness in the Prime31 target field, any elliptic relation algorithm,
or an asymptotic or measured speedup. Formal-proof fields remain null.

## Unchanged source and bounded dependencies

Copy the complete production identity module and SHA-256 module from the
existing native identity task, without changes:

- `identity_certificate.rs`: Git blob `114f1f4a3f2a1be027a9b046ef166259df49dd45`,
  SHA-256 `d81ee915ad769a87d7aac5d9664631239607f4a0deb27850cdb52cea84e34270`.
- `sha256.rs`: Git blob `9566e3de448e21ff98811cfc4c6992ff3448962e`,
  SHA-256 `2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf`.

The source task is
`/workspace/scratch/857f9e575d1c/work/vitse_prime31_compatibility_20261123/identity`.
Retain its Cargo.lock as an input; alter only the local package name for this
dedicated workspace. Registry package versions and checksums stay identical.
Copy only the 12 locked crate archives and their index entries from the available
cache into a task-local CARGO_HOME, verifying the archives against the lockfile
checksums before compilation. Cargo extracts fresh source from those archives;
no borrowed unpacked source directories are used. Use the existing pinned
Rust/Cargo toolchain, a new local target, and `--frozen --offline --locked`.
Do not edit the source task, its cache, or any production repository file.

## Native calls and controls

1. Freeze protocol, source, lockfile and driver hashes before building.
   Build separate issuer and checker binaries. Build timeout: 180 seconds.
2. Issue all three records with seed `20261123` and 32 evaluation points.
   For each statement, underdeclaring its degree by one must be rejected
   structurally; adding one to its right side must fail at a retained
   counterexample. The issuer creates a new certificate directory and opens
   each certificate exclusively.
3. A separate checker process verifies the exact expected statements and
   protocol fields. For each certificate, replay its 32 points and evaluate
   32 extra points for seed `20261124`, then repeat the replay with 32 extra
   points for seed `20261125`. These are 64 additional evaluations, not a claim
   that every sampled point is distinct. A right-side-plus-one mutation with
   recomputed statement ID must reject; a one-hex-digit value-digest mutation
   must produce `DigestMismatch`.
4. Issuer and checker calls have 60-second bounds. Preserve exit codes,
   raw standard output/error, certificate bytes, compiler/binary hashes and
   pre/post custody checks in an immutable attempt directory. A failed call
   remains evidence; a rerun needs a new attempt. No timings are collected.

Every expected predicate must hold. Do not promote the production API's
numeric `false_accept_log2` to a rigorous probability for fixed deterministic
seeds. Same-team issuer/checker processes are not independent PCC, a Lean proof,
or an integer polynomial-identity proof. The separate written algebra supplies
the ordinary mathematical argument; these records check its expression-tree
transcription and certificate plumbing on their declared field.
