# 14-bit rung transfer test: degree-32 t=5 vs equal-15 interval — 2026-11-25

Frozen before any timed call. This is the 14-bit gate that
`RESULT_F4_DEGREE32_SOURCE_PINNED_EQUAL18_20261125.md` lists as open
("transfer to 14-bit"). It tests one thing only: whether the degree-32
construction keeps its equal-size relation-extraction component lead when the
field doubles, or whether the 13-bit ratio was a small-field effect.

## Binary

`ic` SHA-256 `c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`
(`/Volumes/SSD990-2/.codex/tmp/pkm-a550-target/release/ic`), built from commit
`a5503f4bb66ac3aa4eac3125fba01534d2574096`. Its JSON `software.git_commit` is
`null`, so the SHA-256 pins execution and the commit pins the source tree, but
the binary does not self-report a commit.

## The 14-bit rung has exactly one roster instance

A size-only probe of CLI seeds `20261300..20261400` (101 seeds, preflights
only) returned the same instance, `icv1-fp14-tm43-158b6914`, for every seed:
`--seed` selects the planted scalar, not the curve, at this rung. A
three-instance 14-bit panel is therefore not constructible from `--seed`
alone. This protocol is a **single-instance mirrored panel with 16 distinct
scalars**, and it must not be described as instance replication. Ledger:
`scan14_roster_instances.jsonl`
(SHA-256 `5c86f8033adff9bda946d700b4f8c289fd7aacc7c2cc95e99346200d35c6744d`).

## Frozen size selection

Same method as the 13-bit panel: fix the interval first, then take the first
ascending auxiliary seed at `t = 5` whose abscissa count equals it.

- interval: `prime-abscissa:below=32` → **15** abscissae,
  oracle `pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain`.
- auxiliary search `t=5, aux=ordinary, seed 0..24` →
  `13, 23, 18, 17, 16, 21, 19, **15**, 19, 12, ...`; the first exact match is
  **aux seed 7** (degree 32, five degree-2 Vélu steps).
- candidate: `prime-isogeny:t=5,seed=7,aux=ordinary`,
  oracle `pkm-semaev:m=2,engine=f4,degree=8,mode=chain`.

Both arms are 15 abscissae. Ledger: `scan14_aux_t5.jsonl`
(SHA-256 `3d8d0f73f90929f3ddb724a5d338984daae2043d0d6d3b5f7779e6229ec45318`).
No timed call preceded this selection.

The first driver invocation used the equal-18 script's default auxiliary seed
5 by mistake; its preflights returned 15 vs 21 abscissae and the size gate
stopped it before any timed call. That record is preserved locally as
`degree32_equal15_14bit_20261300_seed5_preflight_mismatch.jsonl` and is not
measurement evidence. `f4_degree32_equal_size.py` takes `--candidate`
explicitly so the protocol's seed cannot be lost again.

## Frozen run

CLI seed `20261300`, `--bits 14`, `--family generic`, `--targets walk`,
`--repeats 16`, `--rho-runs 1`, `--max-trials 1000`,
`--solver-budget-seconds 5`, whole-call cap 3600 s, order
**interval, candidate, candidate, interval**.

## Gate

Passes only if all four invocations exit zero, both arms stay at 15
abscissae, all 16 recovered scalars agree across the four runs, both mirrored
pairs have GAE ratio below 1 (and combined charged GAE below 0.9). Wall and
child CPU are recorded but are descriptive: the host was running a concurrent
Cargo release build, as it was for the 13-bit panel.

## Claim boundary

A pass establishes transfer of the equal-size relation-extraction component
lead to one 14-bit roster curve with 16 scalars. It does not establish
instance replication, a full relation/LA/descent advantage, a same-target
advantage over rho (`s_over_rho` is expected to stay in the 1e5–1e6 range),
transfer to 15–18 bits, or anything about s59, P-192 or the NIST curves.
