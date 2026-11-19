# Hyperelliptic cover infrastructure

`hyperelliptic-cover` is a bounded, certificate-oriented command-line tool.
It constructs and verifies the repository's two explicit cover families and
runs checked Jacobian arithmetic only when the infinity configuration matches
an implemented representation. It is not an index-calculus or key-recovery
solver. [pcc:unverified]

The additive catalog/evidence schema is
[`hyperelliptic-infrastructure.schema.json`](hyperelliptic-infrastructure.schema.json).
That schema records EC1/ICV1 identity, source curves, Jacobian contexts,
morphisms, induced maps, evidence, and unsupported cases. It is not the CLI
request schema. The CLI wire contract is specified below.

## Standalone invocation and release status

After unpacking a release artifact that contains this revision, invoke the
executable directly:

```text
./hyperelliptic-cover --help
```

A local release build has the equivalent path:

```text
./target/release/hyperelliptic-cover --help
```

Users of a packaged artifact do not need `cargo run`. The seven command shapes
are:

```text
./hyperelliptic-cover construct --input MODEL.json [--output RESPONSE.json]
./hyperelliptic-cover verify --input VERIFY.json [--output RESPONSE.json]
./hyperelliptic-cover arithmetic --input OPERATION.json [--output RESPONSE.json]
./hyperelliptic-cover transfer --input TRANSFER.json [--output RESPONSE.json]
./hyperelliptic-cover catalog-export --registry REGISTRY.json [--output covers.json]
./hyperelliptic-cover transfer-catalog-export --registry REGISTRY.json \
  --cover-graph cover-links.yaml --traits traits.json \
  --producer-commit COMMIT --rustc-version VERSION \
  [--output binary-transfer-certificates.json]
./hyperelliptic-cover transfer-catalog-verify --input CORPUS.json \
  --registry REGISTRY.json --cover-graph cover-links.yaml --traits traits.json \
  [--output RESPONSE.json]
```

The source capabilities documented here are active-branch work. The release
workflow is configured to package `hyperelliptic-cover` and
`curve-transfer-sweep` and to smoke-test transfer, malformed-input, and bounded
sweep requests. Workflow configuration is not package evidence. A historical
installed-layout replay exists for a retention-bound artifact built from
crypto revision `74752a1e`; it is neither stable publication nor a package of
the current implementation revision. Do not attribute current commands to an
older published archive. A fresh current-head artifact digest, stable
publication, installed-layout replay, and direct replay of the complete live
registry remain open release obligations.

At the 2026-11-18 source snapshot, the regenerated geometry graph covers all
22,417 registry models with 22,417 maps and 22,415 distinct cover models. Its
1,090 EC1 links are identity records, not subgroup certificates. The dated
15,791-map transfer assessment remains bound to its historical catalog commit;
it is not silently relabelled as live-package evidence.

For the first four commands, `--input -` reads standard input. Omitting
`--output` writes JSON to standard output. Successful `catalog-export` writes
the existing deterministic `curve-covers/v1` catalog shape; it does not
silently claim enriched Jacobian or transfer evidence. `--registry` is always
required because the executable does not embed a registry. The repository's
release workflow stages `docs/curves/registry.json`, `covers.json`, and
`cover-links.yaml` beside the executables. Callers may use that exact bundled
registry after a new package is published or supply another explicit path.
Ordinary command, registry, and traits inputs are capped at 32 MiB; the live
cover graph accepted by `transfer-catalog-export` is capped at 96 MiB. A
larger file or standard-input stream returns `invalid_input` rather than being
partially parsed.

Command response status and process exit status are distinct and stable:

| JSON `status` | Exit | Meaning |
|---|---:|---|
| `ok` | 0 | Requested bounded operation completed |
| `unsupported` | 2 | Well-formed request is outside the implemented mathematical boundary |
| `invalid_input` | 3 | Malformed JSON, model, certificate, divisor, or operation arguments |
| `error` | 1 | Internal certificate replay or output failure, including a failed stdout write |

Every enveloped response has:

```json
{
  "schema": "hyperelliptic-infrastructure-command/v1",
  "command": "construct | verify | arithmetic | transfer",
  "status": "ok | unsupported | invalid_input | error",
  "result": {}
}
```

The strings separated by `|` above document alternatives; an actual response
contains one string.

## Construct and verify

`construct` accepts a supported elliptic-model JSON shape. It does not look
up a slug or consult a registry. A registry's `model_json` value is one way to
obtain such an object; the surrounding registry row is not accepted. Unknown
model fields are rejected. For example, the model represented by
`icv1-f2m7-t13-616700dd` uses this complete input:

```json
{
  "a": "0x0",
  "b": "0x1",
  "field": "f2m-7-e1a6f45a",
  "form": "y^2+xy=x^3+a*x^2+b",
  "modulus": "0x83",
  "v": "1"
}
```

Run:

```text
./hyperelliptic-cover construct --input f2m7-model.json --output f2m7-cover.json
```

The command constructs the fixed family and immediately replays its geometry
certificate. A verification request contains exactly `model` and
`certificate`. For the same binary fixture, this is a complete request:

```json
{
  "model": {
    "a": "0x0",
    "b": "0x1",
    "field": "f2m-7-e1a6f45a",
    "form": "y^2+xy=x^3+a*x^2+b",
    "modulus": "0x83",
    "v": "1"
  },
  "certificate": {
    "construction": "binary_cubic_pullback_v1",
    "degree": 3,
    "f": ["0x0", "0x1", "0x0", "0x0", "0x0", "0x0", "0x0", "0x1"],
    "genus": 3,
    "h": ["0x0", "0x0", "0x1"],
    "x": ["0x0", "0x0", "0x0", "0x1"],
    "y_0": ["0x1"],
    "y_v": ["0x0", "0x1"]
  }
}
```

```text
./hyperelliptic-cover verify --input f2m7-verify.json --output f2m7-verified.json
```

The supported constructors are deliberately separate:

- odd characteristic greater than three: the fixed quadratic pullback
  `prime_quadratic_pullback_v1`, producing a genus-two monic sextic with two
  rational infinity points; and
- characteristic two: the fixed ordinary binary cubic pullback
  `binary_cubic_pullback_v1`, producing a genus-three odd-degree model with
  one rational infinity point.

An unsuccessful bounded construction never proves that no cover exists.
The `construct`, `verify`, and arithmetic `model` paths reject additional
properties even when the required coefficients are otherwise valid.
[pcc:unverified]

## Arithmetic request contract

An arithmetic request is a JSON object with no unknown fields and the
following members:

| Member | Required | Contract |
|---|---|---|
| `schema` | yes | Exact string `hyperelliptic-jacobian-operation/v1` |
| `model` | exactly one curve selector | Exact binary or prime elliptic model object accepted by `construct`; a supported prime model reconstructs its split-sextic cover |
| `odd_curve` | exactly one curve selector | Separately typed odd-prime one-infinity or split-sextic hyperelliptic curve object |
| `operation` | yes | `validate`, `identity`, `equal`, `negate`, `add`, or `scalar_multiply` |
| `left` | by operation | Required by `validate`, `equal`, `negate`, `add`, and `scalar_multiply` |
| `right` | by operation | Required by `equal` and `add` |
| `scalar` | by operation | Required by `scalar_multiply`; unsigned decimal or `0x` string |

Operations reject unused operands as well as missing ones: `identity` accepts
none; `validate` and `negate` accept only `left`; `equal` and `add` require
only `left` and `right`; and `scalar_multiply` requires only `left` and
`scalar`.

One-infinity `left` and `right` values are canonical Mumford pairs:

```json
{
  "u": ["coefficient of x^0", "coefficient of x^1"],
  "v": ["coefficient of x^0", "coefficient of x^1"]
}
```

Coefficient arrays are in ascending-power order. The zero polynomial is the
empty array. Coefficients must be canonical field elements; trailing zeroes
are rejected. In particular, binary Mumford `u` and `v` polynomials must not
carry a redundant final zero coefficient. The identity divisor is
`{"u":["0x1"],"v":[]}`.

Split-sextic balanced divisors add the required infinity counter:

```json
{
  "u": ["coefficient of x^0", "coefficient of x^1"],
  "v": ["coefficient of x^0"],
  "n_plus": 0
}
```

The balanced identity is `{"u":["0x1"],"v":[],"n_plus":1}`.
`n_plus` is rejected on binary and odd-prime one-infinity inputs, and is
required on split-sextic inputs. The two representations are not coerced.

The two curve selectors are intentionally not interchangeable.

### Binary cover arithmetic

For `model`, the tool verifies the elliptic model, reconstructs its fixed
cover, verifies the cover certificate, and then initializes the checked
one-infinity binary Jacobian. This complete request returns the identity on
the F2^7 catalog cover. [pcc:unverified]

```json
{
  "schema": "hyperelliptic-jacobian-operation/v1",
  "model": {
    "a": "0x0",
    "b": "0x1",
    "field": "f2m-7-e1a6f45a",
    "form": "y^2+xy=x^3+a*x^2+b",
    "modulus": "0x83",
    "v": "1"
  },
  "operation": "identity"
}
```

An addition request uses the same `schema` and `model`, plus:

```json
{
  "operation": "add",
  "left": {"u": ["0x1"], "v": []},
  "right": {"u": ["0x1"], "v": []}
}
```

Those members are illustrative fragments to merge into the complete request;
the first binary block is directly runnable JSON.

### Separate odd-prime one-infinity arithmetic

`odd_curve` has exactly these fields:

| Member | Contract |
|---|---|
| `family` | Exact string `odd_prime_one_infinity_v1` |
| `p` | Unsigned odd modulus; values at most 64 bits are checked deterministically, while wider one-infinity inputs retain the library's cached probable-prime screen and therefore carry a primality assumption |
| `genus` | Positive integer |
| `f` | Ascending coefficients of monic squarefree `f`, with exact degree `2g+1` |
| `infinity` | Exact string `one_rational` |

This complete request returns the identity for
`y^2=x^5+x^2+1` over F3:

```json
{
  "schema": "hyperelliptic-jacobian-operation/v1",
  "odd_curve": {
    "family": "odd_prime_one_infinity_v1",
    "p": "3",
    "genus": 2,
    "f": ["0x1", "0x0", "0x1", "0x0", "0x0", "0x1"],
    "infinity": "one_rational"
  },
  "operation": "identity"
}
```

Its complete successful response is:

```json
{
  "schema": "hyperelliptic-infrastructure-command/v1",
  "command": "arithmetic",
  "status": "ok",
  "result": {
    "operation": "identity",
    "context": {
      "family": "odd_prime_one_infinity_v1",
      "field": {
        "characteristic": "3",
        "representation": "prime_residue"
      },
      "curve": {
        "family": "odd_prime_one_infinity_v1",
        "p": "3",
        "genus": 2,
        "f": ["0x1", "0x0", "0x1", "0x0", "0x0", "0x1"],
        "infinity": "one_rational"
      },
      "infinity_configuration": "one_rational_point"
    },
    "result": {
      "divisor": {
        "u": ["0x1"],
        "v": []
      }
    }
  }
}
```

## Prime split-sextic balanced arithmetic

Passing a supported prime elliptic `model` to `arithmetic` reconstructs and
replays its monic sextic cover, then initializes the distinct balanced
two-infinity backend when the prime is proved by either the complete `u64`
test or the strict Sorenson--Webster `psi_13` bound. It
never applies the odd-degree one-infinity representation to that cover.
[pcc:unverified]

This complete model-derived F5 request returns the balanced identity for
`v^2=u^6+1`, the cover of `y^2=x^3+1` under `(x,y)=(u^2,v)`:

```json
{
  "schema": "hyperelliptic-jacobian-operation/v1",
  "model": {
    "a": "0",
    "b": "1",
    "field": "fp-5",
    "form": "y^2=x^3+a*x+b",
    "p": "5",
    "v": "1"
  },
  "operation": "identity"
}
```

A caller may instead supply the exact split curve through `odd_curve`:

```json
{
  "schema": "hyperelliptic-jacobian-operation/v1",
  "odd_curve": {
    "family": "odd_prime_split_sextic_balanced_v1",
    "p": "5",
    "genus": 2,
    "f": ["1", "0", "0", "0", "0", "0", "1"],
    "infinity": "two_rational"
  },
  "operation": "validate",
  "left": {"u": ["1"], "v": [], "n_plus": 1}
}
```

The canonical triple `(u,v,n_plus)` has `deg(u)<=2` and
`0<=n_plus<=2-deg(u)`. Infinity plus and minus are labelled by
`y/x^3=+1` and `y/x^3=-1`. The implementation checks validity, reduction,
equality, negation, addition, and scalar multiplication. Reference enumeration
is separately capped at `p<=31`; ordinary arithmetic is not an enumeration.

The same balanced representation now supports the induced maps for a direct
short-Weierstrass prime-field cover and for its checked projective composition
with a Montgomery target model. The checked formulas implement pullback,
arbitrary-class pushforward, the deck action, and both compositions
`pi_* pi^*=[2]` and `pi^* pi_*=1+deck_*`. This map implementation has the same
strict deterministic prime-proof boundary as balanced arithmetic: the
complete `u64` witness set, plus the first sixteen prime bases through 53
under the exclusive Sorenson--Webster bound
`3317044064679887385961981`. A Montgomery request inside that boundary parses
points only in the original Montgomery coordinates and conjugates both maps
through the checked degree-one isomorphism. Edwards normalization still
returns `unsupported_normalized_target_transfer_backend` before parsing a
point, because its rational charts need typed projective exceptional-point
handling. Extension fields remain unsupported for balanced arithmetic and
induced maps.
[pcc:unverified]

The production-independent split-sextic oracle now has a second exact wrapper
for the composed Montgomery path. Over `F_7` it fixes original Montgomery
parameters `A=3`, `B=2`, cover shift `c=1`, the short model
`y^2=x^3+3x+1`, and source
`v^2=u^6+3u^4+6u^2+5`. It independently obtains `N1=14`, `N2=58`,
`#Jac(C)(F_7)=120`, and `#E(F_7)=12`, then checks all 14,400 source sums,
1,728,000 associativity triples, every pullback and pushforward, and both
composition identities before comparing with production. The exhaustive
support inventory contains 2,608 principal zero divisors, 21 classes with
irreducible-quadratic support, 12 repeated-support classes, 1,989
shared-support pairs, and 1,989 conjugate-support pairs. This is evidence for
one tiny-field fixture only. It does not promote any of the seven live
proof-gated Montgomery bindings or establish family-wide or catalog-wide
coverage, a subgroup result, a solver, scaling, or computational advantage.
[pcc:unverified]

Odd extension fields return structured `unsupported_extension_field`.
All-passing prime moduli outside the strict proof bound return structured
`unsupported_without_primality_certificate`; a Miller--Rabin witness remains
an unconditional composite result at any size. A supplied nonprime, nonsquarefree, nonmonic,
wrong-degree, or malformed balanced input is `invalid_input`, not an
unsupported existence claim. A failed odd-map request is never a
nonexistence result.

### Prime validation and diagnostic bounds

`SplitHyperellipticCurveP` accepts only deterministically proved odd primes:
the complete word-sized test or strong Miller--Rabin to the first sixteen
prime bases below the strict `psi_13` bound. Its response serializes the
method, configured bases, and exclusive bound. The separate one-infinity
`HyperellipticCurveP` still retains its pre-existing cached probable-prime
screen for wider inputs, so callers must not cite that path as a primality
proof. The
optional quadratic-extension reference backend
has the narrower `p <= 4096` boundary, matching the bounded quadratic point
counter: `Fp2Ctx` rechecks that `p` is an odd prime, constructs and validates
a canonical nonzero quadratic nonresidue, and every element operation
canonicalizes both coordinates before use. Forged public element fields
therefore cannot bypass reduction. [pcc:unverified]

The exhaustive point counters are diagnostics, not general large-field
algorithms:

- linear `F_p` enumeration requires `p <= 1,000,000`; and
- quadratic `F_{p^2}` enumeration and `Fp2Ctx` require `p <= 4096`.

Requests outside a checked bound return a structured error where the API is
fallible; convenience wrappers may panic and are not public-input entry
points. These counting bounds do not restrict ordinary Jacobian arithmetic
within its declared field-validation contract.

## Transfer boundary and evidence records

The structured `transfer` command supports the checked binary degree-three
family and the direct short-Weierstrass prime-field degree-two family. Every
request has exact schema `hyperelliptic-transfer-operation/v1`, an exact
elliptic `model`, one operation, and only that operation's operands:

| Operation | Required operands | Binary cubic | Direct odd-prime quadratic |
|---|---|---|---|
| `pushforward` | `divisor` | Elliptic image of any valid reduced one-infinity class | Elliptic image of any canonical balanced class |
| `pushforward_certificate` | `divisor` | Closed-point components, reconstructed support, and component-sum checks | Same checked image as `pushforward`, with the symmetric base-field method identified |
| `image_endomorphism_certificate` | `divisor` | Checked image endomorphism and kernel relation fields | Unsupported binary-only certificate shape |
| `torsion_projector_certificate` | `divisor`, declared `order` | Conditional projector; exactness of `order` is not checked | Unsupported binary-only certificate shape |
| `deck_transform` | `divisor` | Unsupported odd-prime-only operation | Checked `(u,v)->(-u,v)` action, including the infinity-branch swap |
| `pullback` | target `point` | Pullback of a rational elliptic point class | Balanced pullback of a rational elliptic point class |
| `pull_push_composition_certificate` | target `point` | Typed `phi_* phi^*=[3]` witness | Typed `pi_* pi^*=[2]` witness |
| `push_pull_composition_certificate` | `divisor` | Unsupported odd-prime-only operation | Typed `pi^* pi_*=1+deck_*` witness |
| `pullback_closed_point_certificate` | target `point` | Fiber and closed-point component witness | Unsupported binary-only certificate shape |
| `generated_subgroup_certificate` | target `point`, declared `order` | Named-generator annihilation, pullback, norm, degree-three composition, and recovery checks | Named-generator annihilation, pullback, degree-two composition, and recovery checks |
| `subgroup_status` | declared `order` | Conditional degree-three coprimality/kernel status | Conditional degree-two coprimality/kernel status |

Unknown fields, missing operands, and irrelevant operands are rejected. Target
points use `{"kind":"infinity"}` or canonical affine coordinates. Binary
coordinates are hexadecimal polynomial-basis bitsets; prime coordinates may
be decimal or `0x` strings. Binary divisors use canonical one-infinity `(u,v)`
pairs and must not include `n_plus`. Direct prime-cover divisors use canonical
balanced `(u,v,n_plus)` triples.

This complete non-F8 F2^7 request checks the ramified infinity composition:

```json
{
  "schema": "hyperelliptic-transfer-operation/v1",
  "model": {
    "a": "0x0",
    "b": "0x1",
    "field": "f2m-7-e1a6f45a",
    "form": "y^2+xy=x^3+a*x^2+b",
    "modulus": "0x83",
    "v": "1"
  },
  "operation": "pull_push_composition_certificate",
  "point": {"kind": "infinity"}
}
```

The successful response binds the exact target model, reconstructed cover
certificate, field degree, source genus, map degree, and formula
`(x,y)=(u^3,u*v+d)`. The composition result contains both the formal
pushforward and `[3]` input and requires `identity_holds=true`.

For a direct odd-prime model, the command reconstructs and rechecks
`v^2=F(u^2+c)` and the map `(x,y)=(u^2+c,v)` before evaluating a balanced
class. This complete request checks the target-side composition over F5:

```json
{
  "schema": "hyperelliptic-transfer-operation/v1",
  "model": {
    "v": "1",
    "form": "y^2=x^3+a*x+b",
    "p": "5",
    "field": "fp-5",
    "a": "0",
    "b": "1"
  },
  "operation": "pull_push_composition_certificate",
  "point": {"kind": "affine", "x": "0", "y": "1"}
}
```

The result explicitly compares the pushforward with `[2]` of the input. The
source-side composition takes a balanced `divisor` and compares the pullback
of its image with the input plus its deck transform. Direct prime-field
transfer rejects extension models and moduli outside the strict deterministic
proof range with a structured `unsupported` response. Geometry construction
over an extension field remains supported separately; transfer support is not
inferred from that construction. A bounded five-row `GF(5^2)`/`GF(7^2)` pilot
locally checks the exact catalog polynomial bases with implementation-separated
finite-field arithmetic, source-sextic squarefreeness, the separable
degree-two map, all rational points and fibers, and the two unramified infinity
branches. Graph regeneration still uses the repository checker and the basis
comparison is differential against `Fp2Ctx`, so this is not an independent
source-Jacobian implementation. It implements no general source-Jacobian
reduction or arbitrary-class transfer, and those five rows have no named EC1
binding; the other 1,400 odd-extension rows remain structured unsupported.

The Montgomery-to-short projective isomorphism is a checked typed module with
exhaustive adapter tests over `p=5,7,11`, including infinity and an independent
group law. It is now composed end to end with the quadratic-cover transfer
API. A complete `p=7` fixture checks all 12 target points and all 72 balanced
classes, both compositions, and pullback/pushforward additivity. All six live
Montgomery normalizations have moduli outside the accepted deterministic
prime-proof range, so no normalized live row is promoted. Edwards
normalization remains geometry-only.

`pushforward_arbitrary_jacobian_class` handles every valid reduced class of
support degree at most three on the supplied checked binary model. It factors
the support, evaluates each closed point in its residue field, and takes the
Frobenius-orbit norm. Malformed, foreign, or noncanonical divisors fail closed.
The exact polynomial-basis F8 fixture `a=1`, `b=3`, `d=7` remains an exhaustive
control with 486 reduced classes and 236,196 ordered additivity pairs; it is no
longer the implementation's field/model boundary.

That F8 control is a norm-mechanics test, not an injective full-group ECDLP
transfer: its target has order 6, the six norm fibers each have size 81, and
the map degree 3 is not coprime to 6. The generic degree-coprime subgroup
argument therefore does not resolve its full target-group kernel.

The fixed fixture's Jacobian cardinality has a separate machine-readable
[order certificate](fixed-f8-genus3-order-certificate.json). Its
[implementation-separated native replay](../../src/bin/ecdlp_audit/fixed_f8_order.rs)
exhaustively counts `6`, `104`, and `558` points over `GF(8)`, `GF(8^2)`, and
`GF(8^3)`, reconstructs the genus-three zeta numerator
`1-3T+24T^2-48T^3+192T^4-192T^5+512T^6`, and verifies
`#Jac(H)(GF(8))=P(1)=486`. [pcc:unverified]

```sh
cargo run --locked --bin ecdlp-audit -- verify-fixed-f8-order \
  --input docs/curves/fixed-f8-genus3-order-certificate.json --repo-root .
cargo test --locked --bin ecdlp-audit fixed_f8_order
```

That replay is a separate Rust module which does not import the production
binary-curve, Mumford, Jacobian, or transfer arithmetic. It
certifies the order through a separate implementation only; it does not
certify the group structure, reduced-divisor enumeration, `A/B` presentation,
arithmetic law, norm map, homomorphism, pull-push composition, or subgroup
preservation. The adjacent 12-test suite includes the checked-in positive
replay plus fail-closed mutations of claims, equations, counts, encodings,
keys, and the certificate UID.

The generic `subgroup_status` classifier remains conditional on an exact
subgroup order and reports `exact_order_verified_by_command=false`. The more
specific `generated_subgroup_certificate` checks a named nonidentity point
`G`, a supplied annihilator `r`, `[r]G=O`, `[r]phi^*(G)=0`, the relevant
pull-push identity, and recovery through the inverse map degree modulo `r`.
When `gcd(r,degree)=1`, those checks prove injectivity on the named cyclic
subgroup `<G>` even if `r` is only an annihilator rather than the exact order.
They do not prove that `r` is prime or exact, do not describe other source
Jacobian components, and do not establish a computational advantage.
[pcc:unverified]

The F2^7 evidence fixture additionally checks in the same native stack that
the named generator is nonidentity and is killed by the prime 29, and
therefore has exact order 29 before applying the degree-inverse argument. This
is not an independent source-order implementation. A separate fixed-GF(8)
oracle, frozen at producer commit
`e0ae0fe5d09c0492a8b50600e88d0b9da68afef1`, constructs all 486 reduced
source classes
from 233 graph places and 33,856 retained principal divisors without importing
the production source law. It checks all 57,513,726
principal-relation lookups, 114,791,256 associativity triples, 236,196 ordered
pushforward-additivity pairs, every pullback, and both composition identities
before comparing every class operation with production. Exact counters also
pin inert and higher-degree rejections plus repeated, nonsplit, ramified,
shared, and conjugate support. The clean Rust 1.98 replay passed 2/2 and binds
the raw output and whole-Cargo-invocation resource receipt, including its
wrapper-level peak RSS, in the companion draft assessment. This supports the
source law and maps for this one fixed fixture; it has no signed PCC standing
and retains packaged replay, complete hardware/isolation, mutation-control,
named-subgroup, independent-review, and catalog-wide obligations.
The dated 2026-11-18 report is preserved; its
[additive status note](../../research/curve_transfer_infrastructure_20261118/ERRATA-2026-11-19.md)
records this later fixture result without rewriting the earlier evidence.

A separate exact `F_2` control uses
`E: y^2+xy=x^3+1`, `H: v^2+u^2v=u^7+u`, and
`phi(u,v)=(u^3,uv+1)`. It constructs the 20 source classes from principal
divisors without production Jacobian addition, checks all 400 ordered sums,
all 8,000 associativity triples, independently implemented pullback and
pushforward, and both composition identities, and only then compares with
production. This remains independent source-law evidence for a second tiny
general-path fixture; it is not the basis for the 486-class result and does not
establish a catalog-wide oracle.

The conforming machine-readable envelope is
[`../../research/hyperelliptic_cover_infrastructure_20261006/evidence.json`](../../research/hyperelliptic_cover_infrastructure_20261006/evidence.json).
Its F2^7 record binds all of the following:

- ICV1 `icv1-f2m7-t13-616700dd` and EC1
  `EC1N7Ce0hb6d297a2ca08`;
- the exact source `H`, `Jac(H)`, curve morphism, field identity, and bounded
  induced-map domains;
- the supported degree-three composition and exact order-29 subgroup check;
- content hashes and local test receipts; and
- a structured refutation of arbitrary-class pushforward support.

That last item describes the immutable October 6 snapshot, not current binary
support. The dated crosswalk is the generated v2
[`cover-transfer-assessments.json`](cover-transfer-assessments.json), with
structural contract
[`cover-transfer-assessment.schema.json`](cover-transfer-assessment.schema.json)
and a dated
[transfer report](../../research/cover_transfer_assessment_20261013/README.md).
It classifies the exact catalog at commit
`728304718cb0ab6df70da620a908a5f76f175293`: 15,791 maps, 15,789 distinct
cover models, and 979 exact EC1 bindings. Its map profiles contain 165 binary
cubic covers, 14,272 direct prime-field quadratic covers, 30
normalized-target prime-field covers, and 1,324 odd-characteristic extension
covers. Its subgroup profiles contain 142 binary and 837 odd-characteristic
representations. “Assessed” means that every declared claim in that frozen
cohort has an explicit status; it does not mean that every capability is
supported or that later rows inherit a result.

The 2026-11-18 live graph instead has 22,417 maps and 22,415 distinct covers:
174 binary, 20,808 direct prime-field, 30 normalized-target prime-field, and
1,405 odd-extension maps. It links 1,090 EC1 representations: 150 binary, 909
direct prime, 22 normalized prime, and 9 odd extension. Every live
`subgroup_transport` field remains `not_tested` with null evidence. Geometry
coverage and identity linkage do not promote a transfer or subgroup claim.
The separate `transfer-catalog-export` command replays the 174 binary maps and
their named EC1 generators into a new hash-bound corpus; it does not rewrite
those historical graph fields.

Current binary source has checked one-infinity arithmetic and arbitrary-class
degree-at-most-three transfer. The bounded direct odd-prime backend supplies
balanced arithmetic and the induced degree-two maps for checked split sextics.
Its deterministic proof boundary covers 20,539 live direct-prime maps and 675
linked EC1 records, including the two 73-bit rows at
`p=5902977611310272157103`. The other 269 wide-direct maps and 234 bindings,
normalized targets, and extensions do not receive direct-backend support: the
current canonical odd corpus classifies 240 bindings as proof-range
unsupported, 15 as Edwards normalized-backend unsupported, 9 as
extension-backend unsupported, and one as invalid target-generator data. The
seven Montgomery bindings have a composed backend but remain in the proof-gap
bucket because their fields or declared orders lack accepted certificates;
they are not live-supported. The widened live catalog support is therefore
exactly a two-row pilot under the strict exclusive `psi_13` proof bound, not a
general wide-prime claim. Numeric exact-order evidence remains a separate
per-record field.
The supplied traits file is recorded as advisory provenance and cannot
promote that field. The exporter records non-`unknown` evidence only from its
bounded deterministic small-prime check; the named-subgroup certificate
itself never promotes an annihilator into an exact order. Earlier standalone
evidence includes the order-29 overlay and the
implementation-separated cardinality-486 certificate for the separate fixed
F8 source fixture. The latter is not an EC1 subgroup-order certificate. All
computational-advantage and end-to-end solver costs for both the 15,791-row
historical assessment and the larger live graph remain unknown. Cryptanalysis
implementation/custody head
`69b803d2a31f39737674e2a33da623197a304157` mirrors the exact/adapted
12-record source closure through crypto producer
`5c6e43a6f0cffe0afc4c962ff54056716cf6437e`. The direct and composed transfer
modules are byte-exact; the prime module omits only the unmirrored
operation-pricing module, and the integration test changes only the crate path.
Local implementation validation passed the composed integration target 3 of
3, the focused normalized module 6 of 6, the existing model-adapter regression
3 of 3, the parity verifier 8 of 8, all 12 live parity records, targeted Rust
1.98 `rustfmt`, JSON parsing, exact/adapted comparisons, and
`git diff --check`.

The exact-head custody set has a 27,552-byte workflow at SHA-256
`084795e244e5854fbc18c0a8023c8ef548a6687d968644e096fb8e4e848a809e`,
the unchanged 7,803-byte G3 driver at
`7ac0e9892f1e08c399ab5431716cd13628c2f74f8330db41ea1766ecb5618d9a`,
the 29,406-byte G3 verifier at
`e21b1b79e46717965772e16d854165ae07a50d5e992f35d006918fe386fe064c`,
the 4,489-byte G3 transition at
`2e121138eec70ab8dfca51cc811351a9d987a223d20afc2ab369df91e0c574b0`,
and the 6,307-byte FP transition at
`d8bfa5aff2bf039878bec961b90482deadd0e74fc4745ba0f4b3d5398ed8dd7a`.
The frozen FP and G3 predecessor evidence and the G3 arithmetic replay body
were not rewritten.

Manual exact-head run `44570090434` passed all seven jobs. G3 reached the
arithmetic/proof stage: producer and consumer tests passed; Lean `Descent`
(12 theorems), `ModelTransport` (2), and `CanonicalNorm` (13) passed with zero
trusted declarations; and the final receipt records
`predecessor437+successor431plus6/allinputguards`. G3 job `135722448213`
uploaded artifact `14553252727`, 68,150 bytes, at SHA-256
`8baf6047ef49bb8ab130a46cd512fe42d2af8cae91b81bdb2cfc8e1a3cf8f4ce`.
The docs-only successor head
`0507db1f155ff5e41ebbab1cd0190f4d97a5da71` does not change this
implementation/custody identity. The generated corpus and independent `F_2`
source-law oracle remain crypto-owned and were not copied into cryptanalysis.
The fixed 486-class `GF(8)` independent source-law gate passes locally and has
a read-only mathematical review; required review, signed standing, a fresh
current-head package and extracted-package replay, and catalog-wide
independent divisor-map replay remain open. This receipt is not a Jacobian
index-calculus solver or evidence of a speedup. The assessment does not change
the geometry semantics or browser status fields in `covers.json` and
`cover-links.yaml`.
[pcc:unverified]

Regenerate and check the assessment deterministically with:

```text
node scripts/build_cover_transfer_assessments.mjs --write
node scripts/build_cover_transfer_assessments.mjs --check
python -m jsonschema -i docs/curves/cover-transfer-assessments.json \
  docs/curves/cover-transfer-assessment.schema.json
node research/cover_transfer_assessment_20261013/verify_assessment.mjs
```

The verifier replays membership partitions, claim/evidence bindings, source
pins, and all 15,791 frozen cover formulas. The accompanying Lean package
formalizes
only the conditional group-theoretic injectivity step under an already-proved
push-pull identity, exact finite subgroup cardinality, and coprimality. It does
not construct the maps or certify catalog subgroup orders. A full Jacobian
index-calculus solver remains a separate deliverable. [pcc:unverified]

The historical envelope records the prime F827 cover as an explicit
construction while marking use of one-infinity arithmetic on its even sextic
as `refuted`. That record predates the balanced backend and must not be
silently rewritten. The original PR #1383 artifact contains 115 rows (28
prime and 87 binary); the later audit-baseline catalog contains 121. These
populations must not be conflated.

## Bounded transfer sweeper

`curve-transfer-sweep` is a separate standalone binary for deterministic,
finite candidate inventories:

```text
./curve-transfer-sweep capabilities --output CAPABILITIES.json
./curve-transfer-sweep sweep \
  --request REQUEST.json --registry docs/curves/registry.json \
  --output SWEEP.json
./curve-transfer-sweep verify \
  --input SWEEP.json --registry docs/curves/registry.json \
  --output REPLAY.json
```

It resolves an exact EC1 alias, ICV1 identity/slug, or model and enumerates
only requested parameters from `binary_cubic_pullback_v1` and
`prime_quadratic_pullback_v1`. Requests declare inclusive prime-shift bounds,
maximum attempts, field bits, genus, and degree. Search spaces larger than the
declared attempt budget fail rather than returning a truncated prefix.

Every output retains exact coefficients, HC1/CV1 identities, map formulas,
field, degree, attempt dispositions, subgroup obligations, and separate
construction/arithmetic/transfer/solver statuses. “Exhaustive” means only the
declared finite family and bounds. Zero candidates is not a proof that no
cover, isogeny, correspondence, higher-dimensional representation, or
transfer exists. The sweeper is not an index-calculus solver and does not
establish an advantage.

The `curve-transfer-sweep-capabilities/v1` inventory lists six repository
shapes but marks only the binary-cubic and prime-quadratic pullbacks as
integrated request families. The bielliptic atlas and selected genus-three
hyperelliptic constructor route to separate bounded tools. The plane-quartic
row is an elliptic norm projection without a full quartic Jacobian group, and
the small-`F_2` factor search constructs no transfer map. See
[`CURVE_TRANSFER_SWEEP.md`](CURVE_TRANSFER_SWEEP.md) for the exact routing and
claim boundaries.

## Catalog export

Rebuild the existing cover catalog directly from the exact registry:

```text
./hyperelliptic-cover catalog-export \
  --registry docs/curves/registry.json \
  --output covers.json
```

The exporter preserves exact coefficients and curve identifiers. Schema
validation alone is not a mathematical certificate: consumers must also
recompute EC1/ICV1 identities, resolve evidence and implementation references,
replay the cover equations and infinity checks, and enforce each declared
domain boundary.

The exported catalog identifies its wrapper with
`generated_by = "hyperelliptic-cover catalog-export"`. Its
`producer_source_sha256` is SHA-256 of the exact UTF-8 checker source, model
normalization source, and `hyperelliptic-cover` wrapper source concatenated in
that order. At this revision that digest is
`07de7a833349cab197c8accb7ee5b67059f78630a62ef13e29333c23c3b0ffea`.
Read the value from the exact export rather than copying a digest from an
earlier revision. Changing any of the three producer inputs changes the
provenance digest, so this literal must change with them. Both
`hyperelliptic-cover catalog-export` and `curve_cover_check` require callers
to supply a registry path; the release archive bundles one, but neither
executable embeds it.

### Binary named-subgroup certificate export

The second exporter replays the exact live binary map rows and every linked
EC1 generator rather than rebuilding geometry alone:

```text
./hyperelliptic-cover transfer-catalog-export \
  --registry docs/curves/registry.json \
  --cover-graph docs/curves/cover-links.yaml \
  --traits docs/curves/traits.json \
  --producer-commit 0123456789abcdef0123456789abcdef01234567 \
  --rustc-version "rustc VERSION" \
  --output binary-transfer-certificates.json
```

Use the actual forty-lowercase-hex revision containing the producer and its
actual compiler version; the placeholders above are only command-shape
examples. The output schema is
[`binary-subgroup-transfer-certificates.schema.json`](binary-subgroup-transfer-certificates.schema.json),
with the claim boundary and semantic replay obligations in
[`BINARY_SUBGROUP_TRANSFER_CERTIFICATES.md`](BINARY_SUBGROUP_TRANSFER_CERTIFICATES.md).
The producer binds the registry, graph, traits input, source digest, exact
ICV1/EC1/HC1/CV1 identities, witnesses, and separate exact-order evidence.
Schema validation does not establish hash preimages, cross-file completeness,
or arithmetic correctness; replay and independent evidence gates remain
required.

Replay a saved corpus with the installed executable and the exact three input
documents:

```text
./hyperelliptic-cover transfer-catalog-verify \
  --input docs/curves/binary-subgroup-transfer-certificates.json \
  --registry docs/curves/registry.json \
  --cover-graph docs/curves/cover-links.yaml \
  --traits docs/curves/traits.json \
  --expected-sha256 CORPUS_SHA256 \
  --expected-bytes CORPUS_BYTES \
  --output binary-transfer-certificates.replay.json
```

The hash and byte arguments are optional as a pair. When present, they are
checked before parsing. The verifier then rebuilds the graph, replays every
map and named generator, regenerates the complete corpus using the saved
caller-supplied producer strings, and requires byte-for-byte equality with
the producer's canonical pretty JSON (including the trailing newline). Exit
status `0` means an exact complete replay, `1` means an exact corpus containing
failed records, and `3` means malformed, noncanonical, stale, foreign, or
semantically different input. `producer.git_commit` and `rustc_version` remain
caller-supplied metadata; a successful same-implementation replay is not an
independent execution attestation or a claim of exact order where the record
still says unknown.

For the tracked 2026-11-19 source-custody successor, `CORPUS_SHA256` is
`a55c7f9a0c836614281388f19b3b31fa622eeb1650ddc00c59e4e78562471392`
and `CORPUS_BYTES` is `500924`. It records producer revision
`5c6e43a6f0cffe0afc4c962ff54056716cf6437e` with length-framed producer-source
digest
`e7a12f60c219624b2442bb54a79bb114ad8c89cfd1d33f6f0ca4643af7bae7dd`.
That source frame includes the binary-module re-exports and the deterministic
primality implementation and word-sized helper used by exact-order evidence.
Release run `44596599220`, Rust GNU job `135817148417`, passed the 21-file
source-tree replay plus native installed-layout exact replay and receipt
comparison at evidence head `98e4fbfe…`, then uploaded artifact `14567327688`;
independent download of the
artifact remains proxy-blocked. The durable job-side record is
[`standalone-package-run-44596599220.json`](../../research/curve_transfer_infrastructure_20261118/standalone-package-run-44596599220.json).
The repository-mode 21-file Git-tree verifier
and receipt additionally reproduce `e7a12f60…` with replacement objects
disabled and logical paths checked against their normalized `include_str!`
targets. Its 8/8 local controls and the exact-head release replay pass. Curve
run `44618940782` at successor head `37b9b2a6…` has green assessment and Lean
jobs; its certificate job passed the combined transfer/source replay,
optimized oracles, odd-transfer replay, native plan, fixed-F8 certificate, and
Rust standards checks. A pre-existing global IC display input mismatch then
failed the Python standards wrapper and skipped the final cover/browser checks;
a scoped successor is pending. This source receipt does not attest the executable build or supply an
independent arithmetic check.
The earlier `67142558…` / `e0ae0fe5…` and `f402ccd4…` / `8c61e842…` corpora
remain historical evidence, not the current same-implementation replay target.
[pcc:unverified]

For the 2026-11-18 graph, the expected bounded population is 174 binary map
records, 147 maps with 150 named generator bindings, and 27 maps without a
named subgroup input. Of the 150 bindings, 102 have locally proved exact
generated order and 48 deliberately retain unknown exact order; none failed
the recorded transfer obligations. A nonzero process exit accompanies any
retained failed obligation. Successful named-generator records prove only the
scoped injectivity result described above. They are not solver, timing, or
computational-advantage records.
[pcc:unverified]

## Odd-characteristic catalog certificates

`odd-transfer-catalog` is a separate standalone exporter and exact replay
tool for the 22,243 odd-characteristic maps and 940 named bindings. Its command
contract and fail-closed exit semantics are documented in
[`ODD_SUBGROUP_TRANSFER_CERTIFICATES.md`](ODD_SUBGROUP_TRANSFER_CERTIFICATES.md).
The tracked 17,879,292-byte canonical corpus has SHA-256
`1ecfabcdaecc3e00d78ab2ce7eebeb94607e1c2c224f261fb5718a4f53b65268`.
It was generated from clean producer commit
`8c94831426fa35040fa7b38b1379299bf6e6d957` with framed source digest
`a5412b92e5e027b281d79c541cc34df130e16d9c71a7034f2065c6e181d8b3ef`
over 29 source files. Its binding partition is 675 supported direct, 240
proof-range unsupported, 0 kernel-intersection unsupported, 15 Edwards
normalized-backend unsupported, 9
extension-backend unsupported, 1 failed invalid generator, 0 other invalid,
and 0 transfer-arithmetic failures. Canonical same-implementation replay exits
`1` with `verified_named_subgroup_incomplete`; repository source replay exits
`0` after binding all 29 source files and the Lean artifact to the producer
commit. Malformed or stale input exits `3`.

This corpus is additional evidence, not a rewrite of the generated graph.
The seven Montgomery bindings use the composed backend but remain proof-gated
because accepted field/order evidence is absent; all fifteen Edwards rows and
the extension rows still need their typed backends. The one invalid generator
is not relabelled as a missing proof. The recorded Rust compiler label is
caller-supplied and unverified, and the local unsigned Lean theorem proves only
the conditional structural implication under supplied hypotheses.
Installed-package replay, independent replay, and the remaining source-order
certificates are separate release gates. The corpus supplies no relation
solver, key recovery, scaling result, or computational-advantage evidence.
[pcc:unverified]

The earlier 17,879,352-byte `7c60fc…` pre-composition corpus from producer
`e0ae0fe5…`, with partition 675/233/22/9/1, remains immutable historical
evidence.
