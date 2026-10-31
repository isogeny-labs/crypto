# Protocol: phase-aware Frobenius summation, paired fixed-budget comparison

Registered 2026-10-11 before the target manifest was frozen and before any
solver run. Stage diagnostic on toy fields. `S`, end-to-end cost and speedup
stay unset; no ECDLP speedup is claimed at any size.

## Question

The 2026-10-09 retrospective reports an "implicit phase-aware" encoding at
7.14 s per independent relation rank against 15.06 s for its control, from
2 against 1 independent relations: a sample too small to read. The draft
matrix in PR #1630 says of itself that it "is not a statistically controlled
comparison of identical target instances between phase-on and phase-off".
This protocol runs that controlled comparison on the harness in
[`..`](../README.md): same frozen targets, same per-attempt budget, same
formulations, one solver thread, interleaved order, every unsuccessful
search counted, every relation verified on the curve.

## Hypotheses (falsifiable)

- **H1 (mechanism).** With one-hot Frobenius phases (arm B) each of the
  `|reps|` orbit representatives stands for `n` admissible x-coordinates, so
  at equal matrix width the effective factor base is `n` times larger. On
  identical targets arm B should therefore (i) decompose targets that arm A
  proves UNSAT and (ii) spend fewer wall and CPU seconds per newly
  independent verified relation, charging every attempt.
- **H0.** Arm B's larger circuit costs more than the extra decomposable
  targets return: seconds per newly independent relation are equal or
  higher for arm B.

Either outcome is a result. H1 is read as supported only if, for the same
formulation, arm B's seconds per newly independent relation (all attempts
charged) are below arm A's on both wall and CPU clocks and every relation
passes both checks below.

## Frozen inputs

- Curve `E0: y^2 + xy = x^3 + 1` over `F_{2^13}`, modulus
  `X^13 + X^4 + X^3 + X + 1`, odd subgroup `H = 4E` of order `r = 2003`
  (prime). Factor space: the seeded `s = 4` subspace of `ker Tr` drawn by
  `run.factor_space` with `random.Random(20260930 * 1000 + 13)`, the same
  space as the recorded tables in `../README.md` (5 representatives, 65
  admissible x-coordinates once phases are free).
- Target seed `20261011`. Three sets of 4 targets each, full coordinates in
  `manifest.json`:
  - `a*`: planted, 4 distinct representatives, random signs, random
    Frobenius phase per summand (redrawn if all four phases are 0);
  - `b*`: planted, 4 distinct representatives, random signs, all phases 0;
  - `c*`: uniform in `H \ {O}` as `[k]g`, `g = 4P` for a random point `P`.
- Construction witnesses, brute-force logs and brute-force decomposability
  go to `witness_sealed.json`. The solver arms never read that file.
- `manifest.json` is written once by `freeze_targets.py`; its SHA-256 is
  recorded in `SHA256SUMS` and in every receipt header. It is not
  regenerated after the first solver run.

## Arms and execution

| | arm A | arm B |
|---|---|---|
| unknowns per summand | `s` coordinate bits | `s` coordinate bits + `n` one-hot phase bits |
| `x_i` | representative `v_i` | `v_i^(2^k_i)` |
| matrix columns | 5 representatives | 5 representatives |
| effective factor base | 5 x-coordinates | 65 x-coordinates |

- Formulations `subgroup_xor` and `line_subgroup` (the two fastest recorded
  at n = 13), unchanged from `../encode.py` and `../run.py`.
- Budget 120 CPU seconds per attempt (pycryptosat `time_limit` is a CPU
  clock, measured in this session: 2.04 s CPU against 3.49 s wall under
  load). After a rejected tuple the solver is rebuilt with the remaining
  budget so no attempt exceeds 120 CPU seconds of solving.
- One process, one solver thread, attempts strictly sequential.
- Order: formulation-major; within a formulation the targets in manifest
  order; pair `i` runs `A, B` when `i` is even and `B, A` when odd (ABBA).
- A `--cpu-cap` of 4500 CPU seconds stops the run at a pair boundary;
  attempts not run are written as `skipped` receipts, never silently
  dropped. If the second formulation does not fit, the result says so.
- Every attempt writes one JSON line: arm, form, target, status (`verified`,
  `unsat`, `timeout`, `skipped`), build/load/solve/verify wall and CPU
  seconds, rejected tuples, found signed points, circuit size, load
  average, RSS. The header carries platform, Python and pycryptosat
  versions, source SHA-256s, manifest SHA-256 and the exact command.

## Verification (fail closed)

1. Every solver answer is checked on the curve by `encode.verify` before
   it is called a decomposition (as in the parent harness).
2. `rank.py` recomputes each relation independently: it maps every summand
   `x_i` to `(representative j, phase k_i)`, fixes the sign against
   `pi^k_i(V_j)` with `V_j = lift(v_j)`, forms the coefficient
   `±lambda^k_i mod r` with `lambda` the root of `lambda^2 + lambda + 2 = 0
   mod r` satisfying `pi(G) = [lambda]G` on the curve, and checks both
   `sum ±pi^k_i(V_j) = R` on the curve and
   `sum coeff_j * log V_j = log R (mod r)` against brute-force logs.
3. Every `unsat` is compared with brute-force decomposability from the
   sealed file after the run. Any disagreement is a defect that voids the
   table until explained.

## Metrics and accounting

- **Primary:** wall seconds per newly independent verified relation, per
  arm and formulation, charging every attempt of that arm (verified,
  unsat, timeout). CPU seconds per newly independent relation reported
  beside it because the host is shared.
- **Secondary:** verified relations per attempt, seconds per attempt,
  verified / attempts, rejected tuples, UNSAT count, timeouts.
- **Ceiling stated in advance:** the relation matrix has 5 columns, so
  rank saturates at 5 in both arms. Rank growth is reported as cumulative
  rank against cumulative seconds; relations found after saturation are
  counted as verified relations but not as independent ones.
- Cost accounting is exclusive and complete: build, load, solve, decode,
  verification and blocking are all inside an attempt's total. Manifest
  construction is a one-off setup cost, reported separately.

## Stop conditions

- CPU cap 4500 s for the n = 13 block; total session compute under ~90
  CPU minutes including tests and figures.
- Any relation failing check 2, or any UNSAT contradicted by check 3,
  stops the analysis; the defect is reported.
- n = 19 (2 phase-0 planted, 2 random-phase planted, 300 s, one
  formulation, both arms) runs only if the n = 13 block completes and the
  session has budget left; otherwise the exact command is recorded as the
  next step.

## What this cannot show

A stage ratio at n = 13 on 5 representatives says nothing about end-to-end
cost at m = 51, 53, 83 or 131. Any statement about those sizes in
`RESULTS.md` is labelled known fact (circuit sizes, recorded timeouts) or
requirement, never projection-as-result.

---

## Addendum 2026-10-26 (before the manifest was frozen and before any solver run)

Everything above is the text registered on 2026-10-11 and is unchanged
(commit `db05afd8b`). This addendum changes no hypothesis, input, arm,
budget, metric or stop condition. It records circumstances and
clarifications.

1. **Resumed after a session rate limit.** The branch is based on
   `bbd330a9d` (main of 2026-10-11), about 15 days stale; no fetch was
   possible. PR #1630 was not re-read, so its status today is unknown.
   GitHub access is gone (raw git 403, API 404), so nothing is pushed and no
   pull request exists. The PR body is drafted outside the repository;
   publication is deferred until access returns.
2. **Host changed on restart.** Registration saw a Xeon at 2.80 GHz; the
   resumed session reports 2.10 GHz. Every receipt records the host it ran
   on. Wall and CPU seconds are comparable only within this run, and not with
   the 2026-09-30 tables.
3. **Rank definition.** The relation matrix has one column per orbit
   representative (5) and no column for the target. A target column would
   make every relation trivially independent because each target is distinct.
   Consequences: rank is capped at 5 per arm; the targets here carry no
   `[a]G + [b]Q` structure, so no unknown-`Q` column exists. Each row is
   checked against the target by a second, log-based test (`rank.py`), which
   is where the target enters. Order for "newly independent" is receipt
   order within an arm.
4. **Budget clock.** `pycryptosat` `time_limit` is a CPU clock measured per
   `solve()` call (2.04 s CPU for a 2.0 s limit while wall read 3.49 s; each
   later call on the same solver got a fresh 2 s). To keep the 120 s budget
   per attempt rather than per call, a rejected tuple triggers a rebuilt
   solver, with all blocking clauses re-added, limited to the remaining
   budget. Rebuilds are counted in the receipts. The earlier harness did not
   do this, so its totals after a rejection can exceed the stated budget.
5. **Target streams.** Each set uses `random.Random(target_seed*10000 +
   n*10 + i)`, i = 1, 2, 3 for a, b, c, as stated in `freeze_targets.py`
   and the manifest. No target is shared between sets.
6. **Graph and PDF.** The graph is SVG (editable text) with its generating
   script. No PDF engine is installed (`pandoc` is present; `typst`,
   `pdflatex`, `xelatex`, `wkhtmltopdf` and `weasyprint` are not), so
   `report.pdf` is built with matplotlib `PdfPages`.
7. **Language rule.** `AGENTS.md` ("Implementation language: no Python")
   says new research comparisons should use native code, and that Python
   harnesses are legacy. The brief for this lead named the existing Python
   harness as the code to build on, so this study extends it and keeps its
   legacy provenance in plain view: the SAT backend is native CryptoMiniSat,
   but circuit construction, orchestration, verification and the rank code
   are Python. Consequently no performance claim is made. A matched native
   rerun is a prerequisite for any such claim and for any extension of this
   harness to larger fields.
8. **Disk.** The worktree is a sparse checkout (cone mode) of the harness
   directories only; the full tree did not fit beside the other workers.

---

## Addendum 2 2026-10-26 (after the run)

The registered text and addendum 1 are unchanged. Facts since addendum 1:

1. The manifest was frozen before any solver run (commit `54dcf4ddb`, SHA-256
   `8cb7f7989e5ea35656b34e7ba8bf7104e0c081277b8dbbd6b0f7507369b01231`); the harness
   code was committed (`2ea85cfb0`) before the run and was not edited afterwards except
   that `verify.py` gained path arguments and `summarize.py` was added; neither is in the
   receipts' hashed source list.
2. The run executed exactly as registered: `python3 run_paired.py --out
   results/paired_n13.jsonl --budget 120 --forms subgroup_xor,line_subgroup --cpu-cap 4500`.
   All 48 attempts ran, the CPU cap was not reached, there were 0 timeouts, 0 skipped
   attempts and 0 rejected tuples.
3. Both forms fit, so the "if two forms do not fit" fallback was not used.
4. The coordinator then directed that no further Python research compute be run, citing
   `AGENTS.md` "Implementation language: no Python". The optional n = 19 block was
   therefore not run, no `report.pdf` was built, and no repeat trial was taken. One
   `verify.py` pass and one `summarize.py --no-pdf` pass over the finished receipts were
   run as the wind-down. The results are labelled provisional and not for merge, and a
   matched native re-measurement is required before any claim.
