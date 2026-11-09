---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# A prime-five smooth Cantor cycle

For $y^2=x^5+1$ over $\mathbf F_3$, Lean proves that the complete
reduced polynomial-pair model has ten elements, eight with split $u$.
The actual five pairs $(1,0),(X,\pm1),(X^2,\pm1)$ form a cyclic
arithmetic image. Its generator $(X,1)$ has order five, different from
the characteristic. All five members are smooth, giving an exact
inequality $1\ne4/5$ against the ambient fraction. [pcc:unverified]

The count proof classifies every one of the 91 bounded coefficient
shapes and proves a bijection with arbitrary actual pairs: $u$ monic,
$\deg u\le2$, $\deg v<\deg u$, $u\mid v^2-(X^5+1)$.
It proves exactly ten valid pairs. Exactly two have nonsplit
$u=X^2+X+2$, with $v=\pm X$; splitting is the actual
`Polynomial.Splits` predicate. The curve polynomial is monic, degree
five and separable. [pcc:unverified]

For every ordered pair in the five-cycle, an explicit certificate gives
common divisors and a Bezout equation identifying the actual normalized
full gcd. The proof then checks the composition and numerator equations:

$$d=\gcd(\gcd(u_P,u_Q),v_P+v_Q),\qquad d^2U=u_Pu_Q,$$
$$a u_P+b u_Q+c(v_P+v_Q)=d,$$
$$a u_Pv_Q+b u_Qv_P+c(v_Pv_Q+f)=d(V+Uq).$$

The monic intermediate $U$, reduced intermediate $V$, and exact final
reduction are checked in all 25 cases. Only after this matching is the
Fin5 group transported to the actual five pairs. Lean proves generation,
order five, properness of the image and both exact fractions.
[pcc:unverified]

![Actual polynomial classification and all 25 addition certificates establish the finite cyclic-image inequality. Dashed bridges remain open.](proof.pdf){width=100%}

The full ten-pair ambient group operation and geometric Picard/Jacobian
identification remain separate formal obligations. This proves a finite
arithmetic cycle inequality; it does not refute Gaudry's fixed-genus limit
or a qualified large-prime/typical-subgroup heuristic. [pcc:unverified]

\newpage

# Proof map and verification scope

The combined declaration is

`MumfordStollCycleGroupAudit.stoll_cycle_audit_certificate`.

| Obligation | Declaration, abbreviated namespace |
|:--|:--|
| Arbitrary-pair completeness | Stoll.canonical_valid_shape |
| Complete ten/eight counts | Stoll.reducedPair_cardinality / smooth_reducedPair_cardinality |
| All 25 actual additions | StollCantor.cycle_all_pairs |
| Actual normalized full gcd | CycleGroup.normalized_full_gcd / step_has_actual_full_gcd |
| Final remainder | CycleGroup.reduction_matches_remainder |
| Group addition matches certificates | CycleGroup.addition_is_Cantor |
| Generator/order/generation | generator_coordinates / generator_order / generated_by_actual_point |
| Properness and unequal fractions | cycle_is_proper / exact_proportion_inequality |

Exact full names and hypotheses are in `../../STOLL_CYCLE.md` and the
three Lean modules. The tracked project gate passes 348 declarations,
zero failures and zero trusted-native exceptions, including generated
declarations. Lean 4.35.0 and Mathlib revision
`56a005855ea17e0b0958f73ec801ae140b0a1679` are pinned. Only propext,
Classical.choice and Quot.sound are accepted; no holes, custom mathematical
axioms or native evaluation are used. [pcc:unverified]

Source/plan commit `327d3183` precedes supplement
`IDC1hb7102c15384f2648` and theorem packet `PCC-20261109-d8b77b`.
The local PCC check passes at 23:00:19Z on producer host `762f5fc120c9`.
Signed standing remains unverified. Source/issuer/lock bytes were checked
against the commit before issue and again afterward. [pcc:unverified]

The scalar supplement checks 32 derived and 11 extra seeded points over
$2^{61}-1$, for five variables and total degree five. It covers only the
quadratic quotient/remainder rewrite for $x^5+1$. Its exact 4117 bytes
are retained without a JSON roundtrip. Counts, splitting, actual gcd,
all 25 additions, order, generation and fraction inequality rest on Lean;
the supplement does not prove those statements. [pcc:unverified]

Failed and warning-bearing attempts remain in a lossless archive with
per-file hashes. The final Cantor build and group build pass with no
printed warnings; the whole project retains existing Lake module-system
advisories. No linter is waived. See `../../stoll-verification.json` and
`../../evidence/stoll-cycle-gate.log`. [pcc:unverified]

Primary source: Stoll2014 Example 4.19, printed pp. 25--26, read in full,
and the representation/addition sections in `../../stoll-sources.json`.
The rational worked example motivates the cycle; characteristic-three
validity, completeness and arithmetic are separately checked here.
Original Cantor1987 remains unread after publisher403. No novelty claim
is made. Native full-population evidence is a separate report. The
original fourteen-item audit remains partial. [pcc:unverified]
