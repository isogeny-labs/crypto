# Guarded summation-polynomial fall degrees

This study records ordinary polynomial-ring degree bounds for an explicit
membership presentation. It contains mathematical derivations, native proof
replay, historical source snapshots, and a complete Lean proof of the ordinary
guarded three-abscissa prime-field pair `(4,5)`. It makes
no timing, ECDLP recovery, sub-rho, or literature-priority claim.

Use `k=F_p`, `p>3`, for the short Weierstrass statements and `k=GF(2^n)`
for the binary statements. Let `E/k` be nonsingular and let `R` be an affine
`k`-rational target, with `r=x(R)`. For a set `S` of `B` distinct abscissas
of affine `k`-rational points, write `g(T)=product_{s in S}(T-s)`.
The inputs are

```text
g(x), g(y), g(z), S3(x,y,u), S3(u,z,r), v*(x-y)-1.
```

Unsatisfiable means no `k`-solution, equivalently no algebraic-closure
solution under these rational-lift hypotheses. The last equation requires
distinct first-pair abscissas. Both point signs
above each abscissa are allowed. Degree means ordinary total degree with
explicit inputs, not degree after implicit Boolean reduction. `d_ff^ord`
below is the homogeneous-top syzygy degree modulo Koszul syzygies; it is
not automatically the operational first-fall convention of the existing
Weil-descent ledger.

| Setting; unsatisfiable instances | Derived first fall | Derived last fall |
|:--|:--|:--|
| Short Weierstrass curve, characteristic greater than 3, `B=3` | `4` | exactly `5` |
| Same presentation, `B>=3` | `B+1` | `max(5,B+1) <= d_F <= 3B-4` |
| Ordinary binary curve, `B=3`, `g(T)=T^3+c` | `4` | exactly `4` |
| Ordinary binary curve, `B=3`, all other split cubics | `4` | exactly `5` |
| Ordinary binary curve, `B>=4` | `B+1` | `B+1 <= d_F <= 3B-4` |

The binary model is `Y^2+XY=X^3+a2*X^2+a6`, with `a6 != 0`.
The cube-orbit exception requires even extension degree; over `GF(2^n)`
with odd `n`, every unsatisfiable three-abscissa instance has pair `(4,5)`.
The first-fall column is the stated **ordinary-ring** convention. Do not
identify it with a finite-field truncated syzygy definition when truncation
changes the relevant degree.

A separate direct theorem covers the fully Boolean binary chain: encode
`x,y,z` by three linear `ell`-bit maps into `GF(2^n)`, encode the single
shared `u` (and optional guard variable `v`) by `n` bits each, and include
every bit equation `b^2+b`. For every unsatisfiable instance, with or without
the inverse guard,

```text
d_F <= 3*ell+1,  ell>=1.
```

This bound is independent of `n` and needs no rational-lift assumption.
Fixing the base bits leaves an affine system in the auxiliary bits; staged
assignment indicators give the degree bound. Proof size may be exponential.
It establishes an upper bound for this exact encoding, with no first-fall
equality or growing lower bound.

A separate prime-field result uses Boolean-constrained variables with
coefficients still in `F_p`. For normalized lookup inputs whose base images
have rational point lifts, surjective auxiliary bit encodings give
`d_F <= 3*ell+2`. Raw inputs instead give
`d_F <= max(3*ell+2,d_raw)`. This is not a `GF(2)` carry-circuit model;
lookup size and proof size can be exponential. See the
[exact prime-field bit-restricted theorem](extensions/large_curves/prime_bit_restricted_upper.txt).

## Proofs and executable checks

- [Degree-five proof and matching lower bound](proofs/universal_five_symmetric_audit.txt).
- [Growing-cardinality proof](extensions/growing_b/proof.txt) and
  [binary bridge](extensions/growing_b/binary_bridge.txt).
- [Characteristic-two proof and classification](extensions/characteristic_two/extension_proof.txt).
- [Fully Boolean binary upper bound](extensions/characteristic_two/boolean_upper_bound.txt).
- [Large standard-curve applications and exact replay](extensions/large_curves/README.md):
  four public known-answer fixtures over 256-, 384-, and 521-bit prime fields.
- [Native prime-modulus DAG replay](native/README.txt): exact sparse polynomial
  arithmetic with strict input binding; no search and no Python execution.
- [Native binary-field checker](extensions/char2_compute/native/README.txt):
  genuine polynomial-basis `GF(2^m)` arithmetic, not integers modulo `2^m`.
- [Complete Lean exact-pair proof](formal/complete-exact-pair.md):
  `GuardedExactPair.exact_pair_prime` proves the original prime-field `(4,5)`
  statement with attainment and minimality. It constructs the degree-five
  refutation, proves the rationality/unit-ideal bridge, and defines and
  computes the actual weighted homogeneous-top/Koszul first fall. The
  [proof map](formal/PROOF_MAP.json) binds the theorem and dependencies to
  pinned sources and verification receipts. Growing-base, characteristic-two
  and Boolean results retain their separate formalization status.

From the repository root:

```sh
cargo test --locked --manifest-path research/guarded_fall_degree_20261028/native/Cargo.toml
cargo run --locked --release --manifest-path research/guarded_fall_degree_20261028/native/Cargo.toml --bin fall-degree-replay -- verify research/guarded_fall_degree_20261028/evidence/p101_problem.json research/guarded_fall_degree_20261028/evidence/p101_upper_D5.json
cargo test --locked --manifest-path research/guarded_fall_degree_20261028/extensions/char2_compute/native/Cargo.toml
```

The binary crate includes one pinned, already-proved cube-orbit regression.
It is a development correctness test, not a new research sweep. No EKS
experiment or performance measurement was run for these extension proofs.

## Larger curves

The ordinary-ring theorem already quantifies over every prime `p>3`.
The native implementation now also supports arbitrary-precision coefficients.
Fixed public fixtures on secp256k1, secp256r1 (P-256), secp384r1 (P-384), and
secp521r1 (P-521) have separately rechecked curve witnesses and independently
replayed degree-five refutations. Their first/last pair `(4,5)` follows from
the stated theorem; no new degree-four closure computation is claimed.

The ordinary binary theorem likewise allows arbitrary `n`. In particular,
the standard odd binary field sizes 163, 233, 239, 283, 409, and 571 exclude
the cube-orbit exception. This extension supplies numerical certificates for
the four prime-field fixtures, not those large binary curves. Increasing the
field size preserves the fixed five-variable, three-abscissa presentation;
the fully Boolean and growing-cardinality models retain their separate bounds.

## Connection to the existing fall-degree work

The existing [fall-degree ledger](../notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md)
and [E12 continuation](../fall_degree_bounds_20261008/e12/completed-20261028/README.md)
concern **Boolean Weil descent**. The latter records labels `>=5,4,>=5,4`
for the completed cell, a frozen lower median of `4`, and an independent
closure-replay gap. Those records are preserved and are not promoted by this
study.

Here `B` is cardinality and the membership input has degree `B`. A subspace
base of dimension `ell` usually has cardinality `2^ell`. The growing lower
bound therefore depends on the explicit membership degree. It proves no
linear lower bound in the number of variables of a fixed-degree Boolean
encoding. Those lower bounds still have no applicable transfer theorem.
The direct Boolean upper bound above uses a separate affine-specialization
argument; applying it to E12 requires checking that its actual encoding
matches the stated generators and auxiliary blocks.

## Evidence and historical provenance

The compact public input/proof files are in [evidence/](evidence/). The
native checker replays the recorded polynomial operations and rejects any
intermediate degree overflow, changed input, invalid DAG reference, or
non-unit goal. It does not certify the universal theorem, primality, curve
semantics, or the historical lower-space rank.

`historical/python/` preserves the earlier execution sources as evidence.
They are not the supported execution path, are not invoked by the native
tools or CI, and were not run after the repository's native-code requirement
was discovered. Historical verification claims remain labelled historical.

See [sources](SOURCES.md), [requirements and status](STATUS.md), and the
[report and dependency diagram](reports/REPORT.md). The leaderboard and
performance dashboards are unchanged because this study reports no measured
pipeline or component speed improvement.
