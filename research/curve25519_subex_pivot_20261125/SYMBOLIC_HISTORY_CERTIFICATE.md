# Implicit symbolic-history certificate

The allocated `usize` backend cannot enter the source's strict recursive
branch. This note records the replacement finite boundary: a schema-versioned
scalar transcript of the asymptotic source schedule, checked without allocating
a graph. It is not a graph-realization certificate or a proof of the external
contraction theorem. `[pcc:unverified]`

## Producer and verifier separation

`admission::dimensions::symbolic_history_certificate` serializes the existing
BigUint history and recursion-composition audits. The standalone
`symbolic_history_certificate` verifier imports neither `admission::dimensions`
nor contraction, packing, deformation, factorization, or graph code. From the
plain fields it independently reconstructs:

1. `h=floor(sqrt(lambda))` and `u_0=floor(lambda/2^17)`;
2. every possible active depth with `8^d<h`;
3. `u_d`, prior and current packet exponents, and exact shrink factors;
4. inherited-precision and evaluation-loss absorption inequalities;
5. `u_0<=4^(d+1)u_d` at every retained depth;
6. leaf-scale and packet-entry absorption; and
7. the reported constants `96`, `3`, `288`, `290`, and `148480`.

Schema 2 also commits to the local source-bound chain

```text
K L <= 3M,
tau D = 2L,
n D <= 6M,
p u <= 34 M D,
n p <= 204 M^2/u,
log2(T) <= 80 M/sqrt(u).
```

The verifier checks the constants `3,6,34,204`, proves
`14^2<204<=15^2`, and reconstructs `80=5*15+5`. It then checks that tuple,
active-node, interpolation-sample, and radix terms fit the declared local
constant `96`. These are exact implications of the stated scalar premises;
the certificate does not prove that a graph realizes them. `[pcc:unverified]`

Missing, extra, reordered, or altered depth rows fail closed. Schema,
precision-transition, and final-composition mutations are explicit negative
controls. `[pcc:unverified]`

## Stable threshold fixture

The checked fixture uses the first claimed uniform-history threshold
`lambda=2^26`. Its five depth rows are:

```text
depth     0     1     2     3     4
8^depth  1     8    64   512  4096
u_d     512   256   128    64    32
a_d    1024  1536  1792  1920  1984
```

Each depth row commits to the first and last tile/distance exponents. The
verifier checks both endpoint identities
`1+a = u+(a-u+1) = a+1`, which proves `tau D=2L` uniformly for every
coordinate between them.

The newline-terminated schema-2 JSON fixture is 2,325 bytes. The native command

```text
cargo run --locked \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin symbolic-history-certificate-check -- \
  crates/hashed-ecdl-tensor/examples/symbolic-history-certificate.json
```

reports five active depths, root `u=512`, total root-`u` exponent constant
`290`, root-log constant `148480`, and tuple exponent constant `80`. Input is
capped at 1 MiB.
`[pcc:unverified]`

## Remaining refinement boundary

This transcript certifies scalar implications of declared source premises. It
does not show that a symbolic graph realizes `M,K,L,tau,p`, that the packing
and factorization objects correspond to those scalars, or that the external
contraction theorem applies. The next useful certificate layer must commit to
per-node symbolic dimensions and connect them to schema-2 factorization
obligations, while keeping graph realization an explicit independent premise.
It must also include certificate-description bits in the complete depth-first
composition. `[pcc:unverified]`
