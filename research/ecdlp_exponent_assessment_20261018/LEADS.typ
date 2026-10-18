= Further ECDLP leads, ranked (2026-10-18)
Companion of #link("REPORT.md")[`REPORT.md`] §4, which ranks four leads (Koblitz chaining at `m ≥ 4`, Gaudry `k = 4` end to end, generic constants to the floor, kernel engineering). This note lists what else the committed record of the four repositories supports, ranked by expected movement per unit of work under the success criterion (a verified, measured improvement over the current index-calculus baseline; distance to rho is reported, not gated). Every lead names its level in the bounds protocol (`docs/bounds/README.md` §2: exponent, constant, primitive, machine, or accounting), the evidence for and against it with the file that holds it, the smallest experiment that would settle it, and an effort estimate. Sources were read on this branch and on the open PRs \#2156 (T01–T17), \#2152 (E19–E25), \#2163 and \#1888; `crypto-autoresearcher`\'s ledger and `cryptanalysis`\'s October experiments were read for rejections so that nothing below re-proposes a closed idea (§5). Nothing here is a result; every number is the source\'s.

`lead_map.svg` (from `lead_map.py`, values in `lead_map.diagram.json`) plots the leads by effort and level; `LEADS.pdf` embeds it.

== 1. Where an exponent can still move (five leads, each with a decisive test)
#strong[Outcomes so far (2026-10-18, this PR).] L3 closed (#link("../s5_small_roots_variants_20261018/README.md")[`../s5_small_roots_variants_20261018/`]): no sublattice gain at levels 1–3, multi-polynomial variants not applicable. L5 measured (#link("../ghs_cubic_quartic_ic_20261012/SCALING_20261018.md")[`../ghs_cubic_quartic_ic_20261012/SCALING_20261018.md`]): the counted cost scales as `ℓ^{2.50}` against rho\'s `ℓ^{0.50}`, because the recorded \"pure IC\" recovery is a transfer enumeration of the subgroup, not a relation solve; the class-arithmetic problem card is reopened. L2 measured (#link("../koblitz_halfdim_base_20261018/README.md")[`../koblitz_halfdim_base_20261018/`]): chain-free `m = 2` is 18× cheaper than the one-link `m = 3` chain at matched dimension on `icv1-f2m23-t5197-69e76b73`, but its best `S` is 17× rho there and 256× at `n = 31`, with the pair table, not the chaining term, as the cost. L1 (closure degree at `m = 4`) and L4 (msolve at `m = 3, N = 21`) are running; first L1 draw at `(13, 5)`: closure degree 4.

These are the only leads with a mechanism that could change `α`; each is low-probability and bounded in cost, which is why they rank first.

=== L1. Closure degree as the true solving degree of the Koblitz chain (exponent-adjacent, 2–3 weeks)
- #strong[Mechanism.] The ECC2K-130 chain cost rows (`ℓ + 4` fit, `2^{+112}`, `docs/ic/BOUNDARY_TARGETS_CHARGED_SUPPLEMENT_20261008.md`) were built on plain-XL solving degrees of 5–7 rising with `ℓ`. The MutantXL closure engine refutes the same systems at degree 3–4, flat through `m·ℓ ≈ 20` (`research/dreg_closure_mutantxl_20261016/RESULTS.md`, `research/dreg_closure_scale_20261016/RESULTS.md`). If closure degree 4 holds at `m·ℓ = 24`, every chained-system cost in the Koblitz programme is over-priced by the Macaulay growth between degrees 4 and 7, and the \"free-oracle floor\" reading of the assessment (`2^{56.46}` at `m = 4` against rho\'s `2^{60.81}`, `2^{5.71}` operations per oracle call) has to be redone with the cheaper oracle.
- #strong[For.] `(12,3)`, `(14,3)`, `(13,4)` at `m = 4` are flat at 4 on 8/8 draws; 400/400 at `n = 19` for the mutant closure (`KN-FIND-c3a917`); the leftover degree-5 refutations are a plain-Macaulay artefact (`KN-FIND-5a8d3e`).
- #strong[Against.] `(19,7)` and `(21,7)` at `m = 3` exceed 4 on 8/8 draws; the open reading is \"set by `m·ℓ`\" (21 rising, 16–18 refuting). The Hilbert-series predictor failed (T06, 8/148; T15 null). Deciding cells need 70–150 GB on dense engines against 14–16 GB available.
- #strong[Test.] The two cells `m = 4, ℓ = 5` (`m·ℓ = 20`) and `ℓ = 6` (24), 8 draws each, on the implicit-row block-Wiedemann closure engine the assessment names as T05/T06\'s follow-up (not started). Closure ≥ 5 on ≥ 7/8 draws at either cell keeps the chain model; 4 at both moves every chained row and re-opens `m ≥ 4` at the sizes REPORT §4.1 names.
- #strong[Effort.] The sparse closure engine is the cost (one to two weeks); the cells are hours each.

=== L2. Half-dimension Frobenius-invariant base, handled implicitly (representation half of REPORT §4.1; 2 weeks)
- #strong[Mechanism.] The chaining term `(m − 2)·n` disappears when the invariant base has dimension `≈ (n+1)/2` and `m = 2` (quadratic system, `≈ n + 1` unknowns instead of `≈ n²/dim`); the binding constraint becomes materialising `2^{dim}` points, and past `n ≈ 40` the base has to be used through its defining linearised polynomial, which is the low-degree condition the algebra already uses (`research/notes/ecc2k130/RESEARCH_KOBLITZ_SCALING_TARGET.md`, \"The chaining term is avoidable\").
- #strong[For.] `n = 31` at dimension 16 is buildable today (32 unknowns, quadratic, against 190 cubic for the single-factor base; `build_frobenius_factor_base_from_divisor`).
- #strong[Against.] With `m = 2` the linear algebra is `(|F|/n)² ≈ r/n²`, which is below `√r` only while `r < n⁴` (true at `n = 31`, `r = 2^{20.5}`; false from `n ≈ 41`). So `m = 2` is a tool to measure the orbit saving without chaining, not an exponent; the exponent needs `m ≈ n/ℓ`, which is the wall. The construction is empty where 2 is primitive modulo `n` (`n = 131, 163`: two cosets only; `RESEARCH_KOBLITZ_INDEX_CALCULUS.md` :244–267).
- #strong[Test.] `n = 31`, dim 16, `m = 2` end to end with every phase charged and `S` against the counted signed-Frobenius rho; then the implicit base at `n = 41` and `47` with `m = 3` (one chain link, `3ℓ + n` unknowns). The primary metric of the scaling note (unknowns under 64 at `n = 31` #emph[and] `n = 63` with a solve and a clean gate) is the finish line.
- #strong[Effort.] Implicit-base membership and the `m = 3` driver: two weeks.

=== L3. Small roots of `S₅` at `δ ≈ 1/5` by methods the probes did not cover (exponent, prime field; 1 week, low probability)
- #strong[Mechanism.] The only prime-field lever with a falsifier: a decomposition oracle for `S₅` cheap at `B ≈ p^{1/5}` (`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.4, §4.1; `m = 4` is the first arity that can beat rho once the linear algebra is counted).
- #strong[For.] The Probe B/C framework is in place and is the falsifier. The variants #emph[not] covered by Probes B/C or by the on-paper closure of box-Coppersmith (`research/lit2026_w10_s5_coppersmith_20261011`: `δ*` 1/6, 1/24, 1/80, 16× short of 1/5) are named in the program itself (:234–246): Boneh–Durfee sublattices, Herrmann–May unravelled linearisation, cuso Gröbner-optimal shifts for fixed `M`, multi-polynomial Coppersmith adding the curve equations `y_i² = x_i³ + a x_i + b` or sharing unknowns across targets, non-box bounds, and the 2026/1027 LP (fetch failed, HTTP 403).
- #strong[Against.] Every reach measured to date fades by `δ = 0.18` and the symmetric form only makes the reduction 3–10× cheaper, not further. Rigorous Howgrave–Graham fails past `δ = 0.10`.
- #strong[Test.] The milestone row: `δ ≥ 1/4` at `m = 3` on a 40-bit curve (ties rho); the target row `δ ≈ 1/5` at `m = 4`. An LP or cuso asymptotic bound below `p^{1/5}` closes the lever for good (`research/literature_search_20261011/README.md` item 7).
- #strong[Effort.] Days per variant at `p ≈ 2^{32}`; the closing bound is the cheaper outcome.

=== L4. The PKM tower at `m = 3, N = 21` (exponent decision for the auxiliary-curve line; a 6 h wall on the present host, not a bigger host: the earlier stop was the 45-minute cap at 2.6 GB)
- #strong[Mechanism.] Whether the Kummer-tower solving degree stays bounded at `m = 3` decides the P-192 auxiliary-curve search gated behind it (`research/backlog/XP-20261015-pkm-m3-n21-m4-n20/README.md`, `research/elliptic_snfs_survey_20261016/README.md` §7).
- #strong[For.] It is the only open question in the PKM line that decides an exponent; the `m = 2` line rises to 6 at `N = 20` and the torus and split degrees agree where measured (`research/pkm_dickson_torus_20261015`).
- #strong[Against.] `m = 4, N = 20` answered negative (`D ≥ 8`; the line reads 6, 7, 7, ≥ 8 at `N = 8, 12, 16, 20`). The `m = 3, N = 21` cell is inconclusive at `D ≥ 7`. Correction (2026-10-18): that cell did not hit memory. `ic_next10_20261014/03-pkm-degree/runs/*.mem` shows the three `m = 3`, `N ≥ 18` runs stopped by `timeout 2700` (exit 124) at 1.5–2.6 GB resident; the memory wall (exit 139 at 4.86 GB of a 5 GB cap) is the `m = 4` cells\'. So the blocker is wall time, not a 256 GB host, and the rerun is `research/pkm_m3_n21_20261018/` (14 GB cap, 6 h wall, msolve built from source). The exponent-space structure of the tower is falsified (`research/ic_exponent_falsifiers_20261013`: largest-class share 0.244 against null 0.248).
- #strong[Test.] Finish `(m, N) = (3, 21)`: `D ≥ 7` persisting closes the line like `m = 4`; a bounded `D` re-opens the P-192 search with the degree as its cost.
- #strong[Effort.] Hours of wall on the present host (two systems, 6 h cap each); a disk-backed reducer only if the second degree-7 step itself runs past the cap.

=== L5. The `m = 2` genus-3 plane-quartic cover transfer with relations by lines: measure the IC scaling against rho (exponent on paper; 3 weeks; cubic-extension curves with cofactor ≥ 4 only)
- #strong[Mechanism.] Diem\'s genus-3 index calculus on the cover costs `Õ(q^{4/3})` against rho\'s `≈ q^{3/2}` on `E(F_{q³})`; the `m = 3, r = 5` hyperelliptic transfer is executed (Cantor, injective on the odd part, `|ker φ| ∈ {2, 4}`); the `m = 2` plane-quartic class arithmetic exists since 2026-10-14 (`quartic_ic.rs`, Pic checks and a pure IC target path passing 90/90 at `q ∈ {5, 7, 11}`; `problems/plane-quartic-jacobian-class-arithmetic-missing.md` is `resolved (toy smokes)`). What is missing is the scaling measurement: the cover IC\'s counted cost against rho on the same instances at growing `q` (`research/ghs_cubic_quartic_ic_20261012/SCALING_20261018.md`; `research/ghs_cubic_hyperelliptic_genus3_20261013`, `research/ghs_cubic_cover_census_20261012`).
- #strong[For.] For `4 | #E(K)` a genus-3 cover is one 2-isogeny away on 438 of 439 curves at `q = 5, 7, 11`. Linear algebra on genus 3 already scales `q^{1.52}` locally with the relation phase at `≈ 1.43`, 0.06 above `4/3` (`research/ic_improve_20261015`).
- #strong[Against.] Prime-order curves over a cubic extension have no such cover. Odd-characteristic GHS covers of genus 6, 10, 14 do not exist, and 846 of 848 `formula_advantage` verdicts are priced on a genus the construction does not deliver (`research/odd_char_descent_genus3_20261014`). The ledger closes genus-3 and cross-genus covers in general (`KN-TECH-36e667`, `KN-FIND-61347e`); this lead is the specific executed class, not the family.
- #strong[Test.] Implement linear equivalence on a plane quartic; execute the transfer on the `q = 7` case; collect relations by lines through pairs of `F_q`-points; count against matched rho in one unit; replay.
- #strong[Effort.] Two to three weeks.

== 2. Constants on measured rows (the success criterion\'s daily work)
=== L6. Register the extension-field domains and run the four-size challenge (accounting first, then constant/exponent; 1 week)
No `ecbench` domain, bound or challenge exists for `E(F_{p⁴})` (`research/gaudry_quartic_d4_cost_20261014/README.md` :179–181) or for the genus-3 covers; the `k = 4` rows (E24: 35–240× below D4, handover `2^{116.7} → 2^{90.1}`; E25: GLS fold 1.86× median) therefore cannot be labelled at any level. The first step of REPORT §4.2 is this registration: incumbent bound (`ecbench bound fit` over four sizes), then a sealed four-size challenge. Until then every `k = 4` figure stays a stage number. Next concrete engineering on that domain is the compiled straight-line `S₅` solver (record once, replay per trial; the decision rule of `research/ic_speed_and_rho_scope_20261013` is C4 ≥ 10× below the §11.17 value, exactness checked against the MITM oracle on every solve).

=== L7. Phase-aware τ-slots into a production solver, and family-optimum base sizing (constant, `log₂ m!` bits; 1–2 weeks)
- Measured off-line: 2.8× more decompositions per trial at `n = 23` in table form; 1.79× (`n = 23`) and 1.34× (`n = 17`) in algebraic form against a predicted 2 (`docs/ic/LEAD_LADDER_METHODOLOGY_20261010.md` §7.5–7.8); not in any production solver. At `n = 131` the table does not fit (`K² n` states) and the slots must ride the `S₃`-chain extraction.
- Family-optimum sizing: 5 columns against 45 at `n = 23` cut the total 22× and land 1.55× of counted rho; re-basing the leaderboard\'s tables A and B on `compact-orbit-scan` bases at optimum size is a measurement campaign, not done (`research/ic_leads_20261010/RESULTS_ROUND2.md` items 1–2).
- #strong[Test.] `ecbench challenge` of the folded pair table at `n = 41 … 61` with slots on, counts only; the table A rows\' `× reference` column is the outcome.

=== L8. Multi-target amortisation against the `S·T²` line (accounting; in flight)
98.5–99.7 % of a one-target table A row is reusable set-up (`docs/ic/LEADERBOARD.md`, phase shares), and table C already reads 3.89× rho at `n = 47` for 32 targets. The honest frame is the Corrigan-Gibbs–Kogan line `S·T² ≳ r/2n`: every pair-table IC is generic, the rungs sit 25–27 bits above the line, and the N83 Bernstein–Lange control beats the IC online path 3.74–395× at equal precompute (T10; `cryptanalysis/experiments/koblitz-n83-bl-arm-20261014`: IC/BL online 235 \[61, 645\]). PR \#2163 freezes the equal-advice protocol at `n = 73/83`; the owed piece is K2, charging factor-base construction in both arms (bounds `R ≤ 3.24`, cold `S ≥ 9.68` in the stacked producer study). Do not open a second lane; read \#2163\'s result.

=== L9. A better rho reference (constant on the reference; 1 week)
Every \"× reference\" column is only as honest as the reference. Known defects: distinguished-point fruitless-cycle abandonment truncates trails at `dp = 12` to 0.43–0.48 of geometric and costs 1.37–1.58× the ideal steps; paired co-Z `bsgs_fast` steps are 1.15–1.67× in wall and 16.75 → 3.71 M-equivalents per point but are not applied to the `prime_fast` walk; interleaved negation-map BSGS is derived at `0.943√n` against `1.00√n`; `pollard_collab`\'s negation-map 2-cycle escape never fires (`research/lit2026_w11_generic_engineering_20261011`, `research/generic_dlp_memory_parallelism_20261017/README.md` :266–290). #strong[Test.] `ecbench challenge` of `rho.negation` and `rho.signed_frobenius_strong` with the trail fix at matched `dp`; counts must drop ≥ 1.3× for the reference to move, and every IC row\'s distance is then re-read against it.

=== L10. Gate-first F6-IC ordering and RREF root as defaults; cost-based wide-path switch (machine/primitive; days)
Exact on 4 cells: gate-first 1.57–1.78× lower stage unit, RREF root 1.07–1.48× fewer units than the default echelon root; both pending an x86 eager-rewrite check and more cells (`research/lit2026_w4`/`w5`, `research/f6_gate_first_x86_20261016`). The F4 wide-path switch is `n_vars ≥ 24`, not cost: at `n = 23` the symmetric systems take 9–14 s on the narrow path against 2.5–3.0 s forced wide (cross-session observation; needs one interleaved measurement).

=== L11. Block structure in the Gröbner step and several invariant bases per run (constant; days)
Both are named as \"measurable now\" in `RESEARCH_KOBLITZ_INDEX_CALCULUS.md` :906–914 and never measured: the block/homogeneous structure of the `m ≥ 3` chained systems (the equivariant F4 of T07 exists, `n` prime, `d ≤ 40`), and the paper\'s use of several factors of `x^n − 1` per run (the driver uses one). #strong[Test.] `n = 23 … 31`, `m = 3`, word-XOR counts with and without, 8 draws.

== 3. Blocked or long
- #strong[L12. ECC2K-130 PDP by a structurally different relation producer.] Capacity floors for 1 % one-shot support: `B ≥ 6.06e7` (`m = 5`), `4.12e6` (`m = 6`); an explicit `S₃` pair-key index on the exact N131 weight-≤ 6 base needs `≥ 2^{88.36}` actions; every ordinary SAT/XOR run is censored and planted controls fail too (`cryptanalysis/experiments/ecc2k130-263-capacity-gate-20261004`, `…/compact-s3-m4-*`). The stated next step is the fully coupled four-leaf symbolic-support search with the gates N53 known → N83 known → N83 ordinary. Blocked by search power; zero yield is not shown.
- #strong[L13. A method that sees the conductor.] The falsifier is sharp (`ic.pipeline` `log₂ S` excluding 0 at both `ℓ = 263` and `8209` with the same sign); the prior is low (volcano level effect 1.022 \[0.983, 1.062\]; conductor gap invisible at matched structure, round 2 :210). Six backlog cards are blocked and five campaigns are queued in PR \#1888; read them before spending.
- #strong[L14. A Galois-invariant base from another commutative algebraic group over `F₂` with a point of order 131] (`KN-OPEN-095df5`): tori, elliptic curves (`#E(F₂) ≤ 5`) and abelian varieties of dimension ≤ 4 are excluded; a construction or a non-existence proof beyond dimension 4 is theory work with no compute.
- #strong[L15. Learned structure.] Every cost label measured so far is null (`ml-cryptanalysis/experiments/hardness`, `isogeny-stratified`); M2b (learner-invented feature) and M3 (blind search with a pre-registered `ecbench` exit) are unstarted; the ledger\'s RELN lane owes a cycles-of-length-≤ 4 statistic (NA-1). Keep it behind L1–L11.

== 4. Measurement and tooling leads (cheap; raise the evidence floor)
- #strong[M1. The IC exponent with a base that scales.] The sweep\'s fixed 32-point base makes `α > 1/2` by construction (REPORT §5: `0.757 ± 0.051`); the untested counterpart is `|F| ∝ (p·T_dec)^{1/3}` at `m = 2` (`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.4 predicts `2/3`). One `ecbench sweep` option (factor-base size as a function of size) and an afternoon of runs.
- #strong[M2. Pins and drift.] Re-pin `calibration_pins` on an isolated host (loaded-host pins priced identical work at 0.22 against 8.7 GAE); resolve the inherited-F4 word-XOR cross-host drift (120/168 replay-all identical, 48 counter mismatches) before that counter enters a bound; counted group operations per phase for the n83 producer stack so the §5a band can be computed (`research/ecbench_frontier_20261013/README.md` :204–217; `cryptanalysis/experiments/koblitz-n83-stacked-20261016`, follow-up 1).
- #strong[M3. Zero-compute items the ledger itself names.] The Assumption-1 disclosure (`KN-OPEN-7f0511` (C)), the `S₆` top-form check (`KN-OPEN-02200b`, predicted degree 80), and RC-1 of `KN-OPEN-3c8f51`.
- #strong[M4. External Gröbner baselines] (T17: msolve, M4RI as optional arms) and the F6 generalised root finder\'s `≥ 2×` gate (`research/generalized_rootfinding_20261016`).

== 5. Closed: do not re-propose
Each of these is recorded as negative, closed on paper, or superseded, with the record that closes it.

#align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [idea], [closed by],
  [Couveignes–Lercier invariant bases for Koblitz rungs `n > 5`],
  [Hasse caps `#E(F₂)` at 5 (`RESEARCH_KOBLITZ_INDEX_CALCULUS.md` :892–904)],
  [Box-Coppersmith small roots of `S₃`, `S₄`, `S₅`; symmetric rewriting],
  [reach 1/6, 1/24, 1/80; Probes B/C; `lit2026_w10`],
  [Multiplicative-subgroup bases, xedni, embedding into `F_{p^n}`],
  [`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §4.6],
  [Sub-`B`-degree algebraic factor bases (bounded degree)],
  [`IDEA-20260801-021` theorem; `KN-OPEN-020` keeps only the unbounded case],
  [Structured factor-base geometries; isogeny-class variation of yield],
  [`H-FB-001`, `H-ISO-001`, `KN-FIND-007` (yield conservation)],
  [Jets, incidence, elliptic nets, transfer operator, tensor trees, noncommutative paths],
  [`H-JET/INC/NET/TRA/TTN/NCP-001` rejected\_scoped],
  [F4 trace replay (one trace, many targets)],
  [T08: net loss or refused; `H-CERTBIN-a73f1c`],
  [Single large prime on the folded table; residual matching; cheaper canonicalisation],
  [`LEAD_LADDER_METHODOLOGY_20261010.md` §7.4, L3, L2 (ceiling 1.04×)],
  [`Z[τ]`-multiplier closure as a new lever],
  [\"cosmetic\" (`RESEARCH_TRANSFER_AWARE_IC_20261010.md`); structured-GGM `S·T ≳ N`],
  [Zero-minor guess-and-determine; 3SUM transfer; Hu 2025/015],
  [`lit2026_w12`; `threesum-ic-transfer-20261007` (exponent gain 0.0008)],
  [PKM `m = 4, N = 20`; exponent-space structure of the tower],
  [`D ≥ 8`; `ic_exponent_falsifiers_20261013`],
  [Quasi-subfield chains below `β = 1/2`; Hamming-ideal bases],
  [`H-QSP-5540d7` (`β > 1/2`); `KN-OPEN-5e3dd0` (calls \`],
  [GHS covers of genus 6, 10, 14 in odd characteristic],
  [do not exist (`odd_char_descent_genus3_20261014`)],
  [ECC2K-130 descent to the 263-floor; isogeny levels as a lever],
  [mapped back by a `2^{15.6}` isogeny; no level effect (`isogeny-volcano-cost_20261017`)],
  [Learned predictors of rho or IC cost from curve structure],
  [every cost label null (`ml-cryptanalysis` M7, M9f; T15)],
)
]

== 6. How this list was built, and its limits
Three readers covered: this repository\'s research notes, `docs/ic`, `docs/bounds` and the October research directories (about 75 of 290 read for their limits and next-step sections; the rest by heading); the open PRs \#2156, \#2152, \#2163 and \#1888 by body; `crypto-autoresearcher`\'s `knowledge/open-problems` (64), the ECDLP frontier rows (52), hypotheses by status (2,125), decisions filtered to pause and inconclusive (3,435) and the idea registries; `cryptanalysis`\'s October experiment reports (about 247, those carrying a decision read in full) and `ml-cryptanalysis`\'s plan and reports. Not read: the 2,679 ledger proposals and 774 pending ideas, `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (not in the checkout), and the research directories listed as unread in the readers\' notes. A lead absent here may be in those.

No number above was re-derived from raw artefacts in this pass; each is quoted as its source states it, and the source is the place to check.
