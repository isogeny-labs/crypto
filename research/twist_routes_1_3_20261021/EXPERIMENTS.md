# Twist-assisted IC: experiment program

Date: 2026-10-21. Scope: pursue routes 1–3 and their combinations, devise discriminating experiments, and preserve adjacent conditional routes without claiming an algorithm or novelty.

This program extends [PR #2380](https://github.com/aburan28/crypto/pull/2380) and the [central tracker](../notes/index-calculus/RESEARCH_TWIST_IC_ROUTES_1_3.md). It does not substitute an unexecuted plan for measured results. Bounded native panels have now run; the [execution results](reports/EXECUTION_RESULTS.md) separate verified finite checks, partial measurements and remaining construction/validation gaps. The original acceptance criteria below are retained.

## Questions and deliverables

| Requested item | Deliverable | Status |
| --- | --- | --- |
| Pursue trace restrictions | Explicit coordinate, slope, norm, multiplicative and divisor families; E01–E06 | Bounded native restrictions/censuses executed; broader full-cost criteria open |
| Pursue covers | Explicit C_c family and complement equations, gluing alternatives, subgroup/class-map gates; E10–E13 | Ten executable class transfers checked; destination solver/full cost open |
| Pursue solver symmetry | Four membership encodings, equivalent rational witnesses, torsion descent and solver panel; E07–E09 | Equivalent systems and three bounded solver attempts executed; broader grid/full cost open |
| Pursue combinations | Representation/isogeny ablations and full factorial pipeline panel; E14–E15 | 1,920 native factorial records; 1,704 exact verified replays; cost calibration open |
| Preserve other angles | Exact pairing applicability and bounded characteristic-zero lifting ingress; E16–E17 | Public applicability and bounded lift/pairing controls executed; no NIST transfer supplied |
| Make formula checks reproducible | Independent small Rust crate reusing the repository identity checker | E00 and nine explicit Lean lemmas checked; concrete field/group/map bridges remain open |
| Preserve follow-up execution | Three versioned backlog protocols, machine-readable experiment matrix, graph and report PDF | Results and receipts recorded; cards remain blocked for unmet full-campaign criteria |

## Common mathematical contract

The source remains a declared E/F_p, characteristic p > 3. The actual DLP target is the original rational order-r subgroup, not the larger ambient E(F_(p²)) group. An x-preserving twist uses d*y²=f(x), omega²=d a nonsquare. Alternative models carry their explicit conversion.

The trace is tau=1+pi. Anti-invariant points lie in its kernel. Over an odd-order rational group, a trace fiber is [2^-1]T+E^-; counting all lifts can inflate witness counts while leaving projected rows unchanged. This is the reason for the exact-F and rational-witness controls, not a complexity lower bound against all coordinate algorithms.

### Concrete trace families

Write S=(u+v*omega,w+z*omega).

- **Coordinate boxes:** u in U_H and v in V_H, with U_H=V_H={0,...,H-1} and H in {2,4,8}; both signs of y and all exceptional branches retained.
- **Slope boxes:** u in U_H and m=z/v in M_H, with the v!=0 branch certified separately. The trace has X=m²-2u. A pure-F_p witness formulation uses the same quartic below, without representing an extension point.
- **Norm families:** u²-dv² in W_H, optionally intersected with a coordinate box; analogous y-norm restrictions are separate arms.
- **Multiplicative families:** subgroups/power images only at H dividing p-1, with the exact image size checked. A parameterization with costly membership is not a free small base.
- **Divisor families:** generic degree-two places versus pure twist pairs. Principal divisors of x-x0 are removed before row formation; they carry no new class relation.

The quartic used in the slope arm is

$$
\Delta_T(m)=m^4-6Xm^2-8Ym-3X^2-4A,\qquad 4dv^2=\Delta_T(m).
$$

The line-intersection identity gives a rational-only formulation with exactly the same witnesses. E00 checks the polynomial arithmetic; the group interpretation and exceptional cases remain separate obligations.

### A concrete cover family and its complement

Let f(x)=x³+Ax+B and choose c with f(c)!=0. Consider

$$
C_c:\;v^2=f(u^2+c).
$$

The two quotient maps on their affine domains are

$$
(u,v)\longmapsto(u^2+c,v)\in E,\qquad
(u,v)\longmapsto(t,z)=(u^2,uv)\in D_c,
$$

where

$$
D_c:\;z^2=t f(t+c).
$$

For t!=0, put F=f(c), a_2=3c²+A and

$$
X=F/t+a_2/3,\qquad Y=Fz/t^2.
$$

The short complement coefficients are represented without denominators by

$$
3A_D=3Ac^2+9Bc-A^2,
$$

$$
27B_D=-27Bc^3+18A^2c^2+27ABc+27B^2+2A^3.
$$

These are candidate explicit model-conversion formulas with a polynomial preflight, not a certified Jacobian subgroup transfer. Require nonsingularity, f(c)!=0, characteristic >3, t!=0 on this affine chart, separate infinity/ramification charts, and actual divisor-class maps.

The complement must be checked: compare its trace and j, then construct an F_p-isomorphism to the requested twist when claimed. Same j alone confuses twists; opposite trace alone supplies no explicit map. Preserve complements that fail the twist test as controls.

For a smooth projective geometrically irreducible C/F_p, genus g, with a nonconstant degree-delta map to E, the standard point/fiber bound gives an order-p image when g and delta are fixed. E13 therefore tests restrictions and pays for them; embedding the original subgroup in a larger Jacobian is not itself an advantage.

## Inputs and selection rules

The already registered ordinary prime-order structural fixtures are:

| Source curve | p | A | B | #E(F_p) |
| --- | ---: | ---: | ---: | ---: |
| icv1-fp6-tm7-fc20da94 | 59 | 1 | 13 | 67 |
| icv1-fp10-t5-192cb216 | 827 | 1 | 15 | 823 |

They test group shape and implementation mechanics, not NIST coefficient fidelity or scaling. The registered even-order controls icv1-fp7-t4-0c09e92f and icv1-fp7-t2-4910f2d0 expose 2-primary assumptions; the odd-composite control icv1-fp8-tm1-1ed3feb7 separates oddness from primality. Verify all parameters and subgroup choices rather than importing a generator by bit count.

For later scaling, the proposed train/holdout ladder has prime-field bit lengths {8,10,12,14}, A=-3, ordinary odd prime-order curves, and two independently generated instances per rung. Freeze the generator, all rejected candidates, registered ICV1/EC1 representations, extension moduli, generators, exact fixture files and SHA-256 hashes in an amendment before timing. No such larger fixtures have been generated or admitted by this plan.

Stage panels use H in {2,4,8}, arity m in {2,3,4}, projected base budgets B in {8,12,16}, and declared seeds 0–3 for design and 4–7 for holdout. Unsupported and empty cells stay explicit. The resource envelope and expected cost need calibration before readiness; these grids are design inputs, not an authorization to launch a paid campaign.

Tiny exact-census cells enumerate every nonidentity rational target plus a separate identity branch. A performance cell is one previously unseen public target, with target generation excluded symmetrically from the online intervals. Planted decompositions are correctness controls only. Do not use their yield as a uniform-target estimate.

## Experiment matrix

The canonical machine-readable version is [experiments.json](experiments.json). Every row below names a falsifier and what its outcome would change.

### E00 — Polynomial preflight

- **Route / status:** common / verified polynomial preflight and five Lean polynomial lemmas; concrete group/map bridges remain open.
- **Question:** Do the proposed trace and complement coordinate formulas satisfy their exact cleared-denominator polynomial identities?
- **Arms and inputs:** Five compiled statement ASTs; exact integer coefficient expansion and repository identity certificates; deliberately false +1 variants.
- **Controls:** Exact expansion and modular evaluation are different arithmetic checks; all negative variants must reject.
- **Measurements / acceptance:** All five exact coefficient differences zero; 32 issuer and 32 domain-separated checker points pass; five false variants reject.
- **Stop, null outcome and next decision:** Any mismatch stops use of the affected formula; no transfer or performance claim follows from passing.
- **Prerequisites:** source and E00 protocol frozen before execution.

### E01 — Trace fibers and separation

- **Route / status:** 1 / verified exhaustive bounded fixture census; general formal bridge open.
- **Question:** Which apparent relation gains are just multiple lifts of the same trace?
- **Arms and inputs:** All points on the two registered odd-prime-order toy curves over the declared quadratic extension; add separate registered even-order and odd-composite controls.
- **Controls:** Rational-only traces; pure twist trace zero; S+U with U in ker(tau); identity; rational 2-primary cases.
- **Measurements / acceptance:** Exact trace image/fiber counts, distinct projected rows, row ranks and target-row span.
- **Stop, null outcome and next decision:** Do not count repeated fibers as new information; a failed asserted identity is a correctness failure, not a performance loss.
- **Prerequisites:** registered exact extension models and native point verifier.

### E02 — Coordinate boxes

- **Route / status:** 1 / partial: declared restrictions executed; full larger-field norm-y/power families open.
- **Question:** Do small coordinate boxes create a useful small trace base with cheaper membership or decomposition?
- **Arms and inputs:** S=(u+v omega,w+z omega) with u in U and v in V; H in {2,4,8}; enumerate both y signs and branches.
- **Controls:** Exact-image rational base; same-size hash-selected rational base; unrestricted lift control.
- **Measurements / acceptance:** Distinct traces, nonzero coverage, fiber multiplicities, construction/encoding cost and verified independent rows.
- **Stop, null outcome and next decision:** Record empty bases and size collisions; never normalize away construction or duplicate-removal work.
- **Prerequisites:** E01.

### E03 — Slope and real-part restrictions

- **Route / status:** 1+3 / verified 30 finite witness-set comparisons; cost comparison open.
- **Question:** Does restricting u and m=z/v help beyond a rational parametrization of X=m^2-2u?
- **Arms and inputs:** u in U and m in M, H in {2,4,8}; enforce 4dv^2=Delta_T(m); add independent v restriction as an ablation.
- **Controls:** Same quartic witness written entirely over Fp; exact-F table; ordinary low-degree addition witnesses.
- **Measurements / acceptance:** Projected solution-set equality first; then encoding size and counted work per independent row.
- **Stop, null outcome and next decision:** If the candidate and rational witness systems are identical after renaming, classify the result as encoding reuse rather than new twist geometry.
- **Prerequisites:** E01.

### E04 — Norm and multiplicative restrictions

- **Route / status:** 1 / partial: norm/image controls executed; larger-field fibers and costing open.
- **Question:** Can norm constraints or multiplicative structure reduce solve cost after parameterization and membership are charged?
- **Arms and inputs:** Norm(x)=u^2-dv^2 in W; norm(y) restrictions; H-th-power or subgroup conditions only when H divides p-1; intersect with a trace-box family.
- **Controls:** Same F without norm witnesses; same-size rational base; matched subgroup-size control; nonsquare d rescaling.
- **Measurements / acceptance:** Image size, membership degree/selector size, projection collisions and complete stage work.
- **Stop, null outcome and next decision:** Unchanged projected sets at unchanged or higher cost close only that restriction; missing roots do not count as a win.
- **Prerequisites:** E01.

### E05 — Divisors and degree-two places

- **Route / status:** 1+2 / partial: two sparse-function/divisor controls; unsplit degree 3/4 factors open.
- **Question:** Can generic conjugate pairs help smooth-divisor relations when the principal pure-twist pairs are removed?
- **Arms and inputs:** Sparse rational functions a(x)+y b(x), coefficient support frozen; distinguish x in Fp from genuinely nonrational degree-two places.
- **Controls:** Remove divisors of x-x0 before forming rows; rational-only function family; generic degree-two-place control.
- **Measurements / acceptance:** Factoring and lifting work, nontrivial projected divisor rows, row span and failed factorizations.
- **Stop, null outcome and next decision:** Principal pairs and empty projected rows are not relations; retain the full degree and coefficient-height budgets.
- **Prerequisites:** E01; native divisor and factorization verifier.

### E06 — Exact relation census

- **Route / status:** 1+3 / partial: 1179 configured census rows, 678 verified and 501 unsupported; decomposition cost open.
- **Question:** Which restricted bases actually span the needed target rows?
- **Arms and inputs:** m in {2,3,4}, B in {8,12,16}; all nonidentity targets for tiny fixtures; freeze sign/orbit conventions.
- **Controls:** Complete enumeration or MITM oracle; identical projected base and target; raw versus canonicalized rows.
- **Measurements / acceptance:** Exact relation set, target coverage, rank over r, target-row span and verified known-answer recovery.
- **Stop, null outcome and next decision:** Do not silently replace unattainable full-column rank with a new stopping rule; freeze target-span stopping in advance.
- **Prerequisites:** E02 or E03 or E04.

### E07 — Equivalent system builders

- **Route / status:** 3 / partial: 240 distinct B8/H8 projection cells verified; B12/16 and other bases open.
- **Question:** Which membership representation avoids merely moving cost into a higher-degree constraint?
- **Arms and inputs:** Univariate product membership, bit decomposition with proven range, selector/product chains and explicit branching.
- **Controls:** Untwisted addition chain; exact same projected F; independently enumerated projected solutions.
- **Measurements / acceptance:** Equality of full projected sets including multiplicity/sign policy and exceptional branches; equations/variables/degrees.
- **Stop, null outcome and next decision:** A mismatch blocks all solver comparisons until repaired; coefficient fields and modular-wrap conditions are part of the input.
- **Prerequisites:** E06.

### E08 — Solver and ordering panel

- **Route / status:** 3 / partial: three 1930-cell bounded solver attempts retained; applicable signature reference, broader grid and full costs open.
- **Question:** Does symmetry reduce actual solving cost against the strongest equivalent encoding?
- **Arms and inputs:** Frozen variable orders, chain/quadratized/symmetric encodings; msolve/open reference plus available applicable F4/F5 engines.
- **Controls:** A/A runs; untwisted witness system; redundant-variable positive/negative controls; same output verifier.
- **Measurements / acceptance:** Counted native operations, Macaulay dimensions/nnz, reductions, solving degree, memory, failures and cost per useful row.
- **Stop, null outcome and next decision:** Lower displayed degree alone is not a win; timeouts and OOMs remain incomplete outcomes under the same envelope.
- **Prerequisites:** E07; calibrated external-solver adapters.

### E09 — Torsion over extensions

- **Route / status:** 1+3 / verified bounded translated-torsion controls; general proof bridge open.
- **Question:** Can torsion symmetry survive Galois descent without false factor-base compression?
- **Arms and inputs:** 2-torsion over Fp^3 and combined Fp^6 where applicable; Galois-invariant coordinates and descent constraints.
- **Controls:** No-torsion encoding; [2] map on the odd subgroup; retained extension-field cost; rational torsion special-family control.
- **Measurements / acceptance:** Equivalence, actual orbit/image size, descended system degree and complete representation cost.
- **Stop, null outcome and next decision:** A geometric torsion orbit that leaves the rational base is not a rational factor-base quotient.
- **Prerequisites:** E07; registered extension models and torsion verification.

### E10 — Explicit degree-two covers

- **Route / status:** 2 / verified exhaustive declared-family search over 27140 parameters on ten fixtures.
- **Question:** When is the complementary elliptic quotient actually the required twist?
- **Arms and inputs:** C_c: v^2=f(u^2+c), f(c)!=0; quotient E and D_c: z^2=t f(t+c); use the explicit short complement coefficients.
- **Controls:** Reject f(c)=0; same-j only control; trace-opposite only control; explicit Fp isomorphism check.
- **Measurements / acceptance:** Smoothness/genus, complete quotient maps including infinity, complement trace/j, isomorphism and image size.
- **Stop, null outcome and next decision:** Neither matching j nor opposite trace alone certifies the requested isomorphism or subgroup transfer.
- **Prerequisites:** E00; native smoothness and map checker.

### E11 — Gluing and alternative complements

- **Route / status:** 2 / partial: module filters and two-cover search; three positive toy module cases and the public P-521 module filter need actual torsion/polarization/quotient/map constructions.
- **Question:** Do other gluing orders or complementary-factor constructions provide a better executable route?
- **Arms and inputs:** Galois-equivariant 2/3-torsion gluing where applicable; recover the complement from a supplied cover; bounded typed candidate families.
- **Controls:** Degenerate/product polarization control; incompatible Frobenius control; identical-field checks.
- **Measurements / acceptance:** Actual curve/map construction, indecomposable polarization, subgroup kernel, factored evaluation cost and failures.
- **Stop, null outcome and next decision:** A rejected family or failed bounded search does not establish universal nonexistence.
- **Prerequisites:** E10; gluing/complement implementation.

### E12 — Jacobian arithmetic and recovery

- **Route / status:** 2 / partial: ten executable transfers, 5855 classes, independent PARI orders; formal/independent-host class-map bridge open.
- **Question:** Does the proposed transfer work on the divisor classes the collector actually produces?
- **Arms and inputs:** Pullback/norm on the chosen infinity model; arbitrary valid reduced classes; image subgroup order and recovery.
- **Controls:** phi_*phi^*=[degree]; origin and ramified fibers; independent CAS; direct-E solver.
- **Measurements / acceptance:** Exact group-law and kernel/recovery checks; separate setup, maps, arithmetic and recovery costs.
- **Stop, null outcome and next decision:** Unsupported two-infinity or arbitrary-class arithmetic remains unsupported, not approximated by rational point-class tests.
- **Prerequisites:** E10 or E11; native divisor-class arithmetic and norm.

### E13 — Restricted cover images

- **Route / status:** 1+2 / partial: low-height image census; Jacobian decomposition and useful-row cost open.
- **Question:** Can restricting cover points give a small base without a worse membership or map problem?
- **Arms and inputs:** All-C(Fp) control; restricted u/trace/norm supports; degree/genus ladder only with frozen factored maps.
- **Controls:** Same projected base on E; fixed-genus image bound; independent point counts.
- **Measurements / acceptance:** Actual image size, g/degree/representation size, construction and membership cost, projected rows.
- **Stop, null outcome and next decision:** A fixed bounded-degree cover with order-p image does not supply a small base merely by changing the ambient variety.
- **Prerequisites:** E12.

### E14 — Model and isogeny ablations

- **Route / status:** 1+2+3 / verified bounded map/dual construction: three explicit degree-seven maps and all three algebraic duals in V2; full IC costs open.
- **Question:** Are any savings due to cheaper constants or another isogenous representative rather than new relations?
- **Arms and inputs:** Nonsquare d choices, x-preserving/standard twist models, fixed small-degree rational isogeny routes and verified return maps.
- **Controls:** Equivalent direct-E representation; same source target/subgroup; conversion charged both ways.
- **Measurements / acceptance:** Identical recovered outputs and projected relation policy; map, primitive and solver cost separated.
- **Stop, null outcome and next decision:** Different j or field models require typed identities; do not infer an unknown-scalar speedup from faster known-scalar multiplication.
- **Prerequisites:** E07; verified map adapters.

### E15 — Factorial pipeline ablation

- **Route / status:** 1+2+3 / partial: 1920 native records, 1704 verified and identically replayed, 216 empty-base errors; calibrated full cost and independent host open.
- **Question:** Which component or combination improves the actual IC baseline?
- **Arms and inputs:** Trace restriction on/off, cover on/off, solver symmetry on/off; pairs use equal target/base or explicitly separate changed-base studies.
- **Controls:** Strongest current IC baseline; strongest same-target rho; direct destination generic solver; independent holdouts.
- **Measurements / acceptance:** Cold single-target complete counted cost and S; setup/online split; 0/0.1/1 GAE pricing band; verification and replay.
- **Stop, null outcome and next decision:** Every claimed IC improvement needs baseline/candidate >1 with all phases; rho is a reported gap, not the IC improvement pass gate.
- **Prerequisites:** E06; E08; E12 for cover cells.

### E16 — Homomorphism and pairing applicability

- **Route / status:** conditional / verified fifteen public-parameter checks and known degree-two pairing positive control; no NIST transfer supplied.
- **Question:** Do small twist factors or a pairing actually supply an admissible transfer on the original subgroup?
- **Arms and inputs:** Exact gcd(r,#E_tw), embedding-degree and torsion/Frobenius tests on the declared public parameters.
- **Controls:** Trace/norm killing pure anti-invariants; translated-coset control; known small-embedding-degree positive family kept separate.
- **Measurements / acceptance:** Exact applicability certificates and a named nondegenerate transfer if one exists.
- **Stop, null outcome and next decision:** A smooth twist order or shared x-coordinate relation does not supply a subgroup-preserving transfer; do not run a nonexistent destination attack.
- **Prerequisites:** exact public-parameter and transfer checker.

### E17 — Arithmetic lift and twist ingress

- **Route / status:** conditional / partial: integral and two 254-case rational-height panels; one rank interval, target lifting and discovery cost open.
- **Question:** Could a characteristic-zero twist reduce to E while providing a usable small-height relation source?
- **Arms and inputs:** Only twists whose parameter is square mod p for a same-E reduction; bounded height/rank/search families stated before a pilot.
- **Controls:** Nonsquare reduction goes to the other finite-field component; fixed-lift and equal-height controls; charge discovery and target lifting.
- **Measurements / acceptance:** Explicit reduction map, target lift, relation independence, heights and construction/search costs before any solver comparison.
- **Stop, null outcome and next decision:** High rank alone is not a cheap target lift or a speedup; no unbounded rank or designer-secret assumption is treated as evidence.
- **Prerequisites:** separate arithmetic-lift construction and height protocol.

## Fair comparisons and attribution

Run three controls when a candidate provides an image F:

1. **Exact-F table:** materialize the identical projected point set and charge construction/storage.
2. **Rational witness:** retain every useful F_p auxiliary parameter and the same quartic/membership constraints; remove only the extension-field interpretation.
3. **Strong ordinary IC baseline:** the best applicable verified pipeline with its own fair tuning budget, plus a matched strong rho reference.

A candidate losing its apparent advantage against control 2 has not discovered a benefit from twist relations. It may still be a useful encoding optimization, which is reported against the previous IC baseline. If a cover changes F, either compare paired arms on the same F or declare a separate changed-base experiment; an eight-cell factorial design cannot assume unrelated workloads match.

Membership is part of the problem. Compare product polynomials, bounded bit decompositions, selector/product chains and explicit branch enumeration with all selection, range and modular-wrap checks charged. A high-degree membership constraint cannot be omitted from the solver input.

Use canonical signed/orbit representatives only under a proved rule. Count projected nontrivial coefficient rows, their rank and the target-row span. Duplicate lifts, zero rows and scalar/multiple rows stay in the raw evidence and are not called independent relations. Full column rank is not silently assumed; target recoverability and the stopping rule are frozen before execution.

## Success, stopping and cost

The immediate success criterion is a verified lower cost against the best current IC baseline on identical accepted workloads. For a stage, compare counted cost per independent useful projected row and label it a stage result. For a full pipeline, report baseline_total/candidate_total >1 with every phase included. A rho crossover is a separate claim requiring the same target, complete intervals and the strongest applicable rho implementation.

Use the repository's single-target ecbench contract for method comparisons. Report cold cost, setup and online costs separately; retain attempted and completed counts, verification, failed searches, OOMs and timeouts. Price uncalibrated work at 0, 0.1 and 1 GAE symmetrically for all arms. A ranking that changes within this band is unresolved. Wall-time results additionally require isolation, A/A controls, paired reruns and admitted uncertainty.

For solver experiments, export exactly the same systems and verify full projected solution sets before ranking. Compare with the fastest available applicable external implementation; missing Magma or other proprietary comparisons are named, not silently replaced by a weaker reference. Exponents require at least four sizes and full phase costs; two toy fixtures are not a scaling result.

For an admitted performance comparison, freeze per-cell limits after cost calibration. The initial L0 diagnostic caps were frozen before execution, with full calibration still open and no speed claim admitted. Stop the affected cell on a correctness mismatch. Preserve budget exhaustion as an incomplete outcome under the original envelope. A null result closes the tested family only, not every possible twist-related construction.

## Dependency and execution order

- E00 can run after its source and protocol are committed.
- E01 and E10 are independent construction/geometry diagnostics.
- E02–E06 generate and qualify restricted bases; E07 verifies system equivalence before E08/E09 timing.
- E11/E12 must establish actual divisor-class maps before E13 or any cover-containing E15 cell.
- E14 separates representation costs from relation effects.
- E15 is the final combined pipeline panel; E16/E17 remain conditional ingress tests with no assumed transfer.
- Backlog route cards own these phases and remain blocked until their actual native drivers/checkers, source pins, resource estimates and fixture hashes exist. No unsupported component is marked runnable by adding this document.

See [PROGRAM.svg](PROGRAM.svg) for the dependency diagram.

![Experiment dependencies and bounded execution; the original full campaign remains partial](PROGRAM.pdf)

The PDF includes that same diagram. No performance dashboard value changes until a result meets the repository validation gate.

## Proof map and limits

| Item | Check implemented here | Remaining proof / execution obligation |
| --- | --- | --- |
| Chord polynomial division and quartic expansion | E00 coefficient/evaluated certificates and corresponding Lean identities | Concrete elliptic group and exceptional-chart bridge |
| Complement coefficients and cleared model conversion | E00 arithmetic, Lean identities and ten executable toy class transfers | Faithful formal field/chart/Jacobian maps and independent arbitrary-class replay |
| Trace-fiber and separation statements | Nine exhaustive native censuses and four conditional Weierstrass point-group Lean statements | Concrete finite-field Frobenius/twist and odd-subgroup bridge |
| Image-size bound | A stated standard bound with explicit hypotheses | Applicable curve/morphism hypotheses and proof mapping for any selected candidate |
| Solver equivalence and benefit | 240 distinct projection checks, bounded solver panels and native IC/replay | Broader grids, complete applicable external reference and calibrated full costs |
| NIST-size applicability or novelty | Not established | Separate scalable construction, applicable evidence and a literature novelty audit |

A certificate with proof=null is evaluated-tier, not a Lean proof. Different checker points in one process do not constitute independent-host replay. Clearing denominators certifies only a polynomial identity; it does not prove the original rational map is defined everywhere.

## Sources

- [Semaev, summation polynomials, ePrint 2004/031](https://eprint.iacr.org/2004/031).
- [Faugere–Huot–Joux–Renault–Vitse, torsion symmetries, EUROCRYPT 2014](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf).
- [Joux–Vitse, cover and decomposition, ePrint 2011/020](https://eprint.iacr.org/2011/020); extension-field results are not a NIST prime-field result.
- [Broker–Howe–Lauter–Stevenhagen, gluing constructions](https://arxiv.org/abs/1403.6911).
- [Gallese–Lombardo–Naccarato–Zannier, Finding the complement of an elliptic curve inside a Jacobian, arXiv:2606.02429v1](https://arxiv.org/abs/2606.02429). Prior art for computing a complementary factor, not an IC speedup.
- [Higher-genus implementation contract](../../.agents/skills/ecdlp-surfaces/references/higher-genus.md), [identity certificate contract](../../docs/identity-certificates/README.md).
- [PR #2368](https://github.com/aburan28/crypto/pull/2368), unmerged prior mixed-component study at the time this program was written; no new replay of its results is claimed here.

## Execution evidence

The historical [E00 preflight report](reports/RESULTS.md) checks five exact polynomial identities and evaluated certificates, rejects five false variants, and records 22 supporting tests. The subsequent [bounded execution report](reports/EXECUTION_RESULTS.md) and per-experiment statuses above record E01–E17 execution/applicability evidence and every remaining original requirement. No calibrated speedup or novelty claim follows.
