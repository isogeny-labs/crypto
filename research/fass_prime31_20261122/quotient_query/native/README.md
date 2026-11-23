# Native quotient query component

This standalone development qualifier reuses the unchanged production
`semaev_prime_poly.rs` field and polynomial operations, the published
`target_quotient/native/oracle.rs` direct norm helper, and production SHA-256.
It is not a whole-library integration build or a timing benchmark.

## Files and interfaces

- `kernel.rs`: dynamic-evaluation extended Euclid in `x` over
  `Fp[t]/G(t)`. `solve` accepts canonical monic split squarefree `F` and
  `G`, and canonical `H mod (F,G)`. A nonunit coefficient splits the
  current factor of `G`; no complete root inventory is passed to `solve`.
  Current supported fields are 97 and 2147483647 and modulus degrees are
  capped at 256. Inputs outside this domain are refused.
- `norm.rs`: the established division-free quadratic-selector Lucas
  recurrence from `src/cryptanalysis/semaev_selector.rs`, evaluated
  directly in the tensor quotient. States remain explicit coefficient
  vectors. The declared panel does not run this construction on five
  degree-63 cases; those cases use the earlier dense direct-product input.
- `independent.rs`: a separate-role implementation of field/polynomial
  arithmetic, certificate identities, splitness/partition checks,
  complete-grid comparison, original-S3 input binding and tamper controls.
  It does not call the producer's arithmetic or gcd implementation.
- `checker.rs`: a separate-role reader for retained certificates and their
  input metadata, with an independent fixed-fixture registry and input-binding
  rejection controls. These paths do not call the producer's gcd search or norm.
- `main.rs`: the fixed 95-case development panel, source-bound input
  construction, selected certificate persistence and compact diagnostics.

The public certificate stores low-first polynomials in `t` as the
coefficients of a low-first polynomial in `x`. Every leaf carries a
modulus `q(t)`, a monic `D(x,t)`, cofactors `F1,H1`, and Bezout factors
`A,B`. The independent verifier checks

    F = D F1,  H = D H1,  A F + B H = D  (mod q).

The leaf moduli are monic and pairwise coprime with product `G`.
`D=1` certifies an empty block. Positive-degree `D` describes an exact
triangular algebraic support block. Its root isolation and rational
signed-point extraction are separate operations. No signed relation is
claimed by this qualifier.

## Frozen invocation

Use the source snapshot and manifests specified by the parent protocol.
The executable requires a new output directory and the frozen protocol hash:

```sh
fass-quotient-query-qualifier NEW_OUTPUT_DIRECTORY PROTOCOL_SHA256
```

The parent records compiler/build/source hashes and imposes the execution
envelope. The executable refuses to replace output directories or certificate
files. It emits complete certificates only for the predeclared 27 cases:
all seven synthetic controls, eight ordinary F97 degree-2 cases, six
rank-one F97 degree-2 cases, and six Prime31 degree-7 cases. It checks
all 95 certificates inline and records their exact serialized SHA-256
and byte count; the other complete certificates are reconstructed by
replaying the frozen source rather than persisted as individual files.
For each persisted case, `input_sha256` and `input_bytes` bind the exact
serialized input sidecar written to disk. Both fields are explicitly null
for nonpersisted cases. The persisted subset also receives the independent
tamper panel. [pcc:PCC-20261123-df2e5e] [pcc:unverified]

Every persisted certificate is required to fit the 64-KiB data-artifact
threshold. An excess is an explicit retained failure requiring artifact-store
publication; the predetermined subset is not changed after observing sizes.

The selected files can be checked without rerunning the query solver:

```sh
fass-quotient-query-qualifier --check CERTIFICATE_DIRECTORY RAW_JSONL
```

The optional third argument supplies the retained compact case records for
SHA-256 and byte-count binding of both certificates and input sidecars.
When omitted, the receipt explicitly records that no aggregate-record binding
was supplied. In either mode, the checker requires the fixed 27-case set and
uses its independently specified fixture registry to select S3 or synthetic
verification. It checks the sidecar's kind, curve, target and selector policy
against that registry; named synthetic cases must carry their exact declared
polynomial inputs. Input sidecars retain the complete original curve,
selector and root inventories for independent S3 binding. The stdout header
defines the compact keys and deterministic file names; `persist:false` means
`full_certificate_persisted=false`.

The independent rejection controls can be run against the retained directory:

```sh
fass-quotient-query-qualifier --check-controls CERTIFICATE_DIRECTORY RAW_JSONL
```

This mode tests altered fixture metadata and case identities without invoking
the query solver or norm constructor. Its complete fixed control panel requires
the raw-record argument and exercises the fixture guards both with and without
aggregate-record binding, as well as certificate and sidecar byte bindings.

For selector targets, the additional `cancel` diagnostic counts support
coordinates satisfying `x=X` or `S3(x,X,t)=0` and the remaining coordinates.
It does not classify signed relation rows. `degree_mass` groups terminal
modulus degrees by outer gcd degree. `degree1_t` records only the roots of
linear terminal moduli whose outer gcd also has degree one. These are
descriptive values, not acceptance assertions about the cancellation
prediction.

The diagnostics count algebraic support cells, dynamic splits, Euclidean
steps and stored coefficient slots. They are not a complete field-operation
ledger or a performance ranking. Empty query certificates concern only the
specified affine algebraic support. Identity-prefix branches and signed
point checks remain obligations of the consuming relation extractor.
