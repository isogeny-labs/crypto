# Primo SMT solver

This integration adds [Primo](https://github.com/primo-solver/primo) as an
optional external backend for `cryptanalysis::smt_oracle`'s Boolean `QF_UF`
Weil-descent systems.

## Provenance and supported boundary

- upstream: `https://github.com/primo-solver/primo`
- revision: `fcc274c23ff5b146cc43d80b1b0317faf91c7f08`
- reported version: `primo 0.2.0`
- license: MIT, copyright 2026 Mikolas Janota

The pin documents support for `QF_UF` and typed Boolean models through
`get-model`. Primo does not implement `get-value`, arrays, bit-vectors,
finite-field theory, or nonlinear arithmetic at this revision. Consequently,
Crypto uses it only with `SmtEncoding::Bool`; the prime-field and `QF_BV`
paths remain on their existing solvers.

This is a correctness-qualified integration, not performance evidence. Crypto
still validates every returned assignment against the original Boolean rows
and validates lifted point decompositions before accepting them. No end-to-end
or cryptanalytic performance gain is claimed without a separately registered, matched
benchmark under the repository's evidence rules.

## Install the pinned build

The installer needs Git, GNU Make, CMake 3.20+, a C++20 compiler, Python 3,
M4, and network access for Primo's pinned dependencies. It does not use or
modify credentials.

```sh
PRIMO_PREFIX="$PWD/.tools/primo" scripts/install-primo.sh
export PATH="$PWD/.tools/primo/bin:$PATH"
```

The default prefix is `${XDG_DATA_HOME:-$HOME/.local/share}/isogeny-labs/primo`.
The installer refuses a dirty or unrelated existing source directory, checks
out the exact revision above, builds it, runs the committed model smoke test,
and exposes `bin/primo`. Re-running it at the same clean pin is supported.

To select a build without changing `PATH`, set the environment variable used
by `find_solver`:

```sh
export SMT_PRIMO_BIN=/absolute/path/to/primo
```

Rust callers select `SmtSolverKind::Primo` with `SmtEncoding::Bool`. The
adapter adds Primo's `--quiet --stats` flags and requests `get-model`; cvc5,
Z3, and generic SMT-LIB callers retain their existing `get-value` behavior.

## Use it in index calculus

`primo-smt` is registered as a first-solution `SystemSolver`. The production
`descent-algebraic` oracle can therefore hand its exact Weil-descended Boolean
system to Primo inside the same end-to-end `ic bench` pipeline as F4, FES, and
the native CDCL engine:

```sh
cargo build --release --bin ic
SMT_PRIMO_BIN="$PWD/.tools/primo/bin/primo" \
  ./target/release/ic bench \
    --koblitz-degree 13 --koblitz-a 0 \
    --factor-base binary-subspace:dimension=6 \
    --oracle descent-algebraic:m=2 \
    --solver 'primo-smt:primo_timeout_ms=60000' \
    --solver-budget-seconds 60 --rho-runs 0
```

This command is a functional pipeline smoke run; `--rho-runs 0` deliberately
leaves the comparison column empty. A performance study must instead use a
frozen matched protocol, the same instances and host for every solver, the
repository's strongest applicable reference, and its normal benchmark and
validation gates.

For a solver-stage diagnostic on identical seeded systems:

```sh
SMT_PRIMO_BIN="$PWD/.tools/primo/bin/primo" \
  ./target/release/ic descent \
    --cells 13:7:2 --targets 8 --repeats 3 --budget-seconds 60 \
    --solver exhaustive \
    --solver 'primo-smt:primo_timeout_ms=60000'
```

The framework intersects `primo_timeout_ms` with its own per-call budget,
records Primo-reported conflicts and decisions when available, and treats a
process error, external timeout, `unknown`, or invalid model as
`BudgetExceeded`. Only a real `unsat` response becomes `Unsatisfiable`.
Every SAT assignment is evaluated against the original Boolean rows before it
is converted to a solution mask. Because Primo returns one model, its rows are
kept separate from complete-enumeration engines in solver comparisons.

The supported IC boundary is binary-field descent (including Koblitz curves)
through `descent-algebraic`. Prime-field IC is not silently bit-blasted through
this plug-in: Primo 0.2.0 has no `QF_BV`, `QF_FF`, or nonlinear finite-field
theory. That missing route is tracked in
[`problems/primo-prime-field-theory-gap.md`](../../problems/primo-prime-field-theory-gap.md).

## Verify the adapter

With the binary installed or `SMT_PRIMO_BIN` set:

```sh
cargo test --release cryptanalysis::smt_oracle::tests::primo_
```

The round trip enumerates a small known solution set, independently evaluates
every model, blocks each model, and requires a final `unsat`. This establishes
protocol compatibility on the fixture only; it is not a solver benchmark.

The IC plug-in has an additional live test:

```sh
SMT_PRIMO_BIN="$PWD/.tools/primo/bin/primo" \
  cargo test --release cryptanalysis::ic_framework::solvers::tests::primo_smt_
```

It checks both a satisfiable Boolean system and a contradictory one through
the `SystemSolver` contract.

## Sources checked

- Janota and Jakubův, *Vibe-Coded and Tuned: A State-of-the-Art SMT Solver
  for QF-LRA*, 2026, [doi:10.29007/rn52](https://doi.org/10.29007/rn52).
- Primo's pinned [`capabilities.md`](https://github.com/primo-solver/primo/blob/fcc274c23ff5b146cc43d80b1b0317faf91c7f08/docs/capabilities.md)
  and [`options.md`](https://github.com/primo-solver/primo/blob/fcc274c23ff5b146cc43d80b1b0317faf91c7f08/docs/reference/options.md).
- [`RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md`](../../research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md),
  which places general Boolean SMT backends as a solver-stage engineering arm,
  not an exponent result.
- [`research/prime_field_smt_20261005/RESULT.md`](../../research/prime_field_smt_20261005/RESULT.md),
  the existing prime-field `QF_BV`/`QF_FF` controls and frozen 20-bit result.
