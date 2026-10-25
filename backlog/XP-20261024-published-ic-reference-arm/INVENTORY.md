# Inventory: what exists for the published-IC reference arm, what must be built

Paths are on `origin/main` at `5cb553810e` (the worktree is sparse; `research/` read with
`git show origin/main:<path>`). "Exists" means a file in that tree; I did not run it unless
SMOKE says so. Estimates are engineer-days for one person who knows the repo, *predicted*.

## 1. Piece by piece

| piece | exists | where | gap |
|:--|:--|:--|:--|
| Semaev `S_{m+1}` and Weil descent over `F_2`-subspace | yes | `src/cryptanalysis/koblitz_groebner.rs` (`from_subspace_vars`, 467; `build_decomposition_system`); **`binary_semaev_s4.rs`: the `e_i`-symmetrised binary `S_4` (`e_1,e_2,e_3`, quadratic over the `e_i`) with a correspondence system over x-bits in `<1, z, ..., z^(l-1)>`, `b = 1` - the closer published (FGHR) construction and the M1 starting point**; `binary_semaev.rs` (`S_3`); `pq_descent.rs`, `pq_descent_symbolic.rs` (Petit-Quisquater Weil descent of Semaev systems over a subspace `V`); `symmetrized_semaev.rs` `decompose_symmetric` (line 350; FGHR rewriting into elementary symmetric functions, generic) | none of these is wired to a whole-solve for random (non-Koblitz) binary curves; the `icv1-f2m*` non-Koblitz ids have no descended-system builder |
| Subspace base on the **u-frame**, `F_u = {P : u(P) in V}`, `u = 1/(x+1)` (`koblitz_symmetrised.rs` 225-235), `V` random non-invariant | yes | `random_subspace_containing_one` (2311), `build_symmetrised_factor_base_from_basis` (236) | this is **not** the standard x-base. The x-frame base `{P : x(P) in V}` is `plain_subspace_factor_base` (2344, over the same `V`); Koblitz only, `n < 64`, `MAX_VARS` |
| Frobenius-invariant subspace base (second base for Koblitz) | yes | `koblitz_symmetrised.rs` `divisor_for_dimension` (150), `build_symmetrised_factor_base` (195) | none for Koblitz |
| u-frame 2-torsion invariants (lab-original, **not** FGHR `S_m`-symmetrisation), m = 2, 3 | yes (Koblitz, `u = 1/(x+1)`, `T = (0,1)`, `x -> 1/x`; per-point `w_i = u_i^2 + u_i` of degree 2 each, **no reduction to elementary symmetric functions**) | `koblitz_symmetrised.rs` `symmetrised_terms` (414; m = 2, 3 only), `build_symmetrised_system` (578), `lift_symmetrised_root` (717), `enumerate_symmetrised` (371), from `RESEARCH_EXOTIC_COORDINATES.md` | **m = 4: `symmetrised_terms` returns `None`** (414-443; card `XP-20261018` M1). Random binary curves: no generator. PROTOCOL §1 forbids lab-original ingredients, so this code is a plumbing stand-in only; the reference arm needs the `e_i`-symmetrised system of M1 |
| Galbraith-Gebregiyorgis variable choice | partly | `docs/ic/IC_TECHNIQUE_INTEGRATION_20261012.md` line 29 (item 6: `koblitz_sym_s4`, "special subspace, direct symmetric S4 at m=3"); papers cited at `RESEARCH_ISOGENY_FIBERS_ECDLP.md` 47, 589, 1807 | the special-subspace choice is not a base option in the build functions; no code selects `V` per the paper |
| FGHR / Joux-Vitse on prime extensions | `p^3` line (`fghr_line.rs`, `fghr_full.rs`); `p^4` `D_4` (`gaudry_quartic_d4.rs`, `d4_weighted_s5.rs`); **`p^5`: `joux_vitse.rs` (Joux-Vitse `(n-1)`-decomposition at `n = 5`, one trial measured and checked), `jv_quintic.rs`, `jv_quintic_edwards.rs`, `jv_quartic.rs`**; `symmetrized_semaev.rs` `decompose_symmetric` | `src/cryptanalysis/` | Joux-Vitse is the published method for small-degree extensions and belongs in the P4/P5 cells; no Gaudry/FGHR `S_6` for `n = 5`, and no msolve arm on these systems has been confirmed |
| Standard prime-extension base `{x in F_p}` | yes `p^3`, `p^4` | `gaudry_cubic.rs`, `gaudry_quartic.rs`, `gaudry_quartic_d4.rs` | `p^4` solver is the lab's own `D_4` elimination (`ic_next10_01_fp4_ladder`; "gaudry-d4:macaulay=14" in the ecbench README), **not msolve** |
| msolve as solver | ad-hoc only | `examples/semaev_rootfind_e2e_bench.rs` (`Msolve`, `-g 2`, paired control, CPU via rusage, prime `m = 2` ideal), `ic_next10_06_chain_export.rs` (writes GF(2) input), `ic_next10_03_pkm_degree.rs` (convert), `gb_cas_export.rs` | **not a `SystemSolver`**: `ic_framework/solvers.rs` registry lists `f4-f2`, `buchberger-f2`, `matrix-f4/f5`, `inherited-f4`, `crossbred-f2`, `xl-f2`, `sat-cdcl`, `fes-f2*`, `exhaustive`, no msolve. Root extraction from the msolve basis not implemented |
| msolve installed | v0.9.5 at `/opt/homebrew/bin/msolve` (`msolve -h`) | host | cards `XP-20261018-*` pin v0.6.5 built from source (`research/dreg_closure_m4l6_20261018/pod/boot.sh:85-89`); version drift to be A/A'd |
| WDSat | **absent** | `grep -ril wdsat` over the tree: only `README.md:121`, `AGENTS.md:996` and PR logs. Upstream `mtrimoska/WDSat` at the pin: `src/` (`main.c`, `wdsat.c`, `config.h`, `makefile`, ...), `benchmarks/{IC_S4,MQ}`, a prebuilt `wdsat_solver` (do not use), **no instance generator** | the repo must emit WDSat's inputs itself (CNF, CNF-XOR or ANF). CLI per `main.c`: `-i -o -a -n -l -m -x -b(br_sym) -c -s -g -t`; symmetry breaking is the solver-side flag `-b`; sizes (`__MAX_ANF_ID__`, `__MAX_DEGREE__`, `__MAX_ID__`, ...) are compile-time constants in `config.h`, so **rebuild per cell**; `m = 4` support is unverified. GPL-3.0; pin `61c6ff3f49445af2729274819edb27b85a89efc1` |
| Internal SAT arm (not WDSat) | yes | `solvers.rs` `sat-cdcl` (885), `semaev_sat.rs`, `binary_semaev_chain_sat.rs`, `koblitz_symmetrised::symmetrised_sat_decompose` (1045) | it is the lab's CDCL, not the published solver; cannot stand in for WDSat |
| Whole-solve harness (collection, linear algebra, verify) | yes | `ic` bin (`src/bin/ic.rs`, `ic bench`, `ic boundary`), ecbench `ic.pipeline` (`src/cryptanalysis/ecbench`) | oracle plug-in point `SystemSolver` exists, so msolve/WDSat adapters slot in; ecbench arms exist for rho |
| GAE pricing | yes for native counts | `docs/ic/calibration.json` (ratios `ns_per_word_xor`, `ns_per_row_op`, ...), `docs/ic/LEADERBOARD.md` Table B unit; `ic_boundary::calibrate_word_xor` | **no unit for an external process**; `semaev_rootfind_e2e_bench` charges msolve child CPU seconds (a clock). Calibration has char2 random n = 15, 18, 21, 24, 27, Koblitz n <= 41, **no Koblitz n = 43..61, no random binary 13/19/37, no `p^4`** (instances list); Table A's unit is a clock-priced addition, not GAE |
| Matched rho / verify | yes | Table B reference; ecbench `rho.negation` | none |
| Degree / F4 trace reader for msolve | partial | Python `run_f4.py` in `research/ic_next10_20261014/06-equivariant-f4-m4/` | native `msolve_degree_log` not built (card `XP-20261018` M0) |
| Specialisation (fix tower variables) | yes | `research/dreg_closure_specialise_20261017/` (`run.sh`, `RESULTS.md`) | not needed by this arm |
| Host with Callgrind | external | AGENTS/leaderboard use Callgrind `Ir` (Table "complete-solve Callgrind") | Valgrind does not run on this Apple-silicon macOS host; Linux isolation host needed |

## 2. What the smoke run showed (see SMOKE_20261024.md)

Run on Koblitz n = 13 (a = 0), 15, 17 at m = 3 with the u-frame system: 18 targets, msolve agreed with exhaustive
enumeration on 18/18 and 11/11 extracted relations verified in the group. Improvised there (system = u-frame invariants on a u-subspace base, not FGHR `S_4` on the x-base): random-subspace Koblitz instance (no random-binary builder), msolve invoked by
`Command`, verdict from GB `[1]`, root extraction by evaluation over `2^n_vars`, no linear
algebra, no GAE hook, CPU = child rusage. The precise results and limits are in that file.

## 3. Build list and honest estimates

| id | build | days | notes / risk |
|:--|:--|--:|:--|
| M1 | binary `S_{m+1}` descent + FGHR generator for **random** binary curves and **m = 4** (interpolate symmetrised `S_5` over `F_2`, T-invariants; the existing m = 2, 3 are Koblitz) | 8-12 | the FGHR invariants must be re-derived without Frobenius; m = 4 has 4(l-1)+1 unknowns and the m = 4 terms are unknown size (the m=3 form has 18 terms) |
| M2 | `MsolveSolver: SystemSolver` (write input, run, parse GB, **extract all roots**: FGLM from the grevlex basis or msolve's rational parametrisation if it supports `p = 2`, which I did not verify; else repo sparse-FGLM, `LEADERBOARD.md` "Linear stage after F4 (sparse FGLM)"), memory/time guards | 5-8 | root extraction is the real work; evaluation sweep (smoke) is inadmissible past ~24 unknowns |
| M3 | WDSat vendoring: fetch pin, build, generator adaptor (system -> its input), parse model, root lift | 5-7 | GPL-3.0 external process only; upstream generator is Weil-descent-specific so the repo's system is not the system WDSat solves unless re-derived; may need to run WDSat's own pipeline and only reuse our lift/verify |
| M4 | binary random-curve constructor + factor base `{x in V}` for `icv1-f2m*` non-Koblitz ids, shared by M1 | 3 | |
| M5 | whole-solve harness for the reference arm: relation collection until rank, linear algebra mod `r`, recover, verify; reuse `ic` pipeline stages and register the arm in ecbench | 6-10 | integration of external solvers with ecbench's isolation tiers |
| M6 | GAE for external processes: `Ir`-per-GAE ratio (Callgrind of msolve/WDSat and of the group-add loop on the Linux host); pin calibration for Koblitz 43..61, random binary 13/19/37, `p^4`, `p^5` | 6-9 | requires the Linux isolation host; calibration drift history in `calibration.json` `why` field |
| M7 | `p^4`: msolve arm on the `D_4` system (export from `gaudry_quartic_d4`/`d4_weighted_s5` as GF(p) input), p = 29, 37, 73, 97 | 5-7 | the lab's own `D_4` solve is the thing being compared; the msolve arm must be on the *same* system |
| M8 | `p^5`: Joux-Vitse arm on the existing `joux_vitse.rs`/`jv_quintic*.rs` systems as the published reference (plus Gaudry/FGHR `S_6` if a published `n = 5` symmetrisation is wanted), instance pinned, msolve | 8-12 | the code for Joux-Vitse exists (corrects my earlier 'no code' claim); what is missing is the msolve arm, the whole-solve harness and the calibration. Decide at freeze whether Gaudry `S_6` is also run |
| M9 | tuning sweeps and controls (§6, §7 of PROTOCOL), reports, leaderboard section, benchmarks row | 5 | |
| total | | ~45-70 engineer-days (about 2-3 months one person; 4-6 weeks with two) | REVIEW §5.1 F2's "one week to run" holds only after M1-M9; its "2-3 days to specify" excludes build |

Compute after build (predicted): binary cells are seconds to minutes at n <= 27; n = 37 and
Koblitz n = 41..61 at m = 3, 4 are the expensive cells (msolve memory is the wall, cf. the
99.6 GiB death at PKM m = 3 N = 21 in `pkm_m3_n21_msolve_20261018/RESULTS.md` as cited in REVIEW §1 item 3);
cap runs at 16 GB and record "did not complete" rows, not omissions.
