# semaev_alternatives_audit_20261104

Audit of the 2026-11-04 program document *"Alternatives to Semaev summation polynomials for
prime-field ECDLP index calculus"*. Review + corrected experiments; no attack claimed, no deployed
curve affected.

**Headline.** The calibration (§6) reproduces. The oracle gate does not: the document's
"e < 1 at m = 4" is not implied by its own cost model, and a generic 2+2 meet-in-the-middle
oracle passes it (measured `s = 1.93` on `T = B^s`, `e = 0.4` at `B = p^{1/5}`) while costing
`p^{2/3}`, worse than rho. Correct gate: `s < m-3`, i.e. `e < (m-3)/4`. Idea 10 is measured dead
(1 squarefree collision for all `B ≤ 2^16`). Ideas 2 and 3 are m = 2 objects and cannot change the
exponent; Idea 2 is already `O(B)`.

## Contents

| Path | What |
|---|---|
| `REPORT.md` / `REPORT.pdf` | the audit report as delivered (memo + measurements + accounting + PROOFS appendix) |
| `REVIEW.md` | the memo: reproduced claims, corrections, disposition of the 18 ideas |
| `PATCHES.md` | verbatim before/after text for the document (P1–P8) |
| `apply_patches.py` | applies PATCHES.md to a copy of the document (whitespace-insensitive, fail-closed) |
| `leads/` | five drop-in lead-atlas entries (Idea 1, 2, 3, 10, 15) |
| `experiments/idea1_lattice.py` → `idea1_lattice.jsonl` | Idea 1 harness v1 (shift lattices, fplll, op counts); 40 cells, m = 2 |
| `experiments/control_univariate.py` → `control_univariate.jsonl` | positive control: degree-2 Coppersmith reaches e = 0.48, fails at 0.52 |
| `experiments/reach_volume.py` → `reach_volume.json` | lattice-volume diagnostic per shift family |
| `experiments/idea12_shift_search.py` → `idea12_shift_search*.json` | random shift-family search (Idea 12) |
| `experiments/idea1_cuso_bounds.py`, `experiments/cuso_presentations.py` → `cuso_*.json` | certified reach via cuso (1/6, 1/6, 1/24) |
| `lib/` | `ec.py` (curve ops + counters), `lattice.py` (shift lattices, fplll, recovery), `accounting.py` (GAE) |
| `tests/` | `PYTHONPATH=. pytest tests -q` — 8 tests |
| `lean/Routes.lean`, `lean/Roots.lean` | seven Lean 4 theorems: arity 2/3, the m=4/5 gate, the MITM route, and the two-roots bound (`lean Routes.lean && lean Roots.lean`) |
| `PROOFS.md` | strict ledger: **only Lean theorems count as "ruled out"**; everything else is marked measured/argued/open |
| `experiments/m4_instances.py` → `m4_instances.jsonl` | frozen m = 4 (`S_5`) instance set, p ∈ {2^40, 2^48}, B = ⌊p^{1/5}⌋, 20 planted instances each, witnesses + `S_5 = 0` checks |
| `experiments/mitm_oracle.py` → `mitm_m4.json` | generic m = 4 oracle baseline (`T = B^s`, s ≈ 2) |
| `experiments/make_figure.py` → `figures/audit.svg` | LLL recovery curve + MITM cost |
| `scripts/*.py` | independent reproduction of §6.1, §6.2, §6.3, Idea 2 and Idea 10 |

## Reproduce

```
python3 scripts/verify_core.py
python3 scripts/lattice_check.py
python3 scripts/carry_check.py
python3 scripts/sqf_census.py
python3 scripts/vieta_conic_enum.py
python3 experiments/m4_instances.py experiments/m4_instances.jsonl
python3 experiments/mitm_oracle.py experiments/mitm_m4.json
python3 experiments/make_figure.py
```

Environment: Python 3.9.10, sympy 1.14.0, matplotlib 3.9.4.

## Status / limits

* No replay certificate. Numbers are a single local run (2026-11-04); treat as unverified until
  re-run under the repo validation gate.
* The program document itself is **not in this checkout** (`crypto` HEAD = 2026-10-12; the referenced
  `lever2_*`, `prime_ic_lead_review_20261031`, `prime_digit_presentation_20261029` trees are
  absent), so patches are text and this directory is untracked.
* `fpylll`, `flatter`, `cuso`, `SCIP` are not installed, so the Idea 1 certified-reach audit — the
  one experiment that settles whether presentations differ — is **not** run here. `msolve`, `fplll`
  and `gp` are available.
* The m = 4 instance set uses `#E(F_p) ≈ p` for the matched-rho reference; a real run should use
  the exact order (SEA) per curve.
* At `B = p^{1/5}` a random target decomposes with probability `1/B`; every instance is planted.
  Unplanted targets are negative controls, and any measured exponent must correct for the density.

## Git

Committed on branch `semaev-alternatives-audit-20261104` (see `git log`). Not pushed and no PR:
this checkout's `origin/main` is 21,293 commits ahead of its HEAD (2026-10-12), so a PR from here
would not be reviewable. Transplant this directory into a current clone before opening one.
A stale `.git/index.lock` from 2026-10-24 was moved aside as `.git/index.lock.stale-20261024`.
