# Native binary line FFT on the full M31 Dickson fibre

Date: 2026-11-23. Parent: isogeny-labs/crypto PR #5049.

## Result and scope

The bounded native qualifier passed at `p=2147483647` and depths `k=1,...,8`.
It checked 32 forward transforms against independent monomial Horner
evaluation, 32 inverse roundtrips and 32 quotient products against schoolbook
polynomial multiplication plus monic remainder. The forward comparisons covered
2,040 field values. Both deliberately wrong variants were detected. Compile and
run exit codes were zero; source and binary custody checks passed. These are
producer-only correctness observations, without independent signed PCC standing.
[pcc:unverified]

The implemented operation is arithmetic in the full unfiltered selector algebra
`A_k = F_p[X]/(D_(2^k)(X,1))`. It is a small specialization of the established
Circle FFT, not an ECDLP solver. No timings, useful-relation counts, linear-algebra
ranks or logarithm recoveries were measured. No production library file was
changed and no full-root library build was performed. [pcc:unverified]

The separate [identity record](identity/README.md) adds three production-schema
certificates for butterfly sum, butterfly difference and denominator-cleared
trace doubling. Their issuer and checker passed all prescribed controls. These
are fixed finite evaluations over `F_(2^61-1)` with null formal-proof fields;
they do not certify the Prime31 root count or FFT executable. [pcc:unverified]

## Reproducibility

Run a new immutable attempt from this directory:

```sh
PPLUS_RUSTC=/path/to/rustc bash qualify.sh attempt-02
```

The toolchain used for attempt-01 was `rustc 1.100.0 (0ef1bc386 2026-11-09)`.
The source is standard-library-only. The driver requires GNU `timeout`,
`sha256sum`, `cut` and a POSIX-like shell. It refuses to overwrite a frozen
attempt. The attempt files retain all commands, outputs, exit codes and hashes.
[pcc:unverified]

| Artifact | SHA-256 |
| --- | --- |
| Frozen `PROTOCOL.md` | `e1a6e6a2870c36a74db1971e6230a1fdfbea806967d27debe5a4bbd1e7f4da31` |
| Frozen `line_fft.rs` | `79cafe01cf405d4de2ad58085379228dfe5ee94b5182e54a0b5b322fa162096d` |
| Unchanged production extraction | `c6dff02843fceab1cf221465056598e6cd94d3d29a350b72eed85bee4c02c56c` |
| Native binary | `f25beae959a07872200554c45e9e5b7db20f70b6530ae6581c5f29a74e82795d` |
| Native output | `189448210ea8bee9bed754d84909aa6f1408ae1496ca17ad935ea6ede1de67f9` |

These hashes are custody identifiers, not proof of execution or independent
correctness verification. [pcc:unverified]

## Production reuse and independent reference

`production.rs` is copied byte-for-byte from the sibling native qualifier, whose
manifest binds selected unchanged ranges of `ic_boundary.rs` and
`ic_framework/pkm_semaev.rs` at revision
`bd20b95dce44a91d7f74a1367853e724f280a3b6`. It supplies the actual project
`dickson_abscissae`, modular multiplication and modular addition. The isolated
extraction excludes unrelated imports and wraps unchanged method ranges;
`production-source-manifest.json` records those wrappers explicitly.
[pcc:unverified]

The reference side uses separate `u128` modular arithmetic, expands the binary
basis directly into monomials, then applies Horner or schoolbook multiplication
and long division. It does not use the transform to produce its expected output.
Both implementations are written or assembled within this producer session;
the different arithmetic and polynomial representation make the cross-check
useful but do not turn it into signed independent validation. [pcc:unverified]

## Cost interpretation

With inputs already in the binary hierarchical basis and precomputed twiddles,
evaluation is a binary butterfly computation with `B/2 * log2(B)` field
multiplications and `B * log2(B)` additions. Its output has exactly `B` values.
The normalized inverse in this small implementation uses two multiplications
per butterfly; the established scaled inverse can defer normalization. Thus both
directions, and quotient multiplication through two evaluations, pointwise
multiplication and interpolation, have `O(B log B)` arithmetic cost.
This is a derived operation bound for this primitive, not a measured speedup.
[pcc:unverified]

The transform itself has `O(B)` live field-element storage. For independent
checking, the qualifier stores all `B` expanded basis polynomials, with
`B(B+1)/2` monomial coefficient slots, and uses a schoolbook product whose degree
is at most 510. The frozen protocol's statement about its largest polynomial
refers to individual univariate degree, not to total reference storage. No
multivariate `B^m` relation table is constructed. [pcc:unverified]

## What this enables and what is still missing

This supplies a concrete candidate primitive for dense operations on an
unfiltered Dickson leaf quotient: evaluation, interpolation and quotient
products. One can pad rational-curve masks and chosen y-values to the full
domain, retaining the transform, but their coefficient vectors can be dense.
The smaller filtered set and the elliptic pair-sum spectrum need not inherit
the uniform two-to-one tower. For `m` independent leaves the full tensor
algebra still has dimension `B^m`; applying this transform in every axis does
not reduce that dimension. [pcc:unverified]

The next bounded implementation decision is where the existing relation solver
actually performs sufficiently dense quotient arithmetic to benefit from this
primitive. It needs an equal-output component protocol against the existing
sparse `TowerRing` operations and a pinned upstream Circle-FFT implementation,
with setup and representation conversions charged. That comparison was not run
here. The separate four-summand extraction and signed useful-rank obligations
remain with their existing lanes. [pcc:unverified]

## Task log

### What was done

Read primary Circle-FFT mathematics and official Rust source; searched the three
research repositories; derived the precise scaled x-coordinate correspondence;
implemented and ran the frozen bounded native qualifier. All expected
predicates passed on attempt-01; no failed attempt was discarded.
[pcc:unverified]

### Decisions made

Use the full unfiltered fibre and the existing production selector. Keep the
transform in its native hierarchical basis. Make no relation-solving or
novelty claim, and preserve existing torus-cover and pair-spectrum scope limits.
[pcc:unverified]

### Open threads

Independent review and signed validation, optimization or production integration,
external equal-output comparison, and any ECDLP relation-query application.
[pcc:unverified]

### Errors

No compiler or arithmetic failure occurred. Primary ePrint 2024/278 downloads
were blocked by the web tool; the full primary S-two Whitepaper and official
source files supplied the needed transform definition. The whitepaper's
extracted Eq. (17) displays a `+1` sign inconsistent with its Eq. (11); this
implementation uses the independently derived squaring map `2a^2-1`, or
`X^2-2` for `X=2a`. [pcc:unverified]
