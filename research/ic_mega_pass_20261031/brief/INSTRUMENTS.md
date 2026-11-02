# INSTRUMENTS — runnable toy tooling for falsifier experiments (verified 2026-10-31)

Every claim below was checked by running the command shown, in this sandbox (macOS, no Sage, no
network to crates.io, writes only under `$TMPDIR` / `research/ic_mega_pass_20261031`).
"OK" = ran; "FAILS" = observed failure with the reason.

## 0. Environment facts (measured)

| item | value |
|---|---|
| `python3` | pyenv shim → `/Users/adamburan/.pyenv/versions/3.9.10/bin/python3` (3.9.10). numpy 2.0.2, scipy 1.13.1, sympy 1.14.0, cypari2 2.2.2, torch 2.8.0 all import. |
| `python3.13` | `/Library/Frameworks/Python.framework/Versions/3.13/bin/python3.13`: numpy, scipy 1.15.3, sympy 1.14.0, cypari2 import. Use if a 3.11+ feature is needed (csd's `pyproject` says `>=3.11`, but all six target modules ran on 3.9 thanks to `from __future__ import annotations`). |
| `gp` | `/opt/homebrew/bin/gp`, PARI 2.17.3. `gp -q <<'EOF' … EOF` works. |
| `msolve` | `/opt/homebrew/bin/msolve` 0.9.5. Prints a harmless `OMP: Warning #179 … /tmp file` line in the sandbox. |
| SAT | `/opt/homebrew/bin/cryptominisat5`, `/opt/homebrew/bin/kissat` present (not exercised). |
| `cargo` | 1.93.1. **Cannot build** the crypto crate here (see §3). Use the 690 prebuilt release example binaries in `/Volumes/SSD990-2/crypto/target/release/examples/` (built 2026-10-26). |
| cypari2 first use | prints `*** Warning: new maximum stack size = 2000011264` once (from `csd.genus3.pari()` setting parisizemax 2 GB). Harmless. |

## 1. `/Volumes/SSD990-2/ml-cryptanalysis/csd/*` (pure Python + numpy/scipy + cypari2)

Import without installing:
```python
import sys; sys.path.insert(0, "/Volumes/SSD990-2/ml-cryptanalysis")
from csd import semaev as sm, fpn_degrees as fd, binary_ghs as bg, genus3 as g3, field, curve
```
`csd/__init__.py` is a one-line docstring; nothing heavy is imported at package level (torch is only
pulled by `hardness*/m7/gnn`, not by these six). Smoke (all six imports + the calls below): 1.75 s +
3.0 s wall.

### 1a. `csd/semaev.py` (228 lines, deps: stdlib only) — OK
Semaev summation polynomials over **Z[a,b]** for `y² = x³+ax+b`, sparse dicts `{exp_tuple: int}`,
variable order `(x_1..x_m, a, b)`.
- `S3()` (13 terms), `S4()` = Res_X(S3(x1,x2,X), S3(x3,x4,X)) (540 terms, total degree 12), `semaev(m)` (generic, resultant recursion; m=5 is already large — not timed), `total_degree(f)`.
- `elementary(k, nv)`, `symmetrise(f, k)` (rewrite symmetric-in-first-k poly in e_1..e_k, FGHR-style), `symmetric_semaev(n)` (S_{n+1} in e_1..e_n, x_R, a, b; n=3 → 143 terms), `desymmetrise(g, k)`.
- `specialise(f, keep, values, zero, one, add, mul, from_int)` maps into any field given as callables; `padd/pmul/pscale/var/const` for Z-poly arithmetic.
- Caveat: char-0 formulas (`−2`, `4b`) — for char 2 use `binary_semaev.rs` / `ffd_semaev_core.py` formula `(X1X2+X1X3+X2X3)² + X1X2X3 + a6`, not this module.

### 1b. `csd/fpn_degrees.py` (1144 lines, deps: numpy, **scipy.sparse** (inside `macaulay_scan`), cypari2 via `csd.genus3.pari()` for `Curve.card()`) — OK
Weil descent of `S_{n+1}` over `F_{p^n}` with base `{x ∈ F_p}` and exact Macaulay-rank degree labels.
- `GF(p, n, modulus=None)` tower arithmetic (`add_s/mul_s/inv_s/frob_s/norm_s/is_square_s/from_int/random`), vectorised `*_v` variants; `default_modulus(p,n)`.
- `Curve(F, a, b)`: `card()` (PARI ellcard), `j()`, `add_s/neg_s/mul_s/on_curve`, `factor_base()`, `n_sums(nsum)`, `random_point(rng)`, `twist(d)`, `iso_model(u)`; `curve_from_j`, `isogenous_js(E, ell)`.
- Descent: `descended_system(E, xR, kind)` with `kind="E"` (symmetrised S_{n+1} in e_1..e_n, any n with `symmetric_semaev(n)`; n=2,3 intended) or `"X"` (unsymmetrised S_3, n=2 only) → list of F_p polys as dicts; `weil_components`, `field_equations(nv, p)`.
- Degree labels: `macaulay_scan(polys, nv, p, dmax, stop_on_one=False, on_degree=hook)` (kernel recursion; asserts `p < 46341`), `macaulay_rank_dense` (check), `hilbert_series`, `d_reg_formula`, `system_labels(polys, nv, p, dmax)` → dict with `d_ff, d_excess, d_dev, d_ff_harness, D_GB, D_sat, D_cert, deg_I, profile=[{D,rows,cols,rank,rank_gen,...}]`, `refutation_degree(polys, nv, p, dmax)` → `D_star` (adds `v^p − v`; note D* ≥ p over F_p, so only useful for tiny p), `top_form_profile`, `try_certify` (border-basis certificate → multiplication matrices), `fp_points_from_certificate`, `enumerate_fp_points`, `split_over_fp`, `decompositions_from_abscissae(E, R, xs)`.
- Smoke: `GF(7,2)`, `Curve(F,2,3)`, `card()=60`, `descended_system(E, xR, "E")` → 2 quadrics; `system_labels(..., dmax=6)` → `{d_ff: None, deg_I: 4, D_cert: 3, D_GB: 3, D_sat: 2}`; `refutation_degree(..., 10)["D_star"] = 8`. Sub-second.
- Caveat: dense numpy int64 kernels — columns `C(nv+D, nv)` beyond ~10⁴ get slow; p must be < 46341; prime p only (no char-2 path here).

### 1c. `csd/binary_ghs.py` (1103 lines, deps: stdlib, cypari2 for `ellcard`, modular polys, irreducibility) — OK
GHS magic numbers / descent-route pricing over `F_{2^N}`, `E: y²+xy = x³+ax²+b`.
- `GF2n(N, modulus=None)` int-bitmask field (`mul/sq/inv/frob/trace/relative_trace/sqrt/minimal_subfield_degree/to_pari/from_pari`), `default_modulus(N)` (PARI-checked irreducible), `towers(N)`.
- `ghs_magic(F, l, a, b)` → `GhsMagic(l, n, magic_b, trace_a, a_may_add_one, magic, orbit_length, span_b, audit_magic, audit_type_i)`; `audit_order(F,a,b)` over all towers; `ghs_genus_for_type(m, type_i)`; `annihilator_degree`.
- `CostModel` (frozen literature formulas), `descent_routes`, `binary_screen(F,a,b,trace,subgroup_bits)` → `{reference_log2, verdict, routes, best_route}`, `price_jacobian_route`.
- Isogeny-class tools: `ellcard(F,a,b)`, `a_for_order`, `ell_neighbours(F, j, ell)`, `walk_class`, `volcano_level`, `frobenius_conductor`; `planted_b(F, l, target_degree, rng)` (plant a b with a chosen magic number); `f2_poly_divisors_of_xn1`.
- Smoke: `GF2n(15)` (modulus 0x8003), `ellcard=32862`, three towers' magic numbers printed. 3 s including PARI start.
- Caveat: labels are formula-level (no cover constructed); `N` up to ~64–131 fine for arithmetic; `walk_class` with `cap` guards cost.

### 1d. `csd/genus3.py` (552 lines, deps: cypari2) — OK
Smith (2,2,2)-isogeny census, genus-3 hyperelliptic `y² = F(x)` (deg 8) over F_p → plane quartic.
- `pari()` (shared cypari2 instance, 512 MB stack), `pol(coeffs)`, `random_octic(p, rng)`, `factor_pattern(p, F)`, `weierstrass(p, F)`, `stable_pairings(W)`, `analyse_pairing(W, m)` → `PairingRecord` (trigonal-rationality Legendre bit, isogeny rationality), `count_H`, `count_X` (point counts certifying the isogeny), `analyse_curve(p, F)` → `{p, F, pattern, n_stable, chi_lcF, chi_discF, sgn_roots, pairings, has_rational_trigonal, has_rational_isogeny}`.
- Smoke: `analyse_curve(31, random_octic(31))` OK, sub-second.
- Caveat: `count_X` enumerates fibres over F_p (O(p)) — keep p ≤ ~10⁴ per curve.

### 1e. `csd/field.py` (263 lines, stdlib) — OK
`is_prime, factor, largest_prime_factor, legendre, sqrt_mod (Tonelli), primitive_root_of_unity, multiplicative_order, primes_in_range, random_prime(bits, rng), fundamental_discriminant, reduced_forms_*`, `class_number(D)` (smoke: `class_number(-23)=3`). Trial-division factor — fine for ≤ 2^50.

### 1f. `csd/curve.py` (260 lines, numpy + `.field`) — OK
Prime-field short Weierstrass toy curve. `Curve(p, a, b)` frozen dataclass; `.j`, `.discriminant`, `.aut_order` are **properties** (not methods); methods `contains, neg, add, dbl, mul(k,P,ctr), random_point(rng), lift_x, order()` (Legendre sweep below 2^24, BSGS above), `order_bsgs`, `twist()`, `automorphisms()`; `OpCounter` for group-op accounting; `quadratic_character_table(p)`, `curve_from_j(p, j, twist_bit)`, `random_curve(p, rng)`. Smoke: `random_curve(101)` order 102.

## 2. `/Volumes/SSD990-2/crypto-autoresearcher/research/verification/`

39 files: 13 `.py`, 5 `.sage`, rest are result JSON/TXT. **Sage files are unusable here** (Sage hangs
in this sandbox). Scripts write results to **cwd** — run them from `$TMPDIR` via
`python3 -I -c "import sys; sys.path.insert(0,'<dir>'); exec(open('<dir>/X.py').read())"` or copy.

| file | engine | status | notes |
|---|---|---|---|
| `ffd_semaev_core.py` | pure python | **OK** (library + `__main__` sweep) | Binary Weil-descended S_3 over subspace: `MODS` (irreducible polys n=2..), `build_system(n, nprime, a6, xR, basis)`, `fall_profile(eqs, N, Dmax)`, `first_fall(prof)`, `degree_profile`, `random_matched(eqs,N,rng)` (null control), `a_real_xR`, `run_cell(n, nprime, seed, Dmax, a2, a6)` → dict with `semaev_ffd, null_ffd, semaev, null`. Smoke `run_cell(5,2)` → `{N:4, neqs:5, semaev_ffd:2, null_ffd:2}` 0.4 s. `__main__` writes `ffd_summary.json` to cwd. |
| `ffd_semaev_measure.py` | pure python (imports core) | **OK** 21 s | Sweep n=6..7, n'=3..4; writes `ffd_semaev_results.json` to cwd. |
| `ffd_s3_formula_control.py` | pure python | **OK** 0.5 s | Verifies binary S_3 formula `(X1X2+X1X3+X2X3)²+X1X2X3+a6` on real points (560/560). Reusable char-2 S_3 reference. |
| `ffd_frobenius_witness.py` | pure python | **OK** 1.9 s | Checks `μ = x_R^{-2}` kills the degree-2 part of descended S_3 for every subspace (d_ff ≤ 2 witness). |
| `falldeg_weil_dreg_check.py` | pure python | **OK** 65 s wall / 19 s CPU | Tests CCG Prop 4.13 `d_reg(Weil F) = n·d_reg(F) − n + 1` on tiny quadratic systems over F_2^n (4/4 match). Slow-ish; self-contained F_2 linear algebra. |
| `thm_incbarrier2_check.py` | numpy | runnable (not run) | Exact chord-profile identity for incidence barrier; writes report JSON to cwd. |
| `semaev_char23_checks_20261029.py` | pure python | runnable (not run) | `python3 semaev_char23_checks_20261029.py OUT.json`; exhaustive S_3 checks in char 2 and 3 by group law. |
| `commutator_kernel1_check.py` | python | needs `runs/RUN-NCP-001-a/raw.json` artefact | Replays a non-commutative quiver experiment; not a toy tool. |
| `replay_ffd_semaev_20261022.py` | python (argparse) | needs archived draw JSON | Truth-table/rank replays of frozen Semaev draws; exports `parity_rows_for_degree`, `MODS`, `mul_ref`. |
| `generate_semaev_*_rank_certificate_20261022.py` (2) | python → **Lean** | generator runs, consumer needs Lean/Mathlib | Emit Lean matrix-rank witnesses. |
| `check_ffd_semaev_lean_literal_20261022.py`, `check_semaev_finite_lean_semantics_20261022.py` | python | need Lean log / evidence files | Transcription checks only. |
| `thm_bkkmv1_verify.sage`, `thm_bkkmv1_verify_m5.sage`, `thm_bkkmv2_verify.sage` | **Sage** | FAILS here (Sage hangs) | Symbolic S_m over QQ[a,b]: support sizes 13/439/54777, projection saturation, u_m bottom fibres. Port target: `csd.semaev.semaev(m)` + sympy for m ≤ 4. |
| `thm_incbarrier1_check.sage`, `thm_jetbarrier1_check.sage` | **Sage** | FAILS here | Incidence-barrier richness and jet-barrier symbolic lemmas; results archived in the adjacent `*_output.json`. |

## 3. Rust examples in `/Users/adamburan/ic-work/crypto/examples/` and `src/cryptanalysis/`

**Build verdict: `cargo run --release --example X` is NOT plausible in this sandbox.**
- `cargo metadata --offline` FAILS (needs crates.io; local `~/.cargo/registry` 1.0 GB incomplete); online FAILS (`static.crates.io:443` denied by sandbox proxy).
- 4 of the 9 `[[example]]` entries in `Cargo.toml` point at paths missing from the sparse checkout (`experiments/koblitz-s3-pair-query-20261007-v4/{baseline,candidate}.rs`, `experiments/koblitz-n53-compact-cold-20261009/native_{replay,run}.rs`) → cargo would refuse the manifest even offline.
- Heavy deps (aws-sdk-s3, tokio, rusqlite bundled, rayon…) → a cold release build would be many minutes even with network.
- `/Users/adamburan/ic-work/crypto/target` does not exist; the crypto tree is not writable from this task anyway.

**Use instead:** prebuilt binaries `/Volumes/SSD990-2/crypto/target/release/examples/<name>` (690 of them, built 2026-10-26 from the full tree, 514 examples) and bins `/Volumes/SSD990-2/crypto/target/release/{ic,icx,quartic-ic,hyperelliptic-cover,jacobian_transfers,ghs_screen,…}`. Examples are auto-discovered (`autoexamples` default), so every `examples/*.rs` is a target name. 37 ic-work examples have **no** prebuilt binary (added after Oct 26), among them: `semaev_lattice_reach`, `semaev_gap_census`, `semaev_prime_structure`, `semaev_shift_search`, `prime_fall_degree_tools`, `fall_degree_e11_tools`, `pkm_symmetric_complete`, `dreg_invariant*`, `dreg_linear_guess*`, `dreg_chain_solutions`, `weighted_proj_cover_identity`.

Many examples hand-roll argv parsing: `--help` panics with "unknown argument" (e.g. `ffd_s4_subspace`, `koblitz_x5_fall`, `jv_cover`) — read the `//!` header in the `.rs` for flags. Some write into the repo tree and panic with `PermissionDenied` here (`odd_char_ghs_genus`, `pkm_coverage`); pass an `--out`/`--dir` under `$TMPDIR` where supported.

Verified-running prebuilt examples (no args unless shown):
| binary | what | flags / notes |
|---|---|---|
| `ffd_sweep_demo` | first-fall degree of unrestricted binary S_3 descent, n=3..7, rank vs generic table | none; instant |
| `pc_degree_demo` | refutation degree D* (PC degree) of unsat subspace S_3 instances, n=4..8 | none; instant |
| `hky_last_fall_degree` | HKY last fall degree d_F of descended S_3 over subspace (m=2), JSONL rows `{ffd,d_last,e0,closure_dims,ideal_dims,…}` | `cell --n N [--family random\|subfield] [--target rand\|sat] [--draws K] --seed S [--cmax C]`; `replay RECORDS.jsonl`. Smoke n=11: 0.06 s |
| `semaev_higher_demo` | S_3..S_6 in last variable over F_271, degree progression 2,4,8,16 | none |
| `semaev_profile` | SAT-based S_3 pipeline conflict profile (n15l5) | none; 1.2 s |
| `semaev_decomp_bench`, `semaev_scaling`, `semaev_rootfind_scaling_bench` | decomposition oracle benches (binary / prime) | none; seconds |
| `pkm_symmetric_tower` | symmetric PKM tower F4 solving degree, JSONL | `--p --m --arms --budget --t-max-m2 --t-max-m3 --random`; runs default sweep without args |
| `blocked_macaulay_bench` | total-degree Macaulay vs per-block bound refutation cost table | none |
| `descent_degree_cost` | priced table of descent cost vs degree model (m=3) | none |
| `smallroot_joint_lattice` | joint Coppersmith lattice dims for (m,w)=(4,2), JSONL | `--control` |
| `lit2026_w10_coppersmith_bound` | S_3/S_4/S_5 Coppersmith reach report (markdown) | `--json --max-ell --part` |
| `klein_descent`, `diem_descent_demo`, `ghs_attack_demo` | Klein-invariant descent over F_{p^k}; Diem toy over F_25; GHS pipeline demo | none |
| `macaulay_dense` | Boolean Macaulay refutation at one degree from Singular ideal files | `--file cell.sing --degree D [--threads] [--mem-gb]` (panics without `--file`) |
| `dreg_closure` | XL vs MutantXL closure on committed draws | `[--max-cols N] FILE.jsonl…` (needs draw files) |
| `ic` (bin) | clap CLI: `descent` (Weil-descent degree reached vs semi-regular, `--cells n:n':m --families K,R --targets --json`), `corpus` (writes Weil-descended S_4 instances as `.anf/.dimacs/.magma`, `--degree n --dimension l --sat --unsat --dir`), `boundary`, `bench`, `rho`, `prime`, … | `ic descent --families K --cells 11:6:2 --targets 2 --json`: 0.09 s. `ic corpus --degree 9 --dimension 3 --sat 1 --unsat 1 --no-cnf --dir $TMPDIR/x --json` OK. |

`ic corpus` `.anf` → msolve: one GF(2) poly per line (`*`=AND, `+`=XOR, `=0` implied, comment lines `c`, header `p anf`), variables `x1..`, `e1..`. Verified converter: strip `c`/`p` lines, collect variables, append `v^2+v`, write msolve file; n9l3 UNSAT instance → msolve `-g 2` returns `[1]` after F4 steps at degrees 2,3,3,3 (0.2 s).

Library modules (names only, in `src/cryptanalysis/`): `binary_semaev.rs` (`binary_semaev_s3`, `binary_semaev_s3_in_x3`, `binary_semaev_s4`, `solve_artin_schreier`, `solve_quadratic_f2m`), `binary_semaev_s4.rs` (`weil_descend_s4`, ANF arithmetic), `ffd_harness.rs` (`weil_descend_s3`, `weil_descend_s3_subspace`, `measure_polys`, `run_sweep`, `build_macaulay_rows_monomial`), `pc_degree_harness.rs` (`measure_refutation_degree`, `refutation_scan`, `restrict_to_subspace`, `sparse_rank_and_refute`), `symmetrized_semaev.rs` (`symmetrise_s3`, `elementary_symmetric`, `power_sum`, `decompose_symmetric`), `semaev_higher.rs` (`s3_in_last`…`s6_in_last`, `sylvester_resultant_in_x`), `semaev_prime_poly.rs` (`summation_poly(m, z)`, `interpolate_grid`, `support`, `restrict_to_line`), `lattice.rs` (`lll_reduce`, `lll_reduce_hp`, `bkz_reduce`), `sparse_macaulay.rs`, `f4_fp.rs` (degree-bounded F4 over F_p, 7 kloc), `groebner_f4.rs` (Buchberger), `ic_descent_degrees.rs` (`semi_regular_degree`, `price_descent_cell`), `gaudry_cubic.rs` (`SymmetrisedS4::weil_restrict` over F_{p^3}), `weil_charts.rs`, `koblitz_symmetrised.rs`, `pq_xl.rs`, `pq_groebner_f2.rs`, `pq_f4_f2.rs`, `hyperelliptic_index_calculus.rs`, `ghs_descent.rs`, `diem_descent.rs`, `joux_vitse.rs`. Reachable only through the prebuilt binaries above (no build).

## 4. msolve (`/opt/homebrew/bin/msolve` 0.9.5) — OK

Input file:
```
x,y            # line 1: variables, comma-separated, no trailing comma
65521          # line 2: characteristic (0, or prime < 2^31; 2 is allowed)
x^2+y^2-1,     # polynomials, comma-separated, no comma after the last
x*y-3
```
Calls:
- `msolve -f in.ms -o out` → over F_p a rational parametrisation `[0,[p, nvars, deg, [vars], [lin-form], [[elim poly],[denom],[[param polys]]]]]`; `[-1]` if no solution over the closure; `[1,nvars,-1,[]]` if positive-dimensional.
- `msolve -g 2 -f in.ms -o out` → reduced **grevlex Gröbner basis**, one element per line, header comments `#field characteristic / #variable order / #length of basis`; `-g 1` → leading ideal only. Inconsistent system prints `[1]:`.
- `msolve -v 2 …` (stderr+stdout) prints the **F4 step table**: columns `deg sel pairs mat density new-data zero time` — `deg` is the degree of the pairs selected in that round; the maximal `deg` across rounds is the F4 solving degree witness; `#zero reductions` per round shows degree falls. Parse lines matching `^\s+\d+\s+\d+\s+\d+\s+\d+ x \d+`.
- F_2 / Boolean: set characteristic `2` and **append field equations `v^2+v`** yourself. `-g 2` works (smoke: 3-var system → basis `[z, x+y+1, y^2+y]`). **Without `-g`** on char 2 the parametrisation step reports `Staircase is not generic` and writes an **empty** output file → always use `-g 1|2` for F_2 and read the basis.
- Characteristic ≥ 2^31 → `Warning: characteristic must be 0 or < 2^31`, exit 0, no output (checked with 2^61−1). 2^31−1 works.
- Other useful flags: `-t THR` threads, `-e ELIM` elimination order (two grevlex blocks), `-S` saturation (32-bit primes only), `-n NF` normal forms, `-m` max pairs per matrix, `-q 1` signature-based (F5-like), `-l 42/44` probabilistic linear algebra.
- Smoke timings: 2-var F_65521 0.01 s; 24-var Boolean S_4 instance 0.2 s.

## 5. PARI/gp 2.17.3 (`gp -q`) and cypari2 2.2.2 — OK

Verified one-liners (all in one `gp -q <<'EOF' … EOF` block):
```
p=1000003; E=ellinit([2,3],p); ellcard(E)                          \\ 999708
P=random(E); ellorder(E,P); ellmul(E,P,12345); elladd(E,P,Q); ellap(E); E.j
g=ffgen(Mod(1,p)*(x^2+1),'t); E2=ellinit([2*g^0,3*g^0]); ellcard(E2); ellgroup(E2)   \\ F_{p^2}: [250001978100,4]
T=ffgen(Mod(1,2)*ffinit(2,13),'T); Eb=ellinit([1,0,0,0,T^5]); ellcard(Eb)            \\ y^2+xy=x^3+T^5 over F_2^13: 8272
Pb=random(Eb); Qb=ellmul(Eb,Pb,77); elllog(Eb,Qb,Pb)                                  \\ 77
K=ffgen(Mod(1,2)*ffinit(2,131),'w); ellcard(ellinit([1,0,0,0,K^0]))                   \\ K_0 over F_2^131: 4 * 680564733841876926932320129493409985129 (ECC2K-130 order check)
polresultant(S3(x1,x2,X,a,b), S3(x3,x4,X,a,b), X)                                      \\ generic S_4 (deg 4 in each x_i); instant
polresultant(Mod(1,101)*(x^2+3*x+1), Mod(1,101)*(x^3+x+7), x)                           \\ Mod(12,101)
factormod(x^6+x+1, 2); lift(factormod(x^4+x^3+x+2, 7)); polrootsmod(f, p); polisirreducible(Mod(1,2)*f)
matrank(Mod(1,7)*[1,2,3;2,4,6;1,1,1])                                                   \\ 2 ; also works on t_FFELT matrices
ellfromj(Mod(5,101)); ellissupersingular(E); elltwist(E); ffprimroot(g); fflog
```
Syntax gotchas (observed failures): `ffgen(x^2+1, p)` is wrong → use `ffgen(Mod(1,p)*poly,'t)` or `ffgen([p,n],'t)`; `ffnbirr` does not exist in 2.17 — use `ffinit(p,n)` for an irreducible; `ellj(1/2)` is the modular function, not the j-invariant (`E.j`).
cypari2: `import cypari2; P=cypari2.Pari(); P.ellcard(P.ellinit([2,3],1000003))` OK; `P('matrank(Mod(1,7)*[1,2;2,4])')`, `P.factormod('x^6+x+1',2)` OK. `csd.genus3.pari()` gives a shared instance with a 512 MB stack. Version via `importlib.metadata.version('cypari2')` (no `__version__`).

## 6. Quick recipes for falsifier agents

1. **Prime-field Weil descent degree label** (open lever 1/9 analogue over F_{p^n}): `fd.GF(p,n)`, `fd.Curve`, `descended_system(E, xR, "E")`, `system_labels(..., dmax)` — exact `d_ff / D_GB / D_sat`; cross-check with msolve `-v 2 -g 2` on the same polys (write dict polys to msolve syntax; p < 2^31, n ≤ 3 for e-system).
2. **Binary subspace S_3 last fall degree** at n up to ~45: prebuilt `hky_last_fall_degree cell --n N --seed S --draws K --cmax C`; first fall via `ffd_semaev_core.run_cell`.
3. **Binary S_4 instances → any Boolean solver**: `ic corpus --degree n --dimension l --dir $TMPDIR/… ` then `.anf`→msolve (converter above) or `.dimacs`→`cryptominisat5`/`kissat`.
4. **Char-0 symbolic S_m structure** (replacement for the Sage `thm_bkkmv*` scripts): `csd.semaev.semaev(m)` / `symmetric_semaev`, or gp `polresultant` chains; sympy for factoring.
5. **Point counts / orders / logs on toys**: gp `ellinit` over `ffgen` fields (`ellcard`, `ellorder`, `elllog`, `ellgroup`), including F_{2^131} K_0.
6. **GHS/magic-number, genus-3 Smith isogeny censuses**: `csd.binary_ghs.audit_order / binary_screen / planted_b`, `csd.genus3.analyse_curve`.
