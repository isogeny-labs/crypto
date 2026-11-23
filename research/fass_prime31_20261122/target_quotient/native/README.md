# Native target-quotient correctness qualifier

This standalone crate implements the finite checks in the frozen
[`../PROTOCOL.md`](../PROTOCOL.md), SHA-256
`3eafeab5c2dc73c0add918bb23c9e8cdac1cf82d567d18a49792fd84aee0b20f`.
Its scope is development correctness for the selected fields, curves, selector
orders and public targets. Dense products and root grids are correctness
oracles; no timing or solver-performance claim is made. [pcc:unverified]

## Code reuse

`main.rs` includes the unchanged published
`src/cryptanalysis/semaev_prime_poly.rs` at base
`f0ca664eca0081a30adfcc79dcf62a32de40d2af`. Field operations, scalar S3,
univariate modular multiplication, remainders, gcd, powering, derivative and
interpolation all use that module. Its SHA-256 is
`c8249ba825c946dbdb37fa5d6762f8f32d15320344693443e8af0d52f122784a`.

`oracle.rs` copies the previously published product and matrix helpers from
`coupled/native/main.rs` (SHA-256
`585172d4871b3b55591d1562eca95fb93ff44f9d08d0fb1bf92f2138f7e663ee`),
with adjusted visibility and imports. No frozen `coupled/` file is edited.
The standalone lock pins `rand=0.8.8`, `serde=1.0.229` and
`serde_json=1.0.151` to the repository's versions.

## Objects checked independently

For all selector roots `V`, the qualifier first constructs the canonical
coefficient matrix of `H(x,t)` with `x^d=1`. It keeps these later objects
separate:

| Object | Construction and interpretation |
| --- | --- |
| Full right polynomial `R_V` | Product of `S3(t,z,X)` over every `z` in `V`, retaining multiplicities and nonrational factors. |
| Rational squarefree right polynomial `G_X` | Monic `gcd(R_V,t^p-t)`, obtained with the published modular powering and gcd. |
| Canonical rational remainder | Each `t` coefficient row of `H` reduced modulo `G_X`. |
| Value matrix | Values of the remainder at all `x` in `V` and all distinct roots of `G_X`; its rank must equal the canonical coefficient rank. |
| Boolean support matrix | Entry one exactly where the value is zero; its rank is recorded separately over the same field. |
| Liftable-left row restriction | The value and Boolean matrices restricted to selector coordinates that lift to the curve, including 2-torsion. This is a separate row set. |

The rational root set is also constructed as the union of roots of each
degree-at-most-two factor. F97 additionally checks that union exhaustively
against all field elements. For the coefficient remainder, an independent
path multiplies and reduces univariate S3 factors at each selector `x`, then
reconstructs the coefficient matrix using interpolation basis polynomials
returned by the published implementation. These checks do not rely on any
observed rank being maximal.

The output separates multiplicity loss `deg gcd(R_V,R_V')` from the degree of
discarded nonrational squarefree factors. It records target/selector collisions,
2-torsion, degree drops, repeated quadratic factors and root-fiber collisions.
Target categories are retained even when two categories select the same point.

When the target coordinate belongs to `V`, S3 symmetry gives the additional
zero-row check `H(X,t) mod G_X = 0`. The corresponding rank bounds are tested
on both the complete selector grid and the liftable-left restriction. These
are structural bounds, not rank-saturation assumptions. [pcc:unverified]

## Controls and finite targets

The two ordinary fixtures and their selector orders are exactly those in the
protocol. Public targets are the first three non-2-torsion affine points found
by increasing integer abscissa, with the smaller square root as ordinate;
the first liftable sorted selector root is an additional collision category.
F97 also includes `(30,0)`. No scalar is generated or supplied to this code.

The nonsingular rank-one curve `y^2=x^3-x` uses `d=2` and the prescribed three
target categories. Its full polynomial is checked coefficient by coefficient
against `16*t^2`, before recording whatever rank its target quotient has.

For each field and selector order, the cusp specialization checks the unique
positive `k` in `{1,max(1,floor(d/2)),d}`. With the first `k` sorted selector
roots, the monic right polynomial is checked against `t^(2k)`. The first
`r=min(d,2k)` coefficient columns and the prescribed selected-row determinant
are checked. Its radical is separately checked as `t`, with rank one; these
two quotient ranks are not identified with each other. [pcc:unverified]

The empty-subset control has right polynomial one and a zero-dimensional
coefficient remainder. A point-at-infinity target is explicitly flagged as
outside the qualified affine chart. Algebraic support counts on either row
set are not counts of verified signed curve relations.

## Producer evidence and reproduction

[`../evidence/producer/attempt1/`](../evidence/producer/attempt1/) contains
the complete source/dependency snapshot copied before compilation, build
manifest and output, pre-call executable hash, separate runtime stdout/stderr,
exit status and post-call hashes. The snapshot sources were made read-only;
the executed binary was separately copied and made read-only before its hash
was recorded. Build and execution each used a 60-second timeout, 2 GiB virtual
address-space ceiling and one build job.

The native output reports 91 finite targets, 52 cusp minor controls, 20 empty
controls, 91 rational root-set and rank equalities, 35,391 quotient-grid
comparisons and independently reconstructed coefficient comparisons, and
4,171 exhaustive F97 membership checks. The single build and qualifier call
both returned zero; no corrected retry was used. These are unsigned producer
observations pending the separately recorded review. [pcc:unverified]

For a new reproduction, preserve a new source snapshot and a new receipt
directory. A build from the ordinary source directory is:

```sh
timeout 60s sh -c 'ulimit -v 2097152; exec cargo build --release --offline --locked --jobs 1'
```

Copy the resulting executable to a new immutable path and record its SHA-256
along with the protocol, source and lock hashes before calling that copy with
the same timeout and memory limit. Capture stdout, stderr and exit status in
distinct new files. The stored producer build instead points Cargo at its
frozen source snapshot and sends build products to `native/target/attempt1`.
Preserve failed calls; do not replace pre-call manifests. Full-repository
Cargo validation is a distinct gate from this exact-module qualifier.

## Fixed-degree remainder certificate

`certificate/` is a separate crate that includes the unchanged published
identity-certificate and SHA-256 modules. It evaluates the single identity

\[
h_6(T)=1-12T+66T^2-220T^3+T^4q(T)
\]

over `2^61-1`, where the left side is built from the quadratic norm recurrence
and the explicit quotient on the right comes from the closed binomial
coefficients. This is a fixed `d=6,k=2` cusp remainder witness, with degree
bound 12 and `proof: null`; it does not certify the generic minor or radical
quotient theorem.

The certificate CLI preserves the existing repository schema. Its `issue`
mode refuses an existing output path, checks producer extra points, rejects
a right side changed by one, and rejects an altered value digest. Its `check`
mode accepts a separate decimal checker seed.

The retained producer record is
[`cusp-d6-k2-remainder.identity.json`](../evidence/producer/certificate-attempt1/cusp-d6-k2-remainder.identity.json),
ID `IDC1h7e169f28f6a50bb6`. The original checker accepted 32 issuer and 32
producer extra-stream points, and both tamper controls were rejected. Build
and execution returned zero under the same bounds. These are unsigned
evaluated checks, with `proof: null`. [pcc:unverified]

```sh
fass-target-quotient-certificate issue NEW_CERTIFICATE_PATH
fass-target-quotient-certificate check CERTIFICATE_PATH DECIMAL_CHECKER_SEED
```

Use the same frozen-source, pre-call, timeout and memory procedure for these
calls. An independent replay has its own receipt; it does not rewrite the
producer certificate's authorship field.
