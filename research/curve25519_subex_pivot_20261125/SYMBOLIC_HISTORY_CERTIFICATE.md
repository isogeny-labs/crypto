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

Schema 4 also commits to the local source-bound chain

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
constant `96`. These are exact implications of the stated scalar premises.
In addition, every depth row carries a compact normalized local witness. The
verifier reconstructs

```text
L=2^a,  M=uL,  K=3u,
p_i=34 L (L/2^i)
```

at the first and last coordinates. It then checks the exact identities
`KL=3M`, `tau D=2L`, `nD=6M`, `pu=34MD`, and
`npu=204M^2`. Because `p_i` is reconstructed from `D_i` and `tau_i D_i` is
fixed, these endpoint commitments define the same closed-form witness over the
whole coordinate range. They show that the scalar obligations are mutually
compatible and bind concrete `M,K,L,tau,p` values; they still do not prove that
a graph or packing realizes the witness. `[pcc:unverified]`

Schema 4 also charges the schema-2 factorization-certificate description
envelope. After the tuple count `T` has been charged, every remaining scalar,
coefficient-width, container, and framing factor in the JSON and canonical
binary bounds is covered by the deliberately coarse exponent

```text
Delta_d = 2 L_d + 16 (lambda + a_d + u_d + 1).
```

Here `log2(S)<=2L+1`, `d=4lambda`, `M<=s<=2^lambda`, and the checked source
bounds give `Q<=204M^2/u`; no description term contains more than one factor
of `T`. Thus `log2(description bits)<=log2(T)+Delta_d`. For an active node,
`M>s/h` and `s>2^(lambda-1)`, where `h=floor(sqrt(lambda))`. The verifier
checks at every retained depth that

```text
Delta_d h ceil(sqrt(u_d)) <= 2^(lambda-1),
```

so `Delta_d<=M/sqrt(u_d)`. The local ledger is now exactly
`80+1+4+5+6=96`: tuple count, description overhead, active nodes,
interpolation samples, and radix. The global constants therefore remain
unchanged. This is an arithmetic replay of the description formulas, still
conditional on the source and active-node premises. `[pcc:unverified]`

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

The newline-terminated schema-4 JSON fixture is 7,355 bytes. The native command

```text
cargo run --locked \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin symbolic-history-certificate-check -- \
  crates/hashed-ecdl-tensor/examples/symbolic-history-certificate.json
```

reports five active depths, ten normalized endpoint witnesses, description
overhead constant `1`, root `u=512`, total root-`u` exponent constant `290`,
root-log constant `148480`, and tuple exponent constant `80`. The companion
native emitter deterministically regenerates the fixture. Verifier input is
capped at 1 MiB.
`[pcc:unverified]`

## Remaining refinement boundary

This transcript certifies scalar implications and one normalized dimension
family satisfying them. It does not show that a symbolic graph realizes those
`M,K,L,tau,p` values, that packing and factorization objects correspond to the
normalized rows, or that the external contraction theorem applies. The next
useful certificate layer must define a graph/packing realization interface to
the schema-2 factorization obligations while keeping the external theorem an
explicit independent premise. An independent or kernel-checked derivation of
the description-composition inequality also remains open. `[pcc:unverified]`
