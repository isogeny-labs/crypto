# Strong-rho control for the GHS-weak plane-quartic frontier

Status: implementation and protocol frozen; measurement pending. There is no
measured result, frontier change, or leaderboard row yet.

This study reopens the restricted `E0/F_{q^3}` plane-quartic program for one
specific obligation: compare its end-to-end index-calculus result with a rho
reference that combines the negation map, lockstep walks, Montgomery batch
inversion, and six-product cubic-field multiplication. It preserves the frozen
N4 and N4-v2 sources and runs their historical rho in the same driver as a
control.

The preregistered primary is `strong-b8`. A development-only width pilot fixed
that choice before paid data: wider batches saved inversions but increased
collision work, so batch 64 was not retained as the reference
`[pcc:unverified]`. The walk also applies the frozen rho-neg look-ahead rule and
escapes residual cycles by doubling the cycle's canonical minimum.

The scope remains public synthetic known-answer instances in the special GHS
family. It says nothing about generic, standard, or deployed curves. The
protocol, EKS request, benchmark records, visual report, validation receipt,
and any dashboard update will be added before a performance conclusion is
made [pcc:unverified].

Development checks:

```sh
cd research/quartic_ghs_strong_rho_20261114
rustc --edition 2021 -O main.rs -o /tmp/ghs-strong-rho
/tmp/ghs-strong-rho selftest
/tmp/ghs-strong-rho run 223 1 0 2 ic-repeat old-neg,strong-b4,strong-b8,strong-b16
```

`ic-repeat` is the primary policy: every independent rho walk uses the exact
target consumed by the frozen N4-v2 IC run for the same `(q, seed)` pair.
`sequence` retains that target at index zero and emits deterministic holdouts
after it for correctness-only sensitivity checks.

Sources checked before implementation:

- `research/quartic_n4_20261019/{PROTOCOL.md,README.md}` — frozen historical
  pipeline and serial rho.
- `research/quartic_n4v2_20261020/{PROTOCOL.md,README.md}` — negation-map rho,
  exact family, and unbatched-inversion limitation.
- `research/ghs3_vs_rho_20261023/{PROTOCOL.md,TABLES.md}` and
  `src/cryptanalysis/jv_rho_strong.rs` — existing same-repository strong-rho
  control design.
- Diem (ANTS VII, 2006), Diem–Thomé (J. Cryptology 21, 2008), and
  Gaudry–Hess–Smart (J. Cryptology 15, 2002), bibliographic records in
  `papers/ecdlp-index-calculus-program/refs.bib`.

KB receipt: the research-kb MCP was not exposed and the wrapper found no
executable. The relevant lead-atlas YAMLs were visible but vault-encrypted in
this runtime. No absence or novelty claim is based on that failure.
