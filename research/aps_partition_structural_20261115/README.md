# APS partition structural experiments

This is bounded correctness evidence, not a performance protocol or speed
result. The user authorized native implementation and local correctness
experiments through an explicit extension of the Conductor exception.
Performance admission and all private publication restrictions remain.

## Source and standing

Executed source:
[`2089b9ce`](https://github.com/isogeny-labs/crypto/commit/2089b9cef0e1763a1c03f1e352a78347087da137).
The [native lab](../../tools/aps-partition-lab/README.md) is a separate Rust
package and does not change production solver identities or assertions.

Native statement `PCC-20261115-7ad1d5` pins the
[complete canonical report](structural-report.canonical.json), normalization
provenance and pre/post-call custody.
Native statement `PCC-20261115-d10da9` pins the
[package checks](validation-final.json). Both have **unverified** standing;
unsigned artifact attestations do not establish execution or signed proof
[pcc:unverified]. Their local ledger suffix remains unpublished pending the
existing publication/reconciliation hold. No protected ledger was rewritten.

## Questions, controls and fixed bounds

The fixed candidate family uses ID modulo arity followed by a deterministic
hash mixer. Its sizes are one, eight and thirty-two; no family was extended
after its coverage became visible. Synthetic populations contain eight or
twelve IDs and use arity three or four. No outcome-dependent sampling or
timing was used. The reference traverses cartesian color cells, separately
from the candidate's combination traversal and first-coloring ownership.
Uncovered tuples are retained explicitly in the report.

Specialization features are nonzero support and coordinate degrees of f3
with one coordinate fixed. Expansion is compared with a separate literal
formula, and every distinct-coordinate root is compared with ordinary
affine group-sign arithmetic. The coordinate pool excludes zero ordinates;
repeated coordinates and two-torsion are outside this comparison. The tiny
known-label control independently constructs aG+bQ on the public order-19
fixture, preserves canonical signs, retains zero-B incidents and checks the
scalar from every informative incident.

Negative controls include duplicated colorings, renamed colors, explicit
uncovered tuples, invalid bounds, changed labels and same-signature systems
with different roots. This tool does not simulate a successful F4 trace.
Production APS sampling and actual certified trace reuse remain untested.

## Observations and interpretation

For twelve synthetic IDs, coverage was as follows. These are represented
unordered tuples, not successful logarithms or work-normalized yields
[pcc:unverified].

| Arity | All tuples | One coloring | Eight colorings | Thirty-two colorings |
| --- | ---: | ---: | ---: | ---: |
| 3 | 220 | 64 | 191 | 220 |
| 4 | 495 | 81 | 295 | 479 |

The capped family therefore cannot be called a complete cover for the
quadruple fixture: sixteen tuples remain absent. Coverage alone says
nothing about whether the added systems pay for themselves [pcc:unverified].

On the known-label fixture, the eight-coloring triple case retained twelve
informative signed incidents and two zero-B incidents, versus six informative
incidents for one coloring. The thirty-two-coloring quadruple case retained
thirty-four informative incidents and one zero-B incident. Its eight-ID tuple
cover still omitted two tuples; those omissions happened to contain no zero
relation in this particular fixture. This does not establish completeness
on another pool, and original label guarantees must not be inherited after
recoloring [pcc:unverified].

The specialization panels over p=17,19,31 had four, one and two support
buckets respectively. Every panel retained a same-signature/different-root
counterexample. For example, on y^2=x^3+2x+2 over F17, the specializations
at c=3 and c=5 share the feature signature, but f3(0,10,3)=0 and
f3(0,10,5)=8. A signature is a scheduling hint, not permission to reuse
numerical roots or a basis without certification [pcc:unverified].

The next discriminating integration is to route real APS specializations
through these frozen features while retaining the existing numerical
certification and fallback path, and to charge family construction and all
repeated work. An admitted cost comparison, not these structural counts,
must decide whether either mechanism is useful.

## Checks, custody and limits

The complete standalone package ran ten library and two CLI cases on Rust
1.98.0 and 1.99.1, with strict all-target Clippy and formatting checks on both
toolchains. These observations apply to this package; no root-library suite,
production APS solve or performance run is claimed [pcc:unverified].

[Source hashes](source-files.sha256), [build metadata](build-manifest.json),
[pre-call manifest](pre-call-manifest.json) and
[post-call manifest](post-call-manifest.json) are distinct. The native call
used a separately copied read-only qualifier whose hash was checked before
and after execution. The manifests and report use exclusive creation.
They are producer custody records, not execution attestations
[pcc:unverified]. Raw dependency/build logs and detailed host data remain
private. No benchmark, EKS session or performance checkpoint was launched.

The literal native JSON output is 82,067 bytes and remains local pending
artifact-store upload. Git carries a 15,816-byte canonical JSON view with all
values retained, below the repository's data-size threshold. The
[post-call normalization record](normalization.json) binds both hashes and
records the value-equality check. This view is not presented as the native
output's literal bytes, and neither the original output nor pre-call manifest
was replaced [pcc:unverified].
