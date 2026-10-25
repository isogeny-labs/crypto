# DRAFT pre-registration: the published-IC reference arm

**DRAFT. NOT FROZEN.** Written 2026-10-24 for card `XP-20261024-published-ic-reference-arm`.
It freezes only when milestones M1-M5 of `INVENTORY.md` are merged, binaries and the WDSat
commit are recorded in §10, and the commit adding the frozen text is recorded in `card.json`
(`protocol.blob`). No cell is timed before then. Labels as in README.

## 1. Definition of "the reference arm"

For a cell (family, n, m, instance) the reference arm is the pipeline a published paper's
method section would specify, with no lab-original ingredient: no Z[tau] multiplier base, no
cm-/binary-multiplier closure, no twist/GLS closure, no equal-advice walks, no lab precomputed
tables. The lab producers are *candidates*, never part of the reference.

Published sources fixed by name: Semaev 2004 (`S_{m+1}`); Gaudry 2009 and Diem 2011 (descent,
standard base on `F_p` / a line); Petit-Quisquater 2012 (binary subspace base);
Faugere-Gaudry-Huot-Renault, J. Cryptology 2014 (ePrint 2012/199; 2-torsion symmetrisation,
invariants); Galbraith-Gebregiyorgis, INDOCRYPT 2014 (ePrint 2014/806; variable choice and
subspace choice in characteristic two); Trimoska-Ionica-Dequen (WDSat, SAT+XOR for Weil
descent; the repository's symmetry-breaking commit is the 2024 one). Citing the papers is the
spec; any detail this file does not fix is taken from them, and a deviation is recorded.

Note on the lab's own constructions: `koblitz_symmetrised.rs` (u-frame 2-torsion invariants, per-point
`w_i`, no reduction to elementary symmetric functions) is a lab-original construction. It may appear
as a *candidate*, never as the reference. The reference binary system is the `e_i`-symmetrised `S_{m+1}`
of FGHR (`binary_semaev_s4.rs`, `symmetrized_semaev.rs`, to be extended to m = 4: M1) on the x-base
`{P : x(P) in V}`. The 2026-10-24 smoke ran the lab's u-frame system and says so (SMOKE_20261024.md).

## 2. Cells

| id | family | sizes | m or n | base | symmetry | solver |
|:--|:--|:--|:--|:--|:--|:--|
| B3-F4 | binary random and K_0 | n in {13,15,18,19,21,23,24,27,37} | m = 3 | standard random subspace `V ∋ 1`, `l = ceil((n+1)/3)` | FGHR T-symmetrised `S_4` | msolve F4 |
| B3-SAT | same | same | m = 3 | same | same | WDSat |
| B4-F4 | same | n in {13,15,18,19,21,23,24,27} (n=37 only if M-cost allows) | m = 4 | `l = ceil((n+1)/4)` | FGHR `S_5` (T plus the symmetric group in the invariants) | msolve F4 |
| B4-SAT | same | same | m = 4 | same | same | WDSat |
| K3, K4 | Koblitz K_0/K_1 | n in {41,43,45,47,53,57,59,61} | m in {3,4} | (i) standard random `V`; (ii) Frobenius-invariant `V` (labelled second base, `x^n-1` root space, as `koblitz_symmetrised::divisor_for_dimension`) | FGHR T; plus Frobenius invariants only for base (ii) | msolve; WDSat |
| P4 | `E(F_{p^4})` | p in {29, 37, 73, 97} (+ 269, 1009 oracle-only) | n = 4 | `{P : x(P) in F_p}` | FGHR `D_4` (`gaudry_quartic_d4`); second arm Joux-Vitse `(n-1)`-decomposition (`jv_quartic.rs`) | msolve |
| P5 | `E(F_{p^5})` | p to be chosen at freeze, instance generated and pinned in `docs/ic/calibration.json` | n = 5 | `{x in F_p}` | Joux-Vitse `(n-1)`-decomposition (`joux_vitse.rs`, `jv_quintic.rs`, `jv_quintic_edwards.rs`: the published method for small-degree extensions); Gaudry/FGHR `S_6` only if a published `n = 5` symmetrisation is adopted at freeze (no code) | msolve |

Per cell: 8 targets, planted logarithm, `[d]G = Q` verified; 2 independent seeds for the
instance subspace `V` (the base is part of the draw).

## 3. What a solve is

Cold whole solve, every phase charged: factor-base generation, relation collection (each
`S_{m+1}` system built, solved, roots extracted and lifted, summands re-added in the group),
linear algebra over `Z/r`, recovery, verification. Relations collected until the matrix has
full column rank + 8 (or the reference-arm stopping rule of the cited paper if it states one).
No precomputed rho tables, no amortisation across targets, no setup excluded.
Relation systems with no solution are charged (they are most of the trials; compare
`gaudry_quartic_d4_ecbench` README "accounting gap").

## 4. Unit and pricing

GAE as in `docs/ic/LEADERBOARD.md` Table B: native counts converted at the dimensionless ratios
pinned per instance in `docs/ic/calibration.json`; instances without an entry are pinned
*before* any timed cell (M6), never left on the host fallback. Group operations, field
operations and Rust-side linear algebra are native counts. **External solvers have no native
count.** Primary pricing: msolve and WDSat run under Callgrind (instruction count, `Ir`) on the
Linux isolation host; `Ir` converts to GAE at an `Ir`-per-GAE ratio measured by Callgrind on the
repo's group-addition loop on the same host, pinned in the card. Secondary, reported but
non-admissible: child CPU seconds (rusage), which is the smoke run's only quantity. No wall
clock is ever a claim on a loaded host. Both numbers are always reported; the card does not
choose between them post hoc.

**Cross-unit control (before freeze).** Run one lab Rust solver (e.g. `f4-f2`) on the same B3 system
both natively counted (word XORs through `calibration.json`) and under Callgrind (`Ir` through the
`Ir`-per-GAE ratio). The two GAE figures must agree within a tolerance stated here before the run
(proposal: a factor of 1.5; the tolerance is part of the freeze). If they do not, **every arm,
including the lab's, is priced in `Ir`** and the native-count conversion is not used in this card.

## 5. Solver settings (fixed before running)

- msolve: pinned release (repo recipe builds v0.6.5, `research/dreg_closure_m4l6_20261018/pod/boot.sh`
  per card `XP-20261018`; the host binary is 0.9.5, so the version is *recorded*, and both are run
  on the n = 13 cell as an A/A), field `GF(2)` with the field equations `x^2 + x` appended
  (`ic_next10_06_chain_export`), `-g 2` reduced Groebner basis, one thread, memory cap 16 GB, no
  per-cell tuning of `-c`, `-t`. Roots: via the reduced basis and an FGLM/enumeration step charged
  to the arm (the smoke improvises evaluation, which is inadmissible beyond ~24 unknowns).
- WDSat: upstream `github.com/mtrimoska/WDSat`, GPL-3.0, external process only (not linked), pinned
  commit `61c6ff3f49445af2729274819edb27b85a89efc1` (2024-06-20, "Add symmetry-breaking technique";
  head of `master` when the GitHub API was queried 2026-10-24). At the pin the tree is `src/`
  (`main.c`, `wdsat.c`, `config.h`, `makefile`, ...), `benchmarks/{IC_S4,MQ}` and a prebuilt
  `wdsat_solver`; **there is no instance generator**, so the repo emits the input (CNF, CNF-XOR or ANF
  per the upstream README) itself. **Build from `src/` with `make`; the checked-in binary is never
  used.** CLI (from `main.c`): `-i <file> -o <file> -n <n> -l <l> -m <m> -x -b -c -s -g <mvc> -t <thr>`
  and `-a compile|time|delete`; `-b` is the symmetry-breaking switch (`br_sym`), `-x` Gaussian
  elimination; `wdsat_solve` also takes the field polynomial and `X3` argument. Problem sizes
  (`__MAX_ANF_ID__`, `__MAX_DEGREE__`, `__MAX_ID__`, `__MAX_BUFFER_SIZE__`, `__MAX_EQ__`,
  `__MAX_EQ_SIZE__`, `__MAX_XEQ__`, `__MAX_XEQ_SIZE__`, and `__FIND_ALL_SOLUTIONS__` for relation
  enumeration) are compile-time constants in `config.h`: **one rebuild per cell, constants recorded in
  the row**. `m = 4` support at the pin is unverified. The commit is a pin, not a verification that it
  builds or that its input format matches our system; M3 confirms, and a failure there is a recorded
  blocker, not a silent drop of the SAT arm.
- Variable choice (Galbraith-Gebregiyorgis): the choice of `V` among random subspaces is the
  published recommendation; if the paper's special-subspace choice applies, it is run as a second
  labelled base, not substituted.

## 6. Tuning parity

The reference arm gets the same tuning budget as the best lab candidate: `m in {2,3,4}` where
the paper allows it, `l` swept over `{l0-1, l0, l0+1}` with `l0 = ceil((n+1)/m)`, factor-base
size swept as the lab's `K` sweep is. The reported reference is the best verified cell of that
sweep. The sweep also includes the large-prime option `LP in {0, 1, 2}` wherever the lab candidate
uses large primes or the cited paper allows them (relations with up to `LP` summands outside the base,
matched to the candidate's accounting). Otherwise "x times published" is a strawman. Sweep points are listed here before the
run; the sweep itself is cold (each point a whole solve).

## 7. Controls

A/A: same arm twice per cell (counted metrics must agree; Ir within the Callgrind tolerance).
Positive control: the plain unsymmetrised arm (no FGHR) at the smallest n, expected strictly
costlier in system size (derived, `n_vars` and degree). Negative control: a target outside the
base's span (relation systems with no solution) must be refuted, not "solved". Matched rho
reference per `AGENTS.md` §5a band for every row.

**Literature-calibration control (before freeze).** Reproduce one published number with the pinned
tools: WDSat's own `benchmarks/IC_S4` instances (solve outcome and the solver's reported effort at the
instance sizes the repository ships), or a Galbraith-Gebregiyorgis 2014 (ePrint 2014/806) `m = 3`
timing or degree at a matching `n`. Record the expected value from the source, the measured value, and
the agreement criterion in §10. A reference arm that cannot reproduce a published number is not frozen.

## 8. Predictions (stated before any run)

- P-a: at n <= 27, m = 3, the reference arm's S is dominated by relation collection and is
  above matched rho (the lab's Table B IC rows are 13x-38x rho with a *tuned* m = 2 pipeline).
  Range: reference/rho in [5, 500]. Not a measurement; it will likely be wrong by a factor.
- P-b: WDSat beats msolve on B3 and B4 cells with n >= 19, as the SAT literature reports for
  larger Weil-descent systems; not for n <= 15.
- P-c: no lab producer is below the reference on every row; a lab producer below the reference
  on a row is a *candidate* finding to be replayed, not a result.

## 9. Gates and rules after freeze

Answers verified; every number from a committed row; no combining of rows from different
experiments; frozen evidence directory carries `VALIDATION.json` (docs/VALIDATION_GATE.md). A
deviation after freeze is an amendment recorded here, never an edit.

## 10. Pins (empty until freeze)

msolve sha256 / version; WDSat commit (above) and build log; `Ir`-per-GAE ratio; calibration
entries added; host; commit of the harness `examples/published_ic_ref_arm.rs` (not yet written).
