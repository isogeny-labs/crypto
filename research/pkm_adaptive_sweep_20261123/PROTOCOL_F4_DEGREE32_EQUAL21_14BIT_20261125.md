# Equal-21 shifted interval control for the 14-bit PKM map

Date: 2026-11-25. This is an amendment to
`PROTOCOL_F4_DEGREE32_14BIT_SINGLE_CURVE_20261125.md`, not a rewrite of its
15/15 seed-7 measurements. Its original size screen covered `lo=0..64`.

## Selection, frozen before timing

The fixed roster curve is `icv1-fp14-tm43-158b6914`, with
`p=16187, a=1, b=12` and prime order 16231 (registry row in
`docs/curves/registry.json`). The original degree-32 auxiliary map
`prime-isogeny:t=5,seed=5,aux=ordinary` has 21 carrying abscissae. Scan all
`lo=0..p-1` in ascending order for the first 32-abscissa cyclic interval
whose `y^2=x^3+ax+b` right-hand side is a quadratic residue or zero at
exactly 21 positions. The first match is `lo=118`; the full scan found 465
matches. This selection uses membership and size only, before any solver or
target performance result. The executable preflight must confirm both bases
have 21 abscissae on the same instance before timing.

Control: `prime-interval:t=5,lo=118` with
`pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=118,mode=chain`.
Candidate: `prime-isogeny:t=5,seed=5,aux=ordinary` with
`pkm-semaev:m=2,engine=f4,degree=8,mode=chain`.

The retained binary is
`/Volumes/SSD990-2/.codex/tmp/pkm-1ae5-target/release/ic`, SHA-256
`1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08`.
`SOURCE_BUILD_RECEIPT_PRIME_INTERVAL_20261125.md` pins its source and the
`prime-interval` registration. The earlier `a5503f4` binary cannot run this
control because its `ic bench` dispatch does not register `prime-interval`.

## Preflight and continuation

Run `equal21_preflight.py --bin <retained-binary>` once. It writes an
append-only JSONL header, deterministic full-shift scan receipt, and both
one-repeat subtraction preflights. Require the same instance and order,
21 abscissae in each arm, and no skipped configuration. Preserve every
command, exit code, full JSON, and stderr, including failed preflights.

Only after that gate, use target seeds `20261301..20261303`, 16 repeats per
invocation, interval/candidate/candidate/interval mirrors, 1,000 trials,
five-second F4 call cap, one matched rho run and 900-second outer cap. Each
invocation is cold and serial under the quiet-load checks of the source-pinned
14-bit protocol. Append failures and censoring. Compare each mirror's charged
GAE, wall, CPU, peak memory, relation attempts, rank and linear algebra, and
verified target scalar. This one-curve panel is a component comparison; it
cannot promote the map under the two-size, three-curve cold-cost goal.

Sources checked: `src/cryptanalysis/ic_framework/pkm_semaev.rs` defines the
exact shifted 32-abscissa interval and matching bits oracle;
`src/bin/ic/bench.rs` registers that base in the retained build. The prior
single-curve protocol above and `SOURCE_BUILD_RECEIPT_PRIME_INTERVAL_20261125.md`
explain the original limited scan and binary identity.
