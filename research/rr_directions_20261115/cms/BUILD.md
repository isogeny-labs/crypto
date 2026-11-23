# CryptoMiniSat 5.11.21 build (driver around the library; NOT the upstream CLI)

## Source provenance
- Requested: GitHub tag tarball `https://github.com/msoos/cryptominisat/archive/refs/tags/5.11.21.tar.gz`.
  Result: blocked. The proxy answered "GitHub access to this repository is not enabled for this session"
  (the downloaded 378-byte body is a JSON error, sha256 b3e03aaaff81d68730d67392a135ac3fdfdf022880c66632ab188d2fe084cda3);
  codeload.github.com and api.github.com gave HTTP 403. `add_repo` for msoos/cryptominisat was denied by the permission
  classifier and not retried or worked around.
- Fallback used: PyPI sdist `pycryptosat-5.11.21.tar.gz`
  URL `https://files.pythonhosted.org/packages/18/34/6568c0feda88a83f638bd1a79c924eb6f9a19d697b9dd444fa6e19fca744/pycryptosat-5.11.21.tar.gz`
  sha256 `8ddc5a265bfcf0c63233f036e63c623c492b03327ced51c8bec372d950601501` (equals the digest PyPI's JSON lists).
  (`pip download --no-binary :all:` itself failed in the isolated build-deps step; the file was fetched with curl.)
- The sdist holds the library sources (`src/*.cpp`, `src/dimacsparser.h`, picosat, oracle) and the Python wrapper, but
  no `main.cpp`/`cryptominisat5` command-line program, no CMakeLists.txt, no cadical/cadiback, no m4ri/breakid use.
  So `cryptominisat5` and its `--help` / `--version` do not exist in this build.
- Unpacked at `/tmp/claude-0/-home-user/87aed7c9-04ad-528e-8252-34148b4b4848/scratchpad/cms_src/src/pycryptosat-5.11.21`
  (outside the repository). Version string: `pyproject.toml` version = "5.11.21", compiled in as `CMS_FULL_VERSION="5.11.21"`.

## What was built instead
- `cms_driver.cpp`: a ~60-line stand-in for `cryptominisat5.cpp`. It reads the file with the library's own
  `DimacsParser` (the code that parses `x` lines), creates a `SATSolver`, sets 1 thread, verbosity, seed 0
  and `set_max_time`, calls `solve()`, prints `print_stats()` and `c DRIVER ...` summary lines
  (conflicts, decisions, propagations from `get_sum_*`), `s SATISFIABLE|UNSATISFIABLE|INDETERMINATE` and a `v` line.
  Options: `--verb N --maxtime S --gauss 0|1 --seed N FILE`. No other CLI option exists.
- `cms_shim.cpp`: `#define private public`, then `#include "cryptominisat.cpp"` unchanged (the library source is not
  modified), plus two helpers that reach `conf.gaussconf` (the public `SATSolver` API has no setter for it).
- All other solver conf values are library defaults, except `set_num_threads(1)`.

## Build
`./build_driver.sh SRC_DIR BUILD_DIR OUT_BIN` (compile commands in `build_cmds.txt`):
`g++ 13.3.0 -O3 -DNDEBUG -std=c++17 -I src -I src/picosat -DTRACE= -DCMS_FULL_VERSION="5.11.21"` on the source list of the
sdist's `setup.py` (with `cryptominisat.cpp` replaced by the shim), picosat.c/version.c with gcc -O3, linked with `-lpthread`.
No cmake used (no CMakeLists in the sdist); no zlib, M4RI, Bliss, BreakID, sqlite, or cadical.
- Binary: `/tmp/claude-0/-home-user/87aed7c9-04ad-528e-8252-34148b4b4848/scratchpad/cms_src/build/cms_driver`
  sha256 `f5425ac560303c88bdb8060ec4a6530da94230ca8e2563390c4df23ade769ea8`
- `cms_driver --version`: not available (prints usage). The `c GAUSSCONF ...` line at startup records the active config.
- Hashes of the files in this directory used for the build: cms_driver.cpp 929034ca...e392ed, cms_shim.cpp f6e54e7e...6671,
  build_driver.sh 5b959733...e93d, check_model.c e2583b14...4eef (full values: `sha256sum` them).

## Gauss-Jordan on / off
- Library default (`GaussConf`, solverconf.h): `max_num_matrices=5`, `autodisable=true`, `max_matrix_rows=2000`,
  `max_matrix_columns=1000`, `min_gauss_xor_clauses=2`. `--gauss 1` leaves these untouched: startup prints
  `c GAUSSCONF max_num_matrices=5 ...` and the log shows `[matrix] Good matrix ...`.
- Off: `--gauss 0` sets `gaussconf.max_num_matrices = 0` (the field upstream's `--maxnummatrices` sets). Chosen because with
  zero matrices no XOR matrix is built: the log shows `[matrix] UNused matrix ...` and `GAUSSCONF max_num_matrices=0`;
  XORs are still found/encoded as CNF clauses, so propagation uses the clause encoding only. `autodisable` is not used
  because it is an adaptive switch that still builds matrices.

## Convention and model check
`check_model.c` (C): checks every line of the CNF against the `v` line (clause: some literal true; `x` line: parity of the literal
truth values = 1; empty clause: violated). Tiny hand cases (`x 1 2 0` with `x -2 3 0` and `1 0` gives model `1 -2 -3`;
`x 1 2 0` + `x -1 2 0` UNSAT; empty clause UNSAT) behave as the panel convention requires. Every SAT result in results.jsonl
carries `model_ok`.
