# Strict-root / symbolic-history composition boundary

The schema-1 composition-boundary certificate independently replays the
strict symmetric root transcript and the implicit symbolic-history transcript.
It proves the facts those objects actually share and rejects stronger claims.
It is not a graph-realization certificate, an independent proof of the
external contraction theorem, or a practical Curve25519 attack.
`[pcc:unverified]`

## What composes

At the first history threshold `lambda=2^26`, both transcripts reconstruct

```text
lambda = 67,108,864
u      = 512
a      = 1,024.
```

The symmetric source has exact child size `M/32`. The symbolic history needs
only the upper bound `M_child<=M/8`, so the source bound implies the history
bound. The standalone verifier checks these statements from the two serialized
objects rather than trusting copied constants. `[pcc:unverified]`

## What does not compose

The objects use deliberately different dimensions:

```text
strict root:          M = 2^lambda,       K = 2^(lambda-a), L = 2^a
normalized exemplar: M_norm = u*2^a,     K_norm = 3u,       L = 2^a.
```

The normalized row witnesses that the local scalar inequalities are mutually
consistent. It is not the strict root and cannot be substituted for it. At the
threshold the verifier reports `normalized_root_edge_factor=512` and
`normalized_root_packets=1536`, versus strict-root exponents `67,108,864` and
`67,107,840`. Schedule equality therefore does not imply dimension equality or
realize the recursive graph. `[pcc:unverified]`

The wire format makes six assertions explicit. An accepted certificate has
schedule compatibility and source-bound implication `true`; normalized/root
dimension equality, strict-recursion graph realization, independent external
theorem verification, and a certified subexponential total-work conclusion
are all `false`. Mutating any unsupported conclusion to `true` fails closed.

## Native replay and controls

```text
cargo run --quiet --locked \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin composition-boundary-certificate-emit |
cargo run --quiet --locked \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin composition-boundary-certificate-check
```

The checked transcript is 8,318 bytes and reports
`status=conditional_premises_only`. Five focused tests cover the accepted
boundary, valid-but-mismatched component schedules, source schedule/shrink
mutations, history schedule/shrink mutations, and all four unsupported
promotions. The verifier imports only the two standalone component verifiers;
it imports no graph, packing, contraction, or producer implementation.
`[pcc:unverified]`

## Next measurable goal

Do not try to make the normalized exemplar equal the root. The next useful
bridge is a **parametric one-step refinement certificate**. It should take
symbolic `M,K,L,D,delta,u,a` plus independently checked premise tokens and
derive, without allocating the root:

1. `KL<=3M`, the normalized precision and tuple-count bounds;
2. source class/load caps and `M_child<=M/8`;
3. preservation of the next-call degree and fallback disjunction; and
4. the local exponent ledger used by the symbolic history.

The optimization target is binary and reviewable: one independent checker must
accept the strict symmetric root's symbolic premises and the generic child
premises through every retained depth, while mutations of every premise edge
fail. Passing that target would close the local parametric interface, not the
external Theorem D.1 or its asymptotic running-time claim. `[pcc:unverified]`
