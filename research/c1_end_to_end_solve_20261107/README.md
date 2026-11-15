# C1 end to end: twisted all-lift line against the standard base, against strong rho

2026-11-07/08. **Status: implemented, tested locally, protocol frozen, ready to dispatch, not
run.** No measured data exist. Every sentence here is an unverified assertion of the
preparing session [pcc:unverified].

- **Protocol.** [`PROTOCOL.md`](PROTOCOL.md) and [`protocol.json`](protocol.json), frozen; the
  `freeze` block names the commit.
- **Analysis.** [`analyze.sh`](analyze.sh), hash in the protocol.
- **Instrument.** [`specs/`](specs/): the stage 1 std and tw dispatches at m = 5, and stage 2 at m = 7.
- **Code.** `src/cryptanalysis/ecbench/xline_bin.rs`, plus a fallback in
  `src/cryptanalysis/ecbench/band.rs`.
- **Launch.** The dispatch is held back. The checkpoint upload gate
  (`scripts/checkpoint.sh gate`) fails with AccessDenied and no upload token is present, and
  the coordinator is raising that with the owner.

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
- **Tests (passed 2026-11-08, `cargo test --release --lib`).**
  - `xline_bin` (3 of 3):
    - the `e`-form of `S₅` vanishes on four-point sums and equals `CurveB::s5`;
    - the construction rules hold at `m = 5` and `7`;
    - the oracle's decomposition sets equal an independent pair-table enumeration on 12
      targets per line.
  - `ecbench::workload` and `registry_audit_tests`: 17 passed.
  - `f2m4`, `negation_strong` and `gaudry_quartic_bin`: 15 passed, 1 ignored.
- **The §5a band on these records.**
  - `ic.pipeline` on `F2m4` prices its lifts, row operations and guard probes in pinned
    units whose ratios are this run's own counted `F_q` multiplications. The repository's
    pinned calibration table has no row for these curves, so `ecbench compare` refused the
    band ("priced by pinned units … but no calibration row").
  - The x-line records now carry `detail.calibration_ratios`.
  - `band::calibrated_native` reads those ratios only when the table has no row, so every
    existing record is priced as before.

## Development smoke (not evidence)

Settings:

- `m = 5`, curve `icv1-f2m20-t133-d7e842ba` (`r ≈ 2^18`, unregistered: register before
  citing it in prose).
- `target_seed = 99`, not a frozen target.
- One workload, rounds 1, L0, `--cpus none`, on the shared preparing container.

Every run verified `[k]G = Q`, and `ecbench verify --replay-all` reproduced every record
exactly (4 of 4 and 5 of 5).

| arm | S | wall per execution | trials | relations / columns |
|---|---:|---:|---:|---:|
| `rho-strong-l1` | 3.007 | < 1 ms | | |
| `rho-strong-l16` | 2.904 | < 1 ms | | |
| `ic-std` | 30257.2 | 57.7 s | 199 | 13 / 15 (pinned at rank 13) |
| `ic-tw-a` | 9634.4 | 17.5 s | 59 | 32 / 31 |
| `ic-tw-b` | 7085.8 | 13.5 s | 43 | 32 / 31 |
| `ic-tw-a-aa` | 9634.4 | 18.6 s | 59 | 32 / 31 |

These numbers set the dispatch plan only. They come from one workload, and they are not a
result.

## Why it has not run


- **Build and local checks: resolved on 2026-11-08.** On 2026-11-07 the shared disk had only
  4.5 GB free and the session could not build. Once disk was freed, the code compiled, the
  tests passed and the smoke ran (above).
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

There has been no run, so there is no cost, no result, no leaderboard or `benchmarks/` row and
no pcc claim: there is nothing to claim.

**The `tools/benchrec.py gate` failures predate this branch.** The gate reports 659
problems across 54 records. They come from existing records on `main` whose files lie
outside this sparse checkout (for example
`benchmarks/records/20261107T2145Z-split-prime-p1-rr-dickson.json`, whose graph is not in the
checkout), together with the progress report built from them. None comes from this branch,
which adds no benchmark record because nothing ran.

## To dispatch (once the checkpoint gate passes)

1. Open a `bench.record/v1` record for each dispatch (`python3 tools/benchrec.py init`).
2. Dispatch `isolab-experiment.yml` on `main` twice, with `source_revision` set to the frozen
   commit (`protocol.json` `freeze.commit`, or any later commit that leaves `specs/`,
   `analyze.sh`, `xline_bin.rs`, `band.rs` and `methods.rs` unchanged):
   - `spec_path=research/c1_end_to_end_solve_20261107/specs/c1-e2e-m5-std.json`,
     `instance_type=c8a.2xlarge`, `seconds=3600`;
   - `spec_path=research/c1_end_to_end_solve_20261107/specs/c1-e2e-m5-tw.json`, with the same
     instance type and deadline.
3. Fetch both sessions into `sessions/` and run `analyze.sh STD TW`. Then build the
   `VALIDATION.json`, the report, the graph and the PDF.

## Sources checked

- The research-kb MCP server is not connected in this session. I read the cited studies directly:
  - `crypto:research/c1_twisted_line_ladder_20261027/README.md`
  - `crypto:research/lift_dense_factor_base_20261027/README.md`
  - `crypto:research/subgroup_trap_20261107/README.md`
- I also read this study's own source references: `crypto:src/cryptanalysis/gaudry_quartic_bin.rs`,
  `crypto:src/cryptanalysis/ecbench/negation_strong.rs` and `crypto:src/cryptanalysis/ecbench/fp4.rs`.
