# C1 end to end: twisted all-lift line against the standard base, against strong rho

2026-11-07. Status: **prepared, not run. No measured data exist.** Every sentence here is
an unverified assertion of the preparing session [pcc:unverified].

- Protocol: [`PROTOCOL.md`](PROTOCOL.md), [`protocol.json`](protocol.json). It validates and was committed
  before any data (`bfcd18db85`). It is still a draft: the freeze tool needs a measured per-arm timing first.
- Instrument: [`specs/c1-e2e-m5.json`](specs/c1-e2e-m5.json) (stage 1) and
  [`specs/c1-e2e-m7.json`](specs/c1-e2e-m7.json) (stage 2).
- Code: `src/cryptanalysis/ecbench/xline_bin.rs`.

## What ecbench already had (survey)

- **The tower and its counted unit.** The tower `F_{(2^m)^4}` is
  `Instance::F2m4` / `f2m4_torsion` (`gaudry_quartic_bin`). Its counted unit is the
  `F_q` multiplication on `Gf2q::muls`, priced at the curve's `fq_muls_per_add`.
- **An existing `n = 4` pipeline.** `ic.pipeline` already runs Gaudry's `n = 4`
  decomposition there with the base `bin-t-line` (FGHR's 2-torsion-stable
  `t`-line) and the oracle `gaudry-d4-bin`. The oracle is a weighted
  `(2, 2, 2, 1)` Macaulay solve at degree 26, with an eigen step along the
  chains, a split and sign checks. Solver work is priced by count.
- **The rho reference.** `rho.negation_strong` is the strong negation walk on the same tower,
  charged in the same unit.
- **Missing pieces.**
  - No base takes an abscissa line `x ∈ u·F_q`. The `t`-line is a different construction.
  - No oracle solves the C1 system in `(e₁², e₂², e₃², e₄)` of `x/u`.
  - `f2m4_torsion` draws random `a`, so `Tr(a) = 1` occurs. On such a curve the twisted line lifts
    nothing (PR #2704, H-Y4).
- **The prime-field analogue.** `ic.k4_gaudry` (`fp4.rs`) charges a measured per-trial constant
  and enumerates relations. It is a lower bound and cannot promote anything, so it was not used
  as the model.

## What was built

- **`CurveSpec::F2m4A0` (`kind: f2m4_a0`).** `y² + xy = x³ + b` with `a = 0` and
  `b ∉ F_{q²}`, built as an `F2m4` instance. Every F2m4 method therefore runs on it,
  `rho.negation_strong` and the generic walks included.
- **`bin-x-line`.** The parameter is `line=standard` or `line=twisted,line_seed=N`. The
  twisted `u` is `z^q + z` with `Tr_{q⁴/q}(√b/u) = 0`, `u ∉ F_q` and
  `u ∉ b^{1/4}F_q`. The base refuses a twisted line that is not all-lift.
- **`gaudry-x-bin`.**
  - It ports the ladder's symbolic `S₅` and its symmetrisation.
  - Per target it builds the four-equation Weil restriction in `(E₁..E₄)` and solves it
    with the same `solve_bin` the `t`-line oracle uses.
  - It splits each rational solution through `X⁴ + e₁X³ + e₂X² + e₃X + e₄`, lifts the roots,
    and checks the signs.
  - It is charged as `gaudry-d4-bin` is. The line search and the `u`/`b` power
    precomputation are solver work in its own prepare.
- **Tests (written, not yet run).**
  - The `e`-form of `S₅` vanishes on four-point sums and equals `CurveB::s5`.
  - The construction rules hold at `m = 5, 7`.
  - The oracle's decomposition sets equal an independent pair-table enumeration.

## Why nothing ran

- **No build, so no local checks.**
  - The shared disk had 4.5 GB free throughout.
  - The task forbids a build that would leave less than 4 GB, and this crate's release
    build with its 266 dependencies would. The code has been run through `rustfmt` only:
    it has not been compiled, unit-tested or smoke-solved.
  - PROTOCOL.md stopping rule 1 requires those checks and a timing before any dispatch.
- **The AWS bridge is denied.**
  - The container's AWS identity, `arn:aws:iam::590183823895:user/adam`, gets
    `AccessDenied` on `codebuild:BatchGetProjects` for
    `isolab-experiment-launcher`. It also gets `AccessDenied` on `s3:ListObjectsV2` for
    `isolab-results-104186811093-us-east-2/runs/`.
  - It is not an authorized lab identity (ACCESS.md §2).
  - The route is therefore the GitHub dispatch of `isolab-experiment.yml` on `main` with the
    frozen `source_revision`. The GitHub tooling can read that workflow's runs.
- **The checkpoint push is denied.** `scripts/checkpoint.sh gate` returned `AccessDenied` on
  `s3://setec-astronomy-artifacts/checkpoints/`, and no upload token is present.

No run, no cost, no result, no leaderboard or `benchmarks/` row, no pcc claim (there is
nothing to claim).

## To resume

See `.resume/RESUME.md` in the preparing worktree; the exact steps are in PROTOCOL.md
"Stopping rules" 1–2. With ≥ 8 GB free:

```sh
cargo test --release --lib xline_bin
cargo build --release --bin ecbench
# smoke: a 2-target, rounds-1 scratch copy of specs/c1-e2e-m5.json, out dir outside the repo
./target/release/ecbench run --wait --spec /tmp/smoke.json --out /tmp/smoke-out --allow-busy
./target/release/ecbench verify --dir /tmp/smoke-out --replay 4
```

Then open a `bench.record/v1` record and dispatch `isolab-experiment.yml` on `main` with
`spec_path=research/c1_end_to_end_solve_20261107/specs/c1-e2e-m5.json`,
`instance_type=c8a.2xlarge` and `seconds=3600`.

## Sources checked

- The research-kb MCP server is not connected in this session. I read the cited studies directly:
  - `crypto:research/c1_twisted_line_ladder_20261027/README.md`
  - `crypto:research/lift_dense_factor_base_20261027/README.md`
  - `crypto:research/subgroup_trap_20261107/README.md`
- I also read this study's own source references: `crypto:src/cryptanalysis/gaudry_quartic_bin.rs`,
  `crypto:src/cryptanalysis/ecbench/negation_strong.rs` and `crypto:src/cryptanalysis/ecbench/fp4.rs`.
