# Preregistration: binary (GF(2^n)) Koblitz fall-degree study

**Slug** `binary_falldeg_20261118`. **Registered** 2026-11-18, before any
measured row was read. This file is the frozen protocol
(`experiment.protocol/v1` in spirit; the cross-repo checker is in
crypto-autoresearcher). Pilot data taken on the local container is
`[pcc:unverified]` and is **not** a study row; it exists to calibrate the
instrument and to size the pod cells (§10). No row here reaches the IC
leaderboard or `benchmarks/README.md`; this is an algebra study of fall
degree, not a cost or security claim (§9).

This is the bridge between the prime-field last-fall-degree work
(`research/prime_fall_degree_20261027/`, `research/pkm_falldegree_unblock_20261107/`)
and the binary Koblitz/Frobenius work. The prior binary program
(`research/fall_degree_bounds_20261008/`) measured the Huang–Kosters–Yeo
(HKY) last fall degree `d_F` of Weil-descended Semaev systems over
`GF(2^n)` with a **random** and a **subfield** factor base (3–4 up to
`N = 60`, first `d_F ≥ 5` at `m = 3`, `ℓ ≥ 7`). The new axis here is the
**Frobenius-orbit** factor base — the squaring-closed subspace that the
Koblitz endomorphism makes natural — against a random base of the same
dimension and a structured non-orbit control.

---

## 0. Disclosed before registration

- The prior binary program's headline readings above
  (`research/fall_degree_bounds_20261008/RESULTS.md`, `TABLE.md`), its
  definitions (ledger §0), and its controls (random Boolean quadratic
  systems, trace relation). This study reuses its `d_F` definition.
- The recorded signed-Frobenius-orbit **relation-rank** census
  (`research/rootfind_pair_collision_census_20261116/`): the public Koblitz
  cells `K_0` over `GF(2^13)` (standard subspace dim 5), `GF(2^19)` (dim 7),
  `GF(2^23)` (dim 8), curves
  `ICV1:f2m-13-752c6463`, `…-19-87f45376`, `…-23-c07a6b8c`, subgroup orders
  `r = 2003, 130873, 2095853`. Those are **relation-matrix** ranks after a
  signed-Frobenius column fold, a different object from the fall degree
  here; they fix the public curve identities and standard-subspace
  dimensions this study reuses, not any fall-degree value.
- The library instruments reused unchanged:
  `koblitz_groebner::{build_decomposition_system, sym_semaev_s3,
  first_fall_degree, solving_degree}`,
  `koblitz_bench::{random_subspace_basis, chained_solution_count_fast}`, and
  the exact HKY degree-`c` mutant closure of `examples/hky_par.rs`
  (`closure_par`), copied verbatim into the instrument so a `random`-arm row
  reproduces `hky_par`'s `d_F` by construction (§6 gate G3).

---

## 1. Question

For the Weil-descent / Semaev summation systems of binary Koblitz curves,
how does the fall degree (first-fall `d_ff`, and last-fall `d_F` where
computable) behave as a function of the field extension `n` and the number
of summands `m`, and **does the Frobenius-orbit structure of the factor
base change it, versus a random factor base of the same size** (and versus a
structured non-orbit base)?

---

## 2. System (frozen)

- **Curve family.** Binary Koblitz `E_a: y² + xy = x³ + a x² + 1` over
  `GF(2^n)`, `a ∈ {0, 1}`. `K_0 = E_0`, `K_1 = E_1`. The subgroup order is
  the curve's largest-prime factor; `a` and the modulus are read from the
  curve database / the recorded public cells, never recomputed.
- **Summation system.** For `R = P_1 + … + P_m` with every `x(P_i)`
  constrained to the `F_2`-span `V` of the arm's basis, the system is the
  Weil restriction of the chained third summation polynomial that
  `build_decomposition_system` builds: `S_3(x_1,x_2,e_1)`,
  `S_3(e_i, x_{i+2}, e_{i+1})`, …, last link closes on the target `x(R)`.
  Variables: `m·ℓ` subspace coordinates, then `(m−2)·n` intermediate-point
  coordinates. `n_eqs = (m−1)·n` Boolean equations in `n_vars = m·ℓ + (m−2)·n`
  Boolean unknowns.
- **The binary `S_3`, exactly** (`sym_semaev_s3`, read from source):
  `S_3(x_1,x_2,x_3) = (x_1+x_2)² x_3² + x_1 x_2 x_3 + (x_1 x_2)² + b`,
  which expands to `x_1²x_2² + x_1²x_3² + x_2²x_3² + x_1x_2x_3 + b`.
  **This is the `a_2`-independent form** (no `a x²` term survives in the
  symmetric summation polynomial of a binary curve). Koblitz has `b = 1`.
  **Consequence, registered as a finding, not a hypothesis:** `K_0` and
  `K_1` give the *identical* `S_3` summation system. Any fall-degree
  difference between `a = 0` and `a = 1` can therefore come only from the
  factor base's relation to the curve group and from which targets `x(R)`
  are reachable — never from the polynomial. The two Koblitz values are
  **not** two independent systems for `d_ff`/`d_F`; the study runs the
  system once per `(n, m, ℓ, arm, draw)` and reports it for both `a`
  with this caveat stated.
- **Target `x(R)`.** Drawn uniformly at random per `(n, m, draw)` and shared
  by all three arms, so the arms differ only in `V`. Only draws with an
  exact Boolean solution count of **0** (`chained_solution_count_fast`) are
  measured — an inconsistent system is the one whose fall degree the HKY
  framework reads. Consistent draws are skipped and counted, never forced.
- `b = 1` fixed; the field modulus is the lab's sparse irreducible
  (`find_irreducible_sparse(n)`).

---

## 3. Arms (factor base `V`, matched `F_2`-dimension `ℓ_eff`)

The three arms are three choices of the `F_2`-subspace `V ⊂ GF(2^n)`:

- **(i) `orbit` — Frobenius-orbit base.** `V` is a **squaring-closed**
  (`τ`-invariant, `τ`: `x ↦ x²`) `F_2`-subspace: the span of a union of
  complete squaring-orbits of random field elements. This is the
  summation-system analogue of the Koblitz Frobenius-orbit factor base (the
  curve endomorphism acts on `x`-coordinates as `x ↦ x²`, so a point factor
  base closed under `τ` has squaring-closed `x`-coordinates). `V` is
  `τ`-invariant **by construction**, checked per row (`squaring_closed`).
- **(ii) `random` — the null population.** A uniformly random
  `ℓ_eff`-dimensional `F_2`-subspace (`random_subspace_basis`), not required
  to contain `1` or be `τ`-stable. This is the distribution the orbit claim
  says the structured base differs from.
- **(iii) `subspace` — structured non-orbit control.** The polynomial-basis
  low-degree window `V = span{1, x, …, x^{ℓ_eff−1}}`. Structured (a linear,
  nameable subspace) but generally **not** squaring-closed — it isolates
  "structured" from "Frobenius-structured". (The prior program's subfield
  base `GF(2^{n/2})` is *both* a subfield and `τ`-invariant, so it cannot
  separate the two; this window can.)

**Dimension-matching rule (frozen).** A genuinely `τ`-invariant `F_2`-subspace
has dimension equal to a sum of sizes of 2-cyclotomic cosets mod `n`; it
cannot be an arbitrary `ℓ`. The **orbit arm therefore fixes `ℓ_eff`**: build
the orbit base to the smallest dimension `≥ ℓ_target` reachable by whole
orbits, record `ℓ_eff`, and build the other two arms at exactly `ℓ_eff`. All
three arms of a cell share one `ℓ_eff` and one target per draw. Where no
`τ`-invariant subspace of dimension `≥ ℓ_target` below `n` exists, the cell
is **"orbit not available"** and is reported as such — the dimension
constraint itself is a finding (§7, H-avail).

---

## 4. Fall-degree definitions in the binary setting (frozen)

All three are computed in the **Boolean ring** `B = F_2[x_1,…,x_{nv}] /
(x_i² + x_i)`, where every variable is idempotent (the field equations of
`F_2`). The Weil descent already lands in `B`, so — unlike the prime field —
the field equations have degree 2, the normal form is square-free, every
ideal containing the field equations is radical, and the HKY closure is
natively well-defined. This is the key transfer statement (§4a).

- **First fall degree (rank proxy) `d_ff`.** `first_fall_degree`: the least
  `D ≥ 2` at which the bounded Macaulay matrix (rows `= u·f`, `deg u ≤ D − 2`)
  has `rank < rows` **and** `rank < cols`. This is the operational FFD of
  Galbraith–Gebregiyorgis 2014 §4 / Petit 2015. The library labels it a
  **rank-deficiency proxy**, not the syzygy first-fall degree of
  Caminata–Gorla; duplicate rows can trigger it. Reported as a proxy.
- **Last fall degree `d_F` (HKY).** The Huang–Kosters–Yeo mutant closure:
  `V_{F,c}` is the `F_2`-span of the degree-`c` Macaulay rows `u·f`
  (`deg ≤ c`), closed under `g ↦ x·g` whenever `deg g < c`, in `B`; `d_F` is
  the least `c` with `1 ∈ V_{F,c}`. Computed by the exact `closure_par`
  engine (copied from `hky_par`). For an **inconsistent** system this `d_F`
  is the degree at which the summation ideal is certified to have no
  `V`-decomposition. A closure reaching `c = cmax` without `1` reads
  `d_F > cmax` (censored, one-sided). This is the number the prior binary
  program reported.
- **Resolution degree `d_res` (context only).** `solving_degree`: the least
  degree at which bounded multiples of the *original generators* refute or
  pin every occurring variable. The library states it is **not** the HKY
  `d_F`, not an F4/F5 trace, not a complete Gröbner test. Reported alongside
  so `d_ff ≤ d_F ≤ d_res` can be checked (an instrument sanity, not a
  hypothesis).

### 4a. What carries over from the prime-field `V_D`/closure, and what does not

- **Carries.** The HKY closure `V_{F,c}` and `d_F` are *natively* defined over
  `GF(2^n)` after Weil descent, because the ambient ring is already the
  Boolean ring with square-free normal form. The prime-field program had to
  *recover* this (`research/prime_fall_degree_20261027/PREREGISTRATION.md`
  §1): over `F_p` the field equations `x^p − x` have degree `p`, so HKY "does
  not port" to an unstructured base, and a PKM tower was needed to restore a
  square-free normal form. **No such device is needed in binary** — the
  square-free normal form is free. `d_F` is well-defined and computable (to a
  cmax set by memory) for every arm.
- **Does not carry.** (1) Frobenius here is an **automorphism of the field**
  `GF(2^n)` (`x ↦ x²`), not the source of extra field relations as in the
  prime tower. It does not change the ideal; it acts on the factor base `V`.
  So "Frobenius-orbit" means a `τ`-invariant `V`, not a tower presentation.
  (2) The prime study's `λ`-decomposability flag, C-step growth law and
  `D = f4` bracket are prime-tower-specific and are **not** imported; the
  binary inconsistency filter is the exact solution count, as the prior
  binary program used it. (3) "Last fall degree" is well-defined but
  **computability is the binding limit**: the degree-`c` closure has
  `~C(nv, ≤ c)` columns, so `d_F` is readable only at small `(n, m, ℓ)` — the
  pilot ceiling (§8, §10). Where only `d_ff` (Macaulay rank) is reachable,
  `d_F` is reported censored.

---

## 5. Instrument (frozen)

`examples/binary_falldeg_arms.rs` (this PR), built
`cargo build --release --example binary_falldeg_arms`, `Mask = u64`
(`n_vars ≤ 64`). Per `(n, m, arm, draw)` it emits one `binary_falldeg/v1`
JSONL row: `arm, m, n, ell_target, ell_eff, draw, seed, n_vars, n_eqs, x_r,
v_basis, squaring_closed, d_ff_proxy, d_F (or d_F_at_least), d_res,
per_degree_macaulay (rows/cols/rank per degree), per_degree_closure
(plain/closed/of/rounds/fell per degree), secs, loadavg, threads`. The
degree profiles are the exact row/column/rank counts and closure dimensions
— the operation/row count the deliverable asks for. Field-operation tallies
are the library's `bitop_tally` counters, deterministic, not timings.

**Seeds (derived, frozen).** Master seed `0x4b4f425a`. Per draw the target
seed is `FNV(master, "xr", n, ℓ_eff, m, draw)`; each arm's basis seed is
`FNV(master, arm, n, ℓ_eff, m, draw)`. Target is shared across arms;
bases are arm-specific. The orbit arm's `ℓ_eff` is read once per cell from a
reference orbit basis under `FNV(master, "ell_eff", …)`.

**Arm order (interleaved).** Draws are the outer loop; within each kept draw
all three arms are built and measured before the next draw. A draw is kept
only if it is inconsistent in **all** requested arms, so a kept draw gives
one comparable row per arm (same target, same `ℓ_eff`).

---

## 6. Gates (before any measured/pilot cell; any failure stops that engine)

- **G1 — unit tests.** `cargo test --example binary_falldeg_arms` passes:
  (a) the orbit basis is squaring-closed and of dimension `≥ ℓ_target`;
  (b) the window basis has the requested dimension; (c) the squaring orbit
  closes and is distinct; (d) `d_ff ≤ d_F` on an inconsistent resolved
  `m = 2` cell.
- **G2 — ordering sanity.** On every measured row with both defined,
  `d_ff ≤ d_F ≤ d_res`. A violation is an instrument bug and stops the run.
- **G3 — equality with `hky_par`.** On the `random` arm, for a cell both can
  run, `d_F` and the per-degree closure dims equal `hky_par newcells`'s on
  the same `(m, n, ℓ)` and the same draw (the closure engine is copied
  verbatim, so this is by construction; checked once).
- **G4 — positive control (detector power).** A *planted* consistent system
  (a target that **is** a `V`-sum) must have solution count `> 0` and be
  skipped — never measured as a fall degree. Conversely the inconsistent
  filter must admit draws. (The HKY detector's power is that `1 ∈ V_{F,c}`
  exactly when inconsistent; the planted arm proves the filter separates the
  two.)

---

## 7. Hypotheses and decision rules (fixed now)

Per `(n, m)` cell, take the median over kept draws of `d_ff` and `d_F` for
each arm. Report with the A/A noise floor (§7a). Arm labels are masked
during the read where feasible.

- **H-orbit (the central claim; comparative/statistical).** "The
  Frobenius-orbit base changes the binary fall degree vs random." Reads
  **"orbit differs"** if the orbit median `d_F` (or `d_ff` where `d_F` is
  censored) differs from the random median by **more than the A/A spread**
  at the two largest common `(n, ℓ_eff)`, in a consistent direction; reads
  **"no detectable effect"** if every cell's difference is within the A/A
  spread; **"mixed"** otherwise.
- **H-struct (orbit vs structured non-orbit).** Same rule, orbit vs
  `subspace`. Separates "Frobenius structure" from "any structure".
- **H-null-matches (random vs subspace).** The two non-orbit arms set the
  baseline spread between *different* structured/unstructured bases; a
  difference here bounds how much of any orbit effect is just "a particular
  subspace".
- **H-avail (dimension constraint).** Report, per `n`, the set of
  `τ`-invariant dimensions (sums of 2-cyclotomic coset sizes) and the gap
  between `ℓ_target ≈ n/m` and the nearest reachable `ℓ_eff`. The reading
  **"orbit base is dimension-constrained"** holds if for index-calculus
  dimensions (`ℓ ≈ n/m`) the orbit arm must overshoot or is unavailable at
  the prime `n` in scope. This is a structural finding independent of the
  degrees.
- **H-n / H-m (scaling).** Whether each arm's median `d_F`/`d_ff` is flat or
  grows (rise `≥ 2` across the ladder = "grows") in `n` at fixed `m`, and in
  `m` at fixed `n`, as the prior binary program read them.

### 7a. Controls (controlled-experiment skill)

- **Negative control / null population:** the `random` arm (uniform
  `F_2`-subspace of the same `ℓ_eff`) — the closest null to "a subspace with
  no Frobenius structure", matched on dimension, field, curve, target and
  instrument.
- **Positive control (detector power):** the planted consistent system of
  G4, run through the same filter and closure; the detector must flag it
  (solutions `> 0`) and the closure must *not* find `1`.
- **A/A control (noise floor):** two `random`-arm sub-arms differing only in
  the basis seed (same `n, ℓ_eff, m, target`). Its spread in `d_F`/`d_ff` is
  the floor every arm difference is read against. (Seeds differ, so it
  measures variance, not determinism.)
- **Same instrument on every arm:** identical binary, commit, thread count,
  budgets, metric and stopping rule; only `V` changes. Recorded per row.

### 7b. Variable ledger

| factor | treatment |
|:--|:--|
| instrument (commit, binary SHA, `RAYON_NUM_THREADS`) | fixed, recorded per row |
| host (local 4-vCPU container; pod later) | measured (`loadavg`, `threads`); pilot vs pod never pooled |
| factor-base arm (`V` structure) | **crossed** (the variable under test) |
| `ℓ_eff` | fixed per cell (orbit-determined), shared by arms |
| target `x_R` | fixed per draw, shared by arms |
| basis seed | randomized (derived), the A/A nuisance |
| curve `a` | **not a factor for the system** (§2: `S_3` is `a_2`-independent) |

Every arm comparison is `one_factor` (only `V` differs). No comparison
crosses two factors.

---

## 8. Cells

Pilot (local, `[pcc:unverified]`, calibration only): `m ∈ {2, 3}`, and `n`
in two families chosen for their 2-cyclotomic structure —
- **prime `n ∈ {13, 19, 23}`** (the recorded public cells): the orbit arm's
  only `τ`-invariant interior dimensions are near `1` and near `n`, so the
  orbit base must overshoot `ℓ ≈ n/2`; this exercises H-avail;
- **composite `n ∈ {9, 15, 21}`** (rich coset structure): many `τ`-invariant
  dimensions, so `ℓ_eff ≈ n/2` is reachable and the three arms are
  dimension-matched at an index-calculus-like `ℓ`.

`ℓ_target ≈ ⌊n/2⌋` for `m = 2`, `⌊n/3⌋` for `m = 3` (subject to the
dimension-matching rule). `cmax = 4` for `d_F` (the dense closure's reach on
4 vCPU / a few GB), `ffd_max = 6`, `dres_max = 6`. `--keep 3` inconsistent
draws per cell (sub-minute cells repeated ≥ 3×). The exact pod ladder is §10.

---

## 9. What no outcome can show

Toy `n ≤ 23` (pilot) / `≤ ~60` (pod). No exponent, no `S`, no cost, no
security claim, no leaderboard row, no "rho beaten". A flat `d_F` is evidence
on the open HKY question (as in the prior binary program), never a cheap
algorithm — the memory and row counts of each degree are reported so a low
degree is not mistaken for a cheap solve. `a = 0` vs `a = 1` is *not* a
system difference (§2). The orbit arm's effect, if any, is a statement about
the algebra of a structured factor base, not about any deployed curve.

---

## 10. Pod plan (frozen command; not run here)

Filled from the pilot fit in RESULTS before any pod submission; the exact
`n`-ladder, arms, predicted time/memory and the frozen `isolab-eks` command
card go in `research/binary_falldeg_20261118/POD_PLAN.md`. Pod cells need the
owner's price approval (AGENTS.md §9a); this session does not launch them.

---

## Stopping rule

Stop the pilot when either: all §8 pilot cells have `≥ 3` kept draws per arm
with `d_ff` recorded and `d_F` recorded-or-censored; **or** a single cell's
`d_F` closure exceeds ~1 h or the container's memory on 4 vCPU (record it as
the ceiling and move it to §10). Hand back at the ceiling; do not start a
cell that needs a pod.
