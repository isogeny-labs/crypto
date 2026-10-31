# Results: fall-degree experiments E1, E1b, E2 (`m = 2`)

**Latest E12c preservation update (2026-10-28):** the additional `4:28:7`,
`N=84` Spot attempt retained **one of four requested completed draw records**
before reclaim (exit 143). Its window is uncertified, with null degree-result
fields: this is **inconclusive**, not a `d_F >= 5` measurement. See the appended
interrupted-continuation section and [report](e12/interrupted-c-20261028/README.md).

**Latest continuation (2026-10-28):** E12b cell `3:24:7` completed four draws,
`≥5, 4, ≥5, 4`. Amendment 15's frozen lower-median rule gives **4**, hence
the registered reading **“mixed.”** See the appended completed-continuation
section and [report](e12/completed-20261028/README.md). Earlier dated sections
below retain their historical state.

Scored on 2026-10-08 by `score.py` (output in `score-output.txt`), from
the committed JSONL in `runs/` and `exp/`, against `PREREGISTRATION.md`
and its Amendment 1. E3, E4 and E5 have not run.

## Verdicts

| hypothesis | as run | Amendment 1 | decided by |
|---|---|---|---|
| H1: the trace relation `L` lies in `span(F)` | falsified, 58/112 | **held, 112/112** | the as-run script used constant `Tr(x_3)`; every as-run failure is a constant mismatch |
| C1: `Tr\|_V ≢ 0` gives `d_ff = 2` | **held, 67/67** | same | — |
| C2: `Tr\|_V ≡ 0` with `Tr(b/x_3²) = 1` is refuted at degree 2 | falsified, 8/22 | **held, 17/17** | same cause as H1 |
| C3: `Tr\|_V ≡ 0` with constant 0 gives no trace fall | consistent | random bases: `d_ff = 3` with no degree-1 relation on 4/4 | subfield bases get `d_ff = 2` from Prop. B instead |
| side: `dim(span(F) ∩ R_{≤1}) = 1` when `Tr\|_V ≢ 0`, `n ≥ 12` | **held, 38/38** | same | — |
| H1b: the EXP-J syzygy is `(L+1)·L = 0` | **held, 69/69** | same | — |
| H2a: Semaev `d_F` ≤ planted-linear control `d_F` | **held** at `n = 8 … 18` | same | — |
| H2b: the gap is nondecreasing in `n` | **held**, but flat at 1 for `n = 10 … 18` | same | no growth seen at toy size |

## The E2 table: medians of `d_F`

| `n` | Semaev sat | Semaev unsat | plain sat | plain unsat | planted sat | planted unsat | semi-regular `D_reg` |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 8 | 3 | 3 | 3 | 3 | 3 | — | 3 |
| 10 | 3 | 3 | 4 | 4 | 4 | 4 | 4 |
| 12 | 3 | 3 | 4 | 4 | 4 | 4 | 4 |
| 14 | 3 | 3 | 4 | 4 | 4 | 4 | 4 |
| 16 | 3 | 3 | 5 | — | 4 | 4 | 5 |
| 18 | 4 | 4 | 5 | 5 | 5 | 5 | 5 |

The Semaev values come from `runs/`, random base. The control cells
have 4 draws each, and the Semaev cells 3–8.

## What the results establish

- **The trace relation and its consequences are now proved and
  confirmed.** Prop. A of the ledger
  (`research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md` §5)
  derives `L = Tr(X_1) + Tr(X_2) + Tr(b/x_3²)` from Kosters–Yeo Cor. 4.11.
  E1 confirms it on every draw.
- **The repository's "bounded-defect lemma" is explained.** Its one
  syzygy is the Boolean shadow of the trace relation. That gives a proof
  of existence. It also removes it as evidence for a lower bound: one
  linear equation is all it carries.
- **Subfield bases are easy at degree 2, by a theorem.** Prop. B: the
  quadratic content of the subfield descent lives in an `n'`-dimensional
  space. Degree 2 then yields `n' − 1 + Tr(b/x_3²)` affine relations in
  `X_1 + X_2`, measured on 39 of 40 draws. The outlier has
  `x_3 ∈ F_{2^{n'}}`.
- **The low `d_F` is not generic.** A random quadratic system of the same
  shape, with the trace fall planted, needs one more degree at every
  `n = 10 … 18`. The plain control tracks the semi-regular `D_reg`, and
  Semaev sits one below it. So whatever keeps `d_F` low is structure
  beyond the trace fall, and it is not yet identified.
- **What it does not establish.** Whether that gap grows. At `n ≤ 18` it
  is a constant 1. Kousidis–Wiemers' F4 bound, `d_F ≤ 5` at `n = 48`
  against a semi-regular 8, says it must grow eventually. E3 tests where.

## Next

- **E5 (native closure engine).** It is the gate for E3 and E4. The
  Sage closure costs 3–4 minutes per draw at `n = 22` and will not reach
  `n = 32`.
- **E2 extension.** Run the planted control at `n = 20, 22`, where
  Semaev reads 4 and the semi-regular reference is 5. This is the first
  place the gap could reach 2 within Sage's reach.
- **P3.** Prove that `span(F) ∩ R_{≤1}` is exactly `⟨L⟩` for a random
  base. E1 supports it on 38 of 38 draws at `n ≥ 12`.

---

# Results: E5, E3, E4, E6 (2026-10-12), E7 (2026-10-19) and E8 (2026-10-25)

These results come from the native engines and the amendments registered before each run
(Amendments 2–9 of `PREREGISTRATION.md`).
- Run reports, graphs and PDFs: [reports/run-20261012/](reports/run-20261012/) (E3–E6) and
  [reports/run-20261019/](reports/run-20261019/) (E7, the E6 completion) and
  [reports/run-20261025/](reports/run-20261025/) (E8).
- Lean scoring: [lean/](lean/).
- Records: `e3/`, `e4/`, `e5/`, `e6/`, `e7/`, `e8/`.
- Provenance notes, including three in-place rebuilds and one container restart, with why
  none changes a value: `e3/PROVENANCE.md`, `e6/PROVENANCE.md`.

## Headline

**At every size reachable here, the Huang–Kosters–Yeo last fall degree `d_F` of
Weil-descended Semaev systems is 3 or 4.**

| `m` | reach |
|--:|:--|
| 2 | `n ≤ 32`, ℓ ≤ 16 |
| 3 | ℓ ≤ 6, `N ≤ 36` |
| 4 | ℓ ≤ 3 (`N ≤ 30`) |
| 5 | ℓ = 2, `N = 40` |

Over the same cells:
- the plain Macaulay refutation degree `D*` of the `dreg_ladder` rises from 5 to `≥ 7`;
- structure-matched random controls rise 5 → 6 → `≥ 7` as `N` grows.

The flat value is a property of Semaev structure, not of the system's shape. It is toy-size
evidence on an open question. The ledger's conditional (§2.2) says `d_F` must eventually grow
like `ω(m²)` unless ECDLP over `F_{2^n}` is subexponential. **No exponent or security claim
is made.**

## E5: native engine — gate passed

`examples/hky_last_fall_degree.rs` reproduces all 32 reproducible Sage records exactly
(`subfield` cells, `n = 8–14`; `e5/replay_subfield.jsonl`):
- `DG`, `d_last` and `e₀`;
- the closure, ideal and plain dimensions at every shared degree.

The random-base Sage records do not store `V`, so they are compared by distribution (§E3).
That deviation is disclosed in Amendment 2.

## E3 (`m = 2`, random base, 8 draws × rand and sat per `n`) — registered scoring

| `n` | 18 | 20 | 22 | 24 | 26 | 28 | 32 |
|:--|--:|--:|--:|--:|--:|--:|--:|
| `d_F`, all 16 draws | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Sage medians (`TABLE.md`) | 4 | 4 | 4 | — | — | — | — |
| semi-regular `D_reg` | 5 | 5 | 5 | | | | |

- **P3a** (`d_F ≤ 5` at every `n ≤ 32`): **held**, 112 of 112 draws.
- **P3b** (`d_F` steps from 4 to 5 at most once between 18 and 32): **held**. It never steps.
- **LOG against LIN** (fitted on the medians at `n ≤ 22`, `e3/fit.md`):
  - LOG: `d = 0.27 + 0.81·log₂ n`; LIN: `d = 2.04 + n/11.2`.
  - Mean absolute error on `n = 24–32`: LOG 0.145, LIN 0.491. **LOG is preferred.**
  - Even LOG over-predicts at `n = 32` (4.32 against 4).
- **Kill condition for "`d_F` bounded"** (`d_F` reaches 5 by `n = 32`, with step widths
  growing less than geometrically): **not met.** The `m = 2` values agree with the
  Kousidis–Wiemers F4 bound (≤ 5 at `n = 48`).

## E4 (`m = 3`, registered cells, committed unsatisfiable `dreg_ladder` draws)

| cell | `N` | `D*` (recorded) | `d_F` |
|:--|--:|--:|:--|
| (5, 2) | 11 | 5 | 4 3 4 4 |
| (7, 2) | 13 | 5 | 4 4 4 3 |
| (5, 3) | 14 | 6 | 4 4 4 4 |
| (7, 3) | 16 | 6 | 4 4 4 4 |

- **P4a** (`d_F ≤ D*`): **held** on 16 of 16 draws.
- **P4b** (`d_F < D*` on at least half): **held** on 16 of 16.
- **P4c**: not tested (Amendment 3).

**Independent check.** Every draw was recomputed by `examples/hky_crosscheck.rs`, which is
round-based, fully reduced and uses its own monomial order. It agreed on every draw, and its
plain refutation degree reproduced the recorded `D*` on every draw.
- A first version of the cross-check pivoted on the lowest monomial. That breaks the graded
  property, so it under-generated and disagreed. It was fixed to Sage's convention before
  any number from it was used. The episode is recorded in the file header.

## E4 extension and E6 Parts A and B (exploratory, Amendments 3 and 4)

| `m` | cell | `N` | `D*` (recorded) | `d_F` | cross-checked |
|--:|:--|--:|:--|:--|:--|
| 3 | (9, 3) | 18 | 6 | 4 4 4 4 | yes |
| 3 | (11, 4) | 23 | 6 | 4 4 4 4 | yes |
| 3 | (13, 4) | 25 | 6 | 4 4 4 4 | yes |
| 3 | **(13, 5)** | 28 | **≥ 7** | 4 4 4 4 | yes |
| 3 | **(15, 5)** | 30 | **≥ 7** | 4 4 4 4 | yes |
| 3 | (18, 6), new draws | 36 | — (the plain fit predicts 7) | 4 4 4 4 | no (too large for the cross-check) |
| 4 | (5, 2) | 18 | 6 | 4 4 3 3 | yes |
| 4 | (7, 2) | 22 | 6 | 4 3 4 4 | yes |
| 4 | **(9, 3)** | 30 | **≥ 7** | 4 4 4 4 | yes |
| 4 | (13, 4), new draws | 42 | — | 4 4 4 4 | no; computed by `hky_par` (Amendment 7), which reproduces the validated engine on 24 committed draws and on all four (10, 2) draws |
| 5 | (10, 2), new draws | 40 | — | 4 3 4 4 | no (draws 0–2 reproduced exactly by a rerun after a container restart) |

- (13, 5) and (15, 5) are the cells the "does the degree level off at 7?" question was about.
  Their plain tower needs degree ≥ 7, about 110–120 GB to build. Their `d_F` is 4, at
  24,158 and 31,931 columns.
- **Amendment 3 reading rule** (`d_F ≤ D* − 2` at the largest ℓ for both `m`): **"`d_F` flat
  while `D*` rises".**

## E7: `m = 5, 6`, ℓ up to 3, `N = 44–54` (Amendments 7 and 8)

The engine is `examples/hky_par.rs`, the parallel and chunked closure. The gate was run on
the exact binary that ran E7 (sha256 `24cf10d6…`):
- it reproduces `d_F` and every per-degree closure dimension of the validated engine on 24
  committed draws (`e7/validate.log`: `AGREE 24 DISAGREE 0`);
- it reproduces all four m = 5 (10, 2) draws of E6 Part B bit for bit
  (`e7/gate_B_5-10-2.jsonl`).

| `m` | `(n, ℓ)` | `N` | columns `≤ 4` | `d_F` | degree-4 closure, s (4 threads) |
|--:|:--|--:|--:|:--|:--|
| 6 | (8, 2) | 44 | 149,986 | 4 4 4 4 | 68–172 |
| 5 | **(11, 3)** | 48 | 213,053 | 4 4 4 4 | 334–553 |
| 6 | (9, 2) | 48 | 213,053 | 4 4 4 4 | 90–124 |
| 5 | **(13, 3)** | 54 | 342,541 | 4 3 4 4 | 728–839 |

- **Reading (Amendment 7): "`d_F` bounded through `m = 6`".** All 16 draws have
  `d_F ≤ 4`, on both the local tier and the moved N = 54 cell.
- **The degree-4 closure is the whole space.** `1 ∈ V_{F,4}` on every draw with
  `d_F = 4`. The one `d_F = 3` draw refutes at degree 3 in 2 s.
- **Memory.** The N = 54 cell peaked at 12.6 GB (VmHWM, `e7/peak_5-13-3.txt`) on the
  15 GB host.
- **Not run:** the large-host tier, (6, 10, 3) N = 58 and (5, 15, 3) N = 60. No host with
  ≥ 64 GB was available, because AWS credentials are incomplete in this environment.
  These cells are not extrapolated.
- **N ≈ 100 is out of reach** of dense degree-4 elimination: 4.1M columns, a ≈ 1 TB
  basis. It needs a sparse or structured method.

## E8: `d_F` against `m` at fixed `(n, ℓ)` (Amendment 9)

Same binary as E7 (sha256 `24cf10d6…`), 4 draws per cell, `cmax = 4`. No cell was
cut short. `N = (m − 2)·n + m·ℓ`.

| ladder | `m` | `N` | `d_F` per draw | median |
|:--|--:|--:|:--|--:|
| (8, 2) | 3 | 14 | 4 4 4 3 | 4 |
| (8, 2) | 4 | 24 | 4 4 3 4 | 4 |
| (8, 2) | 5 | 34 | 4 4 4 4 | 4 |
| (8, 2) | 6 | 44 | 4 4 4 4 (E7) | 4 |
| (8, 2) | 7 | 54 | 4 4 3 4 | 4 |
| (6, 2) | 3 | 12 | 4 3 3 4 | 3 |
| (6, 2) | 4 | 20 | 4 3 3 3 | 3 |
| (6, 2) | 5 | 28 | 4 4 4 3 | 4 |
| (6, 2) | 6 | 36 | 4 4 3 4 | 4 |
| (6, 2) | 7 | 44 | 4 4 4 3 | 4 |
| (6, 2) | 8 | 52 | 3 3 3 3 | 3 |

- **Reading (Amendment 9), as registered:**
  - the **(8, 2) ladder is "`d_F` flat in `m`"** (medians 4 4 4 4 4);
  - the **(6, 2) ladder is "mixed"**: no median reaches 5, but the medians 3 3 4 4 4 3 are
    not non-decreasing, because `m = 8` drops to 3.
- **`d_F` does not grow with `m` here.** No draw has `d_F ≥ 5`, up to `m = 8`.
- **The `m = 8`, `n = 6` drop is a small-field effect, not evidence for anything.** Here
  the field is `F_{2^6}` and the factor-base span has 4 elements. An 8-summand chain
  over so few points is refuted at degree 3 in about 2 s on every draw. The registered
  rule is not adjusted for this: the ladder reads "mixed" as written.
- **Cost.** The degree-4 closure at (8, 2), `m = 7`, `N = 54` takes 228–339 s per draw,
  against 728–839 s for the E7 `m = 5` (13, 3) cell at the same `N`. At this one `N`, raising `m`
  was cheaper than raising `ℓ`; two cells are not a trend.
- **What this does not test.** `m` here is at most 8 with `n ≤ 8`, so `m ≈ n^{1/3}` (HKYY's
  regime) is not reached.

## N = 58–60 and N ≈ 100: status

- **`N = 58–60`: not run.** `AWS_SECRET_ACCESS_KEY` and a region are still absent from the
  environment. `e7/aws_large_tier.sh` is prepared (user-data, no SSH, gate-first, S3
  upload, self-terminating) but has not been run, and has never been syntax-tested
  beyond `bash -n`.
- **`N ≈ 100`: not measured.** The dense degree-4 basis would be ≈ 1 TB.
  `e8/N100_SCOPING.md` records what a sparse engine would have to do and why fill-in is
  the unknown that decides it.

## E6 Part C: satisfiable draws (`m = 3`)

- `d_last = 4` on all 6 satisfiable draws available: 4 from (7, 3) and 2 from (11, 4). The
  (9, 3) records hold no satisfiable draw.
- The enumerated solution sets match the ladder's recorded counts exactly: 6, 3, 12, 15, 6
  and 6.
- `DG = 2` on every draw.

## E6 Parts D and D′: structure-matched random controls

- **Part D** is not testable. Every control is satisfiable, because `random_control_system`
  plants a root (Amendment 6).
- **Part D′** compares `d_last` on those satisfiable controls with the Semaev systems at the
  same `(n_vars, terms/eq)`:

| shape | `N` | control `d_last` | Semaev |
|:--|--:|:--|:--|
| `n5l2` | 11 | 5 5 5 5 | `d_F` 3–4 |
| `n7l3` | 16 | 6 6 6 6 | `d_F` 4; satisfiable `d_last` 4 |
| `n11l4` | 23 | ≥ 7 ≥ 7 ≥ 7 ≥ 7 | `d_F` 4; satisfiable `d_last` 4 |

**Amendment 6 reading: "Semaev-specific".** The control median is `≥ 5` at every shape, and
it rises with `N` while the Semaev systems stay at 4.

## Lean and cairn

- **`lean/FallDegree/Verdict.lean`**, kernel `decide` over 246 rows, proves
  `report rows = ⟨p3a, p3b, p4a, p4b, bounded, semaevSpecific⟩ = all true`. The 246 rows are
  generated from `e3/`, `e4/`, `e6/`, `e7/` and `e8/` by `examples/fall_degree_lean_data.rs`. Changing one
  verdict fails the build.
- **Disclosure.** `Rule.lean` transcribes the prose rules of the pre-registration and
  Amendments 2–9. The transcription was written after part of the data had been seen. The
  prose rules it encodes were not.
- **E7 in Lean.** E7 rows are kind 4. `e7Bounded = true` and `e7Grows = false` are part of
  the same `decide` theorem. Changing one E7 draw to 5 fails the build.
- **cairn.** The result objective `sha256:fc828009…` (E3/E4/E6) and its E7 successor
  `sha256:16fc1da8…` (206 rows) are the same theorem, over `Rule.lean` and `Data.lean`.
  The E8 result objective `sha256:23aabfa4…` (246 rows) adds the ladder readings. Changing
  three `m = 7` draws to 5 (enough to move the median) fails the build; changing one does
  not, because the rule uses medians.
  The first E7 attempt returned "unavailable", because `lean` was not on cairn's PATH. It
  is recorded in the log; the retry was accepted. cairn's Lean jail accepted it. `CAIRN_EPOCH_SECONDS=5 cairn --log
  cairn/log.jsonl audit` reports "chain intact".

## What this does and does not show

- **It shows** the following, at toy size:
  - the mutant-closure (polynomial-calculus) degree of these systems is 4;
  - the degree the repository's plain towers need is not intrinsic;
  - a degree-4 closure decides cells the plain tower could not build on this host.
    `m = 4` (9, 3) takes about 100 s with 31,931 columns, against the plain tower's
    degree-7 matrix of 2.1M × 2.8M.
- **It does not show** that `d_F` is bounded. HKYY's conditional says it cannot stay
  bounded for `m ≈ n^{1/3}` unless binary ECDLP is subexponential. Here `m ≤ 6` and
  `N ≤ 54`.
- **Conditional model** (`e6/crossover_model.md`, `examples/fall_degree_crossover.rs`). If
  `d_F = 4` held at every size, and one dense elimination in `V_{F,4}` cost `M(4)^ω`:
  - IC would beat rho only from `n ≈ 283` (ω = 2.81) or `n ≈ 233` (ω = 2.37);
  - ECC2K-130 stays above rho by 51 or 36 bits.

  That gives the sizes where the bounded-`d_F` question would start to matter for
  security: the NIST binary curves K-233 and K-283 and up. It does not say that they are
  at risk.
- **Next decisive measurement:** E7 measured `m = 5` with ℓ = 3 up to `N = 54`, and
  `d_F` stayed 4. What remains:
  - the registered large-host cells, `N = 58–60`;
  - `N ≈ 100`, which needs a sparse closure engine rather than more RAM;
  - and, more informative than raw `N`, larger `m` at fixed `n`, since HKYY's growth is
    in `m`.

---

# Results: E9 (N = 58–60) and E10 (the balanced diagonal `mℓ = n`), 2026-10-27

Registered in Amendment 10 before any draw.
- **Engine:** `hky_par` at source commit `5296ce8a7438`. Each instance built it from an S3
  `git archive`; the binary sha256 was `533f182b…` on all three.
- **Hosts:** `ephemeral` Spot, lab account, us-east-1. m7a.8xlarge for job A, m7a.4xlarge
  for jobs B and C. No Spot interruption.
- **Gates 0–3:** passed on every instance.
- **Records:** `e9/run-r3/`, `e10/run-r3-B/`, `e10/run-r3-C/`.
- **Report:** [reports/run-20261027T0517Z/](reports/run-20261027T0517Z/) (`report.pdf`, graph).

## Headline

**The first `d_F > 4` in this study.** On the `m = 3` diagonal, every draw of (21, 7),
(24, 8) and (27, 9) has a degree-4 closure that stops without 1. The closure reaches 47–95%
of the degree-≤4 space, so these draws are recorded as `d_F ≥ 5`.

Every other cell up to `N = 60` reads `d_F ≤ 4`. That includes:
- the `m = 4` diagonal to (20, 5);
- the `m = 5` point (15, 3);
- the `m = 6` point (12, 2);
- the E9 cell (6, 10, 3).

| exp. | cell m:n:ℓ | `N` | `d_F` per draw |
|:--|:--|--:|:--|
| E9 | 6:10:3 | 58 | 4 3 4 4 |
| E9 | 5:15:3 | 60 | 4 4 4 4 |
| E10 | 4:12:3 | 36 | 4 4 4 4 |
| E10 | 4:16:4 | 48 | 4 4 4 4 |
| E10 | 4:20:5 | 60 | 4 4 4 4 |
| E10 | 6:12:2 | 60 | 3 3 4 4 |
| E10 | **3:21:7** | 42 | **≥5 ≥5 ≥5 ≥5** |
| E10 | **3:24:8** | 48 | **≥5 ≥5 ≥5 ≥5** |
| E10 | **3:27:9** | 54 | **≥5 ≥5 ≥5 ≥5** |

## Registered readings (Amendment 10)

- **E9:** **"`d_F` bounded"**. Every large-host draw has `d_F ≤ 4`. Pooled with E7,
  every draw with `m ≥ 5`, up to `N = 60`, is `≤ 4`.
- **E10, `m = 3` diagonal:** **"`d_F` grows on the diagonal"**. With E6 (18, 6), the
  medians read 4, then ≥5, ≥5, ≥5.
- **E10, `m = 4`, `5`, `6` diagonals:** **"flat"**.
- **Cross-`m` at fixed `N`:** **"not rising"** at `N = 48` (`m = 3`: ≥5, `m = 4`: 4) and
  at `N = 60` (4, 4, 3). At `N = 48` the order is the opposite of "rises with `m`".

**Lean.** `lean/FallDegree/Verdict.lean` (`registered_scoring`) now has 282 rows, adding
E9 as kind 6 and E10 as kind 7. It states the readings above. Its kernel build runs in
job E12b, because there is no Lean toolchain on the authoring host. Its status is recorded
below when that job finishes.
- **Disclosure:** the Amendment 10 rules were transcribed into `Rule.lean` after the
  E9/E10 data had been read. The prose rules were not changed.

## What this does and does not show

- **Observed pattern, not registered:** the censored draws are exactly the draws with
  `ℓ ≥ 7`. All of them have `m = 3`. Every draw with `ℓ ≤ 6`, at any `m` and any `N ≤ 60`,
  has `d_F ≤ 4`.
  - So growth tracks the factor-base dimension per summand, not `N` or `m`.
  - That fits the HKYY conditional, which needs `d_F` to grow along `n' = n^{2/3}`.
  - The data cannot yet separate an `ℓ` effect from an `n` effect at `m = 3`, or from an
    `m = 3`-only effect. Amendment 11 (E12b, E12c) tests both.
- **`≥ 5` is a censored value.** The exact `d_F` (5, 6 or 7?) is not measured. E12a runs
  the degree-5 closure on two of the (3, 21, 7) draws.
- **It is not a statement about the first fall degree assumption.** Kousidis–Wiemers
  measured a first fall degree of 7 at `m = 3` (`m² − m + 1`). A last fall degree of
  `≥ 5` there is consistent with that and refutes nothing.
- **Independent checks of (3, 21, 7):**
  - The E11 subagent's run on its own host and build read `d_F ≥ 5` on the same four
    draws. Two solution counters agree on 0 solutions (`e11/scaling/`).
  - That run uses the same closure code.
  - The separately written round-based engine `hky_crosscheck` is running on draw 0
    (`e10/crosscheck/`). Its result is added here when it finishes.
- **Toy size only.** No exponent and no security claim.

## E11: restricted-closure engine (exploratory, not registered)

See `e11/ENGINE.md`.
- **Soundness:** a degree-4 closure restricted to a monomial subset is a subspace of
  `V_{F,4}`. Reaching 1 in it therefore certifies `d_F ≤ 4`; it can never certify
  `d_F ≥ 5`.
- **Validation:** 0 soundness violations on 124 draws. Policy `w1x` certified all 90
  committed draws with dense `d_F = 4`, using 11–64% of the dense columns.
- **N ≈ 100 numbers:** projections only.
- **Use:** E12c applies it to (4, 28, 7), `N = 84`.

## E12 (Amendment 11): interim state, 2026-10-27 20:40 UTC

No E12 part has a registered reading yet. This section records what is fixed so far (updated 2026-10-27 22:10 UTC with E12b r2).

**Lean (E12b, job r1).** The `FallDegree` package builds with `lake build`
(`e12/run-b1/out/lean/status`: `exit=0`; `lake.log`: "Build completed successfully
(5 jobs)"). Built modules: `Rule`, `Data`, `Verdict`. The 282-row `registered_scoring` table
from Amendment 10 therefore type-checks in the kernel.

**E12a and E12c: not completed.** Both jobs were Spot-reclaimed twice:
- r1: SIGTERM before any draw.
- r2 (`e12/launch2.card.yaml`): gates passed, then SIGTERM. `run-a2` was killed at
  16:03 UTC after it started 3:21:7. `run-c2` was killed at 16:07 UTC after it started 4:28:7.
  Neither wrote a draw (`E_3-21-7.jsonl` and `W_4-28-7.jsonl` are empty).

Amendment 11 allows one relaunch per interruption, and both jobs have used it. Under its
rules they are **not completed**, with no data, and have **no reading**. A third attempt needs
a new amendment before launch.

**E12b: both attempts reclaimed at 3:24:7.** The first attempt (r1, `run-b1`) and the one
Amendment-11 relaunch (r2, `run-b2`; job `20261027-181036-fd-e12b-ell-20261027-r2-409f97`,
m7i.4xlarge Spot, archive commit `371035216942`) both passed gates 0–3. Both finished 3:30:4,
3:27:5 and 3:24:6, and draw 0 of 3:24:7. Both were then SIGTERM'd by a Spot reclaim, r2 at
19:06 UTC. r2 reproduces r1 exactly on every non-timing field (13/13 draws: `d_F`,
per-degree dimensions and rounds, `v_basis`, `x_r`, solution count). The relaunch budget is
spent.

| cell (m:n:ℓ) | N | status | `d_F` per draw (r2) | median |
|:--|--:|:--|:--|:--|
| 3:30:4 | 42 | completed | 4 4 4 4 | 4 |
| 3:27:5 | 42 | completed | 4 4 4 4 | 4 |
| 3:24:6 | 42 | completed | 4 4 4 4 | 4 |
| 3:24:7 | 45 | **not completed** (1 of 4 draws) | ≥5 | — |

Against the Amendment 11 rules:
- **"`n`-driven" is excluded.** It needs 3:30:4 and 3:27:5 at median `≥ 5`; both are 4.
- **"Consistent with H_ℓ"** needs the three `ℓ ≤ 6` cells at median `≤ 4`, which holds, and
  3:24:7 at median `≥ 5`. 3:24:7 is not completed, with one finished draw, which is censored
  at `≥ 5`. Amendment 11 says a cell that does not complete is reported "with whatever
  finished". It does not say whether one draw gives a median. **The reading is therefore
  left open** between "consistent with H_ℓ (on one 3:24:7 draw)" and "mixed". It is decided
  only by a registered amendment or a completed 3:24:7 cell, not here.

The Lean build passed again on r2 (`run-b2/out/lean/status`: `exit=0`).

**Report:** `reports/run-20261027T1811Z/`. It contains `report.pdf`, `report.html` and the graph `e12b_r2_fall_degree.svg`/`.png`, made by `plot_e12b.py` from the r2 records.

**Cross-check (`hky_crosscheck`, 3:21:7 draw 0).** Both local attempts were lost to container
restarts (`e10/crosscheck/attempt{1,2}_lost_on_container_restart.*`). Attempt 2 ran about
2 hours without finishing on the 4-core authoring host. It has no result.

## E12b (Amendment 15): completed continuation, 2026-10-28

The additional authorized Spot attempt
`20261028-031317-fd-e12b-spot-20261028-r3-24148b` completed cell `(3,24,7)`,
`N=45`, seed `20261012`, four draws, degree cap 4, exit 0. The frozen source is
`371035216942660c4615dfbb88d85a14afaf5c16` and protocol commit is
`020f03193c46d975935a065c966c81941cbdbf6d`. All original gates and the additional
draw-0 identity gate are recorded as passing. Raw rows, hashes, read-only record
verification and the final S3 archive reference are in
[`e12/completed-20261028/`](e12/completed-20261028/README.md).

| Cell (m:n:ℓ) | N | Data retained for registered reading | Recorded `d_F` | Lower median |
|:--|--:|:--|:--|--:|
| 3:30:4 | 42 | carried forward from r2 | 4, 4, 4, 4 | 4 |
| 3:27:5 | 42 | carried forward from r2 | 4, 4, 4, 4 | 4 |
| 3:24:6 | 42 | carried forward from r2 | 4, 4, 4, 4 | 4 |
| 3:24:7 | 45 | completed Amendment-15 attempt | ≥5, 4, ≥5, 4 | **4** |

**Mechanical registered reading: “mixed.”** Amendments 12/15 specify the second
smallest of four values, with a censored result scored as `cap + 1` only for
this statistic. `[5,4,5,4]` sorts to `[4,4,5,5]`; two of four score at least 5,
where the rule requires at least three. The old partial draw 0 is used as an
identity reference and is not counted again. This resolves the missing-cell
reading under the frozen rule; it is not a hypothesis promotion or an
asymptotic conclusion.

The record audit checked the archive checksum, frozen protocol and identities,
carried-forward rows, draw-0 non-timing equality, and gate records against their
references. It did not independently execute the closure algorithm or establish
unsatisfiability/saturation. Both `≥5` rows remain censored; their exact degrees
are unknown. The prior 282-row Lean table excludes these four new rows. No new
Lean proof or independent implementation validation is claimed.

The [PDF](e12/completed-20261028/report.pdf) and censoring-aware
[graph](e12/completed-20261028/draws.svg) describe this completed attempt. At the
time of that report, E12a and E12c were separate ongoing attempts. Both have now
ended after Spot reclaim; their terminal records follow below. This E12b section
adds no results for them.

## E12c interrupted additional attempt, 2026-10-28

The Amendment-13 attempt at `(m,n,ell)=(4,28,7)`, `N=84`, retained **one of four
requested completed draw records** before Spot reclaim. Draw 0 records zero
solutions, `window.w1x.certified=false`, `d_f_window=null` and `d_f_le3=null`.
These are producer-reported labels. A failed restricted window is inconclusive
and cannot establish `d_F >= 5`; there is no exact degree or lower-bound claim.
The frozen E12c positive reading requires at least two completed degree-4
certifications; this attempt has none and supplies **no reading**.

Job `20261028-021036-fd-e12c-20261028-r3-914d46` records a Spot notice at
07:21:44 UTC, exit **143**, final output upload, then shutdown. The terminal
receipt reports `interrupted`; output status remains `gate_passed`, without the
wrapper's final `done`. Three requested completed records are missing. This is
not an overall completion or a resumable solver-memory checkpoint.

The original and canonical S3 archives passed download/hash verification. The
archive contains 24 regular `out/` files plus `host.txt` and `rusage.txt`; all
24 separately downloaded final outputs match the archive. Archive SHA-256:
`fa4edf72399ec45b293b783d8582d9a26406c1670a3612a75b6110dd1e824f09`.
The raw 775-byte row, original terminal/snapshot receipts, canonical PUT/GET
readback, full identities, [report](e12/interrupted-c-20261028/README.md),
[completion diagram](e12/interrupted-c-20261028/flow.svg) and
[PDF](e12/interrupted-c-20261028/report.pdf) are preserved together.

Gate logs were inspected; no solution recount, closure replay, formal independent
experiment review or Lean proof was performed. Earlier reports remain historical
records. Existing degree plots and unrestricted `fall_degree.*` metrics exclude
this null window result because it is not a censored unrestricted degree. E12a
and the general first-/last-fall-degree questions remain separate unfinished work.

## E12a interrupted additional attempt, 2026-10-28

The Amendment-13/14 attempt at `(m,n,ell)=(3,21,7)`, `N=42`, degree cap 5,
ended after Spot reclaim with **zero of two requested completed target records**.
Job `20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f` records a Spot notice at
12:03:18 UTC, command exit **143**, and terminal status `interrupted` at
12:04:22 UTC. The saved target JSONL and error file are empty; output status
remains `gate_passed`. The attempt supplies **no registered degree-5 reading**
and establishes neither an exact degree nor a lower bound.

The [terminal preservation packet](e12/terminal-a-20261028/README.md) retains
the receipts, final output snapshot, producer gate records, hashes and canonical
S3 PUT/GET evidence. Its 14,706-byte canonical bundle has SHA-256
`3030d78c8e0a21bbd9c774294b338586cb58a136565e4fd35c5da4678bd09ba9`.
These are verified output files, not a solver-memory checkpoint. No closure
replay, independent algorithm validation, retry or deadline extension accompanies
this preservation. Earlier still-running records are historical; the requested
E12a degree determination and the general first-/last-fall questions remain open.
