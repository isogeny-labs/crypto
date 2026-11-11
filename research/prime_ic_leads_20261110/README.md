# Prime-field index calculus: the distance to rho in the lab's own observable, the equation budget every new lead must meet, and two cheap tests (2026-11-10)

**Class.** A desk study with two derivations, one implemented presentation experiment
(a development check on this host, not an admitted run) and a ranked lead list. It
reports no `S`, no cost ratio, no leaderboard or `benchmarks/` row, and changes no
frozen record. Every number quoted from a committed file carries its path; every
number computed here is labelled **derived**; every measurement made here is a
development check labelled `[pcc:unverified]`.

**Request (user, 2026-11-10).** "Work on prime-field index-calculus leads; it seems we are
stuck and cannot reach rho parity."

**Answer in one paragraph.** On the generic track parity is impossible and the gap
widens like `r^{1/6}` (program note §1, §2.4): its remaining value is measured constant
gains. On the algebraic track the only exponent lane on record, the PKM tower family, is
converging to closure: the last fall degree `d_F` grows with the tower height at one
degree per four levels (C-step holds at `t = 11, 12`; `t = 13` reads `≥ 6`) and with the
arity at half a degree per unknown bit (stage C). §3 turns that growth into the distance
to parity in the lab's own observable: at the linear-algebra exponent `ω' = 2`, parity at
arity `m = 4` needs a fall-degree slope below `0.017` per unknown bit, and no arity up to
10 tolerates more than `0.066`; the measured slopes are `0.125` (in `t`) and `0.5` (in `m`).
§4 names the mechanism behind the growth as an *equation budget*: a prime field supplies
one cross-summand equation per relation, and every further one costs about `log₂ p`
unknown bits, against one per bit for Weil descent over `F_{2^n}`. That budget is a filter;
every idea examined today (§5) fails it, including five that are new to the atlas, and
the one family that passes it for free, the `[k]`-multiple consequences of the relation,
is implemented as a presentation control (§6, `--controls multiples`) with pre-stated
predictions. The leads that remain (§7) are the two decisive PKM cells already queued, the
first counted algebraic-track row, the multiples and null-support probes, and the generic
constant gains. Nothing here reaches parity, and nothing on record can; what this note
adds is the exact statement of what a new lead would have to supply.

## 0. Sources checked

- **Knowledge base: unavailable.** The `research-kb` MCP server is not loaded in this
  session and `scripts/research-kb status` reports "no executable found". `ecdb`, `cairn`
  and Conductor are configured in `.mcp.json` but not connected. The fallback below was
  used: the lead atlas and repository paths.
- **Lead atlas** (`crypto-autoresearcher` `main` `3ba0739cb9` and later, sparse clone of
  `knowledge/leads`, `tools`, `orchestration`): 176 leads, 30 in `ic-prime/`.
  `python3 tools/lead_atlas.py --match` was run for each idea below; the matches are
  listed where the idea is discussed. No `LEAD-ICP-*` entry covers the `[k]`-multiple
  consequence presentation (§6) or the slope threshold (§3).
- **Repository records read** (all on `main` at `f14b66786b`):
  `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` (§1, §2.2, §2.4, §4, §4.6),
  `research/prime_ic_program_20261029/{README,DECISIONS}.md`,
  `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` (§0, §2, §3.7),
  `research/prime_fall_degree_20261027/PREREGISTRATION.md` (§1, §2, §6, Amendments 6–8),
  `research/pkm_falldegree_unblock_20261107/{RESULTS,RESULTS_STAGE_C,RESULTS_T12_BRACKET,STAGE_B_LOST}.md`,
  `research/curve25519_exponent_study_20261108/README.md`,
  `research/semaev_alternatives_audit_20261104/README.md`,
  `research/lever2_closure_20261031/REPORT.md`,
  `research/curve25519_t0_symmetry_20261102/README.md`,
  `problems/curve25519-family-algebraic-ic-tower.md`,
  `src/cryptanalysis/ic_framework/pkm_semaev.rs` (header, `Membership`),
  `examples/pkm_tower_pilot.rs`, `src/cryptanalysis/hky_fp_tower.rs` (`fall_degrees`),
  `docs/session-logs/2026-11-10.md` (the open-leads entries).

## 1. Where the prime-field program stands (measured, by path)

| item | value | source |
|:--|:--|:--|
| Generic track, Table B, `icv1-fp10` … `icv1-fp24` (`log₂ r` 9.7–23.4), `m = 2` pair-table oracle | IC / rho-negation = 5.47× to 13.5× | `docs/ic/LEADERBOARD.md` Table B; `research/prime_ic_program_20261029/README.md` §1 |
| Generic track, 20-bit factor base `N = 32 → 45` | gap to rho 9.47× → 7.75× [6.92, 8.70]; second-host replay 1,008/1,008 identical | `research/prime_base_oracle_tuning_20261029/README.md`; atlas `LEAD-ICP-prime-ic-constant-engineering` |
| Algebraic track, prime-field baseline | **empty** (no counted row; R1 Option C) | `research/prime_ic_program_20261029/DECISIONS.md` |
| Algebraic oracle smoke, `pkm-semaev` on a Curve25519 miniature (`k = 20`, `t = 2`, 3 columns) | `S` = 1027 / 49,513 / 485,893 at `w` = 0 / 0.1 / 1 against 9.05 / 9.46 / 13.16 for the generic incumbent, one target | `problems/curve25519-family-algebraic-ic-tower.md` |
| PKM Kummer `m = 2`, `P1 = 15·2²⁷ + 1`, last fall degree by `t` (`N = 2t`) | `t = 1..10`: 3, 4, 4, 4, 4, 5, 5, 5, 5, 6; `t = 11`: 6; `t = 12`: 6 (cap-5 / cap-6 bracket); `t = 13`: ≥ 6 (cap 5 does not refute; cap 6 lost twice to Spot); `t = 14`: ≥ 6 | `research/pkm_falldegree_unblock_20261107/RESULTS.md` Table 1; `RESULTS_T12_BRACKET.md`; `docs/session-logs/2026-11-10.md` 04:45Z |
| C-step (`d_F = 4 + ⌊(t − 2)/4⌋`, Amendment 6) | holds at `t = 11, 12`; undecided at `t = 13, 14` (predicts 6, 7) | `RESULTS.md` §5; `RESULTS_T12_BRACKET.md` |
| Stage C, fixed `t`, growing `m` (Kummer `P1`) | `t = 2`: `d_F` = 4, 5, 6, 6, ≥ 7 at `m = 2..6` (`N` = 4, 6, 8, 10, 12); `t = 3`: 4, 6, 7, ≥ 8, ≥ 8 at `m = 2..6` (`N` = 6, 9, 12, 15, 18); H-m reads "grows in m" on all six ladders | `RESULTS_STAGE_C.md` Tables 1–2, §6 |
| `ffd` | 4 on every Kummer draw, 5 on every isogeny draw, at every `(t, m)` | `RESULTS_STAGE_C.md` |
| Exponent study, symmetrised subgroup base, Curve25519 miniatures | `m = 2`: `s` = 2.77 [2.31, 3.23] at `k = 20`; `m = 3, 4`: undetermined (censored at 3.5 GB / 1,200 s) | `research/curve25519_exponent_study_20261108/README.md` |
| Stage B (`m = 3..6` N-ladders) | lost to two Spot interruptions; owner decision pending | `research/pkm_falldegree_unblock_20261107/STAGE_B_LOST.md` |
| `m = 4` closure at `N = 20` (Amendment 8, decisive for the lane) | not launched: 112 GiB, up to 49 h, Spot nodes interrupted within the hour | `docs/session-logs/2026-11-10.md` 04:45Z |

Reading. The lane's two readings (Amendment 8) are still open, but every completed cell is
on the "grows" side: C-step has not been refuted, and the `m`-ladder grows faster than the
`t`-ladder `[pcc:unverified]`. The decisive cells (`t = 14` cap 7; `m = 4` at `N = 20`) are infrastructure
problems now, not research problems.

## 2. What parity requires (derived, from the cost model on record)

With a base of size `B`, arity `m`, per-target decomposition cost `T = B^s` and a sparse
solve `≈ m B²`, the end-to-end cost is about `m!·r·B^{s−m+1} + m B²`, balanced at
`B = r^{1/(m+1−s)}`, total `≈ r^{2/(m+1−s)}` (program note §2.4; tower note §3.7). It is
below `r^{1/2}` if and only if

    s < m − 3,

the gate the Semaev-alternatives audit corrected (`research/semaev_alternatives_audit_20261104/README.md`:
"`s < m − 3`, i.e. `e < (m − 3)/4`"). So `m = 4` needs a per-target cost sub-linear in the
base, `m = 5` needs below `B²`, `m = 6` below `B³` (the meet-in-the-middle cost, which is
why the generic track ties rho there only with `B³` memory). Any oracle that costs
`B^{s}` with `s ≥ m − 3` is on or above the rho line whatever its constant.

## 3. The distance to parity in the lab's observable: the fall-degree slope (derived)

The lab measures `d_F` (the HKY last fall degree of the closure) and `D` (F4's solving
degree, `D ≥ d_F` on every gated draw, G2/G3). Both are degrees in the tower quotient ring,
where every monomial is multilinear in `N = m·t` one-bit unknowns (plus the chain's free
unknowns, which only add). A dense closure or Macaulay elimination that has to reach degree
`d` meets at least

    M(N, d) = Σ_{k ≤ d} C(N, k)   columns,   log₂ M(N, αN) ≈ N·H(α)   for α ≤ 1/2,

with `H` the binary entropy. If the elimination costs `M^{ω'}` (`ω' = 1` is reading the
matrix once; `ω' = 2` is the sparse-elimination exponent the lab uses for the relation
matrix), then with `B = 2^t`, `N = m log₂ B`, the per-target cost is

    T ≥ B^{m·H(α)·ω'},   i.e.   s ≥ m·H(α)·ω'.

Put into the gate of §2: the lane can reach parity only if

    H(α) < (1 − 3/m) / ω'.

| `m` | `α*` at `ω' = 1` | `α*` at `ω' = 2` | `α*` at `ω' = 3` |
|--:|--:|--:|--:|
| 4 | 0.042 | 0.017 | 0.010 |
| 5 | 0.079 | 0.031 | 0.019 |
| 6 | 0.110 | 0.042 | 0.025 |
| 8 | 0.156 | 0.056 | 0.033 |
| 10 | 0.189 | 0.066 | 0.038 |

(`α* = H⁻¹((1 − 3/m)/ω')`, computed here; the script is `scripts/thresholds.py`.)

Measured slopes, from §1: in `t` at `m = 2`, C-step gives one degree per four levels,
`α = 1/8 = 0.125` per unknown bit; in `m` at fixed `t`, stage C gives `(6 − 4)/(8 − 4) = 0.5`
at `t = 2` and `(7 − 4)/(12 − 6) = 0.5` at `t = 3`. Both exceed every threshold in the table
at `ω' ≥ 2`, and the `m`-slope exceeds every threshold at `ω' = 1` too. The distance to
parity on this lane is therefore not a constant: it is the factor by which the fall-degree
slope would have to fall (about 7× in `t`, about 10–30× in `m`, at `ω' = 2`), and the
`m`-ladder is the one that matters because §2 needs `m ≥ 4`.

Two checks this derivation invites, both from recorded fields:

- **Column counts.** The rows record `max_cols`. At `t = 13`, cap 5, F4 used `max_cols =
  16,572` against `M(26, 5) = 83,682` (`docs/session-logs/2026-11-10.md` 03:20Z; `M`
  computed here). So F4's matrices are about 5× sparser than the dense count at that
  degree; the bound above is for the dense closure, and a sparse engine evades it only by
  the ratio `max_cols / M(N, d)`, which the rows make measurable. That ratio must shrink
  exponentially in `N` for the conclusion to change; a constant factor does not move `s`.
- **Cost against degree.** The uncapped `D = 6` row at `t = 11` has `muladds 4.88·10¹²`
  and the cap-6 rows at `t = 12` have `4.22·10¹³` (`docs/session-logs/2026-11-10.md`
  03:20Z): a factor 8.6 for `N` 22 → 24 at the same degree, i.e. about `2^{1.55}` per bit,
  which is already `B^{3.1}` per summand at `m = 2`. That is a single pair of rows and is
  reported, not fitted.

**What this does not establish.** It is a lower bound for dense degree-`d` elimination
and a model for F4; it says nothing about an engine that never forms the degree-`d`
matrix. It assumes the fall degree keeps its slope beyond the measured `N ≤ 28`; a flat
`d_F` at larger `N` would be the H1 outcome the study is designed to detect, and the `m =
4`, `N = 20` closure is the cell that tests it.

## 4. Why the degree grows: the cross-summand equation budget (derived)

Weil descent over `F_{2^n}` works because one relation `R = Σ P_i` with `x_i ∈ V`
(`V` an `n'`-dimensional `F_2`-subspace) yields `n` equations over `F_2` (the `n`
coordinates of `S_{m+1}`) in `m·n'` bit unknowns, each equation of bounded degree in the
bits, with `m·n' ≈ n`. Equations and unknown bits are in balance, one to one, and the
first-fall-degree phenomenon follows from that balance (HKY, Petit–Quisquater).

Over `F_p` a tower base `V = g·μ_{2^t}` gives the same kind of unknowns (`t` one-bit tower
variables per summand, each with a quadratic defining equation that fixes it given the
next) but the relation yields **one** equation over `F_p`. There is no Galois group to
split it. The only ways to get a second equation from the same relation are:

1. **apply a fixed rational map** `φ` (an endomorphism, an isogeny, a translation) to
   every summand and write the relation for the images: `φ(R) = Σ φ(P_i)`. For the new
   equation to be low degree in *tower* variables the images must lie in a tower too,
   which multiplies the base's defining conditions: `|F| = p·(|V|/p)(|V'|/p)`. Keeping
   `|F| = B` forces `|V|·|V'| = B·p`, so the second equation costs `log₂ p` extra unknown
   bits. With `j` such maps: `j + 1` equations against `log₂ B + j·log₂ p` bits per
   summand. The ratio tends to `1/log₂ p`; for Weil descent it is 1.
2. **enlarge the coefficient ring** so that one relation has several components:
   `F_{p^k}` (the extension embedding: atlas `LEAD-ICP-extension-field-embedding`,
   obstructed, Gaudry cost `p^{2−2/k}`; the trace-zero toy test
   `research/tracezero_toy_20261106` found no structure), `Z/p^k` (the torsion lift of
   `E(F_p)` to `E(Z_p)` exists and is a homomorphism, but the second digit of `x` is a
   function of degree about `N²` of the first: atlas `LEAD-TRF-padic-canonical-section`,
   closed obstructed), or jets `F_p[ε]/(ε²)` (atlas `LEAD-EXO-padic-jet-canonical-lift`,
   open). Each costs `log₂ p` bits per extra component, by the same count.
3. **use consequences of the relation that need no new unknowns**: the multiples
   `[k]R = Σ [k]P_i`, because `End(E) ⊇ Z` is the one structure every curve has. These are
   free, but `x([k]P)` has degree `k²` in `x`, and in the Kummer quotient a polynomial of
   degree `e` in `x` has multilinear degree up to `popcount(e)`, so the `[2]`-consequence
   enters at degree 6 and `[3]` at about 8 (§6). They add equations at a degree that grows
   like `log k`, not at a bounded degree.

So a prime-field tower system is a one-generator system in a `2^N`-point quotient ring
(tower note §3), and the budget says why it stays one: each additional cross-summand
equation of bounded degree costs about `log₂ p` unknown bits, unless it is a multiple of
the relation, in which case it costs degree. This is a counting argument, not a theorem:
it assumes the base is cut out by bounded-degree conditions on the coordinates of the
summands and of finitely many fixed rational images of them, presented with auxiliary
variables of bounded fibre, and it is heuristic about what the solver does with the
equations it has. Its cousin on record is the bounded-degree factor-base theorem
(`IDEA-20260801-021`, cited in `research/lever2_closure_20261031/REPORT.md`), which bounds
the *scheme* degree; this bounds the *equation count per relation*.

**The filter.** A proposed prime-field algebraic lead should answer, before any run:

1. How many equations of bounded degree does one relation give, after every per-summand
   membership equation is removed? (Weil descent: `n`. PKM: 1.)
2. How many one-bit (or bounded-fibre) unknowns does it need per summand to describe a
   base of size `B`? (Weil descent: `n'`. PKM: `t = log₂ B`.)
3. Does every extra equation come with a new `F_p`-valued unknown, or with an extra
   membership condition that shrinks the base? If yes, the ratio is `1/log₂ p`.
4. If the equations are consequences of one relation (multiples, conjugates under a
   group that does act), at which degree do they enter the quotient ring?

A lead that cannot raise the ratio in (1)/(2) above `O(1/log₂ p)` with bounded-degree
equations is, by this count, a presentation of the same one-generator problem.

## 5. Ideas examined today and closed on paper (with the atlas matches)

Each of these came up while looking for a new equation source; each fails the filter.
They are recorded so that they are not re-proposed.

| idea | what it is | why it fails | atlas match (`--match`) |
|:--|:--|:--|:--|
| 2-torsion (`T₀`) symmetrised PKM tower on Montgomery-form curves | base closed under `x ↦ B/x`, tower on the invariant `u = x + B/x`, FGHR/FHJRV-style symmetrised `S_{m+1}` | For `y² = x³ + Ax² + Bx` the 2-isogeny with kernel `(0, 0)` has `x`-map `x ↦ x + A + B/x` (Vélu; standard, not a new identity), so a tower on `u` **is** a plain tower on the isogenous curve `E/⟨T₀⟩` pulled back by the isogeny, with `R = Σ P_i + εT₀` ⇔ `φ(R) = Σ φ(P_i)`. Same one-generator system, same `d_F` ladder; the symmetrisation halves the column count (a constant), as `research/curve25519_t0_symmetry_20261102` measured for the pair table. The Kummer tower is also unavailable on Curve25519 (`v₂(p − 1) = 2`, `problems/curve25519-family-algebraic-ic-tower.md`). | `LEAD-ICP-pkm-tower-algebraic-oracle` (15), `LEAD-ICB-cofactor-torsion-quotients` (6), `LEAD-ICP-gfp5-symmetrized-gaudry` (4) |
| Translate-pair bases `{P : x(P) ∈ V, x(P + T) ∈ V'}` for a fixed `T` (or `x(φP) ∈ V'` for an endomorphism `φ`) | the relation and its translate give two summation equations linked per summand by `S₃(x_i, x_T, x'_i)` of degree 2 | Filter item 3: `|V|·|V'| = pB`, so the second equation costs `log₂ p` bits per summand; ratio `2/(log₂ B + log₂ p)`. | `LEAD-ICP-algebraic-membership-factor-base` (6, 13) |
| Wagner `k`-tree / generalised-birthday filtering on partial sums landing in a tower set `V'` | lists of base points merged where the pair sum has `x ∈ V'` | The merge keeps `|L|²·|V'|/p` of `|L|²` pairs; a list of size `|L| ≤ |F|` is preserved only if `|F| ≥ p/|V'| ≥ √(2p)`; the filter set must be a subgroup-like set, which a prime-order group lacks. Same obstruction as a two-level index calculus with an `m = 2` lower level. | `LEAD-EXO-four-list-ktree-join` (14), `LEAD-ICB-cofactor-torsion-quotients` (22) |
| `p`-adic second digit from the torsion lift `E(F_p) → E(Z_p)` | the prime-to-`p` torsion of `E(Z_p)` maps isomorphically onto `E(F_p)`, so every relation lifts exactly and `S` mod `p²` gives a second, linear equation in the second digits | The second digit of `x(L(P))` is determined by `[N]` on an arbitrary lift (`N = #E(F_p)`): a rational function of degree about `N²` of `x(P)`. Treated as a free unknown it adds `log₂ p` bits per summand for one linear equation. Atlas: closed obstructed. | `LEAD-TRF-padic-canonical-section` (17), `LEAD-EXO-padic-jet-canonical-lift` (14) |
| Two towers on `x` and on `x(φ(P))` for an efficient endomorphism `φ` (CM curves) | second relation `φ(R) = Σ φ(P_i)` with both images in towers | Same as translate pairs; for `j = 0` the `ω`-conjugate equation is the original equation up to relabelling (`S₃(ζx) = ζ·S₃(x)` for `a = 0`), so it adds nothing. | `LEAD-ICP-xline-symmetry-special-j`, `LEAD-STR-special-j-ic-semaev` (8) |

## 6. The one free family: `[k]`-multiple consequences, implemented as a presentation control

**Construction.** For `R = P_1 + … + P_m` the relation `[2]R = Σ [2]P_i` holds with the
same sign pattern, so `S_{m+1}(x([2]P_1), …, x([2]P_m), x([2]R)) = 0`, where
`x([2]P) = N(x)/D(x)`, `N = x⁴ − 2ax² − 8bx + a²`, `D = 4(x³ + ax + b)`. Cleared of
denominators (by `D_i²` for each argument) it is a polynomial identity in the tower
variables, it vanishes at every planted decomposition, and on a prime-order curve `D_i`
has no zero on `V`, so the zero set of the system over `F_p` is unchanged: the appended
equation is in the ideal, and the experiment is a *presentation* change in the sense the
preregistration already uses for `raw` versus `reduced` (`RESEARCH_PKM_TOWER_ORACLE.md` §2.2,
"the difference is data"). For `m ≥ 3` (chain) the partial sums get doubled unknowns
`u'_k = x([2](P_1 + … + P_{k+2}))` linked by `D(u_k)·u'_k − N(u_k) = 0` (degree 4), and the
doubled chain is written in them.

**Degree at which it enters (derived).** In the Kummer quotient `x^e = g^e·Π y_j^{e_j}`,
so a polynomial of degree `e` in `x` has multilinear degree `≤ max popcount` over its
support. `N` has degree 1, `D` degree 2, `N²` and `N·D` degree 3, `D²` degree 2, and the
largest terms of the cleared `S₃` (`N₁N₂D₁D₂`, `N₁²N₂²`, `N₁²N₂D₂`) have degree 6. So the
`[2]`-consequence cannot enter any closure `V_{F,c}` with `c ≤ 5`, and at `m = 2`, `t ≤ 9`
(`d_F ≤ 5`) the two presentations must give identical `d_F` by construction: a built-in
control. The `[3]`-consequence (`x([3]P)` of degree 9 over 8) enters at about degree 8 and
is not used.

**Implementation.** `examples/pkm_tower_pilot.rs`, `--controls multiples` (a new
`Control::Multiples`): the `tower` system in the same order, then the appended equations;
the same cell seed, so the same tower, curve and targets as the `tower` control; unit tests
check `N/D` against `[2]P` by curve arithmetic and that every appended equation vanishes at
planted solutions. Rows carry `"control": "multiples"`, `"multiples": 2`, `"appended"`.
(Implemented in this session by a delegated agent; see the session log for the test
output. If the implementation is not in this PR, this section describes the design only.)

**Pre-stated predictions and what each outcome does.**

- P1 (control): identical `d_F` and closure dimensions wherever `d_F(tower) ≤ 5`.
- P2: at cells with `d_F(tower) ≥ 7` (`t = 3`, `m = 4`: 7; `t = 2`, `m = 5`: 6–7;
  `t = 3`, `m = 5`: `≥ 8`), `d_F(multiples) ∈ {d_F(tower) − 1, d_F(tower)}`, and the
  `m`-slope is unchanged.
- Falsifier of the budget reading: a drop of at least 2 at a cell with `d_F(tower) ≥ 8`,
  or an `m`-ladder that reads "flat" under the multiples presentation where the tower reads
  "grows". That outcome would say that consequence equations entering at degree 6 do what
  Weil descent's conjugates do, and it would reopen §4 item 3 as a lane (then with `[3]`,
  `[4]` and the chain's doubled partial sums).
- The expected outcome (P1 and P2) closes item 3 at toy scale: the one free equation
  source does not change the growth.

**Development check (this host, 2026-11-10; every value `[pcc:unverified]`).** Rows, commands
and the per-cell table are in `devcheck/` (`table.md`, generated by `tabulate.py` from the
rows). Closure engine, Kummer at `P1`, seed `0x504B4D54`, two random targets per cell, both
arms on the same targets. No isolation tier, other jobs on the host; wall times are not
claims. The tower arm reproduces stage A's medians at `t = 2..9` (4, 4, 4, 4, 5, 5, 5, 5)
and stage C's `m`-ladder cells (`t = 2`: 5, 6, 6 at `m = 3, 4, 5`; `t = 3`: 6, 7 at `m = 3, 4`),
which is the instrument check.

| cell | `d_F` tower | `d_F` multiples | Δ | falls tower / multiples |
|:--|:--|:--|:--|:--|
| `m = 2`, `t = 2..9` (16 rows) | 4, 4, 4, 4, 5, 5, 5, 5 (both draws) | identical on every row | 0 | identical sequences |
| `m = 2`, `t = 10` | > 6 (budget-censored at `c = 6`, 600 s) | > 6 (same) | — | [4, 5] / [4, 5] |
| `m = 3`, `t = 2` (`N = 6`) | 5, 5 | 5, 5 | 0 | [4, 5] / [4, 5] |
| `m = 3`, `t = 3` (`N = 9`) | 6, 6 | 5, 5 | **−1** | [4, 5, 6] / [4, 5] |
| `m = 4`, `t = 2` (`N = 8`) | 6, 6 | 5, 5 | **−1** | [4, 5, 6] / [4, 5] |
| `m = 4`, `t = 3` (`N = 12`) | 7, 7 | 6, 6 | **−1** | [4, 5, 6, 7] / [4, 5, 6] |
| `m = 5`, `t = 2` (`N = 10`) | 6, 6 | 6, 6 | 0 | [4, 5, 6] / [4, 5, 6] |
| `m = 5`, `t = 3` (`N = 15`) | > 7 on both draws (`c = 7` closure complete without `1`; `c = 8` budget-censored at 1,500 s) | > 6 on both draws (`c = 6` complete without `1` in 8 s; `c = 7` censored by the entry cap `2.5·10⁸` at 399,850 columns, 400 s; the uncapped attempt was memory-killed at about 14 GB) | inconclusive | [4, 5, 6, 7] / [4, 5, 6] |

Reading against the pre-stated predictions:

- **P1 holds.** At `m = 2` the two presentations agree on every completed cell, draw by draw,
  including the fall sequences, as the degree-6 entry requires (the multiples row has one more
  equation, `n_equations` 2t+2 against 2t+1, and the same `n_vars`).
- **P2 holds, and nothing reaches the falsifier.** At `m ≥ 3` the drop is exactly one degree on
  six of ten completed cells and zero on the other four; no completed cell drops by two, and the slope
  is unchanged: at `m = 4` both arms rise by one degree from `t = 2` to `t = 3` (6 → 7 and
  5 → 6), and in `m` at `t = 3` both rise by one from `m = 3` to `m = 4` (6 → 7 and 5 → 6).
  The one-degree offset at `m ≥ 3` comes with the doubling links (degree 4) and the doubled
  chain's own unknowns `u'_k` (`n_vars` +`m − 2`), so it is a presentation shift of the kind
  the chain presentation itself introduces, not a change in growth. The one cell where the
  falsifier could still fire on this host, `m = 5`, `t = 3`, is censored on both arms (tower
  `> 7`, multiples `> 6`) and is reported as inconclusive: it needs a host with more than
  about 16 GB for the degree-7 closure of the augmented system, or a sparser closure engine.
- **What this closes.** At toy scale, the one free source of extra cross-summand equations
  over `F_p` lowers the last fall degree by at most one and leaves both slopes as they were.
  §4 item 3 stays closed; the budget reading stands as far as this check can test it. What
  it does not test: `[3]`- and `[4]`-multiples together, larger `N`, or an F4 run (`D`)
  on the augmented systems.

## 7. Leads, ranked

| rank | lead | kind | cost | discriminates | atlas |
|--:|:--|:--|:--|:--|:--|
| 1 | **Finish the decisive PKM cells**: `t = 14` cap 7 (C-step predicts 7), and the `m = 4`, `N = 20` closure (Amendment 8: rise / flat / fall) | measurement, queued | EKS jobs already requested (`requests_r2h/`); the `m = 4` job needs a host that survives 49 h or a split amendment | "close the lane" vs "keep as a mechanism question for `m ≥ 6`" (Amendment 8, as written) | `LEAD-ICP-pkm-tower-algebraic-oracle` |
| 2 | **Multiples presentation probe** (§6) at the stage C cells `t = 2, 3`, `m = 2..5`, both draws, `--engine hky`, same seeds | presentation experiment | minutes to an hour on a 4-core host | P1/P2 vs the falsifier above | none (new) |
| 3 | **Null-support slope**: run the pilot's `null` control (random coefficients on `S₃`'s support, planted zero) and supports of width `w = 1..4` on the same towers, and fit `d_F` against `N` and `w` | mechanism closure | same tool, minutes per cell | if `d_F(N, w)` is curve-independent, the growth is a property of a one-generator system in the carry-graded cyclic group algebra and the lane closes with a named mechanism (the note's H0, now with its cause); if the curve's `S₃` sits below the null at equal support, Semaev structure helps and is worth isolating | `LEAD-ICP-construction-normal-form` (instrument), `LEAD-ICE-prime-field-semaev-solving-degree` (closed_known) |
| 4 | **First counted algebraic-track row**: `pkm-semaev` on a Table B curve, `m = 2` full and `m = 3` chain, §5a band, matched rho-negation, replay certificate. *Specification written 2026-11-11* (`algebraic_row_spec/`: `spec.json` with 3 curves × 8 targets × 14 arms, protocol draft validated OK, blockers, submission steps; the Kummer base is unavailable on fp16/18/20, `v₂(p − 1)` = 3, 1, 1, so the isogeny base at `t = 4..6` is used; derived budget 4.2 h single-thread, 46 vCPU-hours ceiling with replays; owner decides the route) | measurement, spec ready | EKS (`D-EKS` approved 2026-10-29: about 49 CPU-hours); the oracle exists (`pkm_semaev.rs`, `Membership::{Kummer, Isogeny}`) | gives the headline track a baseline to ratchet; the smoke row (§1) suggests about 100× above the generic incumbent at `w = 0`, which is the number the track would then report as its distance | `LEAD-ICP-pkm-tower-algebraic-oracle`, program `reference_arm/` |
| 5 | **Generic-track constants.** *Reconciled 2026-11-11* (`prefetch_status/ANALYSIS.md`): the hash-table prefetch Design A patch is obsolete. `main` already carries an independent `layout=lines` implementation of the same line-layout idea (`src/cryptanalysis/ic_framework/plugins.rs`, option `layout=lines` on `mitm-prime-batch` and `mitm:negation_folded=1`, with equality tests), and it was measured: the line-layout tournament v6 at fp28 reads the candidate 12 % slower in the valid sessions and is recorded as `invalid_measurement` (`research/prime_field_lead_assessment_20261107/README.md`, rows 53 and 146; `research/ic_line_layout_tournament_20261031/`) `[pcc:unverified]`. What stays open on this lead is the software-prefetch step itself (`prefetcht0` on the landed layout) and `N > 45` at 20 bits. | engineering | build host + EKS L2 session | the only place a measured improvement over the current baseline is available; cannot reach parity (§2) | `LEAD-ICP-prime-ic-constant-engineering` |
| 6 | **Honest non-uniform accounting**: record `S·T²` per Table B row and the preprocessing-rho reference (Probe A of the program note) | accounting | desk | makes the many-target regime comparable; not an improvement | `LEAD-ICP-retained-setup-online` (obstructed) |

What would change this list: an equation source that passes the filter of §4. None is
known to this note; the three exits in §4 item 2 are the places to look, and the jet lead
is the only one of them still open in the atlas.

## 8. What this note does not claim

No new algorithm, no exponent, no `S`, no rho comparison beyond the recorded ones quoted
in §1, no novelty claim for the budget argument beyond "not in the atlas and not in the
files read" (the knowledge base was unavailable), and no result from the multiples
control beyond what `devcheck/` records. The slope thresholds of §3 are a model with the
assumptions stated there. The identity behind the 2-torsion row of §5 is Vélu's formula for
a 2-isogeny on a Montgomery-form curve, standard and not certified here.

## 9. Files

- `README.md` (this note); `REPORT.pdf` built from `REPORT.typ`.
- `fig_slope.svg` / `.png`: the recorded `d_F` ladders of §1 against `N`, with the `α*`
  lines of §3 (`scripts/plot_slope.py`, recorded values only; data in `data/dF_rows.json`).
- `fig_budget.svg`: the equation-budget diagram of §4 (`scripts/plot_budget.py`).
- `scripts/thresholds.py`: the `α*` table.
- `devcheck/`: the multiples control's development-check rows and commands, when run.
