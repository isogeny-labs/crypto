# Hyperelliptic cover infrastructure

`hyperelliptic-cover` is a bounded, certificate-oriented command-line tool.
It constructs and verifies the repository's two explicit cover families and
runs checked Jacobian arithmetic only when the infinity configuration matches
an implemented representation. It is not an index-calculus or key-recovery
solver.

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

Users of a packaged artifact do not need `cargo run`. The six command shapes
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
```

The source capabilities documented here are active-branch work. The release
workflow is being extended to package `hyperelliptic-cover` and
`curve-transfer-sweep` and to smoke-test transfer, malformed-input, and bounded
sweep requests, but no package containing this revision has been published.
Do not attribute these commands to an older published archive. A fresh
artifact digest, installed-layout replay, and direct replay of the complete
live registry remain open release obligations.

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
the F2^7 catalog cover:

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
| `p` | Unsigned odd prime of at most 64 bits; primality is checked deterministically over the complete supported range |
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
two-infinity backend when the prime fits the deterministic 64-bit domain. It
never applies the odd-degree one-infinity representation to that cover.

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
short-Weierstrass prime-field cover. The checked formulas implement pullback,
arbitrary-class pushforward, the deck action, and both compositions
`pi_* pi^*=[2]` and `pi^* pi_*=1+deck_*`. This map implementation has the same
deterministically checked, at-most-64-bit prime-field boundary as balanced
arithmetic. It does not extend the map support to normalized target models or
extension fields.

Odd extension fields return structured `unsupported_extension_field`, and
prime moduli larger than 64 bits return structured
`unsupported_prime_size`. A supplied nonprime, nonsquarefree, nonmonic,
wrong-degree, or malformed balanced input is `invalid_input`, not an
unsupported existence claim. A failed odd-map request is never a
nonexistence result.

### Prime validation and diagnostic bounds

`HyperellipticCurveP` and `SplitHyperellipticCurveP` accept only
deterministically validated odd primes of at most 64 bits. This replaces the
earlier probable-prime wording for these arithmetic implementations. The
optional quadratic-extension reference backend
has the narrower `p <= 4096` boundary, matching the bounded quadratic point
counter: `Fp2Ctx` rechecks that `p` is an odd prime, constructs and validates
a canonical nonzero quadratic nonresidue, and every element operation
canonicalizes both coordinates before use. Forged public element fields
therefore cannot bypass reduction.

The exhaustive point counters are diagnostics, not general large-field
algorithms:

- linear `F_p` enumeration requires `p <= 1,000,000`; and
- quadratic `F_{p^2}` enumeration and `Fp2Ctx` require `p <= 4096`.

Requests outside a checked bound return a structured error where the API is
fallible; convenience wrappers may panic and are not public-input entry
points. These counting bounds do not restrict ordinary Jacobian arithmetic
within its declared 64-bit prime-field contract.

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
transfer rejects extension models and primes wider than 64 bits with a
structured `unsupported` response. Geometry construction over an extension
field remains supported separately; transfer support is not inferred from
that construction.

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
`#Jac(H)(GF(8))=P(1)=486`:

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
is not an independent source-order implementation. A separate test-only GF(8)
oracle reimplements support factorization, residue fields, Frobenius trace,
and the target elliptic group law for all 486 classes and all 236,196 ordered
image sums. Its source additions still use the production Jacobian law, so it
does not independently validate that group law or establish catalog-wide
support.

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
balanced arithmetic and the induced degree-two maps for checked split sextics;
normalized targets, extensions, and larger primes remain unsupported for
those maps. Numeric exact-order evidence remains a separate per-record field.
The supplied traits file is recorded as advisory provenance and cannot
promote that field. The exporter records non-`unknown` evidence only from its
bounded deterministic small-prime check; the named-subgroup certificate
itself never promotes an annihilator into an exact order. Earlier standalone
evidence includes the order-29 overlay and the
implementation-separated cardinality-486 certificate for the separate fixed
F8 source fixture. The latter is not an EC1 subgroup-order certificate. All
computational-advantage and end-to-end solver costs for both the 15,791-row
historical assessment and the larger live graph remain unknown. The retained
cryptanalysis parity receipt predates the general binary norm, so current
mirror/integration evidence is also open. The assessment does not change the
geometry semantics or browser status fields in `covers.json` and
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
index-calculus solver remains a separate deliverable.

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
that order. Read the value from the exact export rather than copying a digest
from an earlier revision. Changing any of the three producer inputs changes
the provenance digest. Both
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

For the tracked 2026-11-18 corpus, `CORPUS_SHA256` is
`93150649c0754adb1ca50d74fa4419df9a77c98b156e75e33084a9f31f656ce8`
and `CORPUS_BYTES` is `498477`.

For the 2026-11-18 graph, the expected bounded population is 174 binary map
records, 147 maps with 150 named generator bindings, and 27 maps without a
named subgroup input. A nonzero process exit accompanies any retained failed
obligation. Successful named-generator records prove only the scoped
injectivity result described above. They are not solver, timing, or
computational-advantage records.
[pcc:unverified]
