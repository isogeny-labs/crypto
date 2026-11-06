# Protocol: the linear stage after F4, end to end, on a corpus of Gröbner bases

Frozen before any measured session of this study. The owner asked (2026-10-26)
to speed up the linear-algebra stage until it takes the leaderboard's
linear-algebra section, end to end for the linear stage alone, on a variety
of systems whose Gröbner bases come out of F4. The whole index calculus is not
in scope.

## What is measured

The **linear stage** starts from a monic grevlex Gröbner basis (`f4_fp`) of a
zero-dimensional ideal over `F_p`, `p < 2^32`, `n ≤ 8`. It ends with every
`F_p`-rational point, each checked against the input system. The clock covers
everything between the basis and the checked points:
- the staircase;
- the normal forms;
- the multiplication matrix;
- the minimal polynomial;
- the roots;
- the coordinates;
- the check against the input.

Three arms are measured on the same basis:

| arm | code |
|:--|:--|
| `dense` | `jv_cover::linear_stage_from_basis`, `LinMode::Dense`: all multiplication matrices, Hessenberg characteristic polynomial, one null space per root |
| `sparse` | the same function with `LinMode::Sparse`, the incumbent of the IC leaderboard section "Linear stage after F4 (sparse FGLM)". It computes `M_{x_n}`, then Wiedemann/Berlekamp–Massey (`2δ` products and `2δ` dot products), then a corank-one elimination over the shift chains per root, falling back to `dense` |
| `fast` | `zero_dim_la::fast_stage`: the same `M_{x_n}`, as hybrid dense/sparse columns. A single transposed Krylov pass `(M^T)^j u` serves every coordinate; then Berlekamp–Massey, roots from `gcd(f, x^p − x)` (squaring modulo `f` by schoolbook below length 256 and Karatsuba above, with Barrett reduction), and the coordinates read out by the rational univariate representation. Lazy `u32`/`u64` arithmetic with explicit AVX-512 / AVX2 kernels chosen at run time (`ZERO_DIM_LA_SIMD`). Single-threaded (`ZERO_DIM_LA_THREADS` unset). Falls back to `dense` inside its own clock when it gives up |

`scripts/` names the commit and the binary `sha256` of every run.

### Corpus (`examples/postgb_la_bench.rs`, frozen generators and seeds)

The corpus has five families. Four are the decomposition systems of
index-calculus pipelines in this repository. The fifth is a generic control
across sizes.

| family | system | n | δ | p | systems |
|:--|:--|--:|--:|:--|--:|
| `jv-nagao` | Weil-restricted Nagao systems of `jv_cover` (6 quadrics), random and planted residuals, `nagao_systems(p, 1, 8, 4)` | 6 | 64 | 53, 101, 251, 1009 | 4 × 12 |
| `gaudry-s4` | symmetrised S₄ over `F_{p³}`, Weil-restricted (`gaudry_cubic`) | 3 | 64 | 271, 1039, 8191 | 3 × 8 |
| `gaudry-d4` | `D₄`-invariant S₅ over `F_{p⁴}` (`gaudry_quartic_d4::SymmetrisedD4::system`) | 5 | 512 | 97, 1033, 4093 | 3 × 8 |
| `gaudry-s5` | full symmetrised S₅ over `F_{p⁴}` (`gaudry_quartic`) | 4 | 4096 | 13, 101 | 2 × 2 |
| `dense` | `n` random dense polynomials of degree `d`, shifted through a planted point | 2–8 | 32–1024 | 65521, 2³¹−1, 4294967291 | 17 shapes × 3 primes × 3 |

- **F4 per system:** once, untimed, on `jv_options(24, 300)` for `jv-nagao`,
  and on grevlex, degree ≤ 64, a 900 s budget for the others.
- **Unusable bases:** a system whose F4 times out or is inconsistent is
  recorded as skipped, not dropped silently.
- **`gaudry-s5` sizing:** this family is run only if F4 finishes within its
  budget. Otherwise it is reported as not reached.

### Design

- **Order:** 7 interleaved rounds per system, rotating the arm order (A B C,
  B C A, C A B, …), with the same seed for every arm. Each system's statistic
  is the median wall time of each arm.
- **Host:** single-threaded, recorded with `lscpu`. Under the `ecbench` quiet
  rule the run waits for a quiet CPU before it starts.
- **Dense arm above δ = 1024:** the `dense` arm (cubic in δ) is not run, so
  for `gaudry-s5` only `sparse` and `fast` are compared. This is fixed here,
  before any session.
- **Kernel ablation:** a second process (3 rounds) runs `sparse` and `fast`, with
  `ZERO_DIM_LA_SIMD=portable` (no vector kernels). It separates the
  algorithm from the instruction set.
- **End to end inside an IC run:** `jv_cover --exp dlp --sizes 53,101 --seeds 2
  --rho-runs 4 --check-every 64`, with `JV_LIN=sparse` and `JV_LIN=fast`, the
  command of the existing leaderboard section. It gives `S`, the linear
  stage's counted multiplications, the logarithms (all must verify), and S/rho
  against the same-process rho.

## Measures

- **Units.** The headline unit is wall time, the unit of the existing
  leaderboard component section "Linear-algebra engine". Each arm's counted
  multiplications (`lin_muls`, and the fast arm's `FastStats::muls`) are
  reported beside it. The two arms count differently: the incumbent counts
  products with non-zero operands, the fast arm every product it performs.

- Per system: `sparse/fast` and `dense/fast`, the ratios of median wall time.
- Per family: the geometric mean of `sparse/fast` over its systems, with a
  bootstrap 95% interval over systems (4,000 resamples, seed 20261027), and
  its minimum and maximum.
- Over the `dense` family: the Spearman correlation of `sparse/fast` with δ.
- Correctness gate: every arm returns the same solution set on every system,
  and every planted point is found. A system that fails the gate fails the
  study. Its numbers are reported, and the arm that disagrees is named.

## Predictions

- **P1.** Every family has `sparse/fast` geometric mean ≥ 1.5, with its interval
  above 1.
- **P2.** In `dense`, `sparse/fast` grows with δ (Spearman ρ > 0).
- **P3.** `fast` gives up (falls back to `dense`) on at most 5% of systems.
- **P4.** In the `jv_cover` DLP runs, every logarithm is correct.
  - **Counted cost.** The fast arm's counted linear-stage multiplications are
    within ±25% of sparse's on every system. The gain is in the cost of each
    multiplication, not in their number. The `jv_cover` `S` is within ±10% of
    sparse's, because the linear stage is a minor share of a Nagao test.
  - **Wall time.** The fast arm's linear stage takes less wall time than
    sparse's on every DLP run (`lin_ms` summed).

  The pilots showed counted counts within about ±20%. Fewer counted
  multiplications is not predicted.
- **P5.** The portable `fast` (no vector kernels) still has `sparse/fast` ≥ 1.2 in
  every family, so the algorithm and not only the instruction set is faster.

## Runs (cards)

The executor's background shell is limited to 30 minutes, so the corpus runs
as several cards. Each card is one invocation of `postgb_la_bench` with a
family, shape or prime filter, and every card writes
`runs/<card>.json`. Together the cards cover the corpus above exactly once.
The ablation and the `jv_cover` DLP runs have their own cards. A card killed
by the harness is re-run whole; its partial output is never used.

`B = ./target/release/examples/postgb_la_bench`, `J = ./target/release/examples/jv_cover`,
`R = research/postgb_linear_stage_20261026/runs`; every card first waits for a quiet CPU and records `lscpu` and `df`.

| card | command | pilot estimate |
|:--|:--|--:|
| L1 | `B --families jv-nagao,gaudry-s4,gaudry-d4 --per 8 --reps 7 --json R/L1.json` | 5 min |
| L2 | `B --families dense --shapes n2d8,n2d16,n3d4,n3d6,n3d8,n4d3,n4d4,n5d2,n5d3,n6d2,n7d2,n8d2 --per 8 --reps 7 --json R/L2.json` | 10 min |
| L3 | `B --families dense --shapes n2d32,n3d10 --per 8 --reps 7 --json R/L3.json` | 12 min |
| L4 | `B --families dense --shapes n4d5,n5d4,n6d3 --per 8 --reps 7 --json R/L4.json` | 13 min |
| L5 | `B --families gaudry-s5 --p 101 --per 8 --reps 7 --json R/L5.json` | 10 min |
| L6 | `B --families gaudry-s5 --p 13 --per 8 --reps 7 --json R/L6.json` | 17 min |
| A1 | `ZERO_DIM_LA_SIMD=portable B --families jv-nagao,gaudry-s4,gaudry-d4,dense --per 8 --reps 3 --arms sparse,fast --json R/A1.json` | 8 min |
| A2 | `ZERO_DIM_LA_SIMD=portable B --families gaudry-s5 --per 8 --reps 3 --arms sparse,fast --json R/A2.json` | 12 min |
| D1 | `JV_LIN=sparse J --exp dlp --sizes 53,101 --seeds 2 --rho-runs 4 --check-every 64 --json R/D1.json` | 10 min |
| D2 | `JV_LIN=fast J --exp dlp --sizes 53,101 --seeds 2 --rho-runs 4 --check-every 64 --json R/D2.json` | 10 min |

`--per 8` gives 8 random and 4 constructed Nagao systems per `p` (fewer when a
residual is degenerate), 8 systems per `p` for `gaudry-s4` and `gaudry-d4`, 2
per `p` for `gaudry-s5` and 3 per shape and prime for `dense`. The ten cards
run one at a time, on the same host and binaries.

## Decision rule

- **P1 and the correctness gate hold:** `fast` becomes the default linear
  stage (`JV_LIN` default and the generic entry point). It is entered as the
  leader of a new leaderboard component section, "Linear stage after F4 across
  Gröbner-basis corpora", with the `jv_cover` end-to-end row beside it.
- **P1 fails on a family:** the family is reported, and the default is changed
  only for the families where P1 holds.
- **Class.** Engineering: a component speed-up. No row of tables A–C moves,
  because the linear stage is a small share of `S` there. The `jv_cover` row
  reports S/rho as the distance remaining.

## Inadmissible

- Changing generators, seeds, arms or the clock after seeing a measured session.
- Dropping a system.
- Editing any arm's code between the measured sessions.
