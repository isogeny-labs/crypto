# Audit of *Alternatives to Semaev summation polynomials for prime-field ECDLP index calculus* (2026-11-04)

**Status.** Review + corrected experiments. No attack is claimed; no deployed curve is affected.
Everything below is a local run of 2026-11-04 on this machine; it carries **no replay certificate**
and should be read as unverified until re-run under the repository validation gate.
Prior art: the 27-page program document itself (not present in this checkout — see §8).

## 1. Verdict

* The calibration half of the document is **sound**: §6.1's Vieta conic identity, the discriminant
  identity, the 1/12 LLL threshold and the carry sizes all reproduce (§2).
* The **decision rule is wrong**: the oracle gate "e < 1 at m = 4" does not follow from the
  document's own cost model and is *satisfied by a generic meet-in-the-middle oracle* (§3).
  The correct gate is `s < m-3` for `T_dec ∝ B^s`, i.e. `e < (m-3)/4`.
* **Idea 10 is quantitatively dead**, not merely unpromising: measured 1 squarefree collision pair
  for all `B ≤ 2^16`, against a predicted `B^{1/2} = 256` (§4).
* **Ideas 2 and 3 are m = 2 objects** and therefore, by §2 of the document itself, cannot change the
  exponent even if they fully succeed. Idea 2 is already an `O(B)` algorithm and needs no lattice (§5);
  Idea 3's census is only tractable below `δ ≈ 1/12`, i.e. where LLL already succeeds (§6).
* Five of the eighteen directions (2, 3, 5, 7, 18) target arity ≤ 3; §2 says arity 3 ties only with an
  O(1) oracle. Effort should be redirected to m = 4 (S_5) and to the theory tracks (§7).

## 2. What reproduces (independent re-implementation)

| Claim | Document | This audit | Verdict |
|---|---|---|---|
| Vieta remainder `Q_R(e1,e2)` is a conic | total degree 2, 6 monomials | degree 0 in `x1`; total degree 2; monomials `e1², e1e2, e1, e2², e2, 1` | exact match |
| `disc_T S_3 = 16 f(x1) f(x2)` | §3.2 | symbolic difference 0; LC `(x1-x2)²` | exact |
| `x(P1±P2)` are the roots | §6.2 | both `S_3(x1,x2,·) ≡ 0 mod p` | pass |
| LLL 3-dim threshold ≈ 1/12 | §6.3 | crossover 0.09 → 0.10 at 72 and 96 bits (24/24 at δ≤0.08, 13/24 at 0.10, 4/24 at 0.12) | matches |
| carry ≈ `B²p` | §6.2 | 40/45/54/55/61/65 bits for B = 16…16384 at p≈2^41 (doc: 45/51/46/56/56/62) | same regime |

Scripts: `scripts/verify_core.py`, `scripts/lattice_check.py`, `scripts/carry_check.py`.
Figure: `figures/audit.svg` (left panel).

**Implementation note.** The planted vector is frequently a *scalar multiple* of the `S_3`
coefficient vector (gcd 4 when `x1-x2` is even), so a strict equality test under-reports recovery;
the audit compares projectively. A re-run of the document's §6.3 table should use the same test.

## 3. The gate (this is the load-bearing error)

The document's cost model is
`C = (m!/2^m)·p·T_dec/B^{m-1} + m·B²`.
Writing `T_dec ∝ B^s` and optimising in `B` gives `B^{m-s+1} ∝ p`, `C ≈ B²`, hence

> **win ⟺ s < m − 3**  (equivalently `T_dec < B^{m+1}/p`, with `B ≲ p^{1/4}`).

In exponent form, evaluated at the tie point `B = p^{1/4}`: `e < (m-3)/4`, i.e.
`e < 0` at m = 3, `e < 1/4` at m = 4, `e < 1/2` at m = 5.
This agrees with §2 of the document ("m = 4 needs `≪ p^{1/4}`", "m = 3 ties only with an O(1)
oracle") and contradicts §3.1/§5.4 ("e < 1 at m = 4", "e < 0.5 at m = 3").

**The stated gate admits a generic algorithm.** A 4-sum over a generic set of size `B` is found by
2+2 meet-in-the-middle in `T = B²` group operations:

| B | 49 | 84 | 147 | 256 | 446 | 776 |
|---|---|---|---|---|---|---|
| ops (full scan) | 702 | 1640 | 5256 | 15750 | 46010 | 131406 |

fitted `s = 1.93` on operations (`1.99` on wall time; dict-memory overhead inflates the tail).
At `B = p^{1/5}` (m = 4) this oracle has `e = log_p B² = 2/5 = 0.4 < 1`: it **passes the document's
gate** while its optimal total cost is `≈ p^{2/3}`, strictly worse than rho. The corrected gate
`s < m-3 = 1` rejects it, as it should. Any proposal scored with "e < 1 at m = 4" can therefore
"win" without beating rho.

Also fix the companion phrase "T ≪ p^{1/4} at B ≈ p^{1/5}": the two points are different.
At `B = p^{1/5}` the condition is `T < 1`; the `p^{1/4}` threshold sits at `B = p^{1/4}`.

Script: `experiments/mitm_oracle.py`, output `experiments/mitm_m4.json`.

## 4. Idea 10 (squarefree-part collisions) — measured dead

The document predicts `B^{1/2+o(1)}` colliding pairs `x1 < x2 ≤ B` with `sqf(x³+7)` equal, and
proposes finding "20 colliding pairs at B = 2^12".

Measured (exact squarefree kernels, `scripts/sqf_census.py`):

| B | 256 | 1024 | 4096 | 16384 | 65536 |
|---|---|---|---|---|---|
| pairs | 1 | 1 | 1 | 1 | 1 |
| predicted `B^{1/2}` | 16 | 32 | 64 | 128 | 256 |

The only pair in the whole range is `x = 2, 212` (kernel 15: `212³+7 = 15·797²`). The experiment
cannot be run as written.

Why the heuristic fails: `sqf(f(x)) = sqf(f(y))` means `f(x)f(y)` is a perfect square, i.e. an
integer point on `x³+7 = d·m²`. For each `d` this is a Mordell curve (finitely many integer points,
Siegel), and the birthday model over "random integers of size B³" gives `Σ_d C(k_d,2) ≈ (log B)/B`,
not `B^{1/2}`. This is the same global-lift/free-relation wall already recorded in
`RESEARCH_FREE_RELATIONS_CENSUS_20261012.md`; Idea 10 re-derives a closed lead with a wrong count.

## 5. Idea 2 (Vieta-conic sieve) — an O(B) algorithm already exists

`Q_R(e1,e2)` is quadratic in `e2`, so for each `e1 ∈ [1,2B]` there are at most two conic points:
enumerating all conic points with `e1 ≤ 2B` costs `O(B)` field operations, and the square test
`e1²-4e2 = u²` with `x1,x2 = (e1±u)/2 ∈ [1,B]` is `O(1)` per point. Implemented
(`scripts/vieta_conic_enum.py`): recovers planted 2-decompositions at `B = p^{1/2}` in `O(B)` time
(5/5 at p = 2^28, 84k `e1`-steps, 9.4 s). No lattice is involved and none is needed.

Two consequences:

1. The "B univariate Coppersmith calls" formulation is strictly worse than the direct test, and the
   substituted equation `Q_R(e1,(e1²-u²)/4)` is **quartic** in `e1`, not quadratic — univariate
   Coppersmith would give `e1 ≤ p^{1/4}`, not `p^{1/2}`.
2. The result is an `O(√p)` 2-decomposition oracle = the generic bound. By §2, m = 2 cannot beat rho
   even with a *free* oracle, so Idea 2 cannot change the exponent however it ends. It is a lemma
   ("the symmetric quotient does not lower the reach"), worth one paragraph, not a CPU-day.

## 6. Idea 3 (Γ₀(p) census) — feasible only below the LLL threshold

The box `|c_i| ≤ B^4` in 3 dimensions with `det L_R = p` contains `≈ 8B^{12}/p` lattice points:
`O(1)` at `δ = 1/12`, `p²` at `δ = 1/4`. Any exact census is therefore restricted to `δ ≲ 0.1`,
i.e. to the regime where LLL already recovers the form — it cannot probe `δ ≈ 1/5`.

Second correction: with `D = 16 f(x1) f(x2) > 0` the forms are **indefinite**, so SL₂(Z)-orbits are
infinite; "finitely many orbits" is false. What is finite is the set of Γ₀(p)-reduced representatives
of bounded size (equivalently, class-group cosets meeting the root constraint).

## 7. Recommended disposition

| # | Keep / change | Note |
|---|---|---|
| 1 | **keep**, fix justification | "elimination is a resultant ⇒ same reach" is not sound: reach is a property of the integral model and box, and the Vieta box (2B, B²) is graded. Still the right test. |
| 2 | **demote to lemma** | §5; measured `O(B)` already. |
| 3 | **rewrite / demote** | §6; census feasible only below 1/12. |
| 4 | keep | cheap; RR norm system. |
| 5 | demote | m = 2 in disguise; global-lift wall. |
| 6 | keep | cheapest lattice-relevant specialness test. |
| 7 | demote | tests m = 2, 3 only. |
| 8 | **keep (theory)** | negative certificate is a real deliverable. |
| 9 | keep, low priority | rank-deficient by construction. |
| 10 | **close** | §4; measured 1 pair ≤ 2^16. |
| 11 | keep (theory) | one cheap null. |
| 12 | conditional | only if Idea 1 finds a gap. |
| 13 | **keep (theory)** | lattice-query lower bound. |
| 14 | **keep (theory)** | Yokoyama for compositional bases. |
| 15 | **keep** | instance set delivered here (§8). |
| 16 | **keep, run first** | controls. |
| 17 | keep | carry-loss law. |
| 18 | demote | S_3; expect ≤ 1/6. |

Only Ideas 1, 4, 6, 15 address m = 4. Under the corrected gate, that is where the exponent is.

## 8. Delivered alongside this memo

* `experiments/m4_instances.py` → `experiments/m4_instances.jsonl` — frozen m = 4 (`S_5`) instance set,
  `p ∈ {2^40, 2^48}`, `B = ⌊p^{1/5}⌋`, 20 planted instances per prime, witness `x1..x4`, `x_R`,
  group-law check, and `S_5 = 0` verified by resultant recursion for the first three of each size.
  Note the density: at this `B` a random target decomposes with probability `B⁴/p = 1/B`
  (1/256 at 40 bits), so unplanted targets are negative controls, not instances, and any measured
  exponent must divide by the density.
* `experiments/mitm_oracle.py` → `experiments/mitm_m4.json` — generic oracle baseline (§3).
* `scripts/` — reproduction of §6.1, §6.2, §6.3, Idea 2 and Idea 10.
* `PATCHES.md` — exact replacement text for the document.

Plus, from the follow-up run of the same day:

* `experiments/idea1_lattice.py` → `experiments/idea1_lattice.jsonl` — Idea 1 harness v1 (shift
  lattices, fplll, deterministic op counts), validated by a univariate positive control (recovers the
  degree-2 root to e = 0.48, fails at 0.52). The diamond family reaches δ ≈ 0.13–0.14 at m = 2,
  matching the document's Probe B/C band; the sparse family stays at 0–1/6. Two implementation
  fixes: reject multiples of f (classic false positive) and take resultants over Z, not mod p.
  j = 0 and generic j are identical within noise, as predicted from the identical 9-monomial support.
* `experiments/reach_bound.py` — Howgrave-Graham/LLL certificate. Validated on the univariate case
  (certified reach 0.295→0.440 for m=1..6, approaching 1/2); for S_3 it certifies nothing at m ≤ 4
  (bound above p^m by 12–25 bits), while the measured reach is 0.14–0.16. The 1/6 claim is an
  m→∞ limit, not a runnable-m certificate.
* `leads/` — five lead-atlas entries (Idea 1, 2, 3, 10, 15).
* `REPORT.md` / `REPORT.pdf` — the consolidated report with §8 instruction-count accounting.

The document itself is **not present in this checkout** (`crypto` HEAD is 2026-10-12; the referenced
`lever2_*`, `prime_ic_lead_review_20261031`, `research/prime_digit_presentation_20261029` directories
are absent), so this directory is untracked and the patches are supplied as text rather than applied.

## 9. Reproduction

```
python3 scripts/verify_core.py          # Vieta conic + discriminant + root check
python3 scripts/lattice_check.py        # §6.3 LLL table (72/96 bits, 24 trials/cell)
python3 scripts/carry_check.py          # §6.2 carry sizes
python3 scripts/sqf_census.py           # Idea 10 census
python3 scripts/vieta_conic_enum.py     # Idea 2 O(B) enumeration
python3 experiments/m4_instances.py experiments/m4_instances.jsonl
python3 experiments/mitm_oracle.py experiments/mitm_m4.json
python3 experiments/make_figure.py
```

Environment: Python 3.9.10, sympy 1.14.0, matplotlib 3.9.4. `fpylll`, `flatter`, `cuso` and `SCIP`
are **not** installed here, so the Idea 1 certified-reach audit (the one experiment that would settle
the presentation question) is not run in this directory; `msolve`, `fplll` and `gp` are present.
