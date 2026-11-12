# Prime-field norm/rank search audit

No novel sub-exponential ECDLP algorithm was obtained. Read [REPORT.md](REPORT.md),
[DERIVATION.md](DERIVATION.md), or the [three-page PDF](report/report.pdf).
The classical norm representation isolates an unresolved constructive rank-one
search. A Lean counterexample rejects its unrestricted linear relaxation
[pcc:unverified].

## Reproduce the formal checks

With Lean4.35.0 and the pinned lake-manifest dependencies available:

```sh
lake -d research/prime_norm_rank_20261112/lean build
lake -d research/prime_norm_rank_20261112/lean env leanchecker --fresh --verbose NormRank
```

The existing identity issuer is included unchanged by a small standalone Rust
adapter. It performs certificate checks, not a discrete-log experiment:

```sh
cargo run --release --locked --manifest-path research/prime_norm_rank_20261112/certificate/Cargo.toml -- research/prime_norm_rank_20261112/lean/NormRank.lean /tmp/norm-defect.identity.json
```

Original logs retain failed attempts alongside successful receipts. These are
unsigned producer evidence; certificate checking does not itself execute Lean.
No empirical scaling, new algorithm or independent signed standing is claimed.

The prior nonscalar-origin and rank-directed jet-completion suggestions remain
unproved research proposals. Preserving interpolation node values does not by
itself establish a useful crossing-rank bound or a sub-exponential total cost.
This separate norm audit does not establish either suggestion.
