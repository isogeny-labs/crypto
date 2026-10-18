# Results: fall-degree experiments E1, E1b, E2 (`m = 2`)

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

# Results: E5, E3, E4 and E6 (2026-10-12)

These results come from the native engine and the amendments registered before each run
(Amendments 2–6 of `PREREGISTRATION.md`).
- Run report, graph and PDF: [reports/run-20261012/](reports/run-20261012/).
- Lean scoring: [lean/](lean/).
- Records: `e3/`, `e4/`, `e5/`, `e6/`.
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

- **`lean/FallDegree/Verdict.lean`**, kernel `decide` over 190 rows, proves
  `report rows = ⟨p3a, p3b, p4a, p4b, bounded, semaevSpecific⟩ = all true`. The 190 rows are
  generated from `e3/`, `e4/` and `e6/` by `examples/fall_degree_lean_data.rs`. Changing one
  verdict fails the build.
- **Disclosure.** `Rule.lean` transcribes the prose rules of the pre-registration and
  Amendments 2–6. The transcription was written after part of the data had been seen. The
  prose rules it encodes were not.
- **cairn.** The result objective `sha256:fc828009…` is the same theorem, over `Rule.lean`
  and `Data.lean`. cairn's Lean jail accepted it. `CAIRN_EPOCH_SECONDS=5 cairn --log
  cairn/log.jsonl audit` reports "chain intact".

## What this does and does not show

- **It shows** the following, at toy size:
  - the mutant-closure (polynomial-calculus) degree of these systems is 4;
  - the degree the repository's plain towers need is not intrinsic;
  - a degree-4 closure decides cells the plain tower could not build on this host.
    `m = 4` (9, 3) takes about 100 s with 31,931 columns, against the plain tower's
    degree-7 matrix of 2.1M × 2.8M.
- **It does not show** that `d_F` is bounded. HKYY's conditional says it cannot stay
  bounded for `m ≈ n^{1/3}` unless binary ECDLP is subexponential. Here `m ≤ 5` and
  `N ≤ 42`.
- **Conditional model** (`e6/crossover_model.md`, `examples/fall_degree_crossover.rs`). If
  `d_F = 4` held at every size, and one dense elimination in `V_{F,4}` cost `M(4)^ω`:
  - IC would beat rho only from `n ≈ 283` (ω = 2.81) or `n ≈ 233` (ω = 2.37);
  - ECC2K-130 stays above rho by 51 or 36 bits.

  That gives the sizes where the bounded-`d_F` question would start to matter for
  security: the NIST binary curves K-233 and K-283 and up. It does not say that they are
  at risk.
- **Next decisive measurement:** `d_F` at `m ≥ 5` with ℓ ≥ 3, and `N` ≈ 60–100. That is
  `M(4)` ≈ 0.5–4M columns, so it needs a sparse closure engine or a large host.
