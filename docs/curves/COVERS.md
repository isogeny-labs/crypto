# Automatic hyperelliptic cover checks

`curve_cover_check` constructs and replays explicit covers **H -> E** for
the supported elliptic model forms in the ICV1 catalog: short Weierstrass
models over `GF(p)` and over extensions `GF(p^k)` with `p > 3`, and ordinary
binary models. Findings are in
[`covers.json`](covers.json), joined to [`registry.json`](registry.json)
by slug **and the full SHA-256 of the exact model JSON**. The registry's
model and EC1 identities stay unchanged. The lab browser shows the finding
and certificate on each curve page.

```sh
cargo test --bin curve_cover_check
cargo run --bin curve_cover_check --
cargo run --bin curve_cover_check -- --check
python3 scripts/build_lab_browser.py
```

`--registry PATH --output PATH` accepts another ICV1 registry. `--check`
recomputes the report, including all equations and checks, and requires
byte-identical saved output. It rejects stale registry/source digests and
altered certificates. No network or external algebra service is used.

## What a finding means

| Status | Meaning |
| --- | --- |
| `verified_over_declared_field` | Explicit polynomial map, nonsingularity conditions, genus and degree checks pass over the specified field. |
| `unsupported` | These constructors do not handle the model or characteristic. Existence stays null. |
| `invalid_input` | Field, coefficient or elliptic nonsingularity checks failed. Existence stays null; the batch retains the row and exits unsuccessfully. |

An identity mismatch, duplicate slug or malformed registry aborts the batch.
A failed candidate never becomes a claim that *no* cover exists. These
constructors are complete for the stated supported model families; they
are not an exhaustive search over every genus, degree or correspondence.
Minimum genus and minimum degree remain null.

Prime moduli pass a fixed 20-base Miller-Rabin screen. This is **not a
primality proof**: that field's primality remains an explicit registry input
assumption, including when a row says `verified_over_declared_field`.
Binary defining polynomials pass the exact Rabin irreducibility criterion,
and so does an extension's modulus `t^k + c_{k-1}t^(k-1) + ... + c_0`, over
`GF(p)`: `t^(p^k) = t`, and `gcd(t^(p^(k/l)) - t, f) = 1` for every prime
`l | k`. An extension's field label must be the one ICV1 derives from `p`,
`k` and the modulus, and its prime `p` passes the same screen as a prime
field's.
Canonical coefficients, field-label consistency and the elliptic
discriminant condition are checked. Field sizes above 4096 bits are rejected.
The checker certifies model geometry, not the catalog's group order,
generator or EC1 subgroup parameters.

The certificate stores ascending-power coefficient arrays for

`H: v^2 + h(u)*v = f(u)` and `x = x(u), y = y_v(u)*v + y_0(u)`.

A field element is one `0x` integer: a prime-field element itself, a binary
element its polynomial-basis bits, and an element
`e_0 + e_1*t + ... + e_{k-1}*t^(k-1)` of `GF(p^k)` the integer
`e_0 + e_1*p + ... + e_{k-1}*p^(k-1)`.

The curves mean their smooth projective models. The verifier substitutes
the supplied map into the target equation, reduces using H's equation,
and checks both the constant and v coefficients. It separately checks
the construction's genus and degree hypotheses; an equation identity
alone would not certify them.

## Prime characteristic greater than 3

For `E: y^2 = F(x) = x^3 + a*x + b`, choose the first
`c in {0,1,2,3}` with `F(c) != 0`. Such a c exists: a cubic has at most
three roots and these four elements are distinct. Construct

`H: v^2 = F(u^2+c)`, with `(x,y)=(u^2+c,v)`.

The verifier checks that the resulting sextic is squarefree, using
polynomial gcd with its derivative. This also follows from E being
nonsingular and `F(c) != 0`: a repeated root would force either `u=0`
or a repeated root of F. Thus H is a geometrically integral hyperelliptic
curve of genus 2. Both H and E are quadratic over their coordinate lines;
the line map has degree 2, so the function-field tower gives
`[K(H):K(E)]=2`. The map is separable and defined over the **same** field.

**Over an extension `GF(p^k)`** the construction is the same, under the
same name (`prime_quadratic_pullback_v1`): nothing in it uses that the field
is prime. The constants `0, 1, 2, 3` are distinct in characteristic
`p > 3`, the cubic still has at most three roots, and squarefreeness is the
same gcd, taken over `GF(p^k)`. The map is defined over the model's own
field `GF(p^k)`, not over `GF(p)`.

This elementary construction is stronger for this one-map question than
the previously discussed genus-at-most-5 result about *two independent*
maps. In particular, prime-field catalog models do admit degree-2 covers.
This does not impose a special CM condition.

## Ordinary binary models

For `E: y^2 + x*y = x^3 + a*x^2 + b`, require `b != 0` and compute
`d = sqrt(b) = b^(2^(m-1))` in its exact polynomial-basis field. Construct

`H: v^2 + u^2*v = u^7 + a*u^4 + d*u`,

with `(x,y)=(u^3,u*v+d)`.

The verifier checks `d^2=b` and the identity

`y^2+x*y-(x^3+a*x^2+b) = u^2 * (v^2+u^2*v-(u^7+a*u^4+d*u))`.

On `u != 0`, put `s=v/u^2`. Then

`s^2+s = u^3 + a + d/u^3`.

The two poles, at zero and infinity, have odd order 3 and nonzero leading
coefficients. They cannot cancel by an Artin-Schreier change of variable;
the quadratic extension is geometrically integral and separable. The
Artin-Schreier genus formula gives `2g+2=(3+1)+(3+1)=8`, hence **g=3**.
The affine model is smooth: the v derivative is `u^2`; at its only zero,
`u=0`, the u derivative is `d != 0`. In the infinity chart
`t=1/u, w=v/u^4`, the equation is
`w^2+t^2*w=t+a*t^4+d*t^7`, which is smooth at infinity.
The coordinate-line map has degree 3; the two quadratic function fields
give `[K(H):K(E)]=3`, also separable in characteristic 2.

This covers the catalog's ordinary binary models, including ECC2K-130.
It corrects any inference that the absence of a useful intermediate
subfield would prevent an ordinary same-field hyperelliptic cover.

### Additional genus-two family when b=1

For `E_a: y^2+x*y=x^3+a*x^2+1`, an additional same-field family is
`C_a: v^2+u^2*v=u^5+a*u^4+u^3+u`, with the degree-three map

`(x,y)=(u^3/(u+1)^2,(u*(u+1)*v+u^2+u+1)/(u+1)^3)`.

The Artin–Schreier poles have orders 3 and 1, giving genus two. Source
infinity and the points above u=1 map to O; (0,0) maps to (0,1).
The [construction report](../../research/binary_genus2_cover_20261014/REPORT.md)
gives the smoothness/degree proof, reduced Jacobian pullback, image inverse,
native checks for both pinned n=83 subgroups, and the partial Lean proof map.
Use [binary_genus2_check](../../tools/binary-genus2-cover/README.md) for that
execution. This separate entry point does not change the catalog constructor's
binary genus-three selection or its historical not-tested fields.

## Interpretation and scope

These elementary same-field constructions establish existence. This geometry
checker performs **no Weil descent to a smaller field**, no subgroup-map
computation, and no relation-solving or discrete-log benchmark. Its graph
fields therefore remain `not_tested`/null. Existence carries no speedup or
vulnerability label. The elliptic factor already accounts for part of the
cover's Jacobian; its existence alone does not establish cheaper computation
there. The separate `hyperelliptic-cover transfer` command evaluates induced
maps within narrower arithmetic boundaries; its results do not retroactively
change the geometry checker's status fields.

The separate generated
[`cover-transfer-assessments.json`](cover-transfer-assessments.json) v2 ledger
is a historical assessment bound to catalog commit
`728304718cb0ab6df70da620a908a5f76f175293`: 15,791 maps and 979 linked EC1
representations. It is not a classification of the live graph below, and later
catalog additions inherit no transfer status from it. Current binary code has
general arbitrary-class pushforward and a structured CLI. Its separate
`transfer-catalog-export` path replays all live binary map rows and named EC1
generators into a hash-bound corpus while keeping annihilator checks distinct
from exact-order evidence. A separate GF(8) oracle now constructs its own
source group from 33,856 principal divisors, all 486 reduced classes, and 233
admissible graph places. It checks all 114,791,256 associativity triples, all
236,196 pushforward-additivity pairs, pullback and both composition identities,
and only then compares every class operation with production. A clean Rust
1.98 replay binds those checks to producer commit
`e0ae0fe5d09c0492a8b50600e88d0b9da68afef1`, its exact raw output, and a
wrapper-level resource receipt. This closes the independent source-law gate
for that one fixed fixture; signed standing, packaged replay, mutation
controls, and a catalog-wide independent oracle remain open. A separate `F_2`
control constructs all 20
source classes independently
from principal divisors, checks all 400 ordered sums and 8,000 associativity
triples, and verifies the independently implemented maps and both composition
identities before comparing with production. That exact tiny-field fixture is
additional independent evidence rather than the basis for the 486-class
result.
The bounded balanced backend handles direct short-Weierstrass quadratic-cover
maps, and their checked projective Montgomery compositions, over
deterministically proved prime fields in the complete `u64` domain and below
the strict Sorenson--Webster `psi_13` bound. It implements arbitrary
balanced-class pushforward, pullback, the deck action, and both degree-two
composition identities. Edwards exceptional-point charts, extension fields,
moduli without a deterministic proof, solver evidence, and every
computational-advantage claim remain unsupported or unassessed as applicable.

The Rust tests enumerate all affine cover points on every nonsingular
short Weierstrass model over F5, F7, F11 and F25 (`t^2 + 2`), on four
models over F125 (`t^3 + t + 1`) and on every ordinary model over F8; every
nonsingular model over F49 (`t^2 + 1`) verifies. Rabin's test over `GF(p)`
counts the monic irreducible polynomials of degrees 2, 3 and 4 over F5,
2 over F7, 4 over F2 and 3 over F3, against the necklace formula. They also reject singular/reducible inputs and altered maps,
coefficients, genus, degree and identity bindings, and replay the entire
catalog deterministically. Point enumeration checks the maps; the genus
and function-field degree arguments above establish the geometric claims.

## References

- Claus Diem, *Families of elliptic curves with genus 2 covers of degree 2*,
  [arXiv:math/0312413](https://arxiv.org/abs/math/0312413), background on
  genus-2 elliptic covers; the elementary pullback proof above is supplied here.
- Arsen Elkin and Rachel Pries, *Ekedahl-Oort strata of hyperelliptic curves
  in characteristic 2*, Algebra & Number Theory 7 (2013), 507-532,
  [doi:10.2140/ant.2013.7.507](https://doi.org/10.2140/ant.2013.7.507),
  Notation 1.1 and the displayed genus formula on page 508.
- Xavier Xarles, *Hyperelliptic curves covering an elliptic curve twice*,
  [arXiv:1303.4220](https://arxiv.org/abs/1303.4220), Corollary 3.
  This broader two-map theorem is context, not the verifier's algorithm.

## Montgomery and Edwards targets

For `B*y^2=x^3+A*x^2+x` in characteristic greater than three, require
`B*(A^2-4) != 0`. Put `s=A/3`, and use the short equation with
`a=(1-A^2/3)/B^2`, `b=(2*s^3-s)/B^3`. The map from short coordinates is
`x=B*X-s`, `y=B*Y`; its inverse is `X=(x+s)/B`, `Y=y/B`.
The checker verifies the coefficient identities explicitly.

For `a*x^2+y^2=1+d*x^2*y^2`, require `a*d*(a-d) != 0` and set
`A=2*(a+d)/(a-d)`, `B=4/(a-d)`. Montgomery coordinates `(u,w)` map to
`x=u/w`, `y=(u-1)/(u+1)`, with inverse `u=(1+y)/(1-y)`, `w=u/x`.
After clearing denominators, the Edwards equation has numerator
`u^2*(a*(u+1)^2-d*(u-1)^2)-4*u*w^2`, which vanishes by
`B*w^2=u^3+A*u^2+u` and the definitions of A and B.
For the scaled Edwards equation, divide x and y by nonzero c first;
this gives a=1 and the coefficient `d*c^4` in the unit-scale equation.

These are rational inverses, hence birational degree-one maps. They extend
uniquely to the smooth projective models; exceptional affine denominators do
not invalidate the cover. Certificates store both maps in `target_model_map`.
The polynomial cover first targets the short equation and then composes with
that map. Genus two and degree two are preserved. Small-field tests exhaust
all nonsingular parameter choices in characteristics 5, 7 and 11 and verify
the forward equations and inverse identities wherever affine formulas apply.

The transfer library now has a typed, everywhere-defined projective
Montgomery/short-Weierstrass isomorphism and composes it with the genus-two
pullback/pushforward API. Adapter tests exhaust point and projective-map
bijections for every smooth parameter choice over `p=5,7,11`, and an
independently coded group law checks one complete `p=11`, `A=3`, `B=2` model.
A complete composed `p=7` fixture checks all 12 Montgomery points, all 72
balanced classes, both compositions, and both homomorphism laws. The
standalone transfer command consumes and returns original
Montgomery coordinates, while the sweeper replays the composed constructor.
The six live Montgomery-normalized maps (seven named bindings) are all
outside the accepted deterministic prime-proof range, so this adds no live
supported binding. The Edwards charts likewise remain geometry evidence
rather than an end-to-end induced-map implementation.

The [standards importer](standards/README.md) expands the inventory, while
[`cover-links.yaml`](cover-links.yaml) supplies exact ICV1/EC1/HC1/CV1 links.
Pass `--links PATH` alongside custom registry/report paths. `--check` replays
both the report and the graph. An unresolved source import is recorded in
`standards/coverage.json` and is not evidence of cover nonexistence.

## 2026-11-18 live catalog inventory

The regenerated live graph binds all 22,417 registry models to 22,417 maps and
22,415 distinct cover models. Its 1,090 EC1 representations remain exact
identity links, not subgroup-transfer certificates.

| Live profile | Maps | Linked EC1 representations |
| --- | ---: | ---: |
| Binary cubic | 174 | 150 |
| Direct odd prime field | 20,808 | 909 |
| Normalized odd prime field | 30 | 22 |
| Odd extension field | 1,405 | 9 |
| **Total** | **22,417** | **1,090** |

A bounded pilot locally replays the five smallest degree-two extension rows
(two over `GF(5^2)`, three over `GF(7^2)`) with implementation-separated
finite-field arithmetic in their exact catalog polynomial bases. It verifies
source smoothness, separability, rational points, fibers, and infinity
geometry, but its graph regeneration uses the repository checker and its
basis comparison is differential against `Fp2Ctx`; it is not an independent
Jacobian implementation. Those five rows have no named EC1 binding; the
remaining 1,400 extension maps retain structured unsupported records. This
pilot therefore changes neither the nine linked subgroup statuses nor the
graph's historical fields.

The separate tracked `odd-transfer-catalog` corpus contains all 22,243 odd
maps and 940 named bindings. Its 17,879,292 canonical bytes have SHA-256
`1ecfabcdaecc3e00d78ab2ce7eebeb94607e1c2c224f261fb5718a4f53b65268`.
It was generated by the clean producer commit
`8c94831426fa35040fa7b38b1379299bf6e6d957` and records the framed source
digest `a5412b92e5e027b281d79c541cc34df130e16d9c71a7034f2065c6e181d8b3ef`
over 29 source files. Its binding partition is 675 supported direct-prime
certificates, 240 unsupported without an accepted primality certificate, 0
kernel-intersection rows, 15 unsupported Edwards normalized-target rows, 9
unsupported extension rows, 1 failed invalid target generator, 0 other invalid
rows, and 0 transfer-arithmetic failures. The seven Montgomery bindings moved
from the historical normalized
bucket to the proof-gated bucket because their composed backend is implemented
but their fields or declared orders lack an accepted certificate; none is
claimed live-supported. Exact same-implementation replay returns the expected
incomplete status and exit `1`; repository source replay binds all 29 source
blobs and the Lean artifact to that commit and exits `0`. The compiler label remains
caller-supplied and unverified, and the local unsigned Lean theorem proves only
its conditional abstract implication. Independent replay and installed-package
replay remain separate release obligations; these receipts do not rewrite the
graph or historical assessment and establish no solver, scaling, or
computational-advantage claim. [pcc:unverified]

The earlier 17,879,352-byte `7c60fc…` corpus from producer `e0ae0fe5…` and
partition 675/233/22/9/1 remains immutable historical evidence.

All graph `subgroup_transport` fields remain `not_tested` with null evidence.
The historical [2026-10-22 catalog refresh](../../research/quadratic_twist_evidence_20261021/reports/catalog-20261022/README.md)
and the dated transfer assessment retain their original populations; neither
is silently relabelled as live evidence. Geometry coverage, general binary
map code, named-generator replay, and bounded direct-prime induced maps are
separate facts. The binary certificate exporter targets all 174 binary rows,
including 150 EC1 bindings on 147 maps and 27 maps without a named subgroup;
its records leave exact order as separate evidence. None establishes an
index-calculus solver, key recovery, or a computational advantage.
