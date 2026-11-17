# Native Vieta / smooth-coset decomposition

The [half-GCD/rational-tower continuation](next-20261116/RESULTS.md) implements
recursive GCD batching, general homogeneous rational maps, full-fiber certification
and normalized Vélu maps. Its240-call main panel replayed completely. Half-GCD
reduces word62/m16384 multiplication counts by a paired factor1.4437 against
Newton/Euclid, but regresses P224/m2048 and all measured rational cases. See the
[new report](next-20261116/report.pdf) and separate [degree audit](next-20261116/DEGREE_AUDIT.md).
Data/theory standing remains unverified [pcc:unverified].

The [16 November continuation](continuation-20261116/RESULTS.md) adds opt-in
Karatsuba/Newton backends and a matched panel through m16384. The conditioned
word62/m16384 multiplication-count ratio is2.1469 against schoolbook; Newton
division did not engage in the successful study queries. See the
[report](continuation-20261116/report.pdf) and the conditional hand extension
to [split rational towers](continuation-20261116/SPLIT_TOWER_BOUND.md).
All observations and hand arguments remain [pcc:unverified].

The Rust port merged in [PR 4340](https://github.com/isogeny-labs/crypto/pull/4340).
The authorized local scale matrix is complete: see [RESULTS.md](RESULTS.md), the
[scale report](reports/local-scale/report.pdf), and the proposed
[last-fall-degree bound](LAST_FALL_BOUND.md). These are producer observations
and a hand argument, respectively, pending independent signed standing
[pcc:unverified].

The Rust library module `src/cryptanalysis/vieta_coset.rs` implements the
non-interpolating two-point extractor discussed in the user request. It is
exported from `crypto_lib::cryptanalysis::vieta_coset`. The small harness here
compiles **that exact source**, not a second solver. Producer checks and claims
remain [pcc:unverified] pending an independent signed verdict.

```sh
cargo test --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml
cargo run --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml \
  --bin vieta-coset-lab -- check p224 128 2 120
cargo run --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml \
  --bin vieta-coset-lab -- check word17 8 2 30 three > /tmp/vieta-check.jsonl
cargo run --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml \
  --bin vieta-coset-replay -- /tmp/vieta-check.jsonl
```

`check` emits no timings. `bench` has the same arguments and additionally needs
an admitted launcher setting `VIETA_ISOLATED_RUN=1`; that variable is an intent
guard, **not isolation certification**. Do not set it to bypass the protocol.
Every stage has operation/time limits; failures emit rows and a nonzero status.
`vieta-coset-analyze FILE...` summarizes retained rows with descriptive quartiles.
The separate replay binary recomputes two-term answers via BigUint scalar
subtraction and three-term answers via scalar pair-table enumeration. It does
not confer independent signed standing.

Development coverage includes exhaustive rational targets on three small curves,
two coset representatives (not assumed distinct cosets), identity/two-torsion/diagonal cases, word/wide agreement, P-224 known
answers, and explicit invalid-input and root-budget rejection. Larger development
checks use m=4096 on a 62-bit prime and m=1024 on P-224; they establish output
agreement, not performance scaling. See [PROTOCOL.md](PROTOCOL.md) for exact
scope and cost boundaries. The local amendment records the subsequent explicit
backend/checkpoint authorization.

The arithmetic backends here are a counted generic reference: u128-reduced u64
and native Rust BigUint. Dense polynomial operations use schoolbook arithmetic.
The library's optimized Montgomery/binary-GCD backends and an external polynomial
backend still need matched measurements before a fastest-method conclusion.
The m=2^d lift does not bound the elimination or last-fall degree by two.
The separate hand argument gives a proposed 2*log2(m)+2 upper bound for this
exact two-term generating system, not a solver running-time bound.

The original EKS `protocol.json` remains a historical draft. User-authorized
local protocols were frozen before calls; counts and below-L2 diagnostic clocks
are retained through m=8192 over fields up to 224 bits. Twelve Vieta deadlines,
the original missing three-term calls, and the separately frozen fixture repair
are all preserved. No cloud spending or canonical leaderboard promotion occurred.
