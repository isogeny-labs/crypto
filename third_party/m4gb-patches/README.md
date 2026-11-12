# M4GB patches

Patches applied by [`scripts/m4gb/build.sh`](../../scripts/m4gb/build.sh) on top
of the pinned upstream commit of
[cr-marcstevens/m4gb](https://github.com/cr-marcstevens/m4gb) (GPL-3.0). M4GB
is built as a separate binary and only ever run as a subprocess
(`src/cryptanalysis/m4gb_oracle.rs`); no M4GB source is linked into this
crate. The patches are published here under the same GPL-3.0 terms as the
code they modify.

| patch | what it changes | switch |
|:--|:--|:--|
| `0001-gf_p-arithmetic-vector-kernels.patch` | `lib/gf_p_simple.hpp`: the dense-row kernels `add_to`, `mul_to`, `substract_to` compute `l + c·r mod p` with plain integer arithmetic on a compile-time modulus instead of one table gather per coefficient, so the compiler vectorizes them (16 or 32 lanes per AVX2 operation). Scalar table operations are unchanged. | `-DM4GB_ARITH_KERNELS=0` restores upstream |
| `0002-field-equations-declared-variables-only.patch` | `lib/parser.hpp`, `src/m4gb.hpp`: over `GF(2)` the field equations `x_i² + x_i` are added for the variables the input declares (`$vars`) or uses, not for all `MAXVARS`, so one binary compiled for 128 variables does not drag 93 unused variables through a 35-variable computation. | `-DM4GB_FIELDEQ_DECLARED_ONLY=0` restores upstream |
| `0003-serve-mode-persistent-solver-process.patch` | `src/common.hpp`: a `--serve` mode. One solver instance reads `SOLVE <inputfile> <outputfile>` lines from stdin, solves each with the same `solve()` entry point (which re-initialises its state), writes the output file and answers `OK`, `UNSAT` (unit ideal) or `ERR <what>` on stdout; `QUIT` or end of input ends it. The process start, the field tables and the thread pool are paid once per process instead of once per system. The one-shot `--solve` path is untouched. Run it with `--loglevel 1` so the logger (which writes to stdout) does not interleave with the protocol; the client ignores lines that are not protocol words. | none; the mode is opt-in (`--serve`), `m4gb-f2-server` in the Rust crate uses it |

The two compile-time switches exist so an A/B run can build the baseline and the patched
solver from the same tree (`CPPFLAGS="-DM4GB_ARITH_KERNELS=0
-DM4GB_FIELDEQ_DECLARED_ONLY=0"`). Measurements and the protocol are in
`research/m4gb_integration_20261110/`.

To regenerate a patch after editing a checkout at the pinned commit:
`git diff -- <files> > third_party/m4gb-patches/<nnnn>-<slug>.patch`.
