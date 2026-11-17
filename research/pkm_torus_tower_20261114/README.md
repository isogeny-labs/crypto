# PKM-2016 "finding good maps", revisited: torus Kummer towers

**2026-11-14 · stage diagnostic of the solver axis (AGENTS.md §2) · every number
`[pcc:unverified]` · no pipeline priced, no `S`, no leaderboard row.**
Protocol: [`PROTOCOL.md`](PROTOCOL.md) (sha256 `57c95f99…`, frozen 19:25 UTC
before any closure run); amendments: [`AMENDMENTS.md`](AMENDMENTS.md).
Lead: `LEAD-ICP-pkm-tower-algebraic-oracle`. Graph: [`figures/d_vs_n.svg`](figures/d_vs_n.svg).
Report PDF: [`report.pdf`](report.pdf).

## 0. The question asked and the one-paragraph answer

The PKC-2016 slides (Petit–Kosters–Messeng, "Algebraic approaches for the ECDLP
over prime fields") give two ways to find the low-degree maps `L_j` whose
composite cuts out the factor base `V = {x ∈ F_p : L(x) = 0}`, `|V| ≈ p^{1/m}`:
(14) `p − 1 = S·N'` with `S` smooth, and (15) an auxiliary curve `E'` with
`#E'(F_p) = S·N'`, `L_j` the `x`-part of a chain of isogenies with kernel `G`,
`|G| = S`. This repository has taken both far past the slides (§1). What had not
been done is to notice that they are **the same construction on two different
one-dimensional groups**, and to ask whether the solving degree, the quantity
that actually decides the lane, depends on which presentation is used.

Answer, at toy scale: (i) both methods are Kummer towers of a torus, `G_m` (order
`p − 1`) or the norm-one torus `T₂` (order `p + 1`); the supersingular `j = 1728`
isogeny chain the repository already found as a 7 ms shortcut at P-192 *is* the
`T₂` tower (verified numerically, §2). Since `2^{64} | p + 1` at P-192, `2^{96}` at
P-256 and `2^{521}` at P-521, **the auxiliary-curve search phase is never needed
for the NIST primes**; (ii) the raw polynomial-calculus solving degree is **not**
presentation-invariant (the frozen prediction P1 is refuted, replicated on a
second seed and a second prime): writing the tower in the natural rational
coordinate of the torus (`a ↦ (a² ∓ 1)/(2a)`) costs one degree from `t = 2` on,
and a polynomial presentation of the same set (the circle `u² + v² = 1` with the
stereographic link, arm C; or the Kummer tower with a Möbius link, arm M) costs
one degree from `t = 3` on; all presentations rejoin the Kummer value at the
largest cells (`m = 2, t = 6`; `m = 3, t = 3`), so the difference is a shift of
the staircase, not a different growth law (§3). (iii) None of this moves the
exponent: the staircase `D(m,t)` that closes the lane
(`research/prime_leads_chase_20261113/L1-pkm`) is reproduced, not changed. What
is gained is a removed phase (a constant: the aux-curve search was 0.8–25 % of
`S` in `research/pkm_isogeny_tower_ecbench_20261015`, and a derived `2^{74}` GAE at
P-192), the answer to the C6 question (the shortcut is usable, at a cost of at
most one degree in these cells), and a presentation rule for anyone who runs the
`T₂` tower: the circle, never the Möbius coordinate.

## 1. What the slides propose and what the repository already knows

| Slide | Idea | Repository status before this note (paths) | What remained |
|---|---|---|---|
| 12 | relation search = polynomial system `S_{m+1}` + chain `x_{i,j+1} = L_j(x_{i,j})` | the split `S3` chain with `m − 2` auxiliary unknowns, PC/F4 degrees certified for `m ≤ 8` (`research/prime_leads_chase_20261113/L1-pkm/results.csv`); symmetric presentation shifts `D` by one, not the law (`research/pkm_symmetric_tower_20261026`) | a solver whose cost grows slower than the Macaulay size at the measured `D` |
| 13 | precomputed maps serve any curve over the field | the isogeny base is an `ic.pipeline` plug-in independent of the target curve (`src/cryptanalysis/ic_framework/prime_tower.rs`) | — |
| 14 | `p − 1` smooth: `L_j(x) = x^{q_j}` | the Kummer tower `V = g·μ_{2^t}`, `2^t \| p − 1` (`research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`, rounds 1–7; L1) | primes with `v₂(p − 1) = 1` (all NIST primes) have no such tower |
| 15 | isogeny kernels on an auxiliary `E'`, `#E' = S·N'` | 2-Sylow halving search at P-192 (`research/pkm_isogeny_tower_p192_20261015`): 11 core-years for `t = 48` on ordinary curves, 7 ms on supersingular `y² = x³ + x`; search charged in ecbench (`research/pkm_isogeny_tower_ecbench_20261015`): 1.00–1.33× a plain base | "whether a supersingular `E'` is acceptable to PKM's relation phase is not tested" (that README, C6) |
| 16 | random curves vs CM to find `E'` | both measured/derived above; CM not needed once `v₂(p + 1)` is large | — |
| 17 | solving the system | F4, signature F4, exact PC closure, msolve; `D(8,3) ≥ 9` censored; cost-floor audit: the lane cannot beat rho even at `D(8,3) = 9` (`research/prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md`) | a positive sparse relation-finder panel with `γ < 3.5` (that protocol, A) |

So the open PKM items are the solver (not touched here) and the C6 question,
which this note answers.

## 2. One construction, two tori (derivation, checked numerically)

Let `p ≡ 3 (mod 4)`, `i² = −1` in `F_{p²}`.

- `G_m(F_p) = F_p^*`, order `p − 1`. Coordinate `z`; doubling `z ↦ z²`. In the
  Möbius coordinate `a = (z + 1)/(z − 1)` doubling is `σ(a) = (a² + 1)/(2a)`.
- `T₂(F_p) = {z ∈ F_{p²} : z^{p+1} = 1}`, order `p + 1`. It is `F_p`-rational
  through `z = (a + i)/(a − i)`, `a ∈ P¹(F_p)`; doubling is `τ(a) = (a² − 1)/(2a)`.
  Equivalently `T₂` is the circle `u² + v² = 1` (`z = u + iv`), doubling
  `(u, v) ↦ (u² − v², 2uv)`, and `a = v/(1 − u)` (stereographic projection).

A PKM factor base is the preimage under `t` doublings of a point, i.e. a coset
`z₀·μ_{2^t}` of the `2^t`-torsion, read in the `F_p`-coordinate: `|V| = 2^t`, each
`L_j` has degree 2, `Π deg L_j = |V|` as PKM requires, for either torus. `G_m`
needs `2^t | p − 1`; `T₂` needs `2^t | p + 1`.

The supersingular curve `E₀: y² = x³ + x` has `#E₀(F_p) = p + 1`, cyclic. The
Vélu 2-isogeny with kernel `(0,0)` is `x ↦ x₁ = (x² + 1)/x` onto `y² = x³ − 4x`,
then `x₁ ↦ x₂ = (x₁² − 4)/x₁` onto `y² = x³ + 16x ≅ E₀` (`x = 4u`). Identities:
`x₁ = 2σ(x)` and `x₂/4 = τ(x₁/2)`. So the `j = 1728` chain that
`pkm-aux-search --family supersingular-j1728` returns is the torus tower, up to
a scaling per level. `scripts/torus_check.py` verifies, with no library:

| check | result (`runs/torus_check.json`) |
|---|---|
| Vélu chain points land on both codomains and back on `E₀`; `x₁ = 2σ(x)`, `x₂/4 = τ(x₁/2)` | 200 random points at each of `p = 8191, 40963, 65519`-class primes and 20 at P-192: all pass |
| `σ`, `τ` are doublings in the stated coordinates; `z(a)^{p+1} = 1` for all `a` | 200 random `a` per prime, 20 at P-256: all pass |
| `|τ^{−t}(c)| = 2^t` exactly (exhaustive over `F_p`, `p = 8191`, 20 cosets per `t`) | `t = 1..6`: min = max = `2^t` |
| towers at the NIST primes, `t = 12`: `2^t` distinct `a ∈ F_p`, `τ^t(a) = c` on all of `V` | P-192 (`v₂(p+1) = 64`), P-256 (96), P-521 (521): pass, 4–38 s in Python big integers |

Consequence for the cost ledger (accounting, not a measurement): the
auxiliary-curve search phase that `prime-isogeny:t=T` charges (`≈ 2^t` curves
tried, 0.8–25 % of `S` at 20–24 bits; `≈ 2^{74}` GAE derived at P-192, `t = 48`)
is replaced by `O(t)` field operations, for every prime with `v₂(p + 1) ≥ t`
or `v₂(p − 1) ≥ t`. One of `p ± 1` always carries `v₂ ≥ 2`; the NIST primes
carry 64/96/521 on the `p + 1` side. For odd `ℓ` the same holds with `ℓ^t | p ± 1`
and the degree-`ℓ` Chebyshev/Dickson doubling (`T_ℓ`) in place of `σ, τ`; not
measured here.

## 3. Does the presentation change the solving degree?

Instrument: the native closure `closure/` (`pc_closure`, Rust, dense `u32`
rows, exact mod-`p` elimination) on systems dumped by `scripts/raw_closure.py`
(curve, base, target with the exhaustive non-decomposability check, the same
split-`S3` chain as L1's `gen.py`). The Python closure in the same script is the
independent cross-check: on every cell it finished it agrees with the native
crate on the certified degree **and on every per-degree rank and expansion
count** (`python_crosscheck` column of `results.csv`; counts below the table).
Two random inconsistent targets per cell (seeds 1, 2; both values shown), raw
polynomial-calculus degree `D`; a cell reads `≥D` when the dense column budget
(`C(n+D, D) ≤ 45,000`) or the 1200 s cap stopped it with `1 ∉ W_{D−1}` certified.

| m | t | N | K (p=40961) | S (p=40961) | T (p=8191) | T (p=73727) | M (p=40961) | C (p=8191) |
|---|---|---|---|---|---|---|---|---|
| 2 | 1 | 2 | 4, 4 | 4, 4 | 4, 4 |  | 4, 4 | 4, 4 |
| 2 | 2 | 4 | 4, 4 | 5, 5 | 5, 5 | 5 | 4, 4 | 4, 4 |
| 2 | 3 | 6 | 4, 4 | 5, 5 | 5, 5 | 5 | 5, 5 | 5, 5 |
| 2 | 4 | 8 | 4, 4 | 5, 5 | 5, 5 | 5 | 5, 5 | 5, 5 |
| 2 | 5 | 10 | 4, 4 | 5, 5 | 5, 5 | 5 | 5, 5 | ≥5, ≥5 |
| 2 | 6 | 12 | 5, 5 | 5, 5 | 5, 5 |  | 5, 5 | ≥5, ≥5 |
| 2 | 7 | 14 | 5 |  | 5 |  |  |  |
| 3 | 1 | 3 | 5, 5 | 5, 5 | 5, 5 |  | 5, 5 | 5, 5 |
| 3 | 2 | 6 | 5, 5 | 6, 6 | 6, 6 |  | 5, 5 | 5, 5 |
| 3 | 3 | 9 | 6, 6 | 6, 6 | 6, 6 |  | 6, 6 |  |

Python cross-check (same dumped system): 41 cells agree on D and every per-degree rank, 7 agree on the degrees Python finished, 0 disagree, 51 not run in Python.

Planted (consistent) controls, `1 ∈ W_D` must stay false:

| arm | p | m | t | result |
|---|---|---|---|---|
| C | 8191 | 2 | 3 | no 1 up to D=6 |
| K | 40961 | 2 | 3 | no 1 up to D=14 |
| M | 40961 | 2 | 3 | no 1 up to D=10 |
| S | 40961 | 2 | 3 | no 1 up to D=14 |
| T | 8191 | 2 | 3 | no 1 up to D=14 |

![raw PC solving degree against N, five presentations](figures/d_vs_n.svg)

99 cells, 713 s of native closure in total, largest dense matrix 43,758 columns.

**Reading, against the frozen rule.**

- *Positive control.* Arm K reproduces the L1 certified ladder wherever
  `D ≥ 4`: `m = 2`: 4,4,4,4,5,5 at `t = 2..7` (L1: 4,4,4,4,5,5), `m = 3`: 5,6 at
  `t = 2,3` (L1: 5,6). At `t = 1` the raw degree is 4 where L1's normal-form
  closure gives 3, the one-degree raw/NF difference below `D = 4` that THEORY.md
  and the protocol anticipate. The instrument is not suspect.
- *Negative control.* No planted (consistent) target reached `1 ∈ W_D` within the
  budget (`D ≤ 10–14`).
- *Prediction P1 (presentation invariance): REFUTED.* At every cell with
  `2 ≤ t ≤ 5` (`m = 2`) and at `t = 2` (`m = 3`) the Möbius-coordinate arms S and
  T are exactly one degree above K, on both seeds, at two primes for T. The
  polynomial-doubling arms M and C equal K at `t ≤ 2` and are one above at
  `t = 3..5`; A1's prediction that the *link* costs the degree was wrong at
  `t = 2` and right from `t = 3` on.
- *The shape of the difference.* It is a **shift of the staircase, not a new
  law**: at `m = 2, t = 6` all four non-Kummer arms read 5 and K steps up to 5;
  at `m = 3, t = 3` all finished arms read 6. The penalty is one degree over the
  `t`-range where K sits on a tread; nothing here distinguishes a bounded offset
  from a slower-growing one, and at larger `t` the arms rejoin. The growth of
  `D` with `N` that governs the lane is the same in every presentation.
- *Prediction P2 (consequence).* Holds in the weak form: the supersingular
  `j = 1728` shortcut is usable, and the torus `T₂` tower, written as the circle
  (arm C), is never more than one degree above the Kummer tower in these cells
  and equal to it at the largest ones. The strong form ("the same solver cost
  per relation at every size") is not supported: `D(C) − D(K) ∈ {0, 1}` here, and
  one degree multiplies the Macaulay size by about `N`.
- *Verdict under the decision rule:* P1 **REFUTED**, replicated (seed 2, second
  prime for T); the direction is "torus/Möbius presentations are worse by ≤ 1
  degree", so no new lead is opened for the L1 instrument.

## 4. What this does and does not change

- **Success criterion.** A removed phase is an accounting change at the
  constant level; the exponent of the PKM lane is governed by the degree
  staircase, which is reproduced here and unchanged. No row, no `S`, no gap to
  rho is quoted. This does not "beat" anything.
- **The C6 question** (`research/pkm_isogeny_tower_p192_20261015`): a
  supersingular `E'` is acceptable to the relation phase in the sense that its
  tower, written as the circle, is within one degree of the Kummer tower at
  every finished cell and equal at the largest; written in the Möbius
  coordinate (the literal `x`-maps of the Vélu chain) it is one degree worse
  from `t = 2`. The ecbench plug-in should therefore present `T₂` bases as the
  circle tower (two variables per level, one bilinear link), not as `x`-maps.
- **Theory.** The circle tower is the Weil restriction of the split Kummer
  tower in `z ∈ F_{p²}`, so over `F_{p²}` it is linearly equivalent to arm M
  (Kummer + bilinear link); PC degree is invariant under field extension, which
  predicts `D(C) = D(M)` — observed at every finished cell (both seeds). The
  earlier onset of the penalty in arms S/T (`t = 2` instead of `t = 3`) is
  therefore a property of the *rational* doubling (`a = 0 ↦ ∞`: no affine
  coordinate of the non-split torus makes doubling polynomial over `F_p`); the
  later penalty shared by M and C is the bilinear link `x·(…) = …` replacing
  the linear `x = g·y₀`. Both are shifts; the Kummer tower is the best of the
  five presentations of this set and is the one the lane's certified ladder
  already uses.
- **Next tests (not run).** (a) The circle tower at `m = 4`, `t = 4..5` against
  the certified Kummer values (needs the native closure on a larger host);
  (b) `prime_tower.rs`: add the `T₂` circle base and re-run the 2026-10-15
  ecbench panel so the removed phase appears in `S` (an accounting PR under the
  validation gate); (c) odd `ℓ` towers (`3^t | p ± 1`).

## 5. Sources checked

research-kb (MCP `kb_status`/`search_knowledge`) and the lead atlas
(`crypto-autoresearcher/knowledge/leads/ecdlp/`) were **not available** in this
container (no sibling checkout, `scripts/research-kb` exits 127); the
repositories were grepped instead. Read for this note (repo:path):

- `crypto:research/prime_leads_chase_20261113/L1-pkm/{PROTOCOL,THEORY,D83_VERDICT,AMENDMENTS}.md`, `results.csv`, `scripts/gen.py`, `scripts/indep_closure.py`
- `crypto:research/prime_leads_chase_20261113/{README,CONTINUATION_PROTOCOL}.md`, `L5-fibre/REPORT.md`
- `crypto:research/pkm_isogeny_tower_p192_20261015/README.md` (C6), `crypto:research/pkm_isogeny_tower_ecbench_20261015/README.md`
- `crypto:research/pkm_symmetric_tower_20261026/README.md`, `crypto:research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`
- `crypto:research/prime_ic_lead_review_20261031/{REPORT,README}.md`, `lead-matrix.json` (PKM primary PDF sha256 `a53030a6…`)
- `crypto:src/bin/pkm_aux_search.rs`, `crypto:RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2
- Primary: Petit, Kosters, Messeng, PKC 2016 (slides as supplied by the user; paper pinned in the lead matrix).

Negative search: no file in either repository mentions the norm-one torus `T₂`
or the circle presentation for a PKM base (`grep -ri "norm-one\|T_2\|circle tower"`
hits only the unrelated norm-branch work in `research/prime_norm_rank_20261112`).

## 6. Evidence index

`runs/*.jsonl` (one file per cell, per-degree rank/columns/time), `runs/progress.txt`,
`runs/torus_check.json`, `results.csv`, `results_table.md`, `figures/d_vs_n.svg`,
`closure/` (native closure crate), `scripts/` (generator + Python cross-check +
aggregation; Python lays out and cross-checks, the native crate computes the
reported degrees — see §3 for which rows are native-confirmed).
