# Design: making the `m = 4` solving-degree cells runnable

Card [`XP-20261018-m4-solving-degree-equivariant`](README.md). This is the
engineering plan: what exists, what to build, in what order, and the gates between
the steps. Labels as in the README: *measured* (a committed row, cited), *derived*
(arithmetic shown) and *predicted* (an estimate made before any run).

## 0. What exists today, and what is missing

| piece | exists? | where (file:line) | notes |
|---|---|---|---|
| chained `m`-summand Weil-descent system (arm A) | yes | `src/cryptanalysis/koblitz_groebner.rs:911` `build_decomposition_system` | `N = (m−2)n + mℓ`, cubic |
| draw sampler and exact solution count | yes | `src/cryptanalysis/koblitz_bench.rs:1338` `ladder_sample`, `:1200` `chained_solution_count` | `x_R` is a uniform field element; `V` is any random subspace (not forced to contain 1) |
| Macaulay ("matrix-F4") step at a fixed degree | yes | `koblitz_groebner.rs:1374` `matrix_f4_f2` | a fixed-degree XL step, not a pair-selection F4 |
| MutantXL fall closure, dense | yes | `koblitz_bench.rs:1493` `closure_refutation_scan` → `src/cryptanalysis/inherited_f4.rs:1966` `ReducedBasis::from_system_closed_full` | single-threaded; peak `≈ 0.222·cols²` bytes (*measured*, 25.8 GB at 341,056 columns) |
| MutantXL fall closure, sparse | yes, too slow | `koblitz_bench.rs:1583` `sparse_closure_refutes` | `research/ic_leads_review_20261018/README.md` §2.1 item 3 |
| scan drivers | yes | `examples/dreg_ladder.rs` (draws, XL), `examples/dreg_closure.rs` (closure) | |
| msolve export of a chained draw | yes | `examples/ic_next10_06_chain_export.rs` | appends `x_i² + x_i` |
| msolve run and degree parse | Python only | `research/ic_next10_20261014/06-equivariant-f4-m4/run_f4.py` | AGENTS.md "no Python": needs a native reader |
| msolve binary on this host | no | build v0.6.5 as `research/dreg_closure_m4l6_20261018/pod/boot.sh:85–89` | |
| solver engine enum and node reducer | yes | `koblitz_groebner.rs:4756` `SolverEngine`, `:5187` `reduce_system`, `:5243` `reduce_system_uncached` | no equivariant variant, by decision (below) |
| monomials | `u64` masks | `src/cryptanalysis/pq_groebner_f2.rs:77–79` `F2BoolMono { mask: u64 }` | at most 64 unknowns in every native engine |
| G2 Frobenius-equivariant pass: orbits, blocks over `F_{2^d}`, block RREF, inverse transform, agreement with full `F₂` RREF | yes, as an example on the **barrel** `m = 2` system, 192-bit masks | `examples/ic_next10_06_g2_full_pass.rs:1–23` (header), `:623` `character_orbits`, `:645` `rref_gf`, `:680` `rref_bits`, `:717–721` trace | bit-identical RREF at `n = 7, 11, 13, 17` (*measured*) |
| G2 pass as an oracle | yes, example only | `examples/ic_improve_02_equivariant_oracle.rs` | TRADES; step gate fails (one linear consequence fewer); hook would be dead code (02 README:30, :106) |
| binary FGHR (2-torsion) symmetrised system | yes for `m ≤ 3` | `src/cryptanalysis/koblitz_symmetrised.rs:414` `symmetrised_terms` (`None` at `:443` for `m ≥ 4`), `:512` `sym_polynomial`, `:578` `build_symmetrised_system`, `:2311` `random_subspace_containing_one` | `build_symmetrised_system` takes a `KoblitzCurve` and a `BinaryPoint` target |
| `S₅` in `x`, symbolic in `a₆` | yes | `src/cryptanalysis/semaev_leading_form.rs:584` `semaev(4)` | the input to M1 |
| odd-characteristic FGHR | yes, not relevant here | `src/cryptanalysis/fghr_full.rs`, `fghr_line.rs` | `F_{p³}` |
| Faugère–Svartz for any group other than the Frobenius shift | **no** | — | `research/ic_leads_review_20261018/README.md` §2.1: "no Faugère–Svartz block run exists anywhere" |
| a native pair-selection F4 for Boolean systems | **no** | `src/cryptanalysis/groebner_f4.rs` is a matrix-F4 over `F_p` with no symbolic preprocessing | true F4 here means msolve |

## 1. The arms, and why the equivariant arm is not the G2 one

- **Arm A, chained `S₃`.** The system every committed `m = 4` degree was measured on.
  Native closure (primary), msolve F4 (external cross-check, AGENTS.md §15), XL
  (secondary, only where reachable).
- **Arm B, FGHR.** Factor base `F_u = {P : u(P) ∈ V}`, `u = 1/(x + 1)`, `1 ∈ V`;
  unknowns `c_{i,t}` (`ℓ − 1` a summand) and one parity bit `ε`; equations: the `n`
  coordinates of `S₅` rewritten as `A(w₁..w₄, w_R) + s·B(w₁..w₄, w_R)`. `N_B = 4ℓ − 3`
  whatever `n` is. This is the formulation the anchor note asked for ("with FGHR
  symmetrisation applied", §5).
- **Arm B0, `S₅` in `x`.** `N = 4ℓ`, Boolean degree 12. A control at P1 only, to
  separate "no chain" from "symmetry", as `koblitz_symmetrised` did at `m = 3`.
- **Arm E, `C₃` Faugère–Svartz blocks on arms B and B0.** An instrument, not a
  formulation: it must reproduce the plain closure where both fit, and is used alone
  only where the plain closure does not fit.

Why not the G2 blocks. The G2 symmetry is the Frobenius shift `σ`, which acts on a
system only when the factor base is `σ`-closed; random subspaces are not, and at
`n = 131` no nontrivial `σ`-stable `V` exists (`koblitz_symmetrised.rs:2302–2304`).
A `σ`-closed base exists in the barrel formulation (`τ^k(W)` with selectors), but that
is a different factor base, built only for `m = 2`, whose gauge-fixed step already
carries one linear consequence more than the equivariant one (02 README §4). Making a
barrel `m = 4` system and asking whether its equivariant degree beats arm A's is a
separate question for a separate card; nothing here depends on it.

Why `C₃`. Arms B and B0 are symmetric under `S₄` permuting the summand blocks
(`S₅` is symmetric in its first four arguments, `semaev_leading_form.rs` test
`s5_in_last_is_symmetric_in_first_four_args`, `semaev_higher.rs:857`; `s` and the
`w_i` share one basis of `V`). Faugère–Svartz's decomposition needs an abelian group
of order prime to the characteristic. `S₄`'s 2-parts are modular in characteristic 2
(and the FGHR invariants have already used the 2-torsion), so the usable piece is
the Sylow-3 subgroup `C₃ = ⟨(123)⟩`, acting by `c_{1,t} → c_{2,t} → c_{3,t} → c_{1,t}`
with block 4 and `ε` fixed. Every equation is fixed by `σ = (123)`, so Macaulay rows
`t·f` map to rows `σ(t)·f`, and monomials to monomials: the G2 setting with
`r = 3`, `d = ord_3 2 = 2`, `ζ ∈ F₄` a primitive cube root of unity, one trivial block
over `F₂` and one block for the conjugate pair `{χ₁, χ₂}` over `F₄`.

The chained system (arm A) has no odd-order symmetry: the chain `P₁ + P₂ = Q₁`,
`Q₁ + P₃ = Q₂`, `Q₂ + P₄ = R` is symmetric only under `P₁ ↔ P₂`. Arm A therefore gets
no equivariant arm. (A half-chained presentation `S₄(P₁, P₂, P₃, Q) ∧ S₃(Q, P₄, R)`
would carry `S₃` on three blocks and hence `C₃`; it is listed in §7, not scheduled.)

## 2. Stages and gates

```
M0 (≤ ½ day) ── G0 reproduce ──► S0 runs (arm A closure at P1, P2; msolve on committed (13,4), (13,5); msolve sizing probe at P1)
M1 (1–2 days) ── G1 gate tests + d_sys ──► S1 runs (arm B at P1–P3 plain where it fits; B0 at P1)
M2 (2–3 days) ── G2 identity + ratio ──► S2 runs (arm B where plain does not fit, e.g. P3 at D ≥ 9)
M3 (3–5 days, optional) + host (b) ── G3 owner + readings ──► S3 runs (arm A at P3; arm B at E1, E2; msolve where the probe says it fits)
```

S0 does not wait for M1 or M2. M1 is the smallest milestone that turns "nothing to
run" into a new measurement, and is specified in full in §3.

### M0: msolve and a native log reader (half a day)

- Build msolve v0.6.5 from source into `~/opt/msolve` (the `boot.sh:85–89` recipe;
  record the sha256).
- `examples/msolve_degree_log.rs`: reads msolve's `-v 2` trace and the answer file,
  emits one JSON line: per F4 round `{degree, rows, cols, new_polys}`, `max_degree`,
  `gb_is_one`, `gb_len`, `exit`. The degree convention is the 06 study's
  ("`D_F4` is the maximum `deg` over all F4 rounds"). No Python.
- Thin shell runner modelled on `research/pkm_m3_n21_msolve_20261018/run.sh`:
  `nice`, `prlimit --as`, `timeout`, `VmHWM` sampling, `-t 1 -v 2 -g 2`, field
  equations appended by the exporter.

**Gate G0 (reproduction).** On the committed 06 draws: `(11, 4)` u0–u3 read
`D_F4 = 4` with `GB = [1]`; the satisfiable `(11, 4)` draw 0 reads degree 5, `GB`
length 594 (*measured* there, `06-equivariant-f4-m4/README.md` §1 E). One committed
`m = 4`, `(12, 5)` draw replayed by `dreg_closure` refutes at 4 (Result 13). Any
mismatch stops the card until explained.

### S0 runs (host (a), no further code)

1. msolve on the committed 06 exports at `(13, 4)` and `(13, 5)` (seed 20261014,
   unsatisfiable indices 0–3), cap 30 GB, wall 72 h, `-t 1`. Projected 2.8–12.6 GB
   (*derived* band, README §5.2).
2. Arm A closure at P1 `(16, 4)` and P2 `(20, 5)`, degree 4, `--max-cols 600000`.
   Draws from `dreg_ladder` need `1 ∈ V` to match arm B; until M1's harness exists,
   S0 uses `dreg_ladder --m 4` draws as they are (any `V`), and S1 reruns P1/P2 on
   the shared `V ∋ 1` draws. Projected 10 and 61 GB, 0.2 and 1.1 h a draw.
3. msolve **sizing probe** on P1 draw 0: `-t 1`, wall 6 h, cap 30 GB; record each
   round's degree, matrix size and RSS. Extrapolate to (15, 6), P2, P3 and decide
   which msolve cells run on host (a) (projected under 72 h and 100 GB) and which go
   to S3. The probe is disclosed in the result, not counted as a reading.

### M1: FGHR `m = 4` (one to two days; the first milestone, §3)

**Gate G1.** All M1 tests pass with zero mismatches, and `d_sys` (the Boolean degree
of arm B's equations) is printed. Then, from README §5.4 (*derived*):
- `d_sys ≤ 7`: arm B can move an ECC2K-130 row; E1, E2 are eligible for S3.
- `d_sys ≥ 8`: arm B cannot beat the degree-5 row at `ℓ = 33` whatever it measures;
  it runs on host (a) at P1–P3 as a mechanism study and never on host (b).

### M2: `C₃` blocks (two to three days, §4)

**Gate G2.** Identity: checks E1–E5 (§4.4) with zero mismatches on every arm-B draw
at P1 and P2 and on B0 at P1, every degree built. Ratio: at the largest arm-B matrix
both engines build on host (a), equivariant CPU ≤ 1/1.5 of plain and peak RSS ≤ 0.5×
plain. Identity failure: blocks are not used anywhere. Ratio failure only: blocks are
used only where the plain closure cannot be built.

### M3 and S3 (optional; host (b))

- M3: generic monomial masks (`u128` or `[u64; 2]`) through `pq_groebner_f2`,
  `inherited_f4` and the closure path, so arm A at P3 (`N = 72`) can run natively.
  `examples/g2_wide_probe.rs` (192-bit masks) is a template. Three to five days
  (*predicted*). If not built, arm A at P3 runs on msolve only.
- **Gate G3.** The owner approves the host, and S0–S2 leave something for S3 to
  decide: arm A at P1 and P2 both refute at 4 (so P3 asks whether `S = 0` changes the
  degree at `ℓ = 6`), or G1 passed and arm B is flat over P1–P3 (so E1, E2 ask whether
  it stays flat). If arm A already rises at P1 or P2, P3 is moot and is not run.

## 3. The first milestone (M1), in full

Goal: arm B exists, is proved equal to the decompositions it encodes, and has been run
end to end at P1. One engineer, one to two days.

### 3.1 `S₅` in FGHR invariants, exactly (no interpolation)

1. Take `S₅ = semaev(4)` (`semaev_leading_form.rs:584`) with `a₆ = 1` (Koblitz,
   `b = 1`; binary Semaev polynomials involve only `a₆`).
2. Substitute `x_i = (1 + u_i)/u_i` (`u = 1/(x + 1)`) for all five arguments and clear
   denominators: `P(u₁..u₅) = Π u_i^8 · S₅(…)`, degree ≤ 8 in each `u_i`, at most `9⁵`
   terms.
3. Reduce `P` modulo `u_i² + u_i + w_i` for each `i` (so `w_i = u_i² + u_i`). The
   normal form is multilinear in the `u_i` with coefficients in `F₂[w]`.
4. *Derived:* the invariants of the even-translation group are `F₂[w] ⊕ s·F₂[w]` with
   `s = Σ u_i` (`s² + s = Σ w_i`). So the normal form must have no `u`-monomial of
   degree ≥ 2 and equal coefficients `B(w)` on every `u_i`; then `P = A(w) + s·B(w)`.
   Assert both; a failure is a bug, not a result.
5. Emit exponent vectors over `(w₁, w₂, w₃, w₄, w_R, s)` in the format of
   `symmetrised_terms` (`koblitz_symmetrised.rs:414`), and add the `4 =>` arm (or a
   cached computed table) so `sym_polynomial` (`:512`) Weil-restricts it unchanged.
6. Report `d_sys = max over terms of Σ_{i≤4} wt(e_i) + e_s` (the target's `w_R` is
   known and does not count, `koblitz_symmetrised.rs` module header).

Tests: (a) the same pipeline at `m = 3` reproduces the pinned 18-term table exactly;
(b) substituting `w_i = u_i² + u_i`, `s = Σu_i` back into `A + sB` gives `P` as
polynomials; (c) at 10⁴ random points of `F_{2^{13}}`, `S₅(x) = 0` iff `A + sB = 0`.

### 3.2 The system on a field draw

- `build_symmetrised_system_on(basis: &[F2mElement] /* basis[0] = 1 */, x_r, m, st)
  -> Option<(Vec<F2BoolPoly>, usize)>`: the body of `build_symmetrised_system`
  (`:578`) without the `KoblitzCurve`/`BinaryPoint`, so degree draws at `n` up to 63
  need no curve. Refuse `x_R ∈ {0, 1}`.
- `random_subspace_with_one(n, irr, ℓ, rng)`: as `random_subspace_containing_one`
  (`:2311`) on `(n, irr)`, rejecting `σ`-stable subspaces.
- Exact count: Gray-code enumeration of the `2^{N_B}` assignments with bitsliced
  evaluation of the `n` equations (`2^21` at P3, `2^29` at E2). It shares no code
  with the Macaulay path.

### 3.3 The harness `examples/m4_formulations.rs`

- `--cells n:ℓ,…  --arms A,B,B0  --seed S  --unsat K  --sat-controls C  --max-draws M
  --engine closure|xl|closure-c3|msolve  --from D --to D  --max-cols X
  --msolve-bin PATH --export-dir DIR  --out FILE  --resume  --verify FILE`.
- Cell seed `S ^ (n << 40) ^ (ℓ << 32) ^ (4 << 48) ^ (0xB << 56)` (distinct from
  `dreg_ladder`'s); draw `k` is one `(V ∋ 1, x_R)` shared by every arm; each arm
  selects its own first `K` unsatisfiable and first `C` satisfiable draws by exact
  count and records their indices. Arm A's count is `chained_solution_count`; arm
  B's is §3.2's enumeration; arm B0's is enumeration over `2^{4ℓ}`.
- One JSON line per (arm, draw) with the fields `dreg_closure` writes
  (`degree, xl_refuted, closure_refuted, saturated, xl_rank, closure_rank, columns,
  closure_products, closure_word_ops, secs` per degree) plus `arm, n_vars, d_sys,
  solutions, draw, basis, x_r, peak_rss_kb`.
- `--resume` skips (arm, draw) pairs already in `--out` (the card's checkpoint).
- `--engine msolve` exports the draw, runs `--msolve-bin` under the caps and parses
  its log with M0's reader, in-process (no shell).
- `--verify FILE` is the independent check named by the card: for every line it
  recomputes the exact count by enumeration (or `chained_solution_count`), and replays
  every claimed refutation at its degree with the plain closure (for an equivariant
  line: with the plain closure where it fits, else with an independently seeded
  repeat of the block engine). Exit 1 on any disagreement.

### 3.4 Gate tests (`cargo test --release`)

- On 50 draws at `n ∈ {9, 11, 13}`, `ℓ ∈ {3, 4}` on a Koblitz curve
  (`KoblitzCurve`, `n ≤ 24`), the roots of arm B lift to decompositions over `F_u`
  that re-sum to `R` or `R + T`, and enumeration over `F_u` finds no decomposition the
  system misses. Zero mismatches. This mirrors the existing `m = 3` gate.
- The plain closure never refutes a satisfiable arm-B draw (200 draws at `N_B ≤ 13`).

### 3.5 Acceptance

`m4_formulations --cells 16:4 --arms A,B,B0 --unsat 4 --sat-controls 1` completes
on host (a) and prints `d_sys` and each arm's degree scan. Expected minutes
(*predicted*): P1's arm B has at most 8,192 monomials (README §5.3).

## 4. The second milestone (M2): `C₃` blocks in the Macaulay step and the closure

### 4.1 Module `src/cryptanalysis/equivariant_macaulay.rs` (new)

| item | does | from |
|---|---|---|
| `struct VarPerm { image: Vec<u8>, order: u32 }`, `fn apply(&self, mask: u64) -> u64` | the variable permutation `σ` (here the 3-cycle on summand blocks) | new |
| `fn fixes_system(system, n_vars, &VarPerm) -> bool` | every `σ(f)` is in the system (here every `f` is fixed) | new |
| `struct Gf4Row { lo: Vec<u64>, hi: Vec<u64> }`, `fn rref_gf4(...)` | bitsliced `F₄` rows (`a + bζ`, `ζ² = ζ + 1`): add = 2 XOR, multiply by `ζ` = 1 XOR | replaces the log tables of `ic_next10_06_g2_full_pass.rs:472–645` for `d = 2` |
| `fn project(row, &VarPerm) -> (F2 vector, Gf4 vector)` | isotypic projections of one `F₂` row: trivial `Σ_t σ^t v` (|G| = 3 is odd, so `1/3 = 1`), character `Σ_t ζ^{−t} σ^t v`, each restricted to the orbit representatives (fixed columns kept in the trivial block, dropped from the character block) | generalises the G2 block formula `N_j[r̄, c̄] = Σ_t P[r̄, σ^t c̄] ζ^{−jt}` (scope note §3.1) to any row, so the row set need not be `σ`-closed |
| `fn inverse(block_rows) -> Vec<F2BoolPoly>` | trivial: `u[σ^t c] = w[c]`; character: for `λ ∈ {1, ζ}`, `u_λ[σ^t c] = Tr_{F₄/F₂}(λ·w[c]·ζ^{t})` | `ic_next10_06_g2_full_pass.rs:717–721` and its README §3 |
| `pub fn matrix_f4_f2_equivariant(polys, n_vars, degree, &VarPerm) -> Option<(Vec<F2BoolPoly>, Cost)>` | same contract as `matrix_f4_f2` (`koblitz_groebner.rs:1374`): rows spanning the degree-`degree` Macaulay row space, falls first | new |
| `pub fn closure_refutation_scan_equivariant(system, n_vars, d_min, d_max, &VarPerm) -> Vec<ClosureDegree>` | same output type as `closure_refutation_scan` (`koblitz_bench.rs:1493`) | new |

### 4.2 Data flow (one degree `D` of the closure)

```
system (F2BoolPoly, σ-fixed) ──► Macaulay rows t·f, deg ≤ D (never materialised as one matrix)
      │ project each row
      ▼
trivial block over F2 (≈ cols/3 columns)      character block over F4 (≈ cols/3 columns)
      │ RREF (gf2_elim)                             │ RREF (bitsliced F4)
      ▼                                             ▼
   pivots, rank r0                              pivots, rank r1      ──► refuted iff the trivial block holds the constant 1
      └──────────────── falls (leading monomial degree < D) ─────────┘
                                │ inverse transform to F2 rows
                                ▼
                   multiply each fall by monomials up to degree D − deg
                                │ project the products, reduce into both blocks
                                └──► repeat until no new fall or 1 appears (saturated)
```

The constant `1` is `σ`-fixed, so it lies in the trivial isotypic component: the
refutation bit is read from the trivial block alone. The span of the falls is
`σ`-stable and so is the set of their monomial multiples, which is why projecting the
products is enough and no orbit closure of rows is needed.

### 4.3 Where it plugs in, and where it deliberately does not

- `m4_formulations --engine closure-c3` calls `closure_refutation_scan_equivariant`
  for arms B and B0; `--engine xl-c3` calls `matrix_f4_f2_equivariant` per degree.
- **Not** into `reduce_system` (`koblitz_groebner.rs:5187`) in this card. A splitting
  solver assigns one variable at a time, which breaks the `C₃` symmetry below the root,
  so the engine could serve only the root node; the oracle study's arm C is the
  template for that (root pass, reduced rows appended to the generators, the existing
  `InheritedF4` below), and its result (TRADES, mostly from fall rows, not
  equivariance) is the reason to measure the oracle separately before wiring
  anything. A later card can add `SolverEngine::EquivariantRoot { max_degree }` behind
  `fixes_system`, with the existing engine as the control.
- No native pair-selection F4 is built. The degree of an iterated F4 is read from
  msolve; the closure is the native fixed-degree saturation and is what the cost rows
  price.

### 4.4 Agreement checks that must pass before any measured use

| id | check | where |
|---|---|---|
| E1 | `rank(M) = rank(M₀) + 2·rank_{F₄}(M₁)` at every Macaulay degree built | arms B (P1, P2) and B0 (P1), every draw, all degrees |
| E2 | RREF of the inverse-transformed rows is bit-identical to `matrix_f4_f2_counted_with` full read-back (`KIC_F4_SOLVER_FULL_READBACK=1`) under the same degree-descending column order | same |
| E3 | per degree: refutation bit, fall-space dimension and saturated closure rank equal the plain closure's | same, every draw including the satisfiable controls |
| E4 | per draw: closure degree identical | every draw where both fit (P1, P2, and P3 up to `D = 8`) |
| E5 | no satisfiable draw refuted by the block engine | all satisfiable controls |

### 4.5 Expected gain, and its source

- *Derived (dense model).* Elimination cost `∝ cols^ω`. Blocks: `(cols/3)^ω` over
  `F₂` plus `(cols/3)^ω` over `F₄` at about 3 word operations an entry against 1, so
  the time gain is `3^ω / 4`: 6.8× at `ω = 3`, 2.9× at the closure's measured
  work exponent 2.25 (scale study Q6). Memory: the larger block holds 2 bits an entry
  over a third of the columns, `2/9` of the plain matrix, so 4.5× less peak, which
  buys `√4.5 ≈ 2.1×` more columns in the same RAM.
- *Measured analogue.* The G2 pass (Frobenius, `d = 10–12`, table arithmetic) was 2–3×
  faster than the crate's `gf2_elim` on the same symmetric matrices at `n = 11–13`
  (02 README:94), and 25–45× against a naive kernel once assembly was charged
  (06 README §3). `F₄` arithmetic is cheaper than `F_{2^{10}}` but the group is smaller
  (3 against 10–12).
- *Predicted.* 2–4× CPU and 0.2–0.3× peak RSS against the plain dense closure on arm B
  at P3. The gate (G2) asks for ≥ 1.5× and ≤ 0.5×.
- *What FGHR itself buys (measured analogue, not this card's number).* At `m = 3`,
  `icv1-f2m15-tm275-2d22ff5d`, `ℓ = 5`, the FGHR system (13 unknowns, degree 4) refuted a target in
  29 ms against 10,291 ms for the chained system (30 unknowns, degree 3)
  (`research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md:509–510`). That is a
  solve time on a splitting solver, not a degree; at `ℓ = 33` the unknown counts are
  129 against 394 but the degrees differ, which is README §5.4.

## 5. Measured versus predicted, in one place

| statement | label | source |
|---|---|---|
| closure 4 at `m = 4`, `ℓ = 4, 5, 6` at `S ≤ −3` | measured | DREG Results 12–13; m4l6 RESULTS.md:9 |
| msolve F4 4 at `(11, 4)`; caps at `(13, 4)`, `(13, 5)` | measured | 06 README:76–78 |
| no `m = 4`, `ℓ ≥ 4` cell measured at `S = 0` | measured (absence) | same files |
| dense closure `≈ 0.222·cols²` bytes | measured at one point (degree 4) | m4l6 A1 log |
| `d_sys ≤ 9`, `= 5` without `w³` terms | derived | §3.1 step 4, README §5.1 |
| `C₃` blocks 2–4× time, 0.2–0.3× memory | predicted | §4.5 |
| msolve memory and time at P1–P3 | predicted (band) | README §5.1; replaced by the S0 probe |
| every `n = 131` figure | derived extrapolation | README §2, §5.4 |

## 6. Risks and how the plan handles them

- **Unsatisfiable draws scarce.** At `S = −9` only 2 of 251 draws were unsatisfiable
  (m4l6 §10). At `S = 0` the expected number of solutions a draw is about 1 for
  arm A (`2^{N − 3n}` with `N = 2n + 4ℓ`) and `2^{−3}` for arm B (`2^{N_B − n}`), and
  solutions come in permutation orbits, so most draws should be unsatisfiable
  (*derived* expectation; the rate is recorded, and `--max-draws 512` is a guess).
- **The `S₅` rewrite fails step 4.** Then the invariant description used by the `m = 3`
  module does not extend as stated; arm B stops at M1 and the card records why.
- **`d_sys = 9`.** Arm B is then a mechanism study (G1) and the `C₃` blocks are needed
  for P3 at `D ≥ 9`.
- **msolve's memory runs away** (it did at 3 GB on `(13, ·)`). The probe sizes it; every
  msolve run has an address-space cap, and a cap is a censored reading.
- **The host is shared.** At most four concurrent single-threaded jobs and 100 GB
  total on host (a); the other workload has priority. Counted fields, not wall time,
  are the measurement.

## 7. Not in this card

- A barrel (`σ`-closed) `m = 4` formulation with the G2 Frobenius blocks.
- A half-chained `S₄ ∧ S₃` presentation with `C₃` on three blocks.
- XL at degree 7 on `m = 3`, `(15, 5)` (110–120 GB, `RESEARCH_DREG_MEASUREMENT.md:958–961`):
  superseded as a deciding read by the F4 and closure readings there, and it would fill
  host (a).
- Measuring `K` (column touches per group operation); every price row stays in
  unconverted units until it is measured.
