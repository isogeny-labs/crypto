# Protocol: re-measure the PR #1862 linear-harvest default (frozen 2026-10-24, before any session of this study)

Status: **frozen**. Nothing below changes after the first study session is
written. Anything learned afterwards goes in `README.md` as a deviation.

## Question

PR #1862 turned the root linear-row harvest of `koblitz_groebner` on by
default (PR #1891 later OR-ed in a root-rank gate). On the one workload the
pricing-jump bisect looked at, the K_0 n = 13 spec at m = 3, it raised counted
solver word XORs by about 7% (`research/f4_pricing_jump_20261018/`). Is that
typical, or does the default help at other sizes and curves? Is the harvest a
net gain, a net cost, or size-dependent, on the inherited-F4 pdp3-Koblitz
oracle?

## What is being compared

One engine, one base, one target stream. The IC arms differ only in the new
`linear_harvest` method parameter of `pdp3-koblitz` (this PR, `src/bin/ecbench/pdp3_koblitz.rs`):

| value | meaning |
|---|---|
| `off` | the engine before PR #1862 (no harvest) |
| `default` | the library default now in force (gpv gate OR root rank ≥ 331) |
| `gpv` | PR #1862's gate alone (generators per occurring variable < 0.961) |

Arms per spec: `rho-strong` (reference, `rho.signed_frobenius_strong`),
`f4-m3-off` (baseline), `f4-m3-default`, `f4-m3-gpv` (candidates), and as a
negative control `f4-m2-off`, `f4-m2-default` (at m = 2 the harvest gate is
expected never to fire, so these two must agree exactly).

## Workloads (one spec per curve; the base dimension is per arm, so it cannot vary inside a spec)

| spec | curve (ICV1 slug) | dimension | word XORs priced? | target seed |
|---|---|---:|---|---:|
| `SPEC-K0n13.json` | `icv1-f2m13-t181-515ee569` | 5 | yes (pinned) | 202610241301 |
| `SPEC-K1n11.json` | `icv1-f2m11-t67-05f5aa36` | 4 | yes | 202610241101 |
| `SPEC-K1n17.json` | `icv1-f2m17-tm101-00378d4e` | 6 | yes | 202610241701 |
| `SPEC-K1n19.json` | `icv1-f2m19-tm797-9c54981b` | 7 | yes | 202610241902 |
| `SPEC-K0n19.json` | `icv1-f2m19-t797-b6cf2467` | 7 | **no** (counted only) | 202610241901 |

24 public targets per curve, 1 round, no warm-up, isolation L0. The solver
counts are deterministic, so extra rounds add nothing, and no wall-time claim
is made. The dimensions follow the earlier ladder (n = 13: 5, n = 19: 7) and
the same rule for the other sizes; the target seeds are new. SHA-256 of every
spec is in `SPEC.sha256`.

The earlier smoke runs (2 targets per curve, a different seed, scratch only,
not committed) were run to check that each arm executes and how long it takes.
They showed signs that varied by size (about +7% and +10% at n = 13 and 11,
about −1% to −4% at n = 17 and 19). The hypothesis below was formed after
seeing them.

## Hypothesis and decision rules

- **H1** (the effect depends on size and curve): the default's change in
  counted solver word XORs at m = 3 is not one sign on all five curves.
- **H0** (the default is a uniform regression, or a uniform gain): the sign is
  the same on every curve.
- **Control:** at m = 2 the two arms' records agree exactly (same word XORs on
  every target). Any difference means the harness or the parameter is wrong,
  and no result from this study is reported until it is explained.
- **Measured difference on a curve:** the harness's `ops` ratio
  `f4-m3-default / f4-m3-off` has a cluster-bootstrap 95% interval that excludes
  1 on a priced curve. On `K0n19` (word XORs unpriced) the point ratio at weight
  0.1 and 1 GAE per word XOR from `ecbench compare`'s `ops.band` is reported,
  with no interval, and is labelled indicative.
- **Per curve, never pooled across sizes.** `gpv` against `default` is read the
  same way, to see what the rank gate of PR #1891 added.
- **Classification (AGENTS.md §3):** a lower counted `S` at flat ratio to the
  floor is *engineering*; a higher one is a *regression* (reported, not a
  success). Nothing here is an *advance*. No cross-method ranking is made, so
  the §5a band is only used to price the same unpriced word XORs identically
  on both arms.
- **Inadmissible:** changing arms, seeds, curves or dimensions after data;
  dropping a curve or a target; any record that is not `verified`; an audit
  that is not `OK` with every replay identical.
- **Stop condition:** if the control arms differ, or a session fails its audit,
  report that and make no claim about the harvest.

## Instrument and audit

Native `ecbench` built from this branch (opt-level 1, no LTO; counts do not
depend on optimisation). Audit: `ecbench verify --replay-all`, whose receipt's
SHA-256 is the replay certificate. Comparison: `ecbench compare --save` for
each pair and `ecbench table` per session. Python only draws graphs from the
recorded values.

## Host

This container (x86-64, 4 vCPU), `--cpus none --allow-busy`, so isolation L0:
operation counts only. No rented compute.
