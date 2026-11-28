# PR #5153: native counting and comparison correction

The original `README.md`, `l23_rescore.json`, and
`scripts/l23_h016_negation_null.py` remain byte-identical historical evidence
from commit `d1de3989b248a14a56c8d1fdf0556fa4d6de3d8e`. Their reported re-score
is not a corrected replay. Use the native entry point below for subsequent
actual-input comparisons. Source observations and development checks here
remain **[pcc:unverified]** until an independent signed verdict covers them.

## What the integration corrects

| Historical issue | Native behavior |
| --- | --- |
| The rolling pair table skips sums equal to the identity; lookups also skip identity complements and targets. | Count every indexed triple `i <= j <= k`, including identity and cancellation branches. |
| Reordering `[G,-G,2G]` can change the old count for target `2G` from zero to one. | Both orders give the same count; regression checks compare the pair table with a separate three-loop enumeration. |
| `2 * (B / 2)` silently changes a requested 51-point closed support into 50 points. | Require declared and actual point counts to agree. A closed support must satisfy its actual inverse-pair/fixed-point profile; no rounding or implicit identity insertion. |
| A negation-invariant eligibility predicate is treated as evidence that a truncated point sample is closed. | Inspect the supplied points, inverse pairs, singleton orbits, identity/two-torsion points and signed anchor membership. |
| Stored orbit representatives, represented points, target coverage and mean tuple multiplicity can be conflated. | Report each separately. Raw tuple counts never stand in for useful signed-row rank or solving cost. |

The cardinality defect alone changes the raw unordered triple supply from
`C(53,3) = 23,426` to `C(52,3) = 22,100`: 1,326 fewer slots, a derived
5.660377% reduction. This arithmetic does not correct any historical yield.

Repeated target queries are intentional observations. The historical map
accumulates their weights, so duplicates alone did not invalidate its mean.
The native API returns one count per query and retains the same target order
and repetitions; coverage uses the number of queries with a nonzero count.

For a fixed indexed support of size `B`, summing its exact triple counts over
the whole group gives `C(B+2,3)`: every triple has one group sum. Consequently
the uniform whole-group mean multiplicity is `C(B+2,3)/|G|`. Excluding identity
targets instead gives `(C(B+2,3)-D(O))/(|G|-1)`. These identities explain why
a coverage result cannot be transferred directly to a mean-multiplicity
claim. No new asymptotic or performance result is asserted here.

## Integrated code

- `src/cryptanalysis/prime_factor_base_controls.rs` provides exact sparse
  counting, realized-support profiling, signed support expansion and paired
  input admission. Arithmetic comes from the existing `prime_fast::FastCurve`.
- `FastFactorBase::{signed_support,support_profile,triple_counts}` connects
  those APIs to existing native factor bases. A two-torsion representative
  contributes one represented point. Duplicate representatives of one orbit
  are rejected.
- `examples/prime61_factor_base.rs` and
  `examples/prime_ic_collection_frontier.rs` record actual support profiles
  and make the stored-column convention explicit. Their collector and solver
  algorithms are unchanged.
- `examples/h016_factor_base_audit.rs` is the native actual-input execution
  path. It requires both actual bases and the common ordered targets, checks
  canonical coordinates and the curve equation, and checks that each finite
  point is annihilated by the declared odd prime `n`.

The shared library accepts a validated `FastCurve`; the CLI additionally
checks prime field/order parameters and nonsingularity. Annihilation by `n`
does not certify the total curve order or, when rational `n`-torsion has rank
two, membership of one chosen cyclic subgroup. This component computes point
sum counts and makes no such certificate claim.

The new counter uses a sparse point-keyed map. It does not allocate an array
indexed across the M31 field, nor does it force arbitrary random supports
through `FastFactorBase`'s small-x dense index. The optimized collector's
normalized signed rows retain their own semantics; cancellation triples in
the raw census are not extra independent rows.

## Use the native path

Build and run the public permutation fixture:

```sh
cargo build --release --locked --example h016_factor_base_audit
target/release/examples/h016_factor_base_audit \
  --input research_notes/h016_negation_null_20261124/fixtures/m31_permutation.json \
  --output /tmp/h016-native-audit.json
```

The output path must not already exist. The fixture is a deterministic
correctness input, not a random-control experiment or a historical H016 cell.
It uses the existing principal M31 curve with `p = 2^31 - 1` and compares the
same two-point support in opposite orders. Expected per-query multiplicities
are `[1,1,0,1]`; mean multiplicity and coverage are both `3/4` in this fixture.
The module and CLI tests include other inputs where those statistics differ.

Input schema: `crypto.factor-base-paired-input/v1`. Points are canonical
`{"x": ..., "y": ...}` objects; `null` denotes identity. Each arm declares
its actual point count, `explicit_points` or `negation_closed_points`, and
identity policy `allow` or `exclude`. There is one common ordered `targets`
list and one common public `anchors` list. Both bases must have matching
realized profiles and anchor signs. No implicit signs, generated targets,
or inferred support size are accepted.

The output includes exact input/source SHA-256 digests, original point and
target lists, actual profiles, per-query counts, coverage and multiplicity.
Supplied provenance remains labelled as supplied; matching a profile does not
authenticate a builder, its random distribution, or its seed record.
The tool draws no random values. Prospective randomized studies must retain
their own complete paired seed/protocol records and provide the resulting
actual inputs. Unknown rank, scalar-recovery and complete-cost fields remain
null.

## Historical scope and follow-up

The old aggregate means are insufficient to recover the original supports,
target order, identity policy and builder choices. A corrected H016 re-score
therefore remains open until those inputs are supplied. The original report's
inferences about the null shift cannot be retained as corrected evidence, and
this integration does not infer a positive or negative factor-base advantage.

No priced leaderboard row is changed. This is a native correctness and
comparison-accounting integration; there is no new timing dataset or
graphable performance result to promote. See `VALIDATION.md` for the exact
development checks and remaining gates.

## Sources checked

- [Original PR #5153](https://github.com/isogeny-labs/crypto/pull/5153), exact
  original script blob `a50cb7d34d21b9608618a0f27bc9fd28c9026f4e`.
- [G31-001 replication](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/notes/g31-001-replication-20261121/RESULTS.md),
  blob `a9929962176cd58fed040f994854fc4ab3a6ae55`: its full x-window support and
  coverage statistic are distinct from H016's truncated support and mean
  unordered-triple count.
- `src/cryptanalysis/prime_fast.rs`: Montgomery arithmetic, identity-aware
  point addition, stored representatives and native collector row conventions.
- `research/prime31_ic_20261115/native/lib.rs`: principal M31 public fixture.
- [SEC 1 v2.0, section 2.2.1](https://www.secg.org/sec1-v2.pdf): elliptic group
  identity, inverse-point and addition conventions.
- KB unavailable in this runtime; the exact repository sources above and the
  prior H016 counter/admission audit were used as the bounded fallback.
