# Literature and prior-evidence gate

Reviewed before defining or editing the campaign.  “Verified” below means the
source and the cited scope were checked; it does not elevate a repository
measurement to a theorem or a new speed record.

## Primary literature

| source | result used in this review | boundary it imposes |
|:--|:--|:--|
| V. Shoup, “Lower Bounds for Discrete Logarithms and Related Problems,” EUROCRYPT 1997, DOI `10.1007/3-540-69053-0_18` | generic-group square-root lower bound | generic table, MITM, and root-finding searches do not establish a sub-square-root algebraic attack |
| R. Gallant, R. Lambert, S. Vanstone, “Improving the Parallelized Pollard Lambda Search,” Math. Comp. 2000, DOI `10.1090/S0025-5718-99-01119-9` | automorphism/endomorphism speedups for generic walks | matched rho must receive the same usable symmetry |
| P. van Oorschot, M. Wiener, “Parallel Collision Search with Cryptanalytic Applications,” J. Cryptology 1999, DOI `10.1007/PL00003816` | distinguished-point parallel collision search | rho comparisons must include table, restart, and parallel-walk policy |
| J.-C. Faugère, “A New Efficient Algorithm for Computing Gröbner Bases (F4),” J. Pure Appl. Algebra 1999, DOI `10.1016/S0022-4049(99)00005-5` | batched sparse linear algebra | F4 native work is meaningful only with symbolic preprocessing and matrix construction accounted |
| J.-C. Faugère, “A New Efficient Algorithm for Computing Gröbner Bases without Reduction to Zero (F5),” ISSAC 2002, DOI `10.1145/780506.780516` | signature criteria can suppress reductions | a smaller F5 native count is not automatically a smaller ECDLP cost |
| I. Semaev, “Summation polynomials and the discrete logarithm problem on elliptic curves,” IACR ePrint 2004/031 | summation-polynomial decomposition | the factor-base constraint and full relation probability must be inside the measured pipeline |
| P. Gaudry, “Index calculus for abelian varieties of small dimension and the elliptic curve discrete logarithm problem,” JSC 2009, DOI `10.1016/j.jsc.2008.08.005` | small-dimension index calculus | asymptotic regime and representation costs cannot be inferred from toy solver calls |
| Faugère, Perret, Petit, Renault, “Improving the Complexity of Index Calculus Algorithms in Elliptic Curves over Binary Fields,” EUROCRYPT 2012, DOI `10.1007/978-3-642-29011-4_4` | Weil descent plus algebraic solving | Boolean-system generation, solving degree, and full cost are separate evidence obligations |

## Repository evidence

- `docs/ecbench/README.md`: method registry, counted unit, isolation, replay,
  pricing-band, and one-target contracts.
- `docs/bounds/README.md` and `docs/bounds/FRONTIER.md`: 94-record bound
  frontier and procedure-2 interval rules.
- `docs/ecbench/BACKEND_FRONTIER.md`: backend comparisons and the prohibition
  on cross-ranking partial native units.
- `docs/ic/LEADERBOARD.md`: admitted whole-pipeline rows and their unit-local
  references.
- `docs/ic/SWEEPER.md`: supported algebraic cells and absent prime algebraic
  reference arm.
- `research/f4_f5_f6_cross_family_20261011/`: F4/F5/F6 sessions, replays,
  timeouts, and E2E ratios.
- `research/ecbench_rho_optimized_20261014/`: trajectory-identical rho
  optimization and the missing L2 wall promotion.
- `research/prime_base_oracle_tuning_20261029/`: factor-base sweep, holdout,
  pricing band, and independent validation.
- `research/prime_base_oracle_tuning_20261029/second-host/`: L2 EKS replay of
  the P20 result, 2,016/2,016 admitted executions and 1,008/1,008 deterministic
  matches to the original measured records.
- `research/prime_ic_program_20261029/`: two-track ruling: generic root-finding
  versus algebraic relations.
- `research/explicit_relations_e2e_20261030/`: 62-bit GPU explicit-relation
  result and its lack of a matched rho session.
- `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`,
  `RESEARCH_KOBLITZ_INDEX_CALCULUS.md`, and
  `RESEARCH_ECC2K130_IC_FEASIBILITY.md`: mathematical scope and open
  obligations.

## Evidence conflicts resolved

The top-level prime tuning README says leaderboard promotion is pending.  A
later second-host directory in the same study records the requested EKS replay
and independent validation.  This review uses the later evidence and retains
its limits: one 20-bit curve, counted pricing band only, and no scaling claim.

Historical WDSAT XG data are not promoted: a retained audit found a false
UNSAT on one decomposable target, and the external conflict counters are not a
common E2E unit.  SAT-CDCL and FES remain in the live native solver registry,
but no complete common-priced E2E sweep exists for them at this source commit.
