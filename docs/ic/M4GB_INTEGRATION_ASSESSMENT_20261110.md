# M4GB as a Gröbner backend: integration assessment (2026-11-10)

Question from the owner: can we integrate
[cr-marcstevens/m4gb](https://github.com/cr-marcstevens/m4gb)
(Makarim and Stevens, *M4GB: An efficient Groebner-basis algorithm*, ISSAC 2017)?

Short answer: yes, as an **external subprocess backend** behind the existing
`SystemSolver` plug point, in the same way WDSat is wired in today. It is not a
drop-in for the prime-field tower engines, and it cannot enter a leaderboard
row until it counts operations. Everything below is a desk assessment
`[pcc:unverified]`; the build and smoke test could not run in this session
(the runtime refused to execute the downloaded build system).

## What M4GB is (read at source, commit `01860a52`, 2026-09-25)

| property | value | where |
|:--|:--|:--|
| licence | GPL-3.0 | `LICENSE.txt`, every header |
| language, build | C++11, autotools; one solver binary per `(MAXVARS, FIELDSIZE)` pair, both compile-time | `Makefile.am`, `src/config.hpp` |
| prime fields | `GF(p)`, `p` prime, log/antilog tables; a full multiplication table when `p ≤ 256`, an add-multiply table when `p ≤ 64` | `lib/gf_p_simple.hpp`, `src/config.hpp` |
| binary fields | `GF(2^n)`, `n ≤ 16`, fixed defining polynomials | `lib/gf_2n_simple.hpp` |
| `GF(2)` | field equations added automatically (`GB_ADD_FIELDEQUATIONS`), so it works in the Boolean ring our systems live in | `src/config.hpp` |
| order | degrevlex only; monomials packed into one 8-byte integer, so the degree cap falls as `MAXVARS` grows | `lib/monomial_degrevlex.hpp`, `INT_MONOMIAL_SIZE` |
| threads | `--nrthreads` exists; the header says `USETHREADS // NOT FULLY IMPLEMENTED` | `src/m4gb.hpp:44` |
| input | text: `$fieldsize 2`, `$vars 16 X`, one polynomial per line; or the Fukuoka MQ-challenge format | `lib/parser.hpp`, `README.md` |
| output | the reduced basis, monic, one polynomial per line (`--outputfile`) | `lib/solver_base.hpp` |
| counters | none exposed; log lines per pair batch (`lcmdegree=`) at verbose log levels | `src/m4gb.hpp:1769` |
| recent activity | PR #14 (A. Solovyev, merged 2026-09-25) fixed the Gebauer–Möller F-criterion (equal-lcm pairs discarded each other); the regression test is a Weil descent of `S₃` over `GF(2^16)` in the Huang–Kosters–Yeo §5.2 setup | `tests/gebauer_moeller/wd_n16.in` |

The last row matters: the maintainers' own regression test is the `m = 2`
summation-polynomial family this repository measures (`LEAD-05`,
`research/pdp_literature_leads_20261022/leads/LEAD-05.md`), and the handbook
idea H027 in cryptanalysis already names "F4 (msolve/M4GB)" as its instrument.

## Where it fits in this repository

1. **Boolean decomposition systems** (`src/cryptanalysis/ic_framework/stages.rs`,
   trait `SystemSolver`; registry in `ic_framework/solvers.rs`). A
   `BooleanSystem` is a list of `F2BoolPoly` over at most 128 variables
   (`Mask` is `u64` or `u128`). M4GB with `FIELDSIZE=2` takes exactly this.
   The adapter is the WDSat pattern (`src/cryptanalysis/wdsat_oracle.rs`):
   write the system in M4GB's text format, run the binary under the budget,
   parse the basis, read solutions off the linear elements, verify every
   point against the input (the registry already does this for every engine),
   return `BudgetExceeded` on timeout and never `Unsatisfiable` unless the
   basis is `{1}`. Binary located through an environment variable
   (`M4GB_BINARY`), as `WDSAT_BINARY` is.
2. **Small prime fields** (`src/cryptanalysis/f4_fp.rs`, the `F₂₉`/`F₃₁`
   coordinate thread). M4GB's benchmark field is `GF(31)`. The 2026-10-11
   comparison (`research/gb_cas_comparison_20261011/README.md`) found msolve
   12–24× faster than `f4_fp` on square generic systems `[pcc:unverified]` and level on the
   overdetermined ones; the M4GB paper reports M4GB ahead of msolve's
   ancestor FGb, Magma and OpenF4 on dense `GF(31)` systems. This is the cell
   where an external engine is most likely to show a real gap.
3. **Cross-check exporter** (`examples/gb_cas_export.rs`) already writes
   msolve, Singular and Magma inputs. Adding M4GB's default format is the
   cheapest first step (about 30 lines) and lets
   `research/gb_cas_comparison_20261011/run.sh` time it beside the others.

## Where it does not fit

- **Large prime fields and tower rings.** `f4_fp_tower.rs` and the PKM
  systems live over `F_p[y, u]/(towers)` with cryptographic `p`; M4GB's
  tables need `p` to be small (practically `p < 2^16`). No path.
- **`GF(2^n)` beyond `n = 16`.** The Koblitz fields (`n = 83`, `131`) are out of
  range as coefficient fields; only their Weil restrictions to `GF(2)` apply.
- **Decision workloads.** The decomposition oracle decides consistency; our
  inherited F4 does that in 11–24 ms at `n = 17` while a full reduced basis
  takes msolve 42–46 s on the same systems (same README). M4GB computes a
  full basis. Unless it reaches `1` early, it will lose that cell by orders of
  magnitude; this is the first thing to measure, not assume.
- **Accounting.** `AGENTS.md` §6: an engine that reports only wall time is a
  reference arm, not a leaderboard row. M4GB exposes no counter. A small
  patch counting field multiplications and add-multiplies in
  `gf_p_simple.hpp` / `gf_2n_simple.hpp` (one counter per table hit) would
  give a native unit; under GPL-3 we publish the patch, which is fine for a
  separate binary. Linking it into the Rust crate is not proposed.

## Plan, if the owner wants it

| step | work | output |
|:--|:--|:--|
| 1 | Build `solver_m4gb_n128_gf2` and `solver_m4gb_n20_gf31` on an isolab image (pinned commit, SHA-256 of the binary in the record); run `make check` and `tests/gebauer_moeller/wd_n16.in` | build receipt |
| 2 | `gb_cas_export`: add `.m4gb` output; extend `run.sh` with an m4gb arm | inputs beside the msolve ones |
| 3 | Re-run the 2026-10-11 comparison with the m4gb arm on the 13 `f4_fp_bench` systems and the 8 `bool_n17` targets, 3 repeats, same host, interleaved | one report, graph, PDF, benchmark row |
| 4 | Only if step 3 shows a gap: `m4gb-f2` adapter in `solvers.rs` behind `M4GB_BINARY`, `accepts` limited to `n_vars ≤ 128`, wall-time priced, `op_unit` "none (external)" until the counter patch lands | registry entry, docs/ic/FRAMEWORK.md row |

Pre-registered expectations (falsifiable in step 3): on the square `GF(31)`
systems M4GB finishes within 2× of msolve (paper claim; if it is slower than
`f4_fp` the lead is dead); on the Boolean `n = 17` decision targets M4GB is
slower than inherited F4 by more than 100× (if it is within 10×, its
early-exit behaviour deserves a look at larger `n`).

## Sources checked

- KB unavailable: `scripts/research-kb status` reports no executable;
  the `research-kb` MCP server is not attached to this session. The lead
  atlas in crypto-autoresearcher is not checked out here.
- `crypto:research/pdp_literature_leads_20261022/leads/LEAD-05.md` (M4GB PR #14, Solovyev);
  `crypto:research/gb_cas_comparison_20261011/README.md` (msolve, Singular, PolyBoRi numbers);
  `crypto:src/cryptanalysis/ic_framework/{stages,solvers}.rs`;
  `crypto:src/cryptanalysis/wdsat_oracle.rs`; `crypto:examples/gb_cas_export.rs`;
  `crypto:src/cryptanalysis/f4_fp.rs`, `f4_fp_tower.rs`; `crypto:docs/ic/FRAMEWORK.md` §SystemSolver;
  `cryptanalysis:research/handbook-ff-ic-ideas-20261103/IDEAS.md` (H027).
- Upstream: `cr-marcstevens/m4gb` at `01860a52dacb111d6e1533bc7547b2dc8f034073`
  (`README.md`, `src/config.hpp`, `src/common.hpp`, `src/m4gb.hpp`,
  `lib/gf_p_simple.hpp`, `lib/gf_2n_simple.hpp`, `lib/parser.hpp`,
  `lib/solver_base.hpp`, `tests/gebauer_moeller/wd_n16.in`).

## Implementation (same day, PR #3812)

The plan above was carried out in the follow-up PR, with these deviations
from the order proposed:

- Build and patches first: `scripts/m4gb/build.sh` (pinned commit, manifest
  with hashes) and `third_party/m4gb-patches/` (0001 arithmetic `GF(p)` dense
  kernels, 0002 field equations for declared variables only; README there).
- Measurement: `research/m4gb_integration_20261110/` (protocol, A/B, graph,
  PDF; benchmark record `20261110T0551Z-m4gb-arith-kernels-ab`). `GF(31)`
  3.4–4.1× and `GF(65521)` 1.6–2.0× over upstream with identical bases; `GF(2)`
  unchanged under exact-width binaries; wide binaries are memory-bound
  (about 7 GB resident at 128 variables) `[pcc:unverified]`.
- Adapter: `src/cryptanalysis/m4gb_oracle.rs` and `m4gb-f2` in
  `ic_framework/solvers.rs`, behind `M4GB_BINARY` (a binary or a directory of
  binaries, the narrowest that fits is chosen); `.m4gb` output in
  `examples/gb_cas_export.rs`; `docs/ic/FRAMEWORK.md`.
- Corrections to the table above: M4GB signals the unit ideal by exit status
  1 and `contradiction: I=(1)` or `contradiction: 1=0` on stderr with no
  output file, not by printing `1`; upstream prints `^2 + ` lines for the
  field equations of undeclared variables (removed by patch 0002); and the
  thread option does run work on several threads (user time exceeded wall
  time), the header comment notwithstanding.
