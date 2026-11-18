# B0 — the point-decomposition SAT baseline, re-measured, and a new solver that reaches the pair count

**Status: rows in progress.** Every number below is `[pcc:unverified]` until the
rows, the validator replay and the pcc claim land; the sections marked *(rows)*
are filled by `analyze.py` from `rows/l*.jsonl` and are empty until then.
Protocol: [`PROTOCOL.md`](PROTOCOL.md), [`protocol.json`](protocol.json), frozen at
commit `dec928b6` (`FREEZE.sha256`), amended once (`AM1`, `FREEZE-am1.sha256`).
Plan: [`../PLAN.md`](../PLAN.md). No `S` value, leaderboard row or rho
comparison is produced here (protocol "inadmissible").

## 1. Question and metric

For `m = 3` point decomposition on `y² + xy = x³ + x² + 1` over `F_{2^n}`,
`n = 3ℓ − 1`, subspace factor base of dimension ℓ: how much work does each SAT
arm spend relative to the exact pairs-and-solve oracle,

```
R  = conflicts / T_enum,      T_enum = 2^ℓ (2^ℓ + 1) / 2   (pairs X₁ ≤ X₂ the oracle scans on a refutation)
R₃ = conflicts / C(2^ℓ, 3)                                 (the survey's "one conflict per candidate triple")
```

medians over 16 uniform targets per rung (random curve abscissa outside the
base), with 4 planted targets as positive control and an A/A arm as the noise
floor. Every SAT answer is checked: a model must decode to abscissae in the
subspace with `S′₄ = 0`; a refutation must match the oracle.

## 2. Arms

| arm | what | role |
|:--|:--|:--|
| oracle | `SubspaceOracle::low_order(ℓ,1).decompose` | verdict, `T_enum`; `R ≡ 1` |
| native | `sat.rs` CDCL + Gauss–Jordan XOR on symmetrised `S′₄` (`semaev_sat.rs`, lex symmetry) | incumbent |
| native-aa | native, scrambled saved phases | A/A |
| cms | CryptoMiniSat 5.8.0 (crates.io; `third_party/cryptominisat-rs-patches`) on the exported XOR-DIMACS, 1 thread | external XOR-aware reference (ℓ ≤ 7) |
| cadical | CaDiCaL 3.0.1 (`cadical-sys`) on the exported CNF | plain-CDCL control (ℓ ≤ 7) |
| chain-p12 | repository chained-S₃ encoder (`binary_semaev_chain_sat.rs`), AND-gate products, `x₁ ≤ x₂ ≤ x₃`, branch priority on `x₁, x₂` | existing chain implementation (ℓ = 7) |
| **lin** | new solver `examples/pdp_lin_cdcl.rs` (§3), chained S₃, `x₁ ≤ x₂` | candidate |
| lin-nolex | lin without the lex constraint | pair-ordering control |

Exported instances: `inst/l<ℓ>/t<idx>.xnf` (XOR-DIMACS of the native encoding,
tracked) and `t<idx>.cnf` (Tseitin CNF for CaDiCaL, 0.9–5 MB each, not tracked;
regenerated deterministically from the seed by the frozen binary, or fetched from
the artifact store where the repository's workflow moved one).

Instrument: `examples/b0_pdp_sat_baseline.rs`; external arms through
`tools/b0-ext-solvers`; rows by `run_rows.sh` under the bench record
`benchmarks/records/20261115T0359Z-b0-pdp-sat-baseline.json`; binaries' sha256 in
`rows/binaries.sha256`.

## 3. The new solver: CDCL with field-product gates

Why the incumbent spends one conflict per triple: the oracle refutes `2^ℓ`
triples per `(x₁, x₂)` pair by root-finding a univariate polynomial over
`F_{2^n}`, while a CDCL solver on the Weil-descended system assigns all three
summands before the descended equations bind (survey §3). The algebra that the
oracle uses is linear in disguise: in a chained encoding
`S₃(x₁,x₂,t) = 0`, `S₃(t,x₃,x_R) = 0` with
`S₃(a,b,c) = (ab + ac + bc)² + abc + 1`, every monomial is of degree ≤ 1 in `t`
once `x₁, x₂` are assigned (squaring is `F₂`-linear in characteristic 2), and of
degree ≤ 1 in `x₃` once `t` is assigned. The repository's chain encoder writes
products as AND gates, so this structure is invisible to its XOR engine; at
ℓ = 5 it spends 160 conflicts per pair.

`LinCdcl` keeps clauses and XOR rows like any CDCL+GJE solver and adds
**field-product gates** `z = x·y`, each operand an `n`-bit block. A gate is inert
while both operands are open. When one operand block is fully assigned, the gate
injects `n` XOR rows `z_k ⊕ Σ_j M(x)_{kj} y_j = 0` (the multiplication-by-`x`
matrix) at the current decision level, guarded by the operand's literals, and
withdraws them on backtrack. With branch priority on the `x₁, x₂` bits the solver
decides a pair, the gates `p = x₁x₂`, `v = (x₁⊕x₂)·t`, `w = p·t` linearise, the
first `S₃` becomes a linear system in `t` (a quadratic over `F_{2^n}`: 0 or 2
roots, so at most one free bit), then `a = t·x₃` linearises and the second `S₃`
determines `x₃`, whose domain units accept or refute it. Conflict explanations
are exact: the union of assigned literals in the combined original rows plus the
guards. Because the system has a few hundred variables, the engine simply
re-eliminates all active rows at each propagation fixpoint; that is deliberately
simple, not fast.

Predicted count: one conflict per pair with no `t`-root, two per pair with two
roots (each root's `x₃` fails separately), so `R → ≈ 1.5` as ℓ grows, i.e.
`2^ℓ/4.5` fewer conflicts than the incumbent.

## 4. Instrument checks (not rows; logged 2026-11-15)

| ℓ | n | arm | median R | median R₃ | agree | note |
|--:|--:|:--|--:|--:|:--|:--|
| 5 | 14 | native | 10.88 | 1.158 | 20/20 | `2^ℓ/3 = 10.7` |
| 5 | 14 | native-aa | 10.76 | 1.145 | 20/20 | |
| 5 | 14 | cadical | 17.9 | 1.91 | 20/20 | |
| 5 | 14 | cms | 36.4 | 3.87 | 20/20 | per-target 2×–20× |
| 5 | 14 | chain-p12 | 160.5 | 17.1 | 20/20 | chain (all summand bits) 262 |
| 5 | 14 | **lin** | **1.203** | 0.128 | 20/20 | lin-nolex 2.138 |
| 6 | 17 | native | 21.94 | 1.095 | 20/20 | A/A spread 0.4 % |
| 6 | 17 | cadical | — | 1.661 | 20/20 | |
| 6 | 17 | cms | — | 3.395 | 20/20 | 1.10–11.35 |
| 6 | 17 | **lin** | **1.465** | 0.073 | 20/20 | lin-nolex 2.431 |
| 7 | 20 | native | 43.93 | 1.062 | 20/20 | median 32.9 s per refutation |
| 7 | 20 | **lin** | **1.269** | 0.030 | 20/20 | 3.0 s per refutation; lin-nolex 2.252 |

Every uniform refutation in every arm has `R ≥ 1`: no learnt clause excludes
more than one pair in effect.

## 5. Rows (ℓ = 7…10) *(rows)*

Filled from `table.md` / `ANALYSIS.json` when the rungs finish. Claims B1–B5 are
read there against the protocol's intervals.

## 6. Amortisation across targets (candidate 3.3, exploratory, dev binary)

One parametrised solver per rung, `x_R` entering as assumptions, 64 uniform
targets (seed 777):

| ℓ | FRESH median R | REUSE median R | quartile medians of REUSE R | cumulative REUSE/FRESH |
|--:|--:|--:|:--|--:|
| 5 | 1.217 | 0.521 | 0.606, 0.524, 0.509, 0.520 | 0.457 |
| 6 | 1.465 | 1.001 | 1.102, 1.025, 1.003, 0.998 | 0.717 |
| 7 | 1.269 | 0.567 | 0.647, 0.581, 0.572, 0.565 | 0.457 |
| 8 | 1.493 | 1.047 (2nd half; 28 of 64 targets before the session's 2 h limit) | first half 1.135, last four 1.044, 1.037, 1.037, 1.035 | 0.773 |

Stage split (24 targets, ℓ = 7): FRESH spends 5918 t-stage + 4556 x₃-stage
conflicts per refutation; REUSE's second half spends 434 + 4556. The t-stage
(root-less pairs) is what amortises; the x₃-stage (one conflict per (pair,
t-root), target-dependent) does not, so the plateau is x₃-stage/T_enum: 0.98 at
n = 17 and 0.55 at n = 20. Forgetting learnt clauses that mention `x_R` at each
new target keeps the count (R 0.564 vs 0.566) with a 10× smaller clause database
(26.5k vs 260k live) and half the wall (5.6 s vs 11.3 s; FRESH 4.6 s).

Candidate 3.3's rule (REUSE/FRESH < 0.5 at ℓ = 8 and 9) is not met: the ratio
is 0.70 at ℓ = 8 and the plateau is the x₃-stage share (0.98, 0.55, 1.04 of the
pair count at n = 17, 20, 23), so the candidate is closed as "bounded
amortisation of the root-less pairs".

Reading: REUSE learns the target-independent fact "this pair has no `t`-root"
(about half the pairs, one conflict each in FRESH) and plateaus at the remaining
one conflict per `(pair, root)`, whose `x₃` check depends on the target. It is a
lazily learnt table of root-less pairs, the amortisation the survey's
preprocessing bound allows, not a new mechanism.

## 7. Why the pair count is the ceiling of this route

Every ordering of the three summands costs `2^{2ℓ}` leaves: fixing one summand
leaves a system bilinear in the other two, and only fixing two makes the rest
linear. The repository's pair table reaches `2^ℓ` per target by an **inverse
lookup** `t ↦ (x₁, x₂)` over a `2^{2ℓ}` table built once. CDCL clause
propagation implements forward implications (`x₁, x₂ ⇒ t`), not inverse lookups;
expressing "`t` is not in the image" needs `2^n` complement clauses, far more
than the `2^{2ℓ}` table entries. So a CDCL solver on this formulation cannot get
below the pair count per target by learning, and `lin` is at that count. The
honest statement of the SAT route after this work is therefore: a measured
`2^ℓ/3`-fold conflict reduction over every existing SAT implementation here,
parity with the pairs-and-solve oracle in counts, a factor `2^ℓ` behind the pair
table, and the distance to rho unchanged in exponent.

## 7a. Where the wall time goes, and what it means for "speed"

One ℓ = 7 refutation with `lin` (dev binary, shared host, descriptive only):
10,474 conflicts, 44,063 XOR-engine calls (4.2 per conflict), 2.98 s; 79 % of
the wall is the XOR engine (53 µs per call: a full re-elimination of ≈ 250 rows
over ≈ 190 variables), 15 % clause propagation, 2 % analysis; 285 µs per
conflict. An incremental engine (RREF maintained across calls, re-pivoting on
assignment, as CryptoMiniSat's Gauss–Jordan does) would remove most of the 79 %
and is the next engineering step. It would not change the conclusion on speed:
the pairs-and-solve oracle refutes a pair in about 10 ns of carry-less
multiplication, so the solver is four orders of magnitude slower per pair, and
even a 10× engine would leave three. The new solver's value is in the counted
metric (it reaches the pair count, 2^ℓ/3 ahead of every existing SAT
implementation here), in the REUSE amortisation (0.55–1.0 of the pair count
per target), and in being a general CDCL solver whose every SAT answer passes the `S′₄` check — not in
wall-clock parity with a specialised enumerator, and not in
distance to rho, which this route does not change in exponent.

## 8. Sources checked

KB unavailable in this container (no `research-kb` MCP or CLI; lead atlas
vault-encrypted). Read directly:
`research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` §1–3, §6–7;
`research/notes/index-calculus/RESEARCH_SAT_SEMAEV.md`;
`research/sat_pdp_pricing_band_20261111/README.md`;
`research/pdp_followups_20261010/blocked/PROTOCOLS.md`;
`research/sat_ladder_scaling_20261110/README.md`;
`research/pdp_ml_sat_ceiling_20261027/README.md`;
`src/cryptanalysis/{semaev_sat,sat,semaev_decomp,ic_corpus,binary_semaev_chain_sat}.rs`.
