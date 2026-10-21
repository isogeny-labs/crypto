# Twist-assisted index calculus: routes 1–3

- **Updated:** 2026-10-21
- **Status:** planning / tracking only; all three research routes remain open.
- **Target family:** NIST prime-field P-curves, including P-256, P-384 and P-521; initial implementation checks must use registered public synthetic ordinary prime-field fixtures.
- **Deliverable:** preserve the three routes, their prerequisites, controls and completion criteria. This note does not claim a new construction, novelty, a measured improvement, or a security result.
- **Shared blocker:** [missing constraint and solver comparison](../../../problems/twist-trace-constraint-and-solver-gap.md).
- **Prior work:** [PR #2368](https://github.com/aburan28/crypto/pull/2368), the mixed-component twist experiment, was open when this tracker was prepared. Its findings and receipts are prior unmerged work, not a newly replayed or accepted baseline for these routes.

## Requirement and status map

| Route | Question | Actual status | Next deliverable | Execution card |
| --- | --- | --- | --- | --- |
| 1. Traces of extension points | Can a structured set in E(F_(p²)) make decomposition of its small trace image cheaper? | Not attempted under a frozen protocol | Native trace/fiber verifier, a specific membership restriction, and its exact rational-image control | [trace factor base](../../../backlog/XP-20261021-twist-trace-factor-base/README.md) |
| 2. Covers and Jacobians | Can a cover involving E and its twist produce cheaper projected divisor relations? | Construction and subgroup transport not established for a selected candidate | Explicit curve, maps, kernel/subgroup certificate, supported divisor arithmetic and costed destination solver | [cover transfer](../../../backlog/XP-20261021-twist-cover-transfer/README.md) |
| 3. Symmetry and elimination | Can keeping extension variables and exploiting Frobenius reduce actual solving cost? | No candidate encoding or matched solver measurement yet | Equivalent native polynomial systems, exceptional-case handling, independent solution-set replay and paired solver measurements | [symmetry solver](../../../backlog/XP-20261021-twist-symmetry-solver/README.md) |

The cards are deliberately **blocked**, with zero proposed funding and a failing execution sentinel. They are not runnable jobs, scheduled work, admitted performance protocols, or evidence that execution continues.

## Mathematical setup and proof obligations

Work in characteristic p > 3 with a nonsingular short Weierstrass model
E: y² = x³ + A x + B over F_p. Fix a nonsquare d, write omega² = d, and use the x-preserving twist model d y² = x³ + A x + B. Its embedding sends (x,y) to (x,omega y). Record the coordinate conversion if another twist convention is used.

Let pi be p-Frobenius and tau = 1 + pi on E(F_(p²)). Embedded twist-rational points satisfy pi(U) = -U. For a rational target, any relation R = A_sum + B_sum, with rational A_sum and anti-invariant B_sum, forces 2 B_sum = O. For the named NIST P-curves, the rational group has odd prime order, so B_sum = O.

These are working contracts to reproduce and certify, not newly checked formal results in this PR. A proof map, faithful Lean statements/build receipts and identity certificates are owed before any later PR calls the mathematical obligations formally verified. A finite toy check alone does not discharge them.

| Obligation | Exact scope to check | Status / required evidence |
| --- | --- | --- |
| M1. Frobenius separation | Rational and anti-invariant finite-field points; any 2-primary exception explicit | Prior-work pointer only; new independent native replay and faithful proof mapping pending |
| M2. Trace fibers | For odd rational group order r, tau^(-1)(T) = [2^(-1) mod r]T + ker(tau), and ker(tau) is the embedded twist group | Written derivation / native exhaustive replay / Lean bridge pending |
| M3. Coordinate identities | The affine v != 0 branch below; v = 0 and O handled separately | Native algebraic certificate and exceptional-branch verification pending |
| M4. Cover image bound | Smooth projective geometrically irreducible C/F_p, genus g, nonconstant F_p-morphism C -> E of degree delta | Check hypotheses, finite-fiber bound and Weil-bound application; formalization status pending |
| M5. Relation equivalence | Same projected point set, sign convention, multiplicities and target-row span | Exact native census and independent solution-set comparison pending |

No numerical toy outputs from an unarchived scratch calculation are imported as evidence.

## Route 1: traces of general extension points

For T in E(F_p), the trace fiber has the proposed contract

$$
\tau^{-1}(T)=[2^{-1}\bmod r]T+E^-,
\qquad
E^-=\{U\in E(\mathbb F_{p^2}):\pi(U)=-U\}.
$$

Consequently, an unrestricted m-term curve decomposition has (#E^-)^m extension lifts, all projecting to the same relation. Count distinct projected coefficient rows, their rank and the target-row span; do not count those lifts as additional relation information.

The missing object is a specific subset S of E(F_(p²)), with an explicit membership condition coupling the two components, such that F = tau(S) is small and its decomposition problem is cheaper. A uniformly sampled extension set, pure twist points, or translating a fixed rational point by arbitrary twist points is a control, not that construction.

- [ ] Choose and register exact toy curve, twist, field basis, subgroup and generator identities; retain the NIST prime-field scope separately.
- [ ] Verify trace additivity, the full fiber census and projection, including O and the rational/anti-invariant branches.
- [ ] Specify the membership restriction and compute the distinct image F, its duplicates, encoding size and construction cost.
- [ ] Freeze the same-F rational-only control and an explicit-addition-witness baseline before measuring.
- [ ] Census small instances completely; compare projected relation sets, coverage, row rank and target-row span.
- [ ] Price construction, encoding, failed attempts, projection, deduplication and verification per independent projected relation.

**Discriminating outcome:** more lifts with unchanged projected relations and unchanged or higher total cost is a null/negative outcome for that restriction. A verified reduction in relation-stage cost is a stage improvement; it is not an end-to-end DLP result.

## Route 2: covers involving E and its twist

A candidate may seek J(C) isogenous over F_p to E x E_twist. Existence of that product, matching Frobenius factors or an isogeny-class statement does not supply a smooth curve, explicit divisor maps or a faster solver.

For a cover phi: C -> E of degree delta, check the induced pullback and norm, the composition phi_* phi^* = [delta], and the actual subgroup kernel. Coprimality of delta and r is an injectivity criterion once those maps and hypotheses have been established. Record the infinity model; unsupported two-infinity arithmetic is not interchangeable with a one-infinity backend.

A fixed-genus, bounded-degree map also has the working image-size constraint

$$
|\phi(C(\mathbb F_p))|
\ge \frac{p+1-2g\sqrt p}{\delta}.
$$

Thus using every rational point of a fixed cover does not by itself provide a sublinear-size factor base as p grows. Restrictions, growing degree/genus, construction cost and actual image size must be accounted for.

- [ ] Select a construction family and freeze its search boundary; keep failed constructions and unsupported cases.
- [ ] Construct a smooth C over the stated field with an explicit genus and infinity configuration.
- [ ] Verify the maps on divisor classes, exceptional fibers, homomorphism law, image order, kernel and recovery.
- [ ] Identify the complementary elliptic factor rather than assuming it is the required twist.
- [ ] Supply supported divisor arithmetic and a destination solver for the actual subgroup image.
- [ ] Compare against both a direct rational-only IC baseline and the strongest applicable source-group reference on the identical public target.

Use the [higher-genus workflow](../../../.agents/skills/ecdlp-surfaces/references/higher-genus.md). Reuse existing [balanced-Jacobian/map](../../../problems/prime-cover-balanced-jacobian-and-induced-maps-missing.md) and [subgroup-certificate](../../../problems/catalog-cover-subgroup-certificates-incomplete.md) records when a chosen construction encounters those exact gaps.

## Route 3: Frobenius symmetry and auxiliary equations

Write a general extension point as

$$
S=(u+v\omega,\;w+z\omega).
$$

Its base-field coordinate equations are

$$
w^2+d z^2=u^3+3d u v^2+A u+B,\qquad
2wz=v(3u^2+d v^2+A).
$$

For v != 0, set m = z/v and T = tau(S) = (X,Y). The candidate identities to certify are

$$
X=m^2-2u,\qquad Y=m(u-X)-w,
$$

$$
4d v^2=m^4-6Xm^2-8Ym-3X^2-4A.
$$

Retain domain conditions, for example through a checked nonzero branch or saturation. The v = 0 and identity branches must not be lost by clearing denominators. For a fixed valid T, auxiliary solutions can merely describe its twist fiber.

- [ ] Implement a native system builder for a chosen route-1 restriction and a rational control with the same projected factor base.
- [ ] Compare full solution sets after projection, signs, multiplicities, exceptional branches and normalization.
- [ ] Include an untwisted low-degree addition-witness chain; do not compare only with an expanded resultant.
- [ ] Freeze variable orders, solver settings, fixture hashes, arities, resource envelopes, seeds and holdouts before timing.
- [ ] Record solving degree, Macaulay rows/columns, nonzeros, reductions, native work, memory, failures and projected-row yield.
- [ ] Compare with the fastest available applicable external solver on the same systems; name missing proprietary comparisons.
- [ ] Attribute a gain to a change in encoding, representation, algorithm or accounting; lower displayed degree alone is insufficient.

Route 3 can begin with a standalone equivalence verifier. A comparative solver claim waits for a specific restricted set from route 1, or a separately frozen explicit control construction.

## Acceptance and accounting

The immediate success criterion is a verified measured improvement over the strongest current IC baseline for the same workload. Beating rho is a separate long-term goal, not a prerequisite for reporting an IC improvement.

Before execution, freeze a protocol with the actual baseline commit, candidate, exact fixtures, controls, thresholds and stop conditions. The baseline is **not yet selected or frozen** by this tracker. PR #2368 is a dependency to assess, not automatically the baseline.

For a stage comparison, report baseline_cost / candidate_cost and require a ratio > 1 in the same counted unit with identical verified outputs. Record uncertainty and paired confidence intervals for any admitted wall-time claim. For a pipeline comparison, include setup, construction, encoding, every failed attempt, solving, lifting/projection, deduplication, verification, matrix work and target descent. Report cold one-target cost first, setup and online intervals separately, and the matched rho gap. Apply the 0, 0.1 and 1 GAE pricing band symmetrically.

Full rank must not be assumed attainable. Verify target-row span and scalar recovery against the correct baseline rather than silently changing the stopping rule after a rank deficit.

| Evidence level | What it permits | What remains open |
| --- | --- | --- |
| Working formula / literature pointer | A stated proof obligation and candidate design | Faithful formalization, identity certificate, implementation |
| Exact toy replay | Correctness on the frozen finite population | General proof, growing-size behavior, NIST applicability |
| Verified solver-stage improvement | A stage cost reduction on matched inputs | Complete IC pipeline cost |
| Verified cold pipeline improvement | IC improvement on the measured family and sizes | Broader scaling and standard-curve conclusions |
| Missing or inconsistent costs | An implementation gap or unresolved comparison | No unqualified speed or security claim |

## Execution and closeout

- [x] Routes 1–3, their dependency structure, source pointers and unresolved obligations recorded.
- [x] Three blocked backlog cards and a shared problem record linked.
- [ ] Native drivers, resumable execution, independent checkers and concrete resource estimates implemented.
- [x] Route design protocols committed and hashed; E00 source/protocol frozen before its arithmetic run.
- [ ] Route fixture/cost protocols admitted; actual executable route code pinned to a merged commit.
- [ ] Backlog cards reviewed and promoted to ready; lab location and funding approved under the backlog protocol.
- [ ] Mathematical proof map, checked Lean modules and identity certificates attached where feasible.
- [ ] Matched native runs and independent replays completed; null results, failures and OOMs preserved.
- [x] Experiment-program and E00 reports, editable dependency diagram and PDFs delivered.
- [ ] Route curve/solver/transfer reports and applicable external comparisons delivered.
- [ ] Only verified results promoted through the repository validation gate; affected canonical views regenerated when required.

No curve/solver/transfer experiment, remote job or scheduled follow-up is launched by this PR. The separately recorded E00 arithmetic preflight has run. There are no new curve registrations, measured rows or benchmark values, so scoreboard, leaderboard and browser regeneration is not triggered. Execution returns in linked result PRs and updates this checklist; merging a plan does not complete its experiments.

## Sources and existing work

- [Semaev, summation polynomials, ePrint 2004/031](https://eprint.iacr.org/2004/031).
- [Faugere–Huot–Joux–Renault–Vitse, symmetrized summation polynomials, EUROCRYPT 2014](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf). Torsion assumptions must be checked; this is not a NIST speedup.
- [Joux–Vitse, cover and decomposition, ePrint 2011/020](https://eprint.iacr.org/2011/020). Its extension-field setting does not establish a prime-field result.
- [Broker–Howe–Lauter–Stevenhagen, genus-2 constructions](https://arxiv.org/abs/1403.6911). Gluing is prior art, not novelty claimed here.
- [NIST SP 800-186](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf), prime-field P-curve domain parameters.
- [Prior mixed-twist PR #2368](https://github.com/aburan28/crypto/pull/2368), [report at its recorded head](https://github.com/aburan28/crypto/blob/f854eee7dde9f429d78590cd6e5ff015ac20d3f6/research/twist_index_calculus_20261020/MIXED_REPORT.md): unmerged prior work, not independently rerun here.
- [Factor-base notes](RESEARCH_INDEX_CALCULUS_FACTOR_BASE.md), [symmetrized Semaev notes](RESEARCH_SYMMETRIZED_SEMAEV.md), [cover/decomposition ledger](RESEARCH_COVER_DECOMPOSITION_LEDGER.md).

## 2026-10-21: expanded experimental program

The follow-up request to pursue all angles is tracked in the [18-experiment program](../../twist_routes_1_3_20261021/EXPERIMENTS.md) and [machine matrix](../../twist_routes_1_3_20261021/experiments.json). The three cards now contain hashed design protocols. E00 has a native polynomial-preflight implementation; its receipts distinguish arithmetic checks from a group/cover/solver result. E01–E17 remain designed but unperformed, including conditional pairing and characteristic-zero lift ingress. The dependency diagram and PDF accompany the program. Broader construction, equivalent-system, cost and independent-replay obligations remain open.

The [E00 arithmetic result](../../twist_routes_1_3_20261021/reports/RESULTS.md) now records five matching coefficient expansions/evaluated certificates, rejection of five false variants and 22 supporting tests. This does not discharge M1/M2 group proofs, M3 exceptional charts, M4 cover hypotheses, M5 solver equivalence, or any measured improvement.
