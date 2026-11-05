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
or cryptanalytic speedup is claimed without a separately registered, matched
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
adapter adds Primo's `--quiet` flag and requests `get-model`; cvc5, Z3, and
generic SMT-LIB callers retain their existing `get-value` behavior.

## Verify the adapter

With the binary installed or `SMT_PRIMO_BIN` set:

```sh
cargo test --release cryptanalysis::smt_oracle::tests::primo_
```

The round trip enumerates a small known solution set, independently evaluates
every model, blocks each model, and requires a final `unsat`. This establishes
protocol compatibility on the fixture only; it is not a solver benchmark.
