# Brief: native instrument for the m = 8 sparse PKM relation-finder panel (panel A)

Owner authorized the spend on 2026-11-14 ("yes to 1"). Protocol being frozen in this directory
(`PROTOCOL.md`, by the main session) from `research/prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md`
section A and `research/prime_norm_rank_20261112/cost-floor-report/NEXT_TEST.md`. Read both first.
Everything here is `[pcc:unverified]`; this is instrument construction and local development checks only.
Repository rule: research tooling is native Rust (AGENTS.md "Implementation language: no Python");
Python/bash may only tabulate recorded values. Do not execute the inherited Python generator
(`research/prime_leads_chase_20261113/L1-pkm/scripts/gen.py`); read it as a specification.

## Deliverable
`examples/pkm_m8_panel.rs` (one new example, may use `#[path]` submodules under `examples/pkm_m8_panel/`;
do NOT edit existing examples or library files; reuse by `use crypto_lib::...` and, where helpful, copy
small private helpers with a comment naming the source line). Subcommands:

1. `cell --t T --ratio R --seed S --out DIR` : deterministic cell derivation.
   - p = least prime >= 2^(T*R) with 2^T | p-1. (Exception: cell t=3 ratio=8 MUST reuse the L1 decisive
     cell: p=16801793 and the same a, b, g, zeta as `research/prime_leads_chase_20261113/L1-pkm/`; find
     them in that directory's committed files (results.csv/analysis.json/decisive/*.jsonl/PROTOCOL/AMENDMENTS);
     if they are not recoverable, say so in the record and fall back to the deterministic rule.)
   - curve y^2 = x^3 + a x + b over F_p with PRIME group order #E = r: deterministic seeded search
     (StdRng from S and the cell slug), a,b uniform; order by native point counting (Legendre-symbol sum
     is fine up to p ~ 2^27; use/port the fastest existing native counter in the repo if there is one,
     e.g. in examples/prime_tower_curves.rs or pkm_tower_curve_seeds.rs); verify r prime and r*P = O for a
     random point.
   - zeta = a primitive 2^T-th root of unity in F_p; Kummer base V = g * mu_{2^T}: choose g as the least
     integer >= 2 that is not a 2^T-th power... (state the rule you implement; it must be deterministic and
     recorded) ; record the REAL base: the x in V with x^3+ax+b a square (points in E(F_p)), their count
     B_real and the 2*B_real signed points.
   - Write `cell.json` (schema `pkm.m8-panel.cell/v1`): p, a, b, r, T, R, g, zeta, V, V_real, B=2^T,
     B_real, generator P (a fixed point), seed, derivation rule text, sha256 of this file's canonical form.
2. `targets --cell DIR --n-sat 4 --n-unsat 4 --planted 1 --seed S`: natural targets with hidden logs.
   - Hidden instance: Q = k*P with k from a seeded RNG (HIDDEN; written only to `hidden.json`, which the
     solver pipeline never reads; the protocol seals it by sha256). Targets R_j = a_j P + b_j Q with
     a_j,b_j seeded. Classify each R_j by EXHAUSTIVE enumeration of all signed multisets of size 8 of the
     real signed base (native; enumerate once per cell into a hash set of x-coordinates with their
     multiset; B_real<=16 so this is cheap) -> `real_decompositions(R_j)` (count and list). Also count
     algebraic solutions over F_{p^2} the way gen.py's `decomposable` does (all x in V lifted to E(F_{p^2}),
     signs, multisets) -> `algebraic_count`; the solver sees these too, only the real ones are useful.
   - Draw targets in order until 4 real-SAT and 4 real-UNSAT are collected; record every draw index and
     the empirical natural-SAT rate. Plus 1 planted target (sum of 8 random real signed base points):
     interface control only, flagged `planted: true`.
   - Write `targets.json` (public: x_R values, labels sat/unsat/planted, counts, draw indices) and
     `hidden.json` (k, a_j, b_j, the decompositions). The public file must not leak the answer beyond the
     SAT/UNSAT label (the label is needed for the panel's stratification; say so in the schema).
3. `export --cell DIR --targets DIR --out DIR`: for each target, the raw-presentation system EXACTLY in
   the format of `L1-pkm/scripts/toms.py` (variables `y{i}_{j}`, `u{j}`; S3 chain with m-2 aux unknowns
   u_k; tower equations `y_{i,j+1} - y_{i,j}^2`, `1 - y_{i,t-1}^2` written as toms.py writes them, with
   x_i = g*y_{i,0}; coefficients reduced mod p, same term order as toms.py: sorted keys descending).
   Reuse `examples/pkm_oracle_program/algebra.rs` (its `chain` branch and `s3`) if it is general in m,
   else write it. Write `<target>.ms` plus `<target>.system.json` (polys as sparse terms) and sha256s in
   `export.manifest.json`. Also `--format native` dumps the same system for the native solver.
   CROSS-CHECK (required, in `cargo test` of the example): for m=3,t=2 and m=4,t=2 at a small p, the
   polynomial sets must equal the systems `examples/pkm_tower_pilot.rs` builds for its `tower` control
   (raw presentation) up to variable naming and the tower-reduction convention; document how you matched
   them. Also test that a planted decomposition zeroes every exported polynomial, and that S3 vanishes on
   three collinear-sum abscissae for random points.
4. `verify --cell DIR --targets DIR --msolve-out FILE --target NAME --out FILE`: parse msolve's output
   (the main session's pod worker is checking `msolve -P 1` over F_p; coordinate through the file
   `research/pkm_m8_sparse_panel_20261114/pod/MSOLVE_FORMAT.md` which they write; if it is not there yet,
   run msolve yourself: it is installed at /usr/bin/msolve 0.6.5 on this machine; try
   `msolve -v 2 -g 2 -P 1 -f in.ms -o out` on a tiny m=3 instance and record what comes out), extract
   candidate solutions: roots of the eliminating polynomial over F_p natively
   (`crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until` or an equivalent counted
   root finder), reconstruct y_{i,0}, x_i = g*y_{i,0}, lift to points, keep only REAL decompositions
   (all x_i on E(F_p)), check the signed sum equals the target (both y signs tried; count work), and compute
   `useful_relations` = number of distinct real decompositions and `rank_increment` = rank over Z/r of
   the relation rows (unknowns: logs of the B_real base points plus log Q; the row of decomposition
   R_j = a_j P + b_j Q = sum eps_i P_i is [eps_i..., -b_j | a_j]) — rank is computed WITHOUT the hidden
   logs. Charge and record: root-finding field multiplications, lifts, point additions, elimination ops.
   Output `verify.json` with all counts and a `consistent_with_classifier` flag (the real decompositions
   found must be a subset of the classifier's list; a superset is a bug; report both).
5. `native-solve --system FILE --max-degree D --deadline SECS --out FILE`: the independent native arm
   for the small cells: run `crypto_lib::cryptanalysis::f4_fp` (grevlex) on the same polynomial set,
   capture `F4Report` (solving_degree, degree_reached, inconsistent, field ops via the counters
   `field_ops_total`/`native_ops_total`, run log steps rows/cols/ops), then extract solutions if the
   basis is zero-dimensional and small (univariate elimination via the existing machinery if any;
   otherwise record `basis_only: true` and leave extraction to msolve's arm). Record wall/CPU time and
   peak RSS (read /proc/self/status VmHWM).
6. `rho --cell DIR --targets DIR --seed S --out FILE`: protocol-matched rho reference in the same unit:
   solve the SAME hidden instance's DLP (log of Q to base P) with the repository's native negation-map
   rho (`prime_fast` has a rho; reuse ecbench's rho method if simpler), 3 seeds, record group additions,
   CPU seconds, wall. This is only a reference row; it decides nothing.

All outputs JSON with sha256 and the example's own git commit (`git rev-parse HEAD` at build, embedded
via env!("...") or read at runtime and recorded). Deterministic given seeds. `cargo build --release
--example pkm_m8_panel` must succeed with the workspace's pinned toolchain; `cargo test --release
--example pkm_m8_panel` must pass; `cargo clippy --release --example pkm_m8_panel -- -D warnings` clean.
Keep runtime of tests under 2 minutes. Local smoke: run `cell`/`targets`/`export` for t=2 ratio=7
(p ~ 2^14) and msolve locally (`msolve -t 2 -v 2 -g 2 -P 1`) on one SAT target with a 10-minute
timeout and 3 GB `prlimit --as`, then `verify`; report what happened (this is a development check,
not a measurement). Put smoke outputs under research/pkm_m8_sparse_panel_20261114/smoke/ (small files
only; no files > 1 MB; delete big outputs).

Report back: what was built, test results, the smoke outcome, exact CLI usage lines, any deviation from
this brief and why, and open questions for the protocol (e.g. if msolve -P over F_p is not available in
0.6.5, say so and propose the extraction route: e.g. msolve -g 2 reduced GB in grevlex + native FGLM is
NOT available; alternative: append the field equations y^p - y? no: propose and I decide).
