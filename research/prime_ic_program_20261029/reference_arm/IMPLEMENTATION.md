# Published-IC reference arm: implementation spec (plan, 2026-10-29)

Status: **plan**. No code is written and nothing here was compiled or run. File
paths, types and line numbers were read from the checkout at
`/home/user/crypto` on 2026-10-29. Where a detail depends on msolve's source,
which was not read, it is marked *verify at the pinned commit*.

The arm adds one decomposition oracle, `semaev-naive`, and an F_p solver plug
point with three engines (`msolve`, `f4-fp`, `fp-grid`). It runs inside the
existing `ic.pipeline` method, so ecbench charges it in the same unit as rho
and the Table B incumbent, and `ecbench band` prices its solver work like any
other unpriced native work.

## 1. What exists today (read from source)

| piece | where | what it means for this arm |
|:--|:--|:--|
| Stage traits | `src/cryptanalysis/ic_framework/stages.rs`: `FactorBaseBuilder<G>` (line 159), `DecompositionOracle<G>` (line 202), `SystemSolver` (line 510), `SolverCost` (line 478), `SolverTotals` (line 313) | `SystemSolver` takes only a `BooleanSystem` (F_2). **There is no F_p solver plug point.** `FpSystem`/`FpSystemSolver` were designed in `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` §4.1 and never built (no match for `FpSystem` in `src/`). |
| Factor base | `PrimeAbscissaBase` in `src/cryptanalysis/ic_framework/plugins.rs:86`, spec `prime-abscissa:size=N[,stride=S]` | This is already the Semaev small-abscissa base, and it is the incumbent's base. The reference arm reuses it unchanged. |
| Incumbent oracles | `MitmOracle` (`plugins.rs:2056`, spec `mitm:negation_folded=1`), `RootfindSubtractOracle` (`plugins.rs:2297`) | Table and unclassified tracks (`docs/ic/SWEEPER.md`, Tracks). |
| Prime algebraic precedent | `PkmSemaevOracle` in `src/cryptanalysis/ic_framework/pkm_semaev.rs` (private `s3`, `s4`, symbolic `Pol`; solver block in unit `F_p multiply-adds (elimination only)`; grid fallback in `F_p multiplications (grid)`) | The model to copy for accounting. It calls the tower F4 directly rather than through a trait. |
| Semaev polynomials over F_p | `pkm_semaev.rs` (`s3`, `s4` by the two-quadratic resultant), `src/cryptanalysis/semaev_higher.rs` (`S_3`…`S_6` by resultants, BigUint field) | m = 2, 3 are covered; m = 4 (`S_5`) is in `semaev_higher.rs` on a different field type. |
| In-house F_p F4 | `src/cryptanalysis/f4_fp.rs` (`f4`, `solve`, `field_ops_total`) | The second engine, for the AGENTS.md §15 comparison. `research/f4_fp_msolve_parity_20261011/README.md` measured msolve ÷ f4_fp at 0.96–1.22 single-threaded on generic systems. |
| ecbench dispatch | `solve_ic_prime` in `src/cryptanalysis/ecbench/methods.rs:4130–4220`; `pipeline_spec` (line ~4105) already parses a `solver` spec; `IC_PIPELINE_PARAMS` (line 182) | One more match arm and an F_p solver lookup. |
| Solver pricing | `run_pipeline` in `src/cryptanalysis/ic_framework/mod.rs:625–705`; `ic_report` in `methods.rs:3848` | A solver unit with no pinned ratio is priced by host wall time (`measured`); `ic_report` then takes that term back out and lists `solver_<unit>_uncharged`, and `band.rs::unpriced_counts` prices `solver.ops` at w. Only the units `word XORs` (pinned ratio) and exactly `F_p multiplications` with `extra.fp_muls_per_add` (counted) stay inside G. |
| Candidate identity | `Implementation::this_binary` in `src/cryptanalysis/ecbench/claim.rs:336` hashes `ic_framework/{mod,plugins,solvers,linalg}.rs`, `ic_boundary.rs`, `methods.rs`; `pkm_semaev.rs` is bound separately at line 486 | New files are not bound until added. |
| `ic bench` | `run_prime` in `src/bin/ic/bench.rs:586–650`; sweep spaces `src/bin/ic/sweep/space.rs` (`prime-table`, `prime-rootfind`; `docs/ic/SWEEPER.md` says "No algebraic prime space") | One match arm; one new sweep space. |
| msolve prior art (Python, legacy) | `crypto-autoresearcher:src/crypto_autoresearcher/index_calculus/msolve.py` | The system layout, `-c 0 -P 1`, the separating-form retry and the output grammar. It is Python, so under AGENTS.md "Implementation language: no Python" it is reimplemented in Rust, not called. |
| msolve inputs from Rust | `examples/gb_cas_export.rs::write_msolve` (line 184) | The `.ms` format: variables line, characteristic line, comma-separated polynomials. |

## 2. File-by-file changes

### 2.1 New: `src/cryptanalysis/ic_framework/fp_solvers.rs`

The F_p plug point, built as the PKM note §4.1 designed it, beside (not
inside) the boolean `SystemSolver`, so no existing engine changes.

```rust
pub struct FpSystem {
    pub p: u64,
    pub n_vars: usize,
    pub var_names: Vec<String>,
    pub equations: Vec<crate::cryptanalysis::f4_fp::Poly>,
    /// Index of the separating variable z, when the oracle added one.
    pub separating: Option<usize>,
}
pub enum FpVerdict { Solved(Vec<Vec<u64>>), Unsatisfiable, BudgetExceeded }
pub trait FpSystemSolver: Send + Sync {
    fn name(&self) -> &str;
    fn describe(&self) -> String;
    fn parameters(&self) -> &[(&str, &str)] { &[] }
    fn accepts(&self, shape: &SystemShape) -> bool { let _ = shape; true }
    fn finds_every_solution(&self) -> bool { true }
    fn solve(&self, sys: &FpSystem, params: &Params, budget: Option<Duration>)
        -> Result<(FpVerdict, SolverCost), String>;
    /// Strings for the record (binary hash, source revision, flags).
    fn identity(&self) -> BTreeMap<String, String> { BTreeMap::new() }
}
pub fn fp_solver_by_name(name: &str, params: &Params) -> Result<Box<dyn FpSystemSolver>, String>;
```

`solve` returns `Err` for anything that is neither a decision nor a budget
stop (crash, positive-dimensional answer, unparsable output, a returned point
that does not satisfy the system). The oracle turns `Err` into a failed run,
never into "no decomposition". `SolverCost`, `SolverTotals` and `SystemShape`
are reused unchanged.

Engines:

| name | how | `SolverCost.ops` / `op_unit` |
|:--|:--|:--|
| `msolve` (params `recover=param` default, or `recover=gb`) | subprocess, §3 | patched build: F_p multiply-adds in msolve's linear algebra and FGLM, unit `F_p multiply-adds (msolve linear algebra and FGLM)`; unpatched fallback: `msolve F4 matrix entries (proxy)`, §4 |
| `f4-fp` | `f4_fp::solve` on the same `Poly` list, then the same root recovery as `msolve` from the basis | `f4_fp::field_ops_total()` delta, unit `F_p field operations (f4_fp)` |
| `fp-grid` | every unordered tuple of V^m tested against the equations | multiplications, unit `F_p multiplications (grid)` (the pkm-semaev string, deliberately not the bare `F_p multiplications`, so it is not routed to the counted path in `mod.rs:672`) |

### 2.2 New: `src/cryptanalysis/ic_framework/msolve.rs`

The msolve contract (§3): `.ms` writer (shared rendering with
`examples/gb_cas_export.rs`), `std::process::Command` runner with no shell,
`-v 2` log parser, `-P 1` output parser, counter parser, binary-hash check,
and the startup calibration. Kept apart from `fp_solvers.rs` so the
`fp-grid` and `f4-fp` engines build and test without msolve.

### 2.3 New: `src/cryptanalysis/ic_framework/semaev_fp.rs`

`S_3`, `S_4` (two-quadratic resultant) and, for the chain presentation,
repeated `S_3` with auxiliary variables, over u64 F_p, as `f4_fp::Poly`.
Copy the formulas from `pkm_semaev.rs` rather than refactoring it: that file's
hash is part of the PKM arm's identity (`claim.rs:486`), and moving code out of
it would change the identity of committed PKM evidence for no reason. Unit
tests: `S_3(x(P), x(Q), x(P±Q)) = 0` and `S_4` likewise on the `p = 113`
fixture used in `plugins.rs` tests.

### 2.4 New: `src/cryptanalysis/ic_framework/semaev_naive.rs`

`SemaevNaiveOracle: DecompositionOracle<PrimeCurve>`, name `semaev-naive`.

| parameter | meaning |
|:--|:--|
| `m` | summands: 2, 3; 4 only with `mode=chain` |
| `mode` | `full` (one `S_(m+1)`, m ≤ 3; the Yokoyama et al. presentation) or `chain` (Semaev 2015: `S_3` links with m − 2 auxiliaries) |
| solver | an `FpSystemSolver` from the pipeline's `solver` spec (`msolve`, `f4-fp`, `fp-grid`) |

- **`prepare`**: collect the base's distinct abscissae V from `fb.points`
  (they are the keys of `fb.x_index`); refuse a base with fewer than m
  abscissae or with p ≥ 2^31 (msolve's prime-field limit as stated in
  `msolve.py`; *verify at the pinned commit*). Expand
  `F_V(x) = ∏_{c ∈ V}(x − c)` once: N(N − 1)/2 multiplications, reported
  by `setup_native` as `semaev_membership_fp_mults_uncharged`. Build the
  symbolic `S_(m+1)` with `x_R` as a parameter once. Ask the engine whether
  it `accepts` the shape; refuse with its reason if not.
- **`decompose(R)`**: specialise `S_(m+1)` at `x_R` (counted,
  `semaev_system_fp_mults_uncharged`); build the `FpSystem`
  `{F_V(x_1), …, F_V(x_m), S, (chain links), z − Σ c_i x_i}` with the
  separating coefficients `c_i` drawn from ChaCha8 seeded by
  `(spec seed, trial index, retry)`; call the engine. On `Solved`, sort the
  solutions canonically, re-check each one in Rust (`F_V(x_i) = 0` and
  `S = 0`, counted), and pass the first that lifts through
  `ic_boundary::lift_abscissae` (`ic_boundary.rs:6298`), which re-adds the
  summands on the counted ledger. On `Unsatisfiable`, return `None`.
  `Err` aborts the run. `BudgetExceeded` cannot occur in this study (no
  budget is set, `solver_budget_seconds = 0`, so the record stays
  deterministic).
- Root recovery for `recover=param` is in Rust: the F_p roots of msolve's
  eliminating polynomial `w(z)` by `gcd(w, z^p − z)` (square-and-multiply
  mod w) and seeded equal-degree splitting, then each coordinate from the
  parametrisation. All of it is counted as
  `semaev_rootfind_fp_mults_uncharged`. `deg w` is the number of solutions
  over the algebraic closure, which is small here because every solution
  lies in V^m.
- `relation_counters`: the four `semaev_*_uncharged` counters.
  `solver_totals`: the absorbed `SolverCost`s. `last_system`: the shape.
- A new default method on `DecompositionOracle` in `stages.rs`,
  `fn detail(&self) -> Option<serde_json::Value> { None }`, lets the oracle
  hand `ic_report` the engine identity (msolve sha256, source revision,
  flags, startup calibration) as strings. `SolverTotals.extra` is
  `BTreeMap<String, u64>` and cannot carry a hash.
- Module doc: the charge table of §4, and the statement that this is the
  published naive presentation, not a new method.

### 2.5 `src/cryptanalysis/ic_framework/mod.rs`

`pub mod fp_solvers; pub mod msolve; pub mod semaev_fp; pub mod semaev_naive;`.
Pricing code unchanged: the msolve unit has no pinned ratio, so
`run_pipeline` prices it `measured` and `ic_report` removes that term and
lists it unpriced (`methods.rs:3866–3882`), which is what puts it in the band.

### 2.6 `src/cryptanalysis/ic_framework/stages.rs`

Only the `detail()` default method of §2.4. Existing oracles are unaffected.

### 2.7 `src/cryptanalysis/ecbench/methods.rs`

- `solve_ic_prime` (line ~4130): add `"semaev-naive"` to the oracle match.
  Build the engine with `fp_solver_by_name(spec.solver, spec.solver_params)`.
  Refuse a `solver` value on any other prime oracle, and refuse
  `semaev-naive` without one.
- `IC_PIPELINE_PARAMS` (line 182): extend the `oracle` and `solver` help
  strings (`semaev-naive:m=2|3,mode=full|chain`; `msolve[:recover=param|gb]`,
  `f4-fp`, `fp-grid`).
- `ic_report` (line 3848): copy `oracle.detail()` into
  `detail.decomposition.engine`; add `msolve_sha256` to the record so the
  audit and the validation gate can read it.
- No change to `pinned_only_calibration`, `PRICED_NATIVE` or the band. The
  new `*_uncharged` counters are picked up by the generic loop at
  `methods.rs:3913–3919`.
- This edit changes the IC candidate identity, as `ecbench-extend` step 7
  says it must.

### 2.8 `src/cryptanalysis/ecbench/claim.rs`

Add `fp_solvers.rs`, `msolve.rs`, `semaev_fp.rs` and `semaev_naive.rs` to
`Implementation::this_binary` (line 336), and bind the pinned msolve sha256
into the IC1 `candidate_id` of any claim whose oracle is `semaev-naive`, the
way `pkm_semaev.rs` is bound at line 486.

### 2.9 `src/cryptanalysis/ecbench/audit.rs` (replay)

A `semaev-naive` record replays only with an msolve whose sha256 equals the
record's `msolve_sha256`. Otherwise the replay result is
`unavailable: msolve hash mismatch` (or `msolve missing`), which counts as not
replayed. It must never count as identical or be skipped silently.

### 2.10 `src/bin/ecbench.rs`: two subcommands

- `ecbench solver-selftest msolve --p P`: the three checks that caught the
  Ubuntu 0.6.5 build (`msolve.py` docstring): `{x1 − 1, x1 − 2}` must give
  `[-1]`; `{x1^2 − 1, x2^2 − 1}` must give 4 solutions; the m = 2
  decomposition system on a fixture curve at that p must match `fp-grid`.
  Writes `selftest-msolve-p<P>.json` into the session directory. The study
  runs it at each session's p before the first measured execution.
- `ecbench oracle-check --curve C --factor-base F --oracle O --solver S --planted K --seed s`:
  the positive control (`pub-m2-pos`). It draws K planted points `P_i ± P_j`
  from the base, runs the oracle on each, and reports detection and whether
  the planted pair is among the solutions.

### 2.11 `src/bin/ic/bench.rs` and `src/bin/ic/sweep/space.rs`

- `run_prime` (line 586): the same `"semaev-naive"` match arm and solver lookup.
- `space.rs`: a `prime-algebraic` space on `Track::Algebraic`, baseline
  `semaev-naive:m=2,mode=full` + `msolve`, with the base size as a
  scale-relative knob (`2^(θ·scale)`), so the sweeper's algebraic prime track
  has a published baseline. Update `docs/ic/SWEEPER.md` ("No algebraic prime
  space") in the same PR.

### 2.12 Tests

- `semaev_fp.rs`: polynomial identities on the `p = 113` fixture.
- `semaev_naive.rs`: `fp-grid` against `f4-fp` on every trial of a small
  pipeline run (identical relation streams), with no msolve needed. With
  msolve on PATH (CI installs it, §5), `msolve` against `fp-grid` likewise,
  plus a planted-point test.
- `tests/ecbench.rs`: a scratch spec with `pub-m2`, `pub-m2-grid` and
  `rho.negation` on a 12-bit roster curve, `--cpus none`, then
  `verify --replay`: every run verifies and every replay is identical.

### 2.13 Image, build and CI

- New `taskq/deploy/eks-karpenter/build-msolve SRC_COMMIT OUTPUT_DIR`: clone
  msolve at the pinned commit, apply `vendor/msolve/0001-ecbench-counters.patch`,
  build static against static GMP, MPFR and FLINT, with
  `-O2 -march=x86-64-v3` so the binary does not depend on the build host's
  vector flags (the crypto-autoresearcher readiness build was host-specific:
  `research/snfs_ecdlp_ledger_20261017/toolchain/math-readiness/README.md`),
  run `make check`, check `readelf -l` has no `INTERP`, and write `msolve`,
  its sha256 and the configure line.
- `taskq/deploy/eks-karpenter/build-bundle`: call `build-msolve` and add
  `msolve_sha256`, `msolve_source_revision` and `msolve_configure` to
  `build-info.json`.
- `taskq/deploy/eks-karpenter/Dockerfile.ecbench`: `COPY msolve /usr/local/bin/msolve`.
  The image stays a three-input context plus one binary.
- `taskq/deploy/eks-karpenter/contracts/request.schema.json` and
  `remote/validate-request`: let `job.build` pin the helper binary's sha256
  beside `ecbench_sha256`, if the validator rejects an unknown field
  (*check*; not read here).
- `.github/workflows/ecbench.yml` and `.github/workflows/audit-runs-daily.yml`:
  fetch the same static msolve artefact, check its sha256, and put it on PATH
  before `ecbench verify --replay`, so committed reference-arm sessions are
  replayed in CI rather than skipped.

### 2.14 Docs

`docs/ecbench/README.md` §7 charge table row for `semaev-naive`;
`docs/ic/FRAMEWORK.md` plug-in entry; `docs/ic/SWEEPER.md` tracks table.

## 3. The msolve subprocess contract

**Binary.** A static msolve built from one pinned upstream commit with the
counter patch. Do not use the Ubuntu package. `msolve.py` records that the
Ubuntu 24.04 `msolve 0.6.5-1build2` segfaults on the m = 2 decomposition
system at p = 260317, segfaults on `{x1^2 − 1, x2^2 − 1}` at p = 65537 and
2^31 − 1, and returns `{x1 − 2}` for the inconsistent `{x1 − 1, x1 − 2}` at
p = 65537. The Table B roster needs p up to about 2^24. Candidate pin: the
upstream commit `a18d0de45550363405f3dd8747abbf33f4384a06` that
crypto-autoresearcher built and tested (71 upstream tests passed, same
readiness README). The repository's other msolve use is 0.9.5 at p = 17 and
29 (`research/radical_pair_image_20260921/contract.json`). Neither has been
tested above 2^16 here, so the self-test of §2.10 is required at every p.

**Location and identity.** Path from `ECBENCH_MSOLVE` (default
`/usr/local/bin/msolve`). At `prepare`, the oracle hashes the file and refuses
unless it equals the sha256 compiled into `msolve.rs` (`MSOLVE_SHA256`), so a
record can only come from the pinned build.

**Invocation.** One process per decomposition system, no shell, empty
environment, working directory a fresh per-execution temp directory under
`$TMPDIR` (`/work` in the pod):

```
msolve -v 2 -t 1 -c 0 -P 1 -f in.ms -o out.ms        (recover=param)
msolve -v 2 -t 1 -g 2 -f in.ms -o out.ms             (recover=gb; consistency only, solutions from fp-grid)
```

**Input** (`in.ms`): line 1 the variable names
(`x1,x2,z` for m = 2 full), line 2 the characteristic p, then one polynomial
per line, separated by `,` and a newline. Terms are `c*x1^e1*x2^e2` with
`0 ≤ c < p`. The order of variables and equations is fixed by the oracle, so
the file is a deterministic function of (curve, base, x_R, separating
coefficients).

**Output** (`out.ms`, the grammar `msolve.py::parse_output` reads; *verify at
the pinned commit*):

| text | meaning | action |
|:--|:--|:--|
| `[-1]:` | no solution | `Unsatisfiable` |
| `[1, …]:` | positive-dimensional | `Err` (cannot happen with F_V in every variable; indicates a bug) |
| `[0, [p, nvars, deg, vars, linform, [_, [w, den, params]]]]:` | rational parametrisation in z | check `p`, `vars`, `linform = z` and `deg = len(w) − 1`; recover roots in Rust |
| non-zero exit, empty or unparsable file | failure | with `-c 0`, a non-separating form is reported as failure: redraw `c_i` (counter `msolve_separating_retries`) up to 8 times, then `Err` |

**Stdout** (`-v 2` log, as in
`research/radical_pair_image_20260921/results/run-003/raw/*.stdout`): the F4
round table (`deg sel pairs mat density new zero time`), `#rows reduced`,
`#zero reductions`, `#pairs reduced`, `overall(elapsed)`, and from the patched
build two added lines, `#ecbench muladds_la N` and `#ecbench muladds_fglm N`.

**Determinism.** msolve prints `Initial seed for pseudo-random number
generator is 0` and runs with `-t 1`; the separating form is seeded from the
execution. A replay must reproduce every counter. The study's A/A arm checks
this (the counted ratio must be exactly 1.000).

**Timing.** Per AGENTS.md §15, process start-up is not algorithm time.
`SolverCost.wall_ns` records the spawn-to-exit wall time. msolve's own
`overall(elapsed)` (printed to 0.01 s) and a per-session startup
calibration (median of 5 runs on a trivial system, as `msolve.py::startup_seconds`
does) go in the detail, so the report can show both. The legacy measurement
found about 4 ms of start-up a call (crypto-autoresearcher index_calculus
README §2). At 20 bits that is comparable to the solve itself (§6 of the
README), so the wall metric is secondary and labelled.

**FFI (later, optional).** msolve's neogb library has a C entry point used by
AlgebraicSolving.jl (`export_f4`; *verify name and signature at the pinned
commit*). Linking it statically into ecbench would remove start-up and file
I/O. The counted contract stays the same, so the switch changes only wall time.
It is a phase-2 option, not needed for the reference.

## 4. How msolve's work is counted and priced

**Counted.** The patch adds one process-global `u64` incremented at each F_p
multiply-add in the row-reduction kernels of the F4 linear algebra, and at
each multiply-add in the FGLM / parametrisation matrix products. *Locate both
in the pinned source* (the neogb linear-algebra files and the FGLM directory).
This is the unit `pkm-semaev` uses for the in-house tower F4 (`F_p
multiply-adds (elimination only)`), so the two published arms (naive and PKM)
can be read on one scale. Like that unit, it omits symbolic preprocessing and
hash-table work. The counted unit of `f4-fp` (`field_ops_total`) is close but
not identical, and is reported separately.

If the patch cannot be maintained, the unpatched fallback is
`ops = Σ_rounds round(rows × cols × density)` from the `-v 2` table, with the
unit `msolve F4 matrix entries (proxy)`. It is deterministic but is a proxy,
not an operation count. Any band ranking then holds only in that unit and
says so.

**Recorded per call and summed** (`SolverTotals::absorb`): `ops`, `wall_ns`,
`solving_degree` (highest `deg` with `new > 0`), `degree_reached` (highest
`deg`), and `extra`: `msolve_f4_rounds`, `msolve_rows_reduced`,
`msolve_zero_reductions`, `msolve_pairs_reduced`,
`msolve_matrix_rows_max`, `msolve_matrix_cols_max`,
`msolve_param_degree_max`, `msolve_separating_retries`,
`msolve_matrix_entries_proxy`.

**Priced.** Nothing about msolve enters G (the charged GAE). The flow is:

1. `run_pipeline` prices the solver block `measured` (host wall time), since
   its unit has no pinned ratio (`mod.rs:682–684`).
2. `ic_report` rebuilds the relations phase without it and lists
   `solver_F_p_multiply-adds_(msolve_linear_algebra_and_FGLM)_uncharged`
   (`methods.rs:3866–3882`).
3. `ecbench band` / `compare --weight w` add `w · solver.ops`
   (`band.rs::unpriced_counts`, the `solver_*` branch). They also add
   `w · Σ semaev_*_uncharged`, which covers the membership expansion,
   specialisation, root finding, re-checks and grid. The pinned native work
   `n_C` and every unpriced counter of the other arms (the incumbent's
   lookups and inversions, rho's canonicalisations) are added the same way.

So `priced_gae(w) = G − C + w·(U + n_C)` holds for every arm, as AGENTS.md
§5a requires. The engine must not report the bare unit `F_p
multiplications` with `extra.fp_muls_per_add`. That would route it to the
counted branch at `mod.rs:669–681` and hide it from the band.

Group operations stay charged in G as usual: the walk's additions, the lift's
re-additions, linear algebra and verification.

**What w means here.** One msolve multiply-add priced at w = 1 GAE overcharges
it by roughly the field multiplications in one affine addition. At w = 0.1 it
is roughly at par, and at w = 0 it is free. Every comparison is ranked only
where its order holds at all three weights. Because the reference arm's
group-operation count at matched base is close to the incumbent's (same walk),
expect its order against the incumbent to depend on w at w = 0. That is why
the protocol reports `unresolved_in_band` as a possible verdict.
