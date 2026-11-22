# Odd-characteristic subgroup-transfer corpus

`odd-transfer-catalog` builds a deterministic, map-bound evidence corpus for
the odd-characteristic rows in the exact EC1/ICV1 cover graph. It retains every
odd map and every named EC1 binding. It does not rewrite the historical
`subgroup_transport: not_tested` fields in `cover-links.yaml`.

The supported arithmetic slice is deliberately narrow: a direct
short-Weierstrass target over a proved prime field, the checked genus-two
quadratic pullback `pi(u,v)=(u^2+c,v)`, and a named generator whose declared
annihilator is itself proved prime. For such a row the exporter checks:

- exact graph, model, field, generator, curve, cover, and map identity joins;
- generator membership and nonidentity;
- annihilation on the elliptic target and on its checked pullback;
- the typed identity `pi_* pi^*(G)=[2]G`;
- coprimality of the exact prime order with degree two; and
- recovery of `G` by multiplying the pushforward by `2^-1` modulo that order.

Prime-field rows outside the accepted deterministic primality-proof range,
exact-order subgroups whose trivial degree-two kernel intersection is not
certified by coprimality, Edwards targets without an exceptional-point-safe
projective adapter, extension-field rows without a general balanced-Jacobian backend, invalid
catalog data, and transfer-arithmetic failures have distinct statuses. An
unsupported proof range is not a failed generator, and a failed generator is
never silently downgraded to unsupported. The tracked successor corpus knows
the composed Montgomery backend, but its seven Montgomery bindings remain
outside the accepted field/order proof boundary and are therefore proof-gated,
not supported. The other fifteen normalized bindings are Edwards rows and
remain backend-unsupported.

| Record status | Meaning |
|---|---|
| `supported_on_named_generated_subgroup` | The concrete generator, exact prime order, pullback, pushforward, composition, and recovery checks passed. |
| `unsupported_without_primality_certificate` | The field or declared order is outside the accepted deterministic proof range. |
| `unsupported_kernel_intersection` | The declared prime order annihilates the nonidentity generator, but coprimality with degree two does not certify a trivial kernel intersection. |
| `unsupported_normalized_target_transfer_backend` | The target generator was preflighted, but its normalized target family has no checked projective transfer backend. In the current successor corpus this is reserved for the fifteen Edwards bindings. |
| `unsupported_odd_extension_field_transfer_backend` | The catalog-basis generator was preflighted, but general extension-field transfer is absent. |
| `failed_invalid_target_generator` | The encoded generator is malformed, noncanonical, the identity, or off the linked target. |
| `failed_invalid_named_subgroup_data` | A field/model/order binding or annihilation claim is invalid. |
| `failed_transfer_arithmetic` | A checked pullback, pushforward, composition, scalar action, or recovery operation failed. |

## Export

The exporter accepts only a binary built with a canonical embedded commit and
an embedded clean-tree flag. For a source build, set
`CRYPTO_BUILD_GIT_COMMIT=<40-lowercase-hex-commit>` and
`CRYPTO_BUILD_GIT_DIRTY=false` when compiling. The requested
`--producer-commit` must exactly match the embedded value.

From the matching repository checkout or installed archive:

```sh
odd-transfer-catalog export \
  --registry docs/curves/registry.json \
  --cover-graph docs/curves/cover-links.yaml \
  --producer-commit "$GIT_COMMIT" \
  --rustc-version "$RUSTC_VERSION" \
  --output docs/curves/odd-subgroup-transfer-certificates.json
```

The producer commit and clean flag are compile-time values checked by the
exporter. `rustc-version` remains a caller-supplied, explicitly unverified
label. The corpus records the ordered source paths, each file's byte length
and SHA-256, and a framed aggregate digest covering the exact exporter,
graph-builder, transfer, arithmetic, schema, manifest, and lockfile sources
compiled into the executable.

The corpus separately hash-binds
`ChainedTransfer.injective_with_recovery` from `ChainTransfer.lean`, its
declared Lean toolchain, and its conditional scope. That proof record is
`local_unsigned_unverified`: it states a structural theorem under explicit
homomorphism, annihilator, Bezout, and norm hypotheses; it does not construct
or identify the concrete geometric maps.

Exit code `0` means every named subgroup record in that input population is
supported. Exit code `1` means a canonical corpus was written successfully but
one or more named records remain explicitly unsupported or failed. Exit code
`3` means generation, replay, or output failed; Clap retains its own argument
error exit. Consumers must inspect the structured status and must not use
`|| true` without checking it.

## Tracked canonical corpus

The checked-in corpus is
[`odd-subgroup-transfer-certificates.json`](odd-subgroup-transfer-certificates.json).
Its raw artifact pins are:

```text
CORPUS_SHA256=1ecfabcdaecc3e00d78ab2ce7eebeb94607e1c2c224f261fb5718a4f53b65268
CORPUS_BYTES=17879292
PRODUCER_COMMIT=8c94831426fa35040fa7b38b1379299bf6e6d957
PRODUCER_SOURCE_SHA256=a5412b92e5e027b281d79c541cc34df130e16d9c71a7034f2065c6e181d8b3ef
```

It retains 22,243 odd maps and 940 named-subgroup records. The exact record
partition is 675 supported, 240 unsupported without an accepted primality
certificate, 0 unsupported kernel-intersection records, 15 unsupported
normalized-target records, 9 unsupported extension-field records, 1 failed
invalid target generator, 0 failed other named-subgroup data records, and 0
transfer-arithmetic failures. The seven reclassified normalized records are
Montgomery bindings whose composed backend is implemented but whose field or
declared order lacks an accepted proof certificate; they are not live-supported.
All fifteen remaining normalized-backend records are Edwards bindings. Of the
supported records, 542 use ordinary fibers and 133 use ramified repeated
support.

External Draft 2020-12 schema validation passed. Canonical
same-implementation replay with both raw pins returns exit `1` and
`status=verified_named_subgroup_incomplete`; repository source replay returns
exit `0` after checking 29 ordered source blobs plus the recorded Lean artifact
against the producer tree. The saved Rust compiler label remains
`caller_supplied_unverified`. The Lean record remains
`local_unsigned_unverified` and proves only its conditional abstract theorem;
neither replay supplies signed PCC standing or independent Jacobian evidence.
[pcc:unverified]

The exact local successor receipts are retained as
[`odd-transfer-catalog-canonical-replay-8c948314.json`](../../research/curve_transfer_infrastructure_20261118/odd-transfer-catalog-canonical-replay-8c948314.json)
and
[`odd-transfer-catalog-source-replay-8c948314.json`](../../research/curve_transfer_infrastructure_20261118/odd-transfer-catalog-source-replay-8c948314.json).
They are reproducibility records for the checks described above, not an
independent implementation.

The prior 17,879,352-byte pre-composition corpus at SHA-256
`7c60fc122c73abbe7b0df1413d24abbb064a9842bb35c2da8248d30c87f65670`,
producer `e0ae0fe5d09c0492a8b50600e88d0b9da68afef1`, and partition
675/233/22/9/1 remains immutable historical evidence. Its `e0ae0fe5` replay
receipts are preserved and are not the current replay target.

## Exact replay

```sh
odd-transfer-catalog verify \
  --input docs/curves/odd-subgroup-transfer-certificates.json \
  --registry docs/curves/registry.json \
  --cover-graph docs/curves/cover-links.yaml \
  --expected-sha256 "$CORPUS_SHA256" \
  --expected-bytes "$CORPUS_BYTES" \
  --output odd-subgroup-transfer-replay.json
```

Replay rebuilds the cover graph from the registry, regenerates the complete
corpus value, requires canonical pretty-JSON byte equality, and optionally
checks the raw hash and byte length. A valid replay of an incomplete corpus
again exits `1` and reports
`status=verified_named_subgroup_incomplete` with
`checks.all_named_subgroup_records_supported=false`.

This is canonical same-implementation replay. It does not by itself resolve
the recorded commit or compare source blobs with a Git tree.

## Repository source replay

When the repository containing the recorded producer commit is available:

```sh
odd-transfer-catalog verify-source \
  --input docs/curves/odd-subgroup-transfer-certificates.json \
  --repo-root . \
  --output odd-subgroup-transfer-source-replay.json
```

This command resolves the recorded commit, reads every ordered source blob and
the Lean artifact from that tree, checks the individual hashes and lengths,
and recomputes the framed source digest. It exits `0` only when those bindings
match. It does not execute Lean, provide signed PCC standing, independently
prove the curve geometry, or establish computational advantage. A standalone
archive without the recorded Git object can still perform canonical replay,
but cannot perform this repository-mode source check.

The schema is
[`odd-subgroup-transfer-certificates.schema.json`](odd-subgroup-transfer-certificates.schema.json).
Schema validity alone does not replay arithmetic, source digests, population
membership, or canonical bytes. The corpus schema does not validate the two
replay-receipt formats.

## Claim boundary

The corpus certifies only the named generated subgroups in records marked
supported. It is not catalog-wide independent Jacobian arithmetic, does not
resolve unsupported normalized or extension backends, and does not establish
an index-calculus solver, target decomposition, key recovery, scaling result,
or computational advantage. A full Jacobian relation solver remains a
separate deliverable with its own relations, linear algebra, output checks,
failure accounting, memory, and total-runtime evidence.
