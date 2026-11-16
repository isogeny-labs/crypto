<!-- Written by the main session from the red team's returned text, verbatim apart from this line: the red team's harness blocked it from writing this file. The YAML is red_team_report.yaml. -->
# Red team: the sieve-route line search (study A) and the matched negation rho (study B)

**Report id:** `RT-20261103-fd05ef`. **Snapshot:** main `aa1c89ab2a` (PR #3342); the study files are unchanged since `44a1e45ddd`/`2b7c0faa88`. **Studies:** `research/jv_sieve_fbline_20261102/` (A) and `research/jv_sieve_negrho_20261102/` (B), with the combined claim in `research/jv_cover_next_leads_20261102/LEADS.md`.
**Scope:** weak class only (full rational 2-torsion, `#E = 4l`, genus-3 cover over `F_{p^2}`). Nothing in this report bears on prime-order curves.
**Labels:** *measured* = read from a committed record; *derived* = computed from committed records by `analysis/` (Rust, fixed seeds, output `derived_numbers.txt`, reproduced byte for byte from this directory); *model* = a stated pricing or extrapolation.
**Compute:** no new sieve or rho run was needed. Everything below re-reads committed records; about 15 s of CPU in total.

## Verdict per claim

| # | claim (where) | verdict | one-line reason |
|:--|:--|:--|:--|
| A1 | H and U lower the sieve route's net `S` with the search charged (pooled 0.623, 0.600; per prime all upper ends < 1) | **holds** | Recomputed; robust to dropping the 8 pilot-overlap seeds (fresh seeds 25–32 at p = 1511: 0.591, against 0.702 on the pilot seeds) |
| A2 | "U: S⁺ ×1.90, the arm to use" | **holds as measured, unit-inflated** | In a `bitop/v1` proxy that reproduces ic_acct1, U = 0.618 [0.561, 0.680] (×1.62) and H = 0.632. U/H is 0.978 [0.914, 1.051] there and 0.963 [0.883, 1.051] in `S`, so the two arms are unresolved. U wins only in S⁺ (0.954 [0.921, 0.987]) and the band (0.942 [0.922, 0.962]) |
| A3 | "T has the lowest S"; "T improves across the band, pooled" | **breaks** | `table_count` executes `chi[(x0 * p + x1)]`, one integer multiply and one add per walk lookup, and neither is counted. Charged, T's pooled band at w = 0.1 is **1.164 [1.103, 1.231]** (worse than plain) and its S⁺ is 1.016 [0.955, 1.083]. Its S is 1.003 if the multiply is priced as an F_p mult. Without the multiply, T's S win is a §6 relabelling at p ≥ 1511 |
| A4 | The n-rule was frozen before data, and γ = 5 was a sensible borrowing | **holds** (one caveat) | The freeze is a system-clock timestamp with no git anchor before the first run, and the timeline is consistent. The rule's output does not depend on γ: n = 4096 is optimal for every γ in [5, 7.5], and the regret is 0.1% at γ = 4 |
| A5 | The gain is the factor-base size B (B⁻⁵ model) | **holds, per phase** | The passive random-line null is met: elasticities under the move equal those under natural B variation across plain instances for every deterministic phase (table below) |
| A6 | "The gain decays with p, as the B model predicts" | **holds for T and U; H over-decays within noise** | A p-independent phase-mixture model matches T and U within about 7%. H's γ_eff falls from 7.5 to 4.1, and the frozen rule's model slope (0.248) sits at the edge of H's measured 0.711 [0.261, 1.170]. Descent noise explains it |
| A7 | 16 seeds suffice | **for H1 only** | sd of the log descent ratio is 0.68–1.23 per run, so a ±5% per-prime interval on S needs 99–577 seeds (21–90 for the route without descent) |
| B1 | Pooled 142-run reference, chosen by the frozen homogeneity rule | **holds, but the test is near-vacuous** | Power is 0.45 at a 20% shift and 0.20 at 10%. Pooling is legitimate because rho's steps/√l does not depend on the instance (pooled = theory +3.4%), not because of the test. The reference choice moves S/rho by up to 14% |
| B2 | dp-bits 16 versus V2's 15 | **holds** | The overhead difference is 0.03% of the rho mean |
| B3 | The rho is the strongest available | **mostly holds** | Aut(E) = {±1} for the generator. The batch-16 inversion is 2.9% of the tuned price. TUNED's 24-mult F_{q³} product is not minimal: at 18, rho is ×1.34 cheaper in F_p mults |
| B4 | U1/U2 S/rho 0.0065 / 0.0156 compare like with like | **U2 does; U1 does not; S⁺/rho is mixed pricing** | U1 prices rho at a ×2.40 schoolbook harness. "S⁺/rho (rho unpriced)" charges the cover's additions at a full multiplication but rho's 45.1 additions per step at nothing |
| B5 | Reconcile with ic_acct1 `bitop/v1` 0.0098 | **reconciled** | The `bitop/v1` proxy on V2's records gives 0.0091 [0.0085, 0.0099] against the harness rho (ic_acct1: 0.0098 [0.0069, 0.0144]) and 0.0214 against the tuned rho |
| C | Combined ≈0.0042 (U1), ≈0.034 (S⁺ U2) as a product of ratios | **points roughly right; construction should be replaced** | The product mixes seed sets and carries no interval. Directly, H gives 0.0045 [0.0042, 0.0049] on the matched seeds 17–24 (0.0042 on all 16). The 0.034 has mixed pricing; like for like it is 0.0233, and 0.0134 in the `bitop/v1` proxy against the tuned rho |
| S | Weak class only; nothing about prime order | **holds** | README, LEADS and both PDFs say so explicitly. Nothing here transfers |

## Strongest objection (decision-relevant)

**The headline gains and the next-lever choice (L8) depend on S⁺, a unit that prices an 11-bit F_p addition as a full multiplication.** That is about 10× its `bitop/v1` price: `field_add = 13b`, `field_mul = 2(b² + 5b(b−1)) + 13b`, a ratio of 0.096 at b = 11 (`docs/units/BITOPS.md`). In its "rho unpriced" form, S⁺ also leaves rho's 45.1 additions per step free.

- **The arms.** The line search cuts the add-heavy phases hardest: enumeration additions go as B^−7.9 and families as B^−6.9. It also enlarges linear algebra modulo the 62-bit l (B^+2.0), which the `S` unit prices at 16 F_p mults per operation against 31.2 in `bitop/v1`. So S⁺ flatters the arms:

  | unit | U | H |
  |:--|:--|:--|
  | S⁺ (as reported) | 0.525 (×1.90) | — |
  | `bitop/v1` proxy | 0.618 [0.561, 0.680] (×1.62) | 0.632 |

- **The frontier.** With the line search, the frontier against a tuned rho at p = 1511 is 0.0134 [0.0124, 0.0147] (U) in the `bitop/v1` proxy (×75 below rho), against the "≈0.034 (×29)" in LEADS.
- **Lever L8.** Its "up to ×2 in S⁺" has a like-for-like ceiling of **×1.19** at p = 1511 (×1.21 at 1823), even if every emitted addition vanished. Linear algebra is 23.5% of the proxy at 1511 and 27.1% at 1823, and every B lever makes it larger. Its ceiling is ×1.31 / ×1.37. *(derived; the proxy is a model, its assumptions are in the charging section below)*

## Study A: line search on the sieve route

### Freeze and rule (A4)

- **Freeze.** `results/FROZEN.sha` (19:29:06Z, system clock) hashes `PROTOCOL.md`, `run.sh`, `rule.txt` and the binary. The current files match. The first git commit holding the protocol is `73582ce306` (19:37:29Z). By then P_1009 (285 s) and H_1009 (161 s) were complete and T_1009 had 6 of 16 rows. That matches a launch at the freeze, but it is not a cryptographic anchor. `FIT.sha` (the analyser) also matches the committed tool.
- **γ = 5 is harmless for n.** Re-running the frozen rule with C_P from arm P (= V2), n = 4096 is optimal for every γ in {5, 5.4, 5.9, 6.5, 7.5}, and the regret at γ = 4 is 0.1% (`derived_numbers.txt` A5). Borrowing γ could not have steered n_H.
- **The pilot seeds bias against the claim.** The pilot (seeds 17–24, p = 1511, n = 4096) is bit-identical to frozen H on those seeds (same b_best, route and search muls; A6). Those 8 seeds give 0.702, the fresh seeds 25–32 give 0.591.

### Mechanism: is it B alone? (A5; passive random-line null)

The parameter that should destroy the signal is the B gain itself, which shrinks with n → 1 and with p → ∞. The null is natural B variation with no move: plain instances whose base happens to be larger. Elasticities are d ln Q / d ln B. "Move" uses 144 within-instance pairs; "natural" is the prime-demeaned cross-section of the 48 plain instances. The bootstrap resamples instances.

| quantity | move | natural (no move) | move intercept at equal B |
|:--|--:|--:|--:|
| relations per sieve family | 8.05 [7.92, 8.19] | 7.93 [7.46, 8.41] | +0.004 [−0.043, +0.048] |
| sieve families needed | −6.86 [−6.99, −6.71] | −6.85 [−7.37, −6.35] | +0.001 |
| enumeration muls | −8.08 | −8.00 | −0.002 |
| sieve muls | −5.89 | −5.87 | +0.001 |
| descent tests | −5.90 [−8.10, −3.63] | −5.66 [−13.1, 1.6] | +0.27 [−0.50, +1.06] |
| descent cost per test | 0.01 | 0.01 | 0.000 |
| linear algebra muls | **+2.00** | +1.99 | 0.000 |
| route muls (all) | −5.57 [−6.38, −4.71] | −6.08 [−8.69, −3.41] | +0.16 [−0.14, +0.44] |

- **Every deterministic phase matches natural variation to within ±0.1, with intercepts at 0.** Moving the line has no effect beyond B.
- **The law behind it.** Relation yield per family goes as B^{m−1} with m = 9. Families go as B^{1.2−8}, the sieve per family as B^{+1}, the descent as B^−6 (the six-point rate), and linear algebra as B^+2.
- **The single-exponent B⁻⁵ model hides that linear algebra grows.**
- **One aggregate departs.** The route without the descent shows an intercept of +0.10 [0.04, 0.16]. That is a composition effect of summing power laws with exponents −8, −6 and +2, not an effect of the move: every component's intercept is 0.
- **An active null is cheap but untested.** Moving to a random line whose B equals B_plain is the remaining test. Under "B alone" it predicts a ratio of 1.00 within noise.

### Decay with p (A6)

Route-only ratio, measured against a p-independent phase-mixture model. The model weights the plain phase shares with the move elasticities above.

| arm | measured, p = 1009 / 1511 / 1823 | model | γ_eff by prime |
|:--|:--|:--|:--|
| H | 0.472 / 0.612 / 0.724 | 0.547 / 0.614 / 0.661 | 7.46 / 5.51 / 4.05 |
| T | 0.475 / 0.543 / 0.601 | 0.461 / 0.531 / 0.585 | 5.68 / 5.15 / 4.77 |
| U | 0.538 / 0.585 / 0.609 | 0.501 / 0.575 / 0.634 | 5.33 / 5.25 / 5.60 |

- **The decay is modelled.** Two things drive it: the shrinking B gain, (1 + k/√p), and linear algebra's growing share. The model's γ_eff falls from 6.0 to 5.2.
- **H's excess decay is descent noise.** γ_eff(1009) − γ_eff(1823) is 3.49 [0.67, 6.45] for H, but 0.89 [−1.36, 2.86] for T and −0.28 [−2.88, 2.18] for U, on the same instances.
- **The rule under-predicts H's slope.** The slope of ln(H ratio) on ln p is 0.711 [0.261, 1.170]; the frozen rule's model has 0.248.
- **Extrapolation.** The gain is a constant factor that tends to 1 as p grows. Inside the u64 range (p ≤ 2039) the model keeps it at about ×1.5–1.6.

### Noise and seeds (A7)

The descent's geometric test count (two successes, batches of 64) dominates the noise. The sd of the log descent ratio is 0.68–1.23, and the sd of the log S ratio is 0.25–0.60. Sixteen seeds resolve H1 easily. A ±5% per-prime interval on S needs 99–577 seeds; on the route without the descent it needs 21–90. Per-prime decay and H-vs-U-vs-T rankings in S are therefore not resolved with 16 seeds.

### Charging completeness

- **The table build is fully charged.** It costs 38q muls, 29q adds and q lookups, matching `chi_table` (7 Horner steps of 5 products and 4 additions, plus the norm). The candidate squares are charged, and Horner's 17p muls and 15p adds match `line_count`.
- **Index arithmetic is not charged** (the protocol lists it as negligible). Each walk lookup computes `x0 * p + x1`.
  - **Immaterial for U:** S 0.611 → 0.626, S⁺ 0.525 → 0.532 and band w = 0.1 0.499 → 0.509 at p = 1511.
  - **Decisive for T**, whose walk makes p·L = p²(p+1)/2 lookups (1.7·10⁹ at p = 1511, 3.0·10⁹ at 1823, against a route of about 2.0–2.5·10⁹ muls):

    | T at p = 1009 / 1511 / 1823 (pooled) | S | S⁺ | band w = 0.1 |
    |:--|:--|:--|:--|
    | as reported | 0.487 / 0.569 / 0.635 (0.560) | 0.574 / 0.830 / 1.073 (0.800) | 0.608 / 0.915 / 1.207 (0.876) |
    | index arithmetic charged | 0.652 / 1.079 / 1.434 (1.003), multiply priced as an F_p mult | 0.673 / 1.076 / 1.447 (1.016), 2 ops per lookup | 0.751 / 1.240 / 1.696 (**1.164 [1.103, 1.231]**) |

    An incremental, add-only index would save the multiply but not the two operations per lookup. Either way, T's S advantage is a count lowered while the total rose (AGENTS §6).
- **Linear algebra is underpriced.** The route prices a 62-bit modular op at 16 F_p mults, `bitop/v1` at 31.2. Repriced, H's pooled S ratio goes 0.623 → 0.658 and U's 0.600 → 0.641. Linear algebra's share at 31 grows to 29.1% (p = 1511) and 33.8% (p = 1823) for U.
- **Unemitted F4, descent and LA additions.** Imputed at one per multiplication, they move U's S⁺ from 0.525 to 0.530. They are not a hidden cost of the arms: they shrink with B too.
- **Memory.** The χ table is p² bytes (2.3 MB at 1511, 3.3 MB at 1823), and the protocol neither lists nor charges it. §5a requires it to be listed, not charged. Negligible against any limit here.

## Study B: matched negation rho at p = 1511

- **Homogeneity power (B1).** I simulated the frozen rule with 16 draws from #3278's empirical distribution scaled by c, against 126 bootstrap draws (1000 permutations each, 400 simulations).
  - Rejection rate: 0.99 at c = 0.6, 0.84 at 0.7, 0.45 at 0.8, 0.20 at 0.9, 0.06 at 1.0, 0.29 at 1.15, 0.65 at 1.3, 0.92 at 1.5.
  - The normal approximation gives an 80%-power detectable shift of 0.327, which is 36% of the mean.
  - The observed matched mean is 0.877 of the pooled one, inside the blind zone.
  - Pooling is still right, because rho's steps/√l depends only on l: the pooled mean 0.918 is 3.4% above theory (0.888), and the positive control passes.
  - The reference choice moves S/rho U1 from 0.0065 (pooled) to 0.0067 (theory) to 0.0074 (matched). The conservative statement uses the matched or theory value.
- **Pooling across p (1301–1601 against 1511) is legitimate.** steps/√l and the harness price per step (330.9 F_p mults) do not depend on p.
- **dp-bits (B2).** The post-collision overhead, batch·2^dp/√l, is 0.061% of √l at dp 16 and 0.030% at dp 15, a difference of 0.03% of the mean. Negligible.
- **Rho strength (B3).**
  - **Automorphisms.** E: y² = x(x−α)(x−σα), with α random in F_{q³}∖F_q. A subfield model needs j ∈ F_q or F_{p³}. The previous red team verified Aut = {±1} on 9 instances from this generator; V2's 16 instances were not re-verified (cheap control C4). A √3 Frobenius would at worst take U2 S/rho from 0.0156 to 0.027.
  - **Per-step counts.** F_{q³}: 4.818 mul, 1.004 sq and 0.0645 inv per step (batch 16), so inversion is 2.9% of the tuned price and a larger batch saves at most 3%.
  - **Walk.** Negation map, r = 256 (theory √(π/4)/√(1−1/r)), fruitless cycles handled.
  - **TUNED is not minimal.** It prices an F_{q³} product at 24 F_p mults (6 Karatsuba F_q products plus 2 by s ∈ F_q). With s ∈ F_p, or Toom-3, it is 18 or less, giving about 103 F_p mults per step, ×1.34 cheaper (model).
  - **Tag tracing** is a finite-field-DLP device with no comparable gain for an affine EC walk.
  - **Native work.** The rho emits 17.0 native operations per step, about 7.9 of them cycle comparisons. A leaner, periodic cycle check would raise the cover/rho band ratios (layer A, U2, w = 1: 0.12 → about 0.14). No verdict changes.
- **Like with like (B4).**
  - The cover's work is F_p-native: sieve over F_p[x], F4 over F_p, linear algebra mod l. Its few F_q products are counted at 5 F_p mults (schoolbook with ω).
  - The rho's step is F_{q³} arithmetic, which also reduces to 11-bit F_p products, so the F_p-multiplication unit is commensurable provided rho's F_{q³} arithmetic is priced competently. U2 (TUNED) is therefore the like-for-like F_p-mult unit, slightly generous to the cover by the ×1.34 above. U1 prices rho at the schoolbook harness (55 F_p mults per F_{q³} product), ×2.40 dearer.
  - Additions and lookups must carry the same bit-level price on both arms. `bitop/v1` gives an add 0.096 of a mult and a word op 0.043.
- **Reconciliation with `bitop/v1` (B5)**, p = 1511, V2 sparse records, seeds 17–32 (derived; the proxy is a model):

| unit | sieve / rho | reading |
|:--|--:|:--|
| U1, S (adds free; rho at harness) | 0.0065 | rho-penalising lower bound |
| `bitop/v1` proxy against the harness rho | **0.0091 [0.0085, 0.0099]** | same pricing as ic_acct1, which measured **0.0098 [0.0069, 0.0144]** (§18 route, 4 instances, R3) |
| U2, S (rho tuned) | 0.0157 | like for like in F_p mults |
| `bitop/v1` proxy against the tuned rho | **0.0214 [0.0199, 0.0234]** | the honest unit; ×1.34 more against a minimal F_{q³} product (model): about 0.029 |
| S⁺ / rho S⁺, U1 / U2 | 0.0226 / 0.0444 | adds at a full mult on both arms (over-prices the cover's adds about 10×) |
| S⁺ / rho, rho unpriced, U1 / U2 | 0.0268 / 0.0644 | mixed pricing, an upper bound, not a distance |

The proxy prices emitted F_p additions at 0.096, F4 and descent additions imputed at one per multiplication, lookups at a word op, modular LA operations at 31.2, and the search's index arithmetic. It does not re-price inversions: the harness charges 16, `bitop/v1` about 2.1, which over-charges the cover slightly.

## Combined claim (LEADS.md)

The product is not illegitimate in principle. Arm P equals V2 bit for bit (control C1), and study B's rho was matched to V2's instances (same l and c_add_e). The move is an F_q-isomorphism of the cover with a = t², so l and the ECDLP are unchanged. A direct ratio, arm records over the matched reference, therefore exists, and it is the same construction as study B's own headline (V2 records over new rho runs).

A product of geometric-mean ratios equals that direct ratio only on the same seed set. LEADS multiplies 0.0065 (seeds 17–24) by 0.644 (seeds 17–32) and gives no interval. The success-criterion rule against combining experiments asks for the direct form.

Direct values, derived. The bootstrap is over cover seeds only, with the reference fixed, so these intervals are narrower than study B's joint ones.

| arm | seeds | S/rho U1 | S/rho U2 | S⁺/rhoS⁺ U2 | `bitop/v1` proxy against the tuned rho | band A, U2, w = 1 |
|:--|:--|:--|:--|:--|:--|:--|
| P | 17–24 | 0.0065 [0.0056, 0.0079] | 0.0156 | 0.0445 | 0.0213 | 0.123 |
| H | 17–24 | **0.0045 [0.0042, 0.0049]** (LEADS: 0.0042) | 0.0109 | 0.0248 | 0.0147 | 0.069 |
| H | 17–32 | 0.0042 [0.0038, 0.0046] | 0.0101 | 0.0245 | 0.0139 [0.0129, 0.0149] | 0.068 |
| U | 17–32 | 0.0040 [0.0036, 0.0045] | 0.0096 | **0.0233** (LEADS S⁺ U2: 0.034 over the unpriced rho) | **0.0134 [0.0124, 0.0147]** | 0.065 |

## Scope

Both studies, LEADS and both PDFs state "weak class only; nothing bears on prime-order curves", and nothing in the evidence contradicts it. One addition: on this class the sieve route's total work is nearly flat in p (plain means 3.46·10⁹, 3.51·10⁹ and 4.02·10⁹ F_p mults at p = 1009, 1511 and 1823), while rho grows as p³. So the class sits in a different complexity regime. A "distance to rho" here measures nothing about prime-order targets, and the line-search gain tends to 1 as p grows.

## Honest remaining distance and the strongest next lever

- **Distance on this class.** None: the route is already below the matched rho. At p = 1511 (derived):

  | route | U2, S | `bitop/v1` proxy against the tuned rho | band top (layer A, U2, w = 1) |
  |:--|:--|:--|:--|
  | plain | ×64 | ×47 | ≥ ×8 |
  | with line search (U) | ×104 | ×75 | ≥ ×15 |

  Against a minimal F_{q³} rho (model), the line-search figure in the `bitop/v1` proxy is about ×56.
- **Where the cost sits after the line search** (`bitop/v1` proxy, U at p = 1511):

  | phase | share | ceiling if removed |
  |:--|--:|--:|
  | relation phase (enumeration and sieve muls) | 32% | |
  | descent | 24% | |
  | linear algebra (mod l) | 23.5% (27.1% at p = 1823) | ×1.31 (×1.37) |
  | emitted additions | 16% | ×1.19 |
  | other (mostly the search) | 4% | |

- **L8 is the lever for S⁺ and the band only.** In the like-for-like unit, halving the additions is worth about ×1.09.
- **Linear algebra is the growing lever.** It is the one phase that grows with both B (exponent +2.0) and p. Filtering or structured Gaussian elimination before Wiedemann, and a smaller relation surplus (the relations elasticity is 1.2, so the surplus grows with B), are levers no study has measured.
- **No single remaining lever exceeds about ×1.4 like for like.**

## Required controls (cheapest first)

1. **C1 (no run):** emit the walk's index arithmetic, or switch `table_count` to an incremental add-only index, and restate T's S, S⁺ and band. This report's re-pricing is the predicted outcome: T worse than plain at p ≥ 1511 in S⁺ and the band.
2. **C2 (about 15 lines, 48 runs of about 10 s):** the active random-line null, `JV_FB_LINE_PICK=matched`, moving to the candidate whose B is nearest B_plain. Predicted ratio under "B alone": 1.00 within the descent noise. A matching result is a controlled null.
3. **C3:** reduce the descent variance before any per-prime ranking. Run R ≥ 8 independent descent streams per instance, or report the route without descent plus c_cov × the fitted expected tests. With 16 seeds the arms cannot be ranked against each other in S.
4. **C4:** dump α for V2's 16 instances (`JV_DUMP_INSTANCE`) and check j^{p²} ≠ j and j^{p³} ≠ j.
5. **C5:** price the frontier row in `bitop/v1` through the `jv_bitops` hooks, as the previous review recommended and study A did not do. Run arms P and U and the R3 rho on the same seeds in one frozen protocol. This replaces the product of ratios and supplies the matched reference a `VALIDATION.json` needs.

## Next concrete action

Freeze one combined study (LEADS next-round item 3), extended as follows:

- **Arms:** P and U, plus the matched R3 negation rho, on V2's p = 1511 (and 1823) seeds.
- **Pricing:** counted in `bitop/v1` via the `jv_bitops` hooks, with the table walk's index arithmetic emitted and R = 8 descent streams per instance.
- **Bands:** report U2 and the `bitop/v1` tuned-rho band as the headline, not U1 or S⁺ over an unpriced rho.
- **Purpose:** this yields the admissible frontier row. The next improvement study should then target linear algebra (ceiling ×1.31–1.37, growing), not L8 (×1.19 ceiling like for like).

## Deviations and notes

- **Analysis language.** All numbers come from the Rust tool in `analysis/`. One edit to that Rust source was applied as a string replacement by a shell heredoc (`python3`, text substitution only, no computation). The committed source is the final file, and its output reproduces `derived_numbers.txt` exactly.
- **The `bitop/v1` proxy is a model.** It applies the BITOPS.md price list to emitted counters and imputes the unemitted F4 additions. It reproduces ic_acct1's measured ratio within its interval, but it is not an instrumented `bitop/v1` count.
- **Combined intervals.** The direct values hold the rho reference fixed (study B pooled means), so their intervals cover cover-seed noise only.
- **Files.** The harness blocked writing this report and the YAML, so the main session must commit them. The analysis code and its raw output were written to this directory.

## Review attestation

- **Joints owned:** all claims above (no `review_plan` was supplied).
- **Paths read:**
  - Study A: `PROTOCOL.md`, `README.md`, `run.sh`, `results/{FROZEN,FIT}.sha`, `results/rule.txt`, `results/versions.txt`, `results/{P,H,T,U}_{1009,1511,1823}.json`, `analysis/analysis.txt`, `fit/src/{rule,analyze}.rs`; the git history of the study and of `src/cryptanalysis/{jv_cover,jv_sieve}.rs`.
  - Study B: `PROTOCOL.md`, `README.md`, `table.md`, `fit.txt`, `fit.json` (reference means), `results/FROZEN.sha`, `results/walls.txt`, `results/runs/*.json`, `fit/src/main.rs` (loader and cells).
  - #3278: `jv_cover_negrho_rescore_20261101/results/{inputs/old_rhostrong,new}/*.json` (via the tool).
  - Context: `LEADS.md`, `jv_cover_fbline_20261102/review/README.md`, the previous red team's YAML (head) and pilot JSONL, `jv_sieve_strong_rho_v2_20261020/TABLES.md` (T0, T1, T4), `ic_acct1_cover_bitop_20261021/{README,PROTOCOL}.md`, `docs/units/BITOPS.md` (price table), `src/cryptanalysis/{jv_cover,jv_sieve,jv_rho_strong,jv_bitops}.rs` at HEAD (the cited functions), and AGENTS.md §5, §5a, §6, the success criterion and "no Python".
- **Sibling reports read:** none. `review/validator/` exists in the working tree and was not opened.
- **Verdict:** A1, A4 and A5 hold; A2 holds as measured but is unit-inflated; A3 breaks; A6 and A7 are partial; B1–B5 hold with the stated qualifications; the combined product is to be replaced by the direct computation; the scope holds.
