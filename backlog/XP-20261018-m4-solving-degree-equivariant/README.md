# XP-20261018-m4-solving-degree-equivariant

**Status: blocked** (engine work in [`DESIGN.md`](DESIGN.md) milestones M1 and M2 is not built;
the protocol is a draft). Every number carries one label: *measured* (a committed
row, cited), *derived* (arithmetic shown here) or *predicted* (an estimate, stated
before any run). Nothing in this card is a measurement made by this card.

Files: `card.json` (the machine card), this README (question, why, prerequisites,
cost), [`DESIGN.md`](DESIGN.md) (the engineering plan, stages, gates, first
milestone), [`PROTOCOL.md`](PROTOCOL.md) (**DRAFT**, not frozen).

## 1. Question

At decomposition size `m = 4` and factor-base dimension `ℓ = 4, 5, 6`, in the
index-calculus regime (surplus `S = n − mℓ = 0`), what is the refutation degree `D`
of the Weil-descended summation system, under (A) the chained `S₃` presentation the
repository has always measured and (B) the FGHR 2-torsion-symmetrised `S₅`
presentation, and does `D` stay flat in `ℓ` or rise by about one per step of `ℓ`?

## 2. Why it matters for ECC2K-130

The 2026-10-10 anchor note calls this "the only experiment whose outcome bears on
ECC2K-130 security rather than on accounting"
(`RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` §5, §9 item 3). The pricing is in
`research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` (all rows there are
extrapolations to `n = 131`, `ℓ ≈ 33`, in unconverted column-touch units; rho is
`2^60.81`):

| reading of `D` at `m = 4` | total | vs rho | source |
|---|--:|--:|---|
| 4, constant (closure) | `2^68.43` | `+7.62` bits | DESCENT_DEGREE §6, "contradicted" by §8 |
| 5, constant (the current optimistic row) | `2^71.13` | `+10.32` bits | §8 |
| 7, constant ("levels at 7", XL framing) | `2^82.11` | `+21.30` bits | §2 table |
| `ℓ + 4` (XL fit, "follows ℓ") | `2^173.13` | `+112.32` bits | §2 table |

The spread between the rows is the security question for the Weil-descent route on
binary curves. Whatever this card finds, no row beats rho; what moves is how far
the optimistic row sits from it, and therefore whether the symmetry constants
(FGHR, Faugère–Svartz) and the unit conversion `K` are worth anything.

## 3. What has already happened (the blocker statement, corrected)

The previous blocker was "they depend on equivariant F4 blocks that are not
integrated into the solver yet, so there is nothing to run". Reading the code and
the studies since 2026-10-10 changes that in three ways.

1. **The cells do not depend on the equivariant blocks, and most of them have
   been read.** Two native instruments (plain Macaulay/XL, `examples/dreg_ladder.rs`;
   the MutantXL fall closure, `examples/dreg_closure.rs` →
   `src/cryptanalysis/koblitz_bench.rs:1493` `closure_refutation_scan`) and one
   external one (msolve F4) already measure them. *Measured* at `m = 4`:
   - msolve F4 degree 4 on 4/4 draws at `(n, ℓ) = (11, 4)`, `N = 38`, `S = −5`;
     `(13, 4)` and `(13, 5)` hit a 3 GB cap while still in degree-4 rounds
     (`research/ic_next10_20261014/06-equivariant-f4-m4/README.md:76–78`);
   - closure degree 4 at `ℓ = 4` (`N = 42`), `ℓ = 5` (`(11, 5)`, `(12, 5)`, 8/8),
     and `ℓ = 6` (`(15, 6)`, `N = 54`, `S = −9`, 2/2 unsatisfiable draws, 25.8 GB
     RSS, 1,413–1,465 s a draw; the satisfiable control took 2,073 s) (`RESEARCH_DREG_MEASUREMENT.md` Results 12–13;
     `research/dreg_closure_m4l6_20261018/RESULTS.md:9`);
   - XL (the instrument behind "levels at 7" and "`ℓ + 4`") reads `≥ 7` already at
     `ℓ = 3` and is 2–3 degrees pessimistic against the closure on 92/92 draws
     (Result 11). The "7 vs `ℓ + 4`" framing is XL's and is superseded as the
     deciding read; it is kept below only as a secondary readout.

   **Not measured anywhere:** any `m = 4`, `ℓ ≥ 4` cell at `S = 0` (every one so far
   is at `S ≤ −3`, chosen because unsatisfiable draws were affordable there); msolve's
   F4 degree at `ℓ = 5, 6`; and any FGHR-symmetrised `m = 4` system.

2. **The G2 (Frobenius) equivariant blocks cannot apply to these cells.** They need a
   system invariant under the Frobenius shift `σ`. The cells use random subspaces
   `V`, which are not `σ`-stable; at `n = 131` (`ord_131 2 = 130`) the only
   `σ`-stable `V ∋ 1` are `F₂` and the whole field
   (`src/cryptanalysis/koblitz_symmetrised.rs:2302–2304`). The 06 study says so
   (`.../06-equivariant-f4-m4/README.md:246`), and the oracle study records that a
   `reduce_system` hook would be dead code because no pipeline system is
   `σ`-symmetric (`research/ic_improve_20261014/02-equivariant-oracle/README.md:30,106`).
   The built pass (block RREF over `F_{2^d}` plus inverse transform, bit-identical to
   full `F₂` RREF at `n = 7–17`) is in `examples/ic_next10_06_g2_full_pass.rs:1–23`;
   against the crate's `gf2_elim` it is 2–3× faster, not 44–73× (02 README:94).

3. **An equivariant lever that does apply exists, and is unbuilt.** The FGHR
   presentation (unknowns `c_{i,t}` per summand, `w_i = u_i² + u_i`, `s = Σu`) is
   symmetric under permutations of the four summand blocks. `S₄` has an odd-order
   abelian subgroup, `C₃ = ⟨(123)⟩`; its order is prime to 2, so the Faugère–Svartz
   non-modular decomposition applies, with characters in `F₄` (`ord_3 2 = 2`): one
   trivial block over `F₂` and one block over `F₄` (the conjugate pair), each about a
   third of the matrix's dimension. The FGHR `m = 4` system itself is not built:
   `symmetrised_terms` returns `None` for `m ≥ 4`
   (`src/cryptanalysis/koblitz_symmetrised.rs:414–443`), although `S₅` in `x` exists
   symbolically (`src/cryptanalysis/semaev_leading_form.rs:584`, `semaev(4)`).
   The 06 protocol's reason for not running an FGHR arm ("the repo's FGHR is
   `fghr_full.rs` on `F_{p³}`"; `06-equivariant-f4-m4/PROTOCOL.md:83–86`) missed this
   binary module.

So the corrected blocker is: **arm A at `S = 0` can run today (stage S0 of
`DESIGN.md`); arm B needs about one to two days of code (M1); the `C₃` blocks need
about two to three more (M2); `ℓ = 6` at `S = 0` on the chained system needs a
larger host and a 128-bit monomial port.**

## 4. Prerequisites

| item | state | where |
|---|---|---|
| draw sampler, exact solution count, chained system | built | `koblitz_bench.rs:1338` `ladder_sample`, `:1200` `chained_solution_count`; `koblitz_groebner.rs:911` `build_decomposition_system` |
| XL and closure degree scans | built, positive-controlled | `examples/dreg_ladder.rs`, `examples/dreg_closure.rs` |
| msolve export of a chained draw | built | `examples/ic_next10_06_chain_export.rs` |
| msolve binary | **not installed on this host**; v0.6.5 from source as `research/dreg_closure_m4l6_20261018/pod/boot.sh:85–89` does | M0 |
| native msolve log reader (the 06 study parsed with Python, `run_f4.py`) | **not built** | M0 |
| FGHR `S₅` in `(w₁..w₄, w_R, s)`, the `m = 4` symmetrised system, its exhaustive gate | **not built** | M1 |
| one harness that emits arms A, B, B0 on the same `(V, x_R)` draws | **not built** | M1 (`examples/m4_formulations.rs`) |
| `C₃` Faugère–Svartz blocks for Macaulay steps and the closure, with agreement checks | **not built** (G2 code is a template) | M2 |
| more than 64 unknowns in the native engine (`F2BoolMono` is a `u64` mask, `src/cryptanalysis/pq_groebner_f2.rs:77–79`) | **not built**; needed only for arm A at `(24, 6)`, `N = 72` | M3 |
| a host above 124 GB | not available | stage S3 |

## 5. Cost derivation

### 5.1 Inputs

- Unknowns. Arm A (chained): `N = (m − 2)·n + m·ℓ = 2n + 4ℓ` (*derived*,
  `examples/dreg_ladder.rs` header). Arm B (FGHR): `N_B = m(ℓ − 1) + 1 = 4ℓ − 3`,
  independent of `n` (*derived*, `koblitz_symmetrised.rs` module header: one bit `ε`
  plus `ℓ − 1` bits a summand). Arm B0 (`S₅` in `x`, no chain): `N = 4ℓ`.
- Input Boolean degree. Arm A: 3. Arm B0: 12 (`semaev_leading_form.rs` module table,
  top Boolean degree `m(m − 1)`). Arm B: **unknown until M1**. *Derived bound:* the
  invariants of the even-translation group satisfy `s² + s = Σ w_i` in
  characteristic 2, so every invariant is `A(w) + s·B(w)`; `deg_{u_i} S₅ = 8` gives
  `deg_{w_i} ≤ 4`, and `wt(3) = 2` gives Boolean degree `d_sys ≤ 4·2 + 1 = 9`, or 5 if
  no `w_i³` term survives.
- Columns at degree `D`: `C(N, ≤D) = Σ_{k≤D} C(N, k)` (*derived*).
- Bytes. One bit per `F₂` entry, so a square dense matrix is `cols²/8` bytes. The dense
  closure's peak was 25.8 GB at 341,056 columns (*measured*, A1), i.e.
  **`≈ 0.222·cols²` bytes** (1.8× the bare square, from rows ≈ 3.5× columns and
  working copies). An `F₄` block stores 2 bits an entry over about a third of the
  columns: `≈ 2·0.222·(cols/3)²`, two ninths of the plain figure. The degree-5
  closure at `m = 3`, `(19, 7)` exceeded this model (169 GB measured and still growing
  at 760,099 columns, against 128 GB modelled), so degree-5 figures are lower bounds.
- msolve. One measured point at `m = 4`: peak 1.05–1.48 GB at `C(38, ≤4) = 82,993`
  columns, 386–805 s wall on a 4-core box at load 15–20 (*measured*,
  06 README:76). Growth from `(9, 3)` (57–83 MB at 31,931 columns) to `(11, 4)` is
  between quadratic and cubic in columns for memory and steeper for time, so
  projections below use a **band**: memory ∝ cols² to cols³ (*derived*), time ∝
  cols³ to cols⁵ (*predicted*, wide; stage S0 opens with a sizing probe instead of
  trusting it).

### 5.2 Arm A (chained, degree 4; degree 5 is out of reach on any host considered)

| cell | `N` | `S` | `C(N,≤4)` | closure RSS (0.222·c²) | msolve RSS band | `C(N,≤5)` | closure at 5 |
|---|--:|--:|--:|--:|--:|--:|--:|
| (13, 4) committed | 42 | −3 | 124,314 | 3.4 GB | 2.8–4.2 GB | 974,982 | 211 GB |
| (13, 5) committed | 46 | −7 | 179,447 | 7.1 GB | 5.8–12.6 GB | 1,550,201 | 533 GB |
| (15, 6) committed (A1) | 54 | −9 | 342,541 | 26 GB (25.8 measured) | 21–88 GB | 3,505,051 | 2.7 TB |
| **P1 (16, 4)** | 48 | 0 | 213,053 | 10 GB | 8–21 GB | 1,925,357 | 823 GB |
| **P2 (20, 5)** | 60 | 0 | 523,686 | 61 GB | 50–314 GB | 5,985,198 | 8 TB |
| **P3 (24, 6)** | 72 | 0 | 1,091,059 | 264 GB (and `N > 64`) | 216 GB–2.8 TB | 15,082,603 | 50 TB |

Time for the closure: 1,413–1,465 s at 341k columns (*measured*), work
`∝ cols^2.2–2.3` (*measured*, scale study Q6), so about 0.2 h at P1, 1.1 h at P2,
6 h at P3 per draw, single-threaded (*derived*). msolve at `(11, 4)`'s rate scaled by
the band: P1 2–25 h, (13, 5) 1–10 h, (15, 6) 8–270 h, P2 27 h to far beyond any
wall, per draw (*predicted*).

**Reading:** on this host (124 GB, shared) arm A's closure fits at P1 and P2 at
degree 4; P3 needs about 300 GB and the 128-bit monomial port. A degree-5 build of
arm A does not fit anywhere considered, so a non-refutation at degree 4 is read as
the censored value `D ≥ 5`, never as a degree.

### 5.3 Arm B (FGHR), plain closure and with `C₃` blocks

| cell | `N_B` | `D` | `C(N_B, ≤D)` | plain RSS | `F₄` block RSS |
|---|--:|--:|--:|--:|--:|
| P1 (16, 4) | 13 | any | ≤ 8,192 (whole ring) | < 0.1 GB | — |
| P2 (20, 5) | 17 | ≤ 10 | ≤ 109,294 | ≤ 2.7 GB | ≤ 0.6 GB |
| P3 (24, 6) | 21 | 8 | 401,930 | 36 GB | 8 GB |
| P3 | 21 | 9 | 695,860 | 108 GB | 24 GB |
| P3 | 21 | 10 | 1,048,576 | 244 GB | 54 GB |
| E1 (28, 7) | 25 | 7 | 726,206 | 117 GB | 26 GB |
| E1 | 25 | 8 | 1,807,781 | 726 GB | 161 GB |
| E1 | 25 | 9 | 3,850,756 | 3.3 TB | 732 GB |
| E2 (32, 8) | 29 | 6 | 621,616 | 86 GB | 19 GB |
| E2 | 29 | 7 | 2,182,396 | 1.06 TB | 235 GB |

All *derived*. Since `D ≥ d_sys`, arm B's reach depends on `d_sys` (M1 output):

- if `d_sys ≤ 7`, P1–P3 fit this host plainly and E1, E2 fit a 512 GB host with
  blocks;
- if `d_sys = 9`, P3 fits this host only with the `C₃` blocks (24 GB at `D = 9`,
  54 GB at `D = 10`), and E1 needs about 0.75 TB even with blocks.

### 5.4 Why arm B's value is gated on `d_sys` (derived, `n = 131`)

The oracle term of the DESCENT_DEGREE cost model is `C(N, ≤D)` a target (`w = 1`). At
the priced `ℓ = 33`:

| arm | `N` | `D` | `log₂ C(N, ≤D)` |
|---|--:|--:|--:|
| A, chained | 394 | 4 | 29.90 |
| A, chained | 394 | 5 | 36.19 |
| B, FGHR | 129 | 5 | 28.09 |
| B, FGHR | 129 | 6 | 32.48 |
| B, FGHR | 129 | 7 | 36.62 |
| B, FGHR | 129 | 8 | 40.57 |
| B, FGHR | 129 | 9 | 44.33 |
| B0, `S₅` in `x` | 132 | 12 | 55.10 |

So FGHR can improve on the current optimistic row (A at degree 5) only if its
degree is at most 6, and can tie it at 7. If M1 finds `d_sys ≥ 8`, arm B cannot
move any ECC2K-130 row whatever it measures, and it is run on this host only as a
mechanism study (DESIGN gate G1). This comparison ignores the small changes FGHR
makes to the relation and target counts; the result PR re-optimises with
`examples/descent_degree_cost` extended by an `N_B` formula.

### 5.5 What fits where

| host | runs |
|---|---|
| **(a) this host**: 32 vCPU Xeon 6975P, AVX-512, 124 GB, 2 TB EBS, shared with a multi-day job | S0: msolve on the committed (13, 4), (13, 5) draws; the arm-A closure at P1 and P2 (10 and 61 GB); msolve at P1 if the probe projects < 72 h and < 100 GB. S1–S2: arm B at P1–P3 (with `C₃` blocks at P3 if `d_sys ≥ 8`), B0 at P1. At most 4 concurrent single-threaded jobs, at most 100 GB in total. |
| **(b) a 512 GB host** (32–64 vCPU, 500 GB disk) | S3: arm A closure at P3 (264 GB, after the 128-bit port); msolve at (15, 6), P2 and P3 multi-threaded where the S0 probe projects they fit; arm B at E1 to `D = 8` and E2 to `D = 7` with blocks (161–235 GB). |
| **(c) 1 TB or more** | only if `d_sys = 9` and E1 is wanted at `D = 9` (732 GB with blocks). Not proposed unless S1–S2 call for it. |

Core-hours: about 350 on host (a) (*predicted*, dominated by msolve's upper band,
capped by a 72 h wall per draw), about 100–200 on host (b) for the closure and
blocked arm-B cells, plus whatever the msolve probes justify there.

## 6. What "done" means

- A study directory under `research/m4_solving_degree_fghr_20261018/` (the card's
  `outputs.land`) with one table per arm: rows = cells, columns = `N`, `S`, draws,
  per-draw `D` (closure), `D` (msolve) where run, `D` (XL) where reachable, censoring
  flags, columns at `D`, peak RSS, CPU s; the satisfiable controls; the positive
  controls; the `C₃` agreement table (E1–E5, zero mismatches) wherever blocks were
  used; the frozen PROTOCOL's decision rule applied cell by cell; the report, graph and
  PDF the research-visuals skill requires.
- A derived update to `RESEARCH_ECC2K130_DESCENT_DEGREE.md` that adds or withdraws
  rows according to the reading (no row is promoted to a measurement).
- If M2 was built: the `C₃` engine's speed and memory against the plain closure and
  against msolve on the same arm-B systems (AGENTS.md §15), with a `benchmarks/`
  entry.
- The card moved to `done` (or `failed` with evidence). A cell that could not be run
  is reported BLOCKED with its wall, never as a reading.

## Correction 2026-10-24: the +10.32 bits row is a w = 1 floor

The `m = 4`, degree-5 row quoted above as `2^71.13`, `+10.32` bits over rho is priced at `w = 1`
(one touch per Macaulay column), which `RESEARCH_ECC2K130_DESCENT_DEGREE.md` §2 itself describes as
a price "no elimination beats". It is a **lower bound**, not the optimistic case. The same note
already prints the `w = 2` row in §7/§8: `2^106.52`, `+45.71` bits. The measured exponent of the
repository's own dense closure is 2.18 to 2.33 (`research/dreg_closure_driver_20261016/RESULTS.md`),
and the recomputation in `research/m4_reprice_20261024/` (README.md, `table.md`, `examples/m4_reprice.rs`;
the existing rows reproduce exactly as a positive control) gives for `m = 4`, degree 5:

- **measured-exponent optimistic case, w = 2.25: `2^115.58`, `+54.77` bits** in the note's column-touch
  units, and `2^107.14`, `+46.33` bits in rho steps (word-op to rho-step conversion of
  `-8.44` bits from `research/k_conversion_20261018/raw.jsonl`);
- bracket for w from 2.0 to 2.33: `+45.7` to `+57.7` (column touches), `+37.3` to `+49.2` (rho steps).

The `K` of `k_conversion_20261018/README.md` (Ir per column) is not stacked on `cols^w`: it already contains
the super-linear elimination cost. w at 2^36 columns is extrapolated from matrices of 2^12 to 2^14
columns and is the dominant uncertainty. The degree cell (5 held to N near 394) remains unmeasured. The original text above is
left unchanged.
